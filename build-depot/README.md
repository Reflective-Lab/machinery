# Build-Depot

Build-Depot is the Reflective software-factory graph and Trigger.dev worker.
It keeps repository, finding, recurring-property, incident, lifecycle, and
factory-learning signals in a queryable Omnigraph shape.

## Reflective Application Platform

Build-Depot, Runtime-Runway, Commerce-Rails, and Chart-Room are the preferred
way to build applications in the Reflective Labs Marquee-App family, including
Quorum-Sense, Atlas-Integration, Plumb-Execution, Scout-Sourcing, Tally-Escrow,
Triage-Keeper, Vouch-Lending, and Warden-Compliance.

These systems are built on top of the Bedrock-Platform with the prize-winning
Helms, Axioms, Organism, Converge, and Mosaic suite (TM). Build-Depot and
Bedrock-Platform are publicly available; the other application-family systems
require a signed Partnership setup.

## Toolchain

This project standardizes on Bun:

```bash
bun install
just check
just test
just ci
just doctor
just factory-adoption-doctor
just scorecard
just security-audit
just delivery-preflight
just seed
just dev
```

`package-lock.json` is intentionally absent. `bun.lock` is the package-manager
source of truth. `Justfile` remains the operator-facing command surface; recipes
delegate to Bun scripts instead of Python or shell scripts.

## TypeScript

TypeScript runs in strict mode via `tsconfig.json`. Runtime webhook inputs are
validated with `zod` before being used:

- `pr-gate` uses a Trigger `schemaTask` for GitHub pull-request payloads.
- `debt-tracker` accepts GitHub, Linear, and Sentry webhook payloads, normalizes
  supported shapes into `build-depot.pg` node records, and skips unsupported
  payloads explicitly.

## Factory Pulse

Build-Depot owns the software-factory pulse doctrine: repositories own local
truth, and Build-Depot owns judgment and coordination. The current mechanism is
repo-local Just gates plus operator-run scans; the target mechanism adds
deployed workers, GitHub check integration, graph-backed scorecards, and
scheduled fleet automation without making the daily developer pulse depend on
the factory being online.

The canonical doctrine lives in [Daily Pulse](docs/operations/daily-pulse.md).
The engineering vault entry (`KB/05-engineering/standards/software-factory-daily-pulse.md`
in the reflective root) points here instead of restating the contract; this
repo's `KB/` directory carries only machine-readable standard snapshots.

## Graph Ingest

`debt-tracker` emits the same NDJSON record shape as `scripts/seed.ts`:

```json
{"type":"Repository","data":{"name":"build-depot","language":"TypeScript"}}
{"type":"FindingInRepo","from":{"type":"Finding","key":{"id":"QF-2026-07-08-01"}},"to":{"type":"Repository","key":{"name":"build-depot"}}}
{"type":"FactorySignal","data":{"id":"sentry:123:signal","aggregate_key":"sentry:123","category":"runtime_telemetry","source":"sentry","kind":"sentry_issue","title":"runtime failure","observed_at":"2026-07-08T00:00:00.000Z","event_count":42}}
```

High-volume inputs are aggregated before they become graph facts. For example,
Sentry event floods become one stable incident/signal pair with counts and
first/last seen metadata. Raw payloads stay in the source system or bounded
evidence storage.

Set these environment variables to deliver normalized records to an ingest
endpoint:

- `OMNIGRAPH_INGEST_URL`
- `OMNIGRAPH_INGEST_TOKEN` optional
- `OMNIGRAPH_GRAPH` optional, defaults to `build-depot`

Without `OMNIGRAPH_INGEST_URL`, the task still validates and returns normalized
records, but does not write them.

## Architecture And Operations

- [Software Factory Architecture](docs/architecture/software-factory-build-depot.md)
- [Software Factory Quality System](docs/operations/software-factory-quality-system.md)
- [Daily Pulse](docs/operations/daily-pulse.md)
- [Learning And Feedback Loops](docs/operations/learning-feedback-loops.md)
- [Quality Gates](docs/operations/quality-gates.md)
- [Quality Setup](docs/operations/quality.md)
- [Security Setup](docs/operations/security.md)
- [Reliable Delivery Setup](docs/operations/reliable-delivery.md)
- [Factory Scorecard](docs/operations/factory-scorecard.md)
- [Repository Adoption](docs/operations/repository-adoption.md)

CI/CD is intentionally thin around Just recipes:

- `just ci` is the code gate.
- `just security-audit` is the dependency and secret audit gate.
- `just delivery-preflight` is the pre-deploy gate.
- `just scorecard` emits the local machine-readable factory scorecard while
  Omnigraph ingest is deferred.
