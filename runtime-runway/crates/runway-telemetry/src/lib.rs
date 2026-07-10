use std::collections::HashMap;

use anyhow::{Result, anyhow};
use opentelemetry::{global, trace::TracerProvider as _};
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::{runtime, trace as sdktrace};
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

/// Returned by `init()`. Flushes spans and Sentry events on drop.
pub struct TelemetryGuard {
    sentry_enabled: bool,
    #[cfg(feature = "sentry")]
    _sentry: sentry::ClientInitGuard,
}

impl TelemetryGuard {
    #[must_use]
    pub fn sentry_enabled(&self) -> bool {
        self.sentry_enabled
    }
}

impl Drop for TelemetryGuard {
    fn drop(&mut self) {
        global::shutdown_tracer_provider();
    }
}

/// Configuration for the telemetry stack.
pub struct TelemetryConfig {
    /// Cloud Run service name. `OTEL_SERVICE_NAME` overrides the caller default.
    pub service: String,
    /// Deployment environment: "dev", "staging", "prod".
    pub env: String,
    /// Sentry DSN. Required in production; optional in local development.
    pub sentry_dsn: String,
    /// OTLP trace endpoint. Defaults to Cloud Trace; standard `OTEL_*` env vars
    /// are preferred over the legacy `OTLP_ENDPOINT`.
    pub otlp_endpoint: Option<String>,
}

impl TelemetryConfig {
    pub fn from_env(service: impl Into<String>) -> Self {
        let service = service.into();
        Self {
            service: non_empty_env("OTEL_SERVICE_NAME").unwrap_or(service),
            env: std::env::var("ENV").unwrap_or_else(|_| "dev".into()),
            sentry_dsn: std::env::var("SENTRY_DSN").unwrap_or_default(),
            otlp_endpoint: resolve_otlp_endpoint_from_env(),
        }
    }
}

/// Initialise OTel tracing → Cloud Trace, Sentry error tracking, and JSON structured logging.
///
/// Call once at the top of `main()`. Hold the returned `TelemetryGuard` for the process lifetime.
pub fn init(config: TelemetryConfig) -> Result<TelemetryGuard> {
    let sentry_enabled = ensure_sentry_contract(&config)?;

    #[cfg(not(feature = "sentry"))]
    if sentry_enabled {
        return Err(anyhow!(
            "SENTRY_DSN is set but runway-telemetry was compiled without the sentry feature"
        ));
    }

    #[cfg(feature = "sentry")]
    let sentry_guard = if sentry_enabled {
        sentry::init((
            config.sentry_dsn.clone(),
            sentry::ClientOptions {
                release: Some(sentry_release(&config).into()),
                environment: Some(config.env.clone().into()),
                traces_sample_rate: if config.env == "prod" { 0.1 } else { 1.0 },
                send_default_pii: false,
                ..Default::default()
            },
        ))
    } else {
        sentry::init(sentry::ClientOptions::default())
    };

    // Local app-pairing smokes should not construct the OTLP HTTP exporter:
    // reqwest's macOS system-proxy discovery can panic in headless shells.
    // Keep structured logs + Sentry layer locally; enable OTLP outside local dev.
    let local_dev = std::env::var("LOCAL_DEV").as_deref() == Ok("true");
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    if local_dev && config.otlp_endpoint.is_none() {
        let registry = tracing_subscriber::registry()
            .with(filter)
            .with(json_log_layer());

        #[cfg(feature = "sentry")]
        let registry = registry.with(sentry_tracing::layer());

        registry.init();

        tracing::info!(
            service = %config.service,
            env = %config.env,
            sentry_enabled,
            "telemetry initialised without otlp"
        );

        return Ok(TelemetryGuard {
            sentry_enabled,
            #[cfg(feature = "sentry")]
            _sentry: sentry_guard,
        });
    }

    // OTel tracer → Cloud Trace (OTLP/HTTP)
    let endpoint = config
        .otlp_endpoint
        .clone()
        .unwrap_or_else(|| "https://cloudtrace.googleapis.com/v1/traces".to_string());
    let headers = otlp_headers_from_env();

    let exporter = opentelemetry_otlp::new_exporter()
        .http()
        .with_endpoint(endpoint.clone());
    let exporter = if headers.is_empty() {
        exporter
    } else {
        exporter.with_headers(headers)
    };

    let provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(exporter)
        .with_trace_config(sdktrace::Config::default().with_resource(
            opentelemetry_sdk::Resource::new(resource_attributes(&config)),
        ))
        .install_batch(runtime::Tokio)?;
    global::set_tracer_provider(provider.clone());
    let tracer = provider.tracer(config.service.clone());

    // JSON subscriber (→ Cloud Logging) + OTel layer + (optional) Sentry layer
    let registry = tracing_subscriber::registry()
        .with(filter)
        .with(json_log_layer())
        .with(OpenTelemetryLayer::new(tracer));

    #[cfg(feature = "sentry")]
    let registry = registry.with(sentry_tracing::layer());

    registry.init();

    tracing::info!(
        service = %config.service,
        env = %config.env,
        otlp_endpoint = %endpoint,
        sentry_enabled,
        "telemetry initialised"
    );

    Ok(TelemetryGuard {
        sentry_enabled,
        #[cfg(feature = "sentry")]
        _sentry: sentry_guard,
    })
}

