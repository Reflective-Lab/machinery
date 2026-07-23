# KB Consolidation Merge Map (Appendix)

Generated: 2026-07-09. Read-only analysis of `~/dev/reflective/bedrock-consolidated`.
Companion to the consolidation plan. Target = ONE root `kb/`.

Legend for Action:
- MOVE → relocate into target taxonomy (content already single-story-ready)
- MERGE→X → fold into named survivor file; source dies
- REWRITE → content moves but MUST be reconciled to one-story (federation paths, versions, trunk model)
- STAY → entry-point doorstep; not kb-content; remains in place
- DELETE → stale / duplicate / dangling-stub; dies

---

## 0. Source inventory (real counts)

| Source | md files | Nature |
|---|---:|---|
| `kb/` (excl mosaic) | 42 | current root kb (partially reconciled) |
| `kb/mosaic/` | 52 | federation-era duplicate kb of old `mosaic-extensions` repo |
| `foundation/converge/kb/` non-Governance | 137 | richest subsystem kb |
| `foundation/converge/kb/Governance/` | 148 | NOT kb-content — embedded governance validator subsystem (cedar, fixtures, personas, src, tests) |
| `foundation/axiom/kb/` | 34 | subsystem kb |
| `foundation/organism/kb/` | 49 | subsystem kb |
| `extensions/*/kb/` (8) | ~85 | uniform per-extension kb |
| `atelier/kb/` | 16 | tooling/render kb |
| `foundation/axiom/architecture/` | 8 | kb-content living OUTSIDE kb (ADRs already stubbed to kb/) |
| `atelier/truths` + `atelier/quality` + `atelier/docs/strategy` | 16 | mixed: quality authoring + strategy dupes |

Total kb-classified md across all `*/kb/*` dirs (non-vendor): 563.

---

## 1. Root `kb/` (excl mosaic) — 42 files

| Source | Action | Target |
|---|---|---|
| `kb/Home.md` | REWRITE | `kb/Home.md` — Zone-sites table + federation INDEX link block are stale |
| `kb/INDEX.md` | REWRITE | `kb/07-decisions/INDEX.md` or root — ALL locations federation paths (`converge/`, `../mosaic-extensions/`, `../../runtime-runway/`) → monorepo paths |
| `kb/LOG.md` | MOVE | `kb/LOG.md` (append-only mutation log; keep at root) |
| `kb/Observations.md` | MOVE | `kb/07-decisions/Observations.md` |
| `kb/Architecture/Ecosystem Overview.md` | REWRITE→02 | `kb/02-architecture/Ecosystem Overview.md` — layer map must match README 6-layer model |
| `kb/Architecture/Operating Authority Boundary.md` | MOVE | `kb/02-architecture/boundaries/Operating Authority Boundary.md` |
| `kb/Architecture/Helm Coordination Layer.md` | MOVE | `kb/02-architecture/subsystems/helm/Coordination Layer.md` |
| `kb/Architecture/Storage Strategy.md` | MOVE | `kb/05-operations/Storage Strategy.md` |
| `kb/Architecture/Connector Architecture.md` | MERGE→embassy deep-dive | dedupe vs organism + embassy connector docs |
| `kb/Architecture/Fuzzy Rule Grammar.md` | MOVE | `kb/02-architecture/subsystems/arbiter/Fuzzy Rule Grammar.md` |
| `kb/Architecture/Fuzzy Variable Inventory.md` | MOVE | `kb/02-architecture/subsystems/axiom/Fuzzy Variable Inventory.md` |
| `kb/Architecture/Rule Activation Contract.md` | MOVE | `kb/03-contracts-and-seams/Rule Activation Contract.md` |
| `kb/Architecture/Marquee Applications Feedback Loop.md` | REWRITE→06 | `kb/06-consumption/` — marquee-apps is federation sibling |
| `kb/Doctrine/*.md` (3) | MOVE | `kb/01-platform-story/doctrine/` |
| `kb/Standards/*.md` (4) | MOVE | `kb/04-quality/standards/` |
| `kb/Concepts/MOQ.md` | MOVE | `kb/01-platform-story/concepts/MOQ.md` |
| `kb/Business/Reflective Commerce Rails.md` | REWRITE→06 | `kb/06-consumption/Commerce Rails.md` — commerce-rails is federation sibling |
| `kb/Audits/*.md` (3) | MOVE | `kb/08-history/audits/` |
| `kb/History/*.md` (4) | REWRITE→08 | `kb/08-history/` — Stack Compass / Release Notes carry v3.x + federation surface |
| `kb/Experiments/EXP-00{1,2,3}.md`, `LOG.md` | MERGE | see §9 experiments |
| `kb/consolidation/*.md` (8) | MOVE | `kb/07-decisions/consolidation/` — CURRENT TRUTH, keep verbatim |

