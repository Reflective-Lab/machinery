# Deploy Ops Learning Contract

Runtime Runway owns deploy-time observation and immediate operational action.
Build-Depot owns the durable factory learning graph. These are one loop, not two
separate graphs.

## Decision

Deploy-ops learning uses the same logical OmniGraph owned by Build-Depot. Runtime
Runway must not create a second runtime-owned graph database for deployment
learning.

The split is:

- **Dash0, Cloud Monitoring, Cloud Logging, Sentry:** live/raw telemetry stores.
- **Runtime Runway:** emits deploy/runtime facts and makes immediate ops
  decisions.
- **Build-Depot OmniGraph:** stores durable learning facts used by scorecards,
  planning, runbooks, and future agents.

The graph is not treated as schemaless operational storage. Build-Depot's
`build-depot.pg` is the schema contract. New nodes and edges are welcome when
they create queryable learning value, but raw logs, traces, per-request events,
and high-cardinality metrics do not belong in OmniGraph.

## What Runtime Emits

Runtime Runway should publish summarized, source-stable records that Build-Depot
can normalize into existing or extended graph facts:

- deploy started, succeeded, failed, rolled back;
- service, app id, environment, version, commit SHA;
- config, secret, Terraform, and container image fingerprints;
- preflight, deploy, post-deploy smoke, and rollback results;
- Sentry issue count delta and affected-user delta;
- latency, error-rate, saturation, cold-start, and cost deltas;
- storage/auth/LLM/GPU failure summaries;
- agent action taken, runbook used, confidence, and human escalation state;
- operator acknowledgement or override when a human is required.

Build-Depot can already represent the first slice through `Deployment`,
`Incident`, and `FactorySignal`. If the next slice needs richer semantics, extend
Build-Depot deliberately with typed fields or edges rather than pushing raw
telemetry into graph nodes.

## What Stays Out

Do not store these in OmniGraph:

- raw logs;
- raw traces;
- every request or span;
- raw Sentry event floods;
- user payloads, prompts, email bodies, Slack message bodies, secrets, tokens;
- high-cardinality metric series.

Those remain in the live observability or source systems with their own
retention, privacy, and query controls. OmniGraph stores the durable conclusion:
what happened, what it affected, what decision was made, and what should change.

Example graph-level fact:

```text
Deployment api-server@3.6.0 in prod increased p95 latency by 40%, opened Sentry
issue runtime-runway-123, rollback recovered service health in 7 minutes, and
created a finding for a missing storage preflight check.
```

## Decision Loop

1. Runtime Runway deploys or changes runtime state.
2. Runtime Runway observes source-near signals from Cloud Run, Firebase, Sentry,
   OpenTelemetry, storage, auth, GPU workers, and app-host health.
3. Runtime Runway agents act near the source when the action is bounded by a
   runbook: retry, scale, pause, roll back, disable a worker, or open a finding.
4. Humans are notified only when judgment, approval, or accountability is
   required.
5. Build-Depot ingests the summarized event and records graph facts.
6. Future deploy preflights, runbooks, scorecards, and agent policies use those
   graph facts to improve the next decision.

## Agents Act Near The Source

Agents should not listen to Slack channels or parse email inboxes as their
primary control loop. Slack and email are human notification surfaces, not source
truth.

Agent triggers should come from:

- deployment events;
- Sentry issue webhooks;
- OpenTelemetry-derived alerts;
- Cloud Run and Firebase health signals;
- Runtime Runway event-log records;
- Build-Depot graph findings or scorecard deltas;
- explicit operator commands.

The action should happen through Runtime-owned APIs, deploy scripts, queues, or
Trigger.dev tasks with bounded payloads. The notification should describe the
action, current system state, and what human decision is needed, if any.

## Slack, Email, And Trigger.dev

Slack and email provider connections belong in Runtime Runway when they are app
or operator notification transports. Runtime owns provider config, OAuth/secrets,
rate limits, retries, delivery receipts, audit metadata, and redaction.

Meaning remains with the owning layer:

- Apps own user-facing copy and domain reason.
- Helm owns operator-control semantics and approval/audit shape.
- Build-Depot owns factory alerts generated from graph findings, incidents, and
  scorecard drift.

Trigger.dev belongs in the loop as an out-of-band worker/orchestrator, not as a
request-path runtime. Use it for scheduled checks, webhook normalization,
long-running remediation, fan-out notifications, and graph ingestion. Do not make
Slack/email parsing or channel watching the agent command source.

## Build-Depot Schema Direction

Use the same Build-Depot OmniGraph, extending it only when current facts cannot
answer deploy-learning questions.

Likely graph evolution:

- richer `Deployment` fields for commit, service, app id, image digest,
  preflight result, smoke result, rollback state, and recovery time;
- `FactorySignal(kind=deploy_regression|rollback|agent_action|human_escalation)`
  for summarized operational observations;
- edges from deployments to incidents, signals, findings, agent actions, and
  rollback records when Build-Depot needs those joins.

Until those extensions exist, Runtime should emit graph-compatible summaries
that can be represented as current `Deployment`, `Incident`, and
`FactorySignal` records.
