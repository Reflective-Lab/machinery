//! Reflective Commerce Rails contract vocabulary.
//!
//! These are Reflective business concepts. Payment providers such as Stripe
//! Connect are adapters behind this surface, not the domain model.
//!
//! The broader rail control model uses movement terminology: mainspring for
//! accumulated commercial force, gear train for sequencing, escapement for
//! controlled release, balance for regulation, caliber for precision profiles,
//! and complication for optional advanced commerce behavior.

#![forbid(unsafe_code)]

/// Stable Commerce Rails identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommerceId(pub String);

impl CommerceId {
    /// Creates a new identifier value.
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// CR-internal customer identity.
///
/// Opaque, CR-owned, provider-independent. The `EntitlementStore` keys by
/// `CustomerId`, never by a provider-specific reference such as a Stripe
/// `cus_*` ID. Provider references for a given customer live separately as
/// [`ProviderObjectRef`] values resolved at the adapter boundary.
///
/// Wraps a [`CommerceId`] so the broader Commerce Rails identity machinery
/// (string-stable, hashable, serde-friendly via `as_str`) applies uniformly.
/// Use the newtype at API boundaries to prevent accidental cross-entity
/// substitution (e.g. passing a subscription id where a customer id is
/// expected).
///
/// Tracked landing: `QF-CR-08` (boundary debt) — see
/// `commerce-rails/QUALITY_BACKLOG.md`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CustomerId(CommerceId);

impl CustomerId {
    /// Creates a new customer identity from a raw string value.
    ///
    /// Callers should prefer prefixed string forms like `"customer:cr:..."`
    /// for self-describing identifiers, mirroring the convention used by
    /// other [`CommerceId`]-backed entities in this crate.
    pub fn new(value: impl Into<String>) -> Self {
        Self(CommerceId::new(value))
    }

    /// Wraps an existing [`CommerceId`] as a `CustomerId`.
    pub fn from_commerce_id(id: CommerceId) -> Self {
        Self(id)
    }

    /// Returns the underlying [`CommerceId`].
    pub fn as_commerce_id(&self) -> &CommerceId {
        &self.0
    }

    /// Returns the identifier as a string slice.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// External provider reference. Never use this as the Commerce Rails primary id.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ProviderObjectRef {
    pub provider: ProviderName,
    pub object_type: String,
    pub object_id: String,
}

/// Payment or commerce provider name.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ProviderName {
    StripeConnect,
    Other(String),
}

/// Idempotency key for consequential commands.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct IdempotencyKey(pub String);

/// Replay key for webhook and provider-event deduplication.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ReplayKey(pub String);

/// Timestamp encoded at the application boundary.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Timestamp(pub String);

/// ISO-4217 currency code.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CurrencyCode(pub String);

/// Money represented in minor units for deterministic accounting.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MoneyAmount {
    pub currency: CurrencyCode,
    pub minor_units: i64,
}

/// Reflective-owned commercial account.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReflectiveAccount {
    pub id: CommerceId,
    pub display_name: String,
    pub status: AccountStatus,
}

/// Customer organization buying or installing apps through Reflective.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomerOrg {
    pub id: CommerceId,
    pub legal_name: String,
    pub status: AccountStatus,
}

/// Builder account that can create app listings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuilderAccount {
    pub id: CommerceId,
    pub display_name: String,
    pub status: AccountStatus,
}

/// Partner account that can receive revenue-share payouts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PartnerAccount {
    pub id: CommerceId,
    pub display_name: String,
    pub status: AccountStatus,
    pub provider_refs: Vec<ProviderObjectRef>,
}

/// Commercial account lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountStatus {
    Pending,
    Active,
    Suspended,
    Closed,
}

/// App listed for purchase or installation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppListing {
    pub id: CommerceId,
    pub partner_account_id: CommerceId,
    pub slug: String,
    pub display_name: String,
    pub status: ListingStatus,
}

/// Listing lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListingStatus {
    Draft,
    Review,
    Listed,
    Suspended,
    Retired,
}

/// Installed app for a customer org.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppInstallation {
    pub id: CommerceId,
    pub app_listing_id: CommerceId,
    pub customer_org_id: CommerceId,
    pub status: InstallationStatus,
}

/// Installation lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InstallationStatus {
    Pending,
    Active,
    Suspended,
    Removed,
}

/// Sellable plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    pub id: CommerceId,
    pub app_listing_id: CommerceId,
    pub display_name: String,
    pub billing_interval: BillingInterval,
}

/// Billing cadence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BillingInterval {
    Month,
    Year,
    Usage,
    OneTime,
}