## 2. `kb/mosaic/` — 52 files — federation-era duplicate

Whole subtree is the old `mosaic-extensions` repo kb. Uses dead crate names
(`arbiter-policy/`, `crucible-models/`, `mnemos-knowledge/`) and dead paths.
Superseded by the 8 per-extension kbs + root Architecture.

| Source group | Action | Note |
|---|---|---|
| `kb/mosaic/Modules/{Arbiter,Crucible,Embassy,Ferrox,Manifold,Mnemos,Prism,Soter}.md` | MERGE→extension deep-dives | fold any unique content into `kb/02-architecture/subsystems/<ext>/`; then DELETE |
| `kb/mosaic/Architecture/*.md` (11) | MERGE→02 | Extension Topology / Converge Boundary / Port Provider Boundary → survivors in `03-contracts-and-seams`; rest DELETE after harvest |
| `kb/mosaic/Standards/*.md` (7) | MERGE→04 | Extension Standard / Suggestor Contract survive in `04-quality/standards`; dupes DELETE |
| `kb/mosaic/docs/strategy/*.md` (6) | DELETE | exact dupes of `atelier/docs/strategy/*` (see §8) |
| `kb/mosaic/{Home,INDEX,LOG,Ecosystem,Positioning,Capability Matrix,REVIEW-GUIDE}.md` | DELETE | federation meta; replaced by root + extension INDEX/Home |
| `kb/mosaic/Building/*`, `Planning/*`, `Workflow/*`, `Experiments/*`, `History/*` | MERGE/DELETE | harvest unique, else DELETE |

Survivor rule: `kb/mosaic` produces ZERO surviving files at its path — everything
either folds into a root-taxonomy survivor or dies.

## 3. `foundation/converge/kb/` non-Governance — 137 files (survivor-rich)

Converge is the deepest kb; it wins most same-topic collisions.

| Source group | Action | Target |
|---|---|---|
| `Architecture/ADRs/ADR-00{1..8}.md` + README | MOVE (SURVIVOR) | `kb/07-decisions/adr/` — canonical ADR home (axiom stubs already point here) |
| `Standards/ADR/ADR-006-typed-id-newtype.md` | MERGE→07 | fold into `kb/07-decisions/adr/`; resolve ADR-006 number clash (converge ADRs already have an ADR-006) |
| `Architecture/{System Overview, Hexagonal Architecture, Layer Contract, Purity Rules, Dependency Rules, Plug Boundary, Ports, Providers}.md` | MOVE | `kb/02-architecture/subsystems/converge/` + `03-contracts-and-seams/` |
| `Architecture/{Suggestor Contract, Suggestor Catalog, Type Protocol, Transport Protocols, Storage Boundary}.md` | MOVE (SURVIVOR) | `kb/03-contracts-and-seams/` — beats mosaic + axiom copies |
| `Architecture/Audits/*.md` (5, 2026-04-11) | MOVE | `kb/08-history/audits/converge/` |
| `Architecture/{Known Drift, Runtime Retirement, Next-Steps}.md` | REWRITE | reconcile to 4.0.0 main-only; likely fold into `07-decisions/consolidation` |
| `Algorithms/*.md` (17) | MOVE | `kb/02-architecture/subsystems/converge/algorithms/` (unique reference set) |
| `Concepts/*.md` (11) | MOVE (SURVIVOR) | `kb/01-platform-story/concepts/` — canonical domain concepts |
| `Philosophy/*.md` (5) | MERGE→01 | `kb/01-platform-story/` — reconcile vs axiom/organism "Why" files |
| `Building/*.md` (11) | MOVE | `kb/05-operations/build/converge/` |
| `Modules/*.md` (4), `Integrations/*.md` (4), `Stack/*.md` (10) | MOVE | `kb/02-architecture/subsystems/converge/` + `05-operations/stack/` |
| `Planning/*.md` (8) | REWRITE→08 | milestones move to Linear; keep as history only |
| `Workflow/*.md` (4) | MERGE→05 | one workflow story (see §7) |
| `History/CHANGELOG.md`, `Baselines/*`, `Experiments/*` | MOVE/MERGE | §9 |
| `Home/INDEX/LOG/Ecosystem.md` | DELETE | subsystem meta replaced by root |

