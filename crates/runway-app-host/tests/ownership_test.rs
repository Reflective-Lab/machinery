//! D5 Variant-1 integration tests: two routers wired to a single shared
//! redb-backed LeaseStore.
//!
//! The lease is a *per-request admission* primitive (spec sections 5/7/8): it is
//! acquired before the handler and released when the response completes. It does
//! NOT pin a session to an instance across sequential requests. Therefore
//! contention is tested with an *overlapping* handler lifetime — instance A's
//! handler is held open while instance B attempts the same scope — not with two
//! sequential requests (which, by design, may both succeed).

use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    routing::post,
};
use chrono::{DateTime, Utc};
use runway_app_host::SessionOwnershipLayer;
use runway_auth::AuthContext;
use runway_auth::FirebaseClaims;
use runway_storage::local::LocalStorageKit;
use runway_storage::{LeaseScope, LeaseStore};
use tempfile::TempDir;
use tokio::sync::Notify;
use tower::ServiceExt;

async fn inject_dev_auth(
    mut req: Request<Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    req.extensions_mut().insert(AuthContext {
        claims: FirebaseClaims {
            uid: "dev-uid".into(),
            email: Some("dev@local".into()),
            org_id: Some("org-1".into()),
            apps: vec!["test".into()],
            role: Some("admin".into()),
        },
    });
    next.run(req).await
}

fn signal_req_for(session: &str) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(format!("/inquiry/{session}/signal"))
        .body(Body::empty())
        .unwrap()
}

fn signal_req() -> Request<Body> {
    signal_req_for("inq-1")
}

/// The scope the middleware derives for `signal_req()` under the dev-auth org.
fn test_scope() -> LeaseScope {
    LeaseScope {
        org_id: "org-1".into(),
        app_id: "test".into(),
        session_id: "inq-1".into(),
    }
}

/// Router whose handler returns immediately. Used for the instance that should
/// observe contention (or steal an expired lease).
///
/// `holder` distinguishes this "instance" from another in the same process —
/// production uses the process-static `process_holder_id()`, but two in-process
/// routers would otherwise share it and never contend.
fn plain_router(
    leases: Arc<dyn LeaseStore>,
    holder: &str,
    ttl: Duration,
    renew: Duration,
) -> Router {
    let layer = SessionOwnershipLayer::for_app("test", leases)
        .path_param("id")
        .holder_id(holder)
        .ttl(ttl)
        .renew_interval(renew);
    Router::new()
        .route("/inquiry/{id}/signal", post(|| async { "ok" }))
        .layer(layer)
        .layer(axum::middleware::from_fn(inject_dev_auth))
}

/// Router whose handler signals on `entered` once it is running (lease held) and
/// then blocks until `release` is notified. Lets a test hold the lease open
/// across another instance's attempt.
fn gated_router(
    leases: Arc<dyn LeaseStore>,
    holder: &str,
    ttl: Duration,
    renew: Duration,
    entered: Arc<Notify>,
    release: Arc<Notify>,
) -> Router {
    let layer = SessionOwnershipLayer::for_app("test", leases)
        .path_param("id")
        .holder_id(holder)
        .ttl(ttl)
        .renew_interval(renew);
    Router::new()
        .route(
            "/inquiry/{id}/signal",
            post(move || {
                let entered = entered.clone();
                let release = release.clone();
                async move {
                    entered.notify_one();
                    release.notified().await;
                    "ok"
                }
            }),
        )
        .layer(layer)
        .layer(axum::middleware::from_fn(inject_dev_auth))
}

async fn json_body(resp: axum::response::Response) -> serde_json::Value {
    let bytes = axum::body::to_bytes(resp.into_body(), 4096).await.unwrap();
    serde_json::from_slice(&bytes).unwrap()
}

