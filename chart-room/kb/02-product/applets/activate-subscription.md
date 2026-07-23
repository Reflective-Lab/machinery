# Applet Intent Codec - Activate Subscription

Status: Code-backed proof path. The Helm truth is executable through
`workbench-backend`; this page describes the reusable applet shape around that
truth.

Source schema: [[../intent-codec-jtbd-schema|Intent Codec JTBD Schema]]
Template: [[../templates/intent-codec-applet|Intent Codec Applet Template]]
Manifest: `activate-subscription.intent.json`

## Source Evidence

- Truth catalog:
  `bedrock-platform/helms/crates/truth-catalog/src/lib.rs` registers
  `activate-subscription` as a commercial job that turns an agreed commercial
  plan into active subscription and entitlement state.
- Feature file:
  `bedrock-platform/helms/truths/jobs/activate_subscription.feature` states the
  intent: activate subscription and entitlement state from an agreed commercial
  plan.
- Executable path:
  `bedrock-platform/helms/crates/workbench-backend/src/lib.rs` exposes
  `execute_activate_subscription`.
- Tests:
  `execute_activate_subscription_projects_revenue_state` completes when
  `payment_confirmed=true`; `execute_activate_subscription_blocks_without_payment_confirmation`
  returns a blocked session and creates an approval when `payment_confirmed=false`.
- Runtime or commerce event:
  `bedrock-platform/helms/crates/crm-contracts/src/lib.rs` maps
  `SubscriptionActivationRequested` to `activate-subscription`.
- Existing KB note:
  `KB/outcome-workbench/kb/Architecture/Converge Application.md` says
  `activate-subscription` and `refill-prepaid-ai-credits` are live end-to-end
  revenue truths that return structured commercial projection data.

## Intent Codec Entry

