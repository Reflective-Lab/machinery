//! D5 SessionOwnership — admission-time lease middleware.
//!
//! See `runtime-runway/docs/superpowers/specs/2026-06-15-d5-session-ownership-design.md`.

use std::sync::OnceLock;

use uuid::Uuid;

/// Process-static lease holder ID. Computed once on first use as
/// `format!("{K_REVISION|local}:{uuid_v4}")`.
///
/// On Cloud Run, `K_REVISION` is set automatically per revision; locally it
/// falls back to "local". The uuid ensures uniqueness across instances of the
/// same revision (parallel deploys, scale-out).
pub fn process_holder_id() -> &'static str {
    static HOLDER: OnceLock<String> = OnceLock::new();
    HOLDER.get_or_init(|| {
        let revision = std::env::var("K_REVISION").unwrap_or_else(|_| "local".into());
        format!("{}:{}", revision, Uuid::new_v4())
    })
}

use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use axum::{
    Json,
    extract::{MatchedPath, Request},
    http::{Method, StatusCode},
    response::{IntoResponse, Response},
};
use runway_auth::AuthContext;
use runway_storage::{AcquireOutcome, LeaseScope, LeaseStore, RenewOutcome};
use serde_json::json;
use tokio::task::JoinHandle;
use tower::{Layer, Service};
use tracing::{info, warn};

/// Default lease TTL (60s) — covers Quorum's 5–30s formation runs with margin.
pub const DEFAULT_TTL: Duration = Duration::from_secs(60);
/// Default renewal interval (15s = TTL/4) — one missed renewal still leaves
/// 30s of headroom.
pub const DEFAULT_RENEW_INTERVAL: Duration = Duration::from_secs(15);

#[derive(Clone)]
pub struct SessionOwnershipLayer {
    app_id: String,
    path_param: String,
    ttl: Duration,
    renew_interval: Duration,
    holder_id: Option<String>,
    leases: Arc<dyn LeaseStore>,
}

impl SessionOwnershipLayer {
    pub fn for_app(app_id: impl Into<String>, leases: Arc<dyn LeaseStore>) -> Self {
        Self {
            app_id: app_id.into(),
            path_param: "id".into(),
            ttl: DEFAULT_TTL,
            renew_interval: DEFAULT_RENEW_INTERVAL,
            holder_id: None,
            leases,
        }
    }

    pub fn path_param(mut self, name: impl Into<String>) -> Self {
        self.path_param = name.into();
        self
    }

    pub fn ttl(mut self, ttl: Duration) -> Self {
        self.ttl = ttl;
        self
    }

    pub fn renew_interval(mut self, d: Duration) -> Self {
        self.renew_interval = d;
        self
    }

    /// Override the lease holder identity. Defaults to the process-static
    /// `process_holder_id()`, which is correct in production (one identity per
    /// process). Override only to model multiple instances inside a single
    /// process — e.g. in integration tests that share one runtime. This is
    /// identity configuration, not a softening switch: the lease check is
    /// enforced identically regardless of the value.
    pub fn holder_id(mut self, id: impl Into<String>) -> Self {
        self.holder_id = Some(id.into());
        self
    }
}

impl<S> Layer<S> for SessionOwnershipLayer {
    type Service = SessionOwnershipMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionOwnershipMiddleware {
            inner,
            app_id: self.app_id.clone(),
            path_param: self.path_param.clone(),
            ttl: self.ttl,
            renew_interval: self.renew_interval,
            holder_id: self.holder_id.clone(),
            leases: self.leases.clone(),
        }
    }
}

#[derive(Clone)]
pub struct SessionOwnershipMiddleware<S> {
    inner: S,
    app_id: String,
    path_param: String,
    ttl: Duration,
    renew_interval: Duration,
    holder_id: Option<String>,
    leases: Arc<dyn LeaseStore>,
}

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

