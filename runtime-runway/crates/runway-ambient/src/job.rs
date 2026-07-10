use chrono::{DateTime, Utc};
use runway_app_host::JobKey;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Whether ambient work may start before a canonical commitment exists.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AmbientJobPhase {
    /// Pre-burst preparation (research, evidence, candidate options).
    Preparatory,
    /// Requires `commit_ref` — cannot enqueue without a recorded commitment.
    CommitBound,
}

/// Durable job lifecycle state.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum AmbientJobStatus {
    Pending,
    Running,
    Succeeded,
    Failed,
    DeadLetter,
}

/// Enqueue request — serializable on the wire and in Pub/Sub bodies.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AmbientJobRequest {
    pub job_key: JobKey,
    pub org_id: String,
    pub app_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_ref: Option<String>,
    pub phase: AmbientJobPhase,
    #[serde(default)]
    pub payload: Value,
}

impl AmbientJobRequest {
    pub fn preparatory(
        job_key: JobKey,
        org_id: impl Into<String>,
        app_id: impl Into<String>,
        payload: Value,
    ) -> Self {
        Self {
            job_key,
            org_id: org_id.into(),
            app_id: app_id.into(),
            correlation_id: None,
            commit_ref: None,
            phase: AmbientJobPhase::Preparatory,
            payload,
        }
    }

    pub fn commit_bound(
        job_key: JobKey,
        org_id: impl Into<String>,
        app_id: impl Into<String>,
        commit_ref: impl Into<String>,
        payload: Value,
    ) -> Self {
        Self {
            job_key,
            org_id: org_id.into(),
            app_id: app_id.into(),
            correlation_id: None,
            commit_ref: Some(commit_ref.into()),
            phase: AmbientJobPhase::CommitBound,
            payload,
        }
    }
}

/// Persisted job record in `_runway_ambient_jobs`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AmbientJobRecord {
    pub job_id: String,
    pub job_key: JobKey,
    pub org_id: String,
    pub app_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub commit_ref: Option<String>,
    pub phase: AmbientJobPhase,
    pub payload: Value,
    pub status: AmbientJobStatus,
    pub attempts: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub worker_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
    pub enqueued_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl AmbientJobRecord {
    pub fn from_request(request: AmbientJobRequest, job_id: String) -> Self {
        let now = Utc::now();
        Self {
            job_id,
            job_key: request.job_key,
            org_id: request.org_id,
            app_id: request.app_id,
            correlation_id: request.correlation_id,
            commit_ref: request.commit_ref,
            phase: request.phase,
            payload: request.payload,
            status: AmbientJobStatus::Pending,
            attempts: 0,
            worker_id: None,
            result: None,
            last_error: None,
            enqueued_at: now,
            updated_at: now,
        }
    }
}
