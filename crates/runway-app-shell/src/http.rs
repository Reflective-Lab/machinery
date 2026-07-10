//! Axum handlers for shell bootstrap and entitlement projection.

use axum::{
    Extension, Json,
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use runway_auth::AuthContext;

use crate::{auth::AuthBootstrapResponse, entitlement::EntitlementProjection, widget::ShellState};

/// `GET /v1/shell/auth-bootstrap` — Firebase config for SPA init.
pub async fn auth_bootstrap(State(state): State<ShellState>) -> Json<AuthBootstrapResponse> {
    Json(AuthBootstrapResponse::from_config(
        &state.config.app_id,
        &state.auth_bootstrap_config(),
    ))
}

/// `GET /v1/shell/entitlement` — entitlement projection for the signed-in user.
///
/// Stub: returns [`EntitlementProjection::not_entitled`] until Commerce Rails
/// resolver wiring lands in the host.
pub async fn entitlement(
    State(state): State<ShellState>,
    Extension(ctx): Extension<AuthContext>,
) -> Response {
    tracing::debug!(
        uid = %ctx.uid(),
        app_id = %state.config.app_id,
        "entitlement projection stub"
    );
    (StatusCode::OK, Json(EntitlementProjection::not_entitled())).into_response()
}