impl<S> Service<Request> for SessionOwnershipMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<Result<Response, S::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let app_id = self.app_id.clone();
        let path_param = self.path_param.clone();
        let ttl = self.ttl;
        let renew_interval = self.renew_interval;
        let holder_override = self.holder_id.clone();
        let leases = self.leases.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // Safe methods do not mutate session state — pass through without
            // acquiring a lease so GET/POST on the same path can coexist under
            // one router group (D1 `route_*` merges methods per path).
            if matches!(
                req.method(),
                &Method::GET | &Method::HEAD | &Method::OPTIONS
            ) {
                return inner.call(req).await;
            }

            // 1. Extract org_id from AuthContext (which AuthLayer inserted upstream).
            let auth_ctx = match req.extensions().get::<AuthContext>() {
                Some(c) => c.clone(),
                None => {
                    return Ok(error_response(
                        StatusCode::BAD_REQUEST,
                        "ownership_requires_auth",
                        None,
                    ));
                }
            };
            let org_id = match auth_ctx.org_id() {
                Some(s) => s.to_string(),
                None => {
                    return Ok(error_response(
                        StatusCode::BAD_REQUEST,
                        "ownership_requires_org",
                        None,
                    ));
                }
            };

            // 2. Extract session_id from the matched path template via the
            //    configured param name. If the route template does not contain
            //    the param (e.g. POST /inquiry — a creation route with no {id}),
            //    pass through without ownership enforcement: the layer is only
            //    relevant for routes that address an existing session.
            let session_id = match extract_path_param(&req, &path_param) {
                Some(v) => v,
                None => return inner.call(req).await,
            };

            // 3. Build the scope and attempt to acquire.
            let scope = LeaseScope {
                org_id,
                app_id: app_id.clone(),
                session_id: session_id.clone(),
            };
            let holder_id = holder_override.unwrap_or_else(|| process_holder_id().to_string());

            let outcome = match leases.try_acquire(&scope, &holder_id, ttl).await {
                Ok(o) => o,
                Err(e) => {
                    warn!(
                        target: "runway_app_host::ownership",
                        scope = %scope.key(),
                        err = %e,
                        "lease_store_unavailable on try_acquire"
                    );
                    return Ok(error_response(
                        StatusCode::SERVICE_UNAVAILABLE,
                        "lease_store_unavailable",
                        None,
                    ));
                }
            };

            match outcome {
                AcquireOutcome::Acquired(_rec) => {
                    info!(
                        target: "runway_app_host::ownership",
                        scope = %scope.key(),
                        holder_id = %holder_id,
                        "ownership_acquired"
                    );
                    // Spawn background renew task; build LeaseGuard.
                    let guard = LeaseGuard::spawn(
                        leases.clone(),
                        scope.clone(),
                        holder_id.clone(),
                        ttl,
                        renew_interval,
                    );
                    let response = inner.call(req).await?;
                    drop(guard); // explicit drop after handler returns
                    Ok(response)
                }
                AcquireOutcome::HeldByOther(rec) => {
                    info!(
                        target: "runway_app_host::ownership",
                        scope = %scope.key(),
                        our_holder_id = %holder_id,
                        current_holder_id = %rec.holder_id,
                        expires_at = %rec.expires_at,
                        "ownership_held_by_other"
                    );
                    Ok(error_response(
                        StatusCode::CONFLICT,
                        "ownership_held",
                        Some((session_id, rec.expires_at.to_rfc3339())),
                    ))
                }
            }
        })
    }
}

fn extract_path_param(req: &Request, name: &str) -> Option<String> {
    // MatchedPath gives us the route template (e.g. "/inquiry/{id}/signal").
    // We then walk both the template and the actual URI path together to find
    // the value at the matching `{name}` segment.
    let matched = req.extensions().get::<MatchedPath>()?;
    let template = matched.as_str();
    let actual = req.uri().path();
    let needle = format!("{{{name}}}");
    let template_segments: Vec<&str> = template.split('/').collect();
    let actual_segments: Vec<&str> = actual.split('/').collect();
    if template_segments.len() != actual_segments.len() {
        return None;
    }
    for (t, a) in template_segments.iter().zip(actual_segments.iter()) {
        if *t == needle {
            return Some((*a).to_string());
        }
    }
    None
}