/// Price for a plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Price {
    pub id: CommerceId,
    pub plan_id: CommerceId,
    pub amount: MoneyAmount,
    pub status: PriceStatus,
}

/// Price lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PriceStatus {
    Active,
    Archived,
}

/// Customer subscription owned by Commerce Rails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subscription {
    pub id: CommerceId,
    pub customer_org_id: CommerceId,
    pub app_installation_id: CommerceId,
    pub plan_id: CommerceId,
    pub price_id: CommerceId,
    pub status: SubscriptionStatus,
    pub provider_refs: Vec<ProviderObjectRef>,
}

/// Subscription lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubscriptionStatus {
    Incomplete,
    Active,
    PastDue,
    Paused,
    Canceled,
}

/// Entitlement granted by a commercial event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntitlementGrant {
    pub id: CommerceId,
    pub customer_org_id: CommerceId,
    pub app_installation_id: CommerceId,
    pub entitlement_key: String,
    pub source_subscription_id: Option<CommerceId>,
    pub status: EntitlementStatus,
}

/// Entitlement lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntitlementStatus {
    Pending,
    Active,
    Suspended,
    Revoked,
    Expired,
}

/// Revenue-share agreement between Reflective and a partner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevenueShareAgreement {
    pub id: CommerceId,
    pub partner_account_id: CommerceId,
    pub app_listing_id: CommerceId,
    pub partner_share_basis_points: u16,
    pub status: AgreementStatus,
}

/// Agreement lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AgreementStatus {
    Draft,
    Active,
    Suspended,
    Terminated,
}

/// Intent to transfer funds to a partner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransferIntent {
    pub id: CommerceId,
    pub agreement_id: CommerceId,
    pub partner_account_id: CommerceId,
    pub amount: MoneyAmount,
    pub idempotency_key: IdempotencyKey,
    pub status: TransferStatus,
}

/// Transfer lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransferStatus {
    Draft,
    Approved,
    Submitted,
    Settled,
    Failed,
    Canceled,
}

/// Partner payout obligation recorded by Commerce Rails.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PayoutObligation {
    pub id: CommerceId,
    pub transfer_intent_id: CommerceId,
    pub partner_account_id: CommerceId,
    pub amount: MoneyAmount,
    pub status: PayoutStatus,
}

/// Payout lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PayoutStatus {
    Pending,
    Held,
    Payable,
    Paid,
    Failed,
    Canceled,
}

/// Ledger entry for commercial audit and reconciliation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LedgerEntry {
    pub id: CommerceId,
    pub account_id: CommerceId,
    pub amount: MoneyAmount,
    pub direction: LedgerDirection,
    pub kind: LedgerEntryKind,
    pub source_ref: LedgerSourceRef,
    pub occurred_at: Timestamp,
}

/// Debit or credit direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerDirection {
    Debit,
    Credit,
}

/// Ledger entry kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LedgerEntryKind {
    Charge,
    Refund,
    RevenueShareAccrual,
    Transfer,
    Payout,
    Adjustment,
}

/// Ledger source reference.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LedgerSourceRef {
    CommerceObject(CommerceId),
    ProviderObject(ProviderObjectRef),
}

/// Webhook receipt for provider-event audit and replay protection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WebhookReceipt {
    pub id: CommerceId,
    pub provider: ProviderName,
    pub provider_event_id: String,
    pub replay_key: ReplayKey,
    pub signature_verified: bool,
    pub received_at: Timestamp,
    pub status: WebhookReceiptStatus,
}

/// Webhook receipt state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WebhookReceiptStatus {
    Received,
    Accepted,
    Duplicate,
    Rejected,
    Failed,
}

/// Executable Commerce Rails command envelope.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommercialCommand<T> {
    pub id: CommerceId,
    pub idempotency_key: IdempotencyKey,
    pub actor: CommandActor,
    pub scope: CommandScope,
    pub origin: CommandOrigin,
    pub requested_at: Timestamp,
    pub safety: CommandSafety,
    pub payload: T,
}

/// Actor that requested or produced a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandActor {
    pub id: CommerceId,
    pub kind: CommandActorKind,
}

/// Command actor category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandActorKind {
    System,
    Operator,
    CustomerUser,
    PartnerUser,
    ProviderAdapter,
    Other(String),
}

/// Bounded commercial scope a command can affect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandScope {
    pub reflective_account_id: Option<CommerceId>,
    pub customer_org_id: Option<CommerceId>,
    pub partner_account_id: Option<CommerceId>,
    pub app_listing_id: Option<CommerceId>,
    pub app_installation_id: Option<CommerceId>,
    pub subscription_id: Option<CommerceId>,
    pub payout_obligation_id: Option<CommerceId>,
}

