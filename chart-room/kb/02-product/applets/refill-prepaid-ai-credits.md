# Applet Intent Codec - Refill Prepaid AI Credits

Status: Code-backed proof path. The Helm truth is executable through
`workbench-backend`; this page describes the reusable applet shape around that
truth.

Source schema: [[../intent-codec-jtbd-schema|Intent Codec JTBD Schema]]
Template: [[../templates/intent-codec-applet|Intent Codec Applet Template]]
Manifest: `refill-prepaid-ai-credits.intent.json`

## Source Evidence

- Truth catalog:
  `bedrock-platform/helms/crates/truth-catalog/src/lib.rs` registers
  `refill-prepaid-ai-credits` as a commercial job that applies a top-up
  purchase to prepaid AI credit balances with financial traceability.
- Feature file:
  `bedrock-platform/helms/truths/jobs/refill_prepaid_ai_credits.feature`
  states the intent: apply top-up purchase to prepaid AI credit balances.
- Executable path:
  `bedrock-platform/helms/crates/workbench-backend/src/lib.rs` exposes
  `execute_refill_prepaid_ai_credits`.
- Tests:
  `execute_refill_prepaid_ai_credits_updates_entitlement_balance` completes
  for settled top-ups; `execute_refill_prepaid_ai_credits_blocks_pending_payment`
  returns a blocked session when payment is pending.
- Runtime or commerce event:
  `bedrock-platform/helms/crates/crm-contracts/src/lib.rs` maps
  `PrepaidTopUpSettled` to `refill-prepaid-ai-credits`.
- Existing KB note:
  `bedrock-platform/helms/kb/Architecture/Converge Application.md` says
  `refill-prepaid-ai-credits` is live end-to-end and payment-gated.

## Intent Codec Entry

See `refill-prepaid-ai-credits.intent.json` for the machine-readable applet
manifest. Its core JTBD is:

- Functional: apply a settled top-up to prepaid AI credit balances with
  financial traceability.
- Emotional: avoid service interruption from stale balance while preventing
  credit grants for risky or unsettled payment.
- Relational: customer admin, finance, support, runtime metering, billing
  operator, and partner owner need an explainable balance change.

## Layer Mapping

| Layer | This applet maps to |
|-------|---------------------|
| Applet / app | Minimal prepaid refill request/projection surface: show payment status, credit grant, ledger receipt, and stop reason. |
| Helm | Payment/risk review, workflow case, blocked-session receipt, and trust-transfer view. |
| Axiom | `refill-prepaid-ai-credits` Truth shape and candidate IntentPacket. |
| Organism | Future formation selection for unusual top-up or fraud review; not required for the current executable path. |
| Converge | Admission, criteria outcomes, completed or blocked truth session, and stop reason. |
| Mosaic | No specialist required for the current path; future risk or fraud review could use reusable policy/model specialists. |
| Runtime Runway | Event ingress, auth, secrets, telemetry, and durable runtime envelope. |
| Commerce Rails | Payment settlement, subscription commitment, credit entitlement, ledger grant, and provider reconciliation authority. |

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
  - Evidence: The current executable path is Helm/workbench-backed and blocks
    pending or risky top-ups, but unusual top-up review still has no explicit
    applet-to-formation route.
  - Owner: Bedrock / Codex
  - Next action: Route non-standard top-up review through an Organism
    formation once the Helm proof path is selected for wiring.
