# Intent Codec Applet Template

Status: Reusable template. Copy this file when proposing a new applet or Helm
job path.

Source schema: [[../intent-codec-jtbd-schema|Intent Codec JTBD Schema]]
Manifest schema:
`intent-codec-applet.manifest.schema.json`

## Header

- Applet name:
- Primary job key:
- Proposed owner:
- Current status: proposed | code-backed | executable | retired
- Related Truth:
- Related Helm path:
- Related app path:
- Related Commerce Rails path:
- Related Runtime Runway path:

## Source Evidence

List concrete evidence before describing the applet.

- Truth catalog:
- Feature or `.truths` file:
- Executable path:
- Tests:
- Runtime or commerce event:
- Existing KB note:

## Intent Codec Entry

Use the Markdown form for human review and the JSON manifest schema for
machine-readable fixtures, tests, and app-specific projections.

```yaml
job_name: "<business outcome, not product category>"
primary_job_key: "<truth or candidate truth key>"
trigger: "<event that makes the job urgent now>"
current_workaround: "<manual/SaaS/spreadsheet/workflow this replaces>"

functional_need:
  outcome: "<what must become true>"
  inputs:
    - "<input state or source>"
  output: "<governed result>"
  constraints:
    - "<rule, invariant, latency, freshness, or policy>"
  success_signal: "<observable proof that the job completed correctly>"

emotional_need:
  operator_anxiety: "<fear, uncertainty, support burden, or trust gap>"
  desired_confidence: "<what the operator wants to know before delegating>"
  tolerance: "<delay/error/review preference>"

relational_need:
  dependent_parties:
    - "<person, team, customer, partner, regulator, or system>"
  trust_obligation: "<what must be explainable and to whom>"
  handoff_created: "<new obligation, ownership, or next step>"

failure_modes:
  - "<harmful, misleading, wasteful, or unsafe outcome>"

authority:
  requester: "<actor, event, or role>"
  approvers:
    - "<actor, role, or policy>"
  allowed_actions:
    - "<bounded action>"
  forbidden_actions:
    - "<action the applet must not take>"
  approval_points:
    - "<where Helm must pause or show receipt>"
  reversibility: "reversible | partially_reversible | irreversible"
  expiry: "<time or event after which the intent is stale>"
  audit_visibility:
    - "<operator, customer, finance, support, partner, auditor>"

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

runtime_needs:
  - "<auth, storage, telemetry, secret, job, webhook, realtime, or host need>"

commercial_needs:
  - "<subscription, entitlement, install, payout, refund, dispute, reconciliation>"

projection:
  operator_view: "<smallest view, command, or receipt needed>"
  customer_or_partner_view: "<optional external-facing projection>"

non_goals:
  - "<what this applet must not own>"
```

## Layer Mapping

| Layer | This applet maps to |
|-------|---------------------|
| Applet / app | |
| Helm | |
| Axiom | |
| Organism | |
| Converge | |
| Mosaic | |
| Runtime Runway | |
| Commerce Rails | |

## Applet Readiness Gate

- [ ] One primary job is named.
- [ ] Functional, emotional, and relational needs are all present.
- [ ] Non-goals are explicit.
- [ ] At least one Truth or candidate Truth is identified.
- [ ] Approval, stop, retry, and reversal points are named.
- [ ] Runtime needs are separated from product meaning.
- [ ] Commercial needs are separated from app-local state.
- [ ] Provider identifiers are not treated as domain truth.
- [ ] The smallest useful projection can be described in one screen or one command.

## Open Gaps

- Gap:
  - Evidence:
  - Owner:
  - Next action:
