# Bedrock Platform Consolidation — Design Spec

Date: 2026-07-05
Status: Approved design, awaiting implementation plan
Owner: Karl (Kenneth Pernyer)

## Purpose

Package the platform as one product in one repo: `bedrock-platform`. Deliverables
are library crates published to a private registry; engagements and Reflective
apps consume them with `cargo add <crate> --registry reflective-labs` and no sibling
checkouts, no relative-path wiring, no per-workspace version skew.

Today the "platform" is a two-level federation of ~15 git repos (two shell
repos — `bedrock-platform`, `mosaic-extensions` — each gitignoring nested
subsystem repos, plus `arena-tests` and `atelier-showcase`) stitched together
with `../../bedrock-platform/...` path dependencies. This spec collapses the
platform portion of that federation into one repo with one Cargo workspace.

## Decisions

| # | Decision | Choice |
|---|----------|--------|
| 1 | Consumption | Private registry: **Shipyard** (`reflective-labs` org, already provisioned: `ssh://git@ssh.shipyard.rs/reflective-labs/crate-index.git`) |
| 2 | Versioning | **Lockstep** — one platform version for all published crates, starting at **4.0.0** |
| 3 | Repo scope | Everything platform-side in **one root Cargo workspace**; arena and atelier are members with `publish = false` |
| 4 | Granularity | **Move crates as-is**; crate merging is post-consolidation backlog |
| 5 | Home & history | `bedrock-platform` absorbs the others via `git filter-repo --to-subdirectory-filter` + merge — blame and `log --follow` survive |
| 6 | Layout | Tiered subsystem-first: `foundation/` + `extensions/` at the root |
| 7 | Naming | Strict `{subsystem}-{crate}` package names, normalized **before first publish** |
| 8 | Helms | **Split at the headless seam**: headless spine into `foundation/helm/`; GUI/app/prio remainder stays out, targeted at a future app-platform packaging |
| 9 | Publish gating | `publish = true` is earned: consumers + tests required. Unproven crates move in but stay unpublished |
| 10 | Two-product world | Bedrock = the SDK. Helms-remainder + runtime-runway + commerce-rails = a future **application platform** packaging that consumes Bedrock via the registry |
| 11 | Software factory | Bedrock carries its own copy of the established factory — doctor gates, CI workflows, factory scripts, skills, AGENTS.md — adapted from the root repo to single-repo shape |
| 12 | Self-contained KB | Bedrock's `kb/` stands alone: relevant RP-* standards, playbooks, and current skills content included; an engagement cloning bedrock-platform needs nothing from the root repo |
| 13 | Dual-track build | The consolidated tree is built in a **fresh clone** on a long-lived `consolidation/main` branch; the current multi-repo setup keeps building unchanged until an explicit cutover. Sync from the old mains is mechanical (filter-repo is deterministic → incremental re-merges). The dual period is tracked debt with an expiry; cutover happens when consuming apps are on registry deps |
| 14 | Dependency direction & leanness | **Upstream-only rule**: every crate belongs to a declared layer and may depend only on strictly-upstream layers. Transport/server stacks (tonic/axum/hyper) are quarantined to an explicit service-surface layer, never in the engine core. Enforced, not aspirational: Plan A ships a boundary audit; arena's dim-layering + dim-crate-footprint become the permanent gate |

### Rationale highlights

- **Shipyard over Kellnr**: already provisioned; managed service vs. self-run
  infra. Kellnr's only wins (self-hosting, air-gap, rustdoc hosting) are not
  current constraints. All publishes flow through one CI job, so a later
  registry migration stays cheap.
- **4.0.0, not 1.0.0**: the lockstep version must be ≥ every current version;
  converge sits at 3.9.3. A "platform 1.0" would publish converge crates
  backwards.
- **Lockstep**: one number to communicate; trivial compatibility story; simple
  release automation (Bevy/rust-analyzer model). Cost accepted: a patch to one
  crate bumps all.
