//! Stripe provider adapter for Commerce Rails.
//!
//! Runtime Runway owns HTTP ingress, auth context, deployment config, and the
//! eventually-consistent identity mirror. This crate owns the Stripe-specific
//! provider calls, signature mechanics, webhook receipt construction, and
//! commercial event interpretation.

#![forbid(unsafe_code)]

use chrono::Utc;
use commerce_rails_contracts::{
    CommerceId, CustomerId, ProviderName, ReplayKey, Timestamp, WebhookReceipt,
    WebhookReceiptStatus,
};
use hmac::{Hmac, Mac};
use runway_storage::{Document, DocumentStore};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::Sha256;
use std::collections::HashMap;
use std::fmt::Write as _;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

const STRIPE_API_BASE: &str = "https://api.stripe.com/v1";
const WEBHOOK_TOLERANCE_SECONDS: i64 = 300;

/// Document-store collection: Firebase UID → `CustomerId` mapping.
/// Document.id = `firebase_uid`; data = `{ "customer_id": "..." }`.
const COLL_FIREBASE_TO_CUSTOMER: &str = "commerce.firebase_to_customer";

/// Document-store collection: `CustomerId` → `SubscriptionProjection`.
/// Document.id = `customer_id.as_str()`; data = serialized projection.
const COLL_CUSTOMER_PROJECTIONS: &str = "commerce.customer_projections";

/// Document-store collection: provider object refs → `CustomerId`. Used to
/// resolve a Stripe `cus_*` back to a CR-internal `CustomerId` on subsequent
/// webhooks. Document.id = `"{provider}:{object_id}"`; data = `{ "customer_id": "..." }`.
///
/// QF-CR-08: provider IDs never key the entitlement domain; they are
/// `ProviderObjectRef` values resolved at the adapter boundary into a
/// CR-owned `CustomerId`.
const COLL_PROVIDER_TO_CUSTOMER: &str = "commerce.provider_to_customer";

#[derive(Debug, thiserror::Error)]
pub enum CommerceRailsError {
    #[error("commerce configuration error: {0}")]
    Configuration(String),
    #[error("stripe provider error: {0}")]
    Provider(String),
    #[error("invalid stripe webhook JSON: {0}")]
    InvalidWebhookJson(String),
    #[error("entitlement storage error: {0}")]
    Storage(String),
}

impl CommerceRailsError {
    pub fn is_invalid_webhook_json(&self) -> bool {
        matches!(self, Self::InvalidWebhookJson(_))
    }
}

#[derive(Debug, Clone)]
pub struct CommerceRailsConfig {
    stripe: StripeConfig,
    signup_url: Option<String>,
    checkout_url: Option<String>,
    portal_url: Option<String>,
}

impl CommerceRailsConfig {
    pub fn new(
        stripe_webhook_secret: impl Into<String>,
        stripe_secret_key: impl Into<String>,
        stripe_price_team_monthly: impl Into<String>,
        stripe_price_starter_monthly: impl Into<String>,
    ) -> Self {
        Self {
            stripe: StripeConfig::new(
                stripe_webhook_secret,
                stripe_secret_key,
                stripe_price_team_monthly,
                stripe_price_starter_monthly,
            ),
            signup_url: None,
            checkout_url: None,
            portal_url: None,
        }
    }

    pub fn local() -> Self {
        Self::new("", "", "", "")
    }

    /// Sets the static signup URL surfaced by
    /// [`CommerceRails::entitlement_projection`] (QF-CR-05). Apps redirect
    /// not-yet-entitled users here instead of hard-coding the URL in the
    /// SPA.
    #[must_use]
    pub fn with_signup_url(mut self, url: impl Into<String>) -> Self {
        self.signup_url = Some(url.into());
        self
    }

    /// Sets the static checkout URL surfaced by
    /// [`CommerceRails::entitlement_projection`] (QF-CR-05). For dynamic
    /// per-customer checkout sessions, call
    /// [`CommerceRails::create_checkout_session`] directly.
    #[must_use]
    pub fn with_checkout_url(mut self, url: impl Into<String>) -> Self {
        self.checkout_url = Some(url.into());
        self
    }

    /// Sets the static billing-portal URL surfaced by
    /// [`CommerceRails::entitlement_projection`] (QF-CR-05). For dynamic
    /// per-customer portal sessions, call
    /// [`CommerceRails::create_portal_session`] directly.
    #[must_use]
    pub fn with_portal_url(mut self, url: impl Into<String>) -> Self {
        self.portal_url = Some(url.into());
        self
    }

    pub fn from_env(local_dev: bool) -> Result<Self, CommerceRailsError> {
        let stripe_webhook_secret = std::env::var("STRIPE_WEBHOOK_SECRET").unwrap_or_default();
        let stripe_secret_key = std::env::var("STRIPE_SECRET_KEY").unwrap_or_default();
        let stripe_price_team_monthly =
            std::env::var("STRIPE_PRICE_TEAM_MONTHLY").unwrap_or_default();
        let stripe_price_starter_monthly =
            std::env::var("STRIPE_PRICE_STARTER_MONTHLY").unwrap_or_default();

        if !local_dev {
            let missing = [
                ("STRIPE_WEBHOOK_SECRET", stripe_webhook_secret.as_str()),
                ("STRIPE_SECRET_KEY", stripe_secret_key.as_str()),
                (
                    "STRIPE_PRICE_TEAM_MONTHLY",
                    stripe_price_team_monthly.as_str(),
                ),
                (
                    "STRIPE_PRICE_STARTER_MONTHLY",
                    stripe_price_starter_monthly.as_str(),
                ),
            ]
            .into_iter()
            .filter_map(|(name, value)| value.trim().is_empty().then_some(name))
            .collect::<Vec<_>>();

            if !missing.is_empty() {
                return Err(CommerceRailsError::Configuration(format!(
                    "{} must be set in production",
                    missing.join(", ")
                )));
            }
        }

        let mut config = Self::new(
            stripe_webhook_secret,
            stripe_secret_key,
            stripe_price_team_monthly,
            stripe_price_starter_monthly,
        );

        // QF-CR-05: projection URLs are optional and surfaced via the
        // `EntitlementProjection` returned by `entitlement_projection()`.
        // Empty strings are treated as unset (consistent with the Stripe
        // env vars above).
        if let Ok(url) = std::env::var("CR_SIGNUP_URL")
            && !url.trim().is_empty()
        {
            config.signup_url = Some(url);
        }
        if let Ok(url) = std::env::var("CR_CHECKOUT_URL")
            && !url.trim().is_empty()
        {
            config.checkout_url = Some(url);
        }
        if let Ok(url) = std::env::var("CR_PORTAL_URL")
            && !url.trim().is_empty()
        {
            config.portal_url = Some(url);
        }

        Ok(config)
    }
}