## 4. `foundation/converge/kb/Governance/` — 148 files — NOT KB

This is a running governance-validator subsystem (`validator/src`, `validator/tests`,
`cedar/policies`, `personas/*`, `fixtures/*`, `schemas/*`, `reports/*`).
It lives under `kb/` by accident of the old vault layout.

ACTION: OUT OF SCOPE for kb consolidation — RELOCATE the whole tree to a code
path (e.g. `foundation/converge/governance/`) as a separate task, OR confirm it
is fixtures for a crate and move accordingly. Do NOT drag 148 non-doc files into
the root kb. Only the handful of genuine narrative docs (if any `*.md` under
`Governance/docs/`) get harvested into `kb/04-quality/` after review.

## 5. `foundation/axiom/kb/` (34) + `foundation/axiom/architecture/` (8)

| Source | Action | Target |
|---|---|---|
| `axiom/architecture/adr/ADR-00{1..5}.md` | DELETE | already "compatibility path" STUBS pointing to `kb/Architecture/ADRs/` — dangling forward-refs; converge ADRs are the real ones |
| `axiom/architecture/adr/README.md` | DELETE | stub index |
| `axiom/architecture/{ARCHITECTURE,API_SURFACES}.md` | MERGE→02 | fold into `kb/02-architecture/subsystems/axiom/` |
| `axiom/kb/Architecture/*.md` (11) | MOVE | `kb/02-architecture/subsystems/axiom/` |
| `axiom/kb/Concepts/*.md` (9) | MOVE | `kb/01-platform-story/concepts/axiom/` (Truth Documents, Predicates, JTBD…) |
| `axiom/kb/Philosophy/*.md` (3) | MERGE→01 | one "Why the platform" story |
| `axiom/kb/Building/*.md` (3) | MOVE | `kb/05-operations/build/axiom/` |
| `axiom/kb/Marquee/*.md` (2) | MOVE | `kb/06-consumption/scenarios/` |
| `axiom/kb/Workflow/Daily Journey.md` | MERGE→05 | §7 |
| `axiom/kb/{Home,INDEX,LOG,Experiments/*}` | DELETE/MERGE | §9 |

## 6. `foundation/organism/kb/` — 49 files

| Source group | Action | Target |
|---|---|---|
| `Architecture/*.md` (11) | MOVE | `kb/02-architecture/subsystems/organism/` |
| `Architecture/{Converge Contract, Migration from Converge}.md` | REWRITE→03 | "Migration from Converge" is federation-era; reframe as in-monorepo boundary or DELETE |
| `Concepts/*.md` (11) | MOVE | `kb/01-platform-story/concepts/organism/` |
| `Philosophy/{Why Organism, The Gap, Relationship to Converge, Key Invariants}.md` | MERGE→01 | reconcile vs converge Philosophy |
| `Handoffs/*.md` (3) | REWRITE→08 | dated cross-repo handoffs; history only, reconcile paths |
| `Integration/Converge Experience Store.md` | MOVE | `kb/03-contracts-and-seams/` |
| `Audits/*.md` (2) | MOVE | `kb/08-history/audits/organism/` |
| `Stack/*.md` (2), `Building/*.md` (3) | MOVE | `05-operations` |
| `Planning/*.md` (1) | REWRITE→08 | history |
| `Workflow/*.md` (5) | MERGE→05 | §7 |
| `{Home,INDEX,LOG,Ecosystem,Observations}` | DELETE/MERGE | root wins |

## 7. Extensions (8) + atelier kb — collision: Workflow + Home/INDEX/LOG

Each of arbiter/crucible/embassy/ferrox/manifold/mnemos/prism/soter/atelier has
`Home/INDEX/LOG` + `Architecture/Surface.md` + `Building/{Getting Started,Release Commands}`
+ `Planning/MILESTONES` + `Positioning`.