- **Strict naming before first publish**: the registry is empty, so renames
  cost only internal import updates, compiler-verified. Names on a registry
  are effectively forever. `converge-*` squats on crates.io become irrelevant;
  every internal dep carries `registry = "reflective-labs"` explicitly, closing the
  dependency-confusion hole.

### Registry configuration (Shipyard, registry name `reflective-labs`)

Committed in-repo at `bedrock-platform/.cargo/config.toml` (must exist before
any dep says `registry = "reflective-labs"` — cargo rejects unknown registry
names at metadata time):

```toml
[registries.reflective-labs]
index = "ssh://git@ssh.shipyard.rs/reflective-labs/crate-index.git"

[net]
git-fetch-with-cli = true   # git CLI handles SSH auth; libgit2 does not
```

Per-user, NEVER committed (secrets pattern: Keychain + direnv):

- SSH key `~/.ssh/id_ed25519_shipyard` registered with Shipyard (index access).
- Download-auth token (stable-rust hack until RFC 3193 lands) in
  `~/.cargo/config.toml`: `[http] user-agent = "shipyard <token>"` — token
  minted on the Shipyard Tokens page, stored in Keychain, materialized by
  the user, not by any repo file. On nightly, `[unstable] registry-auth = true`
  replaces the user-agent hack.
- Publish token likewise per-user / CI-secret only.

## Target layout

```
bedrock-platform/
├── Cargo.toml            # ONE root workspace, lockstep 4.0.0
├── Cargo.lock            # one lockfile
├── Justfile              # just build / test / check / publish
├── foundation/
│   ├── axiom/            # truth/JTBD compilation        (from nested repo)
│   ├── converge/         # governance engine             (from nested repo)
│   ├── organism/         # intent/planning/simulation    (from nested repo)
│   └── helm/             # headless trust-transfer spine (split from helms)
├── extensions/
│   ├── arbiter/  crucible/  embassy/  ferrox/
│   ├── manifold/ mnemos/    prism/    soter/
├── arena/                # quality gates & contract tests  (publish = false)
├── atelier/              # scenarios & tutorials           (publish = false)
├── docs/
└── kb/                   # per-subsystem kb/ kept in place + root index
```

Subsystems keep their internal `crates/` trees (decision 4). Navigation is
`foundation/converge/crates/kernel` — subsystem first, two levels to source.
Extension directories drop suffixes (`arbiter-policy` → `extensions/arbiter`);
the parent directory now carries that meaning.

### Workspace mechanics

- Root `Cargo.toml` lists all crates as members; the ~15 existing
  `[workspace]` roots and per-workspace `Cargo.lock`s are deleted.
- `workspace.package`: `version = "4.0.0"`, `edition = "2024"`,
  `rust-version = "1.96.0"` (already uniform across workspaces).
- All `../../` path deps become `workspace = true` deps resolved in root
  `workspace.dependencies`. External-dependency version skew between the old
  workspaces gets reconciled here, once, permanently.
- `default-members = foundation crates` — bare `cargo build` / `cargo test`
  is the fast core loop; `--workspace` is the full sweep.

## Naming normalization (decision 7)

Strict `{subsystem}-{crate}`. Applied at workspace unification, before any
publish. Directory moves and package renames are separate commits.

| Current | New | Note |
|---------|-----|------|
| `converge-arbiter-policy` | `arbiter-policy` | mosaic crates lose false `converge-` prefix |
| `converge-mnemos-knowledge` | `mnemos-knowledge` | |
| `converge-fuzzy-inference` | `prism-fuzzy` | lives in prism-analytics |
| `converge-embassy-{source}` | `embassy-{source}` | 15 connector ports |
| `converge-manifold-adapters` | `manifold-adapters` | |
| `converge-crucible-models` | `crucible-models` | |
| `converge-soter-smt` | `soter-smt` | |
| `application-kernel` | `helm-kernel` | helms substrate gains subsystem prefix |
| `application-storage` | `helm-storage` | |
| `truth-catalog` | `helm-truth-catalog` | |
| `capability-core` | `helm-capability-core` | |
| `capability-registry` | `helm-capability-registry` | |
| `director-contracts` | `helm-director-contracts` | |

