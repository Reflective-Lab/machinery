# Repository Adoption

Build-Depot is the software-factory authority for the Reflective workspace.
Each repository benefits from the factory by emitting local evidence through a
small, consistent contract. Repositories should not define their own software
factory; they should point here and expose the signals Build-Depot can read.

## Goal

Make factory adherence mechanical:

1. Every active repo exposes the same minimum evidence surface.
2. Build-Depot scans the fleet and records adoption state as graph facts.
3. Missing signals become findings or Linear issues.
4. Accepted exceptions have an owner, reason, and revisit date.
5. The scorecard shows adoption health alongside quality, security, and
   delivery health.

## Adoption Contract

An adopted repo has these signals:

| Signal | Required behavior |
| --- | --- |
| `AGENTS.md` | Points to Build-Depot for software-factory policy and keeps repo-local guidance scoped to that repo. |
| Tool pointers | `CLAUDE.md`, `CODEX.md`, `GEMINI.md`, or equivalent tool files are short pointers to `AGENTS.md`. |
| `Justfile` | Exposes `just ci`; repo-local checks may be richer, but CI and operators call the Just recipe. |
| CI | Workflows are thin runners that install dependencies and call `just ci` or a documented factory gate. |
| Security | Exposes `just security-audit`, a repo-specific security gate, or an explicit accepted-risk entry. |
| Delivery | Production-affecting repos expose a preflight or release gate through Just. |
| Checkout currency | Active repo scans are run on a checkout that is not behind its upstream branch; stale scans do not produce trusted adoption verdicts. |
| Private Rust registry | Registry-published Rust workspaces keep Cargo `registry = "reflective-labs"` attribution and any non-secret registry index configuration needed for local Cargo metadata. The target state keeps registry credentials and publish orchestration in Build-Depot (Kellnr on the build server); in-repo publish workflows remain adoption debt until the depot publisher is live and the repo is stripped. |
| Linear | Uses a stable repo/module label so issues and findings can be linked to repository facts. |
| Incidents | Production repos document their Sentry project or state that they do not emit runtime incidents. |
| Factory docs | Local docs refer to Build-Depot for quality gates, scorecard schema, recurring properties, and factory graph semantics. |
| Exceptions | Any missing required signal is captured as a finding, Linear issue, or accepted risk with a revisit date. |

Repos can add stronger local policy, but they should not redefine workspace
factory doctrine. A repo-local doc can use this wording:

> Software-factory policy and quality-gate semantics are owned by
> `build-depot/`. This repo emits local evidence through its Just recipes,
> tests, CI, incidents, and findings; Build-Depot normalizes that evidence into
> the workspace factory graph.

## Generated Canon Docs

The "Factory docs" contract signal extends to a repo's *generated canon* — any
doc that renders quality, security, performance, or gate status. These are
factory-owned concerns, so they follow one rule continuously:

- **Repo-owned:** product/platform canon (architecture, safety, business,
  values) and *local-evidence views* — a repo's own numbers (arena scores,
  `cargo audit`, benchmark baselines) rendered thinly, with a header pointing to
  the Build-Depot scorecard / recurring property. A local-evidence view states
  facts about *this repo*; it does not define the gate.
- **Build-Depot-owned:** the gate surface (`quality-gates.md`), scorecard schema
  (`factory-scorecard.md`), security operations doctrine (`security.md`), and the
  recurring-property definitions (`recurring-properties.json`). A repo's
  gate-status doc is a **pointer** to these, never a second definition.