/// Command origin before the Commerce Rails escapement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandOrigin {
    UserRequest,
    OperatorRequest,
    SystemSchedule,
    ProviderWebhook {
        receipt_id: CommerceId,
        replay_key: ReplayKey,
    },
    Replay {
        source_command_id: CommerceId,
    },
}

/// Safety requirements and gate states for a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandSafety {
    pub webhook: WebhookVerification,
    pub replay: ReplayProtection,
    pub policy: PolicyRequirement,
    pub hitl: HitlRequirement,
    pub audit: AuditRequirement,
    pub reconciliation: ReconciliationRequirement,
}

/// Provider webhook verification state for a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WebhookVerification {
    NotRequired,
    Required { receipt_id: CommerceId },
    Verified { receipt_id: CommerceId },
    Rejected { receipt_id: CommerceId },
}

/// Replay-protection state for webhook and provider-event commands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReplayProtection {
    NotRequired,
    Required { replay_key: ReplayKey },
    Accepted { replay_key: ReplayKey },
    Duplicate { replay_key: ReplayKey },
}

/// Policy checks required before command execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyRequirement {
    NotRequired,
    Required { policy_ids: Vec<CommerceId> },
    Passed { checks: Vec<PolicyCheckRef> },
    Denied { checks: Vec<PolicyCheckRef> },
}

/// Reference to a policy decision recorded by the governing layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyCheckRef {
    pub engine: PolicyEngine,
    pub policy_id: CommerceId,
    pub decision: PolicyDecision,
    pub checked_at: Timestamp,
}

/// Policy engine that made a command decision.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PolicyEngine {
    Arbiter,
    Other(String),
}

/// Policy decision for a command.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyDecision {
    Allowed,
    Denied,
    RequiresApproval,
}

/// Human-in-the-loop requirement for a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HitlRequirement {
    NotRequired,
    Required {
        reason: String,
    },
    Approved {
        approval_id: CommerceId,
        approved_at: Timestamp,
    },
    Rejected {
        approval_id: CommerceId,
        rejected_at: Timestamp,
    },
}

/// Audit requirement attached to a command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuditRequirement {
    pub event_kind: AuditEventKind,
    pub audit_event_id: Option<CommerceId>,
}

/// Commercial audit event produced by command processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommercialAuditEvent {
    pub id: CommerceId,
    pub command_id: CommerceId,
    pub kind: AuditEventKind,
    pub occurred_at: Timestamp,
    pub summary: String,
}

/// Audit event kind for command processing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuditEventKind {
    CommandAccepted,
    CommandDuplicate,
    CommandRequiresApproval,
    CommandRejected,
    CommandExecuted,
    CommandDeferred,
    CommandFailed,
}

/// Reconciliation requirement for commands that can affect money or provider state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReconciliationRequirement {
    NotRequired,
    Required {
        reason: String,
    },
    Completed {
        ledger_entry_ids: Vec<CommerceId>,
        completed_at: Timestamp,
    },
}

/// Result envelope returned after command processing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandResult<T> {
    pub command_id: CommerceId,
    pub idempotency_key: IdempotencyKey,
    pub status: CommandResultStatus,
    pub output: Option<T>,
    pub effects: Vec<CommandEffect>,
    pub audit_event_id: CommerceId,
    pub failure: Option<CommandFailure>,
}

/// Explicit command result state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandResultStatus {
    Accepted,
    Duplicate,
    RequiresApproval,
    Rejected,
    Executed,
    Deferred,
    Failed,
}

/// Effect recorded by command execution.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandEffect {
    CreatedCommerceObject(CommerceId),
    UpdatedCommerceObject(CommerceId),
    RecordedLedgerEntry(CommerceId),
    RecordedWebhookReceipt(CommerceId),
    SubmittedProviderMutation(ProviderObjectRef),
    QueuedReconciliation(CommerceId),
    GrantedEntitlement(CommerceId),
    StagedPayout(CommerceId),
}

/// Explicit command failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandFailure {
    pub kind: CommandFailureKind,
    pub message: String,
}

/// Failure category for a command result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CommandFailureKind {
    InvalidScope,
    IdempotencyConflict,
    WebhookVerificationFailed,
    ReplayRejected,
    PolicyDenied,
    ApprovalRequired,
    ProviderRejected,
    LedgerWriteFailed,
    ReconciliationFailed,
    Unknown,
}

