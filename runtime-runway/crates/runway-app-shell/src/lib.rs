//! Shared marquee app shell — entitlement projection, auth bootstrap, widget contract.
//!
//! See the crate README for the integration contract apps must follow when
//! mounting shell routes on a [`runway_app_host::RunwayAppHost`].

mod auth;
mod http;
mod widget;

pub mod entitlement;

pub use auth::{AuthBootstrapConfig, AuthBootstrapResponse, FirebaseWebConfig};
pub use entitlement::EntitlementProjection;
pub use widget::{ShellConfig, ShellState};

use axum::{Router, routing};

/// Public shell routes — no auth required.
pub fn public_routes(state: ShellState) -> Router {
    Router::new()
        .route(
            "/v1/shell/auth-bootstrap",
            routing::get(http::auth_bootstrap),
        )
        .with_state(state)
}

/// Protected shell routes — caller must supply a valid Firebase Bearer token.
pub fn protected_routes(state: ShellState) -> Router {
    Router::new()
        .route("/v1/shell/entitlement", routing::get(http::entitlement))
        .with_state(state)
}

/// All shell routes. Merge `public_routes` and `protected_routes` separately if
/// your host applies `AuthLayer` only to protected paths.
pub fn routes(state: ShellState) -> Router {
    let public = public_routes(state.clone());
    let protected = protected_routes(state);
    public.merge(protected)
}
