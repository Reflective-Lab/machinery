# Signal Capture And Improvement

Build-Depot should capture the smallest useful fact that lets the factory learn.
It should not store raw tool payloads as doctrine. GitHub, Linear, Sentry,
Just, and scheduled scans are inputs; Omnigraph records are the durable factory
facts.

The learning-loop doctrine lives in
[Learning And Feedback Loops](learning-feedback-loops.md). This document defines
the signal mechanics that make those loops possible.

High-volume sources must aggregate before they become graph facts. A Sentry
event flood, repeated flaky-test retry, or dependency advisory burst should
produce one stable signal per source grouping with counts and time-window
metadata, not thousands of individual graph nodes. Raw payloads remain in the
source system or a bounded evidence store; Omnigraph stores the durable issue,
finding, incident, or scorecard signal.

Runtime Runway deploy and runtime events use
`runtime-runway/ops/contracts/runtime-deploy-event-summary.schema.json`. The
summary envelope maps into existing `Deployment`, `Incident`, and
`FactorySignal` facts; the schema is closed so raw telemetry fields do not
silently enter the factory graph.

## Runtime Decision

Omnigraph runtime is intentionally deferred for one week while the expected
deployment shape is reassessed. The likely target is a local-network machine
with a large disk-backed store rather than a cloud-hosted GCS bucket.

This is a timing decision, not an architecture change:

- Build-Depot still owns graph schema, signal semantics, and scorecard
  interpretation.
- Trigger tasks may normalize and return graph records without delivering them
  while `OMNIGRAPH_INGEST_URL` is unset.
- Missing ingest is not drift during the deferral window.
- New work should keep emitting graph-compatible records and tests so the graph
  can be switched on without redesign.

Revisit the runtime decision after one week and record the chosen deployment
shape in [Reliable Delivery](reliable-delivery.md).

## Signal Contract

Every captured signal should answer at least one factory question:

| Question | Primary fact |
| --- | --- |
| Which repo is affected? | `Repository.name`, layer, cohort, tier |
| What happened? | finding, incident, CI verdict, release, adoption scan |
| How risky is it? | bucket, severity, priority, required/optional gate |
| Who owns the next action? | owner, Linear label, repo module label |
| What proves it is fixed? | drift check, CI status, standard, ADR, scorecard movement |
| Should Codex act now? | `codex_safe`, confidence, next action |

Signals should be stable and typed. Prefer closed enums, repo slugs, issue IDs,
and graph fields over prose when a value will be queried later.

## Capture Map

| Source | Captured signals | Omnigraph facts |
| --- | --- | --- |
| `QUALITY_BACKLOG.md` | `QF-*` finding lifecycle, bucket, status, owner, drift check, standards promoted | `Finding`, `RPProperty`, later `Standard` and edges |
| Linear | active implementation work, owner, priority, state, module label | `Finding`, `Repository` |
| GitHub Actions | CI/check suite result, release tag, repo activity | `Repository.ci_status`, `Repository.last_release` |
| GitHub PRs | PR review finding, gate verdict, changed files, repo context | PR-gate output now; future `Finding` and repo health facts |
| Sentry | issue/group id, project, severity, status, event count, affected users, first/last seen, linked `QF-*` if present | `Incident`, `FactorySignal`, `Repository` |
| Runtime Runway deploy summary | deploy id, service, app id, environment, version, status, summarized incidents, aggregate runtime deltas, evidence pointers | `Deployment`, `Incident`, `FactorySignal`, `Repository` |
| Repository adoption scan | cohort, tier, adoption state, missing required signals, exceptions | `Repository` adoption fields |
| Just recipes | local gate surface and pass/fail evidence | repo adoption facts and future check records |

`FactorySignal` is the broad learning lane for facts that are useful before
they become a formal finding. Categories are:

- `quality_gate`
- `security_scan`
- `dependency`
- `test`
- `github_lifecycle`
- `runtime_telemetry`
- `product_feedback`
- `architecture_drift`
- `agent_behavior`
- `operational_hygiene`
- `data_durability`
- `cost_capacity`
- `delivery`
- `repository_adoption`

For aggregated sources, `aggregate_key`, `first_seen_at`, `last_seen_at`,
`event_count`, and `affected_users` preserve flood shape without storing every
raw event.

## Graph Edges

The operating edge set connects findings, source signals, lifecycle objects,
and decisions:

| Edge | Use |
| --- | --- |
| `FindingInRepo` | Query repo-specific quality and delivery debt. |
| `IncidentInRepo` | Query repo-specific runtime incidents. |
| `FindingWorksOnRP` | Show which findings move recurring properties toward green. |
| `FindingToStandard` | Preserve provenance when a finding becomes a standard. |
| `FindingSupersedes` | Preserve quality-history lineage. |
| `PullRequestAddressesFinding` | Link PRs that cite or close a `QF-*` finding. |
| `CheckRunInRepo` | Preserve GitHub check run and check suite evidence by repo. |
| `DeploymentInRepo` | Link releases and deployment status to the repo they affect. |
| `RiskTracksFinding` | Keep accepted-risk provenance queryable. |
| `ADRDecidesFinding` | Link decisions to the findings that forced or resolved them. |
| `OwnerOwnsFinding` | Query active work by person or agent. |
| `SignalInRepo` | Query learning signals by affected repository. |
| `SignalSupportsFinding` | Preserve the source signal behind a finding. |

Learning-loop requirements that are not yet first-class graph edges should be
tracked as target schema work: gate efficacy (`caught_by` and
`should_have_caught`), escaped-defect autopsy, baseline ratchets, signal
exception rate, standing-red duration, debt half-life, expiry breaches, and loop
latency.

## Improvement Loop

Use the graph to close the learning loop:

1. Capture evidence from a tool, scan, incident, or review.
2. Normalize it into graph records with typed fields.
3. Query for drift, stale risks, missing owners, and repeated failure modes.
4. Promote repeated lessons into standards, ADRs, or repo contracts.
5. Add or strengthen a check so the same failure is caught earlier.
6. Feed the result into the scorecard and planning.

The factory improves only when facts change behavior. A signal that never feeds
a query, scorecard, gate, or issue should be removed or reshaped.

## Minimum Queries

The first operating queries should support these decisions:

- which active repos are red, unknown, or missing required gates
- which Bucket A or P1/P2 items are open without owners
- which incidents have no linked finding
- which repo adoption gaps lack Linear issues or accepted-risk entries
- which recurring properties are aspired rather than enforced
- which standards or drift checks were created from recent findings
- which repos have stale release or deployment signals

## Implementation Priorities

Until the Omnigraph runtime is live, prioritize graph readiness:

1. Keep `scripts/seed.ts`, `trigger.dev/debt-tracker.ts`, and
   `scripts/factory-adoption-doctor.ts` emitting compatible records.
2. Add missing `Standard` records and graph edges.
3. Add tests for every new record or edge shape.
4. Keep `queries/*.gq` aligned with `build-depot.pg`.
5. Keep scorecard metrics defined here even if the current export remains
   Markdown.