#[derive(Debug, Clone)]
struct StripeConfig {
    webhook_secret: Option<String>,
    secret_key: Option<String>,
    price_team_monthly: Option<String>,
    price_starter_monthly: Option<String>,
}

impl StripeConfig {
    fn new(
        webhook_secret: impl Into<String>,
        secret_key: impl Into<String>,
        price_team_monthly: impl Into<String>,
        price_starter_monthly: impl Into<String>,
    ) -> Self {
        Self {
            webhook_secret: optional_string(webhook_secret),
            secret_key: optional_string(secret_key),
            price_team_monthly: optional_string(price_team_monthly),
            price_starter_monthly: optional_string(price_starter_monthly),
        }
    }

    fn plan_from_price_ids(&self, price_ids: &[&str]) -> BillingPlan {
        if let Some(team) = self.price_team_monthly.as_deref()
            && price_ids.contains(&team)
        {
            return BillingPlan::Team;
        }

        if let Some(starter) = self.price_starter_monthly.as_deref()
            && price_ids.contains(&starter)
        {
            return BillingPlan::Starter;
        }

        BillingPlan::Free
    }
}

fn optional_string(value: impl Into<String>) -> Option<String> {
    let value = value.into();
    if value.trim().is_empty() {
        None
    } else {
        Some(value)
    }
}

/// Callback invoked synchronously after a successful
/// [`CommerceRails::apply_webhook_action`] mutation. Registered via
/// [`CommerceRails::register_post_apply`].
pub type PostApplyCallback = Arc<dyn Fn(&CommerceWebhookAction) + Send + Sync>;

/// Registry of post-apply callbacks. Internal to `CommerceRails`; callers
/// hold a [`CallbackHandle`] that owns deregistration on drop.
#[derive(Default)]
struct PostApplyRegistry {
    next_id: AtomicU64,
    callbacks: Mutex<HashMap<u64, PostApplyCallback>>,
}

impl PostApplyRegistry {
    fn register(&self, callback: PostApplyCallback) -> u64 {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        if let Ok(mut map) = self.callbacks.lock() {
            map.insert(id, callback);
        }
        id
    }

    fn deregister(&self, id: u64) {
        if let Ok(mut map) = self.callbacks.lock() {
            map.remove(&id);
        }
    }

    /// Fires all registered callbacks synchronously. Snapshots the callback
    /// list under the lock and releases it before invoking — this prevents a
    /// callback that re-enters `CommerceRails` (e.g. to call `is_entitled`)
    /// from deadlocking on the registry mutex.
    fn fire(&self, action: &CommerceWebhookAction) {
        let snapshot: Vec<PostApplyCallback> = match self.callbacks.lock() {
            Ok(map) => map.values().cloned().collect(),
            Err(_) => return,
        };
        for cb in snapshot {
            cb(action);
        }
    }
}

/// Guard returned by [`CommerceRails::register_post_apply`]. Dropping the
/// guard deregisters the callback. Hold for the lifetime of the consumer
/// (e.g. `runway-accounts` stores it in the webhook-handler state).
pub struct CallbackHandle {
    id: u64,
    registry: Arc<PostApplyRegistry>,
}

impl Drop for CallbackHandle {
    fn drop(&mut self) {
        self.registry.deregister(self.id);
    }
}

#[derive(Clone)]
pub struct CommerceRails {
    stripe: StripeAdapter,
    entitlements: Arc<EntitlementStore>,
    post_apply: Arc<PostApplyRegistry>,
    /// Static URLs surfaced by [`Self::entitlement_projection`] (QF-CR-05).
    /// Cloned from `CommerceRailsConfig` at construction; cheap because the
    /// projection method is called only at shell init and entitlement
    /// transitions.
    signup_url: Option<String>,
    checkout_url: Option<String>,
    portal_url: Option<String>,
}

impl CommerceRails {
    /// Constructs a new `CommerceRails` instance.
    ///
    /// `store` is a [`DocumentStore`] used to persist the entitlement state
    /// across process restarts (QF-CR-03). For Tauri local development pass
    /// `runway_storage::StorageKit::local(path).await?.documents`; for Cloud
    /// Run pass `StorageKit::remote(config).await?.documents`. Hermetic unit
    /// tests construct a `StorageKit::local(tempfile::tempdir()?.path())`.
    ///
    /// Calling this constructor does no I/O — the store handle is captured but
    /// not touched until the first `is_entitled` / `apply_webhook_action`
    /// call.
    pub fn new(
        client: reqwest::Client,
        config: CommerceRailsConfig,
        store: Arc<dyn DocumentStore>,
    ) -> Self {
        let CommerceRailsConfig {
            stripe,
            signup_url,
            checkout_url,
            portal_url,
        } = config;
        Self {
            stripe: StripeAdapter::new(client, stripe),
            entitlements: Arc::new(EntitlementStore::new(store)),
            post_apply: Arc::new(PostApplyRegistry::default()),
            signup_url,
            checkout_url,
            portal_url,
        }
    }

