//! Integration tests for `EntitlementStore` + `CommerceRails::is_entitled`.
//!
//! All tests are hermetic — no network, no Stripe API key required.
//! `CommerceRailsConfig::local()` leaves Stripe unconfigured; the
//! persistent `EntitlementStore` is backed by
//! `runway_storage::StorageKit::local(tempdir)` (redb under the hood).
//!
//! QF-CR-03: state survives process restart without webhook replay. The
//! final test (`entitlement_survives_process_restart`) is the acceptance
//! criterion for the persistent-store landing.
//! QF-CR-08: provider IDs (Stripe `cus_*`) are external references resolved
//! to a CR-owned `CustomerId` at the adapter boundary. None of the tests
//! observe `cus_*` strings on the consumer surface; the only API is
//! `is_entitled(uid, app)`.

use commerce_rails_stripe::{
    BillingPlan, CommerceRails, CommerceRailsConfig, CommerceWebhookAction, EntitlementProjection,
    SubscriptionProjection,
};
use runway_storage::StorageKit;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tempfile::TempDir;

/// Builds a fresh `CommerceRails` backed by a tempdir-scoped local
/// `StorageKit`. Returns the `TempDir` so the caller keeps it alive for
/// the duration of the test (dropping it removes the on-disk redb file).
async fn make_service() -> (TempDir, CommerceRails) {
    let dir = TempDir::new().expect("tempdir");
    let kit = StorageKit::local(dir.path()).await.expect("local kit");
    let config = CommerceRailsConfig::local();
    // RP-HERMETIC-UNIT: these tests never invoke the Stripe HTTP path; the
    // client is a sentinel required by `CommerceRails::new`. `local()`
    // leaves Stripe unconfigured so `is_billing_configured()` is false.
    #[allow(clippy::disallowed_methods)]
    let client = reqwest::Client::new();
    let rails = CommerceRails::new(client, config, kit.documents);
    (dir, rails)
}

/// A brand-new service with no state must deny every entitlement query.
#[tokio::test]
async fn fresh_service_says_not_entitled() {
    let (_dir, service) = make_service().await;
    assert!(!service.is_entitled("user-1", "quorum").await);
}

/// Linking a customer ref alone (no subscription yet) must not grant
/// entitlements — there is no `SubscriptionProjection` for the resolved
/// `CustomerId`.
#[tokio::test]
async fn link_customer_alone_does_not_entitle() {
    let (_dir, service) = make_service().await;
    service
        .apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
            firebase_uid: "user-1".to_string(),
            customer_ref: "cus_abc".to_string(),
        })
        .await;
    assert!(!service.is_entitled("user-1", "quorum").await);
}

/// `LinkCustomerRef` followed by an active Starter subscription must grant
/// the `"quorum"` entitlement and only that entitlement (v1 scope; tracked
/// as QF-CR-11 for the per-plan apps mapping).
#[tokio::test]
async fn link_plus_active_starter_grants_quorum() {
    let (_dir, service) = make_service().await;
    service
        .apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
            firebase_uid: "user-1".to_string(),
            customer_ref: "cus_abc".to_string(),
        })
        .await;
    service
        .apply_webhook_action(&CommerceWebhookAction::ApplySubscriptionProjection {
            customer_ref: "cus_abc".to_string(),
            projection: SubscriptionProjection {
                plan: BillingPlan::Starter,
                subscription_status: "active".to_string(),
                ..Default::default()
            },
        })
        .await;

    assert!(service.is_entitled("user-1", "quorum").await);
    // No other app is granted in v1.
    assert!(!service.is_entitled("user-1", "wolfgang").await);
}

/// After a subscription is canceled the entitlement must be revoked.
/// Sequence: link → active subscription (entitled) → cancel → not entitled.
#[tokio::test]
async fn canceled_subscription_revokes_entitlement() {
    let (_dir, service) = make_service().await;
    service
        .apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
            firebase_uid: "user-1".to_string(),
            customer_ref: "cus_abc".to_string(),
        })
        .await;
    service
        .apply_webhook_action(&CommerceWebhookAction::ApplySubscriptionProjection {
            customer_ref: "cus_abc".to_string(),
            projection: SubscriptionProjection {
                plan: BillingPlan::Starter,
                subscription_status: "active".to_string(),
                ..Default::default()
            },
        })
        .await;
    assert!(
        service.is_entitled("user-1", "quorum").await,
        "pre-condition: entitled before cancel"
    );

    service
        .apply_webhook_action(&CommerceWebhookAction::UpdateSubscriptionStatus {
            customer_ref: "cus_abc".to_string(),
            subscription_status: "canceled".to_string(),
        })
        .await;
    assert!(!service.is_entitled("user-1", "quorum").await);
}