/// First partner piggy-back command loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartnerPiggyBackCommand {
    ListPartnerApp(ListPartnerAppCommand),
    InstallPartnerApp(InstallPartnerAppCommand),
    CreateSubscription(CreateSubscriptionCommand),
    GrantEntitlement(GrantEntitlementCommand),
    RecordRevenueShare(RecordRevenueShareCommand),
    StagePartnerPayout(StagePartnerPayoutCommand),
}

/// Command to list a partner app with an initial plan and price.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListPartnerAppCommand {
    pub app_listing_id: CommerceId,
    pub partner_account_id: CommerceId,
    pub slug: String,
    pub display_name: String,
    pub plan_id: CommerceId,
    pub plan_display_name: String,
    pub billing_interval: BillingInterval,
    pub price_id: CommerceId,
    pub price_amount: MoneyAmount,
}

/// Command to install a listed app for a customer organization.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallPartnerAppCommand {
    pub app_installation_id: CommerceId,
    pub app_listing_id: CommerceId,
    pub customer_org_id: CommerceId,
}

/// Command to create a customer subscription.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CreateSubscriptionCommand {
    pub subscription_id: CommerceId,
    pub customer_org_id: CommerceId,
    pub app_installation_id: CommerceId,
    pub plan_id: CommerceId,
    pub price_id: CommerceId,
    pub provider_refs: Vec<ProviderObjectRef>,
}

/// Command to grant an entitlement from a commercial event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrantEntitlementCommand {
    pub entitlement_grant_id: CommerceId,
    pub customer_org_id: CommerceId,
    pub app_installation_id: CommerceId,
    pub entitlement_key: String,
    pub source_subscription_id: Option<CommerceId>,
}

/// Command to record revenue share after a commercial event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RecordRevenueShareCommand {
    pub agreement_id: CommerceId,
    pub partner_account_id: CommerceId,
    pub app_listing_id: CommerceId,
    pub transfer_intent_id: CommerceId,
    pub payout_obligation_id: CommerceId,
    pub ledger_entry_id: CommerceId,
    pub partner_share_amount: MoneyAmount,
}

/// Command to stage a partner payout obligation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StagePartnerPayoutCommand {
    pub payout_obligation_id: CommerceId,
    pub transfer_intent_id: CommerceId,
    pub partner_account_id: CommerceId,
    pub amount: MoneyAmount,
}

/// Output returned by the first partner piggy-back command loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PartnerPiggyBackOutput {
    PartnerAppListed {
        app_listing_id: CommerceId,
        plan_id: CommerceId,
        price_id: CommerceId,
    },
    PartnerAppInstalled {
        app_installation_id: CommerceId,
    },
    SubscriptionCreated {
        subscription_id: CommerceId,
    },
    EntitlementGranted {
        entitlement_grant_id: CommerceId,
    },
    RevenueShareRecorded {
        transfer_intent_id: CommerceId,
        payout_obligation_id: CommerceId,
        ledger_entry_id: CommerceId,
    },
    PartnerPayoutStaged {
        payout_obligation_id: CommerceId,
    },
}

/// Envelope for the first partner piggy-back command loop.
pub type PartnerPiggyBackCommandEnvelope = CommercialCommand<PartnerPiggyBackCommand>;

/// Result for the first partner piggy-back command loop.
pub type PartnerPiggyBackCommandResult = CommandResult<PartnerPiggyBackOutput>;

/// Commercial policy record used by the governing layer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommercialPolicy {
    pub id: CommerceId,
    pub display_name: String,
    pub status: PolicyStatus,
}