- **Freshness is a recurring property.** `RP-DOC-CANON-FRESH` governs it:
  curated canon carries a freshness stamp and is stale-gated; generated canon
  regenerates byte-identically from evidence. The executable check is repo-local
  and thin-runner-shaped (e.g. `bedrock-consolidated`'s `just canon-check`), and
  emits its verdict as a `documentation_trust` FactorySignal into the graph.

The test: could a repo's canon doc drift from the factory truth and mislead a
reader? If yes, it is restating doctrine and must become a pointer or a stamped
local-evidence view.

## Adoption Tiers

Adoption should roll out by operational risk, not alphabetically.

| Tier | Scope | Required outcome |
| --- | --- | --- |
| Tier 0 | Build-Depot | The control plane passes its own doctors, CI, security, and delivery preflight. |
| Tier 1 | Release-train and production repos | Full contract, security signal, delivery signal, Linear label, and incident mapping. |
| Tier 2 | Active apps, sites, and libraries | Full contract except delivery/incident fields may be declared not applicable. |
| Tier 3 | Templates, experiments, archived repos | Pointer docs and explicit lifecycle status; missing gates are not treated as active drift if the repo is marked archived or experimental. |

Tier 1 currently means the repos that can block releases, carry production
risk, or publish shared packages. The concrete list should come from
`release-train.yaml`, GitHub repository state, and active Linear ownership.

## Adoption Cohorts

Rollout is organized by cohorts in `factory-cohorts.json`. A cohort is a
wrapper for planning, scanning, scorecards, and Linear work. It is not
automatically a monorepo.

| Cohort | Scope | Plan |
| --- | --- | --- |
| A | `build-depot`, `commerce-rails`, `runtime-runway`, `forge-templates`, `lattice-mesh` | Factory proof cohort. Adopt first and use it to prove that Build-Depot works across the control plane, runtime, commerce, templates, and an active subsystem. |
| B | `alias-apps`, `blueprint-apps`, `marquee-apps`, `studio-apps` | App-family rollout. Apply the lighter app contract: `AGENTS.md`, `Justfile`, `just ci`, CI calling Just, security evidence or accepted risk, and Linear label. |
| C | `bedrock-platform`, `arena-tests`, `atelier-showcase`, `mosaic-extensions`, `bedrock-consolidated` | Consolidation transition. Pre-consolidation repos should get compatibility pointers only; full Tier-1 adoption targets `bedrock-consolidated`. |
| D | `beacon-sites` plus storytelling, business plans, masterplan, and similar collateral | Docs-collateral governance. Track owner, freshness, source-of-truth links, and Build-Depot policy pointers instead of forcing service-style gates. |

### Group A Wrapper

Group A gets a virtual wrapper in Build-Depot, not a new big repo. The wrapper
is `factory-cohorts.json` plus the adoption doctor, graph fields, scorecard,
and Linear issues that operate over cohort `A`.

Do not merge Group A into one big repo unless these conditions become true:

- one owner can make release decisions for every member
- most changes require atomic edits across the whole group
- the repos share one deploy or release lifecycle
- local development is simpler as one workspace than as separate repos
- the security and incident boundary is clearer after merging

Those conditions do not hold today. `runtime-runway`, `commerce-rails`,
`build-depot`, `forge-templates`, and `lattice-mesh` have different risk,
release, and ownership shapes. A virtual wrapper gives the factory one control
surface without creating artificial coupling.

## Adoption Doctor

Build-Depot provides a fleet scan through:

```bash
just factory-adoption-doctor
```

The doctor should report a repo-by-repo matrix:

- repo name and local path
- tier
- adoption state: `adopted`, `partial`, `blocked`, `exempt`, or `unknown`
- missing required signals
- linked Linear issue or accepted-risk reference for each missing signal
- last scan time and source revision

Required checks:

- `AGENTS.md` exists and points to Build-Depot for software-factory policy.
- Tool-specific instruction files are short pointers, not independent policy.
- Local docs do not re-specify workspace software-factory doctrine.
- `Justfile` exists for active repos and exposes `ci`.
- CI workflows call Just recipes instead of re-implementing factory policy.
- Security gate exists or an accepted-risk reference is present.
- Delivery gate exists for production-affecting repos or is declared not
  applicable.
- The scanned checkout is not behind its configured upstream branch when Git
  upstream metadata is available.
- Registry-published Rust workspaces retain Cargo registry attribution and
  non-secret Cargo registry configuration while private registry credentials and
  publish orchestration are in Build-Depot target state. A transition issue can
  explain remaining in-repo publish machinery, but it does not make the signal
  present.
- Linear repo label exists.
- Sentry project mapping exists for production repos or is declared not
  applicable.
- Stale compatibility docs point to Build-Depot canonical docs.

The doctor should exit nonzero for required Tier 0 and Tier 1 gaps unless the
gap is tied to an accepted risk or active Linear issue.

## Graph Facts

Adoption state should be queryable in Omnigraph rather than trapped in scan
logs. Build-Depot should model at least:

- repository adoption tier
- adoption state
- required signals present or missing
- exception references and revisit dates
- Linear label and linked adoption issue
- Sentry project mapping or not-applicable declaration
- last successful scan revision and timestamp

These facts should feed the factory board, PR gate context, and scorecard.
The first graph surface stores these fields on `Repository` and exposes them
through `queries/repository_adoption.gq`.

## Scorecard Slice

The factory scorecard should include:

- adopted repos by tier
- adopted repos by cohort
- partial or blocked repos by tier
- missing signals by type
- accepted adoption risks past revisit date
- Tier 1 repos without current CI, security, or delivery evidence
- repos carrying local software-factory policy instead of Build-Depot pointers

## Linear Plan

The adoption rollout belongs in the existing Linear project
`E13 - Software Factory: Build-Depot`.

Planned work:

- `RFL-165` - define the repository adoption contract and keep this document
  canonical.
- `RFL-166` - implement the factory adoption doctor.
- `RFL-167` - model adoption status and adoption gaps in Omnigraph.
- `RFL-168` - roll out Group A factory proof cohort.
- `RFL-169` - publish an adoption board and scorecard slice.
- `RFL-173` - roll out Group B app-family adoption.
- `RFL-174` - adopt `bedrock-consolidated` as the Group C target.
- `RFL-175` - define Group D docs-collateral governance.

Rollout refinements:

- `RFL-168` represents the Group A factory proof cohort.
- `RFL-173` represents the Group B app-family rollout.
- `RFL-174` represents the Group C consolidation transition whose adoption
  target is `bedrock-consolidated`.
- `RFL-175` represents Group D docs-collateral governance.

Each issue should include acceptance criteria that tie back to this document, a
repo-by-repo adoption state, and either green evidence or tracked exceptions.

## Operating Rule

If a repo and Build-Depot disagree about software-factory semantics,
Build-Depot wins. Update the repo to point here, or record an explicit
exception with a reason, owner, and revisit date.