/// `BillingPlan::Free` with an `"active"` status still must not grant any
/// app entitlements because `Free::apps()` returns an empty list.
#[tokio::test]
async fn free_plan_does_not_grant_quorum() {
    let (_dir, service) = make_service().await;
    service
        .apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
            firebase_uid: "user-1".to_string(),
            customer_ref: "cus_abc".to_string(),
        })
        .await;
    service
        .apply_webhook_action(&CommerceWebhookAction::ApplySubscriptionProjection {
            customer_ref: "cus_abc".to_string(),
            projection: SubscriptionProjection {
                plan: BillingPlan::Free,
                subscription_status: "active".to_string(),
                ..Default::default()
            },
        })
        .await;
    assert!(!service.is_entitled("user-1", "quorum").await);
}

/// QF-CR-03 acceptance: entitlement state survives a process restart
/// without webhook replay. Phase 1 sets up the entitlement and verifies it;
/// Phase 2 drops the `CommerceRails` (releasing the redb file handle) and
/// constructs a fresh one against the same on-disk path — the entitlement
/// must still resolve. This is the test the panel review committed CR to
/// before quorum-sense may run at `--max-instances > 1`.
#[tokio::test]
async fn entitlement_survives_process_restart() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().to_path_buf();

    // Phase 1 — original process lifecycle: link + activate.
    {
        let kit = StorageKit::local(&path).await.expect("local kit phase 1");
        let config = CommerceRailsConfig::local();
        #[allow(clippy::disallowed_methods)]
        let client = reqwest::Client::new();
        let service = CommerceRails::new(client, config, kit.documents);

        service
            .apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
                firebase_uid: "user-1".to_string(),
                customer_ref: "cus_abc".to_string(),
            })
            .await;
        service
            .apply_webhook_action(&CommerceWebhookAction::ApplySubscriptionProjection {
                customer_ref: "cus_abc".to_string(),
                projection: SubscriptionProjection {
                    plan: BillingPlan::Starter,
                    subscription_status: "active".to_string(),
                    ..Default::default()
                },
            })
            .await;

        assert!(
            service.is_entitled("user-1", "quorum").await,
            "pre-restart: entitled after setup"
        );
    } // drop service + kit; redb file handle releases.

    // Phase 2 — simulated restart: fresh service, same on-disk path. No
    // webhook replay, no manual state seeding.
    {
        let kit = StorageKit::local(&path).await.expect("local kit phase 2");
        let config = CommerceRailsConfig::local();
        #[allow(clippy::disallowed_methods)]
        let client = reqwest::Client::new();
        let service = CommerceRails::new(client, config, kit.documents);

        assert!(
            service.is_entitled("user-1", "quorum").await,
            "post-restart: entitlement survives without webhook replay"
        );
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// QF-CR-06: register_post_apply callback hook
// ─────────────────────────────────────────────────────────────────────────────

/// A registered callback fires synchronously after a successful
/// `apply_webhook_action` mutation. The closure's view of the action is the
/// same reference the store received.
#[tokio::test]
async fn register_post_apply_fires_callback_after_mutation() {
    let (_dir, service) = make_service().await;
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_for_cb = counter.clone();

    let _handle = service.register_post_apply(Arc::new(move |_action| {
        counter_for_cb.fetch_add(1, Ordering::SeqCst);
    }));

    service
        .apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
            firebase_uid: "user-1".to_string(),
            customer_ref: "cus_abc".to_string(),
        })
        .await;

    assert_eq!(counter.load(Ordering::SeqCst), 1, "callback fired once");
}