/// Commercial policy lifecycle state.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PolicyStatus {
    Draft,
    Active,
    Retired,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn id(value: &str) -> CommerceId {
        CommerceId::new(value)
    }

    fn timestamp(value: &str) -> Timestamp {
        Timestamp(value.to_owned())
    }

    fn usd(minor_units: i64) -> MoneyAmount {
        MoneyAmount {
            currency: CurrencyCode("USD".to_owned()),
            minor_units,
        }
    }

    fn stripe_ref(object_type: &str, object_id: &str) -> ProviderObjectRef {
        ProviderObjectRef {
            provider: ProviderName::StripeConnect,
            object_type: object_type.to_owned(),
            object_id: object_id.to_owned(),
        }
    }

    fn empty_scope() -> CommandScope {
        CommandScope {
            reflective_account_id: None,
            customer_org_id: None,
            partner_account_id: None,
            app_listing_id: None,
            app_installation_id: None,
            subscription_id: None,
            payout_obligation_id: None,
        }
    }

    #[test]
    fn commerce_id_is_stable_value_identity() {
        let original = id("partner_account:acme");
        let cloned = original.clone();

        let mut seen = HashSet::new();
        assert!(seen.insert(original.clone()));
        assert!(seen.contains(&cloned));
        assert_eq!(original, cloned);
        assert_eq!("partner_account:acme", original.as_str());
    }

    #[test]
    fn customer_id_wraps_commerce_id_with_stable_value_identity() {
        let inner = CommerceId::new("customer:cr:01HZX0F8K9");
        let cid = CustomerId::from_commerce_id(inner.clone());

        assert_eq!(cid.as_commerce_id(), &inner);
        assert_eq!(cid.as_str(), "customer:cr:01HZX0F8K9");

        let cloned = cid.clone();
        let mut seen = HashSet::new();
        assert!(seen.insert(cid.clone()));
        assert!(seen.contains(&cloned));
        assert_eq!(cid, cloned);

        let other = CustomerId::new("customer:cr:01HZX0F8K9");
        assert_eq!(cid, other, "construction paths converge on the same value");
    }

    #[test]
    fn provider_refs_are_external_mappings_not_primary_ids() {
        let partner_id = id("partner_account:acme");
        let connected_account = stripe_ref("account", "acct_123");
        let partner = PartnerAccount {
            id: partner_id.clone(),
            display_name: "Acme Apps".to_owned(),
            status: AccountStatus::Active,
            provider_refs: vec![connected_account.clone()],
        };

        assert_eq!(partner.id, partner_id);
        assert_eq!(partner.provider_refs, vec![connected_account]);
        assert_ne!(
            partner.id.as_str(),
            partner.provider_refs[0].object_id.as_str()
        );

        let subscription_id = id("subscription:commerce:001");
        let stripe_subscription = stripe_ref("subscription", "sub_123");
        let subscription = Subscription {
            id: subscription_id.clone(),
            customer_org_id: id("customer_org:acme"),
            app_installation_id: id("app_installation:timekeeper:acme"),
            plan_id: id("plan:timekeeper:pro"),
            price_id: id("price:timekeeper:pro:monthly"),
            status: SubscriptionStatus::Active,
            provider_refs: vec![stripe_subscription.clone()],
        };

        assert_eq!(subscription.id, subscription_id);
        assert_eq!(subscription.provider_refs, vec![stripe_subscription]);
        assert_ne!(
            subscription.id.as_str(),
            subscription.provider_refs[0].object_id.as_str()
        );
    }

    #[test]
    fn webhook_receipts_share_replay_key_for_duplicate_provider_events() {
        let replay_key = ReplayKey("stripe:event:evt_123".to_owned());
        let accepted = WebhookReceipt {
            id: id("webhook_receipt:accepted"),
            provider: ProviderName::StripeConnect,
            provider_event_id: "evt_123".to_owned(),
            replay_key: replay_key.clone(),
            signature_verified: true,
            received_at: timestamp("2026-05-17T12:00:00Z"),
            status: WebhookReceiptStatus::Accepted,
        };
        let duplicate = WebhookReceipt {
            id: id("webhook_receipt:duplicate"),
            provider: ProviderName::StripeConnect,
            provider_event_id: "evt_123".to_owned(),
            replay_key: replay_key.clone(),
            signature_verified: true,
            received_at: timestamp("2026-05-17T12:00:03Z"),
            status: WebhookReceiptStatus::Duplicate,
        };

        assert_ne!(accepted.id, duplicate.id);
        assert_eq!(accepted.provider_event_id, duplicate.provider_event_id);
        assert_eq!(accepted.replay_key, duplicate.replay_key);

        let mut seen = HashSet::new();
        assert!(seen.insert(accepted.replay_key.clone()));
        assert!(!seen.insert(duplicate.replay_key.clone()));
    }

    #[test]
    fn provider_webhook_command_carries_receipt_and_replay_gate() {
        let receipt_id = id("webhook_receipt:stripe:evt_123");
        let replay_key = ReplayKey("stripe:event:evt_123".to_owned());
        let mut scope = empty_scope();
        scope.customer_org_id = Some(id("customer_org:acme"));
        scope.subscription_id = Some(id("subscription:commerce:001"));

        let command = PartnerPiggyBackCommandEnvelope {
            id: id("command:stripe:subscription-created"),
            idempotency_key: IdempotencyKey("idem:stripe:evt_123".to_owned()),
            actor: CommandActor {
                id: id("actor:stripe-connect-adapter"),
                kind: CommandActorKind::ProviderAdapter,
            },
            scope,
            origin: CommandOrigin::ProviderWebhook {
                receipt_id: receipt_id.clone(),
                replay_key: replay_key.clone(),
            },
            requested_at: timestamp("2026-05-17T12:00:04Z"),
            safety: CommandSafety {
                webhook: WebhookVerification::Verified {
                    receipt_id: receipt_id.clone(),
                },
                replay: ReplayProtection::Accepted {
                    replay_key: replay_key.clone(),
                },
                policy: PolicyRequirement::NotRequired,
                hitl: HitlRequirement::NotRequired,
                audit: AuditRequirement {
                    event_kind: AuditEventKind::CommandAccepted,
                    audit_event_id: None,
                },
                reconciliation: ReconciliationRequirement::Required {
                    reason:
                        "subscription provider state must reconcile to ledger and entitlement state"
                            .to_owned(),
                },
            },
            payload: PartnerPiggyBackCommand::CreateSubscription(CreateSubscriptionCommand {
                subscription_id: id("subscription:commerce:001"),
                customer_org_id: id("customer_org:acme"),
                app_installation_id: id("app_installation:timekeeper:acme"),
                plan_id: id("plan:timekeeper:pro"),
                price_id: id("price:timekeeper:pro:monthly"),
                provider_refs: vec![stripe_ref("subscription", "sub_123")],
            }),
        };

        assert_eq!(
            command.idempotency_key,
            IdempotencyKey("idem:stripe:evt_123".to_owned())
        );
        assert_eq!(
            command.origin,
            CommandOrigin::ProviderWebhook {
                receipt_id: receipt_id.clone(),
                replay_key: replay_key.clone(),
            }
        );
        assert_eq!(
            command.safety.webhook,
            WebhookVerification::Verified { receipt_id }
        );
        assert_eq!(
            command.safety.replay,
            ReplayProtection::Accepted { replay_key }
        );
    }

    #[test]
    fn partner_piggy_back_result_records_created_objects_and_audit() {
        let result = PartnerPiggyBackCommandResult {
            command_id: id("command:list-partner-app"),
            idempotency_key: IdempotencyKey("idem:list-partner-app:timekeeper".to_owned()),
            status: CommandResultStatus::Executed,
            output: Some(PartnerPiggyBackOutput::PartnerAppListed {
                app_listing_id: id("app_listing:timekeeper"),
                plan_id: id("plan:timekeeper:pro"),
                price_id: id("price:timekeeper:pro:monthly"),
            }),
            effects: vec![
                CommandEffect::CreatedCommerceObject(id("app_listing:timekeeper")),
                CommandEffect::CreatedCommerceObject(id("plan:timekeeper:pro")),
                CommandEffect::CreatedCommerceObject(id("price:timekeeper:pro:monthly")),
            ],
            audit_event_id: id("audit:list-partner-app"),
            failure: None,
        };

        assert_eq!(result.status, CommandResultStatus::Executed);
        assert_eq!(result.effects.len(), 3);
        assert!(result.failure.is_none());
    }

    #[test]
    fn money_moving_command_requires_reconciliation() {
        let mut scope = empty_scope();
        scope.partner_account_id = Some(id("partner_account:acme"));
        scope.payout_obligation_id = Some(id("payout_obligation:001"));

        let command = PartnerPiggyBackCommandEnvelope {
            id: id("command:stage-partner-payout"),
            idempotency_key: IdempotencyKey("idem:stage-partner-payout:001".to_owned()),
            actor: CommandActor {
                id: id("actor:commerce-rails"),
                kind: CommandActorKind::System,
            },
            scope,
            origin: CommandOrigin::SystemSchedule,
            requested_at: timestamp("2026-05-17T12:01:00Z"),
            safety: CommandSafety {
                webhook: WebhookVerification::NotRequired,
                replay: ReplayProtection::NotRequired,
                policy: PolicyRequirement::Required {
                    policy_ids: vec![id("policy:payout-staging")],
                },
                hitl: HitlRequirement::Required {
                    reason: "partner payout staging can become money movement".to_owned(),
                },
                audit: AuditRequirement {
                    event_kind: AuditEventKind::CommandRequiresApproval,
                    audit_event_id: None,
                },
                reconciliation: ReconciliationRequirement::Required {
                    reason: "payout obligation must reconcile to transfer and ledger state"
                        .to_owned(),
                },
            },
            payload: PartnerPiggyBackCommand::StagePartnerPayout(StagePartnerPayoutCommand {
                payout_obligation_id: id("payout_obligation:001"),
                transfer_intent_id: id("transfer_intent:001"),
                partner_account_id: id("partner_account:acme"),
                amount: usd(12_500),
            }),
        };

        assert!(matches!(
            command.safety.hitl,
            HitlRequirement::Required { .. }
        ));
        assert!(matches!(
            command.safety.reconciliation,
            ReconciliationRequirement::Required { .. }
        ));
    }

    #[test]
    fn partner_app_listing_can_be_represented_with_commercial_context() {
        let partner_account_id = id("partner_account:acme");
        let app_listing = AppListing {
            id: id("app_listing:timekeeper"),
            partner_account_id: partner_account_id.clone(),
            slug: "timekeeper".to_owned(),
            display_name: "Timekeeper".to_owned(),
            status: ListingStatus::Listed,
        };
        let plan = Plan {
            id: id("plan:timekeeper:pro"),
            app_listing_id: app_listing.id.clone(),
            display_name: "Pro".to_owned(),
            billing_interval: BillingInterval::Month,
        };
        let price = Price {
            id: id("price:timekeeper:pro:monthly"),
            plan_id: plan.id.clone(),
            amount: usd(2_000),
            status: PriceStatus::Active,
        };
        let agreement = RevenueShareAgreement {
            id: id("revenue_share:timekeeper:acme"),
            partner_account_id: partner_account_id.clone(),
            app_listing_id: app_listing.id.clone(),
            partner_share_basis_points: 7_000,
            status: AgreementStatus::Active,
        };

        assert_eq!(app_listing.partner_account_id, partner_account_id);
        assert_eq!(plan.app_listing_id, app_listing.id);
        assert_eq!(price.plan_id, plan.id);
        assert_eq!(agreement.app_listing_id, app_listing.id);
        assert_eq!(agreement.partner_share_basis_points, 7_000);
    }

    #[test]
    fn customer_app_installation_can_be_represented() {
        let customer = CustomerOrg {
            id: id("customer_org:acme"),
            legal_name: "Acme Manufacturing".to_owned(),
            status: AccountStatus::Active,
        };
        let listing = AppListing {
            id: id("app_listing:timekeeper"),
            partner_account_id: id("partner_account:acme-apps"),
            slug: "timekeeper".to_owned(),
            display_name: "Timekeeper".to_owned(),
            status: ListingStatus::Listed,
        };
        let installation = AppInstallation {
            id: id("app_installation:timekeeper:acme"),
            app_listing_id: listing.id.clone(),
            customer_org_id: customer.id.clone(),
            status: InstallationStatus::Active,
        };

        assert_eq!(installation.app_listing_id, listing.id);
        assert_eq!(installation.customer_org_id, customer.id);
        assert_eq!(installation.status, InstallationStatus::Active);
    }

    #[test]
    fn subscription_can_grant_an_entitlement() {
        let subscription = Subscription {
            id: id("subscription:timekeeper:acme"),
            customer_org_id: id("customer_org:acme"),
            app_installation_id: id("app_installation:timekeeper:acme"),
            plan_id: id("plan:timekeeper:pro"),
            price_id: id("price:timekeeper:pro:monthly"),
            status: SubscriptionStatus::Active,
            provider_refs: vec![stripe_ref("subscription", "sub_timekeeper_acme")],
        };
        let entitlement = EntitlementGrant {
            id: id("entitlement:timekeeper:acme:pro"),
            customer_org_id: subscription.customer_org_id.clone(),
            app_installation_id: subscription.app_installation_id.clone(),
            entitlement_key: "timekeeper.pro".to_owned(),
            source_subscription_id: Some(subscription.id.clone()),
            status: EntitlementStatus::Active,
        };

        assert_eq!(entitlement.customer_org_id, subscription.customer_org_id);
        assert_eq!(
            entitlement.app_installation_id,
            subscription.app_installation_id
        );
        assert_eq!(entitlement.source_subscription_id, Some(subscription.id));
        assert_eq!(entitlement.status, EntitlementStatus::Active);
    }

    #[test]
    fn revenue_share_agreement_can_produce_payout_obligation() {
        let agreement = RevenueShareAgreement {
            id: id("revenue_share:timekeeper:acme-apps"),
            partner_account_id: id("partner_account:acme-apps"),
            app_listing_id: id("app_listing:timekeeper"),
            partner_share_basis_points: 7_000,
            status: AgreementStatus::Active,
        };
        let amount = usd(1_400);
        let transfer = TransferIntent {
            id: id("transfer_intent:timekeeper:invoice-001"),
            agreement_id: agreement.id.clone(),
            partner_account_id: agreement.partner_account_id.clone(),
            amount: amount.clone(),
            idempotency_key: IdempotencyKey("idem:transfer:timekeeper:invoice-001".to_owned()),
            status: TransferStatus::Approved,
        };
        let payout = PayoutObligation {
            id: id("payout_obligation:timekeeper:invoice-001"),
            transfer_intent_id: transfer.id.clone(),
            partner_account_id: agreement.partner_account_id.clone(),
            amount: amount.clone(),
            status: PayoutStatus::Payable,
        };
        let ledger = LedgerEntry {
            id: id("ledger:revshare:timekeeper:invoice-001"),
            account_id: agreement.partner_account_id.clone(),
            amount,
            direction: LedgerDirection::Credit,
            kind: LedgerEntryKind::RevenueShareAccrual,
            source_ref: LedgerSourceRef::CommerceObject(payout.id.clone()),
            occurred_at: timestamp("2026-05-17T12:02:00Z"),
        };

        assert_eq!(transfer.agreement_id, agreement.id);
        assert_eq!(transfer.partner_account_id, agreement.partner_account_id);
        assert_eq!(payout.transfer_intent_id, transfer.id);
        assert_eq!(payout.amount, transfer.amount);
        assert_eq!(
            ledger.source_ref,
            LedgerSourceRef::CommerceObject(payout.id)
        );
    }

    #[test]
    fn stripe_connect_event_maps_to_receipt_without_replacing_commerce_ids() {
        let receipt = WebhookReceipt {
            id: id("webhook_receipt:stripe:evt_invoice_paid_001"),
            provider: ProviderName::StripeConnect,
            provider_event_id: "evt_invoice_paid_001".to_owned(),
            replay_key: ReplayKey("stripe:event:evt_invoice_paid_001".to_owned()),
            signature_verified: true,
            received_at: timestamp("2026-05-17T12:03:00Z"),
            status: WebhookReceiptStatus::Accepted,
        };
        let subscription_id = id("subscription:timekeeper:acme");
        let provider_ref = stripe_ref("subscription", "sub_timekeeper_acme");
        let mut scope = empty_scope();
        scope.customer_org_id = Some(id("customer_org:acme"));
        scope.subscription_id = Some(subscription_id.clone());

        let command = PartnerPiggyBackCommandEnvelope {
            id: id("command:stripe:invoice-paid:001"),
            idempotency_key: IdempotencyKey("idem:stripe:evt_invoice_paid_001".to_owned()),
            actor: CommandActor {
                id: id("actor:stripe-connect-adapter"),
                kind: CommandActorKind::ProviderAdapter,
            },
            scope,
            origin: CommandOrigin::ProviderWebhook {
                receipt_id: receipt.id.clone(),
                replay_key: receipt.replay_key.clone(),
            },
            requested_at: timestamp("2026-05-17T12:03:01Z"),
            safety: CommandSafety {
                webhook: WebhookVerification::Verified {
                    receipt_id: receipt.id.clone(),
                },
                replay: ReplayProtection::Accepted {
                    replay_key: receipt.replay_key.clone(),
                },
                policy: PolicyRequirement::NotRequired,
                hitl: HitlRequirement::NotRequired,
                audit: AuditRequirement {
                    event_kind: AuditEventKind::CommandAccepted,
                    audit_event_id: Some(id("audit:stripe:invoice-paid:001")),
                },
                reconciliation: ReconciliationRequirement::Required {
                    reason:
                        "Stripe invoice state must reconcile to subscription and entitlement state"
                            .to_owned(),
                },
            },
            payload: PartnerPiggyBackCommand::CreateSubscription(CreateSubscriptionCommand {
                subscription_id: subscription_id.clone(),
                customer_org_id: id("customer_org:acme"),
                app_installation_id: id("app_installation:timekeeper:acme"),
                plan_id: id("plan:timekeeper:pro"),
                price_id: id("price:timekeeper:pro:monthly"),
                provider_refs: vec![provider_ref.clone()],
            }),
        };

        assert_eq!(receipt.provider, ProviderName::StripeConnect);
        assert_eq!(receipt.provider_event_id, "evt_invoice_paid_001");
        assert_ne!(receipt.id.as_str(), receipt.provider_event_id.as_str());
        assert_ne!(subscription_id.as_str(), provider_ref.object_id.as_str());
        assert_eq!(
            command.origin,
            CommandOrigin::ProviderWebhook {
                receipt_id: receipt.id,
                replay_key: receipt.replay_key,
            }
        );
    }
}