`converge-*` crates that genuinely belong to converge keep their names. The
full rename table is produced mechanically during implementation (audit every
`[package] name` against its subsystem home).

## Helms: the headless seam (decision 8)

Empirical basis: the three `helm-*-headless` atelier scenarios plus arena's
contract tests define the boundary. The realtime/multiparticipant/sync spine
(`helm-session-host`, `helm-client`, session/director contracts) already
tests headlessly — it belongs below the GUI line, in Bedrock.

**Into `foundation/helm/` (~15 crates):**

- Contracts: `helm-module-contracts`, `helm-session-contracts`,
  `director-contracts`
- Spine: `helm-client`, `helm-session-host`, `helm-coordination`,
  `helm-governed-jobs`, `helm-truth-execution`, `helm-operator-control`
- Substrate: `application-kernel`, `application-storage`, `truth-catalog`,
  `capability-core`, `capability-registry` (renamed into `helm-*`, see above)

**Stays out (app-platform material, remains in the helms repo for now):**
desktop Tauri app, `workbench-backend`, the entire `prio-*` capability-module
family (verified: `prio-agent-ops` is a capability manifest with
gRPC/OpenAPI/GraphQL surface — app-platform catalog, not spine),
`crm-contracts`/`application-contracts`, `plugin-runtime`, `notes`,
`seed-gen`, `prio-apple-notes-cli`, and atelier's `crm-helm` scenario (it
depends on `runway-app-host`/`runway-storage` — already app-layer).

**Amendment (2026-07-08, owner decision on RFL-171):** the crm-helm exile is
temporary. Bedrock OWNS the substrate injection APIs (app-host mounting,
event hub, session ownership, storage kit) as foundation/helm trait
contracts; `runway-app-host`/`runway-storage` are implementations injected
at the composition root. Once Seam A (RFL-171) lands, crm-helm repatriates
to atelier against an in-memory substrate implementation, and
`helms/showcase/` empties. Exile was the workaround; inversion is the fix.

**Blocking seam work (RFL-128 territory) — two edges to cut before the helm
import phase:**

1. `helm-operator-control` → `workbench-backend`
2. `helm-operator-control` → `prio-agent-ops` (and arena's direct
   `prio-agent-ops` dep)

Operator-receipt / agent-run / traceability *vocabulary* the spine needs moves
into `helm-module-contracts`. Once cut, `foundation/helm` has zero app-side
dependencies, and arena + the headless scenarios become the permanent
regression surface for the seam: any future app-side dependency in
`foundation/helm` fails arena.

