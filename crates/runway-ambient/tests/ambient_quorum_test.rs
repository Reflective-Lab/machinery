//! End-to-end: ambient HTTP surface wires enqueue route for any AmbientJobHandler.

use std::sync::Arc;

use async_trait::async_trait;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use runway_ambient::{AmbientError, AmbientJobHandler, AmbientJobRecord, AmbientJobs};
use runway_app_host::{
    AppExecutionPacket, JobKey, JobRegistration, RegistrationSource, RunwayAppHost,
};
use runway_auth::{AuthContext, FirebaseClaims};
use runway_storage::StorageKit;
use serde_json::{Value, json};
use tower::ServiceExt;

const JOB_TEST: &str = "test-job";

struct TestHandler;

#[async_trait]
impl AmbientJobHandler for TestHandler {
    fn job_keys(&self) -> &[JobKey] {
        static KEYS: std::sync::OnceLock<Vec<JobKey>> = std::sync::OnceLock::new();
        KEYS.get_or_init(|| vec![JOB_TEST.parse().expect("valid key")])
    }

    async fn execute(&self, _job: &AmbientJobRecord) -> Result<Value, AmbientError> {
        Ok(json!({ "ok": true }))
    }
}

fn dev_auth() -> AuthContext {
    AuthContext {
        claims: FirebaseClaims {
            uid: "dev-uid".into(),
            email: None,
            org_id: Some("dev-org".into()),
            apps: vec!["test-app".into()],
            role: None,
        },
    }
}

fn test_packet() -> AppExecutionPacket {
    AppExecutionPacket::new(
        "test-app",
        "Test App",
        "Generic test application",
        "/test-app",
    )
    .with_job(JobRegistration::new(
        JOB_TEST.parse().expect("valid key"),
        "Test job",
        RegistrationSource::AppDomain,
    ))
}

#[tokio::test]
async fn ambient_handler_mounts_enqueue_route() {
    let storage = StorageKit::local(tempfile::tempdir().unwrap().path())
        .await
        .unwrap();
    let handler = Arc::new(TestHandler);

    let host = RunwayAppHost::builder(test_packet())
        .with_storage(storage)
        .with_jobs_runtime(Box::new(AmbientJobs::with_handler(handler)))
        .build()
        .await
        .unwrap();

    let app = host.into_router();

    let mut req = Request::builder()
        .method("POST")
        .uri("/test-app/v1/ambient/jobs")
        .header("content-type", "application/json")
        .body(Body::from(
            json!({
                "job_key": JOB_TEST,
                "phase": "preparatory",
                "payload": {}
            })
            .to_string(),
        ))
        .unwrap();
    req.extensions_mut().insert(dev_auth());

    let resp = app.oneshot(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