    /// Registers a callback fired synchronously after every successful
    /// [`apply_webhook_action`](Self::apply_webhook_action) mutation. The
    /// callback receives a reference to the [`CommerceWebhookAction`] that
    /// caused the mutation.
    ///
    /// Callbacks do **not** fire for actions that result in no mutation —
    /// [`CommerceWebhookAction::Ignored`], `UpdateSubscriptionStatus` for an
    /// unknown customer, or any storage error.
    ///
    /// Returns a [`CallbackHandle`] that deregisters the callback on drop.
    /// Hold the handle for the consumer's lifetime — `runway-accounts`
    /// stores it in the webhook-handler state so the Firebase custom claim
    /// refresh fires on every webhook acceptance.
    ///
    /// **Mechanism** (panel-agreed, QF-CR-06): in-process synchronous
    /// callback, no event bus dependency, no Pub/Sub fan-out. Cross-instance
    /// coherence is handled by `EntitlementStore` persistence (QF-CR-03)
    /// plus RR's `refresh-on-403` client pattern.
    pub fn register_post_apply(&self, callback: PostApplyCallback) -> CallbackHandle {
        let id = self.post_apply.register(callback);
        CallbackHandle {
            id,
            registry: self.post_apply.clone(),
        }
    }

    pub fn is_billing_configured(&self) -> bool {
        self.stripe.is_configured()
    }

    pub async fn ensure_customer(
        &self,
        uid: &str,
        email: Option<&str>,
    ) -> Result<String, CommerceRailsError> {
        self.stripe.ensure_customer(uid, email).await
    }

    pub async fn create_checkout_session(
        &self,
        customer_ref: &str,
        price_ref: &str,
        success_url: &str,
        cancel_url: &str,
        firebase_uid: &str,
    ) -> Result<String, CommerceRailsError> {
        self.stripe
            .create_checkout_session(
                customer_ref,
                price_ref,
                "subscription",
                success_url,
                cancel_url,
                firebase_uid,
            )
            .await
    }

    pub async fn create_portal_session(
        &self,
        customer_ref: &str,
        return_url: &str,
    ) -> Result<String, CommerceRailsError> {
        self.stripe
            .create_portal_session(customer_ref, return_url)
            .await
    }

    pub fn verify_stripe_webhook_signature(&self, payload: &[u8], sig_header: &str) -> bool {
        self.stripe.verify_signature(payload, sig_header)
    }

    pub fn accept_stripe_webhook(
        &self,
        payload: &[u8],
    ) -> Result<AcceptedWebhook, CommerceRailsError> {
        self.stripe.accept_webhook(payload)
    }

    /// Apply a typed webhook action to the entitlement store. Returns true
    /// if state was mutated. Call this from the webhook HTTP handler after
    /// `accept_stripe_webhook` returns an `AcceptedWebhook`.
    ///
    /// Async because the entitlement store is backed by a persistent
    /// [`DocumentStore`] (QF-CR-03). Storage errors are logged via `tracing`
    /// and the method returns `false`; the webhook handler should observe its
    /// own tracing scope to decide whether to return 5xx to Stripe (which
    /// triggers a retry).
    pub async fn apply_webhook_action(&self, action: &CommerceWebhookAction) -> bool {
        let mutated = match self.entitlements.apply(action).await {
            Ok(m) => m,
            Err(err) => {
                tracing::error!(error = %err, "entitlement store apply failed");
                return false;
            }
        };
        if mutated {
            // QF-CR-06: fire post-apply callbacks synchronously *after* the
            // store mutation succeeds. Consumers (notably `runway-accounts`)
            // use this to refresh Firebase custom claims for the affected
            // `firebase_uid`. Callbacks for no-op actions (`Ignored` or
            // `UpdateSubscriptionStatus` against an unknown customer) do not
            // fire — there is nothing to refresh.
            self.post_apply.fire(action);
        }
        mutated
    }

    /// Returns true if the `firebase_uid` has an active subscription whose
    /// plan grants the named app entitlement.
    ///
    /// Lookup chain: `firebase_uid` → CR-internal [`CustomerId`] →
    /// [`SubscriptionProjection`]. Active = `subscription_status` is one of
    /// `"active"` or `"trialing"`. Apps come from `BillingPlan::apps()` —
    /// v1: every paid plan grants `"quorum"` (tracked as QF-CR-11).
    ///
    /// Async because the entitlement store is backed by a persistent
    /// [`DocumentStore`] (QF-CR-03). Fail-closed on storage error: returns
    /// `false` and logs via `tracing::error`. Result is valid only for the
    /// lifetime of the JWT that produced `firebase_uid`; never cache past
    /// JWT validity (Marquee App Contract rule 8).
    pub async fn is_entitled(&self, firebase_uid: &str, app: &str) -> bool {
        let customer_id = match self
            .entitlements
            .customer_id_for_firebase(firebase_uid)
            .await
        {
            Ok(Some(cid)) => cid,
            Ok(None) => return false,
            Err(err) => {
                tracing::error!(
                    error = %err,
                    firebase_uid,
                    "is_entitled: customer lookup failed"
                );
                return false;
            }
        };
        let projection = match self
            .entitlements
            .projection_for_customer(&customer_id)
            .await
        {
            Ok(Some(p)) => p,
            Ok(None) => return false,
            Err(err) => {
                tracing::error!(
                    error = %err,
                    customer_id = customer_id.as_str(),
                    "is_entitled: projection lookup failed"
                );
                return false;
            }
        };
        if !matches!(
            projection.subscription_status.as_str(),
            "active" | "trialing"
        ) {
            return false;
        }
        projection.plan.apps().iter().any(|a| a == app)
    }

