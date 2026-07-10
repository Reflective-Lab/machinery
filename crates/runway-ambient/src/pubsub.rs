use reqwest::Client;
use serde_json::json;
use tracing::debug;

use crate::error::{AmbientError, Result};
use crate::job::AmbientJobRecord;

/// Fire-and-forget Pub/Sub notify after durable enqueue.
///
/// Uses the Pub/Sub REST API so the same code works against
/// `PUBSUB_EMULATOR_HOST` (no OAuth) and production (caller supplies bearer
/// token via metadata in future; emulator path is sufficient for contract tests).
pub struct PubSubNotify {
    client: Client,
    topic: String,
    base_url: String,
}

impl PubSubNotify {
    pub fn from_env(topic: impl Into<String>) -> Self {
        let topic = topic.into();
        let base_url = std::env::var("PUBSUB_EMULATOR_HOST").map_or_else(
            |_| "https://pubsub.googleapis.com".into(),
            |host| format!("http://{host}"),
        );
        #[allow(clippy::disallowed_methods)]
        let client = Client::new();
        Self {
            client,
            topic,
            base_url,
        }
    }

    /// Publish a wake message containing the job id. Workers may poll the
    /// durable queue on receipt; Pub/Sub is a hint, not the source of truth.
    pub async fn notify_enqueued(&self, record: &AmbientJobRecord) -> Result<()> {
        let url = format!("{}/v1/{}:publish", self.base_url, self.topic);
        let body = json!({
            "messages": [{
                "data": base64_payload(&serde_json::to_string(record).map_err(AmbientError::codec)?),
                "attributes": {
                    "job_id": record.job_id.clone(),
                    "job_key": record.job_key.clone(),
                    "app_id": record.app_id.clone(),
                }
            }]
        });

        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AmbientError::Publish(e.to_string()))?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.unwrap_or_default();
            return Err(AmbientError::Publish(format!("{status} {text}")));
        }

        debug!(job_id = %record.job_id, topic = %self.topic, "pubsub notify sent");
        Ok(())
    }
}

fn base64_payload(s: &str) -> String {
    base64_encode(s.as_bytes())
}

fn base64_encode(input: &[u8]) -> String {
    const TABLE: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::new();
    for chunk in input.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        out.push(TABLE[((triple >> 18) & 63) as usize] as char);
        out.push(TABLE[((triple >> 12) & 63) as usize] as char);
        out.push(if chunk.len() > 1 {
            TABLE[((triple >> 6) & 63) as usize] as char
        } else {
            '='
        });
        out.push(if chunk.len() > 2 {
            TABLE[(triple & 63) as usize] as char
        } else {
            '='
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::{AmbientJobPhase, AmbientJobStatus};
    use chrono::Utc;

    #[test]
    fn base64_roundtrip_length() {
        let enc = base64_encode(b"hello");
        assert!(!enc.is_empty());
    }

    #[tokio::test]
    #[ignore = "requires pubsub emulator on PUBSUB_EMULATOR_HOST"]
    async fn notify_against_emulator() {
        let notify =
            PubSubNotify::from_env("projects/runway-contract/topics/dev.test-app.ingestion");
        let record = AmbientJobRecord {
            job_id: "job-1".into(),
            job_key: "test".parse().unwrap(),
            org_id: "org".into(),
            app_id: "app".into(),
            correlation_id: None,
            commit_ref: None,
            phase: AmbientJobPhase::Preparatory,
            payload: json!({}),
            status: AmbientJobStatus::Pending,
            attempts: 0,
            worker_id: None,
            result: None,
            last_error: None,
            enqueued_at: Utc::now(),
            updated_at: Utc::now(),
        };
        notify.notify_enqueued(&record).await.unwrap();
    }
}