| Source group | Action | Target |
|---|---|---|
| `<ext>/kb/Architecture/*.md` | MOVE | `kb/02-architecture/subsystems/<ext>/` (Surface.md = per-subsystem deep-dive) |
| `<ext>/kb/Positioning.md` | MERGE→01 | one platform story; per-ext positioning becomes a section |
| `<ext>/kb/Building/{Getting Started, Release Commands}.md` | MERGE→05 | ONE build/run/publish story keyed by subsystem; kill 8× near-dupe "Release Commands" |
| `<ext>/kb/Planning/MILESTONES.md` | DELETE | milestones live in Linear (CLAUDE.md says EPIC/MILESTONES deleted) |
| `<ext>/kb/History/CHANGELOG.md` | MOVE | `kb/08-history/changelogs/<ext>.md` |
| `<ext>/kb/{Home,INDEX,LOG}.md` | DELETE | root INDEX/Home/LOG win |
| `atelier/kb/Philosophy/*`, `Standards/*` | MERGE→01/04 | |
| `atelier/kb/Planning/Converge 3.9 API Drift.md` | REWRITE→08 | v3.9 stale |

Workflow collision: converge(4) + organism(5) + axiom(1) "Daily Journey / Git
Strategy / Working with {Claude,Codex,Gemini}" → ONE set in
`kb/05-operations/workflow/`. Survivor = converge's (richest). Note "Git Strategy"
must state the 4.0.0 main-only trunk, not converge's legacy main+next.

## 8. Stray content-bearing dirs outside any kb

| Source | Action | Target |
|---|---|---|
| `atelier/docs/strategy/*.md` (6) | MOVE (SURVIVOR) | `kb/01-platform-story/strategy/` — survivor over identical `kb/mosaic/docs/strategy/*` |
| `atelier/truths/{High-Risk Claim Portfolio, The Reasoning Substrate}.md` | MOVE | `kb/06-consumption/scenarios/` |
| `atelier/quality/{dashboard,navigator,dimensions/*,properties/*,incidents/*,migrations/*}.md` | EVALUATE | these back the arena dimension authoring — likely STAY as live tool state, or move stable narrative into `kb/04-quality/`. Owner call. |
| `atelier/experiments/*`, `extensions/ferrox/experiments/*`, `foundation/*/experiments/*` | MERGE | §9 |
| `extensions/{ferrox,soter}/docs/superpowers/plans/*.md` | DELETE | dated SDD plans, superseded |
| `foundation/converge/schema/{openapi,proto}/README.md` | STAY | crate-adjacent doorsteps |
| `arena/reports/latest.md` | STAY | generated report |

## 9. Experiments reconciliation

13 EXP files across 5 trees, overlapping IDs (EXP-001 exists in atelier, ferrox,
converge×2, organism, root kb). Canonical experiment log = `kb/Experiments/` (root)
+ `kb/06-.../` per-subsystem. Action: consolidate into `kb/08-history/experiments/`
with a namespaced ID scheme (`CVG-EXP-001`, `ORG-EXP-001`, …). De-dupe converge's
`experiments/EXP-001` (repo root) vs `converge/kb/Experiments/EXP-001` (likely same).

## 10. Entry-points that STAY (doorsteps, not kb)

- Root: `README.md`, `AGENTS.md`, `CLAUDE.md`, `CODEX.md` (README is CURRENT/authoritative)
- Per-subsystem doorsteps: `<sub>/README.md`, `<sub>/AGENTS.md`, `<sub>/CLAUDE.md`,
  `<sub>/CODEX.md`, `<sub>/GEMINI.md` — trim to README + AGENTS where duplicative
- Boilerplate that stays (repo hygiene): `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`,
  `SECURITY.md`, `.github/*` templates (10 each) — NOT kb; leave in place
- `CHANGELOG.md` per subsystem (19): the LIVE one stays at crate root; historical
  snapshot copied into `kb/08-history/changelogs/`

## 11. Agent machinery (D3, separate concern — do NOT move into kb)

- `.claude/skills/*/SKILL.md` (52) and `.codex/skills/*/SKILL.md` — agent
  machinery; STAY. Flag only: `converge/.claude/skills/branch/SKILL.md` +
  `.codex` mirror encode the legacy "main + next" trunk, contradicting README's
  4.0.0 main-only. Reconcile the skill, not via kb.

## 12. Vendor (excluded entirely)

`extensions/ferrox/vendor/{highs,ortools}/**` — hundreds of upstream md files.
NEVER touched, never counted, never moved.