    /// Returns the rich entitlement read for `firebase_uid` against `app`.
    /// Companion to [`Self::is_entitled`] (the hot-path bool gate); called
    /// at app-shell init and after entitlement transitions, not
    /// per-request (QF-CR-05; RR CR-OQ4 answer).
    ///
    /// The returned [`EntitlementProjection`] field set is panel-locked.
    /// Storage errors are logged via `tracing::error` and treated as
    /// "no projection found" — the response still includes any configured
    /// static URLs so the consuming app shell can render an unentitled
    /// state correctly.
    pub async fn entitlement_projection(
        &self,
        firebase_uid: &str,
        app: &str,
    ) -> EntitlementProjection {
        let customer_id = match self
            .entitlements
            .customer_id_for_firebase(firebase_uid)
            .await
        {
            Ok(opt) => opt,
            Err(err) => {
                tracing::error!(
                    error = %err,
                    firebase_uid,
                    "entitlement_projection: customer lookup failed"
                );
                None
            }
        };
        let projection = match customer_id.as_ref() {
            Some(cid) => match self.entitlements.projection_for_customer(cid).await {
                Ok(opt) => opt,
                Err(err) => {
                    tracing::error!(
                        error = %err,
                        customer_id = cid.as_str(),
                        "entitlement_projection: projection lookup failed"
                    );
                    None
                }
            },
            None => None,
        };

        let (entitled, next_renewal, plan_label) = match projection {
            Some(p) => {
                let active = matches!(p.subscription_status.as_str(), "active" | "trialing")
                    && p.plan.apps().iter().any(|a| a == app);
                let next_renewal = p
                    .current_period_end
                    .and_then(|secs| chrono::DateTime::<chrono::Utc>::from_timestamp(secs, 0));
                let plan_label = Some(p.plan.as_str().to_string());
                (active, next_renewal, plan_label)
            }
            None => (false, None, None),
        };

        EntitlementProjection {
            entitled,
            checkout_url: self.checkout_url.clone(),
            portal_url: self.portal_url.clone(),
            signup_url: self.signup_url.clone(),
            next_renewal,
            plan_label,
        }
    }
}

/// Entitlement state backed by a persistent [`DocumentStore`].
///
/// Holds three logical mappings, one per collection:
/// - [`COLL_FIREBASE_TO_CUSTOMER`]: `firebase_uid` → CR-internal [`CustomerId`]
/// - [`COLL_PROVIDER_TO_CUSTOMER`]: provider object ref → [`CustomerId`]
///   (Stripe `cus_*` resolution; mints a fresh `CustomerId` on first sight)
/// - [`COLL_CUSTOMER_PROJECTIONS`]: [`CustomerId`] → [`SubscriptionProjection`]
///
/// Quorum and other consumers do NOT access the store directly; they call
/// `CommerceRails::is_entitled` and `CommerceRails::entitlement_projection`
/// (the latter lands with QF-CR-05). The store is internal CR machinery.
///
/// Persistence policy (QF-CR-03): state survives process restart without
/// webhook replay. Backed by `runway-storage::DocumentStore` — `redb` in
/// Tauri local and Firestore in Cloud Run.
///
/// Identity policy (QF-CR-08): the store keys by CR-owned [`CustomerId`].
/// Provider IDs (Stripe `cus_*`) are kept only as resolver keys in
/// [`COLL_PROVIDER_TO_CUSTOMER`] and never as primary domain identifiers.
pub struct EntitlementStore {
    store: Arc<dyn DocumentStore>,
}

impl EntitlementStore {
    pub fn new(store: Arc<dyn DocumentStore>) -> Self {
        Self { store }
    }

    /// Returns the [`CustomerId`] linked to a `firebase_uid`, if any.
    pub async fn customer_id_for_firebase(
        &self,
        firebase_uid: &str,
    ) -> Result<Option<CustomerId>, CommerceRailsError> {
        let doc = self
            .store
            .get(COLL_FIREBASE_TO_CUSTOMER, firebase_uid)
            .await
            .map_err(|e| CommerceRailsError::Storage(e.to_string()))?;
        Ok(doc.and_then(|d| {
            d.data
                .get("customer_id")
                .and_then(|v| v.as_str())
                .map(CustomerId::new)
        }))
    }

    /// Returns the [`SubscriptionProjection`] for a [`CustomerId`], if any.
    pub async fn projection_for_customer(
        &self,
        customer_id: &CustomerId,
    ) -> Result<Option<SubscriptionProjection>, CommerceRailsError> {
        let doc = self
            .store
            .get(COLL_CUSTOMER_PROJECTIONS, customer_id.as_str())
            .await
            .map_err(|e| CommerceRailsError::Storage(e.to_string()))?;
        let Some(doc) = doc else { return Ok(None) };
        let value = serde_json::to_value(doc.data)
            .map_err(|e| CommerceRailsError::Storage(e.to_string()))?;
        let projection: SubscriptionProjection = serde_json::from_value(value)
            .map_err(|e| CommerceRailsError::Storage(e.to_string()))?;
        Ok(Some(projection))
    }

    /// Resolves a provider customer reference into a CR-owned [`CustomerId`],
    /// minting a fresh one on first sight. Stripe `cus_*` IDs are external
    /// references and never become primary domain identifiers (QF-CR-08).
    async fn resolve_or_mint_customer_id(
        &self,
        provider: ProviderName,
        external_id: &str,
    ) -> Result<CustomerId, CommerceRailsError> {
        let key = provider_resolver_key(&provider, external_id);
        if let Some(doc) = self
            .store
            .get(COLL_PROVIDER_TO_CUSTOMER, &key)
            .await
            .map_err(|e| CommerceRailsError::Storage(e.to_string()))?
            && let Some(s) = doc.data.get("customer_id").and_then(|v| v.as_str())
        {
            return Ok(CustomerId::new(s));
        }
        let minted = CustomerId::new(format!("customer:cr:{}", uuid::Uuid::new_v4()));
        let mapping = Document::new(&key, json!({ "customer_id": minted.as_str() }))
            .map_err(|e| CommerceRailsError::Storage(e.to_string()))?;
        self.store
            .put(COLL_PROVIDER_TO_CUSTOMER, mapping)
            .await
            .map_err(|e| CommerceRailsError::Storage(e.to_string()))?;
        Ok(minted)
    }