/// Variant-1 (Tier-1): while instance A's handler is in-flight (holding the
/// lease), instance B's concurrent request to the same scope returns 409 with
/// the documented body. Once A's response completes the lease is released, so
/// B's retry succeeds.
#[tokio::test(flavor = "current_thread")]
async fn concurrent_contention_returns_409_then_200_after_release() {
    let tmp = TempDir::new().unwrap();
    // One redb-backed kit; both routers share its LeaseStore handle — a valid
    // shared-store simulation within a single tokio runtime + process. (redb
    // takes an exclusive file lock, so a single process cannot open the same
    // file twice.)
    let kit = LocalStorageKit::build(tmp.path()).await.expect("kit");

    let entered = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    // Long TTL + long renew: the lease is freed by A's release-on-completion,
    // not by expiry, so this test isolates the release path.
    let host_a = gated_router(
        kit.leases.clone(),
        "instance-a",
        Duration::from_secs(30),
        Duration::from_secs(60),
        entered.clone(),
        release.clone(),
    );
    let host_b = plain_router(
        kit.leases.clone(),
        "instance-b",
        Duration::from_secs(30),
        Duration::from_secs(60),
    );

    // A's request acquires and parks inside the handler holding the lease.
    let a_task = tokio::spawn(async move { host_a.oneshot(signal_req()).await.unwrap() });
    entered.notified().await;

    // B, concurrent with A's in-flight handler, is rejected.
    let resp_b = host_b.clone().oneshot(signal_req()).await.unwrap();
    assert_eq!(resp_b.status(), StatusCode::CONFLICT);
    let body = json_body(resp_b).await;
    assert_eq!(body["error"], "ownership_held");
    assert_eq!(body["session_id"], "inq-1");
    assert!(body["holder_expires_at"].as_str().is_some());

    // Let A finish; its LeaseGuard drops and fires the release.
    release.notify_one();
    let resp_a = a_task.await.unwrap();
    assert_eq!(resp_a.status(), StatusCode::OK);

    // The release is fire-and-forget (spawned); poll until B can acquire.
    let mut status = StatusCode::CONFLICT;
    for _ in 0..200 {
        let resp = host_b.clone().oneshot(signal_req()).await.unwrap();
        status = resp.status();
        if status == StatusCode::OK {
            break;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    assert_eq!(status, StatusCode::OK, "B should acquire after A releases");
}

/// Variant-1 steal subtest (Tier-1): a holder that is stuck in its handler past
/// the TTL (renewal disabled) leaves an expired record; another instance steals
/// it and gets 200. This models the crash/pause path — the record still exists
/// (no release ran) but is expired.
#[tokio::test(flavor = "current_thread")]
async fn expired_holder_is_stolen_returns_200() {
    let tmp = TempDir::new().unwrap();
    let kit = LocalStorageKit::build(tmp.path()).await.expect("kit");

    let entered = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    // Short TTL, renew far in the future so the held lease actually expires.
    let host_a = gated_router(
        kit.leases.clone(),
        "instance-a",
        Duration::from_millis(200),
        Duration::from_secs(60),
        entered.clone(),
        release.clone(),
    );
    let host_b = plain_router(
        kit.leases.clone(),
        "instance-b",
        Duration::from_millis(200),
        Duration::from_secs(60),
    );

    let a_task = tokio::spawn(async move { host_a.oneshot(signal_req()).await.unwrap() });
    entered.notified().await;

    // Immediately: A holds an unexpired lease → B is rejected.
    let resp_b = host_b.clone().oneshot(signal_req()).await.unwrap();
    assert_eq!(resp_b.status(), StatusCode::CONFLICT);

    // A stays stuck in the handler (no release, no renew). After the TTL the
    // record is expired but still present.
    tokio::time::sleep(Duration::from_millis(350)).await;

    let resp_b2 = host_b.clone().oneshot(signal_req()).await.unwrap();
    assert_eq!(
        resp_b2.status(),
        StatusCode::OK,
        "B should steal the expired lease"
    );

    // Unblock A so the test runtime can drain.
    release.notify_one();
    let _ = a_task.await;
}

/// 30-second renewal-under-load (Tier-1), mocked clock. Instance A holds the
/// lease continuously (one long-running handler) while the renew task keeps it
/// alive. Across 30 simulated seconds instance B is rejected on every attempt,
/// and the holder's `expires_at` advances (proving renewal ran).
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn renewal_under_load_b_always_409_while_a_holds() {
    let tmp = TempDir::new().unwrap();
    let kit = LocalStorageKit::build(tmp.path()).await.expect("kit");

    let entered = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    // Spec defaults: ttl 60s, renew 15s.
    let host_a = gated_router(
        kit.leases.clone(),
        "instance-a",
        Duration::from_secs(60),
        Duration::from_secs(15),
        entered.clone(),
        release.clone(),
    );
    let host_b = plain_router(
        kit.leases.clone(),
        "instance-b",
        Duration::from_secs(60),
        Duration::from_secs(15),
    );

    // A acquires and parks in the handler, holding the lease for the run.
    let a_task = tokio::spawn(async move { host_a.oneshot(signal_req()).await.unwrap() });
    entered.notified().await;

    let scope = test_scope();
    let initial_expiry: DateTime<Utc> = kit
        .leases
        .current(&scope)
        .await
        .unwrap()
        .expect("A holds the lease")
        .expires_at;

    let mut b_409s: u32 = 0;
    let mut elapsed = Duration::ZERO;
    let step = Duration::from_secs(1);
    while elapsed < Duration::from_secs(30) {
        let resp_b = host_b.clone().oneshot(signal_req()).await.unwrap();
        assert_eq!(
            resp_b.status(),
            StatusCode::CONFLICT,
            "B must be rejected at t={elapsed:?} while A holds"
        );
        b_409s += 1;
        tokio::time::advance(step).await;
        elapsed += step;
    }

    let final_expiry: DateTime<Utc> = kit
        .leases
        .current(&scope)
        .await
        .unwrap()
        .expect("A still holds the lease")
        .expires_at;

    assert!(b_409s >= 30, "B got {b_409s} 409s across 30s");
    assert!(
        final_expiry > initial_expiry,
        "expires_at must advance via renewal: {initial_expiry} -> {final_expiry}"
    );

    release.notify_one();
    let _ = a_task.await;
}

/// Variant-2 (Tier-2, env-gated): same overlapping-contention scenario as
/// Variant-1, but against two independent `RemoteStorageKit` instances backed by
/// a Firestore emulator. Concurrency semantics are backend-independent. Skips
/// cleanly when `FIRESTORE_EMULATOR_HOST` is unset (default local `cargo test`).
#[tokio::test(flavor = "current_thread")]
async fn variant2_concurrent_contention_against_firestore_emulator() {
    if std::env::var("FIRESTORE_EMULATOR_HOST").is_err() {
        eprintln!("skipping: FIRESTORE_EMULATOR_HOST not set");
        return;
    }

    use runway_storage::remote::{RemoteConfig, RemoteStorageKit, TokenSource};

    let config = RemoteConfig {
        project_id: "d5-test".into(),
        region: "europe-west1".into(),
        bucket: "d5-test-bucket".into(),
        token_source: TokenSource::Static(String::new()),
    };
    // Two independent kits over the same emulator project — a genuine
    // two-instance fixture (not just a shared Arc).
    let kit_a = RemoteStorageKit::build(config.clone())
        .await
        .expect("kit a");
    let kit_b = RemoteStorageKit::build(config).await.expect("kit b");

    let entered = Arc::new(Notify::new());
    let release = Arc::new(Notify::new());
    let host_a = gated_router(
        kit_a.leases.clone(),
        "instance-a",
        Duration::from_secs(30),
        Duration::from_secs(60),
        entered.clone(),
        release.clone(),
    );
    let host_b = plain_router(
        kit_b.leases.clone(),
        "instance-b",
        Duration::from_secs(30),
        Duration::from_secs(60),
    );

    // Unique session id per run — the emulator persists state across runs
    // unless cleared.
    let session = format!("inq-{}", uuid::Uuid::new_v4());
    let session_a = session.clone();

    let a_task =
        tokio::spawn(async move { host_a.oneshot(signal_req_for(&session_a)).await.unwrap() });
    entered.notified().await;

    let resp_b = host_b
        .clone()
        .oneshot(signal_req_for(&session))
        .await
        .unwrap();
    assert_eq!(resp_b.status(), StatusCode::CONFLICT);
    let body = json_body(resp_b).await;
    assert_eq!(body["error"], "ownership_held");
    assert_eq!(body["session_id"], session);
    assert!(body["holder_expires_at"].as_str().is_some());

    release.notify_one();
    let resp_a = a_task.await.unwrap();
    assert_eq!(resp_a.status(), StatusCode::OK);

    let mut status = StatusCode::CONFLICT;
    for _ in 0..200 {
        let resp = host_b
            .clone()
            .oneshot(signal_req_for(&session))
            .await
            .unwrap();
        status = resp.status();
        if status == StatusCode::OK {
            break;
        }
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
    assert_eq!(status, StatusCode::OK, "B should acquire after A releases");
}