/// Callbacks must NOT fire for actions that produce no mutation
/// (`Ignored`, or `UpdateSubscriptionStatus` against an unknown customer).
/// There is nothing to refresh in those cases.
#[tokio::test]
async fn post_apply_callback_does_not_fire_on_no_op_actions() {
    let (_dir, service) = make_service().await;
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_for_cb = counter.clone();

    let _handle = service.register_post_apply(Arc::new(move |_action| {
        counter_for_cb.fetch_add(1, Ordering::SeqCst);
    }));

    // Ignored: explicit no-op variant.
    service
        .apply_webhook_action(&CommerceWebhookAction::Ignored)
        .await;
    assert_eq!(
        counter.load(Ordering::SeqCst),
        0,
        "Ignored action must not fire callbacks"
    );

    // UpdateSubscriptionStatus for an unknown customer: returns Ok(false)
    // from the store; should not fire callbacks.
    service
        .apply_webhook_action(&CommerceWebhookAction::UpdateSubscriptionStatus {
            customer_ref: "cus_unknown".to_string(),
            subscription_status: "canceled".to_string(),
        })
        .await;
    assert_eq!(
        counter.load(Ordering::SeqCst),
        0,
        "no-op UpdateSubscriptionStatus must not fire callbacks"
    );
}

/// Dropping the [`CallbackHandle`] deregisters the callback. Subsequent
/// `apply_webhook_action` calls do not invoke it.
#[tokio::test]
async fn post_apply_callback_deregisters_on_handle_drop() {
    let (_dir, service) = make_service().await;
    let counter = Arc::new(AtomicUsize::new(0));
    let counter_for_cb = counter.clone();

    {
        let _handle = service.register_post_apply(Arc::new(move |_action| {
            counter_for_cb.fetch_add(1, Ordering::SeqCst);
        }));

        service
            .apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
                firebase_uid: "user-1".to_string(),
                customer_ref: "cus_abc".to_string(),
            })
            .await;
        assert_eq!(counter.load(Ordering::SeqCst), 1, "fired while registered");
    } // _handle dropped here; callback deregistered.

    service
        .apply_webhook_action(&CommerceWebhookAction::ApplySubscriptionProjection {
            customer_ref: "cus_abc".to_string(),
            projection: SubscriptionProjection {
                plan: BillingPlan::Starter,
                subscription_status: "active".to_string(),
                ..Default::default()
            },
        })
        .await;
    assert_eq!(
        counter.load(Ordering::SeqCst),
        1,
        "did not fire after handle dropped"
    );
}

/// Multiple registered callbacks all fire on every successful mutation,
/// in some order — order is not specified by the contract.
#[tokio::test]
async fn multiple_post_apply_callbacks_all_fire() {
    let (_dir, service) = make_service().await;
    let counter_a = Arc::new(AtomicUsize::new(0));
    let counter_b = Arc::new(AtomicUsize::new(0));
    let a_for_cb = counter_a.clone();
    let b_for_cb = counter_b.clone();

    let _handle_a = service.register_post_apply(Arc::new(move |_action| {
        a_for_cb.fetch_add(1, Ordering::SeqCst);
    }));
    let _handle_b = service.register_post_apply(Arc::new(move |_action| {
        b_for_cb.fetch_add(1, Ordering::SeqCst);
    }));

    service
        .apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
            firebase_uid: "user-1".to_string(),
            customer_ref: "cus_abc".to_string(),
        })
        .await;

    assert_eq!(counter_a.load(Ordering::SeqCst), 1, "callback A fired once");
    assert_eq!(counter_b.load(Ordering::SeqCst), 1, "callback B fired once");
}

// ─────────────────────────────────────────────────────────────────────────────
// QF-CR-05: EntitlementProjection schema + entitlement_projection endpoint
// ─────────────────────────────────────────────────────────────────────────────

/// Builds a fresh service with the three projection URLs configured. The
/// store backs onto a tempdir-scoped local `StorageKit` like `make_service`.
async fn make_service_with_urls() -> (TempDir, CommerceRails) {
    let dir = TempDir::new().expect("tempdir");
    let kit = StorageKit::local(dir.path()).await.expect("local kit");
    let config = CommerceRailsConfig::local()
        .with_signup_url("https://example.test/signup")
        .with_checkout_url("https://example.test/checkout")
        .with_portal_url("https://example.test/portal");
    #[allow(clippy::disallowed_methods)]
    let client = reqwest::Client::new();
    let rails = CommerceRails::new(client, config, kit.documents);
    (dir, rails)
}

