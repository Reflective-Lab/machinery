# Observability Contract

Runtime Runway owns the observability envelope for deployed Reflective apps.
Apps and Bedrock libraries emit structured telemetry; Runtime Runway decides how
that telemetry is shipped, sampled, redacted, and routed.

This is a platform boundary, not a vendor choice.

## Decision

The Runtime Runway host initializes telemetry exactly once at process startup:

```rust
let _guard = runway_telemetry::init(TelemetryConfig::from_env("api-server"))?;
```

Callers hold the returned `TelemetryGuard` for the process lifetime. Apps,
Helm modules, and Bedrock libraries use `tracing` spans/events and must not
initialize Sentry, OpenTelemetry exporters, loggers, or vendor SDKs themselves.

## Sentry Contract

Sentry is the developer-support channel for runtime failures. It is used for
exceptions, high-value error events, release/environment context, breadcrumbs,
and incident triage.

Runtime Runway now treats Sentry as a real production contract:

- `runway-telemetry` enables its `sentry` feature by default.
- `ENV=prod` or `ENV=production` requires `SENTRY_DSN`; startup fails without it.
- Sentry release is `SENTRY_RELEASE`, then `{service}@{GIT_SHA}`, then
  `{service}@{runtime-runway-version}`.
- `send_default_pii` is disabled. Do not put prompts, secrets, credentials, raw
  user content, or unnecessary PII in tracing fields.
- Build-Depot consumes Sentry issue webhooks and aggregates them into Omnigraph
  `Incident` and `FactorySignal` records. Runtime Runway emits; Build-Depot
  normalizes.

## OpenTelemetry Contract

OpenTelemetry is the ops substrate. Runtime Runway uses it for vendor-neutral
telemetry routing and correlation.

Current implementation:

- exports traces over OTLP/HTTP;
- emits structured JSON logs to stdout for Cloud Logging;
- includes current span data in JSON logs;
- reads standard OTEL endpoint/header environment variables;
- keeps local `LOCAL_DEV=true` from constructing the OTLP HTTP exporter unless
  an OTLP endpoint is explicitly configured.

Next implementation target:

- promote runtime counters/histograms to OpenTelemetry metrics;
- decide whether app logs should flow through an OTel Collector, Cloud Logging
  sink, or direct OTel logs once the Rust logs path is mature enough for this
  repo's stability bar.

## Backend Routing

Cloud Trace and Cloud Logging are the current GCP defaults. Dash0, Honeycomb,
Grafana, Datadog, or another backend should be an OTLP configuration change, not
an app code change.

Use standard environment variables first:

```text
OTEL_SERVICE_NAME=api-server
OTEL_EXPORTER_OTLP_ENDPOINT=https://collector.example.com
OTEL_EXPORTER_OTLP_HEADERS=Authorization=Bearer <token>
OTEL_RESOURCE_ATTRIBUTES=deployment.environment=prod,service.namespace=runtime-runway
```

For backends that require a signal-specific endpoint, set:

```text
OTEL_EXPORTER_OTLP_TRACES_ENDPOINT=https://collector.example.com/v1/traces
OTEL_EXPORTER_OTLP_TRACES_HEADERS=Authorization=Bearer <token>
```

`OTLP_ENDPOINT` remains a legacy compatibility variable. New deployments should
prefer the `OTEL_*` names.

## App And Bedrock Rules

Apps own product meaning. Bedrock owns SDK and domain primitives. Neither owns
the process telemetry bootstrap.

They may:

- create spans around domain operations;
- attach stable, low-cardinality fields such as `app.id`, `org.id`,
  `job.key`, `route.owner`, `authority.effect`, and `receipt.family`;
- log durable business events through Runtime-owned storage/event-log contracts.

They must not:

- initialize `tracing_subscriber`, Sentry, OTLP exporters, or a collector;
- add app-local auth/secrets/telemetry bootstrap;
- emit raw prompts, secrets, provider credentials, tokens, or large user content;
- use high-cardinality values as metric labels.

## Build-Depot Boundary

Build-Depot owns the factory graph and Trigger.dev workers. It should consume
aggregated runtime signals from Sentry, GitHub, Linear, deployment checks, and
scheduled scans.

Runtime Runway should expose deployment and runtime facts that Build-Depot can
consume. It should not move app request-path execution into Trigger.dev, and
Build-Depot should not own runtime emission.

Deploy-learning uses the same logical Build-Depot OmniGraph. Runtime Runway does
not create a second graph database for ops learning; it emits summarized facts
that Build-Depot normalizes. See [[Deploy Ops Learning Contract]].

## Slack And Email Boundary

Slack and email provider connections belong in Runtime Runway when they are app
or operator notification transports: secrets, OAuth/provider config, delivery
retries, rate limits, audit metadata, and outbound runtime execution.

They are human notification channels, not the primary agent input bus. Agents
should act from source-near deployment, Sentry, OTel, Cloud Run, Firebase,
Runtime event-log, and Build-Depot graph signals.

Meaning stays elsewhere:

- Apps own user-facing copy and domain reason.
- Helm owns operator-control semantics and approval/audit shape.
- Build-Depot owns factory alerts generated from graph findings or incidents.