fn json_log_layer<S>() -> tracing_subscriber::fmt::Layer<
    S,
    tracing_subscriber::fmt::format::JsonFields,
    tracing_subscriber::fmt::format::Format<tracing_subscriber::fmt::format::Json>,
>
where
    S: tracing::Subscriber + for<'span> tracing_subscriber::registry::LookupSpan<'span>,
{
    tracing_subscriber::fmt::layer()
        .json()
        .flatten_event(true)
        .with_current_span(true)
        .with_span_list(true)
}

fn resource_attributes(config: &TelemetryConfig) -> Vec<opentelemetry::KeyValue> {
    let mut attrs = vec![
        opentelemetry::KeyValue::new("service.name", config.service.clone()),
        opentelemetry::KeyValue::new("deployment.environment", config.env.clone()),
        opentelemetry::KeyValue::new("service.version", env!("CARGO_PKG_VERSION")),
        opentelemetry::KeyValue::new("service.namespace", "runtime-runway"),
    ];

    if let Some(raw) = non_empty_env("OTEL_RESOURCE_ATTRIBUTES") {
        attrs.extend(
            parse_key_values(&raw).map(|(key, value)| opentelemetry::KeyValue::new(key, value)),
        );
    }

    attrs
}

fn ensure_sentry_contract(config: &TelemetryConfig) -> Result<bool> {
    let sentry_enabled = !config.sentry_dsn.trim().is_empty();
    if is_production(&config.env) && !sentry_enabled {
        return Err(anyhow!(
            "SENTRY_DSN is required for production Runtime Runway services"
        ));
    }
    Ok(sentry_enabled)
}

fn is_production(env: &str) -> bool {
    matches!(env.trim(), "prod" | "production")
}

fn sentry_release(config: &TelemetryConfig) -> String {
    non_empty_env("SENTRY_RELEASE")
        .or_else(|| non_empty_env("GIT_SHA").map(|sha| format!("{}@{}", config.service, sha)))
        .unwrap_or_else(|| format!("{}@{}", config.service, env!("CARGO_PKG_VERSION")))
}

fn resolve_otlp_endpoint_from_env() -> Option<String> {
    non_empty_env("OTEL_EXPORTER_OTLP_TRACES_ENDPOINT")
        .or_else(|| non_empty_env("OTEL_EXPORTER_OTLP_ENDPOINT").map(trace_endpoint_from_base))
        .or_else(|| non_empty_env("OTLP_ENDPOINT"))
}

fn trace_endpoint_from_base(endpoint: String) -> String {
    let trimmed = endpoint.trim_end_matches('/');
    if trimmed.ends_with("/v1/traces") {
        trimmed.to_string()
    } else {
        format!("{trimmed}/v1/traces")
    }
}

fn otlp_headers_from_env() -> HashMap<String, String> {
    let mut headers = HashMap::new();
    if let Some(raw) = non_empty_env("OTEL_EXPORTER_OTLP_HEADERS") {
        headers.extend(parse_key_values(&raw));
    }
    if let Some(raw) = non_empty_env("OTEL_EXPORTER_OTLP_TRACES_HEADERS") {
        headers.extend(parse_key_values(&raw));
    }
    headers
}

fn parse_key_values(input: &str) -> impl Iterator<Item = (String, String)> + '_ {
    input.split(',').filter_map(|pair| {
        let (key, value) = pair.split_once('=')?;
        let key = key.trim();
        let value = value.trim();
        if key.is_empty() || value.is_empty() {
            None
        } else {
            Some((key.to_string(), value.to_string()))
        }
    })
}

fn non_empty_env(key: &str) -> Option<String> {
    std::env::var(key)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn appends_trace_path_to_base_otlp_endpoint() {
        assert_eq!(
            trace_endpoint_from_base("https://collector.example.com".to_string()),
            "https://collector.example.com/v1/traces"
        );
        assert_eq!(
            trace_endpoint_from_base("https://collector.example.com/".to_string()),
            "https://collector.example.com/v1/traces"
        );
        assert_eq!(
            trace_endpoint_from_base("https://collector.example.com/v1/traces".to_string()),
            "https://collector.example.com/v1/traces"
        );
    }

    #[test]
    fn parses_otlp_header_pairs() {
        let pairs: HashMap<_, _> =
            parse_key_values("Authorization=Bearer token, x-dash0-dataset = runway ").collect();
        assert_eq!(
            pairs.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
        assert_eq!(pairs.get("x-dash0-dataset"), Some(&"runway".to_string()));
    }

    #[test]
    fn production_requires_sentry_dsn() {
        let config = TelemetryConfig {
            service: "api-server".into(),
            env: "prod".into(),
            sentry_dsn: String::new(),
            otlp_endpoint: None,
        };

        assert!(ensure_sentry_contract(&config).is_err());
    }

    #[test]
    fn dev_allows_missing_sentry_dsn() {
        let config = TelemetryConfig {
            service: "api-server".into(),
            env: "dev".into(),
            sentry_dsn: String::new(),
            otlp_endpoint: None,
        };

        assert_eq!(ensure_sentry_contract(&config).unwrap(), false);
    }
}
