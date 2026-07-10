# Linear Work Items

These are the Linear records created for the deploy-ops learning and
source-near notification work.

## Project: E14 — Runtime Deploy Ops Learning

URL: https://linear.app/reflective-labs/project/e14-runtime-deploy-ops-learning-720b5c05392f

Team: `RFL`

Labels:

- `module:runtime-runway`
- `area:deploy-ops`
- `area:observability`
- `area:automation`

Summary:

Runtime Runway should emit deployment and runtime summaries that Build-Depot
normalizes into the same OmniGraph used for factory learning. Live telemetry
stays in Sentry, Dash0/OTel, Cloud Logging, Cloud Monitoring, Firebase, and GCP;
OmniGraph stores durable learning facts. Agents act near the source of truth,
not by reading Slack or email.

Description:

Runtime Runway currently owns auth, telemetry, storage, deployment, and app-host
execution. Build-Depot owns OmniGraph and Trigger.dev factory workers. We need a
deploy-ops loop that connects these without creating a second runtime graph.

Deliverables:

- Define Runtime Runway deploy event summaries: deploy start/success/failure,
  rollback, service, app id, environment, version, commit SHA, image digest,
  config/secrets/Terraform fingerprints, preflight result, smoke result,
  recovery time.
- Emit summarized operational deltas: Sentry issue count, affected users, p95,
  error rate, cold starts, saturation, cost, storage/auth/LLM/GPU failure
  summaries.
- Define which facts map to existing Build-Depot `Deployment`, `Incident`, and
  `FactorySignal` records, and which graph schema extensions are needed.
- Keep raw telemetry out of OmniGraph: logs, traces, per-request events, raw
  Sentry floods, prompts, user payloads, Slack/email message bodies, secrets,
  and high-cardinality metrics stay in source systems.
- Define source-near agent triggers from deployment events, Sentry webhooks,
  OTel-derived alerts, Cloud Run/Firebase health, Runtime event logs,
  Build-Depot graph findings, and explicit operator commands.
- Define bounded agent actions: retry, scale, pause, roll back, disable worker,
  open/update finding, request human approval.
- Add runbook and validation coverage for deploy learning ingestion.

Acceptance criteria:

- Runtime Runway documentation names Build-Depot's OmniGraph as the single
  deploy-learning graph.
- Runtime emits graph-compatible deploy/runtime summaries without raw telemetry.
- Build-Depot has a documented mapping for current `Deployment`, `Incident`,
  and `FactorySignal` records or explicit schema extension tasks.
- Agents are triggered by source-near events, not Slack/email parsing.
- A production deploy can be traced from deploy event to live telemetry to
  graph summary to follow-up finding or improved preflight.

References:

- `runtime-runway/kb/Architecture/Deploy Ops Learning Contract.md`
- `runtime-runway/kb/Architecture/Observability Contract.md`
- `build-depot/build-depot.pg`
- `build-depot/docs/architecture/software-factory-build-depot.md`

## Issue: RFL-214 — Runtime notification transports and source-near action loop

URL: https://linear.app/reflective-labs/issue/RFL-214/runtime-notification-transports-and-source-near-action-loop

Team: `RFL`

Labels:

- `module:runtime-runway`
- `area:notifications`
- `area:automation`
- `area:deploy-ops`

Summary:

Add Runtime-owned Slack/email notification transports and Trigger.dev handoff
rules so humans are notified when judgment or approval is required, while agents
continue to act from source-near system signals.

Description:

Slack and email should be notification and escalation surfaces, not the control
plane for agents. Agents should not watch Slack channels or parse inboxes as
their primary trigger. They should receive bounded events from Sentry, OTel/GCP,
Runtime event logs, deployment workflows, Build-Depot graph findings, or
explicit operator commands.

Deliverables:

- Define Runtime-owned notification transport contracts for Slack and email:
  provider config, secrets, OAuth/API credentials, rate limits, retries,
  delivery receipts, audit metadata, and redaction.
- Define notification payload shape: system state, affected service/app,
  severity, action already taken, required human decision, deadline/escalation,
  links to Sentry/Dash0/Cloud Run/OmniGraph/Linear.
- Define when Runtime sends notification directly versus when Build-Depot sends
  factory alerts from graph findings or scorecard drift.
- Define Trigger.dev usage boundaries: scheduled checks, webhook normalization,
  long-running remediation, fan-out notifications, graph ingestion; never
  request-path runtime and never Slack/email parsing as agent input.
- Add a source-near agent action interface: trigger source, runbook, allowed
  action, confidence, idempotency key, audit trail, human-escalation reason.
- Add tests or contract fixtures for notification redaction and retry behavior.

Acceptance criteria:

- Slack/email are documented and implemented as human notification transports.
- Agents have source-near triggers and do not depend on Slack/email message
  parsing.
- Notification payloads include enough context for humans to act without
  becoming raw telemetry dumps.
- Trigger.dev usage is bounded to async/off-path automation.
- Delivery receipts and agent actions can be summarized into Build-Depot
  `FactorySignal` records.

References:

- `runtime-runway/kb/Architecture/Deploy Ops Learning Contract.md`
- `runtime-runway/kb/Architecture/Observability Contract.md`
- `build-depot/docs/operations/signal-capture.md`
