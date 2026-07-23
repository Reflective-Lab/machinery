# Intent Codec JTBD Schema

Status: Product doctrine. Not yet enforced by runtime schema or CI.

This page defines how Reflective describes a business need before it becomes a
Helm path, applet, Truth, formation, or commercial package.

The goal is to keep apps thin. A good applet is not a small SaaS clone. It is a
job description with just enough surface area to express intent, show evidence,
collect approval, and project the governed result.

## Purpose

The Intent Codec translates human business need into a structured job shape
without granting authority.

It sits at the product boundary between JTBD language and the Axiom/Organism
handoff:

```text
raw business need
  -> functional / emotional / relational JTBD
  -> authority and evidence envelope
  -> Axiom Truth / IntentPacket candidate
  -> Organism formation selection
  -> Converge-governed execution
  -> Helm or app projection
```

The codec is successful when a new app idea can be evaluated as a repeatable
job before anyone designs a full product surface around it.

## Required Fields

Every candidate applet or Helm job must answer all sections below.

| Field | Question | Owned by |
|-------|----------|----------|
| `job_name` | What outcome is the user hiring the system to produce? | Product |
| `functional_need` | What has to be done, transformed, decided, generated, reconciled, or prevented? | Product / Axiom |
| `emotional_need` | What anxiety, confidence gap, trust deficit, status risk, or relief is driving adoption? | Product |
| `relational_need` | Which people, teams, customers, partners, regulators, or systems must trust or act on the result? | Product / Helm |
| `trigger` | What event makes the job urgent now? | Product |
| `current_workaround` | What manual process, SaaS workflow, spreadsheet, meeting, or expert judgment is being replaced? | Product |
| `success_signal` | What observable evidence proves the job was done well? | Product / Converge |
| `failure_modes` | What would make the job harmful, misleading, wasteful, or unsafe? | Product / Converge |
| `authority` | Who may request, approve, reject, retry, reverse, or expire the job? | Helm / Converge |
| `evidence_contract` | Which sources are allowed, required, stale, sensitive, or disputed? | Axiom / Mosaic |
| `runtime_needs` | What auth, storage, telemetry, secrets, jobs, or hosting are required? | Runtime Runway |
| `commercial_needs` | What entitlement, billing, install, payout, refund, or reconciliation state is involved? | Commerce Rails |
| `projection` | What does the operator see, approve, compare, or export? | Helm / app |
| `non_goals` | What must this applet not own? | Product / architecture |

## JTBD Triad

Functional, emotional, and relational needs are all required. A functional-only
description is usually a feature list in disguise.

### Functional

Describe the concrete business result:

- input state
- expected output
- transformation or decision
- constraints
- metric or invariant
- acceptable latency or freshness

Example: "Activate a customer subscription and grant the correct entitlements
after payment confirmation."

### Emotional

Describe the operator's felt risk:

- uncertainty
- fear of making the wrong call
- fear of customer embarrassment
- need for confidence before delegating
- desire for relief from chasing status
- tolerance for delay versus error

Example: "The operator wants confidence that payment, access, and customer
communication agree before the customer notices a mismatch."

### Relational

Describe the trust and accountability network:

- who depends on the result
- who must approve it
- who receives the consequence
- who can dispute it
- what must be explainable to a customer, partner, auditor, or internal owner
- which handoff or obligation is created

Example: "Sales, support, finance, the customer admin, and Commerce Rails all
need the same subscription state, with a receipt trail if access is denied."

## Authority Envelope

The codec never grants authority by itself. It describes the authority that
Helm, Converge, Runtime Runway, and Commerce Rails must enforce.

Use this shape:

```yaml
authority:
  requester: "<actor or role>"
  approvers:
    - "<actor or role>"
  allowed_actions:
    - "<bounded action>"
  forbidden_actions:
    - "<action the job must not take>"
  approval_points:
    - "<where Helm must pause or show receipt>"
  reversibility: "reversible | partially_reversible | irreversible"
  expiry: "<time or event after which the intent is stale>"
  audit_visibility:
    - "operator"
    - "customer"
    - "finance"
```

## Evidence Contract

The evidence contract prevents applets from smuggling domain truth through
provider-specific strings or local assumptions.

```yaml
evidence_contract:
  required_sources:
    - source: "<system, document, event, or human approval>"
      freshness: "<duration or event>"
      authority: "primary | corroborating | advisory"
  disallowed_sources:
    - "<source or class of source>"
  confidence_floor: "<threshold or policy>"
  conflict_policy: "stop | ask_operator | prefer_primary | run_adversarial_review"
  sensitive_fields:
    - "<field name>"
```

## Layer Mapping