fn error_response(status: StatusCode, error: &str, held: Option<(String, String)>) -> Response {
    let body = match held {
        Some((session_id, holder_expires_at)) => json!({
            "error": error,
            "session_id": session_id,
            "holder_expires_at": holder_expires_at,
        }),
        None => json!({ "error": error }),
    };
    (status, Json(body)).into_response()
}

/// RAII guard wrapping an acquired lease. Spawns a background tokio task that
/// renews on `renew_interval`; on drop, aborts the task and fire-and-forgets a
/// release. v1 does not surface renewal loss to handlers (no fencing); a
/// future D5.1 will insert `SessionLeaseLost: watch::Receiver<()>` into request
/// extensions for opt-in graceful abort.
pub(crate) struct LeaseGuard {
    leases: Arc<dyn LeaseStore>,
    scope: LeaseScope,
    holder_id: String,
    renew_task: Option<JoinHandle<()>>,
}

impl LeaseGuard {
    pub(crate) fn spawn(
        leases: Arc<dyn LeaseStore>,
        scope: LeaseScope,
        holder_id: String,
        ttl: Duration,
        renew_interval: Duration,
    ) -> Self {
        let task_leases = leases.clone();
        let task_scope = scope.clone();
        let task_holder = holder_id.clone();
        let task = tokio::spawn(async move {
            let mut consecutive_errs: u32 = 0;
            loop {
                tokio::time::sleep(renew_interval).await;
                match task_leases.renew(&task_scope, &task_holder, ttl).await {
                    Ok(RenewOutcome::Renewed(_)) => {
                        consecutive_errs = 0;
                    }
                    Ok(RenewOutcome::Lost { current }) => {
                        info!(
                            target: "runway_app_host::ownership",
                            scope = %task_scope.key(),
                            our_holder_id = %task_holder,
                            current_holder_id = ?current.as_ref().map(|r| &r.holder_id),
                            "ownership_lost"
                        );
                        break;
                    }
                    Err(e) => {
                        consecutive_errs += 1;
                        warn!(
                            target: "runway_app_host::ownership",
                            scope = %task_scope.key(),
                            holder_id = %task_holder,
                            attempt = consecutive_errs,
                            err = %e,
                            "ownership_renew_transient_error"
                        );
                        if consecutive_errs >= 3 {
                            warn!(
                                target: "runway_app_host::ownership",
                                scope = %task_scope.key(),
                                "ownership_renew_giving_up_after_3_errors"
                            );
                            break;
                        }
                    }
                }
            }
        });
        Self {
            leases,
            scope,
            holder_id,
            renew_task: Some(task),
        }
    }
}

impl Drop for LeaseGuard {
    fn drop(&mut self) {
        if let Some(task) = self.renew_task.take() {
            task.abort();
        }
        // Fire-and-forget release. Cloning the Arc is cheap; we don't block
        // the response on Firestore latency.
        let leases = self.leases.clone();
        let scope = self.scope.clone();
        let holder = self.holder_id.clone();
        tokio::spawn(async move {
            if let Err(e) = leases.release(&scope, &holder).await {
                warn!(
                    target: "runway_app_host::ownership",
                    scope = %scope.key(),
                    holder_id = %holder,
                    err = %e,
                    "ownership_release_failed"
                );
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_holder_id_is_stable() {
        let a = process_holder_id().to_string();
        let b = process_holder_id().to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn process_holder_id_has_revision_prefix() {
        let id = process_holder_id();
        let (revision, _) = id.split_once(':').expect("colon separator");
        assert!(!revision.is_empty(), "revision must be non-empty");
    }
}