/// Entitled customer + active subscription: projection reports
/// `entitled = true`, plan label, next renewal, and any configured URLs.
#[tokio::test]
async fn entitlement_projection_entitled_includes_plan_label_and_next_renewal() {
    let (_dir, service) = make_service_with_urls().await;
    service
        .apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
            firebase_uid: "user-1".to_string(),
            customer_ref: "cus_abc".to_string(),
        })
        .await;
    service
        .apply_webhook_action(&CommerceWebhookAction::ApplySubscriptionProjection {
            customer_ref: "cus_abc".to_string(),
            projection: SubscriptionProjection {
                plan: BillingPlan::Team,
                subscription_status: "active".to_string(),
                current_period_end: Some(1_900_000_000), // 2030-03-17T01:46:40Z
                ..Default::default()
            },
        })
        .await;

    let projection = service.entitlement_projection("user-1", "quorum").await;

    assert!(
        projection.entitled,
        "entitled when active + plan grants app"
    );
    assert_eq!(projection.plan_label.as_deref(), Some("team"));
    assert!(projection.next_renewal.is_some(), "next_renewal populated");
    assert_eq!(
        projection.next_renewal.unwrap().timestamp(),
        1_900_000_000,
        "next_renewal round-trips through Unix epoch"
    );
    assert_eq!(
        projection.signup_url.as_deref(),
        Some("https://example.test/signup")
    );
    assert_eq!(
        projection.checkout_url.as_deref(),
        Some("https://example.test/checkout")
    );
    assert_eq!(
        projection.portal_url.as_deref(),
        Some("https://example.test/portal")
    );
}

/// Not-entitled UID: projection reports `entitled = false` and still
/// surfaces the configured signup URL so the SPA can route the user to
/// upgrade. No `plan_label` or `next_renewal` (no stored projection).
#[tokio::test]
async fn entitlement_projection_not_entitled_returns_static_urls() {
    let (_dir, service) = make_service_with_urls().await;

    let projection = service.entitlement_projection("user-fresh", "quorum").await;

    assert!(!projection.entitled);
    assert!(projection.plan_label.is_none());
    assert!(projection.next_renewal.is_none());
    assert_eq!(
        projection.signup_url.as_deref(),
        Some("https://example.test/signup"),
        "signup_url is present even for unentitled UIDs — that is its job"
    );
}

/// A service with no URL config returns `None` for all three URL fields,
/// not empty strings.
#[tokio::test]
async fn entitlement_projection_omits_unconfigured_urls() {
    let (_dir, service) = make_service().await;

    let projection = service.entitlement_projection("user-fresh", "quorum").await;

    assert!(!projection.entitled);
    assert!(projection.signup_url.is_none());
    assert!(projection.checkout_url.is_none());
    assert!(projection.portal_url.is_none());
}

/// JSON wire shape matches the panel-locked schema (RR B2). Optional
/// fields use `skip_serializing_if = "Option::is_none"` so absent fields
/// don't appear as `null` in the JSON. The locked field set is
/// `{ entitled, checkout_url?, portal_url?, signup_url?, next_renewal?, plan_label? }`.
#[tokio::test]
async fn entitlement_projection_serializes_with_locked_field_set() {
    let projection = EntitlementProjection {
        entitled: true,
        checkout_url: Some("https://example.test/checkout".to_string()),
        portal_url: None,
        signup_url: Some("https://example.test/signup".to_string()),
        next_renewal: chrono::DateTime::<chrono::Utc>::from_timestamp(1_900_000_000, 0),
        plan_label: Some("team".to_string()),
    };

    let json = serde_json::to_value(&projection).expect("serialize");

    let obj = json.as_object().expect("projection is a JSON object");
    assert_eq!(
        obj.get("entitled").and_then(serde_json::Value::as_bool),
        Some(true)
    );
    assert_eq!(
        obj.get("checkout_url").and_then(|v| v.as_str()),
        Some("https://example.test/checkout")
    );
    assert_eq!(
        obj.get("signup_url").and_then(|v| v.as_str()),
        Some("https://example.test/signup")
    );
    assert_eq!(obj.get("plan_label").and_then(|v| v.as_str()), Some("team"));
    assert!(
        obj.get("next_renewal").is_some(),
        "next_renewal serialized when populated"
    );
    assert!(
        !obj.contains_key("portal_url"),
        "absent portal_url not serialized as null"
    );

    // Round-trip preserves the value.
    let restored: EntitlementProjection = serde_json::from_value(json).expect("deserialize");
    assert_eq!(restored, projection);
}
