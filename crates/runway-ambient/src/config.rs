use std::time::Duration;

/// Runtime configuration for ambient workers and optional Pub/Sub notify.
#[derive(Debug, Clone)]
pub struct AmbientConfig {
    pub enabled: bool,
    pub poll_interval: Duration,
    pub max_attempts: u32,
    /// Full topic resource name: `projects/{project}/topics/{name}`
    pub pubsub_topic: Option<String>,
}

impl AmbientConfig {
    pub fn from_env() -> Self {
        let enabled = std::env::var("RUNWAY_AMBIENT_ENABLED")
            .map(|v| v == "true" || v == "1")
            .unwrap_or(false);
        let poll_ms = std::env::var("RUNWAY_AMBIENT_POLL_MS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1000);
        let max_attempts = std::env::var("RUNWAY_AMBIENT_MAX_ATTEMPTS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);
        let pubsub_topic = std::env::var("RUNWAY_AMBIENT_PUBSUB_TOPIC").ok();
        Self {
            enabled,
            poll_interval: Duration::from_millis(poll_ms),
            max_attempts,
            pubsub_topic,
        }
    }
}
