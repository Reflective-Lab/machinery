# Software Factory Quality System

Build-Depot is the single source of truth for Reflective's software-factory
quality system. Other workspace docs may point here, but they should not define
their own software-factory process.

The factory is a learning quality control plane for the whole Reflective
workspace. It is not just a checklist. Its core loop is:

1. Capture a finding as evidence.
2. Classify it by risk and area.
3. Fix it or record accepted risk.
4. Promote the lesson into a standard or ADR.
5. Add a drift check so it is caught next time.
6. Feed the result into the scorecard and Build-Depot graph.

The thesis is simple: a bug, incident, flaky gate, bad release, or AI shortcut
should not just get fixed once. It should become evidence, then a standard,
then a check, then a signal in the scorecard.

## Layers

### Evidence And Governance

These artifacts hold durable state:

- `QUALITY_BACKLOG.md` - append-only findings ledger.
- `KB/05-engineering/standards/` - promoted engineering standards.
- `KB/04-architecture/decisions/` - ADRs for settled decisions.
- `KB/06-operations/risk-register.md` - accepted risks and revisit dates.
- `KB/06-operations/factory-scorecard.md` - historical scorecard rows until
  Build-Depot graph/export fully owns scorecard generation.
- Linear - live project/issue state.

Build-Depot does not replace the ledger or Linear. It indexes, normalizes, and
connects them as graph facts.

### Executable Checks

Executable checks are the factory's drift sensors:

- `just doctor` - aggregate workspace drift gate.
- `just quality-doctor` - policy, ledger, RP table, and snapshot consistency.
- `just agents-doctor` - cross-agent instruction-file drift.
- `just shim-doctor` - first-class workaround and disabled-test debt.
- `just project-doctor` - release train and workspace structural invariants.
- `just ci` - project-local code gate.
- fresh-clone checks - clean-machine build/test confidence.
- release checks - semver, packaging, yank, and train integrity.
- security/audit checks - dependency, secret, and supply-chain risk.

The canonical gate surface is documented in [Quality Gates](quality-gates.md).

### Control Plane

Build-Depot is the control plane:

- It normalizes GitHub, Linear, Sentry, scheduled scans, and repo-local Just
  signals into Omnigraph records.
- It is the target owner for private distribution operations, including
  private-registry (Kellnr) publish orchestration and credentials for Rust workspaces that
  expose only structural Cargo registry facts. Until the depot publisher is
  live, in-repo publish workflows are tracked transition debt.
- It owns graph facts for repositories, findings, incidents, recurring
  properties, standards, lifecycle evidence, aggregate factory signals, and
  scorecard inputs.
- It feeds PR gates, repo health, incident views, finding queries, and
  Converge-ready quality signals.

## Recurring System Properties

The factory checks are best understood as recurring system properties, not
isolated lint rules. The current workspace ledger tracks 18 `RP-*` properties,
with the latest snapshot reporting 16 currently green.

The property families include:

- deterministic tests
- hermetic unit tests
- semver gating
- release-train integrity
- crate size budgets
- rustc pinning
- CI parity
- AI evidence citation
- AI shortcut declaration
- first-class shims
- branch hygiene
- typed cross-layer semantics
- policy freshness
- snapshot portability
- yank discoverability
- test/code attribution
- auto-bless auditability
- fresh-clone confidence

Build-Depot treats these as typed factory facts. The ledger remains the
historical source for finding lifecycle, but software-factory interpretation
belongs here.

## Ownership Rule

Other repos should not define the Reflective software factory. They should:

- own their local product/platform behavior
- expose local Just recipes and CI verdicts
- record findings in their ledger or Linear
- point to Build-Depot for factory doctrine, quality gates, scorecard schema,
  and graph semantics

The concrete repo contract and rollout tiers are documented in
[Repository Adoption](repository-adoption.md).

The signal contract and deferred Omnigraph runtime decision are documented in
[Signal Capture And Improvement](signal-capture.md).

When a repo-specific doc needs to mention the factory, use this wording:

> Software-factory policy and quality-gate semantics are owned by
> `build-depot/`. This repo emits local evidence through its Just recipes,
> tests, CI, incidents, and findings; Build-Depot normalizes that evidence into
> the workspace factory graph.

## Canonical Documents

- [Architecture](../architecture/software-factory-build-depot.md)
- [Quality Gates](quality-gates.md)
- [Quality Operations](quality.md)
- [Security Operations](security.md)
- [Reliable Delivery](reliable-delivery.md)
- [Signal Capture And Improvement](signal-capture.md)
- [Factory Scorecard](factory-scorecard.md)
- [Repository Adoption](repository-adoption.md)
