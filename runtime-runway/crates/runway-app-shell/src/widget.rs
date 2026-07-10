//! Shell widget contract — configuration the shared topbar consumes.

use serde::{Deserialize, Serialize};

/// Per-app shell identity wired at host bootstrap.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShellConfig {
    pub app_id: String,
    pub display_name: String,
    pub route_prefix: String,
    pub auth_domain: String,
    pub firebase_api_key: String,
}

/// Runtime state shared by shell Axum handlers.
#[derive(Debug, Clone)]
pub struct ShellState {
    pub config: ShellConfig,
}

impl ShellState {
    pub fn new(config: ShellConfig) -> Self {
        Self { config }
    }

    pub fn auth_bootstrap_config(&self) -> crate::auth::AuthBootstrapConfig {
        crate::auth::AuthBootstrapConfig {
            firebase_api_key: self.config.firebase_api_key.clone(),
            auth_domain: self.config.auth_domain.clone(),
        }
    }
}
