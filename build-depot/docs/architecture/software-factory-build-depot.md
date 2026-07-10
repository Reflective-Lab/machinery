# Build-Depot Software Factory Architecture

Build-Depot is the factory control plane for Reflective engineering work. It
turns repository, quality, security, delivery, and incident signals into typed
Omnigraph facts, then feeds those facts back into PR review, Linear planning,
operator scorecards, and Converge.

This repo is the canonical home for software-factory doctrine, quality-gate
semantics, factory graph semantics, and scorecard interpretation. Parent and
other repo docs should refer here rather than re-specifying the factory.

## Goals

- Keep software-factory state queryable instead of trapped in comments, CI logs,
  or issue bodies.
- Normalize GitHub, Linear, Sentry, and scheduled fleet observations into one
  graph schema.
- Make quality, security, and delivery health visible before incidents or stale
  backlog entries expose the same drift later.
- Keep local and CI verdicts aligned through Bun, strict TypeScript, and
  Justfile recipes.

## System Context

Build-Depot sits between the systems that produce signals and the systems that
consume factory state.

```text
GitHub webhooks      Linear webhooks      Sentry webhooks      Scheduled scans
       |                   |                    |                    |
       +-------------------+--------------------+--------------------+
                                    |
                              Trigger.dev
                                    |
             +----------------------+----------------------+
             |                                             |
       PR gate task                              Debt tracker task
             |                                             |
 GitHub PR comment/checks              Omnigraph Repository/Finding/
                                       Incident/RP/Standard/FactorySignal records
                                                        |
                         +------------------------------+----------------+
                         |                                               |
                  Graph queries                                  Scorecard export
                         |                                               |
        Operator views, runbooks, Linear planning              Converge signals
```

## Runtime Components

- **Trigger.dev tasks** run factory work. `pr-gate` reviews pull requests and
  `debt-tracker` normalizes incoming GitHub, Linear, and Sentry payloads.
- **Bun + strict TypeScript** is the project automation runtime. External data
  is validated with `zod` before it reaches graph-facing records.
- **Justfile** is the operator command surface. CI and humans both call the
  same recipes.
- **Terraform** wires cloud-facing resources: service account, storage,
  secrets, and webhooks.
- **Omnigraph** stores the normalized graph defined in `build-depot.pg`.
- **Private Rust distribution** is a factory operation when Reflective crates
  are published to the private Kellnr registry (build server). Rust repos keep Cargo registry attribution in
  manifests and may carry non-secret registry index configuration required for
  local Cargo metadata. The target state is for Build-Depot to own credentials,
  release-tag reaction, and publish orchestration; until the depot publish
  worker and secret values are live, any in-repo publish workflow is transition
  debt rather than completed adoption.

Omnigraph runtime is deferred for one week while the deployment target is
reassessed. The current likely target is a local-network machine with a large
disk-backed store. This does not change the graph contract: Build-Depot should
continue producing Omnigraph-compatible records and tests while ingest is
disabled.

## Signal Sources

| Source | Events | Primary task | Factory facts |
| --- | --- | --- | --- |
| GitHub | pull request, push, release, check run, check suite | `pr-gate`, `debt-tracker` | Repository, Finding, delivery status |
| Linear | issue create/update/state changes | `debt-tracker` | Finding, Repository, ownership, status |
| Sentry | issue create/update/resolve/ignore | `debt-tracker` | Incident, Repository, aggregate runtime telemetry signal, linked finding |
| Kellnr registry (build server) | release tags, publish dry runs, private crate publish results | scheduled or webhook-triggered delivery tasks | Deployment, FactorySignal, Repository release metadata |
| Scheduled scans | fleet CI, releases, dependency/security status, repository adoption | scheduled Trigger tasks | Repository health, adoption state, scorecard inputs |

## Graph Model

`build-depot.pg` is the source of truth for graph shape.

- `Repository` tracks repo identity, layer, CI state, release freshness, Sentry
  project, Linear label, deployment target, adoption cohort, and factory
  adoption state.
- `Finding` tracks quality/security/delivery work by bucket, area, status,
  confidence, owner, and next action.
- `Incident` tracks Sentry incidents and optional links back to findings.
- `RPProperty` tracks recurring factory properties.
- `Standard` tracks promoted engineering standards.
- `PullRequest`, `CheckRun`, and `Deployment` track GitHub lifecycle evidence
  that proves whether a fix moved through review, gates, and release.
- `Risk`, `ADR`, and `Owner` track governance and accountability links.
- `FactorySignal` tracks broad learning inputs such as security scans,
  dependency advisories, flaky tests, runtime telemetry, product feedback,
  architecture drift, agent behavior, operational hygiene, data durability,
  cost/capacity, delivery, and repository adoption.