**Doctrine update required:** `KB/04-architecture/current-system-map.md`
"Bedrock owns Helm" becomes "Bedrock owns the headless Helm spine; the Helm
application surface (workbench, prio capability modules, desktop) is
application-platform territory." Update the registry first, then sweep citing
READMEs (per the registry's own rule).

## Publish gating (decision 9)

Audit result (2026-07-05): 172 crates total; 80 with zero internal consumers,
of which ~57 are intentional leaf binaries (scenarios, tutorials, examples,
servers, apps), 3 are in-flight RFL-128 work, 2 are self-running harnesses.
The real dead-weight candidates: the **15 `embassy-*` connector ports** (zero
consumers, zero tests) plus `application-contracts`.

Rule: a crate ships `publish = true` only when it has (a) at least one
consumer (internal crate, scenario, or app) and (b) tests. Everything else
moves in with `publish = false` and a single Linear issue lists the
unpublished set with promotion criteria (test suite + consuming atelier
scenario). "What's published" is the honest product catalog; nothing is
deleted (history preserves everything anyway).

Day-one published set: foundation crates + extension crates with consumers
and tests. Day-one unpublished: 15 embassy ports, `application-contracts`,
and anything else failing the rule at migration time.

## Software factory & self-contained KB (decisions 11–12)

Bedrock-platform must run the same quality factory the root repo established,
without reaching outside its own clone.

**Factory port (from `~/dev/reflective`):**

- `just` recipes: `check`/`build`/`test`/`fmt`/`lint`/`doc` collapse from the
  root's fleet loops (`_bedrock-loop`, `ws-*`) to plain single-workspace
  invocations — the consolidation makes most of the orchestration layer
  unnecessary, which is the point.
- Doctor gates: `project-doctor` + `quality-doctor` recipes move in with all
  mechanized RP checks (RP-RELEASE-TRAIN-INTEGRITY, RP-LAYERING,
  RP-CRATE-SIZE-BUDGET, RP-SNAPSHOT-PORTABLE, RP-RUSTC-DRIFT-CONTAINED,
  RP-BRANCH-HYGIENE, RP-POLICY-FRESH, check 9 layering — now also guarding
  `foundation/helm` against app-side deps).
- CI workflows adapted to single-repo shape: `doctor.yml` (gate on every
  push/PR), `hermetic-audit.yml`, `test-code-attribution.yml`,
  `fresh-clone.yml` (dramatically simpler — no sibling checkouts), plus the
  new `publish.yml` (Shipyard, tag-triggered). `factory-alert.yml` (fleet
  andon) stays in the root repo — it watches the fleet, and the fleet shrinks.
- Operating layer: `.claude/skills/` playbooks (branch/pr/check/test/wip/
  merge-cleanup/…), `SKILLS.md`, and an `AGENTS.md` derived from the root
  policy, all adjusted for single-repo workflows and current — not stale
  copies of superpowers-era defaults.

**Self-contained KB:** bedrock's `kb/` must let an engagement operate without
the root repo. It absorbs: the per-subsystem `kb/`s (already moving with
their subsystems), the RP-* standards that the doctor gates enforce (an
engagement must be able to read the rule a failing check cites), the
engineering playbooks that apply inside the repo, and a root `kb/README.md`
index. Personal/strategy/commercial KB content stays in the root repo — the
boundary is "what a consuming engineer needs," not "everything Karl knows."
Root-repo KB pages that remain canonical (e.g. the boundary registry) are
referenced by URL, not copied — one source of truth.

### Amendment to decisions 11-12 (2026-07-08, owner direction)

The software factory is becoming a specialized service: **build-depot** (the
factory graph + Trigger.dev workers — repo/finding/recurring-property/incident
signals, factory-adoption-doctor, security-audit, delivery-preflight). It sits
beside **runtime-runway** (run/deploy/host), **commerce-rails** (commercial
authority), and — added 2026-07-09 — **chart-room** (company operations:
strategy alignment, drift, roadmap; private) as the four specialization repos. Product repos — Bedrock included —
stay LIGHTWEIGHT on these aspects: thin adapters and registration, not forked
machinery.

Decision 11 therefore evolves: Bedrock does NOT carry a wholesale copy of the
factory. It keeps (a) fast local gates (just check/test/fmt/lint), (b) arena —
which is product substance (the quality proof layer), not factory tooling, and
(c) minimal CI that CALLS build-depot's capabilities. Doctors, finding
tracking, security audit, and delivery preflight are consumed from build-depot.
Decision 12 (self-contained KB) is scoped accordingly: self-contained for a
CONSUMING ENGINEER (standards the gates cite, playbooks, architecture), not
factory internals. Plan C is written against this shape.

### Amendment to decision 12 + layout (2026-07-09, owner direction)

**ONE knowledge base at the repo root.** The per-subsystem `kb/` directories
(foundation/*/kb, extensions/*/kb, kb/mosaic) consolidate into the root `kb/`
— fully worked through, capturing all dimensions of the platform, one story.
Stray markdown outside root entry points and `kb/` is minimized. Supersedes
the original "per-subsystem kbs kept in place + root index" line.

**Full recursive Justfile.** Beyond the honest `ci` gates: recipes to capture
and run the system in all phases predictably, working recursively down into
subsystem directories from each level (root orchestrates; per-subsystem
operation remains possible).

**GitHub-grade README**: true to the story, module explanations, Mermaid
architecture illustrations.

## Dependency direction & leanness (decision 14)

Bedrock must be lean, self-contained, and performant (build and runtime),
with dependency **direction** as a first-class rule: a crate may only depend
on things strictly upstream of it. Provisional layer order (upstream first):

1. **Engine core** — converge model/pack/kernel/core/provider/experience/
   optimization/storage. No transport, no server frameworks, no app domains.
2. **Reasoning layers** — axiom, organism. Depend on engine contracts only.
3. **Helm headless spine** (Plan B).
4. **Extensions** — specialists consuming converge-shaped contracts.
5. **Service surface** — converge-protocol, converge-client, converge-runtime
   (tonic/prost/axum/hyper live HERE and only here, behind features).
6. **Proof** — arena, atelier (publish = false; may depend on anything).

Nothing in Bedrock depends on the app platform (runway/commerce/helms-app) —
that direction is already policed by project-doctor check 9.

**Known violations at spec time (audited 2026-07-07), verdicts pending:**

- `axiom → converge-manifold-adapters` (foundation → extension, via crates.io
  1.1.2, `llm-all`): axiom should depend on a converge provider contract;
  manifold gets wired at the composition root (scenario/app).
- `organism`, `axiom`, `organism-catalog-seed` → `converge-runtime`
  (reasoning layer → service surface): the engine consuming its own server.
- Transport stack inside converge (protocol/client/runtime) — decide whether
  it stays as Bedrock's layer 5 (publish-gated, feature-gated, out of
  default-members) or migrates to the app-platform packaging. Runway already
  consumes protocol/client, which argues for keeping them published — but as
  an explicitly separate layer, never a dependency of layers 1–4.