```yaml
job_name: "Activate paid subscription"
primary_job_key: "activate-subscription"
trigger: "A commercial commitment is ready to become active after payment confirmation."
current_workaround: "A billing or RevOps operator manually checks payment, plan, entitlements, and opening balance before enabling customer access."

functional_need:
  outcome: "Turn an agreed commercial plan into active subscription, entitlement, and auditable opening financial state."
  inputs:
    - "organization_id"
    - "subscription_id"
    - "catalog_item_id"
    - "payment_confirmed"
  output: "Completed truth session with subscription projection and entitlement IDs, or blocked truth session with approval/workflow references."
  constraints:
    - "Subscription must belong to the organization."
    - "Active subscription must resolve to a valid catalog plan."
    - "Payment confirmation is required before activation."
    - "Activation exceptions move through a workflow case and approval."
    - "Entitlements and opening financial state must remain auditable."
  success_signal: "The truth session completes, subscription_id is projected, entitlements are derived, and no approval is required."

emotional_need:
  operator_anxiety: "The customer may have paid but still be locked out, or may receive access before payment and plan terms are trustworthy."
  desired_confidence: "Payment, plan, subscription state, entitlement grant, and operator receipt agree before support or the customer sees a mismatch."
  tolerance: "Prefer a blocked session and manual review over silent activation when payment or terms are uncertain."

relational_need:
  dependent_parties:
    - "customer admin"
    - "billing operator"
    - "RevOps"
    - "support"
    - "finance"
    - "partner owner when marketplace or revenue-share terms apply"
  trust_obligation: "Explain why access was granted or paused without treating provider IDs as commercial truth."
  handoff_created: "If activation is blocked, Helm owns the operator-visible approval/workflow handoff."

failure_modes:
  - "Activating access before payment is confirmed."
  - "Granting entitlements for the wrong organization or subscription."
  - "Treating Stripe or provider object IDs as canonical entitlement truth."
  - "Mutating commercial state without an auditable operator receipt."
  - "Hiding activation exceptions inside app-local state."

authority:
  requester: "subscription_activation_requested commerce/runtime envelope"
  approvers:
    - "Commerce Rails policy"
    - "billing operator for blocked activation"
  allowed_actions:
    - "activate subscription"
    - "derive entitlements from catalog plan"
    - "open approval workflow"
    - "project subscription and entitlement state"
  forbidden_actions:
    - "grant entitlement without payment confirmation"
    - "let the applet own provider reconciliation"
    - "store provider object ID as canonical entitlement"
    - "bypass Helm approval on blocked activation"
  approval_points:
    - "payment_confirmed is absent or false"
    - "non-standard plan terms or manual review signal"
  reversibility: "partially_reversible"
  expiry: "payment or activation event replay window"
  audit_visibility:
    - "operator"
    - "finance"
    - "support"
    - "partner owner"

evidence_contract:
  required_sources:
    - source: "Commerce Rails verified subscription contract"
      freshness: "current at activation time"
      authority: "primary"
    - source: "Runtime or commerce payment confirmation envelope"
      freshness: "within replay window"
      authority: "primary"
    - source: "catalog plan definition"
      freshness: "current at activation time"
      authority: "primary"
    - source: "Helm approval record when activation blocks"
      freshness: "current workflow case"
      authority: "primary"
  disallowed_sources:
    - "raw provider ID as entitlement truth"
    - "app-local boolean that bypasses commercial verification"
  confidence_floor: "payment_confirmed must be true for automatic activation"
  conflict_policy: "stop"
  sensitive_fields:
    - "organization_id"
    - "subscription_id"
    - "catalog_item_id"
    - "payment reference or provider correlation IDs"

runtime_needs:
  - "normalized commerce/runtime event ingress"
  - "secret handling for payment-provider verification outside the applet"
  - "telemetry for activation, blocked session, approval, and retry"
  - "durable workflow/approval references"

commercial_needs:
  - "subscription lifecycle state"
  - "catalog plan resolution"
  - "entitlement grant"
  - "opening ledger or balance context"
  - "provider reconciliation outside the applet"

projection:
  operator_view: "subscription ID, plan/catalog item, activation state, entitlement IDs, approval IDs, workflow case IDs, stop reason"
  customer_or_partner_view: "access active or activation paused with support-safe reason"

non_goals:
  - "Build a billing dashboard inside the applet."
  - "Let the applet own Stripe/provider verification."
  - "Let the applet own entitlement or subscription canonical truth."
  - "Add unrelated billing jobs such as top-up, upgrade, suspension, or reconciliation to this applet."
```

## Layer Mapping

| Layer | This applet maps to |
|-------|---------------------|
| Applet / app | Minimal activation request/projection surface: show status, stop reason, and receipt. |
| Helm | Operator approval, workflow case, blocked-session receipt, and trust-transfer view. |
| Axiom | `activate-subscription` Truth shape and candidate IntentPacket. |
| Organism | Future formation selection for non-standard activation review; not required for the current executable path. |
| Converge | Admission, criteria outcomes, completed or blocked truth session, and stop reason. |
| Mosaic | No specialist required for the current path; future risk or fraud review could use reusable policy/model specialists. |
| Runtime Runway | Event ingress, auth, secrets, telemetry, and durable runtime envelope. |
| Commerce Rails | Subscription, catalog plan, entitlement, ledger/opening balance, and provider reconciliation authority. |

## Applet Readiness Gate

- [x] One primary job is named.
- [x] Functional, emotional, and relational needs are all present.
- [x] Non-goals are explicit.
- [x] At least one Truth or candidate Truth is identified.
- [x] Approval, stop, retry, and reversal points are named.
- [x] Runtime needs are separated from product meaning.
- [x] Commercial needs are separated from app-local state.
- [x] Provider identifiers are not treated as domain truth.
- [x] The smallest useful projection can be described in one screen or one command.

## Open Gaps

- Gap: Organism formation selection is not yet wired into this proof path.
  - Evidence: `MASTERPLAN.md` identifies Axiom -> Organism -> Converge in a
    product path as the most important current gap.
  - Owner: Bedrock / Codex
  - Next action: Route non-standard activation review through an Organism
    formation once the Helm proof path is selected for wiring.