Every applet proposal must map its pieces to the owning layer.

| Layer | Codec output |
|-------|--------------|
| Applet / app | User-facing projection, domain language, subject references, local UX state |
| Helm | Operator control, approvals, receipts, exceptions, trust transfer |
| Axiom | Truth definition, verifier report, calibration, IntentPacket candidate |
| Organism | Formation selection, adversarial review, simulation plan |
| Converge | Admission, promotion, facts, stop reasons, criteria, receipts |
| Mosaic | Reusable specialist capability, source ports, solvers, adapters, memory, analytics |
| Runtime Runway | Auth, storage, secrets, telemetry, hosting, jobs, realtime transport |
| Commerce Rails | Billing, entitlement, install, payout, refund, dispute, reconciliation |

If the same row appears in multiple applets, the reusable part probably belongs
in Bedrock, Mosaic, Runtime Runway, or Commerce Rails rather than the applet.

## Applet Readiness Gate

A candidate is ready to become implementation work only when all checks pass:

- One primary job is named.
- Functional, emotional, and relational needs are all present.
- Non-goals are explicit.
- At least one Truth or candidate Truth is identified.
- Approval, stop, retry, and reversal points are named.
- Runtime needs are separated from product meaning.
- Commercial needs are separated from app-local state.
- Provider identifiers are not treated as domain truth.
- The smallest useful projection can be described in one screen or one command.

## Big-App Smells

Stop and reframe the applet when the proposal:

- starts from pages, menus, dashboards, or modules rather than a job
- owns auth, secrets, billing, entitlement, and domain workflow locally
- copies a SaaS product category as the product boundary
- has many unrelated jobs with no shared Truth
- uses strings where typed route owners, actors, states, entitlements, or events
  should exist
- cannot say who must trust the result
- cannot name what Converge would admit or refuse

## Example

```yaml
job_name: "Activate paid subscription"
trigger: "Stripe reports a successful initial invoice payment"
functional_need:
  outcome: "Grant the customer the purchased entitlement set."
  inputs:
    - "provider payment event"
    - "Commerce Rails subscription contract"
    - "customer account reference"
  output: "Accepted entitlement grant and operator-visible receipt"
  success_signal: "Customer can access the paid capability and finance can reconcile the payment"
emotional_need:
  operator_anxiety: "Avoid customer embarrassment from paid-but-locked access"
  desired_confidence: "Payment, entitlement, and receipt agree before support sees the ticket"
relational_need:
  dependent_parties:
    - "customer admin"
    - "support"
    - "finance"
    - "partner owner"
  trust_obligation: "Show why access was granted or refused without exposing provider internals"
authority:
  requester: "commerce webhook envelope"
  approvers:
    - "Commerce Rails policy"
  allowed_actions:
    - "grant entitlement"
    - "pause for manual review"
  forbidden_actions:
    - "store Stripe object ID as canonical entitlement"
  approval_points:
    - "manual review when payment and contract disagree"
  reversibility: "partially_reversible"
  expiry: "payment event replay window"
evidence_contract:
  required_sources:
    - source: "Commerce Rails verified provider event"
      freshness: "within replay window"
      authority: "primary"
    - source: "Commerce Rails subscription contract"
      freshness: "current at event time"
      authority: "primary"
  conflict_policy: "stop"
runtime_needs:
  - "webhook ingress"
  - "secret access"
  - "telemetry"
commercial_needs:
  - "subscription"
  - "entitlement grant"
  - "provider reconciliation"
projection:
  operator_view: "receipt, entitlement, stop reason, retry status"
non_goals:
  - "build a billing dashboard inside the applet"
  - "let the applet own provider reconciliation"
```

## Operating Rule

When a new app or applet is proposed, write the Intent Codec entry first. If the
entry cannot stay small, the proposal is probably a product suite, not an
applet. Split it until each unit has one job, one consequence lane, and one
clear trust-transfer surface.

## Reusable Artifacts

- [[templates/intent-codec-applet|Intent Codec Applet Template]]
- `templates/intent-codec-applet.manifest.schema.json`
- [[applets/activate-subscription|Applied example: activate-subscription]]
- [[applets/refill-prepaid-ai-credits|Applied example: refill-prepaid-ai-credits]]
- Machine-readable manifests:
  `applets/activate-subscription.intent.json`,
  `applets/refill-prepaid-ai-credits.intent.json`
- Arena reusable cases: `../../arena-tests/crates/intent-cases/src/lib.rs`
- Arena smoke checks:
  `../../arena-tests/crates/cross-extension-smoke/tests/intent_codec_applets.rs`
- Atelier interactive example:
  `../../atelier-showcase/tutorials/04-intent-codec-loop`