    /// Updates the store from a typed webhook action. Returns `Ok(true)` if
    /// state was mutated; `Ok(false)` for [`CommerceWebhookAction::Ignored`]
    /// or no-op cases (e.g. `UpdateSubscriptionStatus` for an unknown
    /// customer). Storage errors are surfaced via `Err`.
    pub async fn apply(&self, action: &CommerceWebhookAction) -> Result<bool, CommerceRailsError> {
        match action {
            CommerceWebhookAction::LinkCustomerRef {
                firebase_uid,
                customer_ref,
            } => {
                let cid = self
                    .resolve_or_mint_customer_id(ProviderName::StripeConnect, customer_ref)
                    .await?;
                let doc = Document::new(firebase_uid, json!({ "customer_id": cid.as_str() }))
                    .map_err(|e| CommerceRailsError::Storage(e.to_string()))?;
                self.store
                    .put(COLL_FIREBASE_TO_CUSTOMER, doc)
                    .await
                    .map_err(|e| CommerceRailsError::Storage(e.to_string()))?;
                Ok(true)
            }
            CommerceWebhookAction::ApplySubscriptionProjection {
                customer_ref,
                projection,
            } => {
                let cid = self
                    .resolve_or_mint_customer_id(ProviderName::StripeConnect, customer_ref)
                    .await?;
                let doc = Document::new(cid.as_str(), projection)
                    .map_err(|e| CommerceRailsError::Storage(e.to_string()))?;
                self.store
                    .put(COLL_CUSTOMER_PROJECTIONS, doc)
                    .await
                    .map_err(|e| CommerceRailsError::Storage(e.to_string()))?;
                Ok(true)
            }
            CommerceWebhookAction::UpdateSubscriptionStatus {
                customer_ref,
                subscription_status,
            } => {
                let cid = self
                    .resolve_or_mint_customer_id(ProviderName::StripeConnect, customer_ref)
                    .await?;
                let Some(mut projection) = self.projection_for_customer(&cid).await? else {
                    return Ok(false);
                };
                projection
                    .subscription_status
                    .clone_from(subscription_status);
                let doc = Document::new(cid.as_str(), &projection)
                    .map_err(|e| CommerceRailsError::Storage(e.to_string()))?;
                self.store
                    .put(COLL_CUSTOMER_PROJECTIONS, doc)
                    .await
                    .map_err(|e| CommerceRailsError::Storage(e.to_string()))?;
                Ok(true)
            }
            CommerceWebhookAction::Ignored => Ok(false),
        }
    }
}

