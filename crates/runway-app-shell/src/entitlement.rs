//! Panel-locked entitlement projection schema (2026-06-15).
//!
//! See `kb/Architecture/App Execution Container.md` for the canonical contract.
//! Adding optional fields is non-breaking; renaming or removing fields requires
//! a new dated panel review.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Projection returned by `entitlement_projection(uid, app_id)`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EntitlementProjection {
    pub entitled: bool,
    pub checkout_url: Option<String>,
    pub portal_url: Option<String>,
    pub signup_url: Option<String>,
    pub next_renewal: Option<DateTime<Utc>>,
    pub plan_label: Option<String>,
}

impl EntitlementProjection {
    /// Conservative default used until Commerce Rails wiring is live.
    pub fn not_entitled() -> Self {
        Self {
            entitled: false,
            checkout_url: None,
            portal_url: None,
            signup_url: None,
            next_renewal: None,
            plan_label: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serde_roundtrip_preserves_locked_fields() {
        let projection = EntitlementProjection {
            entitled: true,
            checkout_url: Some("https://checkout.example/session".into()),
            portal_url: Some("https://billing.example/portal".into()),
            signup_url: None,
            next_renewal: Some(Utc::now()),
            plan_label: Some("Pro".into()),
        };

        let json = serde_json::to_string(&projection).unwrap();
        let parsed: EntitlementProjection = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed, projection);
    }
}