- Plan B watch-item: `truth-catalog`'s registered truths are business-domain
  content (`qualify-inbound-lead`, `submit-expense-report`). The catalog
  mechanism is platform; the CRM truth content likely belongs app-side.

**Enforcement:** Plan A produces `kb/consolidation/boundary-audit.md` (full
crate-level graph, layer classification, every violating edge) with one
Linear issue per violation family. Fixes are verdict-driven follow-ups —
Plan A moves code as-is and does NOT fix edges in-flight. Go-forward, the
layer map becomes data for arena's dim-layering (violations fail the gate)
and dim-crate-footprint watches build weight (transport deps in layers 1–4
are automatic failures).

## Dual-track build & cutover (decision 13)

The consolidation must not interrupt apps that build against the current
sibling-checkout layout. Therefore:

- All import/unification work happens in a fresh clone
  (`~/dev/reflective/bedrock-consolidated`) and merges to branch
  `consolidation/main` on the bedrock-platform remote. The existing `main`,
  every nested checkout, and every relative path dep stay untouched.
- Source repos are only ever **cloned** during import — never modified.
  There is no freeze; the manifest is a *snapshot*. Upstream work that lands
  on the old mains after the snapshot (e.g. RFL-128) is pulled in by
  re-running the same filter-repo import against the updated main —
  deterministic rewriting makes this an incremental merge.