fn provider_resolver_key(provider: &ProviderName, external_id: &str) -> String {
    let provider_tag = match provider {
        ProviderName::StripeConnect => "stripe_connect",
        ProviderName::Other(name) => name.as_str(),
    };
    format!("{provider_tag}:{external_id}")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BillingPlan {
    #[default]
    Free,
    Starter,
    Team,
    Enterprise,
}

impl BillingPlan {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Free => "free",
            Self::Starter => "starter",
            Self::Team => "team",
            Self::Enterprise => "enterprise",
        }
    }

    pub fn apps(self) -> Vec<String> {
        match self {
            Self::Free => Vec::new(),
            // v1: all paid plans grant Quorum (Reflective Labs single-app
            // subscription). When a second app ships, extend the variant
            // discriminant or the apps list per plan.
            Self::Starter | Self::Team | Self::Enterprise => vec!["quorum".to_string()],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct SubscriptionProjection {
    pub plan: BillingPlan,
    pub apps: Vec<String>,
    pub subscription_status: String,
    pub subscription_ref: Option<String>,
    pub current_period_end: Option<i64>,
}

impl SubscriptionProjection {
    fn updated(
        plan: BillingPlan,
        subscription_status: impl Into<String>,
        subscription_ref: impl Into<String>,
        current_period_end: Option<i64>,
    ) -> Self {
        Self {
            plan,
            apps: plan.apps(),
            subscription_status: subscription_status.into(),
            subscription_ref: Some(subscription_ref.into()),
            current_period_end,
        }
    }

    fn canceled() -> Self {
        Self {
            plan: BillingPlan::Free,
            apps: Vec::new(),
            subscription_status: "canceled".to_string(),
            subscription_ref: None,
            current_period_end: None,
        }
    }
}

/// Rich entitlement read returned by
/// [`CommerceRails::entitlement_projection`] (QF-CR-05). Companion to
/// [`CommerceRails::is_entitled`] (the hot-path bool gate).
///
/// **Field set is panel-locked** per the 2026-06-15 review (RR B2
/// amendment): adding new optional fields is non-breaking and does not
/// require panel re-review; renaming or removing a field requires a new
/// dated panel review. JSON Schema published at
/// `commerce-rails/kb/Contracts/EntitlementProjection.schema.json`.
///
/// Intended usage: called once at app-shell init and after every
/// entitlement transition (e.g. after a `refresh-on-403` retry). NOT
/// called per-request — for that, use `is_entitled`.
#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct EntitlementProjection {
    /// True iff the `firebase_uid` has an active subscription whose plan
    /// grants the named app.
    pub entitled: bool,
    /// Static checkout URL configured at the deploy boundary
    /// (`CR_CHECKOUT_URL` env or `CommerceRailsConfig::with_checkout_url`).
    /// For dynamic per-customer checkout sessions, call
    /// `CommerceRails::create_checkout_session` directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkout_url: Option<String>,
    /// Static billing-portal URL configured at the deploy boundary
    /// (`CR_PORTAL_URL` env or `CommerceRailsConfig::with_portal_url`).
    /// For dynamic per-customer portal sessions, call
    /// `CommerceRails::create_portal_session` directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub portal_url: Option<String>,
    /// Static signup URL configured at the deploy boundary
    /// (`CR_SIGNUP_URL` env or `CommerceRailsConfig::with_signup_url`).
    /// Apps show this to not-yet-entitled users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signup_url: Option<String>,
    /// Timestamp of the next subscription renewal (ISO 8601 in the JSON
    /// representation). Absent when there is no stored subscription
    /// projection for the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_renewal: Option<chrono::DateTime<chrono::Utc>>,
    /// Lowercase plan label (`"free"`, `"starter"`, `"team"`,
    /// `"enterprise"`). Absent when no stored projection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommerceWebhookAction {
    LinkCustomerRef {
        firebase_uid: String,
        customer_ref: String,
    },
    ApplySubscriptionProjection {
        customer_ref: String,
        projection: SubscriptionProjection,
    },
    UpdateSubscriptionStatus {
        customer_ref: String,
        subscription_status: String,
    },
    Ignored,
}

#[derive(Debug, Clone)]
pub struct AcceptedWebhook {
    pub receipt: WebhookReceipt,
    pub event_type: String,
    pub action: CommerceWebhookAction,
}

#[derive(Clone)]
struct StripeAdapter {
    client: reqwest::Client,
    config: StripeConfig,
}

impl StripeAdapter {
    fn new(client: reqwest::Client, config: StripeConfig) -> Self {
        Self { client, config }
    }

    fn is_configured(&self) -> bool {
        self.config.secret_key.is_some()
    }

    fn key(&self) -> Result<&str, CommerceRailsError> {
        self.config.secret_key.as_deref().ok_or_else(|| {
            CommerceRailsError::Provider("STRIPE_SECRET_KEY not configured".to_string())
        })
    }

    async fn find_customer_ref(&self, uid: &str) -> Result<Option<String>, CommerceRailsError> {
        let Some(key) = self.config.secret_key.as_deref() else {
            return Ok(None);
        };
        let query = format!("metadata['firebase_uid']:'{}'", uid.replace('\'', "\\'"));
        let resp = self
            .client
            .get(format!("{STRIPE_API_BASE}/customers/search"))
            .bearer_auth(key)
            .query(&[("query", query.as_str())])
            .send()
            .await
            .map_err(|e| CommerceRailsError::Provider(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(CommerceRailsError::Provider(format!(
                "customer search failed: {}",
                resp.status()
            )));
        }

        let list: StripeList<StripeCustomer> = resp
            .json()
            .await
            .map_err(|e| CommerceRailsError::Provider(e.to_string()))?;
        Ok(list.data.into_iter().next().map(|customer| customer.id))
    }

    async fn ensure_customer(
        &self,
        uid: &str,
        email: Option<&str>,
    ) -> Result<String, CommerceRailsError> {
        if let Some(id) = self.find_customer_ref(uid).await? {
            return Ok(id);
        }

        let key = self.key()?;
        let mut form: Vec<(&str, String)> = vec![("metadata[firebase_uid]", uid.to_string())];
        if let Some(email) = email {
            form.push(("email", email.to_string()));
        }

        let resp = self
            .client
            .post(format!("{STRIPE_API_BASE}/customers"))
            .bearer_auth(key)
            .form(&form)
            .send()
            .await
            .map_err(|e| CommerceRailsError::Provider(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(CommerceRailsError::Provider(format!(
                "customer creation failed: {}",
                resp.status()
            )));
        }

        let customer: StripeCustomer = resp
            .json()
            .await
            .map_err(|e| CommerceRailsError::Provider(e.to_string()))?;
        Ok(customer.id)
    }

    async fn create_checkout_session(
        &self,
        customer_ref: &str,
        price_ref: &str,
        mode: &str,
        success_url: &str,
        cancel_url: &str,
        firebase_uid: &str,
    ) -> Result<String, CommerceRailsError> {
        let key = self.key()?;
        let idempotency_key = format!("checkout_{firebase_uid}_{}", uuid::Uuid::new_v4());
        let form: Vec<(&str, &str)> = vec![
            ("mode", mode),
            ("customer", customer_ref),
            ("success_url", success_url),
            ("cancel_url", cancel_url),
            ("line_items[0][price]", price_ref),
            ("line_items[0][quantity]", "1"),
            ("client_reference_id", firebase_uid),
            ("metadata[firebase_uid]", firebase_uid),
            ("allow_promotion_codes", "true"),
        ];

        let resp = self
            .client
            .post(format!("{STRIPE_API_BASE}/checkout/sessions"))
            .bearer_auth(key)
            .header("Idempotency-Key", &idempotency_key)
            .form(&form)
            .send()
            .await
            .map_err(|e| CommerceRailsError::Provider(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(CommerceRailsError::Provider(format!(
                "checkout session failed: {}",
                resp.status()
            )));
        }

        let session: StripeCheckoutSession = resp
            .json()
            .await
            .map_err(|e| CommerceRailsError::Provider(e.to_string()))?;
        session
            .url
            .ok_or_else(|| CommerceRailsError::Provider("no URL in checkout response".to_string()))
    }

    async fn create_portal_session(
        &self,
        customer_ref: &str,
        return_url: &str,
    ) -> Result<String, CommerceRailsError> {
        let key = self.key()?;
        let form: Vec<(&str, &str)> = vec![("customer", customer_ref), ("return_url", return_url)];
        let resp = self
            .client
            .post(format!("{STRIPE_API_BASE}/billing_portal/sessions"))
            .bearer_auth(key)
            .form(&form)
            .send()
            .await
            .map_err(|e| CommerceRailsError::Provider(e.to_string()))?;

        if !resp.status().is_success() {
            return Err(CommerceRailsError::Provider(format!(
                "portal session failed: {}",
                resp.status()
            )));
        }

        let session: StripePortalSession = resp
            .json()
            .await
            .map_err(|e| CommerceRailsError::Provider(e.to_string()))?;
        Ok(session.url)
    }

    fn verify_signature(&self, payload: &[u8], sig_header: &str) -> bool {
        let Some(secret) = self.config.webhook_secret.as_deref() else {
            return true;
        };

        verify_stripe_signature(payload, sig_header, secret)
    }

    fn accept_webhook(&self, payload: &[u8]) -> Result<AcceptedWebhook, CommerceRailsError> {
        let event: Value = serde_json::from_slice(payload)
            .map_err(|e| CommerceRailsError::InvalidWebhookJson(e.to_string()))?;
        let event_id = event["id"]
            .as_str()
            .unwrap_or("missing-event-id")
            .to_string();
        let event_type = event["type"].as_str().unwrap_or("").to_string();
        let action = self.action_for_event(&event_type, &event);
        let receipt_status = if matches!(action, CommerceWebhookAction::Ignored) {
            WebhookReceiptStatus::Received
        } else {
            WebhookReceiptStatus::Accepted
        };

        Ok(AcceptedWebhook {
            receipt: WebhookReceipt {
                id: CommerceId::new(format!("webhook_receipt:stripe:{event_id}")),
                provider: ProviderName::StripeConnect,
                provider_event_id: event_id.clone(),
                replay_key: ReplayKey(format!("stripe:event:{event_id}")),
                signature_verified: true,
                received_at: Timestamp(Utc::now().to_rfc3339()),
                status: receipt_status,
            },
            event_type,
            action,
        })
    }

    fn action_for_event(&self, event_type: &str, event: &Value) -> CommerceWebhookAction {
        match event_type {
            "checkout.session.completed" => checkout_completed(event),
            "customer.subscription.created" | "customer.subscription.updated" => {
                self.subscription_updated(&event["data"]["object"])
            }
            "customer.subscription.deleted" => subscription_deleted(&event["data"]["object"]),
            "invoice.payment_failed" => invoice_payment_failed(&event["data"]["object"]),
            _ => CommerceWebhookAction::Ignored,
        }
    }

    fn subscription_updated(&self, subscription: &Value) -> CommerceWebhookAction {
        let Some(customer_ref) = subscription["customer"].as_str() else {
            tracing::warn!("subscription webhook missing customer");
            return CommerceWebhookAction::Ignored;
        };
        let Some(subscription_ref) = subscription["id"].as_str() else {
            tracing::warn!("subscription webhook missing subscription id");
            return CommerceWebhookAction::Ignored;
        };

        let status = subscription["status"].as_str().unwrap_or("unknown");
        let current_period_end = subscription["current_period_end"].as_i64();
        let price_ids: Vec<&str> = subscription["items"]["data"]
            .as_array()
            .map(|items| {
                items
                    .iter()
                    .filter_map(|item| item["price"]["id"].as_str())
                    .collect()
            })
            .unwrap_or_default();
        let plan = self.config.plan_from_price_ids(&price_ids);

        CommerceWebhookAction::ApplySubscriptionProjection {
            customer_ref: customer_ref.to_string(),
            projection: SubscriptionProjection::updated(
                plan,
                status,
                subscription_ref,
                current_period_end,
            ),
        }
    }
}

fn checkout_completed(event: &Value) -> CommerceWebhookAction {
    let session = &event["data"]["object"];
    let Some(firebase_uid) = session["client_reference_id"].as_str() else {
        tracing::warn!("checkout.session.completed missing client_reference_id");
        return CommerceWebhookAction::Ignored;
    };
    let Some(customer_ref) = session["customer"].as_str() else {
        tracing::warn!("checkout.session.completed missing customer");
        return CommerceWebhookAction::Ignored;
    };

    CommerceWebhookAction::LinkCustomerRef {
        firebase_uid: firebase_uid.to_string(),
        customer_ref: customer_ref.to_string(),
    }
}

fn subscription_deleted(subscription: &Value) -> CommerceWebhookAction {
    let Some(customer_ref) = subscription["customer"].as_str() else {
        tracing::warn!("subscription deleted webhook missing customer");
        return CommerceWebhookAction::Ignored;
    };

    CommerceWebhookAction::ApplySubscriptionProjection {
        customer_ref: customer_ref.to_string(),
        projection: SubscriptionProjection::canceled(),
    }
}

fn invoice_payment_failed(invoice: &Value) -> CommerceWebhookAction {
    let Some(customer_ref) = invoice["customer"].as_str() else {
        tracing::warn!("invoice payment failed webhook missing customer");
        return CommerceWebhookAction::Ignored;
    };

    CommerceWebhookAction::UpdateSubscriptionStatus {
        customer_ref: customer_ref.to_string(),
        subscription_status: "past_due".to_string(),
    }
}

fn verify_stripe_signature(payload: &[u8], sig_header: &str, secret: &str) -> bool {
    let mut timestamp: Option<&str> = None;
    let mut signatures: Vec<&str> = Vec::new();

    for part in sig_header.split(',') {
        let part = part.trim();
        if let Some(value) = part.strip_prefix("t=") {
            timestamp = Some(value);
        } else if let Some(value) = part.strip_prefix("v1=") {
            signatures.push(value);
        }
    }

    let Some(ts_str) = timestamp else {
        return false;
    };
    if signatures.is_empty() {
        return false;
    }
    let Ok(timestamp) = ts_str.parse::<i64>() else {
        return false;
    };
    if (Utc::now().timestamp() - timestamp).abs() > WEBHOOK_TOLERANCE_SECONDS {
        return false;
    }

    let signed_payload = format!("{ts_str}.{}", String::from_utf8_lossy(payload));
    let Ok(mut mac) = Hmac::<Sha256>::new_from_slice(secret.as_bytes()) else {
        return false;
    };
    mac.update(signed_payload.as_bytes());
    let expected = hex_lower(&mac.finalize().into_bytes());

    signatures.iter().any(|signature| {
        signature.len() == expected.len()
            && signature
                .bytes()
                .zip(expected.bytes())
                .fold(0_u8, |acc, (left, right)| acc | (left ^ right))
                == 0
    })
}

fn hex_lower(bytes: &[u8]) -> String {
    let mut output = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        write!(&mut output, "{byte:02x}").expect("writing to String cannot fail");
    }
    output
}

#[derive(Debug, Deserialize)]
struct StripeList<T> {
    data: Vec<T>,
}

#[derive(Debug, Deserialize)]
struct StripeCustomer {
    id: String,
}

#[derive(Debug, Deserialize)]
pub struct StripeSubscription {
    pub id: String,
    pub status: String,
    pub items: StripeSubscriptionItems,
    #[serde(default)]
    pub current_period_end: i64,
}

impl StripeSubscription {
    pub fn price_ids(&self) -> Vec<&str> {
        self.items
            .data
            .iter()
            .map(|item| item.price.id.as_str())
            .collect()
    }

    pub fn is_active(&self) -> bool {
        self.status == "active" || self.status == "trialing"
    }
}

#[derive(Debug, Deserialize)]
pub struct StripeSubscriptionItems {
    pub data: Vec<StripeSubscriptionItem>,
}

#[derive(Debug, Deserialize)]
pub struct StripeSubscriptionItem {
    pub price: StripePrice,
}

#[derive(Debug, Deserialize)]
pub struct StripePrice {
    pub id: String,
}

#[derive(Debug, Deserialize)]
struct StripeCheckoutSession {
    url: Option<String>,
}

#[derive(Debug, Deserialize)]
struct StripePortalSession {
    url: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use runway_storage::Query;
    use runway_storage::traits::Result as StorageResult;

    /// Inert `DocumentStore` for unit tests that exercise signature
    /// verification or webhook parsing only — paths that never touch the
    /// entitlement store. Integration tests in `tests/entitlement_store.rs`
    /// use a real `StorageKit::local(tempdir)` backend.
    struct NoopStore;

    #[async_trait]
    impl DocumentStore for NoopStore {
        async fn put(&self, _collection: &str, _doc: Document) -> StorageResult<()> {
            Ok(())
        }
        async fn get(&self, _collection: &str, _id: &str) -> StorageResult<Option<Document>> {
            Ok(None)
        }
        async fn delete(&self, _collection: &str, _id: &str) -> StorageResult<()> {
            Ok(())
        }
        async fn query(&self, _collection: &str, _q: Query) -> StorageResult<Vec<Document>> {
            Ok(vec![])
        }
    }

    fn rails() -> CommerceRails {
        let config = CommerceRailsConfig::new("whsec_test", "", "price_team", "price_starter");
        // RP-HERMETIC-UNIT (QF-2026-06-02-05): the tests that use `rails()`
        // exercise signature verification and webhook parsing only — they
        // never invoke the HTTP path, so the client here is a sentinel. If
        // a future test needs to hit a stubbed Stripe API, wire a stub
        // client (e.g. backed by `wiremock`) via the existing
        // `CommerceRails::new(client, config, store)` DI constructor.
        #[allow(clippy::disallowed_methods)]
        let client = reqwest::Client::new();
        CommerceRails::new(client, config, Arc::new(NoopStore))
    }

    fn signature_header(payload: &[u8], secret: &str) -> String {
        let timestamp = Utc::now().timestamp();
        let signed_payload = format!("{timestamp}.{}", String::from_utf8_lossy(payload));
        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(signed_payload.as_bytes());
        let signature = hex_lower(&mac.finalize().into_bytes());
        format!("t={timestamp},v1={signature}")
    }

    #[test]
    fn verifies_stripe_webhook_signature() {
        let payload = br#"{"id":"evt_test","type":"invoice.payment_failed"}"#;
        let header = signature_header(payload, "whsec_test");

        assert!(rails().verify_stripe_webhook_signature(payload, &header));
        assert!(!rails().verify_stripe_webhook_signature(payload, "t=1,v1=bad"));
    }

    #[test]
    fn maps_subscription_webhook_to_commercial_projection() {
        let payload = br#"{
            "id":"evt_sub_updated",
            "type":"customer.subscription.updated",
            "data":{
                "object":{
                    "id":"sub_123",
                    "customer":"cus_123",
                    "status":"active",
                    "current_period_end":12345,
                    "items":{"data":[{"price":{"id":"price_team"}}]}
                }
            }
        }"#;

        let webhook = rails().accept_stripe_webhook(payload).unwrap();

        assert_eq!(webhook.receipt.provider, ProviderName::StripeConnect);
        assert_eq!(webhook.receipt.provider_event_id, "evt_sub_updated");
        assert_eq!(webhook.receipt.status, WebhookReceiptStatus::Accepted);
        assert_eq!(
            webhook.action,
            CommerceWebhookAction::ApplySubscriptionProjection {
                customer_ref: "cus_123".to_string(),
                projection: SubscriptionProjection {
                    plan: BillingPlan::Team,
                    apps: vec!["quorum".to_string()],
                    subscription_status: "active".to_string(),
                    subscription_ref: Some("sub_123".to_string()),
                    current_period_end: Some(12345),
                },
            }
        );
    }

    #[test]
    fn maps_unknown_price_to_free_projection() {
        let payload = br#"{
            "id":"evt_sub_unknown",
            "type":"customer.subscription.updated",
            "data":{
                "object":{
                    "id":"sub_123",
                    "customer":"cus_123",
                    "status":"active",
                    "items":{"data":[{"price":{"id":"price_unknown"}}]}
                }
            }
        }"#;

        let webhook = rails().accept_stripe_webhook(payload).unwrap();

        assert!(matches!(
            webhook.action,
            CommerceWebhookAction::ApplySubscriptionProjection {
                projection: SubscriptionProjection {
                    plan: BillingPlan::Free,
                    ..
                },
                ..
            }
        ));
    }
}