High-volume sources are aggregated before they enter the graph. For example,
Sentry event floods become one `Incident` plus one `FactorySignal` keyed by the
Sentry issue, with event counts, affected users, and first/last seen fields.
Raw payloads remain in Sentry or a bounded evidence store.

Edges make derived health queryable:

- `FindingInRepo`
- `FindingWorksOnRP`
- `FindingToStandard`
- `FindingSupersedes`
- `IncidentInRepo`
- `PullRequestAddressesFinding`
- `CheckRunInRepo`
- `DeploymentInRepo`
- `RiskTracksFinding`
- `ADRDecidesFinding`
- `OwnerOwnsFinding`
- `SignalInRepo`
- `SignalSupportsFinding`

## Feedback Loops

Build-Depot is useful only when graph facts return to the systems where people
make decisions.

- **PR loop:** `pr-gate` should publish one stable Build-Depot review comment
  and a GitHub check/status. Blocking verdicts fail closed.
- **Backlog loop:** Linear is the source of truth for project work. Factory
  findings from Linear remain linked to repository labels and graph records.
- **Incident loop:** Sentry issues become `Incident` records. P1/P2 incidents
  without linked findings are surfaced as quality debt.
- **Scorecard loop:** graph queries produce quality, security, and delivery
  metrics for the factory scorecard and Converge.

## Quality, Security, And Delivery Controls

The full system loop lives in
[Software Factory Quality System](../operations/software-factory-quality-system.md).
The gate surface lives in [Quality Gates](../operations/quality-gates.md).
The repo contract and rollout model live in
[Repository Adoption](../operations/repository-adoption.md).
The signal contract and runtime deferral live in
[Signal Capture And Improvement](../operations/signal-capture.md).

Quality controls:

- `just ci` is the code-red gate: strict typecheck plus Bun tests.
- `just quality-doctor` checks structural quality drift.
- `docs/operations/quality.md` defines the local operating model.

Security controls:

- `just secrets-scan` checks tracked files for obvious committed secrets.
- `just security-audit` runs dependency audit plus secret scan.
- `terraform/secrets.tf` keeps secret slots explicit without committing values.
- `docs/operations/security.md` defines secret and audit expectations.

Reliable delivery controls:

- `just delivery-preflight` runs doctor checks and the local CI gate.
- `.github/workflows/ci.yml` is a thin runner around `just ci`.
- `.github/workflows/security.yml` runs scheduled and manual security checks.
- `.github/workflows/delivery.yml` gates manual production deploys with
  preflight.
- `docs/operations/reliable-delivery.md` defines deploy, rollback, and smoke
  expectations.

## Failure Modes

| Failure | Expected behavior |
| --- | --- |
| Unknown webhook payload | Validate, skip explicitly, and return a skipped reason. |
| Invalid model output | Fail closed for PR-gate verdicts. |
| Omnigraph unavailable | Return normalized records and mark ingest failure without losing payload context. |
| Missing secret | Fail the task or workflow early with a named missing variable. |
| Audit advisory wave | Security workflow can go red without making `just ci` code-red. |
| Duplicate webhook delivery | Idempotent graph writes and stable PR comments avoid duplicate state. |

## Linear Implementation Map

The current implementation work is tracked under Linear project
`E13 - Software Factory: Build-Depot`.

- `RFL-156` graph ingest sink.
- `RFL-157` webhook ingress and safe routing.
- `RFL-158` graph edges and derived repository health.
- `RFL-159` scheduled fleet status and board.
- `RFL-160` stable PR review and check results.
- `RFL-161` Sentry incident lifecycle.
- `RFL-162` factory scorecard and Converge-ready signals.
- `RFL-163` production deployment and operations runbook.
- `RFL-165` repository adoption contract.
- `RFL-166` factory adoption doctor.
- `RFL-167` repository adoption status in Omnigraph.
- `RFL-168` Group A factory proof cohort rollout.
- `RFL-169` repository adoption board and scorecard slice.
- `RFL-173` Group B app-family adoption rollout.
- `RFL-174` Group C `bedrock-consolidated` adoption target.
- `RFL-175` Group D docs-collateral governance.

The current rollout cohort map lives in `factory-cohorts.json`.

## Architecture Rules

- Add new signal sources by validating payloads at the boundary, then mapping
  into typed internal records.
- Do not write ad hoc graph shapes. Update `build-depot.pg`, queries, seed
  generation, tests, and docs together.
- Keep delivery gates behind Just recipes. Workflows install tools and call
  recipes; they do not redefine policy.
- Keep security checks separate from `just ci` unless the finding is tied to the
  current commit rather than the outside world.
- Prefer append-only operational history. Linear tracks active work; graph facts
  and Markdown docs explain the durable architecture and operating rules.
