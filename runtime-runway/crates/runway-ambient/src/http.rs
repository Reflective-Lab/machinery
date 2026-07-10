use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
};
use runway_app_host::JobKey;
use runway_auth::AuthContext;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::config::AmbientConfig;
use crate::error::AmbientError;
use crate::job::{AmbientJobRecord, AmbientJobRequest};
use crate::pubsub::PubSubNotify;
use crate::queue::AmbientJobQueue;

#[derive(Clone)]
pub struct AmbientState {
    pub queue: AmbientJobQueue,
    pub app_id: String,
    pub registered_job_keys: Arc<Vec<JobKey>>,
    pub config: AmbientConfig,
}

/// Raw enqueue body. `job_key` stays [`String`] here — this is the wire
/// boundary; it is parsed into a [`JobKey`] before anything downstream sees it.
#[derive(Debug, Deserialize)]
pub struct EnqueueBody {
    pub job_key: String,
    #[serde(default)]
    pub correlation_id: Option<String>,
    #[serde(default)]
    pub commit_ref: Option<String>,
    pub phase: crate::job::AmbientJobPhase,
    #[serde(default)]
    pub payload: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnqueueResponse {
    pub job_id: String,
}

pub fn routes(state: AmbientState) -> Router {
    Router::new()
        .route("/v1/ambient/jobs", post(enqueue_job))
        .route("/v1/ambient/jobs/{job_id}", get(get_job))
        .with_state(state)
}

async fn enqueue_job(
    State(state): State<AmbientState>,
    auth: axum::Extension<AuthContext>,
    Json(body): Json<EnqueueBody>,
) -> Result<Json<EnqueueResponse>, AmbientHttpError> {
    let job_key: JobKey = body
        .job_key
        .parse()
        .map_err(|_| AmbientHttpError::InvalidJobKey(body.job_key.clone()))?;
    if !state.registered_job_keys.contains(&job_key) {
        return Err(AmbientHttpError::UnregisteredJobKey(job_key.into()));
    }

    let org_id = auth
        .org_id()
        .ok_or(AmbientHttpError::MissingOrg)?
        .to_string();

    let request = AmbientJobRequest {
        job_key,
        org_id,
        app_id: state.app_id.clone(),
        correlation_id: body.correlation_id,
        commit_ref: body.commit_ref,
        phase: body.phase,
        payload: body.payload,
    };

    let job_id = state.queue.enqueue(request).await?;
    if let Some(ref topic) = state.config.pubsub_topic
        && let Some(record) = state.queue.get(&job_id).await?
    {
        let notify = PubSubNotify::from_env(topic.clone());
        if let Err(e) = notify.notify_enqueued(&record).await {
            tracing::warn!(error = %e, job_id, "pubsub notify failed; job remains durable");
        }
    }

    Ok(Json(EnqueueResponse { job_id }))
}

async fn get_job(
    State(state): State<AmbientState>,
    auth: axum::Extension<AuthContext>,
    Path(job_id): Path<String>,
) -> Result<Json<AmbientJobRecord>, AmbientHttpError> {
    let record = state
        .queue
        .get(&job_id)
        .await?
        .ok_or(AmbientHttpError::NotFound(job_id))?;

    let org_id = auth.org_id().ok_or(AmbientHttpError::MissingOrg)?;
    if record.org_id != org_id {
        return Err(AmbientHttpError::NotFound(record.job_id));
    }

    Ok(Json(record))
}

#[derive(Debug)]
enum AmbientHttpError {
    NotFound(String),
    MissingOrg,
    /// The submitted `job_key` is not a well-formed key at all.
    InvalidJobKey(String),
    /// Well-formed key, but not registered in the app packet.
    UnregisteredJobKey(String),
    Inner(AmbientError),
}

impl From<AmbientError> for AmbientHttpError {
    fn from(value: AmbientError) -> Self {
        Self::Inner(value)
    }
}

impl IntoResponse for AmbientHttpError {
    fn into_response(self) -> axum::response::Response {
        let (status, body) = match self {
            Self::NotFound(id) => (
                StatusCode::NOT_FOUND,
                json!({ "error": "not_found", "job_id": id }),
            ),
            Self::MissingOrg => (StatusCode::FORBIDDEN, json!({ "error": "missing_org_id" })),
            Self::InvalidJobKey(key) => (
                StatusCode::BAD_REQUEST,
                json!({ "error": "invalid_job_key", "job_key": key }),
            ),
            Self::UnregisteredJobKey(key) => (
                StatusCode::BAD_REQUEST,
                json!({ "error": "unregistered_job_key", "job_key": key }),
            ),
            Self::Inner(AmbientError::MissingCommitRef) => (
                StatusCode::BAD_REQUEST,
                json!({ "error": "commit_bound_requires_commit_ref" }),
            ),
            Self::Inner(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({ "error": e.to_string() }),
            ),
        };
        (status, Json(body)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::Request;
    use runway_auth::{AuthContext, FirebaseClaims};
    use runway_storage::StorageKit;
    use tower::ServiceExt;

    fn dev_auth() -> AuthContext {
        AuthContext {
            claims: FirebaseClaims {
                uid: "dev-uid".into(),
                email: None,
                org_id: Some("dev-org".into()),
                apps: vec![],
                role: None,
            },
        }
    }

    #[tokio::test]
    async fn enqueue_and_get_via_http() {
        let kit = StorageKit::local(tempfile::tempdir().unwrap().path())
            .await
            .unwrap();
        let queue = AmbientJobQueue::new(kit.documents, kit.events, 3);
        let state = AmbientState {
            queue,
            app_id: "test-app".into(),
            registered_job_keys: Arc::new(vec!["sensemap-refresh".parse().unwrap()]),
            config: AmbientConfig::from_env(),
        };
        let app = routes(state);

        let mut req = Request::builder()
            .method("POST")
            .uri("/v1/ambient/jobs")
            .header("content-type", "application/json")
            .body(Body::from(
                serde_json::to_string(&json!({
                    "job_key": "sensemap-refresh",
                    "phase": "preparatory",
                    "payload": { "inquiry": "q-1" }
                }))
                .unwrap(),
            ))
            .unwrap();
        req.extensions_mut().insert(dev_auth());
        let resp = app.clone().oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);

        let body = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .unwrap();
        let enq: EnqueueResponse = serde_json::from_slice(&body).unwrap();

        let mut req = Request::builder()
            .uri(format!("/v1/ambient/jobs/{}", enq.job_id))
            .body(Body::empty())
            .unwrap();
        req.extensions_mut().insert(dev_auth());
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[tokio::test]
    async fn rejects_unregistered_job_key() {
        let kit = StorageKit::local(tempfile::tempdir().unwrap().path())
            .await
            .unwrap();
        let state = AmbientState {
            queue: AmbientJobQueue::new(kit.documents, kit.events, 3),
            app_id: "test-app".into(),
            registered_job_keys: Arc::new(vec![]),
            config: AmbientConfig::from_env(),
        };
        let app = routes(state);
        let mut req = Request::builder()
            .method("POST")
            .uri("/v1/ambient/jobs")
            .header("content-type", "application/json")
            .body(Body::from(
                json!({
                    "job_key": "unknown",
                    "phase": "preparatory"
                })
                .to_string(),
            ))
            .unwrap();
        req.extensions_mut().insert(dev_auth());
        let resp = app.oneshot(req).await.unwrap();
        assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    }
}
