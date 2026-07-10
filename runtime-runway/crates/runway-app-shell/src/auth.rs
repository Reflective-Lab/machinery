//! Firebase auth bootstrap types for SPA / Tauri webview init.

use serde::{Deserialize, Serialize};

/// Host-supplied Firebase web client configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthBootstrapConfig {
    pub firebase_api_key: String,
    pub auth_domain: String,
}

/// Payload returned to the shell SPA on mount.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthBootstrapResponse {
    pub app_id: String,
    pub firebase: FirebaseWebConfig,
}

/// Subset of Firebase web SDK config the shell needs for sign-in.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirebaseWebConfig {
    pub api_key: String,
    pub auth_domain: String,
}

impl AuthBootstrapResponse {
    pub fn from_config(app_id: impl Into<String>, config: &AuthBootstrapConfig) -> Self {
        Self {
            app_id: app_id.into(),
            firebase: FirebaseWebConfig {
                api_key: config.firebase_api_key.clone(),
                auth_domain: config.auth_domain.clone(),
            },
        }
    }
}
