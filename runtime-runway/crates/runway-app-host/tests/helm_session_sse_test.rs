// RFL-128: SSE mount integration test moved from helms/helm-session-host to
// break the circular CI dependency (helms CI checks out runtime-runway main;
// putting the test here avoids the HelmModule type-identity mismatch).

use std::sync::Arc;

use async_trait::async_trait;
use axum::Router;
use axum::routing::get;
use runway_app_host::{
    AppExecutionPacket, EventHub, HelmModule, ModuleState, MountKind, MountedModule, RouteOwner,
    RouteRegistration, RunwayAppHost,
};
use runway_storage::StorageKit;

struct StubSseModule;

#[async_trait]
impl HelmModule for StubSseModule {
    fn module_id(&self) -> &'static str {
        "helm.session-host"
    }

    fn module_state(&self) -> ModuleState {
        ModuleState::Live
    }

    fn router(self: Arc<Self>) -> Router {
        Router::new().route(
            "/v1/sessions/{session_id}/stream",
            get(|| async {
                (
                    [(
                        axum::http::header::CONTENT_TYPE,
                        "text/event-stream; charset=utf-8",
                    )],
                    "data: ping\n\n",
                )
            }),
        )
    }
}

fn session_host_packet() -> AppExecutionPacket {
    AppExecutionPacket::new(
        "test.session-host",
        "Session Host Mount Test",
        "Pins live session-host SSE on RunwayAppHost",
        "",
    )
    .with_mounted_module(MountedModule {
        module_id: "helm.session-host".into(),
        mount_kind: MountKind::Mounted,
        routes: vec![RouteRegistration {
            method: "GET".into(),
            path: "/v1/sessions/{session_id}/stream".into(),
            owner: RouteOwner::HelmModule,
        }],
    })
}

#[tokio::test]
async fn runway_host_mounts_helm_module_sse_route() {
    let dir = tempfile::tempdir().expect("tempdir");
    let storage = StorageKit::local(dir.path()).await.expect("local storage");

    let _hub = EventHub::with_capacity(256);
    let module = Arc::new(StubSseModule);

    let router = RunwayAppHost::builder(session_host_packet())
        .with_storage(storage)
        .mount(module)
        .build()
        .await
        .expect("host builds")
        .into_router();

    let response = tower::ServiceExt::oneshot(
        router,
        axum::http::Request::builder()
            .uri("/v1/sessions/sess-mount-1/stream")
            .body(axum::body::Body::empty())
            .unwrap(),
    )
    .await
    .unwrap();

    assert_eq!(response.status(), axum::http::StatusCode::OK);
    let ct = response.headers().get("content-type").unwrap();
    assert!(ct.to_str().unwrap().starts_with("text/event-stream"));
}