- **Cutover criteria:** 4.0.0 published to Shipyard, consuming apps moved to
  registry deps (runway's git-tag pins are the template), factory CI green on
  `consolidation/main`. Then `consolidation/main` becomes `main`, the old
  layout is archived, and the moved-aside checkouts are removed with Karl's
  authorization.
- The dual period is tracked debt: one Linear issue with an expiry date and
  a running list of upstream commits not yet synced. RFL-128 no longer gates
  phases 1–3 (snapshot imports the mains); it still gates phase 4.

## Migration phases

Each phase lands green: `cargo test` (later `--workspace`) + arena dimensions
+ project-doctor clean. No shims, no disabled tests carried across; anything
that can't pass gets a same-day Linear issue (RP-BRANCH-HYGIENE).

0. **Freeze & snapshot** — announce a freeze point per nested repo; all
   in-flight branches land or are explicitly parked. CI green everywhere.
1. **Restructure bedrock shell in-place** — the shell repo starts tracking
   content: import nested axiom/converge/organism repos under `foundation/`
   via filter-repo + merge. (helms is NOT imported here — see phase 4.)
2. **Import extensions** — 8 nested mosaic repos → `extensions/{name}/`;
   mosaic shell repo's docs/kb absorbed; arena → `arena/`, atelier →
   `atelier/`. Helm-dependent members (arena's helm contract tests, the
   three headless scenarios) are imported but stay **excluded from the
   workspace** until phase 4 — an explicit, gated exclusion listed in the
   phase-4 Linear issue, not a shim.
3. **Unify workspace** — root `Cargo.toml`, delete sub-workspace roots and
   lockfiles, rewrite path deps to workspace deps, lockstep 4.0.0, naming
   normalization, publish flags per decision 9. The one big-bang commit;
   mechanical, verified by full suite + arena.
4. **Helm split import** — gated on the two RFL-128 seam edges being cut.
   Headless spine crates → `foundation/helm/` (filter-repo keeps their
   history); helms repo retains the app-platform remainder. Headless
   scenarios + arena helm contract tests now run against `foundation/helm`.
5. **Port the software factory & self-contain the KB** — per decisions
   11–12: single-workspace Justfile, doctor gates, CI workflows (sibling-
   checkout gymnastics from RFL-132 deleted), skills/AGENTS.md operating
   layer, RP-* standards and playbooks into `kb/` with root index. Root
   repo's fleet Justfile and factory-alert updated to the shrunken fleet.
6. **Ship** — Shipyard publish job (`cargo workspaces publish` in dependency
   order, `registry = "reflective-labs"` on all internal deps), tag `v4.0.0`,
   archive absorbed repos with pointer READMEs, update Linear/doc links,
   update the boundary registry.

Phases 1–3 do not wait on RFL-128; phase 4 does.

## Risks

- **Full clean build is heavy** (~145 crates in Bedrock). Mitigated:
  `default-members = foundation`, machine-wide sccache, CI cache keyed on the
  single lockfile.
- **Feature unification** in one workspace can change what a crate compiles
  with vs. today. Detector: arena's dimension suite at every phase gate.
- **External-dep version skew** across 15 old workspaces reconciles in
  phase 3 — unknown size; an audit lands in the implementation plan before
  timing commitments.
- **Freeze coordination** matters most for helms (active RFL-128 work) —
  which is why helms imports last, behind its own gate.
- **Rename churn** (decision 7) touches every `use` of renamed crates —
  mechanical and compiler-verified, but it must be its own commit series,
  never mixed with moves.

## Out of scope (tracked follow-ups)

- **Application-platform packaging**: helms-remainder + runtime-runway +
  commerce-rails as a second consolidated product consuming Bedrock via
  Shipyard (runway already consumes converge `v3.4.0` git tags — pattern
  proven). Own project, own spec.
- **Crate merging/pruning** within subsystems (decision 4 backlog).
- **lattice-mesh** (docs-only today) joins the app packaging when it grows
  code; **forge-templates** may later ship as `cargo generate` templates in
  the platform repo; **blueprint-apps** is doctrine/KB, never a crate
  deliverable.
- **Embassy port promotion**: per-port test + consuming scenario → publish.
