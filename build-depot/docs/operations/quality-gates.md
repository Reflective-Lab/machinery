# Quality Gates

Build-Depot owns the canonical quality-gate semantics for the Reflective
software factory.

## Workspace Gate

At the root workspace, `just doctor` is the aggregate drift gate. As of the
current root `Justfile`, it runs four checks and sums their exit codes:

1. `quality-doctor` - meta-policy, ledger, snapshot, recurring-property, and
   cross-reference consistency.
2. `agents-doctor` - cross-agent instruction drift and stale planning-source
   references.
3. `shim-doctor` - first-class shims, disabled tests, and workaround debt.
4. `project-doctor` - release-train structure, package boundaries, fixture
   portability, rustc pinning, release-history schema, path-dependency
   direction, branch hygiene, and related workspace invariants.

Parent workspace docs that still describe `just doctor` as two checks are stale
and should be changed to point here.

### Root Doctor Check Inventory

`quality-doctor` currently covers:

- policy files present and tracked: `AGENTS.md`, `QUALITY_BACKLOG.md`
- snapshot `Last review` freshness
- `QF-*` IDs cited in `AGENTS.md` resolve in the ledger
- `RP-*` tracked-by entries reference open findings or the no-tracker marker
- backlog cross-reference paths exist or are explicitly first-use placeholders
- root tool pointer files exist and are tracked: `CLAUDE.md`, `CODEX.md`,
  `GEMINI.md`
- snapshot bullet counts match `just snapshot`
- generated `RP-*` table matches
  `KB/05-engineering/standards/recurring-properties.json`

`agents-doctor` currently covers:

- every nested git repo has `AGENTS.md`
- tool-specific files are short pointers to `AGENTS.md`
- dead home-directory instruction references are absent
- milestone-file references are marked archived because Linear is source of
  truth

`shim-doctor` currently covers:

- every `SHIM(` marker resolves to a live ledger finding
- shim expirations are enforced
- known workaround smells, including bare ignored tests and commented-out tests,
  require a nearby marker

`project-doctor` currently covers:

- `release-train.yaml` is parseable and every member directory exists
- publishable crates do not path-depend on unpublishable crates
- no source file larger than 1 MiB appears in publishable workspaces
- `.stderr` fixtures do not leak machine-specific absolute paths
- train workspaces pin rustc to an exact channel
- `KB/release-history.md` entries have required yank fields
- upstream workspaces do not path-depend into product workspaces
- there are no stranded stashes across the fleet
- Foundation to `runtime-runway` path dependencies require an explicit
  `RP-HELMS-SUBSTRATE-SEAM` exemption

### Root Helper Gates

These helper gates are part of the pre-migration factory surface and remain
accounted for by Build-Depot:

- `factory-status` - fleet status board used by scheduled factory alerting.
- `snapshot` - derives `QUALITY_BACKLOG.md` snapshot counts.
- `rp-table` - prints generated recurring-property table Markdown.
- `rp-table-sync` - updates the generated `RP-*` table in
  `QUALITY_BACKLOG.md`.
- `release-train-sync` - root-scoped release-train YAML shape check.
- `check-all-fresh` - fresh-clone check across configured workspaces.
- `fresh-ws` - per-workspace fresh check helper.
- `test-all-fresh` - fresh-clone test pass across configured workspaces.

### Release Gates

These release gates are preserved as part of the factory surface:

- `release-preflight`
- `release-deps-audit`
- `release-package-size-check`
- `release-public-api-check`
- `release-undo`
- `release-history-audit`
- `release-preflight-all`
- `release-migrate-deps`
- `release`
- `release-all`

### Root CI Workflow Gates

These parent-workspace workflows are also part of the pre-migration factory
surface. Build-Depot should observe them, not replace them silently:

- `.github/workflows/doctor.yml` - runs the doctor family.
- `.github/workflows/factory-alert.yml` - scheduled fleet status alerting.
- `.github/workflows/fresh-clone.yml` - fresh-clone build/test confidence.
- `.github/workflows/hermetic-audit.yml` - no-network runtime audit for
  hermetic tests.
- `.github/workflows/test-code-attribution.yml` - production/test change
  attribution enforcement.

## Project Code Gate

Every project should expose a local `just ci` command. CI workflows should be
thin runners that install dependencies and call that recipe rather than
re-implementing the gate in workflow YAML.

The full repo-level adoption contract is documented in
[Repository Adoption](repository-adoption.md).

For Build-Depot itself:

```bash
just ci
```

runs:

- strict TypeScript typecheck
- Bun tests

## Build-Depot Setup Gate

Build-Depot also has local setup doctors:

```bash
just doctor
just quality-doctor
just security-doctor
just delivery-doctor
```

These checks verify the Build-Depot factory control plane itself: docs,
scripts, TypeScript strictness, workflows, Terraform surface, graph schema, and
Trigger.dev task files.

The fleet repository-adoption gate is exposed separately:

```bash
just factory-adoption-doctor
```

It scans workspace repos against the Build-Depot adoption contract and exits
nonzero for required Tier 0 or Tier 1 gaps without an accepted-risk or Linear
reference.

## Security Gate

Security is deliberately separate from code-red `just ci`:

```bash
just security-audit
```

It runs dependency audit and secret scanning. Dependency advisories can be
world-red without being caused by the current commit, so they are routed as
security findings rather than silently folded into every code gate.

## Delivery Gate

Production-affecting Build-Depot changes should pass:

```bash
just delivery-preflight
```

This runs the setup doctors and local CI before deploy.

For Shipyard-published Rust workspaces such as `bedrock-consolidated`, local
quality gates must remain independent of private registry credentials. Cargo
manifest registry attribution and non-secret registry index configuration are
structural Cargo facts. Shipyard credentials and topological publish execution
are target-state delivery operations for Build-Depot; until the depot publisher
exists, any in-repo publish workflow should be treated as explicit transition
debt rather than proof of completed adoption.

## Gate Interpretation

- A passing local gate is evidence, not proof of universal health.
- A failing drift gate is factory debt until proven obsolete.
- A failing dependency audit should create or update tracked work.
- A bypass, disabled test, shim, or ignored gate must cite a finding or Linear
  issue.
- If a gate and a doc disagree, the Justfile and Build-Depot docs win; update
  the stale doc.
