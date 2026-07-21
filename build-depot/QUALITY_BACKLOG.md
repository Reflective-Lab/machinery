# Quality Backlog

Living, append-only ledger for security, reliability, maintainability, delivery-system, and
platform-leverage findings across the Reflective workspace. Read this file at the start of every
audit, PR review, weekly software-factory health review, and session. Write to it whenever you
open, promote, demote, close, accept, or supersede a finding. See `AGENTS.md` for the operating
rules this ledger serves.

Update this file during audits, PR reviews, release preparation, and recurring
software-factory health reviews. Do not delete closed items; mark them `Done`,
`Superseded`, or `Won't do` with a short note so quality history remains visible.

Build-Depot (`build-depot/`) is the graph-backed software-factory control plane
for this ledger. It should ingest or derive Repository, Finding, Incident,
Recurring Property, and Standard facts from GitHub, Linear, Sentry, scheduled
fleet scans, and this file. Architecture and operating model:
`build-depot/docs/architecture/software-factory-build-depot.md`,
`build-depot/docs/operations/software-factory-quality-system.md`, and
`build-depot/docs/operations/quality-gates.md`.

## Snapshot

Refresh this block at the end of every review cycle. Numbers below describe state at the end of
the cycle named in "Last review."

- Last review: **2026-07-02** (Cycle 3 — recurring health review after the Linear migration and cross-agent alignment. Self-audit of all 13 open findings (2 closed, 1 demoted, 3 history refreshes); risk-register sweep (RR-2026-06-07-01 not yet due, revisit 2026-07-07); drift scan (agents-doctor green; milestone-done check missing → `QF-2026-07-02-04`; hermetic-audit red → `QF-2026-07-02-05`); new-findings pass (branch-hygiene debt → `QF-2026-07-02-06`, audit-ignore expiry → `QF-2026-07-02-07`); scorecard Cycle 3 appended. Prior context in the 2026-06-17 entry retained in git history.)
- Open findings: **20** (A: 1, B: 10, C: 7, D: 2)
- Closed since last review: **51** (`QF-2026-06-02-00`, `QF-2026-06-02-01`, `QF-2026-06-02-02`, `QF-2026-06-02-03`, `QF-2026-06-02-04`, `QF-2026-06-02-05`, `QF-2026-06-02-06`, `QF-2026-06-02-07`, `QF-2026-06-02-08`, `QF-2026-06-02-09`, `QF-2026-06-02-10`, `QF-2026-06-02-11`, `QF-2026-06-02-12`, `QF-2026-06-02-13`, `QF-2026-06-02-14`, `QF-2026-06-02-15`, `QF-2026-06-02-16`, `QF-2026-06-02-17`, `QF-2026-06-02-18`, `QF-2026-06-02-19`, `QF-2026-06-02-20`, `QF-2026-06-02-21`, `QF-2026-06-02-22`, `QF-2026-06-02-23`, `QF-2026-06-02-24`, `QF-2026-06-02-25`, `QF-2026-06-02-27`, `QF-2026-06-02-28`, `QF-2026-06-02-29`, `QF-2026-06-02-30`, `QF-2026-06-03-01`, `QF-2026-06-06-02`, `QF-2026-06-07-01`, `QF-2026-06-07-03`, `QF-2026-06-07-04`, `QF-2026-06-08-01`, `QF-2026-06-08-02`, `QF-2026-06-08-03`, `QF-2026-06-08-04`, `QF-2026-06-08-09`, `QF-2026-06-08-10`, `QF-2026-06-13-01`, `QF-2026-06-24-01`, `QF-2026-06-24-02`, `QF-2026-06-24-03`, `QF-2026-06-24-04`, `QF-2026-06-24-05`, `QF-2026-06-24-06`, `QF-2026-06-27-01`, `QF-2026-07-01-01`, `QF-2026-07-02-01`)
- Bucket A SLA breaches (open > 7 days): **0**
- Standing properties tracked (RP-*): **18**
- Standing properties currently green: **16**
- Accepted risks open: **1**
- ADRs added since last review: **7**
- Standards promoted since last review: **12**
  `QF-2026-06-02-01`, `-08`, `-15`, `-16`, `-17`, `-20`, `-21`, `-22`, `-27`. Plus the 16-repo CI/CD inventory `KB/06-operations/ci-cd-inventory.md` shipped 2026-06-08 under `QF-2026-06-02-03` + `-02-11` closure — operations doc, not standards.)
- Drift findings opened / closed since last review: **2 / 2** (the two violations
  surfaced by the first run of `just project-doctor` — `QF-2026-06-02-29` and
  `QF-2026-06-02-30` — were drift findings; both closed same-cycle. The recipes
  themselves (`QF-2026-06-02-28`) are not drift findings.)
- Autonomous shipments since last review (per Autonomy Contract): **0** (all Cycle 1
  commits human-authored or human-authorized)

## How to use this ledger

- Append-only. Never delete entries. Closed findings are quality history that future agents read
  before re-raising the same point.
- ID format: `QF-YYYY-MM-DD-NN` where `NN` is sequential within the day.
- Cite ledger IDs in PR descriptions, commit messages, and ADRs so traceability survives outside
  this file.
- When a finding changes state (promotion, acceptance, supersession), append to its **History**
  field with the date and reason. Do not overwrite prior values.
- Sort each Open Findings bucket newest first. Closed entries live at the bottom in their own
  section, also newest first.

## Fields

- **ID** — stable dated identifier.
- **Date** — when the finding was recorded.
- **Bucket** — `A. Must fix now`, `B. Should fix soon`, `C. Strategic improvement`,
  or `D. Needs human decision`.
- **Area** — security, CI/CD, tests, developer experience, performance,
  scalability, maintainability, observability, platform extensibility,
  business leverage, release engineering, semver discipline, test
  hermeticity, supply-chain hygiene, or AI-factory discipline.
- **Discovered during** — `incident`, `release rehearsal`, `audit`, `PR review`,
  `agent scan`, `customer report`, `paired session`, `self-audit`, or
  `risk revisit`. The provenance of a finding shapes its calibration — incident-born
  items default to bucket A.
- **Evidence** — concrete file, command, output, CI result, metric, or missing
  artifact. Cite paths and line numbers. "I think" is not evidence.
- **Impact** — why it matters.
- **Risk if ignored** — failure mode or compounding cost.
- **Effort** — `S`, `M`, or `L`.
- **Owner** — person or agent responsible for the next action.
- **Status** — `Open`, `In progress`, `Done`, `Superseded`, `Accepted Risk`, or `Won't do`.
- **Next action** — the smallest concrete next step.
- **Verifies via** — the command, test, metric, or artifact that proves this
  finding is closed. Without it, `Done` is just an assertion.
- **Supersedes / Superseded by** — cross-links to prior or replacement IDs when a
  finding is reframed, split, or merged. Quality history is a graph, not a list.
- **Codex-safe now** — whether Codex can implement the next action under the
  Autonomy Contract in `AGENTS.md` without a human product or risk decision.
- **Properties** — optional list of `RP-*` IDs from the Recurring System
  Properties table that this finding works on (open) or proves (closed).
- **Confidence** — `H`, `M`, or `L` on the predicted impact. Future self-audits
  compare prediction to outcome to calibrate the ledger.
- **Business leverage** — optional one-line quantified impact in time, error
  rate, or revenue terms.
- **Last reviewed** — date and cycle of the most recent self-audit pass.
- **Cycles open** — integer, incremented each review the finding survives.
- **History** — bullet list of state transitions: `YYYY-MM-DD: <change>
  (reason; Cycle N)`. Append-only.
- **Linked PRs / commits** — optional list of PR URLs or commit SHAs.
- **Standard promoted** — optional path to `KB/05-engineering/standards/...` if
  closure produced a standard.
- **Drift check** — optional path to test, CI gate, or scripted scan that
  detects regression of this finding.
- **ADR** — optional path to `KB/04-architecture/decisions/...` for D-tier
  findings with a recorded decision.
- **Risk register entry** — optional path or anchor in
  `KB/06-operations/risk-register.md` if the finding was accepted.

Older entries below may omit fields introduced after they were opened. Fill missing fields the
next time the finding is touched.

## Lifecycle

- `Open` → `In progress` → `Done`. On Done, promote any reusable lesson to
  `KB/05-engineering/standards/` and add a drift check. A closed finding without either is a
  half-completed cycle.
- `Open` → `Promoted` (`C → B → A`). When new evidence shows a strategic item is now
  load-bearing or actively damaging. Record the bucket change in History. Demotion is allowed
  too, with the same rigor.
- `Open` → `Accepted Risk`. A D-tier item the human chose to defer. Create a Risk Register
  entry with a revisit date and link both ways. Never silently drop.
- `Open` → `Superseded`. Replaced by a broader or newer finding. Link forward.
- `Open` → `Won't do`. Short reason. If the underlying risk persists, also open a Risk
  Register entry.

Automatic promotion rules:

- Any drift finding open across two consecutive review cycles is promoted from B to A.
- Any Bucket A item open more than 7 days is an SLA breach and is counted in the Snapshot.

## Cross-references

Paths annotated *(Created on first use)* do not exist yet. The cycle that needs them
creates them; absence is expected until that cycle. The `quality-doctor` recipe
(`QF-2026-06-02-15`) tolerates this annotation; a missing path without it is a drift failure.

- Standards: `KB/05-engineering/standards/` *(Created on first use)*
- Decisions (ADRs): `KB/04-architecture/decisions/` *(Created on first use)*
- Risk Register: `KB/06-operations/risk-register.md` *(Created on first use)*
- Factory Health Scorecard: `KB/06-operations/factory-scorecard.md` *(Created on first use)*
- Build-Depot factory architecture: `build-depot/docs/architecture/software-factory-build-depot.md`
- Software factory quality system: `build-depot/docs/operations/software-factory-quality-system.md`
- Quality gates: `build-depot/docs/operations/quality-gates.md`

## Entry template

Copy this block and fill it in when opening a new finding.

```markdown
### QF-YYYY-MM-DD-NN

- Date: YYYY-MM-DD
- Bucket: <A | B | C | D>
- Area: <see Fields catalog>
- Discovered during: <incident | release rehearsal | audit | PR review | agent scan | customer report | paired session | self-audit | risk revisit>
- Evidence: <concrete artifact, path, command output, metric>
- Impact: <why it matters>
- Risk if ignored: <failure mode or compounding cost>
- Effort: <S | M | L>
- Owner: <person or agent>
- Status: In progress
- Next action: <smallest concrete next step>
- Verifies via: <command, test, metric, or artifact that proves closure>
- Supersedes / Superseded by: <optional QF-* IDs>
- Codex-safe now: <Yes | No — reason>
- Properties: <optional RP-* IDs>
- Confidence: <H | M | L>
- Business leverage: <optional quantified impact>
- Last reviewed: YYYY-MM-DD (Cycle N)
- Cycles open: 0
- History:
  - YYYY-MM-DD: Opened (<source>; Cycle N)
- Linked PRs / commits: <optional>
- Standard promoted: <optional path>
- Drift check: <optional path>
- ADR: <optional path>
- Risk register entry: <optional path or anchor>
```

## Recurring System Properties

Standing invariants the factory aspires to. Each one is a long-lived assertion
about how this codebase should behave, paired with the enforcement mechanism
that makes it observable. Findings reference these properties when the
property is violated. New properties are added carefully — once they're here,
the factory commits to keeping them green.

`Tracked by` lists open findings whose closure would move the property closer
to green; closed findings are kept for provenance but not for the live count.

The table below is GENERATED from `KB/05-engineering/standards/recurring-properties.json`
by `just rp-table-sync` — never edit the rows directly. `just quality-doctor`
check 11 fails CI on drift. To add or modify a property, edit the JSON source
and run `just rp-table-sync` (QF-2026-06-02-18, closed 2026-06-08).

<!-- BEGIN GENERATED RP-TABLE -->
| ID | Property | Enforcement | Status | Tracked by |
|---|---|---|---|---|
| RP-DETERMINISM | Test outputs do not depend on dev-machine env, network, wall clock, or absolute filesystem location. | Scoping doc + enforcement decisions at `KB/05-engineering/standards/determinism.md`. Per-axis: clippy `disallowed_methods = "deny"` on `std::time::SystemTime::now`, `std::time::Instant::now`, `chrono::Utc::now`, `chrono::Local::now`, `rand::random`, `rand::thread_rng`, `std::env::vars` (wall clock + RNG + broad env axes); convention-only for `HashMap`/`HashSet` iteration in assertions and test independence (`HashMap` axis AST detection has high false-positive rate). Pilot landed in `mosaic-extensions/prism-analytics` 2026-06-08 with 5 callsites annotated. Cross-train rollout to remaining 6 workspaces tracked by `QF-2026-06-08-06`. | Aspired (pilot landed; cross-train rollout pending) | QF-2026-06-08-06 |
| RP-HERMETIC-UNIT | Unit tests issue zero outbound network requests. | Clippy `disallowed_methods = "deny"` at the workspace root of every train workspace catches every `reqwest::Client::new` / `reqwest::blocking::Client::new` / `::builder` callsite (construction layer; 6 train workspaces shipped 2026-06-07: organism, helms, commerce-rails, prism-analytics, converge, runtime-runway). Production callsites either use a `with_http_client(...)` DI constructor (struct-field shape), thread `client` through the call chain, or annotate `#[allow]` with a justification comment. Runtime defence-in-depth via `.github/workflows/hermetic-audit.yml` (shipped 2026-06-08): weekly + on-demand `unshare -rn cargo test --offline` per train workspace catches dynamic bypass (raw TCP via `std::net` / `tokio::net`). Standard documented at `KB/05-engineering/standards/hermetic-unit-tests.md`. | Enforced (construction-layer clippy lint + runtime no-network namespace audit) | — |
| RP-SEMVER-GATED | Public-API changes drive the version bump segment. Breaking change → major; additive → minor; otherwise patch. | `just release-public-api-check name` (root `Justfile`) runs `cargo public-api diff v<current>..HEAD` per publishable crate and classifies the diff as breaking / additive / unchanged. Operator runs it before tagging a release; integration into the `release` recipe's bump-target gating (block patch/minor bumps on breaking diffs) lands when `REL_APPLY=1` is wired. | Enforced (detection recipe ready; integration into `release` recipe's bump gate pending `REL_APPLY=1` wiring) | — |
| RP-LAYERING | A publishable crate depends only on publishable crates. UNLICENSED / private crates never appear in a publishable target's `[dependencies]`. | `just project-doctor` check 2 walks `cargo metadata --no-deps` per train workspace and rejects path-deps from a publishable crate to a `publish = false` crate. Wired into `.github/workflows/doctor.yml` `project-doctor` job. Pre-publish gating in the release path is still pending. | Enforced (via `.github/workflows/doctor.yml` project-doctor job) | — |
| RP-RELEASE-TRAIN-INTEGRITY | A planned release either ships its declared set atomically or fails with a partial-release manifest. No silent gaps. | `release-train.yaml` at workspace root is the single source of truth for publish order (QF-2026-06-06-02 closed 2026-06-08: the previous parallel `release_order` literal + `_release-dir` case statement in `Justfile` are gone; the Justfile awk-parses the YAML at runtime). `just project-doctor` check 1 validates parseability + every member directory exists. `just release-train-sync` (preserved as a name in `.github/workflows/doctor.yml`) does the same. Per-release atomic manifest still pending. | Enforced (single-source YAML + structural validation via `.github/workflows/doctor.yml`; per-release atomic manifest still pending) | — |
| RP-YANK-DISCOVERABLE | Every yanked crates.io version has a successor pointer and a public reason. | `KB/release-history.md` records yank-and-replace pairs at the time of yank (runbook + schema in-file). `just release-undo <crate> <version> [reason]` records and (with `REL_APPLY=1`) yanks in one operation, so the entry always exists before the yank command runs. `just project-doctor` check 6 validates required fields are present; `just release-history-audit` cross-references each entry with the crates.io API to confirm the named version is actually `yanked: true`. Yank-and-record discipline codified in `AGENTS.md > Release yank discipline`. Reverse-discovery (every yanked crates.io version has an entry) requires enumerating org-owned crates and is intentionally out of scope. | Enforced (file + write recipe + structural check 6 + crates.io cross-reference) | — |
| RP-TEST-CODE-ATTRIBUTION | A commit that mutates both production source and its directly-corresponding test must declare in the message whether the change is a contract update (test correct, code wrong) or a fixture refresh (code correct, test wrong) — never a silent "test now matches code." | Convention encoded in `AGENTS.md > Test/code attribution` (2026-06-07). Mechanical detector at PR time via `scripts/check-test-code-attribution.sh` + `.github/workflows/test-code-attribution.yml` (2026-06-08): heuristics A (`src/X.rs` paired with `tests/X.rs` or `tests/test_X.rs`) and B (`src/<path>/mod.rs` paired with `src/<path>/tests.rs`); requires one of three classification lines (`Contract update:`, `Fixture refresh:`, `Real bug fix:`) in PR body or commit messages; `[skip-attribution]` bypass token for legitimate non-attribution cases. Per-train-repo rollout pending under `QF-2026-06-08-05`. | Enforced (convention in `AGENTS.md` + CI detector in root repo; per-train-repo rollout pending) | QF-2026-06-08-05 |
| RP-SNAPSHOT-PORTABLE | `trybuild` / `insta` / golden-output fixtures contain no absolute paths, no machine names, no current line numbers from dependencies. | `just project-doctor` check 4 greps `.stderr` fixtures for `/Users/`, `/home/<user>/`, `/private/var/folders/`, and `/tmp/<random>` paths. Wired into `.github/workflows/doctor.yml` `project-doctor` job. `*.snap` and foreign-crate line-gutter scanning still pending. | Enforced (`.stderr` portability via `.github/workflows/doctor.yml`; `*.snap` and foreign-line-gutter scanning still pending) | — |
| RP-AUTO-BLESS-AUDITED | Every `TRYBUILD=overwrite` / `cargo insta accept` / similar auto-bless shows the diff in the commit message and classifies each line of change (cosmetic rustc evolution vs. real contract change). | Convention codified in `AGENTS.md` "Fixture auto-bless classification"; reviewer-enforced at PR time. Mechanical detector (pre-commit / CI shape check) is residual under `QF-2026-06-07-04` siblings. | Enforced (convention in `AGENTS.md`) | — |
| RP-FRESH-CLONE-GREEN | A fresh clone on a clean machine, with no `.envrc` and a clean cargo cache, builds and tests green within a documented time budget per workspace. | `release-train.yaml` now has a `fresh_workspaces` list with `fresh_check_budget_seconds` and `fresh_test_budget_seconds`. `just check-all-fresh` and `just test-all-fresh` read that list, run `cargo clean` before each workspace, execute `cargo check --workspace --all-targets` / `cargo test --workspace --all-targets`, and fail if a workspace is missing, lacks `Cargo.toml`, lacks a budget, exceeds its budget, or fails Cargo. `.github/workflows/fresh-clone.yml` checks out the root plus all train workspaces side-by-side and runs both recipes weekly and on demand. First live CI run remains to be observed under `QF-2026-06-07-02`. | Enforced (recipes + scheduled workflow configured; first live green run pending) | QF-2026-06-07-02 |
| RP-CRATE-SIZE-BUDGET | Each publishable crate stays under crates.io's 10 MiB limit and under a project-set soft budget. | `just project-doctor` check 3 (leading indicator) fails on any source file > 1 MiB in a publishable workspace; wired into `.github/workflows/doctor.yml`. `just release-package-size-check` (trailing indicator) runs `cargo package --list` per publishable crate inside `release-preflight` step 5.6, hard-fails ≥ 9 MiB, warns ≥ 5 MiB. Both indicator layers active. | Enforced (leading-indicator file-size check + trailing-indicator pre-publish `cargo package` size check) | — |
| RP-RUSTC-DRIFT-CONTAINED | rustc toolchain bumps reveal only cosmetic snapshot diffs; semantic test breakage gates the bump. | `just project-doctor` check 5 verifies each train workspace has `rust-toolchain.toml` pinning an exact `1.X.Y` or dated nightly. Wired into `.github/workflows/doctor.yml` `project-doctor` job. Classified-snapshot-diff discipline in bump PRs still relies on `RP-AUTO-BLESS-AUDITED`. | Enforced (pinning half via `.github/workflows/doctor.yml`; classified-diff discipline still convention-based) | — |
| RP-AI-EVIDENCE-CITED | Every finding, plan, or assertion authored by an AI agent cites concrete artifacts (paths, line numbers, command output). Speculation is marked as such. | Convention codified in `AGENTS.md` "AI evidence citation"; structurally checked by `just quality-doctor` check 3 (every `QF-*` ID cited in `AGENTS.md` resolves to a ledger entry); reviewer-enforced everywhere else. | Enforced (convention in `AGENTS.md`) | — |
| RP-AI-SHORTCUT-DECLARED | An AI agent that changes production code in order to make a failing test pass must declare this in the commit message and provide the design rationale, not a mechanical "test now passes." | Convention encoded in `AGENTS.md > Test/code attribution` (2026-06-07); reviewable in commit history. The 2026-06-02 atelier retrieval incident is cited inline in the policy as the motivating failure. | Enforced (convention in `AGENTS.md`) | — |
| RP-TYPED-CROSS-LAYER-SEMANTICS | A string that carries semantics (closed set, bounded number, typed actor / source / route owner / entitlement / event) must be a Rust type before it crosses a layer boundary. | Three reinforcing convention layers: (1) standard `KB/05-engineering/standards/typed-cross-layer-semantics.md` defines the rule, boundary surfaces, and what's in / out of scope; (2) Boundary Checklist question 6 in `KB/04-architecture/runtime-injection-boundaries.md`; (3) PR template `.github/PULL_REQUEST_TEMPLATE.md` includes the typed-boundary checklist item shown to every PR author. Mechanical lint considered and explicitly rejected under `QF-2026-06-08-02` (low-frequency event; boundary identification needs reviewer judgment regardless). If a real boundary-typing failure mode is observed in the wild, the mechanical-lint conversation reopens with a concrete case behind it. | Enforced (convention + standard + PR template; mechanical lint considered + rejected on cost-vs-leverage) | — |
| RP-CI-PARITY | Every train repo's CI workflow is a thin runner around a single `just ci` recipe. Running `just ci` locally gives the same verdict as GitHub Actions on the same commit. | Standard `KB/05-engineering/standards/ci-parity.md` (2026-06-08) defines the canonical Justfile shape (`just ci` orchestrates `fmt-check → check → lint → test → security-audit`) and the canonical workflow shape (single `ci` job, one `run: just ci` step, standardised action versions). Pilot landed in `commerce-rails` 2026-06-08 (PR Reflective-Lab/commerce-rails#2). Cross-train migration tracked by `QF-2026-06-08-11`; per-repo CI-bootstrap gaps tracked by `QF-2026-06-08-07/-08/-09`. Mechanical drift check (project-doctor parser that asserts each train repo's `ci.yml` is a thin runner) is future work, deferred until the canonical shape stabilises through ≥3 workspace migrations. | Aspired (standard documented + first pilot landed; cross-train migration pending) | QF-2026-06-08-11 |
| RP-POLICY-FRESH | The quality factory (`AGENTS.md`, `QUALITY_BACKLOG.md`, and their cross-references) stays internally consistent: policy files tracked; Snapshot fresh; cited `QF-*` IDs exist; `RP-*` `Tracked-by` references current open findings or `—`; cross-reference paths exist or are annotated `Created on first use`; root agent-pointer files present and tracked. | `just quality-doctor` recipe in root `Justfile`; wired into `.github/workflows/doctor.yml` unconditional `quality-doctor` job. | Enforced (via `.github/workflows/doctor.yml`) | — |
| RP-SHIM-FIRST-CLASS | No workaround lands silently: every shim (linker/compiler leniency, #[ignore]d or commented-out test, CI-only gate divergence) carries an inline SHIM(QF-..., expires: ...) marker backed by a live ledger finding; expiries are promises; releases wait for unexpired shims on shipped cargo. | Policy in `AGENTS.md > Shims, disabled tests, and conditional escapes are first-class debt` (2026-07-03); standard `KB/05-engineering/standards/first-class-shims.md`; mechanical check `just shim-doctor` (scripts/factory/shim-doctor.sh) wired into `just doctor` and `.github/workflows/doctor.yml`. Scope v1: root-repo tracked files; fleet rollout residual. | Enforced (via .github/workflows/doctor.yml; root-repo scope) | QF-2026-07-03-01 |
| RP-DEP-CATALOG | Each concern (HTTP, time, errors, async runtime, hashing, ...) resolves to exactly one blessed library at one version line across every project. No sprawl, no cross-project version drift. | Catalog at `build-depot/docs/operations/approved-libraries.md` names the blessed library per concern (answers sprawl). Per-workspace `cargo-deny` `[bans]` enforces locally; `commerce-rails/deny.toml` present (`multiple-versions = warn`), `runtime-runway/deny.toml` landed 2026-07-10 mirroring that shape (tracked by QF-2026-07-10-01). Cross-project drift detector in Build-Depot — parse each repo's `[workspace.dependencies]` + `package.json` against the catalog and flag off-catalog libraries or version divergence (e.g. `sha2` 0.11 vs 0.10) — is future work. | Aspired (catalog authored + runtime-runway deny.toml landed; multiple-versions hardening to deny + cross-project detector pending) | QF-2026-07-10-01 |
<!-- END GENERATED RP-TABLE -->

## Open Findings

### Bucket A — Must fix now

#### QF-2026-07-02-08

- Date: 2026-07-02
- Bucket: A. Must fix now
- Area: reliability / CI gate integrity
- Discovered during: green-main program (runway#11 babysitting)
- Evidence: runtime-runway's `contract (local + emulator)` workflow has
  NEVER completed: 15/15 historical runs killed at the 15-minute cap,
  then one killed at 30m26s, then one killed at the 60-minute cap
  (2026-07-02 ~19:14 UTC). The local half passes (7 tests, 3.05s). The
  emulator half prints `running 5 tests` (18:16:56) and then produces
  NO output for 58 minutes until the runner kills the job — a test
  blocks indefinitely, almost certainly awaiting an emulator connection
  (firestore/pubsub containers and fake-gcs seeding all report healthy
  before it).
- Impact: the contract gate has never gated anything — every run in its
  history ends `cancelled`, which `just factory-status` renders as `•`
  (not red). A permanently hanging gate that displays as neutral is
  silent coverage loss on exactly the integration surface
  (emulator-backed storage/pubsub contracts) the workflow exists to
  protect. It also burns a full runner-hour per push.
- Proposed fix: (1) per-test timeouts so a hang becomes a named FAILURE;
  (2) one run with `--test-threads=1 --nocapture` to identify the
  blocking test; (3) fix the connection/config bug; (4) tighten
  `timeout-minutes` to observed duration + ~30%. Also: factory board
  should count timeout-cancelled workflows as red — a gate that cannot
  conclude is not neutral (fleet-status.sh change).
- Effort: M
- Owner: Claude (diagnosis); Karl if the fix touches emulator infra choices
- Status: Done — 2026-07-02. Contract Suite green (first success in the
  workflow's history), runtime-runway#12 merged. The "hang" decomposed
  into one enabling defect plus five real production bugs in the
  Firestore REST backends, all caught by the suite the moment it could
  conclude:
  1. No request timeouts anywhere (`reqwest::Client::new()` ×6) — a
     non-answering endpoint blocked forever. One sanctioned client
     (`http::client`, connect 10s / request 30s); clippy
     disallowed-methods escape hatch localized there.
  2. Lease document IDs embedded raw in REST paths (`|` illegal in a
     URI; `/` silently nested subcollections) — now percent-encoded
     into deterministic slash-free IDs.
  3. `DocumentStore::query` translated only `Filter::Eq`; range and
     And/Or filters silently dropped → unfiltered supersets. Full
     recursive translation added.
  4. `Query::updated_after` filtered on a field `put()` never wrote
     (updateTime metadata is not queryable) → reserved `_updated_at`
     field written on put, stripped on read.
  5. Events unqueryable twice over: append never stored org_id/app_id
     fields that query filters on, and `parse_event_document` derived
     ids from name POSITIONS (garbage when ids contain `/`) — now
     fields-first with name fallback, event_id from final segment.
  6. Lease CAS: the emulator parses `currentDocument.updateTime` QUERY
     PARAMS as version 0 ("required base version (0)"), rejecting every
     legitimate CAS. Conditional writes moved to `:commit` bodies and
     reads to `:batchGet` — the SDK wire shape both real Firestore and
     the emulator implement faithfully.
  Method note: the timeout fix doubled as instrumentation — each CI run
  named the next defect (systematic-debugging evidence loop, 4
  iterations hang → named failures → green).
- Residual (split out rather than blocking closure):
  - `timeout-minutes: 60` should be tightened to observed wall time
    +~30% once a few green runs establish the baseline (first complete
    run available now).
  - fleet-status.sh still renders timeout-cancelled workflows as `•`
    (neutral) — a gate that cannot conclude should count red.
  - The local half of the suite passed throughout — only remote impls
    drifted. Consider a drift check asserting every `Filter` variant is
    exercised against every backend (the Eq-only gap survived because
    nothing ran the remote suite).
- Confidence: H
- Closed: 2026-07-02
- Linked PRs / commits: runtime-runway#12 (5 commits)

### Bucket B — Should fix soon

#### QF-2026-07-03-01

- Date: 2026-07-03
- Bucket: B. Should fix soon
- Area: build correctness / solver toolchain / shim debt
- Discovered during: fresh-clone gate repair
- Evidence: `libortools.so` built from `ORTOOLS_TAG v9.15` carries an
  undefined `setLocalOptionValue(HighsLogOptions, ...)` that the
  Makefile-pinned vendor HiGHS (`HIGHS_TAG v1.14.0`) does not export with a
  matching signature — OR-Tools was compiled against a different HiGHS than
  the one on the link line (2026-07-02 fresh-clone run, ferrox step, ld.bfd
  "undefined reference"). Interim shim: `-Wl,--allow-shlib-undefined` in the
  workflow-written `ferrox-solvers/.cargo/config.toml`, marked
  `SHIM(QF-2026-07-03-01, expires: 2026-07-17)` per RP-SHIM-FIRST-CLASS.
- Impact: link-time verification of the ortools↔highs symbol contract is
  disabled in the fresh-clone gate; a genuinely missing symbol would
  surface only as a runtime failure in solver tests.
- Root fix: in `mosaic-extensions/ferrox-solvers/Makefile`, either pin
  `HIGHS_TAG` to the HiGHS version OR-Tools v9.15 builds against, or build
  OR-Tools with `-Dhighs_DIR` pointing at the vendor HiGHS so both sides
  share one HiGHS. Then remove the shim and the marker.
- Risk if ignored: shim expires 2026-07-17 and shim-doctor turns the
  factory gate red; or a real symbol miss ships as a runtime crash.
- Effort: M (solver cache rebuild per CI iteration, ~30-45 min each)
- Owner: Codex
- Codex-safe now: Yes (single repo, no API surface)
- Status: Open
- History:
  - 2026-07-03: Opened; shim landed same day, marked and expiring.


#### QF-2026-07-02-05

- Date: 2026-07-02
- Bucket: B. Should fix soon
- Area: meta-quality / drift-check health
- Discovered during: recurring health review (scorecard CI sweep)
- Evidence: `gh run list --workflow=hermetic-audit.yml --limit 1` →
  latest run **failure** (checked 2026-07-02). hermetic-audit.yml is the
  mechanical check backing `KB/05-engineering/standards/hermetic-unit-tests.md`
  (RP-HERMETIC-UNIT, currently marked Enforced). fresh-clone.yml is also red
  but already tracked by `QF-2026-06-07-02`.
- Impact: an Enforced standing property has a red enforcement gate — either
  a real hermeticity regression landed, or the check itself broke. Both
  demand diagnosis; until then "Enforced" overstates reality.
- Risk if ignored: RP table shows green language over a red gate — exactly
  the false-positive-success-gate class the factory exists to prevent.
- Effort: S (read the failing run log, classify gate-broken vs regression,
  fix or file the follow-on)
- Owner: Codex
- Codex-safe now: Yes
- Status: Open
- History:
  - 2026-07-03: Rerun after the structural fixes: **6/7 workspaces pass**
    (converge, axiom, organism, helms, prism-analytics, commerce-rails).
    Sole failure: runtime-runway's runway-app-host contract_test requests
    http://127.0.0.1/healthz from a server it spawns in-process — loopback,
    which `unshare -n` leaves DOWN. That is inside the hermetic seal (no
    external dependency); fixed by `ip link set lo up` inside the namespace.
    No workspace leaks EXTERNAL network — the audit's actual subject.
  - 2026-07-02 (late): Three structural blockers fixed ahead of the rerun:
    no GTK stack (helms could never compile), no /mnt target redirect
    (7 workspaces of test artifacts vs ~14 GB OS disk), and building inside
    the no-network namespace (ort-sys legitimately downloads onnxruntime at
    build time — now builds outside via --no-run, tests run sandboxed).
    Plus CARGO_PROFILE_DEV_DEBUG=0 as in fresh-clone.
  - 2026-07-02 (evening): hermetic-audit.yml restructured to one step per workspace with a Verdict step — future runs show which workspace leaks live. Failure diagnosis pending the in-flight dispatch run.
  - 2026-07-02: Opened (health-review scorecard sweep found the red gate).


#### QF-2026-07-02-06

- Date: 2026-07-02
- Bucket: B. Should fix soon
- Area: delivery / branch hygiene / stranded work
- Discovered during: recurring health review (new-findings pass)
- Evidence: Fleet-wide next/main divergence with cherry-pick duplicates and
  stranded work. atelier-showcase: `next` 9 ahead of origin/main (incl. the
  `just ci` collapse 17bc6ff and RUSTSEC ignores) while origin/main carries 2
  cherry-picked duplicates (15b139c/aaf1f52 duplicate 0121df7/5409f1b).
  helms: policy commit 73897ff says "work directly on main" yet 7 commits sit
  on `next` (incl. the first-ever CI workflow 5d0947a) and local main is
  ahead 1 with a duplicate (903c774/fef5c75). runtime-runway: next 2
  ahead / 4 behind origin/main with a duplicated docs commit
  (41d75bb/4b17651 — created by this review's own cherry-pick).
  Unpushed main commits: mosaic-extensions (e21c58b clean-recipe fix,
  ac32b3e), mobile-apps (81c7703). Uncommitted: converge justfile
  security-audit ignores + MILESTONES banner; studio-apps README doctrinal
  notes. Stashes: mobile-apps ×2 (active refiner line), runtime-runway ×1,
  converge ×1.
- Impact: CI gates and audit-ignore fixes exist only on branches CI does not
  watch; duplicate commits guarantee messy merges; single-machine loss risk
  for the clean-recipe fix and refiner WIP.
- Risk if ignored: the next merge in each repo conflicts on the duplicated
  commits; helms/atelier CI stays effectively un-gated on main.
- Effort: M (per-repo reconcile: merge next→main or rebase, drop duplicates,
  push stranded commits, triage stashes)
- Owner: Karl + Claude (stash/uncommitted triage needs intent judgment)
- Status: Open
- History:
  - 2026-07-02: Opened (health-review new-findings pass).
  - 2026-07-03: Karl ruled all four stashes (converge kernel-surface,
    runtime-runway builder.rs, mobile-apps refiner ×2) **stale and wasted
    work** — drop on his hygiene pass (destructive; needs his hands or an
    explicit go). Root cause codified as standard
    `KB/05-engineering/standards/branch-hygiene.md` (RP-BRANCH-HYGIENE:
    WIP = wip: commits on the lin-XX issue branch, never stash; one route
    to main; end-of-session invariant) and enforced as project-doctor
    check 8 (any fleet stash = fail, dirty tracked = warn). Converge's
    uncommitted justfile/MILESTONES edits from the evidence were committed
    since the review. Remaining to close: drop the 4 stashes, disposition
    studio-apps README (+15) and quorum-sense Cargo.lock (train
    re-resolve), reconcile the duplicated-commit branch pairs.
  - 2026-07-03 (later): Stash triage COMPLETE. Check 8's first sweep found
    six stashes across five repos (two beyond the review's four:
    www.converge.zone Apr-18 header/footer redesign, wolfgang-chat Apr-02
    skills WIP). Karl delegated disposition ("merge small meaningful sets,
    guide on bigger chunks"). Verdicts, all evidence-checked: converge
    kernel re-export → landed as 9f3b637; runway builder ambient-auth →
    generalized into app-host `router()` AuthLayer (lib.rs); mobile-apps
    refiner ×2 → landed via the M6 line same evening (symbols present on
    main); converge.zone redesign → targets deleted pre-SvelteKit files,
    RustFS article itself is live in `src/lib/data/mechanics.ts`;
    wolfgang review-skill hunk → targets deleted skill. One salvage: the
    fix-skill "handoff to human" test-plan step, merged as wolfgang-chat
    b621db9 (pushed). All six archived to
    `~/dev/.stash-archive-2026-07-03/*.patch`, then dropped. Doctor fully
    green. Still open: quorum-sense Cargo.lock, studio-apps README,
    duplicated-commit branch pairs, www.converge.zone 281 dirty tracked
    files on main.

#### QF-2026-07-02-07

- Date: 2026-07-02
- Bucket: B. Should fix soon
- Area: security / gates / audit-ignore hygiene
- Discovered during: recurring health review (new-findings pass)
- Evidence: The security-audit gate was deliberately restructured this week
  (root d383d64: security-audit moves out of `just ci` — world-red vs
  code-red), and same-day advisory ignores were added fleet-wide: converge
  548b0d7 (RUSTSEC-2026-0194/-0195), organism 4d48346 (0187/0192/2025-0141/
  2024-0436 in security.yml + deny.toml), axiom 8d9f3dc, atelier-showcase
  bb10d78/e265f60. No ignore entry carries a review-by date or removal
  trigger; the scheduled Security workflow is now the sole advisory gate.
- Impact: justified individually, but the ignore lists only grow — an
  advisory that later gains a fix (or an exploit) stays silenced.
- Risk if ignored: silent accumulation of permanently-ignored advisories;
  the world-red/code-red split loses its "world catches up" half.
- Effort: S (annotate each ignore with `# review-by: YYYY-MM-DD` + a doctor
  check that fails on missing/expired annotations)
- Owner: Codex
- Codex-safe now: Yes
- Status: Open
- History:
  - 2026-07-02: Opened (health-review new-findings pass).

#### QF-2026-07-02-04

- Date: 2026-07-02
- Bucket: B. Should fix soon
- Area: meta-quality / standards drift
- Discovered during: recurring health review (drift scan)
- Evidence: `KB/05-engineering/standards/milestone-definition-of-done.md`
  names `just milestone-done <id>` as its mechanical check, but the recipe
  exists nowhere — not in the root Justfile, scripts/, or any workflow; the
  sole occurrence of the string is the standard doc itself. A standard whose
  drift check does not exist is itself drift (per Drift Detection policy:
  "standards whose drift checks are absent").
- Impact: the milestone definition-of-done bar is unenforceable; with
  tracking now in Linear the standard also needs re-pointing (milestones are
  Linear issues, not MILESTONES.md sections).
- Risk if ignored: the standard silently rots into aspiration.
- Effort: S (either implement a Linear-aware `milestone-done` recipe or
  revise the standard to declare the check convention-only, as
  typed-cross-layer-semantics.md does)
- Owner: Codex
- Codex-safe now: Yes
- Status: Open
- History:
  - 2026-07-02: Opened (health-review drift scan).


#### QF-2026-07-02-03

- Date: 2026-07-02
- Bucket: B. Should fix soon
- Area: CI parity / cross-repo coupling
- Discovered during: green-main program merge sequence
- Evidence: `cargo fmt --all -- --check` (the canonical `fmt-check` recipe)
  formats local *path dependencies*, not just the repo's own workspace
  members. Observed twice on 2026-07-02 as circular PR deadlocks:
  arena-tests#3 ↔ atelier-showcase#6 (arena's fmt-check tripped on
  atelier main's `helm-realtime-stem-headless`), then helms#4 ↔
  atelier-showcase#6 (same files; atelier#6 in turn tripped on helms
  main's `helm-session-host`). Both required admin-merges to break.
- Impact: Any unformatted commit on any repo's main turns fmt-check red in
  every sibling that path-depends on it. PR pairs deadlock; local `just ci`
  verdicts diverge from CI when local sibling checkouts differ from origin
  mains — exactly the parity failure RP-CI-PARITY exists to prevent.
- Risk if ignored: Recurring cross-repo deadlocks; operators learn to
  distrust fmt-check reds ("probably a sibling") — andon erosion.
- Proposed fix: scope `fmt-check` to the repo's own sources, e.g.
  `find crates apps -name '*.rs' -not -path '*/target/*' | xargs rustfmt
  --edition 2024 --check` (organism's historical pattern — which also
  covers trybuild fixtures that `cargo fmt --all` misses, see organism#17),
  and update `KB/05-engineering/standards/ci-parity.md` invariant 2.
- Effort: S per repo (16 repos)
- Owner: Claude
- Status: Open
- Confidence: H
- Last reviewed: 2026-07-02

#### QF-2026-07-02-02

- Date: 2026-07-02
- Bucket: B. Should fix soon (architecture decision needed)
- Area: layering / dependency direction
- Discovered during: green-main program (helms first-CI rollout), flagged by Karl
- Evidence: `bedrock-platform/helms/Cargo.toml` path-depends on two crates that
  live in `atelier-showcase/crates/` — `converge-atelier-domain` (dep line +
  `[patch.crates-io]`) and `organism-domain` (same). Because cargo metadata
  loads atelier's whole workspace, helms CI transitively needs checkouts of
  atelier-showcase, arena-tests, and embassy-ports (see
  `helms/scripts/ci/checkout-reflective-siblings.sh`).
- Impact: A platform application depends on the showcase repo — inverted
  layering ("Atelier shows it; Arena tests it"; both are downstream consumers).
  Heaviest fleet CI drags three extra repos; any atelier/arena workspace break
  cascades into helms CI (observed twice on 2026-07-02).
- Risk if ignored: The domain-pack crates keep living downstream of their
  consumers; every atelier restructuring is a potential helms break.
- Attempted fix (2026-07-02): switching to the published crates —
  `converge-atelier-domain 1.0.2`, `organism-domain 1.4.0` — fails: published
  `organism-domain 1.4.0` was built against an older `organism-pack` than the
  local organism v1.9.x line, so two `organism-pack` instances enter the graph
  (E0308 type mismatches in `workbench-backend`). Registry route blocked until
  fresh domain-crate versions are published.
- Options (human decision): (a) move `atelier-domain` + `organism-domain` out
  of atelier-showcase into a platform repo (converge/organism or a new domains
  repo) — the clean fix; (b) publish fresh versions from atelier and pin
  registry versions in helms — keeps the crates' home, cuts the path/CI edge.
- Effort: M (option b) / L (option a)
- Owner: Karl (decision), Claude (execution)
- Status: Open
- Confidence: H
- Last reviewed: 2026-07-02


#### QF-2026-06-06-01

- Date: 2026-06-06
- Bucket: B. Should fix soon
- Area: supply-chain hygiene / security
- Discovered during: audit
- Evidence: GitHub Dependabot reports `glib` advisory `GHSA-wrw7-89jp-8q8g`
  in `marquee-apps/fathom-narrative/apps/desktop/src-tauri/Cargo.lock` and
  `marquee-apps/scout-sourcing/apps/desktop/src-tauri/Cargo.lock`. After
  updating both desktop locks to `tauri 2.11.2`, `cargo tree -i
  glib@0.18.5 --target all` still resolves through `tauri 2.11.2 -> gtk
  0.18.2 / webkit2gtk 2.0.2 -> glib 0.18.5`. Upstream evidence matches the
  local tree: `wry 0.55.1` remains the latest crate release and its dependency
  dashboard still lists `gtk 0.18`, `webkit2gtk =2.0.2`, and `gdkx11 0.18`;
  Tauri issue https://github.com/tauri-apps/tauri/issues/12048 documents this
  advisory class as blocked by the unmaintained GTK3 bindings.
- Impact: The desktop lockfiles keep a medium-severity Rust advisory open even
  after all app-controlled and currently available Tauri patch updates are
  applied.
- Risk if ignored: Dependabot remains noisy for a real transitive advisory, and
  a Linux desktop build may keep carrying the vulnerable GTK binding stack until
  Tauri/Wry move to `glib >= 0.20`.
- Effort: M
- Owner: Codex
- Status: Accepted Risk
- ADR: `KB/04-architecture/decisions/2026-06-07-tauri-gtk3-glib-risk.md`
- Risk register: `KB/06-operations/risk-register.md#rr-2026-06-07-01`
- Next action: Re-check Tauri/Wry releases during the next dependency
  remediation pass. If Fathom or Scout add Linux desktop release artifacts
  before upstream moves to `glib >= 0.20`, promote this back to active backlog
  and block that Linux release path.
- Verifies via: `cargo tree -i glib@0.18.5 --target all` fails in both desktop
  crates, lockfiles contain `glib >= 0.20`, and GitHub Dependabot no longer
  reports `GHSA-wrw7-89jp-8q8g` for those desktop locks.
- Codex-safe now: No — the next closure depends on an upstream Tauri/Wry
  dependency move or a human risk decision.
- Confidence: H
- Business leverage: Preserves Dependabot signal quality by separating alerts
  the factory can patch now from upstream-pinned desktop advisories.
- Last reviewed: 2026-06-06 (dependency remediation)
- Cycles open: 0
- History:
  - 2026-06-06: Opened (Dependabot alert remediation; Cycle N/A)
  - 2026-06-06: Re-checked latest crates.io releases. `tauri 2.11.2` was
    available and applied to Fathom and Scout desktop locks, but `glib 0.18.5`
    still resolves through Tauri/Wry's Linux GTK3 stack. Linked upstream issue
    tauri-apps/tauri#12048.
  - 2026-06-07: Accepted as a temporary bounded risk via ADR
    `2026-06-07-tauri-gtk3-glib-risk.md` and risk register entry
    `RR-2026-06-07-01`. Alerts remain visible; Linux desktop release artifacts
    are not allowed while the risk is open.

#### QF-2026-06-08-07

- Date: 2026-06-08
- Bucket: B. Should fix soon
- Area: CI/CD reliability
- Discovered during: CI/CD inventory (`KB/06-operations/ci-cd-inventory.md`)
- Evidence: `bedrock-platform/helms` has no `.github/workflows/`
  directory and no `dependabot.yml` / `deny.toml`. The local Justfile
  has 24 recipes (desktop-focused: `build-desktop`, `dev-desktop-*`,
  `desktop-rust-fmt`, etc.) but **no** `check`, `lint`, `security-audit`,
  or `coverage` recipes. Every PR or push to helms ships without any
  CI safety net.
- Impact: A workbench-layer regression can land on main with no
  verification — operators discover it only when the desktop build
  breaks for everyone.
- Risk if ignored: Helms is the operator-facing surface; broken
  builds there break the demo and erode trust in the whole platform.
- Effort: M (helms's Justfile is desktop-focused; needs adapter
  recipes that wrap `cargo check` / `clippy` / `audit` over both the
  Rust workspace AND the desktop bits).
- Owner: Codex (with confirmation before push per helms's CLAUDE.md
  `Never push to main without confirmation` rule).
- Status: Open
- Next action: (a) Add `just check`, `just test`, `just lint`,
  `just fmt-check`, `just security-audit` recipes to helms's Justfile,
  matching the canonical shape proposed in
  `KB/06-operations/ci-cd-inventory.md`. (b) Add minimal
  `.github/workflows/ci.yml` that calls them. (c) Add
  `.github/dependabot.yml`. (d) Add `deny.toml` matching the train
  default.
- Verifies via: `cd bedrock-platform/helms && just check && just lint
  && just test && just security-audit` all exit zero; CI workflow
  runs on every PR; dependabot opens dependency PRs.
- Codex-safe now: Yes for the Justfile + workflow scaffolding;
  per-recipe body needs paired review on the desktop bits.
- Properties: RP-CI-PARITY (proposed; see `QF-2026-06-08-10`)
- Confidence: H (the gap is concrete; the fix is mechanical)
- Last reviewed: 2026-06-08 (audit)
- Cycles open: 0
- History:
  - 2026-07-02: Self-audit: bootstrap in flight — thin-runner ci.yml + dependabot.yml + `just ci` recipe exist on helms `next`, but CI runs are red (3 failures 2026-07-02) and nothing is on origin/main yet; deny.toml and security-audit recipe still missing. Keep open.
  - 2026-06-08: Opened (CI/CD inventory revealed helms has no CI).

#### QF-2026-06-08-08

- Date: 2026-06-08
- Bucket: B. Should fix soon
- Area: CI/CD reliability
- Discovered during: CI/CD inventory (`KB/06-operations/ci-cd-inventory.md`)
- Evidence: `arena-tests` has no `.github/workflows/` directory and
  no `dependabot.yml` / `deny.toml`. The local Justfile has 10 recipes
  (`report`, `build`, `test`, `lint`, `fmt-check`, `fmt`) but no CI
  invocation surface. Arena is the cross-stack integration-tests
  workspace — its job is to catch regressions across the train, and
  it does so only when an operator remembers to run it.
- Impact: Cross-stack regressions land silently because the workspace
  designed to catch them is not part of any PR gate.
- Risk if ignored: The 2026-06-02 PR Gate Cycle 1 surfaced multiple
  drift-related findings; arena-tests is exactly the place those
  would be caught if CI ran. Today, the workspace exists but its
  signal is opt-in.
- Effort: S (workflows similar to mosaic-extensions' ci.yml +
  security.yml; just need the canonical shape).
- Owner: Codex
- Status: Open
- Next action: (a) Add `.github/workflows/ci.yml` and `security.yml`
  matching the canonical mosaic pattern. (b) Extend the Justfile with
  `check`, `security-audit`, `coverage` recipes. (c) Add
  `dependabot.yml` and `deny.toml`.
- Verifies via: CI workflow runs on every PR; `just security-audit`
  matches the CI invocation; dependabot opens dependency PRs.
- Codex-safe now: Yes
- Properties: RP-CI-PARITY (proposed)
- Confidence: H
- Last reviewed: 2026-06-08 (audit)
- Cycles open: 0
- History:
  - 2026-07-02: Self-audit: same in-flight state as -07 — ci.yml + dependabot on disk, `just ci` recipe present, but all arena-tests CI runs red (2026-07-02) and the workflow is not on origin/main. No security.yml/deny.toml. Keep open.
  - 2026-06-08: Opened (CI/CD inventory revealed arena-tests has no CI).


### Bucket C — Strategic improvement

#### QF-2026-07-10-02

- Date: 2026-07-10
- Bucket: C. Strategic improvement
- Area: AI-factory discipline / maintainability / supply-chain hygiene
- Discovered during: paired session (software-factory consolidation)
- Evidence: Factory governance was split across two git repos. machinery/build-depot
  owned the doctrine, RP JSON, scorecard, and doctor script implementations, but the
  RP-table generator/checker (`scripts/rp-table.py`, `rp-table-check.py`), the 14
  standards `.md` files, and the factory CI workflows lived ONLY in the outer
  `reflective` repo — machinery could not regenerate or verify its own RP table
  without it. Separately, `chart-room/strategic/validator` (Rust binary,
  `strategy-validator`) had no Justfile, no `deny.toml`, no CI, and was absent from
  root orchestration and `factory-cohorts.json` — the one software project not
  governed by the factory (and the one that carried 19 Dependabot alerts 2026-07-10).
- Impact: Two-source-of-truth risk (diverged QUALITY_BACKLOG.md / recurring-properties.json);
  machinery not self-contained; an ungoverned Rust project accumulating fmt + clippy +
  supply-chain debt invisibly.
- Risk if ignored: RP table silently drifts; consolidation reverses; ungoverned
  projects ship unformatted, unlinted, un-audited code.
- Effort: M
- Owner: <unassigned>
- Status: In progress
- Next action: Consolidation landed 2026-07-10 (all additive, inside machinery):
  (1) ported `rp-table.py` + `rp-table-check.py` into `build-depot/scripts/`; added
  `rp-table` / `rp-table-sync` / `rp-table-check` recipes to `build-depot/Justfile`
  (verified: rp-table-check OK, rp-table-sync in-sync). (2) Copied the 14 standards
  `.md` into `build-depot/KB/05-engineering/standards/`. (3) Onboarded strategy-validator:
  canonical Justfile (RP-CI-PARITY), `deny.toml`, fleet-aligned clippy lint config,
  wired into root `Justfile` ci/check/test/security-audit and `factory-cohorts.json`;
  `just ci` + `just security-audit` now GREEN. Remaining: migrate `backoff` (unmaintained,
  RUSTSEC-2025-0012, tracked ignore in validator deny.toml); retire the now-duplicate
  copies in the outer `reflective` repo (human decision — separate repo/remote).
- Verifies via: `cd build-depot && just rp-table-check` → OK; `cd chart-room/strategic/validator && just ci && just security-audit` → green; `just factory-adoption-doctor` lists strategy-validator.
- Supersedes / Superseded by: —
- Codex-safe now: Yes — additive tooling/config + mechanical lint fixes.
- Properties: RP-CI-PARITY, RP-DEP-CATALOG, RP-POLICY-FRESH
- Confidence: H
- Business leverage: One self-contained factory control plane; every Rust + TS
  project plugs into the same gates; advisories caught before Dependabot.
- Last reviewed: 2026-07-10 (Cycle 3)
- Cycles open: 0
- History:
  - 2026-07-10: Opened (paired session; Cycle 3)
- Standard promoted: build-depot/docs/operations/factory-overview.md

#### QF-2026-07-10-01

- Date: 2026-07-10
- Bucket: C. Strategic improvement
- Area: supply-chain hygiene / maintainability
- Discovered during: paired session (dependency governance review)
- Evidence: No catalog of blessed libraries; each Rust workspace declares deps
  independently via `[workspace.dependencies]`, so nothing prevents two libraries
  for one job or divergent versions across projects. Concrete drift: `sha2` is
  `0.11` in `runtime-runway/Cargo.toml` vs `0.10` in `commerce-rails/Cargo.toml`.
  `runtime-runway` had no `deny.toml` (only `clippy.toml`); `commerce-rails/deny.toml`
  present. Dependabot surfaced 19 alerts on machinery (2026-07-10, all in
  `chart-room/strategic/validator/Cargo.lock`) — a reactive signal that arrives
  only after a version is public and flagged.
- Impact: Library sprawl and cross-project version drift raise audit, patch, and
  cognitive cost with no functional gain, and drift is invisible until someone
  reads two manifests side by side. A local advisories gate catches vulnerable
  versions before Dependabot rather than after.
- Risk if ignored: Duplicate/near-duplicate crates accrete; shared versions
  silently diverge; supply-chain surface grows unbounded; CVEs land as reactive
  Dependabot noise instead of a pre-merge gate.
- Effort: M
- Owner: <unassigned>
- Status: In progress
- Next action: (1) `runtime-runway/deny.toml` landed 2026-07-10 (advisories +
  bans, mirroring commerce-rails); wire `cargo deny check` into each repo's
  `just security-audit`; (2) converge `sha2` to one version; (3) spec the
  `RP-DEP-CATALOG` cross-project detector in Build-Depot.
- Verifies via: `cargo deny check` (advisories + bans) green in every Rust
  workspace and run in CI; Build-Depot detector reports zero off-catalog /
  drifted deps.
- Supersedes / Superseded by: —
- Codex-safe now: Yes — additive policy + config, no runtime behavior change.
- Properties: RP-DEP-CATALOG
- Confidence: H
- Business leverage: Bounds supply-chain audit surface; one blessed library per
  job cuts patch/review fan-out; advisories gate shifts CVE detection left of
  Dependabot.
- Last reviewed: 2026-07-10 (Cycle 3)
- Cycles open: 0
- History:
  - 2026-07-10: Opened (paired session; Cycle 3)
- Standard promoted: build-depot/docs/operations/approved-libraries.md

#### QF-2026-06-26-01

- Date: 2026-06-26
- Bucket: C. Strategic improvement (demoted from D on 2026-07-02)
- Area: platform extensibility / maintainability / architecture boundary
- Discovered during: paired session (Session Intelligence Spine design review)
- Evidence: The Session Intelligence Spine design
  (`KB/04-architecture/2026-06-26-session-intelligence-spine-design.md` §3, §5)
  proposes a new server-side `helm-session-host` crate (finding routing,
  `SessionPush` SSE, HITL-gate lifecycle). Helms already ships two crates that
  overlap that surface: `helm-coordination` (`SessionRegistry` heartbeat-leased
  membership, `PresenceRegistry`, `DecisionLedger`, `/v1/coordination/stream`;
  `helm-coordination/src/lib.rs:7-14`) and `helm-governed-jobs`
  (`/v1/jobs/{key}/stream` SSE with pause → waiter → resume HITL gates;
  `helm-governed-jobs/src/lib.rs:5-13`). The Spine's own Platform Alignment Guard
  forbids standing up parallel session/SSE/gate machinery, so whether the new
  crate extends, composes, or supersedes the existing ones — and whether a
  "decision session" is the same concept as "multi-operator coordination" — is an
  architecture-boundary choice with no safe local default.
- Impact: Wrong call either duplicates working session/SSE/gate infrastructure
  (violates the guard, accrues drift) or over-couples the Spine's
  lifetime/membership model to operator-coordination semantics it does not want.
- Risk if ignored: Plan 2 (`helm-session-host`) builds on an unresolved boundary
  and bakes in either duplication or a leaky coupling that is expensive to undo
  once three consumers depend on it.
- Effort: M (decision) + L (the migration it triggers)
- Owner: Karl
- Status: In progress (ADR **Accepted**; upstream Event/SSE consolidation **Done**
  runtime-runway@2aaa40c + helms@5d6ae03 2026-06-27; quorum-sense #5+#6 on
  `main@6208233`; Plan 2 slice 2 **pushed** 2026-06-29 — branches pending merge:
  runtime-runway@fdc563c, helms@c0df546, quorum-sense@2af8d2b; gate-waiter
  extraction still deferred)
- Next action: **Merge train** — runtime-runway `feat/hub-publish-returns-sequence`
  → helms `feat/helm-session-host-slice-2` → quorum-sense
  `feat/plan2-director-live-projection`. Then slice 3 (session membership) and
  finding ingestion. Gate-waiter / `GateDecision` extraction stays in
  `helm-governed-jobs` per ADR.
- Verifies via: characterization + existing integration tests green in both
  workspaces; helm public APIs unchanged except additive
  `GovernedJobsModule::with_shared_state`, `mount_live_modules`, and internal
  `CoordinationPublisher::new` signature (crate-private).
- Codex-safe now: No — the decision required human architecture judgment; the
  resulting migration is a Tier-1+ refactor of shipped crates.
- Properties: RP-LAYERING
- Confidence: H
- Last reviewed: 2026-06-26 (spine design review)
- Cycles open: 0
- History:
  - 2026-07-02: Demoted D → C (self-audit). The human decision is made and
    executed through slice 2 — the "merge train pending" commits are all on
    main (runtime-runway fdc563c, helms c0df546, quorum-sense 2af8d2b).
    Remaining work (slice 3 session membership + finding ingestion) is pure
    execution, not a decision.
  - 2026-06-26: Opened (spine design review). Decision **Accepted** same day —
    Option 4 (extract a shared `helm-session-core` stem now), justified by two
    in-tree consumers (`helm-coordination`, `helm-governed-jobs`) clearing the
    ≥2-consumer guard. Recorded in ADR
    `KB/04-architecture/decisions/2026-06-26-helm-session-host-vs-coordination.md`,
    linked both ways.
  - 2026-06-26: API audit completed (canonical:
    `KB/04-architecture/2026-06-26-helm-session-core-api-audit.md`) and
    Stem-extraction plan authored
    (`KB/08-roadmap/2026-06-26-spine-stem-plan-helm-session-core.md`), both in the
    parallel session; a duplicate stem-plan draft from this session was retired in
    favor of them. The audit forced two ADR corrections (now in the ADR Decision):
    the extractable gate primitive is **Converge-free** (Converge lives only in
    `run_job_task`, a consumer; `Engine::resume` lives in `helm-session-host`, not
    the core), and the optimistic `DecisionLedger` is operator-coordination-specific
    (racing operators) rather than obviously a decision-session need — deferred and
    re-evaluated when `helm-session-host` is built. Remaining work is execution.
  - 2026-06-26: **Redirected.** Under the "we own what we own — strengthen upstream,
    do not wrap" directive, the ADR's Option-4 (extract `helm-session-core` now) is
    superseded by Option 5. Decisive evidence: `runway-app-host/src/realtime.rs` shows
    the hub already owns a sequence counter and auto-stamps on the durable path
    (`:132,190,218-222`); the in-memory path just sets `next_sequence: None` (`:156`),
    which is the only reason both helm crates pre-stamp. `runway-app-host` already
    deps `axum` + ships `sse.rs`. So the "stem" was wrapping a half-built upstream we
    own. First pass now: finish upstream sequencing + add an SSE combinator upstream +
    de-dupe `GateDecision`/gate-wait within the existing helm edge. Zero new crates.
    Stem plan to be reworked accordingly.
  - 2026-06-27: **Helms migration landed** @5d6ae03 (`next_sequence` removal,
    `mount_live_modules`, live readiness feed requirement). Quorum-sense consumer
    wires `mount_live_modules` on `next` (shared `JobStreamState` hub; coordination
    module held for future manifest mount). Remaining: Plan 2 `helm-session-host`.
  - 2026-06-29: **Plan 2 slice 2 pushed** — live session store + Quorum director
    projection. Evidence: `helm-session-host` `store.rs`/`presenter.rs`/`service.rs`
    @ helms `c0df546` (`feat/helm-session-host-slice-2`); `EventHubHandle::publish`
    returns `u64` @ runtime-runway `fdc563c`; Quorum
    `resolve_director_snapshot()` + shared hub mount @ quorum-sense `2af8d2b`
    (`feat/plan2-director-live-projection`). Mobile `{ version, frame }` contract
    preserved; fixture v1844 fallback when no live state. Plan:
    `KB/08-roadmap/2026-06-26-spine-plan-2-helm-session-host.md`.
- ADR: `KB/04-architecture/decisions/2026-06-26-helm-session-host-vs-coordination.md`

#### QF-2026-06-12-01

- Date: 2026-06-12
- Bucket: C. Strategic improvement
- Area: platform architecture / dependency hygiene
- Discovered during: Plan 5 Cloud Run deploy of quorum-sense (build
  #4 cargo dep-resolution failure)
- Evidence: `bedrock-platform/helms/Cargo.toml` carries two
  `workspace.dependencies` entries pointing into atelier-showcase:
  - `converge-domain = { package = "converge-atelier-domain", path = "../../atelier-showcase/crates/atelier-domain" }`
  - `organism-domain = { path = "../../atelier-showcase/crates/organism-domain" }`
  The contagion path: `helms/crates/workbench-backend` is the only
  consumer; `helm-operator-control` depends on workbench-backend, so
  any marquee-app consuming `helm-operator-control` transitively
  drags atelier-showcase into its build/runtime graph.
- Impact: Violates Karl's stated rule "we can never use
  atelier-showcase or tests as libraries that real code rely on —
  they are development examples." Every marquee-app that mounts
  helms (Quorum, Atlas, Wolfgang, Tally, etc.) currently has a
  build-time dep on the showcase tree. Plan 5's Cloud Build had to
  clone atelier-showcase into the container as a tactical workaround.
- Risk if ignored: Showcase code drifts independently of platform
  release discipline; production apps end up with showcase-quality
  types in their stable surface. Refactors in atelier-showcase can
  silently break every consuming app.
- Effort: M. Two real options:
  1. **Promote**: move `organism-domain` and `atelier-domain` out of
     atelier-showcase into `bedrock-platform/organism/crates/` (or a
     new helms-internal crate). Update workbench-backend's path.
     Cross-repo: atelier-showcase (deletion), organism or helms
     (addition), helms (path update).
  2. **Inline**: read workbench-backend's actual usage of these
     types (probably a small surface) and inline into helms,
     dropping the path deps entirely.
- Owner: TBD (platform team / Karl)
- Status: Open
- Resolution: TBD (see Effort options 1 vs 2)
- Verifies via: `grep -rE 'path\s*=\s*"\.\./\.\./atelier-showcase'
  bedrock-platform/helms/Cargo.toml bedrock-platform/helms/crates/*/Cargo.toml`
  returns no matches.
- Codex-safe now: No — requires architectural decision (promote vs
  inline) before mechanical work.
- Properties: none yet; candidate for an RP-PLATFORM-NO-SHOWCASE-DEPS
  recurring property once the principle is canonicalized.
- Confidence: H (rule is explicit; violation is mechanical to spot).
- Business leverage: Every paying-customer marquee-app ships cleaner
  if showcase code is removed from the runtime graph. Quorum is the
  immediate beneficiary (Plan 5 deploy); Atlas and Wolfgang inherit
  on next rebuild.
- Last reviewed: 2026-06-12 (opened during Plan 5 deploy debugging)
- Cycles open: 0 days

#### QF-2026-06-12-02

- Date: 2026-06-12
- Bucket: C. Strategic improvement
- Area: build infrastructure / shared base image lag
- Discovered during: Plan 5 Cloud Run deploy of quorum-sense (build
  #5 rustc version mismatch)
- Evidence: Cloud Build failed with `package requires rustc 1.96.0` for
  20+ platform crates (organism-*, runway-*, converge-* @1.9.3 and
  @3.9.2 all pin rustc 1.96+). The shared
  `kenneth-backend-base:builder-latest` image on
  `europe-west1-docker.pkg.dev/wolfgang-kb-prod/wolfgang/` carries
  an older rustc. The Plan 5 Dockerfile was using this base image
  to share cargo build cache with wolfgang's deploys.
- Impact: Every marquee-app that adopts the kenneth-backend-base
  pattern inherits wolfgang's rustc cadence. When the platform
  floor advances, every consumer breaks until wolfgang rebuilds the
  base. Quorum's Plan 5 had to fork to `rust:1.96-bookworm`
  directly as a workaround, losing the shared cache.
- Risk if ignored: Cross-app friction every time the platform's
  rustc floor moves. Apps need to either coordinate base rebuilds
  with wolfgang or self-host their own builder.
- Effort: S (wolfgang-side rebuild + repush) for the immediate fix.
  M for a systemic answer (per-app builder vs. shared, with
  pin-bump CI on the shared one).
- Owner: TBD (wolfgang owns the base image)
- Status: Open
- Resolution: TBD. Tactical: wolfgang rebuilds `kenneth-backend-base`
  with rustc ≥ 1.96 and pushes new `builder-latest`/`runtime-latest`
  tags. Once that lands, Quorum's Dockerfile can switch back to the
  shared base (already documented in the Dockerfile header).
- Verifies via: After wolfgang's rebuild,
  `docker run --rm europe-west1-docker.pkg.dev/wolfgang-kb-prod/wolfgang/kenneth-backend-base:builder-latest rustc --version`
  reports ≥ 1.96.0. Reverting Quorum's Dockerfile to use
  `${BASE_REGISTRY}:builder-${BASE_TAG}` produces a successful Cloud
  Build.
- Codex-safe now: Yes for wolfgang-side rebuild (mechanical, follows
  wolfgang's existing base-image pipeline).
- Properties: candidate for an `RP-SHARED-BASE-RUSTC-CURRENT`
  recurring property — periodic check that the shared base's rustc
  matches or exceeds the highest pinned `rust-version` across all
  platform crates.
- Confidence: H.
- Business leverage: Low immediate (each app can self-host a builder
  image cheaply). Medium long-term (the shared-base pattern is the
  whole point of the kenneth-backend-base layer; if every app forks,
  we've lost the cache savings).
- Last reviewed: 2026-06-12 (opened during Plan 5 deploy debugging)
- Cycles open: 0 days

#### QF-2026-06-08-11

- Date: 2026-06-08
- Bucket: C. Strategic improvement
- Area: CI/CD reliability / delivery system
- Discovered during: closure of `QF-2026-06-08-10` (canonical shape
  + first pilot shipped)
- Discovered in: late-session paired work (2026-06-08)
- Evidence: `QF-2026-06-08-10` shipped the `RP-CI-PARITY` declaration,
  standard doc (`KB/05-engineering/standards/ci-parity.md`), and the
  first pilot (commerce-rails, PR Reflective-Lab/commerce-rails#2).
  The canonical shape is proven on real feature code. The remaining
  13 train workspaces need migration to the same shape: 2 with no CI
  at all (arena-tests, helms — also tracked by `-08-07/-08`), 8
  mosaic sub-extensions, 3 foundation crates, runtime-runway, and
  atelier-showcase.
- Impact: Today only commerce-rails has the canonical CI parity.
  The remaining 13 repos still suffer the drift documented in
  `KB/06-operations/ci-cd-inventory.md` gap classes B/C/D/E. Operator
  "verify locally, know it goes through" reliability remains uneven.
- Risk if ignored: The pilot proves the shape works; the value of
  RP-CI-PARITY scales with rollout. Stalled rollout means stalled
  parity gains — operators continue to be surprised by CI in
  unmodified repos.
- Effort: L (estimated 30-60 min per workspace × 13 workspaces;
  parallelisable per workspace with confirmation before push for
  each).
- Owner: Codex (per workspace, with confirmation before push per
  each repo's CLAUDE.md).
- Status: Open
- Next action: Per-workspace migration in inventory-recommended order:
  1. arena-tests (also closes `-08-08`)
  2. helms (also closes `-08-07`; needs desktop adapter work)
  3. mosaic-extensions/* (8 sub-repos; can be batched after first 1-2
     prove uniform shape)
  4. bedrock-platform/{converge, axiom, organism} — foundation
  5. runtime-runway
  6. atelier-showcase
  Per-repo ship pattern (mirrors commerce-rails pilot):
  - Promote Justfile to canonical shape (preserve existing repo-
    specific recipes alongside).
  - Add/update `.github/workflows/ci.yml` to thin-runner shape.
  - Add `.audit-ignores` + `deny.toml` + `dependabot.yml` if missing.
  - Smoke test `just ci` locally; must be green before push.
  - PR via /next branch flow.
- Verifies via: Every train repo's `just ci` runs the same canonical
  sequence; every workflow is the canonical thin runner; the matrix
  in `KB/06-operations/ci-cd-inventory.md` shows uniform shape across
  all 16 repos.
- Codex-safe now: Yes per workspace, with paired review for repos
  with quirks (helms desktop; runway infra; atelier demos).
- Properties: RP-CI-PARITY (cross-train migration)
- Confidence: H (canonical shape is proven by commerce-rails pilot;
  the unknown is how much per-repo quirk handling each workspace
  needs)
- Business leverage: Same as `QF-2026-06-08-10`. Every workspace
  migrated reduces the operator's daily-feedback-loop pain.
- Last reviewed: 2026-06-08 (filed at canonical-shape close)
- Cycles open: 0
- History:
  - 2026-06-08: Opened (closure follow-up to `QF-2026-06-08-10` —
    canonical shape + commerce-rails pilot landed; cross-train
    migration is the strategic rollout that scales the value).

#### QF-2026-06-07-02

- Date: 2026-06-07
- Bucket: C. Strategic improvement
- Area: CI/CD reliability / release engineering
- Discovered during: PR review (Tier 1) — closure of `QF-2026-06-02-13`
- Discovered in: B-tier follow-up session (2026-06-07)
- Evidence: `QF-2026-06-02-13`'s local-runnable verifies-via ("`just
  check-all-fresh` exits zero immediately after a rename commit") was
  satisfied by the `check-all-fresh` recipe shipped 2026-06-07. The
  `RP-FRESH-CLONE-GREEN` property's enforcement spec aspires to a SCHEDULED
  CI MATRIX that nukes target dirs and runs both `just check` AND `just
  test` across the train. The recipe covers the local `check` half;
  test-from-clean and scheduled CI are residual.
- Impact: Operator must remember to run `just check-all-fresh` before
  committing layout changes; if they forget, the `QF-13` incident pattern
  (silent path-dep breaks discovered only at release preflight) can recur.
  No CI-side fresh-clone signal exists today.
- Risk if ignored: Fresh-clone-green stays operator-discipline-dependent. A
  PR that doesn't get a fresh-clone gate can ship cargo cache poisoning
  that downstream consumers hit days later.
- Effort: M (workflow + slow test budget)
- Owner: Codex
- Status: Open
- Partial mitigation shipped 2026-06-07: repo-local GitHub workflows that had
  external upward Cargo path dependencies now materialize their required
  Reflective sibling checkouts before Cargo/audit/coverage/release jobs run.
  Covered repos: `atelier-showcase`, `runtime-runway`,
  `bedrock-platform/organism`, Mosaic extensions (`arbiter-policy`,
  `crucible-models`, `embassy-ports`, `ferrox-solvers`,
  `manifold-adapters`, `mnemos-knowledge`, `prism-analytics`, `soter-smt`),
  and `studio-apps/wolfgang-chat`. Repos with external upward paths but no
  local workflow files remain outside this mitigation because there is no
  repo-local CI file to patch.
- Next action: Provision the two missing native build dependencies that the
  first green fresh-clone run (2026-06-24) surfaced — `helms` needs the
  GTK/glib system stack (`libglib2.0-dev` + the Tauri GTK3 chain; glib-sys
  could not find `glib-2.0`), and `ferrox-solvers`'s `highs-sys` build.rs
  panics for want of a HiGHS build (`FERROX_HIGHS_ROOT` / `make highs`).
  Canonical recipe already exists: `runtime-runway/docker/Dockerfile.math-base`
  builds HiGHS `v1.14.0` (and OR-Tools) via cmake and exports
  `FERROX_HIGHS_ROOT=/opt/highs-src/build` — the fresh-clone workflow can
  either mirror those apt+cmake steps or run inside that prebuilt image.
- Verifies via: The scheduled workflow runs green on a no-changes week; runs
  red on a synthetic PR that breaks a downstream path-dep (e.g., rename a
  workspace dir without updating consumer Cargo.toml). Time budget per
  workspace is documented and CI fails if any workspace exceeds it.
- Codex-safe now: Yes
- Properties: RP-FRESH-CLONE-GREEN
- Confidence: M (depends on cargo registry cache behavior in scheduled CI;
  cargo-cache action may need tuning for fresh-clone semantics — without
  it the workflow runs ~60 min, with it the "fresh" half is partially
  defeated)
- Business leverage: One QF-13-pattern incident per quarter caught at
  scheduled-CI time saves ~4 hours of cross-workspace archaeology and
  prevents the "downstream user reports it first" embarrassment.
- Last reviewed: 2026-06-25 (fresh-clone runner-image regression fixes)
- Cycles open: 0 (newly opened)
- History:
  - 2026-07-03: Rerun after the fixes: **15/16 workspaces green** — helms
    passes (debug=0 resolved the SIGBUS). Sole remaining failure is
    ferrox-solvers, one layer deeper: libortools.so (v9.15) carries an
    undefined setLocalOptionValue(HighsLogOptions...) that vendor HiGHS
    v1.14.0 doesn't export — version skew between the HiGHS OR-Tools was
    built against and the Makefile pin. Shim: -Wl,--allow-shlib-undefined
    (runtime resolution against the co-installed /usr/local pair). Residual:
    align HIGHS_TAG with OR-Tools v9.15's expected HiGHS in
    ferrox-solvers/Makefile.
  - 2026-07-02 (late): Failure diagnosed from the 2026-07-02 dispatch run —
    14/16 workspaces green; the two failures are LINK failures, not tests:
    (1) helms: rust-lld SIGBUS linking the giant registry_test debug binary
    (surrealdb+rocksdb+onnxruntime) — fixed by CARGO_PROFILE_DEV_DEBUG=0 for
    the gate; (2) ferrox-solvers: ld.bfd "libabsl_raw_logging_internal.so:
    DSO missing from command line" — the --copy-dt-needed-entries rustflag
    lands after the -l list, positionally inert; fixed by naming
    absl_raw_logging_internal/log_severity/base explicitly in the
    workflow-written .cargo/config.toml. Residual: upstream those -l names
    into ferrox-ortools-sys build.rs so docker + local builds don't depend
    on the workflow shim.
  - 2026-07-02 (evening): Observability fix landed — fresh-clone.yml now runs one `just fresh-ws <ws>` step per workspace (live per-workspace progress, one compile instead of the old check+test double-clean) with a drift guard against release-train.yaml. Diagnosis of the actual test failures pending the in-flight 2026-07-02 dispatch run.
  - 2026-07-02: Self-audit: both stated next actions have landed in fresh-clone.yml (GTK/glib deps lines 141-154; HiGHS cmake lines 185-236) yet the latest 6 runs all fail, incl. the 2026-06-29 scheduled run (4h07m). Next action is now re-diagnosis of the current failure cause, not provisioning.
  - 2026-06-07: Opened (PR review Tier 1; closure follow-up to
    `QF-2026-06-02-13`)
  - 2026-06-07: Repo-local missing-sibling mitigation added across the 12
    workflow-bearing repos listed above. Verified by workflow YAML parse,
    `bash -n` on each new `scripts/ci/checkout-reflective-siblings.sh`, and
    a default-GitHub-checkout path simulation that confirmed every direct
    external Cargo path dependency lands under a helper checkout destination.
    Root scheduled `test-all-fresh` matrix remains open.
  - 2026-06-07: Root scheduled fresh-clone implementation added:
    `release-train.yaml` now records `fresh_workspaces` with check/test
    budgets for the 16 actual Cargo workspace roots; `Justfile` has
    budget-enforcing `check-all-fresh` and `test-all-fresh`; and
    `.github/workflows/fresh-clone.yml` checks out root + train repos
    side-by-side, installs system/Rust prerequisites, and runs both recipes
    weekly plus `workflow_dispatch`. Static verification passed
    (`just --summary`, YAML parse, budget-list extraction, `git diff --check`);
    first live CI run still pending, so the finding stays In progress.
  - 2026-06-07: Live push exposed a private app dependency:
    `atelier-showcase` depended on `../marquee-apps/quorum-sense`.
    Human direction clarified that Atelier must not depend on production
    apps. The fix is architectural, not token-based: remove the
    `quorum-open-inquiry` scenario from Atelier, remove the Quorum path
    dependencies, and drop the `quorum-sense` checkout from Atelier and root
    workflows.
  - 2026-06-25: Two scheduled-workflow runner-image regressions diagnosed
    and fixed (root PR #2, merged `b594a06`): (a) `fresh-clone` filled the
    runner OS disk (~14 GiB free) during the full-train fresh build —
    `No space left on device`; fixed by redirecting `CARGO_TARGET_DIR` to the
    ~70 GiB `/mnt` disk, nested one level inside a runner-owned parent so
    per-workspace `cargo clean` can remove the target. (b) `hermetic-audit`'s
    `unshare -rn` failed `uid_map: Operation not permitted` because
    `ubuntu-latest` moved to Ubuntu 24.04 with
    `kernel.apparmor_restrict_unprivileged_userns=1`; fixed by a sysctl step
    flipping it to 0 (namespace verify step now passes). With disk fixed, the
    first live fresh-clone run got 15/17 workspaces green within budget and
    surfaced two pre-existing native-dependency failures (previously masked by
    the disk crash): `helms` (glib-2.0 / GTK3 stack absent from the workflow's
    apt install) and `ferrox-solvers` (highs-sys build.rs needs a HiGHS build).
    Tracked in Next action above. Finding stays Open until fresh-clone is fully
    green.

#### QF-2026-06-08-05

- Date: 2026-06-08
- Bucket: C. Strategic improvement
- Area: AI-factory discipline / CI rollout
- Discovered during: closure of `QF-2026-06-07-01` (root-repo detector shipped)
- Discovered in: Camp-1 paired session (2026-06-08)
- Evidence: `QF-2026-06-07-01` shipped the `RP-TEST-CODE-ATTRIBUTION`
  mechanical detector — `scripts/check-test-code-attribution.sh` plus
  `.github/workflows/test-code-attribution.yml` — in the root repo only.
  The check fires on PRs to `Reflective-Lab/reflective`, but the 9 train
  workspaces (converge, axiom, organism, helms, mosaic, atelier, arena,
  runway, commerce) are independent gits with their own
  `.github/workflows/` and the workflow does not propagate.
- Impact: Today the bulk of AI-driven src+test edits happen in train
  workspaces, not the root. The detector exists but covers PRs to the
  workspace most edits don't target. Reviewer-enforced everywhere else.
- Risk if ignored: The `QF-2026-06-02-07` failure mode (AI agent rewrites
  production to make a stale test pass) can re-emerge in any train repo
  that the detector doesn't cover.
- Effort: M (9 workflow files; each repo also needs the script copied
  or fetched; consider whether to centralize the script via curl or
  duplicate per repo)
- Owner: Codex
- Status: Open
- Next action: Two design questions before rollout:
  1. **Script distribution** — copy `scripts/check-test-code-attribution.sh`
     into each train repo (duplication, version skew), OR fetch from
     root at workflow runtime via `curl` (network dependency in CI; one
     source of truth), OR ship as a composite GitHub Action in
     `Reflective-Lab/test-code-attribution-action` (clean, but new repo
     to maintain).
  2. **Workflow location** — one workflow per train repo (uniform CI
     across the train), OR root-level orchestrator that runs in cross-
     repo PR mode (one workflow file, but PRs to train repos wouldn't
     trigger root workflows). The composite action route makes this a
     non-question — train repos just add a 5-line workflow that calls
     the action.
- Verifies via: A synthetic test PR in each of the 9 train repos that
  modifies an `src/X.rs` + `tests/X.rs` pair without a classification
  fails CI; same PR with `Contract update: ...` passes.
- Codex-safe now: Yes once the distribution choice is made; the choice
  itself benefits from a paired call on the trade-offs.
- Properties: RP-TEST-CODE-ATTRIBUTION (rollout half; root coverage
  already Enforced)
- Confidence: M
- Business leverage: Per-train-repo coverage means an AI-driven src+test
  edit gets caught at PR time in the workspace where the edit happened,
  not after merge to the workspace, not at the next root-repo PR.
- Last reviewed: 2026-06-08 (filed at root closure)
- Cycles open: 0
- History:
  - 2026-06-08: Opened (closure follow-up to `QF-2026-06-07-01` —
    root coverage shipped; train-repo rollout needs a paired
    distribution-design call).

#### QF-2026-06-08-06

- Date: 2026-06-08
- Bucket: C. Strategic improvement
- Area: test determinism / reliability
- Discovered during: closure of `QF-2026-06-08-01` (pilot landed in
  prism-analytics)
- Discovered in: Camp-1 #3 paired session (2026-06-08)
- Evidence: `QF-2026-06-08-01` shipped the `RP-DETERMINISM` deny list
  in `mosaic-extensions/prism-analytics`: 7 `disallowed_methods`
  entries covering wall clock (`SystemTime::now`, `Instant::now`,
  `chrono::Utc::now`, `chrono::Local::now`), RNG (`rand::random`,
  `rand::thread_rng`), and broad env (`std::env::vars`). Five callsites
  surfaced and were annotated with `#[allow]` + justification. `cargo
  clippy --workspace --all-targets` green. The pilot proves the deny
  list shape; the remaining 6 train workspaces (converge, axiom,
  organism, helms, runtime-runway, commerce-rails) need the same
  rollout — same playbook as `QF-2026-06-02-05` (RP-HERMETIC-UNIT
  cross-train rollout).
- Impact: Today only prism-analytics is protected. The other 5 train
  workspaces have the same dynamic-test-failure surface — production
  code reading the wall clock without an injection seam, tests that
  pass for a window then start failing later.
- Risk if ignored: Same as `QF-2026-06-08-01`: tests that pass today
  fail next month; `cargo test` reliability erodes.
- Effort: L (cross-train rollout; per-workspace migration calls
  required for `#[allow]` annotation surface; estimated 20-50 callsite
  rollouts across the platform based on the prism-analytics ratio
  scaled by code size)
- Owner: Codex (per-workspace; paired call optional per workspace)
- Status: Open
- Next action: Per-workspace rollout, ideally in this order based on
  baseline-violation count (smallest first to validate the shape; then
  bigger workspaces with more rollout):
  1. **commerce-rails** — smallest, fewest wall-clock reads expected.
  2. **axiom** — small foundation crate; `guide_heading` rewriter was
     the original `QF-05` site, so this is a worth-double-checking
     workspace.
  3. **organism** — already had the `RP-HERMETIC-UNIT` DI migration;
     same playbook applies.
  4. **helms** — workbench layer; expect more wall-clock surfaces for
     UI / Workbench timing.
  5. **converge** — foundation; widest blast radius if shipped
     incorrectly.
  6. **runtime-runway** — largest production-wall-clock surface
     (Cloud Run / Firebase / Stripe all read wall clock); expect the
     most `#[allow]` annotations.
  Per-workspace ship pattern (mirrors `QF-2026-06-02-05`):
  - Add the 7 determinism entries to the workspace's `clippy.toml`.
  - Run `cargo clippy --workspace --all-targets`.
  - Annotate each callsite with `#[allow(clippy::disallowed_methods)]`
    + a one-line `RP-DETERMINISM exemption: <reason>` comment.
  - Verify green; commit + push to that workspace's main.
- Verifies via: `cargo clippy --workspace --all-targets` green on each
  of the 6 train workspaces with the determinism deny list active.
- Codex-safe now: Yes per workspace, with per-workspace human review
  before push (per each workspace's `CLAUDE.md > Never push to main
  without confirmation` rule).
- Properties: RP-DETERMINISM (rollout half; pilot already shipped in
  prism-analytics)
- Confidence: H (the playbook is proven by `RP-HERMETIC-UNIT` rollout
  and now by the prism-analytics pilot)
- Business leverage: Each workspace gets the same dynamic-test-failure
  protection as `RP-HERMETIC-UNIT` already gives for network. The
  `#[allow]` rollouts are documenting work — making explicit which
  production paths read the clock and which don't — and that
  documentation has value beyond the immediate hermeticity gain.
- Last reviewed: 2026-06-08 (filed at pilot closure)
- Cycles open: 0
- History:
  - 2026-06-08: Opened (closure follow-up to `QF-2026-06-08-01` —
    pilot deny list landed in prism-analytics; cross-train rollout
    is the playbook-following next step).

### Bucket D — Needs human decision


#### QF-2026-06-26-02

- Date: 2026-06-26
- Bucket: D. Needs human decision
- Area: governance / business leverage / platform extensibility
- Discovered during: paired session (Session Intelligence Spine design review)
- Evidence: The spine design §6 ("ExperienceStore and the Learning Loops")
  establishes two learning loops — within-session (live coordination) and
  across-session (durable group-dynamics / participant profiles in the
  ExperienceStore). The design explicitly flags a **governance boundary**:
  across-session profiles ("this participant tends to anchor high",
  "this group converges fast on cost, slow on risk") must not silently feed back
  into *live* coordination as anchoring weapons that pre-bias a session before it
  starts. The ADR `2026-06-26-helm-session-host-vs-coordination.md` follow-up 4
  records this as "a distinct Quorum-policy decision," separate from the
  session-host boundary.
- Impact: How (and whether) durable cross-session profiles may influence a live
  session is a risk-tolerance / product-fairness choice — it shapes whether the
  product amplifies participants or quietly manipulates the room.
- Risk if ignored: When across-session profiles get built (not yet — `mnemos`
  group-dynamics capture is underdeveloped), the absence of a stated boundary
  lets profile data leak into live coordination by default, which is the
  groupthink/anchoring failure mode the Quorum doctrine exists to prevent.
- Effort: S (policy decision) + M (the enforcement check when the feature lands)
- Owner: Karl
- Status: Open
- Next action: When across-session ExperienceStore profiles move from "capture
  now" to "use," decide the policy for whether/how they may inform live
  coordination (e.g. surfaced-to-human-only, never auto-applied to topology
  weights), and record it as an ADR. Until then this is a tracked guardrail, not
  active work.
- Verifies via: An ADR stating the across-session → live-coordination policy, plus
  a drift check (or admission-boundary rule) that enforces it once profiles are
  consumed.
- Codex-safe now: No — product/fairness/risk-tolerance decision.
- Confidence: M
- Last reviewed: 2026-06-26 (spine design review)
- Cycles open: 0
- History:
  - 2026-06-26: Opened (spine design review §6 governance boundary; ADR
    `2026-06-26-helm-session-host-vs-coordination.md` follow-up 4).
- ADR: *(pending — open when across-session profiles are implemented)*


#### QF-2026-06-02-26

- Date: 2026-06-02
- Bucket: D. Needs human decision
- Area: release engineering / security / AI-factory discipline
- Discovered during: PR review (Tier 2)
- Discovered in: PR Quality Gate Cycle 1 (Justfile release-train sequence)
- Evidence: `Justfile` line 2: `set dotenv-load := true` loads `.env` for every recipe. The
  release recipes are gated on env var `REL_APPLY=1`. A `REL_APPLY=1` line committed (or
  accidentally written) to `.env` permanently arms the live release path for every recipe
  invocation. Workspace memory records a preference for macOS Keychain + direnv over plaintext
  `.env` for secrets; `.env` discipline is already loose here.
- Impact: Release safety depends on `.env` hygiene that no other tooling enforces.
- Risk if ignored: First time someone debugs a release flow and forgets to remove
  `REL_APPLY=1` from `.env`, the next routine `just release` invocation goes live.
- Effort: S (technical) plus product/policy decision
- Owner: Karl
- Status: Open
- Next action: Decide between (a) refuse to read `REL_APPLY` from `.env` — require it inline
  (`REL_APPLY=1 just release converge`); (b) split into `dotenv-filename := ".env.just"`
  distinct from `.env`; (c) require a positional flag (`just release converge --apply`) in
  addition to the env var. Recommend (a) as minimal change. Record the decision as an ADR.
- Verifies via: A test that sets `REL_APPLY=1` in `.env` and confirms `just release converge`
  still treats the call as dry-run.
- Codex-safe now: No — needs human decision on dotenv discipline; intersects broader secrets
  policy.
- Confidence: H
- Last reviewed: 2026-06-02 (PR Gate Cycle 1)
- Cycles open: 0
- History:
  - 2026-06-02: Opened (PR review Tier 2; PR Gate Cycle 1 on `f359a14`)
- Linked PRs / commits: f359a14
- ADR: *(pending — propose `KB/04-architecture/decisions/0002-rel-apply-secrets-discipline.md`)*

## Accepted Risks

Live register: `KB/06-operations/risk-register.md`. Summarize open accepted risks here at the
end of each review cycle so this file remains a single-page status.

*(none)*

## Completed Findings

### QF-2026-07-02-01

- Date: 2026-07-02
- Bucket: B. Should fix soon
- Area: process / cross-agent instruction drift
- Discovered during: cross-agent alignment session (Claude)
- Evidence: Three agents were executing three different processes. Workflow
  skills lived user-global in `~/.claude/skills/` — invisible to Codex and
  Cursor, leaking into unrelated projects. `~/.codex/skills/` carried
  divergent forks of the same skills (its `focus` read `kb/Home.md`; `next`
  and `roadmap` referenced the retired MILESTONES.md flow). Root
  `AGENTS.md` (pre-fix lines 434–441) still declared the Linear import
  "blocked on `LINEAR_API_KEY`" after the import ran 2026-07-02.
  `marquee-apps` had no `AGENTS.md` — its policy lived in a 97-line
  Claude-only file. `bedrock-platform/CLAUDE.md` (60 lines),
  `bedrock-platform/CODEX.md` (63 lines), and `runtime-runway/CLAUDE.md`
  (45 lines) were forks; three files referenced the nonexistent
  `~/CLAUDE.md`/`~/dev/CLAUDE.md`. `~/.codex/rules/default.rules` globally
  auto-allowed `git push` (10 variants), bare `kill`, 11 stale `rm -rf`
  patterns, and destructive `git restore --source=HEAD`.
- Impact: Process divergence between agents (wrong tracking source, wrong
  branch flow) plus a global permission surface that let Codex push or
  delete without prompting in any repo on the machine.
- Risk if ignored: Every fork drifts further with each policy change —
  each fork is read by exactly one agent, so no session ever sees the
  disagreement.
- Effort: M
- Owner: Claude
- Status: Done — 2026-07-02. Root repo: skills repo-tracked at
  `.claude/skills/` + `SKILLS.md` catalog + `.codex/skills` symlink, stale
  transition rule retired (239ba2e); `just agents-doctor` drift check
  (81b80a2) — found and fixed one additional fork on first run
  (`bedrock-platform/CODEX.md`). Fleet commits: bedrock-platform 673eb7f +
  f972114 + a873599, runtime-runway 4b17651, marquee-apps 0d12a60,
  lattice-mesh 675cbf6, atelier-showcase 0121df7 (on `next`), beacon-sites
  027dab4, blueprint-apps 0315407. Home directories: ten colliding Codex
  skills retired to `~/.codex/skills-retired-2026-07-02/`; claude originals
  in `~/.claude/skills-migrated-2026-07-02/`; `default.rules` pruned
  292 → 259 lines (backup `default.rules.bak-2026-07-02`). Standard
  promoted: `KB/05-engineering/standards/cross-agent-instruction-files.md`.
- Residual: `forge-templates/` is not a git repo, so its new `AGENTS.md` is
  unversioned; `blueprint-apps` has no remote (local-only history);
  runtime-runway `main`/`next` diverged before this work (both now carry
  the docs commit).
- Confidence: H
- Closed: 2026-07-02

### QF-2026-07-01-01

- Date: 2026-07-01
- Bucket: B. Should fix soon
- Area: correctness / ambient job processing / quorum-sense
- Discovered during: session review
- Evidence: `quorum-sense` has no `AmbientJobHandler` of its own. The job keys
  `sensemap-refresh`, `mnemos-recall`, and `drift-scan` and their dispatch logic
  lived in `quorum-sense` temporarily but were not carried forward as a permanent
  handler registration. Without an owned handler, the runtime has no dispatcher
  to route those keys and ambient jobs silently go unprocessed.
- Impact: All three ambient job types (`sensemap-refresh`, `mnemos-recall`,
  `drift-scan`) are dead at runtime — the SenseMap refresh cycle, cross-session
  Mnemos recall, and drift detection do not fire.
- Risk if ignored: M3 ambient capabilities (`SenseMap`, recall, anticipatory
  signals) degrade silently with no observable error; product proof run and the
  Atlas ⇄ Quorum demo rely on live SenseMap state that will not update.
- Effort: S (copy the handler pattern from the source; wire into the server
  startup alongside existing module mounts)
- Owner: Codex
- Status: Done — verified 2026-07-01. `QuorumAmbientHandler` was already fully
  implemented in `crates/quorum-server/src/ambient_handler.rs` and wired at
  `main.rs:1744` via `.with_ambient_handler(...)`. `cargo build -p quorum-server`
  exits 0. The finding was stale before it was filed.
- Confidence: H
- Closed: 2026-07-01

<!-- 2026-06-24 mobile boundary audit (mobile-apps repo): a trust-boundary pass
     beyond `cargo deny`. Findings QF-2026-06-24-01..06 map one audit per seam —
     dependency advisories, the UniFFI panic seam, CI supply chain, the Android
     dependency-verification gap, the crash-telemetry PII pipe, and fuzzing the
     untrusted-input seam. -->

### QF-2026-06-08-09

- Date: 2026-06-08
- Bucket: B. Should fix soon
- Area: CI/CD reliability / commercial authority
- Discovered during: CI/CD inventory (`KB/06-operations/ci-cd-inventory.md`)
- Evidence: `commerce-rails` has no `.github/workflows/` directory
  and no `dependabot.yml` / `deny.toml`. The local Justfile has 6
  recipes (`check`, `test`, `fmt`, `fmt-check`, `lint`), and even
  those use bare `cargo` commands without `--workspace`. Commerce-rails
  is the commercial-authority workspace (billing, entitlements, app
  installs, partner revenue share, payouts, refunds, disputes,
  reconciliation per `KB/CLAUDE.md`) — a regression there has direct
  revenue impact.
- Impact: The workspace owning commercial-authority has no PR safety
  net. A bug landing on main affects billing / entitlements /
  reconciliation and is discovered only at runtime.
- Risk if ignored: Highest-blast-radius gap in the inventory.
  Commercial-authority regressions are not the kind to discover late.
- Effort: S (smallest workspace; the canonical CI shape applies
  cleanly).
- Owner: Codex
- Status: Done — 2026-07-02 self-audit. commerce-rails now has the full set:
  canonical thin-runner `ci.yml` (RP-CI-PARITY shape) on origin/main,
  `security.yml`, `dependabot.yml`, `deny.toml`, `clippy.toml`, and a
  canonical `just ci` recipe. CI green on main push (run 28601314185,
  2026-07-02) and on dependabot PRs.
- Next action: (a) Promote the Justfile to canonical shape
  (`cargo check --workspace --all-targets`, etc.). (b) Add
  `.github/workflows/{ci,security}.yml`. (c) Add `dependabot.yml`,
  `deny.toml`. Use mosaic-extensions/prism-analytics as the template.
- Verifies via: CI runs `cargo check --workspace --all-targets +
  test + clippy -D warnings + audit` on every PR; matches what
  operators get from local `just ci`.
- Codex-safe now: Yes
- Properties: RP-CI-PARITY (proposed)
- Confidence: H
- Business leverage: Commercial-authority correctness is directly
  revenue-relevant. CI here is non-negotiable for any future commerce
  release.
- Last reviewed: 2026-06-08 (audit)
- Cycles open: 0
- History:
  - 2026-07-02: Closed Done (self-audit verified all next-action items shipped; CI green on main).
  - 2026-06-08: Opened (CI/CD inventory revealed commerce-rails has
    no CI — highest revenue-blast-radius gap).
- Closed: 2026-07-02

### QF-2026-06-02-14

- Date: 2026-06-02
- Bucket: D. Needs human decision
- Area: business leverage / platform extensibility
- Discovered during: paired session (commerce-rails dep blocked publish)
- Evidence: `commerce-rails` is `UNLICENSED` with `publish = false` for
  all crates. `runway-accounts` and `runway-app-host` carry a path-dep
  on `commerce-rails-stripe`, which means they cannot be published to
  crates.io until commerce-rails resolves its publish status. The
  release train hit this wall and skipped those two crates.
- Impact: The runtime-runway release is structurally incomplete; the
  Commerce Rails roadmap is implicitly blocked behind a licensing
  decision.
- Risk if ignored: Each release leaves the same two runtime crates
  stranded; downstream apps that want runway-accounts via crates.io
  have no path.
- Effort: M (technical work) + human decision
- Owner: Karl
- Status: Done — 2026-07-02 self-audit. The blocking decision was made and
  implemented in code: `commerce-rails/Cargo.toml` now reads `license =
  "MIT"`, `publish = true` (v0.2.2), and `runtime-runway/Cargo.toml:49`
  consumes it with a publishable dep spec. ADR backfilled:
  `KB/04-architecture/decisions/2026-07-02-commerce-rails-publishable.md`.
- Next action: Decide whether commerce-rails ships as MIT (publishable),
  stays internal (then runway-accounts must split out a smaller
  publishable surface that doesn't depend on it), or sits behind a
  private registry. This is a product / commercial decision, not a
  Codex-safe one.
- Verifies via: A documented decision in `MASTERPLAN.md` or
  `EPICS.md`, with the chosen path reflected in the layering lint
  (QF-2026-06-02-08 (b)).
- Codex-safe now: No — needs human product decision.
- Properties: RP-LAYERING
- Confidence: H
- Last reviewed: 2026-06-02 (Cycle 1)
- Cycles open: 1
- History:
  - 2026-07-02: Closed Done (obsolete — decision implemented in code; ADR backfilled).
  - 2026-06-02: Opened (paired session; Cycle 1)
- ADR: `KB/04-architecture/decisions/2026-07-02-commerce-rails-publishable.md`
- Closed: 2026-07-02

### QF-2026-06-27-01

- Date: 2026-06-27
- Bucket: D. Needs human decision
- Area: architecture boundary / platform extensibility / DX
- Discovered during: paired session (AI Director UX integration with the Spine)
- Evidence: The AI Director UX architecture
  (`KB/04-architecture/2026-06-27-ai-director-mobile-ux-architecture.md`) hedged on
  where the canonical `DirectorFrame` lives — "`helm-session-contracts` (or a thin
  sibling if Plan 1 chooses that split)" (the file's Ownership Split section before
  this session). Two boundaries are actually in play: the server↔client **wire**
  (`helm-session-contracts`: `SessionPush`, `GatedDecision`, ...) and the Rust→FFI/UI
  **projection** (`DirectorFrame` + prompt/action vocabulary). Conflating them in one
  crate couples the wire model to UI shape; forking `DirectorFrame` into
  `mobile-apps/crates/mobile-core` (the parallel risk flagged by the UX agent) would
  split the source of truth across two git roots.
- Impact: A clean two-boundary split makes `DirectorFrame` reusable across mobile /
  desktop / web with one source of truth; the wrong call either entangles the wire
  contract with UI churn or lets mobile fork the canonical type.
- Risk if ignored: The UX agent builds SwiftUI/Compose against a `DirectorFrame` with
  no fixed home, and `mobile-core` grows a divergent copy — the exact duplication the
  "we own what we own, don't wrap" directive forbids.
- Effort: S (decision + contract spec) — implementation rides Plan 1 Tasks 1b + 5b.
- Owner: Karl
- Status: Done
- Resolution: Plan 1 landed in `bedrock-platform/helms` — `director-contracts`
  (projection boundary), `helm-session-contracts` (wire boundary), and
  `helm-client::director_snapshot` + `DomainPresenter` seam. Evidence: helms
  `526e5ef` (8 contract tests, clippy clean) and `4144c45` (44 helm-client tests
  incl. director projection; 52 total across the three spine crates).
- Next action: None — closed. **Handoff:** UX agent consumes/re-exports
  `director-contracts` via `mobile-core` (M3A.8); Quorum FFI implements
  `DomainPresenter`. Do not fork `DirectorFrame` in mobile-apps.
- Decision: `DirectorFrame` and its vocabulary live in a **dedicated
  `director-contracts` crate** (the Rust→FFI/UI projection boundary), distinct from
  `helm-session-contracts` (the wire boundary), depending on it one-way (for shared
  ids like `GateId`). `helm-client` produces a versioned `DirectorFrame`
  (`version` = upstream SSE `sequence`) and stays domain-agnostic via a
  `DomainPresenter` seam (it owns frame structure/lifecycle; the per-app FFI owns the
  domain copy). Prompts may only present contract-backed verdicts (`GateVerdict` =
  approve/reject; no UI-only "later"). Recorded in the spine design §4–§5, Plan 1
  (Tasks 1b + 5b), and the AI Director UX doc Ownership Split.
- Verifies via: `director-contracts` lands as its own crate (round-trip tests green);
  `helm-client::director_snapshot` projects from session state behind the
  `DomainPresenter` seam; `mobile-core` consumes/re-exports it without a forked
  `DirectorFrame`.
- Codex-safe now: No — it set a cross-repo architecture boundary requiring human
  judgment; the implementation is Plan 1 work.
- Properties: RP-LAYERING, RP-TYPED-CROSS-LAYER-SEMANTICS
- Confidence: H
- Last reviewed: 2026-06-27 (AI Director UX integration session)
- Cycles open: 0
- History:
  - 2026-06-27: Opened and **decided same day** under the "we own what we own —
    strengthen upstream, don't wrap" directive. Chose a dedicated `director-contracts`
    crate (projection boundary) over folding `DirectorFrame` into
    `helm-session-contracts` (wire boundary) or letting `mobile-core` fork it.
    Specified the types and the `helm-client` `DomainPresenter` projection seam in
    Plan 1 (`KB/08-roadmap/2026-06-26-spine-plan-1-contracts-helm-client.md`, Tasks 1b
    + 5b); updated spine design §4–§5 and the AI Director UX doc.
  - 2026-06-27: **Done.** Plan 1 implementation landed helms `526e5ef` +
    `4144c45` — all three spine crates green (52 tests, clippy `-D warnings`).
    Handoff to mobile M3A for `mobile-core` re-export + Quorum `DomainPresenter`.
- ADR: *(captured inline in the spine design §5 + Plan 1; promote to a standalone ADR
  if a future consumer reopens the boundary)*

### QF-2026-06-24-04

- Date: 2026-06-24
- Bucket: B. Should fix soon
- Area: supply-chain hygiene / security
- Discovered during: audit
- Evidence: The Android Gradle build (`apps/marquee/quorum-sense/android`)
  resolved Maven artifacts (AGP, Kotlin, Compose, Sentry plugin) with **no**
  checksum/signature gate — no `gradle/verification-metadata.xml` existed, while
  the iOS Swift Package graph is pinned via `Package.resolved`. Sole unguarded
  supply-chain edge in the repo.
- Impact: A tampered or swapped Gradle dependency/plugin would enter the build
  unverified — the classic native-mobile supply-chain hole.
- Risk if ignored: A compromised plugin/AAR runs in CI (which holds
  `SENTRY_AUTH_TOKEN`) and ships in the APK, undetected.
- Effort: M
- Owner: Codex
- Status: Done
- Next action: None — closed. Strict verification is enforced on main; future
  dependency bumps regenerate complete cross-platform metadata via the "Android
  Verification Metadata (Linux)" `workflow_dispatch` job + `just
  quorum-android-verify-metadata`.
- Verifies via: PR #29's Android CI — including the instrumented emulator job —
  passed in **strict** mode on Linux, proving the 581-component macOS+Linux metadata
  is complete; `gradle.properties` carries no `...verification=lenient` override, so
  a checksum mismatch now fails the build.
- Codex-safe now: Yes — done. (The earlier "one-line delete gated on green CI"
  framing was wrong; the real fix was cross-platform metadata generation, now landed.)
- Confidence: H
- Linked PRs / commits: mobile-apps #29 (strict flip + 581-component superset + the
  Linux generator workflow); built on #26 (566-component AGP 9 regen) and the
  original lenient rollout.
- Drift check: strict `verification-metadata.xml` enforcement in every Android CI
  job; `.github/workflows/android-verify-metadata.yml` regenerates it on Linux.
- History:
  - 2026-06-24: Opened (mobile boundary audit). Generated sha256 metadata across
    debug+release+androidTest+unitTest classpaths (469 components); committed in
    **lenient** mode (warn, not fail) to de-risk the one classpath unresolvable
    off-CI (`connectedDebugAndroidTest` emulator-time artifacts). Strict flip is
    the residual.
  - 2026-06-24: **Correction to the strict-flip framing.** The toolchain upgrade
    (mobile-apps PR #26) regenerated the metadata to 566 components (AGP 9 / Kotlin
    2.4 graph) and went green — but only because verification runs in **lenient**
    mode. That same green run logged `Dependency verification failed` for `classpath`,
    `kotlinBuildToolsApiClasspath`, and `detachedConfiguration*` (downgraded to
    warnings). This proved the macOS-generated metadata is **incomplete for the Linux
    CI runner**, so a green *lenient* run is NOT evidence of strict-readiness — the
    prior "one-line delete gated on green CI" was wrong. The real gate is a green
    **strict** run on Linux-inclusive metadata. Reframed Next action / Verifies via /
    Codex-safe-now. Shipped in the v0.1.1 release notes too.
  - 2026-06-25: **Closed — strict enforced.** Added a `workflow_dispatch` job
    (`android-verify-metadata.yml`) that regenerates the metadata on an ubuntu
    runner; ran it, merged its artifact into the file (566 → 581 components, the
    missing Linux build-tooling artifacts now included), removed the lenient
    override, and proved it with a green **strict** CI run on Linux — PR #29, incl.
    the instrumented emulator test. The supply-chain boundary is now enforcing, and
    the audit stands at 6 of 6 closed.

### QF-2026-06-24-06

- Date: 2026-06-24
- Bucket: C. Strategic improvement
- Area: tests / security
- Discovered during: audit
- Evidence: The untrusted-input seam — `draft_field_signal(inquiry_thread_id,
  modality, raw_capture)` and the enum/`Confidence` parsers in
  `crates/mobile-core/src/quorum.rs` — takes attacker-influenceable text
  (transcripts, OCR, free notes) crossing the FFI from the native shells. Covered
  by property tests but not fuzzed; fuzzing is the next rung above the existing
  proptest culture.
- Impact: Coverage-guided fuzzing finds panics/overflows on inputs property
  tests' generators miss — exactly the implicit-abort paths QF-2026-06-24-02
  forbids in shipped code.
- Risk if ignored: A panic-triggering input class stays undiscovered until it
  crashes a user's app across the FFI seam.
- Effort: M
- Owner: Codex
- Status: Done
- Next action: None — harness proven green, seed corpus committed, and a PR-time
  smoke now guards core/FFI/harness changes.
- Verifies via: `just fuzz-core <target>` and `.github/workflows/fuzz.yml` run the
  three libFuzzer targets to completion with no crash; a discovered reproducer
  fails the job and uploads as an artifact. The PR-time `cargo-fuzz smoke
  (draft_field_signal)` job ran green on mobile-apps PR #28.
- Codex-safe now: Yes — harness + scheduled CI are additive and isolated.
- Confidence: H
- Linked PRs / commits: harness in the mobile boundary audit change set
  (`crates/mobile-core/fuzz/` with `draft_field_signal`, `parse_enums`,
  `confidence_roundtrip`; Justfile `fuzz-core`; `.github/workflows/fuzz.yml`);
  corpus + PR smoke in mobile-apps PR #28 (`dcd33d4`).
- History:
  - 2026-06-24: Opened (mobile boundary audit). libFuzzer harness landed,
    detached from the stable workspace (empty `[workspace]` so the nightly
    `libfuzzer-sys` dep never enters `cargo build --workspace`); nightly fuzz
    scheduled. First green CI run is the residual.
  - 2026-06-24: Closed — a manual `Fuzz` dispatch proved all three targets green
    (no crash). Seeded a committed corpus (`seeds/<target>`: every valid
    enum string, boundary `f32` confidence values, realistic capture text incl.
    unicode/empty), read at runtime with discoveries written to the gitignored
    `corpus/<target>`. Added a paths-scoped PR-time smoke of the hottest target
    (`draft_field_signal`, 30s) — self-verified by its own green run on PR #28 —
    plus a fuzz README. The full 3-target matrix still runs nightly + on dispatch.

### QF-2026-06-24-05

- Date: 2026-06-24
- Bucket: B. Should fix soon
- Area: security / observability
- Discovered during: audit
- Evidence: Three Sentry clients ship crash telemetry off-device — Rust
  (`apps/marquee/quorum-sense/ffi/src/lib.rs` `init_observability`), iOS
  (`ios/App/QuorumMobileApp.swift` `SentrySDK.start`), Android (manifest
  auto-init `io.sentry.*` meta-data). The Quorum payload is field-signal capture
  (`raw_capture`: transcript/OCR/free text, `SignalModality` in
  `schemas/quorum-mobile.udl`) — sensitive content that a crash event can carry
  off-device via hostname/user context or an exception string.
- Impact: A crash pipe is an outbound channel we built on purpose; at defaults it
  can exfiltrate device/user identity and potentially capture content,
  contradicting the ADR 0002 "device owns capture" posture.
- Risk if ignored: Sensitive user capture leaks through telemetry — a privacy
  incident that is invisible until audited.
- Effort: M
- Owner: Codex
- Status: Done
- Next action: None — all three clients scrub, and both Android SDKs (app +
  Rust core) now initialise once in `QuorumApplication.onCreate`.
- Verifies via: each client sets `send_default_pii=false` and strips
  `server_name`/`user` before send. Android scrub covered by `ScrubPiiSpec`
  (Kotest, `:app:testDebugUnitTest` green: 2 tests, 0 failures); Rust/iOS scrubs
  compile-verified in `before_send`.
- ADR: `docs/adr/0004-telemetry-pii-boundary.md`
- Drift check: `ScrubPiiSpec` fails if the Android scrub stops nulling
  `serverName`/`user`.
- Codex-safe now: Yes.
- Confidence: H
- Linked PRs / commits: this change set (Rust `scrub_pii`, iOS `options.beforeSend`,
  Android `QuorumApplication` code-init + `scrubPii` + `ScrubPiiSpec`, manifest
  `auto-init=false` + `send-default-pii=false`, ADR 0004).
- History:
  - 2026-06-24: Opened (mobile boundary audit). Rust + iOS `before_send` scrub
    and explicit `send_default_pii=false` landed and compile-verified; Android
    manifest hardened; ADR 0004 recorded. Residual: Android `beforeSend`.
  - 2026-06-24: Closed — Android `beforeSend` wired via code-based
    `SentryAndroid.init` in a custom `QuorumApplication` (manifest auto-init
    disabled); `scrubPii` extracted + unit-tested (`ScrubPiiSpec`); ADR 0004
    status table updated to all-✅. `:app:testDebugUnitTest` green.
  - 2026-06-24: Consolidated init — moved the Rust `initObservability` call from
    `MainActivity.onCreate` into `QuorumApplication.onCreate` so the app SDK and
    Rust-core reporter initialise once, together, at process start.
    `:app:testDebugUnitTest` + `:app:assembleDebug` green.

### QF-2026-06-24-03

- Date: 2026-06-24
- Bucket: B. Should fix soon
- Area: supply-chain hygiene / CI/CD
- Discovered during: audit
- Evidence: All GitHub Actions in `.github/workflows/{ci,release-preflight}.yml`
  were pinned to mutable tags (`actions/checkout@v4`,
  `reactivecircus/android-emulator-runner@v2`, `taiki-e/install-action@cargo-deny`,
  …). A moved tag = arbitrary code execution in CI, which reads
  `SENTRY_AUTH_TOKEN`. No Dependabot config existed.
- Impact: CI is a privileged environment (release symbol upload, repo token); a
  hijacked third-party tag is a direct supply-chain compromise vector.
- Risk if ignored: A single upstream tag move or account compromise runs
  attacker code with the workflow's secret access.
- Effort: S
- Owner: Codex
- Status: Done
- Next action: None — closed. Dependabot now proposes SHA bumps with changelogs.
- Verifies via: `grep -rE 'uses:.*@v[0-9]+$' .github/workflows/` returns nothing;
  every `uses:` is a 40-char SHA with a `# vN` comment; `.github/dependabot.yml`
  covers github-actions + cargo + gradle.
- Drift check: the grep above (candidate `project-doctor` check).
- Codex-safe now: Yes.
- Confidence: H
- Linked PRs / commits: this change set (SHA pins across both workflows +
  `.github/dependabot.yml`).
- History:
  - 2026-06-24: Opened + closed same session (mobile boundary audit). All 9
    distinct actions SHA-pinned with `# vN` comments; Dependabot added for the
    three ecosystems the repo pulls from. SPM left to manual review
    (`Package.resolved`, no root `Package.swift` for Dependabot).

### QF-2026-06-24-02

- Date: 2026-06-24
- Bucket: B. Should fix soon
- Area: security / reliability
- Discovered during: audit
- Evidence: `reflective-mobile-core` and `reflective-mobile-ai` run *underneath*
  the UniFFI seam, where a Rust panic unwinds into Swift/Kotlin (undefined
  behaviour, not a catchable error). No lint guarded implicit-abort paths
  (`unwrap`/`expect`/`panic`) or hand-written `unsafe` in shipped library code.
- Impact: A fallible path that slips an `unwrap` into the domain core can abort
  the host app process across the FFI boundary rather than surface a typed error.
- Risk if ignored: Latent UB / hard crashes that bypass the typed `QuorumError`
  contract, hard to attribute and reproduce.
- Effort: S
- Owner: Codex
- Status: Done
- Next action: None for the pure crates. Residual: the `quorum-ffi` crate cannot
  take a crate-level `forbid(unsafe_code)` / panic-deny because UniFFI's
  `include_scaffolding!` generates `unsafe` + 5 `unwrap`/`expect` inline; its
  hand-written marshalling is already panic-free. A scoped lint on FFI hand-code
  is a possible later refinement (low value — the panic source is the core).
- Verifies via: `cargo clippy --workspace --all-targets -- -D warnings` is green
  with `#![forbid(unsafe_code)]` + `#![deny(clippy::unwrap_used, expect_used,
  panic)]` at the root of both crates' `src/lib.rs`; tests exempt via
  `clippy.toml` (`allow-unwrap/expect-in-tests`).
- Drift check: the clippy gate already in `ci.yml` (the lints are crate
  attributes, so any new `unwrap` in shipped core code fails CI).
- Codex-safe now: Yes.
- Confidence: H
- Linked PRs / commits: this change set (`clippy.toml`, inner attributes in
  `crates/mobile-core/src/lib.rs` + `crates/mobile-ai/src/lib.rs`).
- History:
  - 2026-06-24: Opened + closed same session (mobile boundary audit). Scoped to
    the shipped library via inner attributes (not Cargo `[lints]`, which also
    hit `tests/` integration crates that legitimately unwrap). Verified clippy
    green.

### QF-2026-06-24-01

- Date: 2026-06-24
- Bucket: B. Should fix soon
- Area: supply-chain hygiene / security
- Discovered during: audit
- Evidence: CI ran `cargo deny check bans` (the EPIC-4 server-crate boundary) but
  **not** `cargo deny check advisories` — the actual RUSTSEC CVE gate — despite a
  live network stack in the lockfile (`reqwest`, `tokio`, `hyper`, `rustls`,
  `ureq`). `cargo deny check advisories` passes clean today, so the gate is free
  to add.
- Impact: A newly-disclosed advisory in the TLS/HTTP stack would land silently;
  nothing failed CI on vulnerabilities.
- Risk if ignored: Ship a known-vulnerable transitive dependency in a mobile
  binary with no signal.
- Effort: S
- Owner: Codex
- Status: Done
- Next action: None — closed. A genuinely unfixable transitive advisory is
  recorded as an Accepted Risk + `ignore`d by RUSTSEC id with a dated rationale,
  never blanket-silenced.
- Verifies via: `cargo deny check advisories` is a required step in `ci.yml` and
  exits 0; `deny.toml [advisories]` sets `yanked = "deny"`,
  `unmaintained = "workspace"`, `ignore = []`.
- Drift check: the new CI step itself.
- Codex-safe now: Yes.
- Confidence: H
- Linked PRs / commits: this change set (`deny.toml` advisories config + `ci.yml`
  step).
- History:
  - 2026-06-24: Opened + closed same session (mobile boundary audit). Advisories
    clean at close; gate added so the next disclosure fails CI.

### QF-2026-06-13-01

- Date: 2026-06-13
- Bucket: C. Strategic improvement (closed Done within 24h — same week)
- Area: architecture documentation / platform-vs-app type asymmetry
- Discovered during: Step 3 (CoordinatorSuggestor) verification pass —
  a code reviewer flagged `quorum_app::Proposal: !Clone` as an
  accidental gap that "would let the UpstreamOutcomeFixture collapse
  to a one-liner." Investigation across `bedrock-platform/converge/`
  KB, `bedrock-platform/organism/` KB, and the Quorum self-review
  found the doctrine was implicit in code but absent from any
  written architectural source.
- Evidence:
  - `converge_core::types::Proposal<State>` derives `Debug, Clone`
    (`bedrock-platform/converge/crates/core/src/types/proposal.rs:191`).
  - `quorum_app::Proposal` had no derives at all — not `Clone`, not
    `Debug`, not `Serialize`
    (`marquee-apps/quorum-sense/crates/quorum-app/src/lib.rs:308` at
    opening).
  - Conversion seam `into_proposed_fact(self)` consumed the Proposal;
    admission performed a second exhaustive match in `apply_proposal`
    projecting each variant to either a kernel chain write +
    `DomainEvent`, a cache-only projection, or an Arbiter Cedar gate.
  - Test pattern: `UpstreamOutcomeFixture` in
    `crates/quorum-app/tests/coordinator_role_contracts.rs:43-114`
    seeded upstream Suggestor outputs by running a fixture Suggestor
    through the engine — not by injecting fake proposals.
- Impact: A load-bearing platform-vs-app type asymmetry that any
  future reviewer (human or agent) would have rediscovered as
  "accidental" and proposed "fixing." Each rediscovery costs a
  verification round and risks an actual `#[derive(Clone)]` PR that
  silently weakens the Suggestor → Engine handoff contract.
- Risk if ignored: A future contributor adds `#[derive(Clone)]` to
  `quorum_app::Proposal` to simplify a test, removing the type-system
  signal that proposals are one-shot admission tokens bound to their
  emitting Suggestor's provenance.
- Effort: S (delivered).
- Owner: Quorum app owner + Converge maintainer (parallel work).
- Status: Done
- Resolution: Doctrine landed in three coordinated commits across
  three repos:
  1. Quorum-side "Proposal Envelope Doctrine" section in
     `marquee-apps/quorum-sense/kb/Architecture/Converge Usage Self-Review.md`
     (commit `5015584` on `marquee-apps/quorum-sense@next`).
  2. Upstream Converge KB "App-Layer Proposal Envelopes" section in
     `bedrock-platform/converge/kb/Concepts/Proposals and Promotion.md`
     (integrated by the Converge maintainer alongside `SubjectRef`
     shipment; see `bedrock-platform/converge` `b47b6d4`).
  3. Operationalised in code: every Quorum `Proposal` variant now
     tags its admitted fact with a typed `SubjectRef` via
     `Proposal::subject() + ProposedFact::with_subject(...)`, putting
     the envelope/subject distinction into running code (commit
     `20ea2c5` on `marquee-apps/quorum-sense@next`).
- Verifies via: `grep -nE "admission protocol" bedrock-platform/converge/kb/Concepts/"Proposals and Promotion.md"`
  returns a match (line 99 at closure); the Quorum self-review carries
  the matching "Proposal Envelope Doctrine" + "App Subject Taxonomy"
  sections; `cargo test -p quorum-app --lib proposal_subject` runs the
  7 SubjectRef adoption tests green.
- Codex-safe now: Yes (closed).
- Properties: still a candidate for an `RP-ADMISSION-PROTOCOL-ENVELOPE`
  recurring property once a second app-layer Proposal enum exists on
  the platform (currently only Quorum has one — too early to commit
  the platform to the convention).
- Confidence: H — all three closure artefacts verified at HEAD on
  their respective branches; no remaining open work.
- Business leverage: Future apps on the platform that build
  Suggestor → Kernel admission pipelines now have explicit doctrine
  (Converge KB) + a worked example (Quorum SubjectRef adoption).
- Last reviewed: 2026-06-13 (closed Done same week as opened)
- Cycles open: 0
- History:
  - 2026-06-13: Opened (Step 3 verification pass; Quorum-side
    doctrine landed same day via commit `5015584`).
  - 2026-06-13: Upstream Converge KB mirror integrated by Converge
    maintainer alongside `SubjectRef` shipment.
  - 2026-06-13: Operationalised via Quorum SubjectRef adoption
    (`20ea2c5`); closed Done.
- Linked PRs / commits: `marquee-apps/quorum-sense@5015584`,
  `marquee-apps/quorum-sense@20ea2c5`, `bedrock-platform/converge@b47b6d4`.

### QF-2026-06-08-04

- Date: 2026-06-08
- Bucket: B. Should fix soon (closed Done same session)
- Area: CI/CD reliability / delivery system
- Discovered during: customer report
- Evidence: `atelier-showcase` GitHub Actions on `main` were red on the latest
  push before this fix. CI run `27105687927` failed `Check` on
  `scenarios/round-driven-formation-design/src/main.rs:1575` because
  `LlmSynthesisProducer` implemented `SynthesisProducer` without the new
  associated `Payload` type; the same run's `Format` job failed on
  `bedrock-platform/helms/crates/prio-expenses/src/receipt_extractor.rs`,
  proving `cargo fmt --all -- --check` was formatting a sibling checkout rather
  than only the `atelier-showcase` workspace. Security run `27105687926` failed
  only the raw `cargo audit --deny warnings` job while `Cargo Deny` and secrets
  scan were green; the failing advisories were accepted transitive unmaintained
  warnings (`RUSTSEC-2025-0141`, `RUSTSEC-2025-0119`,
  `RUSTSEC-2024-0436`, `RUSTSEC-2025-0134`). Coverage run `27105687923`
  failed the CRM Helm build script with `Could not find protoc`. First
  post-fix Coverage run `27130271265` then reached measurement and failed
  the floor at 44.0% because `.github/workflows/coverage.yml` ignored
  `examples/` while `Justfile` coverage ignored `tutorials/` and
  `scenarios/`.
- Impact: A routine Atelier push produced multiple independent false-red or
  stale-contract failures, reinforcing the user's report that GitHub CI/CD
  failure was the norm rather than an exception.
- Risk if ignored: Operators keep discounting red CI and real regressions hide
  behind known noise; coverage and security gates remain unactionable.
- Effort: M
- Owner: Codex
- Status: Done
- Next action: No further action; keep watching routine `atelier-showcase`
  pushes for recurrence.
- Verifies via: `cargo check -p scenario-round-driven-formation-design`;
  `just check`; `just lint`; `just test`; `bash scripts/ci/install-protobuf.sh`;
  `bash scripts/ci/cargo-audit-blocking.sh`; `just security-audit`;
  `cargo fmt --check`; `git diff --check`; GitHub Actions CI run
  `27131062287`; Security run `27131062310`; Coverage run `27131062315`.
- Codex-safe now: Yes
- Properties: RP-FRESH-CLONE-GREEN
- Confidence: H
- Business leverage: Restores the signal value of Atelier's required CI jobs;
  each red push no longer requires manual triage across compile, format,
  security, and coverage logs before work can proceed.
- Last reviewed: 2026-06-08 (same-session fix)
- Cycles open: 0
- History:
  - 2026-06-08: Opened and closed (user-directed move into
    `atelier-showcase` after repeated GitHub CI/CD failures; local and
    workflow-level fixes applied in the same session).
  - 2026-06-08: Added follow-up coverage workflow repair after
    post-protobuf run `27130271265` exposed a stale ignore pattern; final
    push verified CI, Security, and Coverage green on GitHub Actions.
- Linked PRs / commits: Reflective-Lab/atelier-showcase@9bce455,
  Reflective-Lab/atelier-showcase@362fa0b
- Standard promoted: n/a
- Drift check: `.github/workflows/ci.yml`, `.github/workflows/security.yml`,
  `.github/workflows/coverage.yml`, and `.github/workflows/stability.yml`
  now run the corrected compile, format, audit, and protobuf setup paths.

### QF-2026-06-08-03

- Date: 2026-06-08
- Bucket: B. Should fix soon (closed Done same session)
- Area: CI/CD reliability / delivery system
- Discovered during: customer report
- Evidence: GitHub Actions run `27122130033` failed both `doctor`
  jobs before running checks. `gh run view 27122130033 --log-failed`
  reported `error: backtick could not be run because just could not find
  the shell: No such file or directory (os error 2)` at `Justfile:340`.
  The root `Justfile:1` configured `set shell := ["zsh", "-cu"]`, while
  the GitHub Ubuntu runner provides `bash` but not `zsh`. Local
  `just doctor` passed before the fix because the developer machine has
  `zsh`, making this a CI-only portability break. The follow-up GitHub run
  `27122829410` proved that the shell portability fix worked (`quality-doctor`
  and `project-doctor` both ran), then exposed a second root-only gate bug:
  `release-train-sync` required all 9 nested release-train directories even
  though the `quality-doctor` job intentionally checks out only the root repo.
- Impact: CI failed as the normal path for any push that evaluated
  `release_order`, even when the underlying quality checks were valid.
- Risk if ignored: Operators learn to treat GitHub red as expected noise,
  reducing trust in the factory gate and making real regressions easier
  to miss.
- Effort: S
- Owner: Codex
- Status: Done
- Next action: Backfill closing commit SHA after commit.
- Verifies via: `just doctor`; `just release-train-sync`; `just
  --evaluate release_order`; `env PATH=/usr/bin:/bin /opt/homebrew/bin/just
  --evaluate release_order` proves the parse-time backtick works with only
  system `bash` on PATH; running `just release-train-sync` in a temporary
  directory containing only `Justfile` and `release-train.yaml` passes and
  mirrors the root-only CI checkout.
- Codex-safe now: Yes
- Properties: RP-POLICY-FRESH, RP-RELEASE-TRAIN-INTEGRITY
- Confidence: H
- Business leverage: Restores signal quality on the required root doctor
  workflow; one red push no longer requires manual GitHub log archaeology.
- Last reviewed: 2026-06-08 (same-session fix)
- Cycles open: 0
- History:
  - 2026-06-08: Opened and closed (user-reported repeated GitHub CI/CD
    failures; root Justfile shell portability defect found in run
    `27122130033` and fixed by switching the default shell from `zsh` to
    `bash`).
  - 2026-06-08: Follow-up GitHub run `27122829410` showed the shell fix worked
    and exposed the next false-red: `release-train-sync` checked for nested
    member directories in the root-only `quality-doctor` job. Fixed by making
    `release-train-sync` a YAML-shape check and leaving directory-existence
    validation in full-checkout `project-doctor`.
- Linked PRs / commits: 7dd9712, a273a55
- Standard promoted: n/a
- Drift check: `.github/workflows/doctor.yml` runs `just quality-doctor`
  and `just project-doctor`; the parse-time shell failure now executes under
  GitHub-available `bash`.

### QF-2026-06-08-10

- Date: 2026-06-08
- Bucket: C. Strategic improvement (closed Done 2026-06-08 same
  session; cross-train migration handed off to `QF-2026-06-08-11`)
- Area: CI/CD reliability / delivery system / platform leverage
- Discovered during: CI/CD inventory (`KB/06-operations/ci-cd-inventory.md`)
- Evidence: The 16-repo inventory documented four classes of drift
  (gap classes B/C/D/E) that broke local-CI parity. The strategic
  fix was a canonical `just ci` recipe + canonical workflow shape +
  `RP-CI-PARITY` property declaration.
- Impact: Same as the inventory's framing — "verify locally, know it
  goes through" was broken; push → CI failure cycle was the
  operator's daily reality.
- Risk if ignored: Operator confidence in CI erodes; every PR
  becomes a guessing game.
- Effort: Considered L (full migration); resolved as M for
  declaration + pilot; cross-train migration handed off as L.
- Owner: Codex
- Status: Done (declaration + standard + first pilot)
- Resolution: Shipped 2026-06-08 (commit `dd230da`) in three pieces:
  1. **`RP-CI-PARITY` declared** in
     `KB/05-engineering/standards/recurring-properties.json` (17th
     RP). Status: `Aspired (standard documented + first pilot landed;
     cross-train migration pending)`. Tracked by `QF-2026-06-08-11`.
  2. **Standard doc** `KB/05-engineering/standards/ci-parity.md`
     documents:
     - The canonical Justfile shape: `just ci` orchestrating
       `fmt-check → check → lint → test → security-audit` in CI
       order. `--workspace --all-targets` discipline universal.
       `.audit-ignores` as the single per-workspace ignore source.
     - The canonical workflow shape: single `ci` job, one `run: just
       ci` step, standardised action versions (`actions/checkout@v6`,
       `Swatinem/rust-cache@v2`, `extractions/setup-just@v3`,
       `taiki-e/install-action@v2` for cargo-audit).
     - Non-negotiable invariants for both surfaces.
     - Migration path (smallest-blast-radius-first ordering).
     - Enforcement plan (reviewer at PR time; future mechanical
       project-doctor check deferred until canonical shape stabilises
       across ≥3 workspace migrations).
  3. **First pilot landed** in `commerce-rails` via PR
     Reflective-Lab/commerce-rails#2 (same commit `QF-2026-06-08-09`
     closes — pilot is its first CI). Smoke-tested: `just ci` runs
     green locally (5 gates pass; 19 tests pass; cargo-audit 0
     advisories on 194 crates). GitHub Actions first run validates
     the shape against real feature code.
  Cross-train migration to the remaining 13 workspaces handed off
  to `QF-2026-06-08-11`.
- Verifies via: `KB/05-engineering/standards/ci-parity.md` exists;
  `RP-CI-PARITY` row in JSON source resolves; commerce-rails PR is
  open with `just ci` validating the canonical shape; `quality-
  doctor` check 8 confirms the RP table integrity holds with the
  new row.
- Codex-safe now: Yes for declaration + standard + this commerce-rails
  pilot; per-workspace migration is Codex-safe with confirmation
  before push per each repo's CLAUDE.md.
- Properties: RP-CI-PARITY (declared; standard documented; first
  pilot landed)
- Confidence: H (canonical shape proven by the pilot; cross-train
  migration is replicable per-workspace work).
- Business leverage: Highest of the Bucket C items today. Directly
  addresses the user's named pain ("CI surprises are corrosive to
  flow"). Every workspace migrated reduces the operator's
  daily-feedback-loop friction.
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 0 days (opened + closed same day, late-session)
- History:
  - 2026-06-08: Opened (CI/CD inventory revealed systemic drift;
    canonical `just ci` is the strategic fix).
  - 2026-06-08: Declared `RP-CI-PARITY`; shipped canonical standard
    doc; landed first pilot in commerce-rails (PR Reflective-Lab/
    commerce-rails#2). Cross-train migration handed off to
    `QF-2026-06-08-11`.

### QF-2026-06-02-03

- Date: 2026-06-02
- Bucket: C. Strategic improvement (closed Done 2026-06-08, answered
  by the 16-repo CI/CD inventory; per-gap follow-ups filed separately)
- Area: Security
- Evidence: The root repo has no dependency manifests of its own;
  security posture was decentralised into nested repos with no
  consolidated picture of what coverage existed where.
- Impact: Security coverage could be uneven across the release train;
  a single neglected repo could be the supply-chain entry point.
- Risk if ignored: Dependency, secret, or supply-chain gaps hide in
  less frequently touched repos.
- Effort: M
- Owner: Codex
- Status: Done
- Resolution: Shipped 2026-06-08 (commit `60f8941`)
  as `KB/06-operations/ci-cd-inventory.md` — a 16-repo inventory of
  workflows, Justfile recipes, dependabot configs, and `deny.toml`
  presence. Combined with `QF-2026-06-02-11` (hook inventory) into
  one document because both questions resolve from the same data.
  Specific security findings from the inventory:
  - **5 repos** lack `deny.toml` (helms, mosaic container, soter-smt,
    arena-tests, commerce-rails, runtime-runway).
  - **8 repos** lack `dependabot.yml` (helms, mosaic container,
    crucible-models, embassy-ports, manifold-adapters, soter-smt,
    arena-tests, commerce-rails).
  - **3 repos** have no CI at all (helms, arena-tests, commerce-rails)
    — tracked separately as B-tier findings `QF-2026-06-08-07`,
    `-08`, `-09`.
  - `cargo-audit` `--ignore` lists drift across repos with no
    consolidated tracking — flagged in the inventory but not yet a
    finding; will surface during the canonical-shape work
    (`QF-2026-06-08-10`).
- Verifies via: `KB/06-operations/ci-cd-inventory.md` exists; covers
  all 16 train repos + container; gap classes A-E enumerated; per-gap
  follow-ups filed.
- Codex-safe now: Yes
- Confidence: H
- Business leverage: The inventory is the precondition for the
  canonical `just ci` work that solves "verify locally, know it goes
  through" — the operator's daily pain.
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 6 days (2026-06-02 → 2026-06-08)
- History:
  - 2026-06-02: Opened (audit; Cycle 1)
  - 2026-06-08: Closed (Done). 16-repo inventory shipped; 4 follow-up
    findings filed (`-08-07/-08/-09` per-repo CI gaps; `-08-10`
    canonical-shape strategic).

### QF-2026-06-02-11

- Date: 2026-06-02
- Bucket: B. Should fix soon (closed Done 2026-06-08, answered by
  the same 16-repo CI/CD inventory as `QF-2026-06-02-03`)
- Area: CI/CD / developer experience
- Discovered during: paired session (organism release pre-commit hook)
- Evidence: Pre-commit hooks were uneven across train repos —
  organism invoked `just lint` in pre-commit while siblings did not
  uniformly install the same hook. Same lint could be a release
  blocker in one repo and silent in the next.
- Impact: Quality gates uneven across the train.
- Risk if ignored: Quality regressed to the lowest-gated repo.
- Effort: M
- Owner: Codex
- Status: Done
- Resolution: Shipped 2026-06-08 (commit `60f8941`)
  as `KB/06-operations/ci-cd-inventory.md`. The inventory reveals
  that the deeper problem isn't just pre-commit hook drift — it's
  recipe-body drift between `just lint` / `just check` / `just test`
  across repos. Even if every repo had the same pre-commit hook
  invoking `just lint`, the lints themselves do different things
  (foundation uses `cargo clippy --all-targets`; mosaic uses
  `--workspace --all-targets`; commerce-rails has no `--workspace`).
  Standardising the hook without standardising the recipe wouldn't
  fix the parity problem. Hook installation is therefore deferred
  in favour of the canonical `just ci` work (`QF-2026-06-08-10`).
  Once `just ci` is canonical, a single shared pre-commit hook that
  calls `just ci` becomes trivially deployable across the train.
- Verifies via: `KB/06-operations/ci-cd-inventory.md` documents the
  recipe-body drift in detail; `QF-2026-06-08-10` tracks the
  canonical-shape work that makes pre-commit hook standardisation
  possible.
- Codex-safe now: Yes
- Confidence: H
- Business leverage: Same as `QF-2026-06-02-03` — the inventory
  enables the canonical work that solves the operator's daily pain.
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 6 days (2026-06-02 → 2026-06-08)
- History:
  - 2026-06-02: Opened (paired session; Cycle 1)
  - 2026-06-08: Closed (Done). The pre-commit hook question is
    subsumed by the canonical `just ci` work tracked by
    `QF-2026-06-08-10`; hook standardisation becomes a trivial
    pre-commit installer once the canonical recipe exists.

### QF-2026-06-08-02

- Date: 2026-06-08
- Bucket: C. Strategic improvement (closed Done 2026-06-08 same
  session; mechanical lint explicitly rejected on cost-vs-leverage,
  no follow-up)
- Area: platform extensibility / architecture
- Discovered during: closure of `QF-2026-06-02-19` (RP-TYPED-CROSS-
  LAYER-SEMANTICS declaration complete)
- Discovered in: Tier 1/2 closure session (2026-06-08); enforcement
  decision shipped in Camp-1 #4 paired call later same day
- Evidence: `QF-2026-06-02-19` shipped the declaration half of
  `RP-TYPED-CROSS-LAYER-SEMANTICS`. The mechanical enforcement half —
  a per-crate lint that flags `&str` / `String` parameters at boundary
  signatures — was framed as residual.
- Impact: Convention without mechanical enforcement relies on reviewer
  attention; the boundary signature smell can slip past PR review
  silently.
- Risk if ignored: Slow erosion of layer typing.
- Effort: Considered M (clippy / `#[boundary]` marker + grep / dylint
  extension); resolved as S (PR template only).
- Owner: Codex after paired call
- Status: Done (mechanical lint rejected; convention-stack expanded)
- Resolution: Shipped 2026-06-08 (commit `139cbb7`).
  Camp-1 #4 paired-call decision: **PR template only; mechanical lint
  explicitly rejected**.
  Rationale documented inline in
  `KB/05-engineering/standards/typed-cross-layer-semantics.md` under
  "Why not a mechanical lint":
  1. **Boundary signatures are rare** (handful per quarter, not per
     day) — mechanical-lint-at-PR-time value scales with frequency.
  2. **The hard problem isn't mechanical** — identifying "which
     functions are boundaries" requires reviewer judgment regardless.
     Either every boundary needs a human-applied marker (humans doing
     the hard part) or heuristics like `pub fn` in `runway-*` will
     misfire on non-boundary internals.
  3. **The standard already exists** — adding mechanical enforcement
     on top would add engineering and false-positive risk without
     much marginal value.
  Three rejected alternatives explicitly named: `#[boundary]` marker
  macro + grep check (~1 hour), full `dylint` extension (~1 day),
  composite GitHub Action (separate repo + tool). All three carry
  engineering and maintenance cost that the low frequency of boundary
  additions does not justify.
  Three reinforcing convention layers ship instead:
  1. **Standard doc** (already shipped under `QF-2026-06-02-19`).
  2. **Boundary Checklist** in
     `KB/04-architecture/runtime-injection-boundaries.md` (already
     existed; cited by the standard).
  3. **PR template** — `.github/PULL_REQUEST_TEMPLATE.md` created
     2026-06-08 with the typed-cross-layer-semantics checklist item,
     shown to every PR author at the moment of opening. The same
     PR template carries items for `RP-TEST-CODE-ATTRIBUTION`,
     `RP-AI-EVIDENCE-CITED`, and `RP-AUTO-BLESS-AUDITED` — making
     the PR template a high-leverage convention surface for several
     RPs simultaneously.
  Honest residual: if a real boundary-typing failure mode is observed
  in the wild (e.g. a `String` boundary parameter that smuggled
  wrong-domain semantics into production), a finding gets filed at
  that incident and the mechanical-lint conversation reopens with a
  concrete case behind it. Today's data does not justify the
  engineering.
- Verifies via: PR template file exists at the conventional GitHub
  path; the typed-boundary checklist item is present; RP table
  reflects "Enforced (convention + standard + PR template)";
  `quality-doctor` check 4 confirms the Tracked-by → `—` rotation
  resolves.
- Codex-safe now: Yes
- Properties: RP-TYPED-CROSS-LAYER-SEMANTICS (Enforced as convention
  + standard + PR template)
- Confidence: H
- Business leverage: PR template hits every author at the right
  moment with minimal author-attention cost; no maintenance burden,
  no false positives. The convention-stack also covers three other
  RPs from the same surface — economies of scope on the high-leverage
  PR template enforcement pattern.
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 0 days (opened + closed same day)
- History:
  - 2026-06-08: Opened (closure follow-up to `QF-2026-06-02-19`).
  - 2026-06-08: Camp-1 #4 paired call — mechanical lint rejected on
    cost-vs-leverage; PR template + standard + Boundary Checklist
    are the enforcement.

### QF-2026-06-08-01

- Date: 2026-06-08
- Bucket: C. Strategic improvement (closed Done 2026-06-08 same
  session; cross-train rollout handed off to `QF-2026-06-08-06`)
- Area: test determinism / reliability
- Discovered during: closure of `QF-2026-06-07-04` (scoping done)
- Discovered in: Tier 1/2 closure session (2026-06-08); enforcement
  shipped in Camp-1 #3 paired call later same day
- Evidence: `QF-2026-06-07-04`'s scoping doc framed five enforcement-
  design questions (wall clock / RNG / HashMap iteration / broad env
  mechanism choice + pilot workspace pick) and explicitly handed off
  the picking to a paired session.
- Impact: Same impact pattern as `QF-2026-06-07-04`. Until the design
  questions were answered, the standard existed but no lint config
  did.
- Risk if ignored: Same as `QF-2026-06-07-04`.
- Effort: M
- Owner: Codex (after paired call)
- Status: Done (pilot landed)
- Resolution: Shipped 2026-06-08 (root commit `4719b1f` + prism-analytics
  commit `22b7008`). Paired-call decisions:
  - **Wall clock**: clippy `disallowed_methods = "deny"` on
    `std::time::SystemTime::now`, `std::time::Instant::now`,
    `chrono::Utc::now`, `chrono::Local::now`.
  - **RNG**: clippy `disallowed_methods = "deny"` on `rand::random`,
    `rand::thread_rng`.
  - **Broad env**: clippy `disallowed_methods = "deny"` on
    `std::env::vars`.
  - **HashMap iteration**: convention only (no lint; AST detection in
    `#[test]` context has high false-positive rate).
  - **Test independence**: convention only.
  - **TempDir paths in assertions**: deferred (overlaps with
    `RP-SNAPSHOT-PORTABLE`; revisit if observed in the wild).
  - **Pilot**: `mosaic-extensions/prism-analytics` (smallest
    `RP-HERMETIC-UNIT` baseline; cleanest place to validate the deny
    list shape).
  Pilot deliverables:
  - `mosaic-extensions/prism-analytics/clippy.toml` extended with 7
    determinism deny entries (existing 4 `reqwest::*` entries unchanged).
  - 5 callsites in `crates/prism/src/engine.rs` test code annotated
    with `#[allow(clippy::disallowed_methods)]` + per-function
    justification: 1 `SystemTime::now()` for tempdir uniqueness; 4
    `Instant::now()` for benchmark timing in `#[ignore]`'d benchmarks.
  - `cargo clippy --workspace --all-targets` green.
  - `KB/05-engineering/standards/determinism.md` updated with the
    per-axis enforcement table, production code patterns
    (`#[allow]` shape), test code patterns (DI for clock + RNG), and
    the pilot result.
  Cross-train rollout (remaining 6 workspaces) handed off to
  `QF-2026-06-08-06`.
- Verifies via: `cargo clippy --workspace --all-targets` in
  `mosaic-extensions/prism-analytics` reports 0 `disallowed_methods`
  errors with the deny list active and the 5 `#[allow]`-annotated
  callsites. RP table reflects the pilot landing.
- Codex-safe now: Yes (pilot is bounded; cross-train rollout is
  Codex-safe per workspace with confirmation before push per each
  `CLAUDE.md`).
- Properties: RP-DETERMINISM (pilot landed; remains Aspired until
  cross-train rollout completes via `QF-2026-06-08-06`)
- Confidence: H
- Business leverage: Same playbook as `RP-HERMETIC-UNIT`. The deny
  list shape validates against a real codebase; per-callsite `#[allow]`
  annotations document the production paths that read the clock.
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 0 days (opened + closed same day)
- History:
  - 2026-06-08: Opened (closure follow-up to `QF-2026-06-07-04` —
    enforcement design needs a paired call).
  - 2026-06-08: Camp-1 #3 paired call — picked clippy `disallowed_
    methods` for wall clock + RNG + broad env, convention for HashMap
    iteration + test independence, pilot in prism-analytics. Closed
    same day.

### QF-2026-06-07-03

- Date: 2026-06-07
- Bucket: C. Strategic improvement (closed Done 2026-06-08, audit-only
  path shipped; macro / cap-std / LD_PRELOAD alternatives explicitly
  rejected as oversized for the once-a-year miss profile)
- Area: test hermeticity / tooling
- Discovered during: closure of `QF-2026-06-02-05`
- Discovered in: Final-4-workspace RP-HERMETIC-UNIT push
- Evidence: `QF-2026-06-02-05`'s original verifies-via was "A test-
  harness assertion (custom panic hook or `#[test]` wrapper) that fails
  any unit test which opens a TCP socket." The shipped enforcement
  (`disallowed_methods` clippy lint at deny across all 6 train
  workspaces) catches HTTP-client *construction*. A test that opens
  `std::net::TcpStream` or `tokio::net::TcpStream` directly would
  bypass the lint.
- Impact: Two layers of defence are stronger than one; the lint is
  static and structural, the runtime audit catches dynamic bypasses
  (e.g. test code that reaches for raw `std::net::TcpStream` to "just
  test this one thing").
- Risk if ignored: Low — the lint is the primary gate; `#[allow]`
  annotations are reviewable at PR time. But the dynamic-bypass class
  exists and would slip silently until a downstream consumer reported
  test flakiness.
- Effort: M (the chosen audit-only path is bounded; macro / cap-std /
  LD_PRELOAD alternatives would each have been L)
- Owner: Codex
- Status: Done
- Resolution: Shipped 2026-06-08 (commit `f0ae9b6`).
  Paired-call decision: **audit-only**. Three alternatives explicitly
  rejected with rationale documented in
  `KB/05-engineering/standards/hermetic-unit-tests.md` "Runtime audit
  layer" section:
  1. `#[hermetic_test]` proc-macro running tests in `unshare -n`
     subprocess — high engineering cost (proc-macro maintenance), macOS
     gaps for dev experience.
  2. `cap-std` adoption — full capability-passing refactor across
     production code; very high cost for low-frequency miss.
  3. `LD_PRELOAD` socket interceptor — custom binary + Linux-only.
  Audit-only ships as `.github/workflows/hermetic-audit.yml`. Per
  schedule (Monday 06:30 UTC, 30min after fresh-clone) + on demand,
  the workflow:
  - Checks out all train-relevant repos (same set as fresh-clone).
  - Installs the pinned Rust 1.96.0 toolchain.
  - Pre-fetches dependencies (network required for cache warmup).
  - Runs `unshare -rn cargo test --workspace --all-targets --offline`
    per the 7 audit-scope workspaces (converge, axiom, organism, helms,
    prism-analytics, runtime-runway, commerce-rails). Kernel-level
    network namespace has no connectivity beyond loopback; any test
    that opens a TCP socket gets `EADDRNOTAVAIL`/`ENETUNREACH` and fails.
  - Aggregates per-workspace results; exits non-zero if any failed.
  Standard documented; RP-HERMETIC-UNIT JSON updated to "Enforced
  (construction-layer clippy lint + runtime no-network namespace
  audit)"; Tracked-by → `—` (no further follow-up needed for the
  defence-in-depth layer; if a real dynamic bypass is ever observed,
  the macro path gets a real case behind it then).
- Verifies via: `.github/workflows/hermetic-audit.yml` runs green on
  first scheduled or `workflow_dispatch` execution. A synthetic test
  that opens `tokio::net::TcpStream::connect("1.1.1.1:80")` would fail
  the workflow (kernel `EADDRNOTAVAIL` in the namespace).
- Codex-safe now: Yes
- Properties: RP-HERMETIC-UNIT (defence-in-depth layer added; property
  is now Enforced at both construction and runtime layers)
- Confidence: M (first live CI run will confirm `unshare -rn` works on
  GitHub Actions ubuntu-latest; if not, fallback to bubblewrap or
  Linux subprocess is straightforward)
- Business leverage: Closes the residual surface from `QF-2026-06-02-05`
  with the smallest viable engineering investment. The heavier paths
  (macro, cap-std) remain available if the audit-only signal ever
  surfaces a real bypass.
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 1 day (2026-06-07 → 2026-06-08)
- History:
  - 2026-06-07: Opened (closure follow-up to `QF-2026-06-02-05`)
  - 2026-06-08: Paired-call decision — audit-only. Workflow + standard
    doc + RP update shipped. Macro / cap-std / LD_PRELOAD alternatives
    explicitly rejected with rationale in the standard.

### QF-2026-06-07-01

- Date: 2026-06-07
- Bucket: C. Strategic improvement (closed Done 2026-06-08; per-train-
  repo rollout handed off to `QF-2026-06-08-05`)
- Area: AI-factory discipline / tooling
- Discovered during: PR review (Tier 1) — closure of `QF-2026-06-02-07`
- Discovered in: A-tier autonomous work session (2026-06-07)
- Evidence: `QF-2026-06-02-07`'s convention half landed in `AGENTS.md >
  Test/code attribution` on 2026-06-07. The hook half — original
  verifies-via "A git pre-commit hook that flags simultaneous src+test
  diffs and prompts for the classification line" — was residual.
  Reviewer-enforcement only; an AI agent's rushed PR could ship a
  paired src+test edit without classification undetected.
- Impact: Convention without enforcement relied on reviewer attention;
  rushed or distracted review could let an unclassified simultaneous
  src+test commit land. The `QF-07` incident pattern (AI agent rewrites
  production to satisfy a stale test) could re-emerge.
- Risk if ignored: Slow design erosion via test-driven code mutation.
- Effort: M
- Owner: Codex
- Status: Done
- Resolution: Shipped 2026-06-08 (commit `2d1b489`).
  Three deliverables:
  1. **`scripts/check-test-code-attribution.sh`** — bash detector.
     Implements heuristics **A** (`src/X.rs` ↔ `tests/X.rs` /
     `tests/test_X.rs`, flat) and **B** (`src/<path>/mod.rs` ↔
     `src/<path>/tests.rs`, module folder). Heuristics C
     (in-source `#[cfg(test)] mod tests`) and D (doc-tests) explicitly
     deferred — both need diff-hunk AST awareness; reviewer-enforced.
  2. **`.github/workflows/test-code-attribution.yml`** — PR-time
     GitHub Action. Triggers on `opened`, `synchronize`, `reopened`,
     `edited` (so adding the classification to PR body after the
     fact rechecks). Uses safe `env:`-block pattern for the
     attacker-controlled `PR_BODY` input (no shell interpolation).
     Allows `[skip-attribution]` token in PR body for legitimate
     non-attribution cases.
  3. **`AGENTS.md > Test/code attribution`** — updated to name the
     mechanical detector + heuristics + bypass token + rollout
     scope.
  Smoke-tested in a `/tmp` sandbox with three cases: pair-without-
  classification → exit 1 + helpful message; pair-with-classification
  → exit 0; pair-with-bypass-token → exit 0. Both heuristics fire
  independently.
  Pilot CI-only (per the paired-call decision) rather than git hooks
  + CI per-repo — the per-train-repo rollout (workflow currently in
  root only) handed off to `QF-2026-06-08-05`.
- Verifies via: `scripts/check-test-code-attribution.sh` with synthetic
  paired changes exits non-zero without classification, exits zero
  with a `^(Contract update|Fixture refresh|Real bug fix):` line in
  the PR body or any commit message body, exits zero with
  `[skip-attribution]` token in PR body.
- Codex-safe now: Yes
- Properties: RP-TEST-CODE-ATTRIBUTION (mechanically enforced for root
  repo; per-train-repo rollout pending under `QF-2026-06-08-05`)
- Confidence: H
- Business leverage: One `QF-07`-pattern incident per quarter caught
  at PR-open time instead of yank time saves ~1 day of reverts +
  downstream coordination. The pattern (file-pair heuristic +
  classification regex + bypass token) is replicable for other
  AI-discipline RPs.
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 1 day (2026-06-07 → 2026-06-08)
- History:
  - 2026-06-07: Opened (PR review Tier 1; closure follow-up to
    `QF-2026-06-02-07`)
  - 2026-06-08: Paired-call decision — heuristics A+B, CI-only,
    root-repo pilot. Detector + workflow + AGENTS.md note shipped.
    Per-train-repo rollout handed off to `QF-2026-06-08-05`.

### QF-2026-06-02-19

- Date: 2026-06-02
- Bucket: C. Strategic improvement (closed Done 2026-06-08, declaration
  complete; per-crate lint pilot handed off to `QF-2026-06-08-02`)
- Area: platform extensibility / architecture
- Discovered during: PR review (Tier 0)
- Discovered in: PR Quality Gate Cycle 1 (uncommitted KB doc updates)
- Evidence: `KB/04-architecture/runtime-injection-boundaries.md` Boundary
  Checklist question 6 stated "Is a string carrying semantics that should
  be a closed set, bounded number, typed actor, typed source, typed route
  owner, typed entitlement, or typed event? Stop and add the type before
  wiring the boundary." This was a candidate Recurring System Property
  with no declared row in the RP-* table and no companion standard doc.
- Impact: Without a declared property + standard, every PR that smuggles
  semantics through a string-typed parameter slipped by review on the
  reviewer's memory alone.
- Risk if ignored: Cross-layer type erosion across the workspace; reverts
  the layering work the diagram exists to clarify.
- Effort: M (property declaration); L (per-crate lint implementation)
- Owner: Codex
- Status: Done (declaration complete)
- Resolution: Shipped 2026-06-08 (commit `c9d5c56`).
  Three pieces:
  1. **RP declaration** — new `RP-TYPED-CROSS-LAYER-SEMANTICS` row in
     the RP-* table (added via `recurring-properties.json` —
     `QF-2026-06-02-18`'s mechanization makes this one JSON edit).
     Status: `Aspired (convention + standard documented; mechanical
     lint pending)`. Tracked by `QF-2026-06-08-02`.
  2. **Standard doc** — `KB/05-engineering/standards/
     typed-cross-layer-semantics.md` documents the rule with concrete
     examples (route owner, entitlement, event kind), names the
     boundary surfaces in this codebase (Runway, Commerce Rails, Helm,
     Converge, Mosaic), and frames what the rule does NOT catch
     (genuinely-free-form strings, intra-layer types, fixtures).
  3. **Lint pilot follow-up** — `QF-2026-06-08-02` filed for the
     per-crate lint engineering. The pilot workspace pick
     (`runtime-runway`) and the mechanism choice (clippy extension vs.
     `dylint` vs. `xtask`) are the design questions handed off.
  The 15 → 16 RP count bump is the headline visible change.
- Verifies via: `RP-TYPED-CROSS-LAYER-SEMANTICS` row appears in the
  generated RP-* table; standard doc exists at the cited path; new
  finding `QF-2026-06-08-02` is open and is referenced by both the new
  RP row and the new standard.
- Codex-safe now: Yes (declaration done); design call needed before the
  lint pilot ships.
- Properties: RP-TYPED-CROSS-LAYER-SEMANTICS (declared)
- Confidence: H
- Business leverage: Same as `QF-2026-06-07-04`'s scoping close: a
  written standard makes the design call concrete instead of abstract.
  PR reviewers can now cite a single page rather than reconstructing
  the Boundary Checklist's intent each time.
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 6 days (2026-06-02 → 2026-06-08)
- History:
  - 2026-06-02: Opened (PR review Tier 0; PR Gate Cycle 1 on
    uncommitted KB updates)
  - 2026-06-08: Closed (Done, declaration). RP row + standard doc
    shipped; lint pilot handed off to `QF-2026-06-08-02`.

### QF-2026-06-07-04

- Date: 2026-06-07
- Bucket: C. Strategic improvement (closed Done 2026-06-08, scoping
  complete; enforcement design handed off to `QF-2026-06-08-01`)
- Area: test hermeticity / reliability / AI-factory discipline
- Discovered during: paired session (RP convention-flip — `RP-AUTO-BLESS-
  AUDITED` and `RP-AI-EVIDENCE-CITED` going `Aspired` → `Enforced`)
- Discovered in: 2026-06-07 root-repo convention-flip work
- Evidence: `RP-DETERMINISM` was the only `Aspired` property without a
  finding tracker. The property covers "Test outputs do not depend on
  dev-machine env, network, wall clock, or absolute filesystem
  location." Three of those four axes already had partial enforcement
  elsewhere; the wall-clock axis plus broader determinism axes
  (HashMap iteration order, unseeded RNG, broad env reads, TempDir path
  leaks) were uncovered.
- Impact: Tests that pass today and fail next month erode trust in the
  suite the same way pre-`QF-05` network-dependent tests did.
- Risk if ignored: Flaky tests get blessed away rather than diagnosed;
  the `RP-HERMETIC-UNIT` discipline erodes as the looser sibling axes
  go unenforced.
- Effort: M for scoping + design call; L for full enforcement across
  all 6 train workspaces.
- Owner: Codex (scoping); paired call needed for enforcement design.
- Status: Done (scoping complete)
- Resolution: `KB/05-engineering/standards/determinism.md` shipped
  2026-06-08 (commit `c9d5c56`). The doc enumerates
  the residual determinism axes against what's already covered by
  `RP-HERMETIC-UNIT` (network, partial env) and `RP-SNAPSHOT-PORTABLE`
  (absolute paths in fixtures), then frames three enforcement
  strategies — clippy `disallowed_methods`, AST-walking `xtask`,
  convention-only — with their respective trade-offs (clippy noisy in
  production code that legitimately reads the wall clock; xtask
  bespoke; convention not mechanical). Picking the specific mix is
  handed off to `QF-2026-06-08-01` for a paired session. The scoping
  itself — naming the axes, citing what's covered, framing the choice
  — IS the deliverable this finding tracked.
- Verifies via: Doc exists; `RP-DETERMINISM` in the RP-* table now has
  `QF-2026-06-08-01` (not `-07-04`) as the tracker. `quality-doctor`
  check 4 confirms the Tracked-by reference resolves.
- Codex-safe now: Yes for the scoping (done); no for the enforcement
  design (handed off).
- Properties: RP-DETERMINISM (still Aspired; tracker rotated to
  `QF-2026-06-08-01`)
- Confidence: H
- Business leverage: The scoping doc gives the future design call a
  tight agenda — five concrete questions, each with a candidate
  answer. The design call becomes "pick from menu" rather than "start
  from scratch."
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 1 day (2026-06-07 → 2026-06-08)
- History:
  - 2026-06-07: Opened (convention-flip session: `RP-DETERMINISM`
    identified as the only `Aspired` property with no finding tracker)
  - 2026-06-08: Closed (Done, scoping). `determinism.md` shipped.
    Enforcement design handed off to `QF-2026-06-08-01`.

### QF-2026-06-02-18

- Date: 2026-06-02
- Bucket: C. Strategic improvement (closed Done 2026-06-08)
- Area: platform extensibility / maintainability
- Discovered during: PR review (Tier 1)
- Discovered in: PR Quality Gate Cycle 1 (commit `d4e3cf3`)
- Evidence: `QUALITY_BACKLOG.md` Recurring System Properties table had 15 rows
  with `Status` and `Tracked by` columns. Both columns were hand-maintained
  Markdown; no automation linked a property row to actual enforcement state
  or to its tracking findings. When a finding closed Done, the `Tracked by`
  column had to be updated by hand; when an enforcement artifact landed,
  `Status` had to be promoted manually. Drift between policy claim and code
  reality could grow silently.
- Impact: Drift between policy claim and code reality could grow silently.
- Risk if ignored: A property could read `Aspired` forever after enforcement
  landed, or be marked `Enforced` without a real drift check.
- Effort: M
- Owner: Codex
- Status: Done
- Resolution: Shipped 2026-06-08 (commit `c9d5c56`).
  Pattern: JSON source-of-truth + Python generator + region markers in
  QUALITY_BACKLOG.md + drift-detection check.
  - `KB/05-engineering/standards/recurring-properties.json` is the source.
    Each entry has `{id, property, enforcement, status, tracked_by}` fields
    (the finding's original spec named YAML; switched to JSON because
    PyYAML is not in the toolchain and JSON parses via stdlib).
  - `scripts/rp-table.py` emits the Markdown table from the JSON.
  - `scripts/rp-table-check.py` is the drift detector (used by check 8).
  - `just rp-table` prints the generated table; `just rp-table-sync`
    writes it between the `<!-- BEGIN GENERATED RP-TABLE -->` /
    `<!-- END GENERATED RP-TABLE -->` markers in QUALITY_BACKLOG.md.
  - `just quality-doctor` check 8 fails CI if the in-file region drifts
    from the generated table (forces operators to edit the JSON and run
    `just rp-table-sync`, never the Markdown directly).
  Two pre-bake updates landed in the same commit:
  RP-DETERMINISM `Tracked-by` updated to `QF-2026-06-07-04` (was `—`,
  stale since I filed `-07-04` 2026-06-07); RP-RELEASE-TRAIN-INTEGRITY
  Status + Enforcement updated to reflect the single-source state from
  `QF-2026-06-06-02`.
  Residual: the finding's spec also called for "Status transition
  `Aspired → Enforced` require the same commit to cite the enforcement
  artifact." Today's mechanization makes status-flip drift VISIBLE
  (it'll show in the JSON diff against the in-file table) but doesn't
  MECHANICALLY enforce artifact-citation discipline. That's reviewer-
  enforced today; mechanical enforcement is a future refinement.
- Verifies via: `just quality-doctor` check 8 reports
  "✓ RP-* table in QUALITY_BACKLOG.md matches JSON source"; any
  direct edit to the Markdown table region without a corresponding
  JSON edit fails the check.
- Codex-safe now: Yes
- Properties: structural enforcement of RP-* table integrity
- Confidence: H
- Business leverage: Editing one place (JSON) instead of two (JSON +
  Markdown) prevents drift; the structural check catches accidental
  Markdown edits. Replicable pattern — same shape as `QF-2026-06-06-02`'s
  single-source YAML refactor (release-train.yaml).
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 6 days (2026-06-02 → 2026-06-08)
- History:
  - 2026-06-02: Opened (PR review Tier 1; PR Gate Cycle 1 on `d4e3cf3`)
  - 2026-06-08: Closed (Done). JSON source + Python generator + region
    markers + quality-doctor check 8. Pattern documented in the
    resolution; replicable for future structured tables.
- Linked PRs / commits: d4e3cf3 (origin), `c9d5c56` (fix)

### QF-2026-06-06-02

- Date: 2026-06-06
- Bucket: C. Strategic improvement (closed Done 2026-06-08)
- Area: release engineering / maintainability
- Discovered during: strategic audit (closure of `QF-2026-06-02-23`)
- Discovered in: 2026-06-06 audit of stale-looking findings
- Evidence: `QF-2026-06-02-23`'s strict next-action ("Justfile parses [the
  manifest] once. … CI test asserts no hard-coded list remains in
  `Justfile`") was not satisfied by what shipped. `release-train.yaml`
  existed with schema `{name, dir}` but the `Justfile` still hardcoded
  both `release_order` (a Just variable literal) and the `_release-dir
  name` case statement. Adding a new train member required 3 coordinated
  edits (yaml + Justfile `release_order` + Justfile `_release-dir` case);
  the sync half of `project-doctor` check 1 + `release-train-sync` was
  doing real work but the duplication itself remained.
- Impact: Two sources of truth confused contributors; every new train
  member required the 3-edit ritual; future train-related logic kept
  hand-coding lists that drifted from the yaml.
- Risk if ignored: The duplication grew more expensive as the yaml
  schema extended; hand-coded lists kept proliferating.
- Effort: M
- Owner: Codex
- Status: Done
- Resolution: Shipped 2026-06-08 (commit `c9d5c56`).
  `Justfile` `release_order` is now a backtick-evaluated variable that
  awk-parses `release-train.yaml`'s `projects:` list; `_release-dir
  name` recipe awk-parses both `projects:` and the new `aliases:` map
  for the mosaic sub-names. `release-train.yaml` extended with an
  `aliases:` section documenting the 8 mosaic sub-aliases (arbiter,
  crucible, embassy, ferrox, manifold, mnemos, prism, soter) that the
  previous `_release-dir` case statement had hand-coded. The
  sync-against-Justfile half of `project-doctor` check 1 + the
  `release-train-sync` recipe is gone; both now validate parseability +
  member-dir existence only. The `release-train-sync` recipe name is
  preserved because `.github/workflows/doctor.yml` references it.
  Smoke-tested: `just release-train-sync` green, `just release-preflight
  <name>` works end-to-end across `axiom`, all 17 names (9 projects + 8
  aliases) resolve correctly via `just _release-dir <name>`, an
  `UNKNOWN` name returns the `UNKNOWN-PROJECT-*` sentinel.
- Verifies via: `grep -E '^release_order :=' Justfile` shows the
  definition is a backtick (not a literal string); `grep -E
  '^_release-dir' Justfile` shows the recipe reads from yaml; every
  train iteration in `Justfile` (still using `{{release_order}}`) traces
  back to a single yaml parse via the backtick variable; `project-doctor`
  check 1 reports "release-train.yaml parseable + every member directory
  exists" without the sync-half lines.
- Codex-safe now: Yes
- Properties: RP-RELEASE-TRAIN-INTEGRITY (still Enforced; mechanism
  evolved from "two sources kept in sync by checks" to "one source the
  checks validate intrinsically")
- Confidence: H
- Business leverage: Adding a new train member is now a single yaml
  edit. The pattern (awk-parse YAML inside Just) is replicable for
  other meta-data tables — most immediately the RP-* table tracked by
  `QF-2026-06-02-18`.
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 2 days (2026-06-06 → 2026-06-08)
- History:
  - 2026-06-06: Opened (strategic audit; closure follow-up to
    `QF-2026-06-02-23`)
  - 2026-06-08: Closed (Done). `Justfile` reads `release-train.yaml`
    via awk for both `release_order` and `_release-dir`. The
    sync-against-Justfile checks are gone — the single source can't
    diverge from itself.

### QF-2026-06-02-09

- Date: 2026-06-02
- Bucket: B. Should fix soon (closed Done 2026-06-08)
- Area: release engineering
- Discovered during: release rehearsal (`runway-storage-contract` upload)
- Evidence: `cargo publish -p runway-storage-contract` failed with
  HTTP 413 "Payload Too Large; max upload size is 10485760" (10 MiB). The
  crate was never published. Cause is unaudited package contents —
  likely test fixtures or generated assets that should be excluded via
  `package.exclude` or `package.include`.
- Impact: Publishable crates can grow silently past the registry limit;
  release halts; humans diagnose by hand.
- Risk if ignored: Larger crates grow past the limit one by one; releases
  become slower and partial.
- Effort: S
- Owner: Codex
- Status: Done
- Resolution: New `just release-package-size-check <name>` recipe shipped
  2026-06-08 (commit `c926ac0`). For every publishable
  crate in the named workspace, the recipe runs `cargo package --list
  --allow-dirty` (the same surface `cargo publish` would upload), sums
  the on-disk sizes of the listed files (an upper bound on the gzip-
  compressed archive), and verdicts:
  - ≥ 9 MiB (90 % of the crates.io 10 MiB cap) → hard fail.
  - ≥ 5 MiB (50 %) → soft warning.
  - < 5 MiB → silent pass.
  Wired into `release-preflight` as sub-step 5.6 so the operator gets
  it automatically. Standalone for ad-hoc inspection.
  Smoke-tested: converge (10 publishable, all under 5 MiB), runway
  (9 publishable, all under 5 MiB — note: the 2026-06-02 incident
  workload has since been trimmed), helms (0 publishable — internal
  workbench, correctly skipped), atelier (1 publishable, under 5 MiB).
  Complements the existing `RP-CRATE-SIZE-BUDGET` enforcement: the
  `project-doctor` check 3 is the LEADING indicator (single git-
  tracked file > 1 MiB); this recipe is the TRAILING indicator (the
  actual archive a publish would upload). Both indicators stay green
  together is the goal.
- Verifies via: `just release-package-size-check <name>` reports
  per-crate verdict; `just release-preflight <name>` invokes it as
  step 5.6 and counts hard-failures into the preflight exit code.
- Codex-safe now: Yes
- Properties: RP-CRATE-SIZE-BUDGET (both indicator layers now active)
- Confidence: H
- Business leverage: A pre-publish guard at 90 % of the cap means a
  10 MiB regression is caught in the operator's local feedback loop
  (seconds) instead of mid-publish (HTTP 413 + recovery), and the soft
  warning surfaces growth trends before they become incidents.
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 6 days (2026-06-02 → 2026-06-08)
- History:
  - 2026-06-02: Opened (release rehearsal; Cycle 1)
  - 2026-06-08: Closed (Done). Pre-publish size guard shipped as
    `release-package-size-check` recipe + `release-preflight` step 5.6;
    smoke-tested across 4 train workspaces.

### QF-2026-06-02-25

- Date: 2026-06-02
- Bucket: B. Should fix soon (closed Done 2026-06-08)
- Area: release engineering / correctness
- Discovered during: PR review (Tier 2)
- Discovered in: PR Quality Gate Cycle 1 (Justfile release-train sequence)
- Evidence: `Justfile` `release-preflight` check 7, downstream-consumer
  detection: `hits=$(grep -rln -F "$dir" --include='*.toml' "$other_dir"
  ...)`. Greps the directory name as a fixed-string substring across
  consumer `.toml` files. False-positives on coincidental substring
  matches (a `.toml` that *mentions* the directory name in comments,
  doc keys, or unrelated paths); false-negatives on relative path-deps
  (`path = "../converge"`) where the stored value is not the absolute
  `$dir` the grep is looking for.
- Impact: Operator can miss a real consumer that needs a Cargo.toml bump, or chase a phantom.
- Risk if ignored: Same shape as `QF-2026-06-02-13` (stack-flatten silently broke 19
  downstream path-deps).
- Effort: M
- Owner: Codex
- Status: Done
- Resolution: Refactored `Justfile` `release-preflight` check 7 to walk
  `cargo metadata --no-deps` per consumer workspace and inspect each
  package's `.dependencies[]` for entries with `.source == null` and
  `.path` resolving (absolute) into `$dir/`. The directory is
  absolutized via `pwd` before comparison so prefix matching is exact.
  Shipped 2026-06-08 commit `c926ac0`. Smoke-tested
  against `converge` (found `helms` + `arena` consumers with exact
  `consumer-crate → dep-name` pairs) and `helms` (found `atelier` +
  `arena`); no false positives observed.
- Verifies via: `just release-preflight <name>` lists consumers via
  exact-match resolved path-deps; relative-path forms (`path =
  "../converge"`) are correctly resolved by `cargo metadata` and
  identified.
- Codex-safe now: Yes
- Confidence: H
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 6 days (2026-06-02 → 2026-06-08)
- History:
  - 2026-06-02: Opened (PR review Tier 2; PR Gate Cycle 1 on `f359a14`)
  - 2026-06-08: Closed (Done). `cargo metadata`-based walk replaces the
    substring grep. Same playbook as `QF-2026-06-02-13` closure.
- Linked PRs / commits: f359a14 (origin), `c926ac0` (fix)

### QF-2026-06-02-02

- Date: 2026-06-02
- Bucket: B. Should fix soon (closed Done 2026-06-08)
- Area: Developer experience
- Evidence: The root `Justfile` has broad aggregate gates across many
  independent repos, but this inspection did not find a fast documented
  "minimum factory health" recipe or status snapshot for recurring audits.
- Impact: Auditors may either skip verification or run expensive full-stack
  checks when a narrower health pass would be enough.
- Risk if ignored: Quality reviews become inconsistent, slow, and harder to
  repeat.
- Effort: S
- Owner: Codex
- Status: Done
- Resolution: `KB/06-operations/factory-health.md` shipped 2026-06-08
  (commit `c926ac0`) documenting the
  recurring-audit recipe surface: `just doctor` (= `quality-doctor` +
  `project-doctor`, the 30-second zero-build / zero-network meta-pass),
  `just snapshot` (the read-only counts emitter), and the escalation
  ladder to `just check` / `just test` / `just check-all-fresh`. The
  recipes themselves were built incrementally during the 2026-06-02
  → 2026-06-07 sweep; this finding's "In progress" label tracked the
  documentation gap, now closed.
- Verifies via: KB page exists and is cross-linked from
  `QUALITY_BACKLOG.md` Snapshot context. `just quality-doctor` already
  verifies that the Snapshot block counts match observable state, so
  the recurring-audit loop is self-checking.
- Codex-safe now: Yes
- Confidence: H
- Business leverage: A documented quick-health recipe cuts each review's
  verification cost from full-stack to seconds, materially improving review
  cadence.
- Last reviewed: 2026-06-08 (closure session)
- Cycles open: 6 days (2026-06-02 → 2026-06-08)
- History:
  - 2026-06-02: Opened (audit; Cycle 1)
  - 2026-06-08: Closed (Done). `KB/06-operations/factory-health.md`
    documents the existing recipe surface as the recurring-audit
    entrypoint. The page is the answer; the recipes were already there.

### QF-2026-06-02-12

- Date: 2026-06-02
- Bucket: C. Strategic improvement (closed Done 2026-06-07)
- Area: AI-factory discipline
- Discovered during: paired session (trybuild bless retrospective)
- Evidence: Commit `af9b754` blessed five trybuild `.stderr` snapshots
  in a single bulk operation and the commit message said "trybuild fixture
  refresh for axiom_enforcement (matches new converge 3.9.2 diagnostic
  surface)" — conflating four cosmetic rustc-evolution diffs with one
  load-bearing absolute-path regression. The fact-check happened in a
  separate paired session, not at PR time.
- Impact: AI-driven bulk blesses hide real regressions inside cosmetic
  changes.
- Risk if ignored: Every snapshot-test framework (trybuild, insta,
  goldenfile, …) becomes a vector for unreviewed semantic drift.
- Effort: S
- Owner: Codex
- Status: Done
- Resolution: Convention shipped in `AGENTS.md` "Fixture auto-bless
  classification" section (2026-06-07, commit `e486b8c`).
  Every bless-shape commit must classify each fixture-line change as
  cosmetic (toolchain text drift) or semantic (real behavioral change),
  with semantic lines requiring separate justification. `RP-AUTO-BLESS-
  AUDITED` flipped from `Aspired` to `Enforced (convention in AGENTS.md)`
  in the same commit. Mechanical detector (pre-commit hook / CI shape
  check) is residual; reviewer-enforced at PR time until that ships.
- Verifies via: Convention in `AGENTS.md`; spot-check via `git log -p
  -- '**/*.stderr' '**/*.snap'` plus commit-message inspection.
- Codex-safe now: Yes
- Properties: RP-AUTO-BLESS-AUDITED (Enforced as convention)
- Confidence: H
- Last reviewed: 2026-06-07 (convention-flip session)
- Cycles open: 5 days (2026-06-02 → 2026-06-07)
- History:
  - 2026-06-02: Opened (paired session; Cycle 1)
  - 2026-06-07: Closed (Done). Convention shipped in `AGENTS.md`;
    property flipped to `Enforced`. Mechanical detector remains residual
    (no separate tracker yet — file one if/when the false-negative rate
    of reviewer-only enforcement becomes visible).

### QF-2026-06-02-05

- Date: 2026-06-02
- Bucket: A. Must fix now (closed Done 2026-06-07)
- Area: test hermeticity / security
- Discovered during: incident (axiom guidance test misdiagnosed as "env-dependent")
- Evidence: `bedrock-platform/axiom/src/guidance.rs::guide_heading` (v0.15.1)
  read `ChatBackendSelectionConfig::from_env()` inside `select_backend`, then
  `manifold::select_healthy_chat_backend(...)` probed live LLM providers via
  real network calls. Unit test
  `guide_heading_falls_back_to_local_on_no_backend` asserted
  `source == "local-heuristic"` but actually returned `"live-chat-backend"`
  whenever a developer had `OPENAI_API_KEY` / `ANTHROPIC_API_KEY` / etc. in
  their environment. `cargo test` was silently issuing billable outbound API
  calls.
- Impact: Cost leak (real LLM calls per test run), non-determinism (tests
  pass or fail by env), and security exposure (credentials end up in tokio
  task data during tests).
- Risk if ignored: A `cargo test` run is no longer a safe local operation;
  cloud bills grow per developer; flaky tests get blessed away rather than
  diagnosed.
- Effort: S per touchpoint; cumulative M across the workspace.
- Owner: Codex
- Status: Done
- Resolution: All six release-train workspaces at clippy
  `disallowed_methods = "deny"` for `reqwest::Client::new` /
  `reqwest::blocking::Client::new` / `reqwest::Client::builder` /
  `reqwest::blocking::Client::builder`. Pattern + replication path
  documented in `KB/05-engineering/standards/hermetic-unit-tests.md`.
  Two migration shapes recorded:
  - **Struct-field DI** (organism, converge): `with_http_client(client,
    ...)` constructor next to `new(...)`. Used for long-lived service /
    provider types where tests can construct with a stub client.
  - **Threaded-client / `#[allow]`** (helms, commerce-rails,
    runtime-runway): client passed as a parameter through the call
    chain, or `#[allow(clippy::disallowed_methods)]` with a
    justification comment at the single convenience-default
    construction site. Used for free functions and production
    constructors that talk to real infrastructure (GCP/Firebase/Stripe)
    where tests stub at the emulator level rather than the constructor
    level.
  - The strict verifies-via — a test-harness assertion that fails any
    unit test which opens a TCP socket at runtime — is carved out to
    follow-up `QF-2026-06-07-03`. The lint catches construction; the
    harness catches actual socket opens. Both layers together would be
    the full enforcement; today only the lint layer is shipped.
- Verifies via: `cargo clippy --workspace --all-targets` on each of the
  6 train workspaces reports 0 `disallowed_methods` callsites
  (organism: 7 migrated to DI; helms: 1 threaded; commerce-rails: 1
  `#[allow]`; prism-analytics: 0 baseline; converge: 1 DI;
  runtime-runway: 11 `#[allow]`). Original incident
  (`guide_heading_falls_back_to_local_on_no_backend`) cannot recur —
  axiom v0.15.2 migrated; the test would now refuse to compile if a
  contributor re-introduced an unannotated `reqwest::Client::new()` in
  the test path.
- Codex-safe now: Yes
- Properties: (closed; `RP-HERMETIC-UNIT` is `Enforced`; further test-
  harness hardening tracked by `QF-2026-06-07-03`)
- Confidence: H
- Last reviewed: 2026-06-07 (final-4-workspace push)
- Cycles open: 5 days (2026-06-02 → 2026-06-07)
- History:
  - 2026-06-02: Opened (incident; Cycle 1)
  - 2026-06-02: Moved to In progress — axiom v0.15.2 migrated `guide_heading`
  - 2026-06-07: Skipped in autonomous A-tier work session — sweep touches 8+
    independent gits (organism, helms, runtime-runway, prism-analytics, etc.);
    best done in a paired session where each nested-repo PR can be reviewed.
  - 2026-06-07: **Pilot migration landed in organism**:
    `bedrock-platform/organism` commits `dbf1895` (`MistralOcrProvider` DI
    pattern + hermetic test) and `cc98b71` (clippy `disallowed_methods`
    lint config at warn-level, with `KB/05-engineering/standards/hermetic-
    unit-tests.md` documenting the pattern + per-workspace migration order).
    Cross-train surface mapped: converge=6, axiom=0 (done), organism=7,
    helms=4, runtime-runway=22, commerce-rails=2 (counts: from_env +
    reqwest src callsites). Visible per-callsite TODO list now greppable
    via `cargo clippy --features ocr` in organism; same pattern replicates
    to remaining providers (DeepSeek, LightOn, vision/, web.rs, receipt.rs)
    and to helms/runtime-runway. Property stays `Aspired (in progress)`
    until every workspace's `clippy.toml` is at `deny` and every test that
    exercises a network-touching path constructs via DI.
  - 2026-06-07: **organism-intelligence fully migrated**: commit `3e934dd`
    completes the remaining 7 callsites — `DeepSeekOcrProvider` +
    `LightOnOcrProvider` (cloud.rs), all four vision backends
    (`AnthropicVision`, `OpenAiVision`, `GeminiVision`, `MistralVision`),
    and `OllamaReceiptOcrProvider` (receipt.rs). Workspace lint
    `disallowed_methods` promoted `warn` → **`deny`** at organism root.
    `just lint` green; `cargo clippy -p organism-intelligence
    --all-features` reports 0 disallowed_methods warnings (was 7). Vision
    backends gained a struct-field `client` (also wins connection-pool
    reuse). organism is now the first workspace at deny-level
    enforcement — when helms/runtime-runway/prism/commerce/converge join,
    `RP-HERMETIC-UNIT` flips to `Enforced`.
  - 2026-06-07: **helms migrated**: commit `2057a2c` threads
    `client: &reqwest::blocking::Client` through `send_ollama_request`,
    `query_ollama_with_text`, and `ollama_extract` in
    `prio-expenses/src/receipt_extractor.rs`. The single previously-inline
    `reqwest::blocking::Client::builder().build()` is now the only
    construction site, annotated `#[allow]` with a justification.
    Workspace `clippy.toml` and `[workspace.lints.clippy]
    disallowed_methods = "deny"` added — the disallowed list includes
    both `::new` and `::builder` variants (helms uses builder; organism
    used new). `cargo clippy -p prio-expenses` reports 0
    disallowed_methods callsites. **helms is the second workspace at
    deny-level enforcement.** runtime-runway / prism-analytics /
    commerce-rails / converge remain.
  - 2026-06-07: **Final 4 workspaces at deny — RP-HERMETIC-UNIT
    Enforced**. Commits this turn (4):
    - `commerce-rails` `ccf2b0c`: `CommerceRails::new(client, config)`
      was already DI-ready; the single test helper `rails()` got
      `#[allow]` + a sentinel-client justification. Workspace
      `clippy.toml` + `disallowed_methods = "deny"` added.
    - `mosaic-extensions/prism-analytics` `ee39a20`: zero src
      callsites — workspace `clippy.toml` + lint at deny added as
      keep-clean enforcement for future LLM hooks.
    - `bedrock-platform/converge` `b48d7e2`: `FirebaseValidator`
      (only callsite) migrated to `with_http_client(client, config)`
      DI constructor; `new(config)` is the convenience default with
      `#[allow]`.
    - `runtime-runway` `4da2616`: 11 callsites annotated with
      `#[allow]` + per-site justification across runway-secrets,
      runway-storage (6 GCP/Firestore/Vertex constructors),
      runway-accounts (production fan-out + test helper), runway-auth
      (Firebase), and runway-app-host (integration-test fixture
      against a 127.0.0.1 listener). Pragmatic choice for the runtime
      substrate — tests use emulators at the harness level, not DI
      through the struct.
    Pattern documented in
    `KB/05-engineering/standards/hermetic-unit-tests.md` now records
    both shapes (struct-field DI vs threaded-client / `#[allow]`).
    Original `guide_heading` failure mode cannot recur: the lint
    refuses any new unannotated `reqwest::Client::new()` /
    `::builder()` in any train-workspace crate. Test-harness assertion
    (TCP socket open at runtime) carved out to `QF-2026-06-07-03`.

### QF-2026-06-02-24

- Date: 2026-06-02
- Bucket: B. Should fix soon
- Area: release engineering / supply-chain hygiene
- Discovered during: PR review (Tier 2)
- Discovered in: PR Quality Gate Cycle 1 (Justfile release-train sequence)
- Evidence: `Justfile` had no `release-undo` recipe. If `release-all` succeeded
  partially (commit + tag + push but `cargo publish` failed mid-train), there
  was no shortcut to cleanly `cargo yank` and append to release history.
  `QF-2026-06-02-10` (`RP-YANK-DISCOVERABLE`) documented the discipline; no
  tooling existed for it.
- Impact: Yank-and-replace was oral tradition; provenance lost.
- Risk if ignored: Each yank leaves no paper trail; ghost versions accumulate
  on crates.io; consumers cannot follow successor pointers.
- Effort: M
- Owner: Codex
- Status: Done
- Resolution: Added `release-undo <crate> <version> [reason]` recipe to root
  `Justfile`. The recipe:
  1. Looks up the successor version via the crates.io API
     (`https://crates.io/api/v1/crates/<crate>` → `crate.max_version`).
     Falls back to `<successor-pending>` if offline.
  2. Appends a fully-formed `### <crate> v<version>` block to
     `KB/release-history.md` with the four required fields (`Yanked:`,
     `Yanked by:`, `Reason:`, `Successor:`, `Migration:`) — automatically
     passing `just project-doctor` check 6 structural lint.
  3. Defaults to dry-run. `REL_APPLY=1` actually executes
     `cargo yank --vers <version> <crate>`.
  4. Prints the appended row for review before commit.

  Together with `QF-2026-06-02-10`'s `release-history.md` runbook +
  `release-history-audit` recipe, this closes the WRITE half of
  `RP-YANK-DISCOVERABLE` (record + yank in one command). The reverse-
  discovery half (every yanked version on crates.io has an entry) remains
  out of scope — requires enumerating all crates owned by the org,
  separate finding if ever opened.
- Verifies via: `just release-undo <crate> <version>` exits 0 with the
  appended row visible in `KB/release-history.md`; the row passes
  `just project-doctor` check 6 immediately. `REL_APPLY=1` path
  exercises `cargo yank` against a real crate.
- Codex-safe now: Yes (dry-run by default; live yank gated on REL_APPLY=1)
- Properties: RP-YANK-DISCOVERABLE
- Confidence: H
- Last reviewed: 2026-06-07 (Cycle 1 follow-up)
- Cycles open: 0 (opened 2026-06-02, closed 2026-06-07; same Cycle 1)
- History:
  - 2026-06-02: Opened (PR review Tier 2; PR Gate Cycle 1 on `f359a14`)
  - 2026-06-07: Closed Done (release-undo recipe shipped; Cycle 1)
- Linked PRs / commits: f359a14 (introduced the gap); closing commit
  pending.
- Standard promoted: see `KB/05-engineering/standards/doctor-recipe-pattern.md`
  for the broader recipe-pattern standard. The yank-and-replace runbook
  lives in `KB/release-history.md` itself.

### QF-2026-06-02-13

- Date: 2026-06-02
- Bucket: C. Strategic improvement
- Area: release engineering
- Discovered during: release rehearsal (stack/ flattening fallout)
- Evidence: Earlier in Cycle 1, flattening `stack/bedrock-platform/` and
  `stack/mosaic-extensions/` up one level silently broke 19 path-deps in
  `arena-tests/Cargo.toml` and `atelier-showcase/scenarios/*/Cargo.toml`
  (`../stack/bedrock-platform/...` → broken; should be
  `../bedrock-platform/...`). The breakage was discovered only when those
  workspaces failed to build during release preflight, not at commit
  time — the touching repo's cargo cache had hidden the breakage.
- Impact: Repo-layout refactors landed green in the touching repo but
  silently broke downstream repos that nobody ran `cargo check` on from a
  clean slate.
- Risk if ignored: Workspace renames keep producing time-delayed
  breakage; nobody learns to scan cross-repo Cargo.toml paths during
  layout changes.
- Effort: M
- Owner: Codex
- Status: Done
- Resolution: Added `just check-all-fresh` recipe to root `Justfile`.
  Reads the train workspace list from `release-train.yaml` (falls back to
  a static list if the yaml is absent); for each workspace runs
  `cargo clean` followed by `cargo check --workspace --all-targets` from
  a clean slate; per-workspace timing reported; failures accumulate into
  a `fails` counter (permissive `set -uo pipefail` per `QF-2026-06-02-22`)
  so one broken workspace doesn't hide the rest.
  Documented as operator workflow: run before committing a layout
  change. Catches the 2026-06-02 stack/-flatten incident pattern
  exactly — a clean check across all 9 train workspaces would have
  surfaced the 19 broken path-deps at commit time, not at release
  preflight.
  The scheduled-CI-matrix half of `RP-FRESH-CLONE-GREEN`'s enforcement
  spec (weekly run + the `test` half + per-workspace time budget) is
  carved out to follow-up finding `QF-2026-06-07-02`.
- Verifies via: Original verifies-via — "`just check-all-fresh` exits
  zero immediately after a rename commit, before the commit lands" — is
  the operator's pre-commit check. Recipe parses (`just --list` shows
  it), workspace extraction from `release-train.yaml` returns the
  expected 9 workspaces, and the shell loop structure matches
  `release-deps-audit` and `release-public-api-check` (both previously
  tested end-to-end). Full ~30-60 min cross-train run deferred to
  first operator invocation; sandbox doesn't have the dependency cache
  to run it fast.
- Codex-safe now: Yes
- Properties: (closed; `RP-FRESH-CLONE-GREEN` is `Enforced` (recipe);
  scheduled CI matrix follow-up under `QF-2026-06-07-02`)
- Confidence: H
- Business leverage: One QF-13-pattern layout incident per quarter caught
  at commit time saves ~4 hours of cross-workspace archaeology + the
  "release preflight surfaced this two days later" embarrassment.
- Last reviewed: 2026-06-07 (B-tier follow-up session)
- Cycles open: 5 days (2026-06-02 → 2026-06-07)
- History:
  - 2026-06-02: Opened (release rehearsal; Cycle 1)
  - 2026-06-07: Closed Done — `check-all-fresh` recipe shipped;
    scheduled-CI-matrix residual carved to `QF-2026-06-07-02`.
- Linked PRs / commits: fc6219c (closing)
- Drift check: `just check-all-fresh` (operator-runnable pre-commit
  check). Scheduled-CI verification via `QF-2026-06-07-02`.
- Standard promoted: *(pending — fold into
  `KB/05-engineering/standards/fresh-clone-discipline.md`: every
  workspace-layout change runs `just check-all-fresh` before commit; CI
  exercises the same recipe weekly; new train member additions land
  with their workspace's clean-build time documented in
  `release-train.yaml`.)*

### QF-2026-06-02-10

- Date: 2026-06-02
- Bucket: B. Should fix soon
- Area: release engineering / AI-factory discipline
- Discovered during: paired session (yank-and-replace round)
- Evidence: During the train, `converge-atelier-domain@1.0.1` and
  `axiom-truth@0.15.1` were yanked from crates.io and replaced with
  `1.0.2` / `0.15.2`. The yank commands were executed ad-hoc; no written
  playbook existed for the yank-and-replace sequence (when to yank, what
  to record, how to notify consumers, how to update a release-history
  file).
- Impact: Yanks happened by oral tradition. Future consumers had no
  pointer from a yanked version to its successor.
- Risk if ignored: Yanks proliferate without a paper trail; trust in the
  release process drops; SemVer-driven consumers see ghost versions.
- Effort: S
- Owner: Codex
- Status: Done
- Resolution: Three artifacts shipped:
  1. **`KB/release-history.md`** — append-only ledger of yank events.
     Contains the runbook (six steps from "open a finding" through "mirror
     to per-repo CHANGELOG") and the required-fields schema. The two
     historical yanks (`converge-atelier-domain v1.0.1`, `axiom-truth
     v0.15.1`) are backfilled with reconstructed reasons and successor
     pointers.
  2. **`AGENTS.md > Release yank discipline`** — codifies the rule:
     publish successor first; add release-history entry BEFORE running
     `cargo yank`; mirror to per-repo `CHANGELOG.md` if maintained.
     Cites both check 6 and the audit recipe as the enforcement layer.
  3. **`just project-doctor` check 6** (Codex, shipped earlier) +
     **`just release-history-audit`** (this commit) — two complementary
     drift checks. Check 6 validates structural completeness (every entry
     has the four required fields: `Yanked:`, `Reason:`, `Successor:`,
     `Migration:`). The audit recipe cross-references each entry with the
     crates.io API (`https://crates.io/api/v1/crates/<crate>/<version>`)
     and confirms `yanked: true` — catches stale or fictional entries.
     The recipe degrades gracefully when offline (skips with warning,
     exits 0) so local runs without network aren't blocked.
- Verifies via: Original verifies-via was "An audit script that
  cross-references `cargo info <crate> --vers <yanked>` with the
  release-history file. Every yanked version must have an entry." Two
  halves:
  - "Every entry in the file is genuinely yanked on crates.io" → satisfied
    by `just release-history-audit`. Recipe logic traced against the
    crates.io API schema; runtime verification against the two known
    yanks is gated on network access (sandbox is offline; CI doctor
    workflow exercises the network path on first run after this commit
    lands).
  - "Every yanked version on crates.io has an entry" → not satisfied
    here; requires enumerating every Reflective-published crate and its
    versions on crates.io, which is broader engineering. Tracked as
    residual under `QF-2026-06-02-24` (`release-undo` recipe that records
    and yanks in one atomic operation, eliminating the discipline gap at
    the source).
- Codex-safe now: Yes
- Properties: (closed; `RP-YANK-DISCOVERABLE` continues to be tracked by
  `QF-2026-06-02-24` for the reverse-discovery half)
- Confidence: H
- Business leverage: One yank-without-trail incident per quarter caught
  at PR time saves ~2 hours of "what was 1.0.1 again?" archaeology when a
  consumer hits the yanked version and reports it.
- Last reviewed: 2026-06-07 (B-tier follow-up session)
- Cycles open: 5 days (2026-06-02 → 2026-06-07)
- History:
  - 2026-06-02: Opened (paired session; Cycle 1)
  - 2026-06-07: `KB/release-history.md` + `project-doctor` check 6
    shipped (Codex earlier in session)
  - 2026-06-07: `AGENTS.md > Release yank discipline` + `just
    release-history-audit` recipe shipped (this commit)
  - 2026-06-07: Closed Done — discipline + structural + cross-reference
    all landed; reverse-discovery carved to `QF-2026-06-02-24`.
- Linked PRs / commits: 70160e5 (closing — audit recipe + AGENTS.md
  section + ledger entry); earlier same-session commits shipped
  `KB/release-history.md` and `project-doctor` check 6.
- Drift check: `just project-doctor` check 6 (structural) +
  `just release-history-audit` (crates.io cross-reference). Both wired
  into the doctor workflow.
- Standard promoted: *(folded into `KB/release-history.md` itself, which
  carries the runbook + schema inline; `AGENTS.md > Release yank
  discipline` is the index entry pointing to it)*

### QF-2026-06-02-07

- Date: 2026-06-02
- Bucket: A. Must fix now
- Area: AI-factory discipline
- Discovered during: paired session (atelier-showcase retrieval test)
- Evidence: In commit `7bb07f8`, `atelier-showcase/crates/atelier-domain/
  src/retrieval.rs:281` was rewritten to construct a per-call provenance string
  `"retrieval:embedder=X,reranker=Y"` so that
  `retrieval::tests::retrieve_as_proposals` would pass. The original code used
  `ATELIER_DOMAIN_PROVENANCE.provenance()` (the design intent from the 3.9 API
  drift migration). The agent (this assistant) treated the test assertion as
  canonical and rewrote production code to match — without consulting design
  intent. Reverted in commit `8d41dd1` (v1.0.2); v1.0.1 yanked.
- Impact: Production semantics drifted to satisfy a stale test. The same
  failure mode at scale would turn the test suite into a forcing function for
  arbitrary code drift.
- Risk if ignored: AI agents become a vector for slow design erosion. Every
  test is permission to mutate code.
- Effort: S (convention); M (tooling)
- Owner: Codex
- Status: Done (with documented split: convention shipped; hook follow-up
  tracked under `QF-2026-06-07-01`)
- Resolution: Convention half landed in `AGENTS.md > Test/code attribution`.
  Commits that modify both a production file and its directly-corresponding
  test must declare one of three classifications in the commit message body:
  **Contract update** (test was wrong; production is correct), **Fixture
  refresh** (production was wrong; the test is the contract), or **Real bug
  fix** (both moved because a bug required both). Cites the atelier retrieval
  incident inline as the motivating failure. Reviewer-enforced at PR time.
  The mechanical pre-commit hook half (the original verifies-via) is carved
  out to `QF-2026-06-07-01` because the "directly-corresponding" heuristic
  needs a paired design call on file-pair semantics (same-crate `src/X.rs` ↔
  `tests/X.rs`, in-source `mod tests`, doc-tests, `src/X/tests.rs`, etc.).
- Verifies via: `AGENTS.md` ships the policy. Original verifies-via (hook)
  remains the success criterion for `QF-2026-06-07-01`.
- Codex-safe now: Yes (convention is now policy; hook needs a paired design
  call on heuristics).
- Properties: (closed; `RP-AI-SHORTCUT-DECLARED` is `Enforced` via
  convention with no open finding tracking it; `RP-TEST-CODE-ATTRIBUTION` is
  `Enforced` via convention with hook follow-up under `QF-2026-06-07-01`)
- Confidence: H
- Last reviewed: 2026-06-07 (A-tier autonomous work session)
- Cycles open: 5 days (2026-06-02 → 2026-06-07)
- History:
  - 2026-06-02: Opened (paired session; Cycle 1)
  - 2026-06-07: Closed Done — convention shipped in `AGENTS.md`; hook half
    carved to `QF-2026-06-07-01`.
- Linked PRs / commits: `7bb07f8` (introduced the bug), `8d41dd1` (revert),
  `atelier-domain` v1.0.1 yanked + v1.0.2 published, 0d049e3 (closing —
  AGENTS.md convention shipped)
- Drift check: Reviewer-enforced at PR time per `AGENTS.md > Test/code
  attribution`. Mechanical hook follow-up under `QF-2026-06-07-01`.
- Standard promoted: *(folded into the convention text in `AGENTS.md`
  itself, which is the standard)*

### QF-2026-06-02-04

- Date: 2026-06-02
- Bucket: A. Must fix now
- Area: semver discipline / release engineering
- Discovered during: release rehearsal (the converge → train cascade)
- Evidence: `bedrock-platform/converge` shipped commit `ef14ec5 pack: type
  Suggestor::provenance() as Provenance, not &'static str` between v3.9.1 and
  v3.9.2, then released v3.9.2 as a patch bump. That change broke
  `Suggestor::provenance()` for every downstream implementor and cascaded
  through `organism-learning`, `converge-prism-analytics`, `axiom-truth`, and
  atelier scenarios. 60+ downstream crates required code edits, not just
  version bumps, to compile against converge 3.9.2.
- Impact: A patch release advertised wire-compatible behavior but broke the
  Suggestor trait surface. Any consumer who took `^3.9` updates with
  `cargo update` would break on next build.
- Risk if ignored: External crates.io consumers of converge hit surprise
  compile breakage on every patch; trust in the version contract erodes.
- Effort: M
- Owner: Codex
- Status: Done (detection landed; release-recipe gate-integration pending
  `REL_APPLY=1` wiring per `QF-2026-06-02-26`)
- Resolution: Added `just release-public-api-check name` recipe to root
  `Justfile`. For each publishable crate under the named project's `crates/`
  directory, runs `cargo public-api diff v<current>..HEAD` and classifies the
  diff: `-` lines (removed or changed items) → **breaking**, `+` lines only
  → **additive** (minor bump), empty diff → **unchanged** (patch bump).
  Exit code reflects the count of breaking crates. Preconditions
  (`cargo-public-api` + nightly toolchain) are checked at recipe start with
  friendly install instructions on miss. Same `fails`-accumulator pattern as
  `release-preflight` / `release-deps-audit` (`set -uo pipefail` annotated
  `# permissive` per `QF-2026-06-02-22`).
  Verified end-to-end against converge: 10 publishable crates checked vs
  `v3.9.2`, output:
    converge-pack: 48 addition(s) vs v3.9.2 (minor bump)
    9 crates: no public-API change vs v3.9.2 (patch bump)
    Summary: 0 breaking, 1 additive, 9 unchanged
  Had this gate existed on 2026-06-02, the Suggestor trait change would have
  shown as a `-` line on `converge-pack` and the recipe would have refused
  the patch-bump release.
- Verifies via: Local green run (above) demonstrates the patch-bump case.
  The synthetic-breaking-change PR scenario remains an offline test (revert
  any v3.9.2-era pub-API item and re-run); the operator runs this before
  tagging. Integration into the `release` recipe's bump-target gating
  (refusing patch/minor bumps when breaking changes are detected) lands in
  the same commit that wires `REL_APPLY=1` per `QF-2026-06-02-26`.
- Codex-safe now: Yes (detection is mechanical; gate integration with
  release recipe is a small next step once `REL_APPLY` is unblocked).
- Properties: (closed; `RP-SEMVER-GATED` is `Enforced` with caveats — see
  the RP table for the integration-pending note)
- Confidence: H
- Business leverage: One QF-04-pattern incident (60+ downstream crates,
  yank-and-replace round, ~1 week of cross-train churn) caught at preflight
  time saves ~40 person-hours and prevents the trust loss on the version
  contract.
- Last reviewed: 2026-06-07 (A-tier autonomous work session)
- Cycles open: 5 days (2026-06-02 → 2026-06-07)
- History:
  - 2026-06-02: Opened (release rehearsal; Cycle 1)
  - 2026-06-07: Closed Done — `release-public-api-check` recipe shipped;
    converge sanity green; release-recipe gate integration carved to the
    `REL_APPLY=1` wiring under `QF-2026-06-02-26`.
- Linked PRs / commits: `ef14ec5` (the original breaking change in
  converge), v3.9.2 release (the cascade), 0d049e3 (closing —
  release-public-api-check recipe + ledger entry)
- Drift check: `just release-public-api-check name` (root `Justfile`).
- Standard promoted: *(pending — propose
  `KB/05-engineering/standards/semver-discipline.md`: every publishable
  crate's pre-tag checklist runs `release-public-api-check`; breaking diff
  forces either a major bump or a revert; cite `QF-2026-06-02-04` as the
  motivating incident.)*

### QF-2026-06-02-23

- Date: 2026-06-02
- Bucket: C. Strategic improvement
- Area: release engineering / maintainability
- Discovered during: PR review (Tier 2)
- Discovered in: PR Quality Gate Cycle 1 (Justfile release-train sequence)
- Evidence: `Justfile` encoded the release train twice: `release_order` (originally
  line 307) and `_release-dir name` case statement (lines 310-334). The eight mosaic
  specialists were also hand-coded twice. A new project added in one place but not the
  other was silently skipped.
- Impact: Drift between the two lists made new train members invisible to one
  execution path or the other.
- Risk if ignored: New specialist added; `release-all` skipped it for two cycles
  before anyone noticed.
- Effort: M
- Owner: Codex
- Status: Done (with documented design choice)
- Resolution: Chose a **drift-check design** instead of the strict "single parse"
  next-action: `release-train.yaml` is the canonical source of order and dirs;
  `project-doctor` check 1 + `release-train-sync` enforce that the Justfile's
  `release_order` variable and `_release-dir` case mirror the yaml. The doctor
  workflow runs both on every PR, so drift is caught at PR time — adding a new train
  member to the yaml without updating the Justfile fails CI. The original spec
  ("Justfile parses it once; no hard-coded list remains") is not literally satisfied
  — two sources still exist, kept in sync by the check rather than collapsed into
  one. The spirit ("no silent gaps; new members can't vanish") IS satisfied. The
  yaml is also missing the `publishable` and `notes` schema fields from the
  original spec. The strict end-state is carved out to follow-up finding
  `QF-2026-06-06-02`.
- Verifies via: `project-doctor` check 1 (`release-train.yaml` order == Justfile
  `release_order`; every member dir exists) runs in `.github/workflows/doctor.yml`
  on every PR. Validated by the 2026-06-04 first-green CI run (job
  `project-doctor`, ID `79472158112`, 53s).
- Codex-safe now: Yes (closure recognizes the shipped drift-check design;
  follow-up `QF-2026-06-06-02` carries the strict end-state)
- Properties: (closed; `RP-RELEASE-TRAIN-INTEGRITY` continues to be tracked by
  `QF-2026-06-06-02`)
- Confidence: M (drift-check approach is honest about the design trade-off)
- Last reviewed: 2026-06-06 (audit of stale findings)
- Cycles open: 4 days (2026-06-02 → 2026-06-06)
- History:
  - 2026-06-02: Opened (PR review Tier 2; PR Gate Cycle 1 on `db6d02e` + `f359a14`)
  - 2026-06-04: First CI run of `project-doctor` check 1 green
    (workflow run `26937960731`)
  - 2026-06-06: Closed Done — drift-check design accepted as alternative to single
    parse; strict end-state carved to `QF-2026-06-06-02`.
- Linked PRs / commits: db6d02e, f359a14 (original duplication),
  `4416e85` / `ed44065` / `8f6e240` / `ac755d6` (release-train.yaml +
  project-doctor check 1 + release-train-sync + RP flips), e4778d5 (closing
  ledger entry; strict end-state carved to `QF-2026-06-06-02`)
- Drift check: `just project-doctor` check 1; `just release-train-sync`.
- Standard promoted: *(pending — fold into
  `KB/05-engineering/standards/release-train-order.md` proposed by
  `QF-2026-06-02-21`: train manifest lives in a single yaml at repo root; per-recipe
  consumers may either parse it directly or keep a synced copy that a doctor check
  asserts matches. Either pattern is acceptable; document the trade-offs.)*

### QF-2026-06-02-06

- Date: 2026-06-02
- Bucket: B. Should fix soon
- Area: tests / release engineering
- Discovered during: paired session (trybuild walkthrough)
- Evidence: `bedrock-platform/organism/crates/pack/tests/compile_fail/
  fact_no_new.stderr` was blessed in commit `af9b754` and captured the absolute
  local path `(reflective-root)/bedrock-platform/converge/crates/pack/
  src/fact.rs:1123:5` along with current line numbers. Test passed only on the
  blesser's machine. Repaired in commit `3e1a7c8` by restoring
  `$CARGO/converge-pack-$VERSION/` placeholders and adding a runtime skip guard for
  the `[patch.crates-io]` case.
- Impact: A snapshot that silently baked in machine state defeated the point of a
  portable contract test.
- Risk if ignored: Future `TRYBUILD=overwrite` blesses would repeat the same
  failure mode in other repos.
- Effort: S
- Owner: Codex
- Status: Done
- Resolution: `project-doctor` check 4 (root `Justfile`, wired into
  `.github/workflows/doctor.yml`) scans every `.stderr` file across the train
  workspaces (`bedrock-platform mosaic-extensions atelier-showcase arena-tests
  runtime-runway commerce-rails`) for absolute-path leaks. Patterns matched:
  `/Users/`, `/home/[a-z]+/`, `/private/var/folders/`, `/tmp/[A-Za-z0-9]{6,}`.
  Fails CI when found; runs on every PR via the doctor workflow. The original
  failure mode (path leak — the exact `/Users/kpernyer/...` bake-in cited in the
  evidence) is fully detected. The next-action also mentioned scanning for
  "4-or-more-digit gutter line numbers in foreign files" — that's scope creep on
  the original path-leak bug and is already noted as a pending refinement on the
  `RP-SNAPSHOT-PORTABLE` row ("*.snap and foreign-line-gutters still pending"); not
  filed as a separate finding to avoid ledger noise.
- Verifies via: Original verifies-via was "Lint script in `xtask/` (or equivalent)
  that grep-scans fixtures and exits non-zero on offense" — satisfied by
  `project-doctor` check 4 inline in `Justfile` (functional equivalent of an
  `xtask/`). Validated by the 2026-06-04 first-green CI run (job `project-doctor`,
  ID `79472158112`, 53s).
- Codex-safe now: Yes
- Properties: (closed; `RP-SNAPSHOT-PORTABLE` continues to be tracked by no open
  finding — the property itself is `Enforced`)
- Confidence: H
- Last reviewed: 2026-06-06 (audit of stale findings)
- Cycles open: 4 days (2026-06-02 → 2026-06-06)
- History:
  - 2026-06-02: Opened (paired session; Cycle 1)
  - 2026-06-02: Initial repair landed in commit `3e1a7c8`
  - 2026-06-04: First CI run of `project-doctor` check 4 green
    (workflow run `26937960731`)
  - 2026-06-06: Closed Done — lint mechanized via `project-doctor` check 4 and
    enforced in CI; original path-leak failure mode caught.
- Linked PRs / commits: `af9b754` (introduced the bug), `3e1a7c8` (initial repair),
  `4416e85` (project-doctor check 4 shipped), `ed44065` (CI wired), e4778d5 (closing
  ledger entry)
- Drift check: `just project-doctor` check 4.
- Standard promoted: *(pending — fold the path-leak rule into
  `KB/05-engineering/standards/trybuild-portability.md`: every `.stderr` and
  `.snap` fixture must scrub absolute paths, machine names, and foreign-crate
  current line numbers before commit; bless-time review classifies cosmetic vs.
  semantic per `RP-AUTO-BLESS-AUDITED`.)*

### QF-2026-06-03-01

- Date: 2026-06-03
- Bucket: B. Should fix soon (closed Won't do same-day)
- Area: CI/CD reliability / operator setup
- Discovered during: PR review (Tier 1)
- Discovered in: Closure follow-up to `QF-2026-06-02-01` (root CI wiring)
- Evidence: An earlier draft of `.github/workflows/doctor.yml` (commit
  `4416e85`) gated the `project-doctor` job on a `CI_REPO_TOKEN` secret —
  a fine-grained PAT was required for `actions/checkout` to fetch the 8
  train repos. This finding was filed to track the operator's
  provisioning of that PAT.
- Impact: While the PAT was unprovisioned, four recipe-backed RPs
  (`RP-LAYERING`, `RP-CRATE-SIZE-BUDGET`, `RP-SNAPSHOT-PORTABLE`,
  `RP-RUSTC-DRIFT-CONTAINED`) could not reach `Enforced`.
- Effort: S
- Owner: Karl
- Status: Won't do
- Resolution: Same-day update to `.github/workflows/doctor.yml` removed
  the PAT gate entirely because all 8 train repos (plus mosaic-extensions)
  are public. `actions/checkout` now fetches them anonymously; the
  `CI_REPO_TOKEN` secret is no longer needed. The four recipe-backed RPs
  flip directly to `Enforced` once the first CI run is green, with no
  intermediate operator action required. If any of the train repos ever
  flip back to private, restore the cross-repo PAT pattern documented in
  commit `4416e85` and re-open this finding.
- Verifies via: `.github/workflows/doctor.yml` (current state) — no
  `secrets.CI_REPO_TOKEN` reference; each `actions/checkout` step uses
  the default anonymous fetch.
- Codex-safe now: n/a (closed Won't do)
- Properties: (closed; the four RPs are now tracked by the workflow
  itself via the unconditional `project-doctor` job rather than by this
  finding)
- Confidence: H
- Last reviewed: 2026-06-03 (Cycle 1 day 2)
- Cycles open: 0 (opened and closed within the same day, same hour)
- History:
  - 2026-06-03: Opened (PR review Tier 1; closure follow-up to
    `QF-2026-06-02-01`)
  - 2026-06-03: Closed Won't do — workflow updated to public-repo
    anonymous checkout; the gating concern dissolved.
- Linked PRs / commits: 4416e85 (introduced the PAT gate), ed44065
  (removed the gate), ac755d6 (recorded this Won't do in the ledger)

### QF-2026-06-02-01

- Date: 2026-06-02
- Bucket: B. Should fix soon
- Area: CI/CD reliability
- Evidence: Root `Justfile` defined cross-workspace `check`, `build`, `test`,
  `lint`, `fmt-check`, and release-preflight recipes, but the root repo had no
  `.github/` directory and no dated CI/CD inventory in tracked root docs.
  `find` only found nested workflow directories under
  `atelier-showcase/.github` and `runtime-runway/.github`.
- Impact: The coordination layer couldn't quickly show which repos have CI,
  security scanning, dependency updates, release gates, or branch protection.
- Risk if ignored: Factory health degrades unevenly across repos and
  recurring reviews spend time rediscovering the same CI/CD map.
- Effort: M
- Owner: Codex
- Status: Done
- Resolution: Shipped `.github/workflows/doctor.yml` (commit `4416e85` —
  Codex; extended in this commit) gating PRs and pushes to `main` with two
  jobs:
  - `quality-doctor` — runs from the root checkout only. Executes
    `just quality-doctor` (10 checks) + `just release-train-sync` (the
    latter added in the closing commit so the manifest-sync invariant is
    enforced even if the cross-repo job is ever degraded). Enforces
    `RP-POLICY-FRESH` and the manifest-sync subset of
    `RP-RELEASE-TRAIN-INTEGRITY`.
  - `project-doctor` — checks out the 8 train workspaces
    (`Reflective-Lab/*`) side-by-side under the root via anonymous
    `actions/checkout` (all 9 train repos are public), installs
    `rustc 1.96.0`, and runs `just project-doctor` (5 checks). Enforces
    `RP-RELEASE-TRAIN-INTEGRITY` (dir-existence half), `RP-LAYERING`,
    `RP-CRATE-SIZE-BUDGET`, `RP-SNAPSHOT-PORTABLE`, and
    `RP-RUSTC-DRIFT-CONTAINED`. The earlier draft gated this job on a
    `CI_REPO_TOKEN` PAT secret; same-day workflow update removed the gate
    once the public-repo state was confirmed, so the finding tracking PAT
    provisioning (`QF-2026-06-03-01`) was closed Won't do.
  Also extended `.gitignore` allow-list with `!release-train.yaml`,
  `!.github/`, and `!.github/**` so the orchestration surface stays tracked.
  The original next-action (a "cross-repo CI/CD inventory" doc) is now
  rendered by the workflow file itself: `.github/workflows/doctor.yml` is
  the dated, tracked inventory of root-level CI. Per-nested-repo inventory
  becomes per-repo work as each train repo grows its own
  `.github/workflows/`.
- Verifies via: Local green confirmed pre-commit (`just quality-doctor`
  10/10 ✓; `just release-train-sync` ✓; `just doctor` 15/15 ✓). First push
  to `main` after this closure exercises both CI jobs on GitHub: both
  expected green (subject to the public-repo assumption holding for all 9
  train repos).
- Codex-safe now: Yes (recipe + workflow are mechanical; no operator
  follow-up remains).
- Confidence: H
- Business leverage: Repeated rediscovery of the CI/CD map was on the order
  of 30-60 minutes per review across ≥10 repos; the persistent workflow now
  enforces the map automatically every PR and push.
- Properties: (closed; previously tracked the absence of root CI. The
  workflow now enforces `RP-POLICY-FRESH` and the manifest-sync subset of
  `RP-RELEASE-TRAIN-INTEGRITY`.)
- Last reviewed: 2026-06-03 (Cycle 1 day 2)
- Cycles open: 1 (opened 2026-06-02; closed 2026-06-03)
- History:
  - 2026-06-02: Opened (audit; Cycle 1 baseline)
  - 2026-06-03: Closed Done (Cycle 1 day 2; root CI wired)
- Linked PRs / commits: 4416e85 (initial workflow with PAT gate), ed44065
  (dropped PAT gate after confirming public repos), 8f6e240
  (`release-train-sync` recipe), ac755d6 (closing ledger entry + RP flips)
- Drift check: The workflow itself. Quality-doctor + release-train-sync
  cover the root surface unconditionally; project-doctor extends coverage
  to the 8 train workspaces once `QF-2026-06-03-01` is closed.
- Standard promoted: *(pending — propose
  `KB/05-engineering/standards/root-ci-gate.md`: every coordination repo
  ships `.github/workflows/doctor.yml` (or equivalent) running
  `quality-doctor` + `release-train-sync` unconditionally; CI gating
  RP-* checks that need cross-repo state uses a cross-repo PAT or
  submodules, with a documented degrade-gracefully path when the
  prerequisite is absent.)*

### QF-2026-06-02-30

- Date: 2026-06-02
- Bucket: B. Should fix soon
- Area: rustc drift / reproducibility
- Discovered during: first run of `just project-doctor` check 5 (RP-RUSTC-DRIFT-CONTAINED)
- Evidence: 8 of 9 release-train workspaces had no `rust-toolchain.toml`
  (`bedrock-platform/converge`, `bedrock-platform/axiom`, `bedrock-platform/organism`,
  `bedrock-platform/helms`, `atelier-showcase`, `arena-tests`, `runtime-runway`,
  `commerce-rails`). The one existing toolchain file in the repo —
  `marquee-apps/scout-sourcing/rust-toolchain.toml` (out of train scope) — pinned
  `channel = "stable"`, which resolves to whatever `rustup` happens to have installed
  rather than a fixed version.
- Impact: Two contributors with different `rustup update` cadences produce different
  snapshot output, different clippy lints, and different compile-fail diagnostic text.
  `cargo update -w` plus a silent rustc bump can land in the same PR as code changes;
  the snapshot drift gets blamed on flakiness rather than the toolchain.
- Risk if ignored: Trybuild fixtures bless cosmetically-different rustc output on one
  machine, then fail on CI or on another contributor's box. The discipline of
  `RP-AUTO-BLESS-AUDITED` (classifying snapshot diffs as cosmetic-vs-contract) becomes
  unenforceable.
- Effort: S (per repo) × 8 independent gits
- Owner: Codex
- Status: Done
- Resolution: Added `rust-toolchain.toml` pinning `channel = "1.96.0"` (current installed
  stable as of 2026-06-02) with `components = ["rustfmt", "clippy"]` to all 8 train
  workspace roots. Identical comment in each file points readers at
  RP-RUSTC-DRIFT-CONTAINED and `just project-doctor` check 5. The named `1.96.0`
  toolchain had to be installed on the local machine via
  `rustup component add cargo --toolchain 1.96.0` — `rustup toolchain install
  1.96.0 --profile minimal` shipped rustc/rustfmt/clippy but not cargo.
- Verifies via: `just project-doctor` check 5 returns `✓ every train workspace pins
  rustc to an exact channel` for all 8 workspaces. Same-day green run, exit 0.
- Codex-safe now: Yes
- Properties: RP-RUSTC-DRIFT-CONTAINED
- Confidence: H
- Last reviewed: 2026-06-02 (Cycle 1; project-doctor mechanization)
- Cycles open: 0 (opened and closed within Cycle 1, same day)
- History:
  - 2026-06-02: Opened (first project-doctor run; Cycle 1)
  - 2026-06-02: Closed Done (8 toolchain files added; Cycle 1)
- Linked PRs / commits:
  - `Reflective-Lab/converge@02d2648` (closing)
  - `Reflective-Lab/axiom@5b19956` (closing)
  - `Reflective-Lab/organism@1548d30` (closing)
  - `Reflective-Lab/helms@5c43817` (closing)
  - `Reflective-Lab/atelier-showcase@dc7b415` (closing)
  - `Reflective-Lab/arena-tests@abd6527` (closing)
  - `Reflective-Lab/runtime-runway@012b81b` (closing)
  - `Reflective-Lab/commerce-rails@2e5680f` (closing)
- Standard promoted: `KB/05-engineering/standards/rustc-pinning.md` (2026-06-07)

### QF-2026-06-02-29

- Date: 2026-06-02
- Bucket: B. Should fix soon
- Area: release engineering / layering discipline
- Discovered during: first run of `just project-doctor` check 2 (RP-LAYERING)
- Evidence: `bedrock-platform/helms/apps/desktop/src-tauri/Cargo.toml`
  (`outcome-workbench-desktop`) did not set `publish = false`. Its path-deps
  `prio-expenses`, `workbench-backend`, `application-kernel`, and `application-storage`
  are all `publish = false`. `cargo metadata` therefore classified the desktop crate
  as publishable, producing 4 layering violations. The desktop binary is a Tauri app
  and cannot be published to crates.io regardless.
- Impact: The crate was misclassified as publishable. A naive `cargo publish -p
  outcome-workbench-desktop` would fail on the unpublished path-deps and the operator
  would have to reverse-engineer why. More importantly, the LAYERING check stayed
  red on a known false-positive, which would erode trust in the gate and mask real
  violations elsewhere.
- Risk if ignored: As helms grows more `apps/` crates, the misclassification spreads;
  the LAYERING signal degrades; humans learn to ignore the gate.
- Effort: XS (single line)
- Owner: Codex
- Status: Done
- Resolution: Added `publish = false` to the desktop Tauri crate's `[package]` block
  with an inline comment pointing at RP-LAYERING and `just project-doctor` check 2.
- Verifies via: `just project-doctor` check 2 returns `✓ no publishable crate path-deps
  an unpublishable one`. Same-day green run, exit 0.
- Codex-safe now: Yes
- Properties: RP-LAYERING
- Confidence: H
- Last reviewed: 2026-06-02 (Cycle 1; project-doctor mechanization)
- Cycles open: 0 (opened and closed within Cycle 1, same day)
- History:
  - 2026-06-02: Opened (first project-doctor run; Cycle 1)
  - 2026-06-02: Closed Done (publish = false added; Cycle 1)
- Linked PRs / commits: `Reflective-Lab/helms@72ad5a0` (closing)
- Standard promoted: `KB/05-engineering/standards/publish-status-declaration.md` (2026-06-07)

### QF-2026-06-02-28

- Date: 2026-06-02
- Bucket: B. Should fix soon
- Area: software-factory mechanization / standing properties
- Discovered during: Cycle 1 follow-up (user-directed mechanization push:
  "can we get more than 3 mechanized?")
- Evidence: Of the 14 Recurring System Properties declared in this ledger, only
  `RP-POLICY-FRESH` had a runnable enforcement recipe (`just quality-doctor`) as of
  `74cb465`. The other 13 were `Aspired` — known failure modes (semver violations,
  layering breaks, oversized crates, unpinned rustc, machine-specific snapshot leaks)
  with no automated drift check.
- Impact: With no project-level recipe, drift was detected only when it caused a
  visible incident (60-downstream-break, 10 MiB upload reject, yank-and-replace,
  etc.). The flywheel cannot spin from "finding → standard → drift check" if step 3
  doesn't exist.
- Risk if ignored: The system stays policy-first / enforcement-second. Each new RP
  gets added to the table and stays Aspired forever; trust in the gates erodes; the
  "learning quality flywheel" becomes an architecture diagram.
- Effort: M
- Owner: Codex
- Status: Done
- Resolution: Added `just project-doctor` recipe to root `Justfile` (sibling to
  `quality-doctor`; aggregated by new `just doctor` recipe). Mechanizes 5 of the 13
  remaining properties as accumulating checks following the same
  exit-code-equals-fail-count pattern as `quality-doctor`:
  1. RP-RELEASE-TRAIN-INTEGRITY — `release-train.yaml` (new, at workspace root) is
     the source of truth for publish order; Justfile `release_order` must match;
     every member directory must exist.
  2. RP-LAYERING — `cargo metadata` walk; publishable crate may not path-dep a
     `publish = false` crate. Surfaced and closed `QF-2026-06-02-29`.
  3. RP-CRATE-SIZE-BUDGET — `find` excludes `target/.git/node_modules/vendor/
     .terraform/*.db`; fails on any source file > 1 MiB in a publishable workspace.
  4. RP-SNAPSHOT-PORTABLE — greps `.stderr` fixtures for `/Users/`,
     `/home/<user>/`, `/private/var/folders/`, `/tmp/<random>` paths.
  5. RP-RUSTC-DRIFT-CONTAINED (pinning half) — verifies each train workspace has
     `rust-toolchain.toml` pinning an exact `1.X.Y` or dated nightly. Surfaced
     and closed `QF-2026-06-02-30`.
- Verifies via: Same-day green run of `just project-doctor` — all 5 checks pass,
  exit 0 — after closing -29 and -30. `just doctor` aggregates with quality-doctor
  (10 checks) for a single 15-check green.
- Codex-safe now: Yes
- Properties: RP-RELEASE-TRAIN-INTEGRITY, RP-LAYERING, RP-CRATE-SIZE-BUDGET,
  RP-SNAPSHOT-PORTABLE, RP-RUSTC-DRIFT-CONTAINED
- Confidence: H
- Last reviewed: 2026-06-02 (Cycle 1; project-doctor mechanization)
- Cycles open: 0 (opened and closed within Cycle 1, same day)
- History:
  - 2026-06-02: Opened (user-directed mechanization push; Cycle 1)
  - 2026-06-02: Closed Done (recipe shipped + 2 surfaced violations fixed; Cycle 1)
- Linked PRs / commits:
  - d85b107 (project-doctor + doctor recipes added to root `Justfile`)
  - 4416e85 (release-train.yaml + .github/workflows/doctor.yml + ledger entries; closing)
- Standard promoted: `KB/05-engineering/standards/doctor-recipe-pattern.md` (2026-06-07)

### QF-2026-06-02-27

- Date: 2026-06-02
- Bucket: B. Should fix soon
- Area: release engineering / correctness
- Discovered during: PR review (Tier 2)
- Discovered in: PR Quality Gate Cycle 1 (Justfile release-train sequence)
- Evidence: `Justfile` section 3 of `release-preflight` was `git -C "$dir" rev-parse "v$ver"
  >/dev/null 2>&1` — local refs only. A tag could exist on origin but not locally if not
  fetched; operator runs preflight, sees no local tag, proceeds; `git push --tags` later fails.
- Impact: Preflight gave a false-clean signal for tag uniqueness.
- Risk if ignored: Mid-release `git push --tags` rejection; ad-hoc resolution; visible to
  external consumers via crates.io ghost state.
- Effort: S
- Owner: Codex
- Status: Done
- Resolution: Rewrote `release-preflight` section 3 to:
  1. Prepend `git -C "$dir" fetch --tags --quiet origin 2>/dev/null || true` so the local
     view of remote tags is fresh before probing.
  2. Probe local with the existing `git rev-parse "v$ver"`.
  3. Probe remote with `git ls-remote --tags origin "refs/tags/v$ver"`.
  4. Report four distinct states: `tag v$ver exists (local + remote)` (the normal
     post-release state); `⚠ tag v$ver exists locally but not on origin (aborted prior
     push?)` (unpushed local tag — possible aborted release); `⚠ tag v$ver exists on origin
     but not locally (run \`git fetch --tags\`)` (only reachable if the silent fetch
     failed); no output if the tag exists nowhere (first release of this version).
  Informational only — no new ✗ failure mode was added; the original behavior was also
  informational. The deeper question of "does v<next> (the bump target) already exist?"
  belongs to the `release` recipe, not preflight, and was out of scope for this finding.
- Verifies via: Same-day sanity run on `bedrock-platform/converge` (current version 3.9.2,
  tag pushed to origin):
  - Before: `tag v3.9.2 already exists locally`
  - After: `tag v3.9.2 exists (local + remote)` (the new check correctly identifies that
    the tag is present in both places, matching reality).
- Codex-safe now: Yes
- Confidence: H
- Last reviewed: 2026-06-02 (PR Gate Cycle 1 implementation, round 5)
- Cycles open: 0 (opened and closed within Cycle 1, same day as gate)
- History:
  - 2026-06-02: Opened (PR review Tier 2; PR Gate Cycle 1 on `f359a14`)
  - 2026-06-02: Closed Done (PR Gate Cycle 1 implementation round 5; Cycle 1)
- Linked PRs / commits: f359a14 (introduced the gap), d85b107 (closing)
- Drift check: tagged as an inline preflight section. A future tightening would compute the
  bump target and assert it does not exist anywhere — but that is a separate concern about
  the next version's tag, not the current one.
- Standard promoted: *(pending — fold into
  `KB/05-engineering/standards/release-preflight-discipline.md` proposed by
  `QF-2026-06-02-20`: every preflight check that touches a remote-observable resource
  must probe both local and remote views and report divergence distinctly)*

### QF-2026-06-02-22

- Date: 2026-06-02
- Bucket: B. Should fix soon
- Area: release engineering / reliability
- Discovered during: PR review (Tier 2)
- Discovered in: PR Quality Gate Cycle 1 (Justfile release-train sequence)
- Evidence: Seven recipes in `Justfile` used `set -uo pipefail` (missing `-e`): `_dispatch`,
  `release-preflight`, `release-preflight-all`, `release-migrate-deps`, `release`,
  `release-all`, `status`. The `_bedrock-loop`, `_containers-loop`, and explicit `ws-*`
  recipes used `set -euo pipefail`. The release path was exactly where loose semantics
  combined worst with `QF-2026-06-02-20` (toothless preflight).
- Impact: Mid-recipe command failures didn't abort the recipe in the release path.
- Risk if ignored: Force-multiplier for partial-success release failures; echoes
  `QF-2026-06-02-08`.
- Effort: S
- Owner: Codex
- Status: Done
- Resolution: Audited every `set -uo pipefail` site in the Justfile against the finding's
  next-action rule "normalize to `-euo` except where loose semantics are intentional, in
  which case keep `-uo` and add `# permissive: <reason>`." Result split into two groups:
  - **Switched to `set -euo pipefail`** (6 recipes — explicit-exit handling already
    present, so `-e` matches current behavior): `_dispatch` (each branch ends in
    `exit $?`), `release-preflight-all` (`|| true` neutralizes failures), `release`
    (explicit `|| exit N` after each operation), `release-all` (same), `snapshot`
    (Python heredoc only), `status` (informational, no significant failure modes; was
    `set -u` only — promoted to `set -euo pipefail` for consistency).
  - **Kept `set -uo pipefail` with explicit `# permissive: <reason>` annotation** (3
    recipes — they accumulate failures into the `fails` counter and would break under
    `-e`): `release-deps-audit`, `release-migrate-deps`, `doctor`. The aggregate gate
    `doctor` is particularly load-bearing — it captures each sub-recipe's `$?` to sum
    them; `-e` would abort before recording the sub-codes.
  Three recipes that already had the permissive annotation are unchanged:
  `release-preflight`, `quality-doctor`, `project-doctor`.
- Verifies via: `grep -nE 'set -uo' Justfile` now returns exactly 6 result lines, each
  paired with a `# permissive: …` (or equivalent "intentional, not -euo") comment block
  on the preceding line(s). Cross-recipe sanity ran clean: `release-preflight converge`
  still accumulates failures (dirty tree → `✗ preflight: 1 check(s) failed`, exit 1);
  `status` runs end-to-end; `quality-doctor` all-green; `release converge` propagates
  preflight failure as before.
- Codex-safe now: Yes
- Confidence: H
- Last reviewed: 2026-06-02 (PR Gate Cycle 1 implementation, round 5)
- Cycles open: 0 (opened and closed within Cycle 1, same day as gate)
- History:
  - 2026-06-02: Opened (PR review Tier 2; PR Gate Cycle 1 on `f359a14`)
  - 2026-06-02: Closed Done (PR Gate Cycle 1 implementation round 5; Cycle 1)
- Linked PRs / commits: f359a14 (introduced the gap), d85b107 (closing)
- Drift check: `quality-doctor` could in principle assert "every `set -uo` line in the
  Justfile is preceded by a `# permissive:` annotation within N lines" — proposed as a
  Bucket C follow-up rather than filed as a separate finding; the closure here records
  the shape of the rule.
- Standard promoted: *(pending — fold into
  `KB/05-engineering/standards/release-preflight-discipline.md` proposed by
  `QF-2026-06-02-20`: shell `set -e` is the default; recipes that accumulate failures via
  a `fails` counter must annotate the deviation with `# permissive: <reason>` citing the
  finding ID that introduced the pattern)*

### QF-2026-06-02-08

- Date: 2026-06-02
- Bucket: A. Must fix now
- Area: release engineering / supply-chain hygiene
- Discovered during: release rehearsal (cargo publish failures)
- Evidence: During the train, multiple workspaces had Cargo.toml internal
  path-deps with no `version =` field. cargo publish failed with
  "dependency does not specify a version" for `runway-storage`,
  `runway-storage-contract`, `runway-accounts`, and others. The dirty fix
  was to add `version = "X.Y.Z"` inline mid-release. Some path-deps point
  at UNLICENSED crates (e.g. `runway-accounts` depends on
  `commerce-rails-stripe` which is `publish = false` and UNLICENSED) — those
  paths are unfixable; they violate layering.
- Impact: Releases stall in the middle of the train; ad-hoc fixes get
  committed mid-publish; the dependency graph carries layering violations
  that the build system never warns about.
- Risk if ignored: Next train hits the same wall; commerce-rails never
  becomes consumable by published crates without violating its UNLICENSED
  status.
- Effort: M
- Owner: Codex
- Status: Done (part a) / Deferred (part b under `QF-2026-06-02-14`)
- Resolution: Implemented part (a) — pre-publish lint for path-deps without
  explicit version. Added standalone `just release-deps-audit name` recipe
  (root Justfile) that runs `cargo metadata --no-deps` against the named
  project's workspace and surfaces every dependency where
  `source == null and path != null and req == "*"` (i.e. path-deps with no
  explicit `version =`). All dependency kinds (normal, build, dev) are
  flagged, because cargo publish has historically rejected dev-deps lacking
  versions in this repo (the original `runway-storage` failure cited in the
  evidence was a dev-dep on `runway-storage-contract`). Wired the audit
  into `release-preflight` as new section 5.5 — its exit code feeds the
  existing `fails` accumulator from `QF-2026-06-02-20`. Standalone usage
  is still available for development: `just release-deps-audit organism`.
  Part (b) — the layering lint that rejects `publishable → publish = false`
  edges — remains deferred under `QF-2026-06-02-14` because the underlying
  decision (whether `commerce-rails` ships as MIT, stays internal, or
  sits behind a private registry) is what determines whether
  `runway-accounts → commerce-rails-stripe` is a violation, a temporary
  exception, or the accepted state. The layering lint is mechanical work
  that falls out naturally once `QF-14` is decided.
- Verifies via: Cross-workspace sanity run (same day):
  - `converge` (recently released; clean) → 0 issues, exit 0 ✓
  - `axiom` (recently released; clean) → 0 issues, exit 0 ✓
  - `organism` → 2 dev-dep stragglers (`organism-runtime ↔ organism-catalog-seed`)
  - `runway` → 2 stragglers (`runway-storage → runway-storage-contract` dev,
    `converge-application → converge-llm` normal)
  - `helms` → 94 stragglers (unreleased workspace; not yet pinned)
  Integration with `release-preflight runway` correctly shows the audit
  inline and ends with `── ✗ preflight: 1 check(s) failed ──`, exit 1.
  Integration with `release-preflight converge` (clean tree, stashed) ends
  with `── ✓ preflight: all checks passed ──`, exit 0.
- Codex-safe now: Yes for (a) — done. Part (b) gated on `QF-2026-06-02-14`.
- Properties: (closed; partial — RP-LAYERING continues to be tracked by
  `QF-2026-06-02-14` which owns the layering decision and the residual lint)
- Confidence: H
- Last reviewed: 2026-06-02 (PR Gate Cycle 1 implementation, round 4)
- Cycles open: 0 (opened in baseline Cycle 1; closed same day)
- History:
  - 2026-06-02: Opened (release rehearsal; Cycle 1)
  - 2026-06-02: Closed Done (part a; PR Gate Cycle 1 implementation round 4)
  - 2026-06-02: Part (b) carved out to `QF-2026-06-02-14` follow-up; not a
    separate finding because the decision and the lint resolve together.
- Linked PRs / commits: 628ccc8 (closing)
- Drift check: `just release-deps-audit name` (standalone) plus
  `release-preflight` section 5.5 (integrated). Both fail non-zero on any
  path-dep without explicit version.
- Standard promoted: *(pending — propose
  `KB/05-engineering/standards/path-dep-versioning.md`: every cargo
  workspace member's path-deps in `[dependencies]` and `[build-dependencies]`
  must carry an explicit `version = "X.Y.Z"` matching the target crate's
  published version; dev-deps follow the same rule in this repo because of
  historical cargo-publish behavior. Drift check is `just release-deps-audit
  <project>` in CI per repo.)*

### QF-2026-06-02-17

- Date: 2026-06-02
- Bucket: B. Should fix soon
- Area: AI-factory discipline / developer experience
- Discovered during: PR review (Tier 1)
- Discovered in: PR Quality Gate Cycle 1 (commit `d4e3cf3`)
- Evidence: `QUALITY_BACKLOG.md` Snapshot block duplicated information derivable from the
  bucket sections. `AGENTS.md` mandated "refresh at the end of every cycle" (weekly
  cadence) but findings landed daily — Cycle 1 alone moved open count 14 → 27 → 25 → 23
  within hours. No mechanism enforced snapshot freshness or detected stale numbers.
- Impact: Snapshot is the single-screen operational status; drift made it untrustworthy.
- Risk if ignored: Operators read the Snapshot and act on stale numbers; the dashboard
  becomes decoration.
- Effort: S
- Owner: Codex
- Status: Done
- Resolution: Added `just snapshot` recipe (inline Python in root `Justfile`) that
  regenerates the eight derivable Snapshot bullets from observable state:
  - **Open findings** by bucket — parses `### Bucket [ABCD]` sections, counts
    `#### QF-` entries.
  - **Closed since last review** — counts `### QF-` entries in `## Completed Findings`,
    lists IDs sorted.
  - **Bucket A SLA breaches** — opens >7 days ago, parsed from each finding's
    `- Date:` field against today.
  - **Standing properties tracked (RP-*)** — counts `| RP-` rows in the RP table.
  - **Standing properties currently green** — counts rows whose `Status` cell contains
    `Enforced`.
  - **Accepted risks**, **ADRs**, **Standards** — filesystem counts of
    `KB/06-operations/risk-register.md`, `KB/04-architecture/decisions/*.md`,
    `KB/05-engineering/standards/*.md` (each contributes `0` until the path exists).
  Wired `quality-doctor` check 7 (`QF-2026-06-02-15`): invokes `just snapshot`, parses
  each bullet's `**N**`, matches the corresponding line in `QUALITY_BACKLOG.md` by
  label prefix, fails if any number differs. Refactored two `## Snapshot` bullets to
  move `(see KB/…)` references after the count value so the label prefix is stable.
- Verifies via: Same-day green run of `just quality-doctor` after the recipe landed —
  all 10 checks passed (the 9 from `QF-2026-06-02-15` plus check 7); intermediate run
  during development surfaced a BSD-grep flag-parsing bug (leading `- ` consumed as
  flag) that was fixed with the `--` separator; another intermediate run surfaced two
  bullets whose `(see KB/…)` parentheticals blocked label-prefix matching, fixed by
  moving the reference after the count. Recipe output is deterministic: same input
  state produces the same eight bullets every time.
- Codex-safe now: Yes
- Properties: `RP-POLICY-FRESH` (this closure tightens the property's enforcement
  surface; declared by `QF-2026-06-02-15`)
- Confidence: H
- Last reviewed: 2026-06-02 (PR Gate Cycle 1 implementation, round 3)
- Cycles open: 0 (opened and closed within Cycle 1, same day as gate)
- History:
  - 2026-06-02: Opened (PR review Tier 1; PR Gate Cycle 1 on `d4e3cf3`)
  - 2026-06-02: Closed Done (PR Gate Cycle 1 implementation round 3; Cycle 1)
- Linked PRs / commits: d4e3cf3 (introduced the gap), 1c232b4 (closing)
- Drift check: `just quality-doctor` check 7.
- Standard promoted: *(pending — fold into
  `KB/05-engineering/standards/quality-policy-drift-check.md` proposed by
  `QF-2026-06-02-15`)*

### QF-2026-06-02-16

- Date: 2026-06-02
- Bucket: B. Should fix soon
- Area: AI-factory discipline / cross-agent coordination
- Discovered during: PR review (Tier 1)
- Discovered in: PR Quality Gate Cycle 1 (commit `d4e3cf3`)
- Evidence: `AGENTS.md` Cross-Agent Coordination states "Codex, Claude, and Gemini all read
  this file" but no root-level `CLAUDE.md`, `CODEX.md`, or `GEMINI.md` existed at the
  coordination repo root. The Claude Code agent in this session loaded `AGENTS.md` only
  because the user pointed at it explicitly.
- Impact: Agent sessions started at the repo root could miss the quality policy entirely.
- Risk if ignored: Each agent session reinvents discipline; the learning flywheel does not
  compound across sessions.
- Effort: S
- Owner: Codex
- Status: Done
- Resolution: Added three short pointer files at the repo root:
  - `CLAUDE.md`: points to `AGENTS.md`, `QUALITY_BACKLOG.md`, and `KB/CLAUDE.md` (Karl's
    personal context).
  - `CODEX.md`: points to `AGENTS.md` and `QUALITY_BACKLOG.md`.
  - `GEMINI.md`: points to `AGENTS.md` and `QUALITY_BACKLOG.md`.
  Extended `.gitignore` allow-list with `!CLAUDE.md`, `!CODEX.md`, `!GEMINI.md`. The decision
  to reference `KB/CLAUDE.md` only from root `CLAUDE.md` (not `CODEX.md` or `GEMINI.md`)
  treats `KB/CLAUDE.md` as Claude Code's personal-context file, which Codex and Gemini do not
  consume.
- Verifies via: `just quality-doctor` check 6 asserts all three files exist and are tracked
  (recipe ships in the same commit, `QF-2026-06-02-15`). Same-day green run confirmed.
- Confidence: H
- Last reviewed: 2026-06-02 (PR Gate Cycle 1 implementation)
- Cycles open: 0 (opened and closed within Cycle 1, same day as gate)
- History:
  - 2026-06-02: Opened (PR review Tier 1; PR Gate Cycle 1 on `d4e3cf3`)
  - 2026-06-02: Closed Done (PR Gate Cycle 1 implementation; Cycle 1)
- Linked PRs / commits: d4e3cf3 (introduced the gap), 74cb465 (closing)
- Drift check: `just quality-doctor` (check 6); see `QF-2026-06-02-15`.
- Standard promoted: *(pending — propose
  `KB/05-engineering/standards/agent-pointer-files.md` or merge into the broader
  `repo-baseline.md` proposed by `QF-2026-06-02-00`)*

### QF-2026-06-02-15

- Date: 2026-06-02
- Bucket: B. Should fix soon
- Area: AI-factory discipline / meta-quality
- Discovered during: PR review (Tier 1)
- Discovered in: PR Quality Gate Cycle 1 (commit `d4e3cf3`)
- Evidence: `AGENTS.md` states "every standard becomes a drift check" yet had no drift check
  for itself. The Snapshot block in `QUALITY_BACKLOG.md` could rot silently as findings
  landed; the `RP-*` table `Tracked by` column required manual maintenance and could drift
  on closure; the Cross-references paths could refer to nonexistent locations without
  signaling intent.
- Impact: The policy compounds only if its assertions stay true; without a drift check both
  files can rot silently.
- Risk if ignored: First gap surfaces during an incident when ledger truth is needed; trust
  in the policy erodes; the Self-Amendment clause in `AGENTS.md` never gets exercised.
- Effort: M
- Owner: Codex
- Status: Done
- Resolution: Added `just quality-doctor` recipe to root `Justfile` with six checks
  accumulating into a `fails` counter (same pattern as `release-preflight`):
  1. `AGENTS.md` and `QUALITY_BACKLOG.md` present and tracked.
  2. Snapshot `Last review` date parsed; failure if absent or older than 14 days.
  3. Every `QF-*` ID grepped from `AGENTS.md` exists in the ledger.
  4. Every `RP-*` row's `Tracked-by` cell references an open finding (parsed from
     `### Bucket [ABCD]` sections, excluding `Accepted Risks`, `Completed Findings`,
     `PR Quality Gates`, `Review Cycles`) or is the dash placeholder `—`.
  5. Cross-references `KB/...` paths in the `## Cross-references` block of
     `QUALITY_BACKLOG.md` either exist on disk or are annotated `Created on first use`
     on the same line.
  6. Root `CLAUDE.md`, `CODEX.md`, `GEMINI.md` present and tracked (also closes
     `QF-2026-06-02-16`).
  Also annotated the four cross-reference paths in `QUALITY_BACKLOG.md` with
  `*(Created on first use)*` so first run passes green. Declared `RP-POLICY-FRESH` in the
  RP table with the `just quality-doctor` recipe as the named enforcement artifact
  (`Status: Aspired (recipe ready; CI not yet wired)`).
- Verifies via: Same-day green run with all 9 checks (sub-items) passing, exit 0. Negative
  cases rely on the same `fails`-accumulator pattern already validated for
  `release-preflight` (dirty tree, unknown project, clean state).
- Codex-safe now: Yes
- Properties: `RP-POLICY-FRESH` (declared by this closure)
- Confidence: H
- Last reviewed: 2026-06-02 (PR Gate Cycle 1 implementation)
- Cycles open: 0 (opened and closed within Cycle 1, same day as gate)
- History:
  - 2026-06-02: Opened (PR review Tier 1; PR Gate Cycle 1 on `d4e3cf3`)
  - 2026-06-02: Closed Done (PR Gate Cycle 1 implementation; Cycle 1; declared
    `RP-POLICY-FRESH`)
- Linked PRs / commits: d4e3cf3 (introduced the gap), 74cb465 (closing)
- Drift check: `just quality-doctor` (Justfile root recipe).
- Standard promoted: *(pending — propose
  `KB/05-engineering/standards/quality-policy-drift-check.md` recording the rule "every
  policy file has a runnable drift check; CI wiring is a follow-up requirement to mark the
  property `Enforced`")*

### QF-2026-06-02-21

- Date: 2026-06-02
- Bucket: A. Must fix now
- Area: release engineering / documentation correctness
- Discovered during: PR review (Tier 2)
- Discovered in: PR Quality Gate Cycle 1 (Justfile release-train sequence)
- Evidence: `Justfile` line 5 (comment header): `Bedrock (axiom → converge → organism → helms)`.
  Line 307 (executable code): `release_order := "converge axiom organism helms ..."`. The two
  sources disagree on the first element.
- Impact: Operator ambiguity about release order in the path where ordering matters most.
- Risk if ignored: Manual release in the wrong order would break 60+ downstream crates per
  `QF-2026-06-02-04`.
- Effort: S
- Owner: Codex
- Status: Done
- Resolution: Verified the bedrock dep graph by Cargo.toml inspection — `axiom` and `converge`
  are independent peers (no inter-bedrock deps); `organism` depends on `converge`; `helms`
  depends on `axiom`, `converge`, `organism`. Replaced the top-of-file Justfile comment with
  an explicit dep-graph block + orchestration rationale: `bedrock := "axiom converge organism
  helms"` is any valid toposort for local check/build; `release_order` places `converge` first
  because its public-API changes have the widest downstream blast radius
  (cites `QF-2026-06-02-04`).
- Verifies via: The Justfile comment block now lists the dep graph explicitly and cites the
  reason for the release-order choice; future drift is caught by `quality-doctor`
  (`QF-2026-06-02-15`) once that lands.
- Confidence: H
- Properties: (closed; previously tracked `RP-SEMVER-GATED`, `RP-RELEASE-TRAIN-INTEGRITY`)
- Last reviewed: 2026-06-02 (PR Gate Cycle 1 implementation)
- Cycles open: 0 (opened and closed within Cycle 1, same day as gate)
- History:
  - 2026-06-02: Opened (PR review Tier 2; PR Gate Cycle 1 on `db6d02e` + `f359a14`)
  - 2026-06-02: Closed Done (PR Gate Cycle 1 implementation; Cycle 1)
- Linked PRs / commits: db6d02e, f359a14, 7d39386 (closing)
- Standard promoted: *(pending — propose
  `KB/05-engineering/standards/release-train-order.md` capturing the rule "publish
  widest-blast-radius package first; document the dep graph alongside the order")*
- Drift check: *(pending — `quality-doctor` rule that asserts the Justfile header dep-graph
  matches `release_order` and that all named projects resolve in `_release-dir`)*

### QF-2026-06-02-20

- Date: 2026-06-02
- Bucket: A. Must fix now
- Area: release engineering / CI/CD reliability
- Discovered during: PR review (Tier 2)
- Discovered in: PR Quality Gate Cycle 1 (Justfile release-train sequence)
- Evidence: `Justfile` lines 337-425, recipe `release-preflight name`. Every `✗` branch
  printed a failure but did not accumulate into an exit code. The recipe defaulted to success.
  `release` line 508 (`just release-preflight {{name}} || exit 1`) therefore never triggered.
  Confirmed at gate time: `just release-preflight converge` against a dirty `bedrock-platform/
  converge` working tree printed `✗ working tree dirty` and exited 0.
- Impact: The release-preflight gate that `AGENTS.md` and the Justfile treated as authoritative
  had no teeth.
- Risk if ignored: First live release fires under bad preconditions; cascade per
  `QF-2026-06-02-04`.
- Effort: S
- Owner: Codex
- Status: Done
- Resolution: Added `fails=0` accumulator at the top of `release-preflight`; converted the
  eight `✗` branches (`branch != main`, `working tree dirty`, `no upstream tracking branch`,
  `could not read version`, `no cargo credentials`, `gh not authenticated`, `cargo check
  failed`, `cargo test failed`) to `fails=$((fails+1))` and an `if/else` shape; added a
  summary line and `exit "$fails"` at the recipe end. Applied the same scaffolding to
  `release-migrate-deps` (no failure branches today; comment notes this is forward
  scaffolding for the `REL_APPLY=1` rewrite path). Annotated both recipes' `set -uo pipefail`
  with a `# permissive: preflight must run every check and accumulate failures` comment so
  that `QF-2026-06-02-22` work preserves the exception.
- Verifies via: Three same-day sanity runs of `just release-preflight`:
  - Dirty tree (`bedrock-platform/converge` modified) → `✗ preflight: 1 check(s) failed`,
    exit 1.
  - Unknown project (`just release-preflight nonexistent`) → fast-fail, exit 1.
  - Clean tree (after `git stash -u` in `bedrock-platform/converge`) →
    `✓ preflight: all checks passed`, exit 0.
- Confidence: H
- Properties: (closed; previously tracked `RP-SEMVER-GATED`, `RP-RELEASE-TRAIN-INTEGRITY`)
- Business leverage: Closing this is the precondition for any future `REL_APPLY=1` work;
  without it the live release path could not be safely enabled.
- Last reviewed: 2026-06-02 (PR Gate Cycle 1 implementation)
- Cycles open: 0 (opened and closed within Cycle 1, same day as gate)
- History:
  - 2026-06-02: Opened (PR review Tier 2; PR Gate Cycle 1 on `f359a14`)
  - 2026-06-02: Closed Done (PR Gate Cycle 1 implementation; Cycle 1)
- Linked PRs / commits: f359a14, 7d39386 (closing)
- Standard promoted: *(pending — propose
  `KB/05-engineering/standards/release-preflight-discipline.md`: every check in a
  preflight-style recipe accumulates failures and the recipe exits with the failure count;
  short-circuit only on truly fatal conditions (e.g., project not found))*
- Drift check: *(pending — a shell test under `xtask/` or a Justfile `release-preflight-test`
  recipe that runs preflight in known-bad states (dirty tree, missing creds, wrong branch)
  and asserts non-zero exit, plus one clean-state assertion of exit 0)*

### QF-2026-06-02-00

- Date: 2026-06-02
- Bucket: C. Strategic improvement
- Area: Platform extensibility
- Evidence: Initial inspection found no tracked root `AGENTS.md` or
  `QUALITY_BACKLOG.md`; `.gitignore` only allowed a small root file set plus
  `KB/`.
- Impact: Codex had no repo-local standing instruction to act as a recurring
  engineering auditor and no durable place to track quality findings.
- Resolution: Added root `AGENTS.md`, added this `QUALITY_BACKLOG.md`, and
  updated `.gitignore` so both files can be tracked.
- Owner: Codex
- Status: Done
- Confidence: H
- Last reviewed: 2026-06-02 (Cycle 1)
- Cycles open: 0 (opened and closed in Cycle 1)
- History:
  - 2026-06-02: Opened (audit; Cycle 1)
  - 2026-06-02: Closed Done (Cycle 1)
- Standard promoted: *(pending — propose
  `KB/05-engineering/standards/repo-baseline.md` covering the required root
  files for any new workspace repo)*
- Drift check: *(pending — propose a CI or `just` recipe that asserts
  presence of `AGENTS.md`, `QUALITY_BACKLOG.md`, and a `KB/` entrypoint in
  tracked repos)*

## PR Quality Gates

Append one entry per `PR Quality Gate` run (per `AGENTS.md > PR Quality Gate`). Distinct from
Review Cycles below: gates run per change-set; reviews run on a recurring cadence. Cross-link
both ways when a gate cycle and a review cycle overlap.

### PR Gate Cycle 1 — 2026-06-02

- Reviewer: Claude Code (paired with Karl)
- Scope: three change-sets gated sequentially in a single calibration session.
- Tier breakdown:
  - **Tier 1** — commit `d4e3cf3` (continuous quality factory guidance: root `AGENTS.md` +
    `QUALITY_BACKLOG.md` + `.gitignore`).
  - **Tier 0** — uncommitted KB updates (8 modified files + new
    `KB/04-architecture/runtime-injection-boundaries.md`).
  - **Tier 2** — Justfile release-train sequence (`db6d02e`, `5dc66e9`, `ca476ad`, `65f0889`,
    `f359a14`); full quality gate plus structured 6-month pre-mortem.
- Blockers identified: 2 (both in the Tier 2 gate).
  - `QF-2026-06-02-20` — `release-preflight` exit code does not reflect ✗ markers.
  - `QF-2026-06-02-21` — Justfile comment vs. `release_order` disagree on the first element.
- Non-blocking concerns: 11 (across the three tiers).
- New findings opened: 13 total.
  - Bucket A: `QF-2026-06-02-20`, `QF-2026-06-02-21`.
  - Bucket B: `QF-2026-06-02-15`, `QF-2026-06-02-16`, `QF-2026-06-02-17`,
    `QF-2026-06-02-22`, `QF-2026-06-02-24`, `QF-2026-06-02-25`, `QF-2026-06-02-27`.
  - Bucket C: `QF-2026-06-02-18`, `QF-2026-06-02-19`, `QF-2026-06-02-23`.
  - Bucket D: `QF-2026-06-02-26`.
- Findings closed: 0 (this gate did not close any pre-existing finding; existing closure of
  `QF-2026-06-02-00` belongs to the baseline Cycle 1 record below).
- RP `Tracked by` updated: `RP-SEMVER-GATED` (+`-20`, `-21`),
  `RP-RELEASE-TRAIN-INTEGRITY` (+`-20`, `-21`, `-23`), `RP-YANK-DISCOVERABLE` (+`-24`).
- Candidate RPs flagged (not yet declared): `RP-POLICY-FRESH` (`QF-...-15`),
  `RP-TYPED-CROSS-LAYER-SEMANTICS` (`QF-...-19`).
- ADRs flagged for future cycles: `0002-rel-apply-secrets-discipline.md` (`QF-...-26`); the
  earlier `0001-commerce-rails-publish-status.md` (`QF-...-14`) remains outstanding.
- Snapshot delta: open findings 14 → 27 (A: 4 → 6, B: 6 → 13, C: 3 → 6, D: 1 → 2).
- Implementation in this gate cycle: review only; no code shipped. The blockers and B-tier
  fixes feed the next implementation cycle.
- Pre-mortem insight (Tier 2): five of six predicted 6-month incidents restate existing ledger
  findings (`-04`, `-08`, `-10`, `-13`). The release scaffold is the surface where those
  findings will materialize in practice — closing them depends on this Justfile being
  trustworthy. The remaining incident (`REL_APPLY=1` accidentally landing in `.env`) is now
  tracked as the D-tier `QF-2026-06-02-26`.
- Calibration notes: gate exposed two gaps in the policy itself that the policy's own
  Self-Amendment clause anticipates: (a) `QF-...-15` (no drift check for `AGENTS.md` and
  `QUALITY_BACKLOG.md`) and (b) `QF-...-16` (no root `CLAUDE.md`/`CODEX.md` for auto-discovery).
  Both should land before the next review cycle.

### PR Gate Cycle 1 — 2026-06-02 (implementation follow-up)

Same-day human-authorized closure of the two Tier 2 blockers surfaced above.

- Closed: `QF-2026-06-02-20` (`release-preflight` exit code now reflects ✗ markers) and
  `QF-2026-06-02-21` (Justfile dep-graph + release-order comment now matches `release_order`
  and cites `QF-2026-06-02-04`).
- Verification: three same-day `just release-preflight` runs — dirty tree → exit 1; unknown
  project → exit 1; clean tree (converge stashed) → exit 0. All as expected.
- Authorization path: explicit user request ("close the two release-path blockers"). Per
  `AGENTS.md > Autonomy Contract`, release-process changes require human review; this is
  recorded as the authorization.
- RP `Tracked by` demoted: `RP-SEMVER-GATED` (−`-20`, −`-21`),
  `RP-RELEASE-TRAIN-INTEGRITY` (−`-20`, −`-21`). Neither property is yet `Enforced` — the
  closure work moved them a notch closer but the drift checks (see Standard promoted / Drift
  check pointers on both closed entries) are still pending.
- New findings created during implementation: 0.
- Standards promoted: 0 (two pending, recorded on the closed entries).
- Snapshot delta: open findings 27 → 25 (A: 6 → 4); closed since last review 1 → 3.
- Open after closure: A=4 (`QF-...-04, -05, -07, -08`), B=13, C=6, D=2. Next implementation
  cycle's leading candidates remain the two policy-gap fixes (`QF-...-15`, `QF-...-16`).

### PR Gate Cycle 1 — 2026-06-02 (implementation follow-up, round 2)

Same-day human-authorized closure of the two policy-gap B-tier findings flagged in the gate's
Calibration notes.

- Closed: `QF-2026-06-02-15` (`just quality-doctor` drift recipe for the quality factory) and
  `QF-2026-06-02-16` (root `CLAUDE.md` / `CODEX.md` / `GEMINI.md` agent-pointer files).
- New property declared: `RP-POLICY-FRESH` — first property in the RP table whose enforcement
  artifact (`just quality-doctor`) actually exists. Status `Aspired (recipe ready; CI not yet
  wired)` until a CI runner invokes the recipe on every PR.
- Cross-references in `QUALITY_BACKLOG.md` annotated *(Created on first use)* on the four
  forward-looking `KB/0N-*` paths so `quality-doctor` check 5 passes on first run rather than
  starting red.
- Authorization path: explicit user request ("close QF-2026-06-02-15 and QF-2026-06-02-16").
- Verification: same-day green run of `just quality-doctor` — 9 checks pass, exit 0
  (`AGENTS.md` and `QUALITY_BACKLOG.md` tracked; Snapshot Last review = today, 0 days;
  every cited QF-* exists in ledger; every RP-* Tracked-by valid; cross-references
  annotated; `CLAUDE.md`, `CODEX.md`, `GEMINI.md` all present and tracked).
- New findings created during implementation: 0.
- Standards promoted: 0 (two new pending follow-ups recorded on the closed entries).
- Snapshot delta: open findings 25 → 23 (B: 13 → 11); closed since last review 3 → 5;
  RP-* tracked 14 → 15.
- Open after closure: A=4, B=11, C=6, D=2. Next leading candidates: any of the four open
  A-tier items (`QF-...-04, -05, -07, -08`) — the inherited release-engineering and
  AI-factory blockers from the pre-policy era.

### PR Gate Cycle 1 — 2026-06-02 (implementation follow-up, round 3)

Same-day human-authorized closure of the remaining "policy gap" B-tier finding.

- Closed: `QF-2026-06-02-17` (`just snapshot` recipe regenerating the eight derivable
  Snapshot bullets from observable state; `quality-doctor` check 7 invokes it and fails on
  drift).
- Format refactor in `## Snapshot`: two bullets (`Accepted risks open`, `ADRs added since
  last review`) had `(see KB/…)` parentheticals embedded inside the label prefix, which
  blocked `quality-doctor`'s label-anchored matching. Refactored to put the reference after
  the count value where it lives as commentary.
- Implementation found two latent bugs in `quality-doctor` check 7 during development:
  (a) BSD `grep` consumed the leading `- ` of the label as flags — fixed with `--` separator;
  (b) parenthetical-inside-label problem above. Both fixed before commit.
- Authorization path: explicit user request ("close QF-2026-06-02-17").
- Verification: same-day green run of `just quality-doctor` — all 10 checks pass, exit 0
  (the 9 from earlier rounds plus check 7). `just snapshot` produces deterministic output:
  `- Open findings: **22** (A: 4, B: 10, C: 6, D: 2)` and seven more bullets, all matching
  the file.
- New findings created during implementation: 0.
- Standards promoted: 0 (the closed entry folds its standard into the
  `quality-policy-drift-check.md` proposed by `QF-2026-06-02-15`).
- Snapshot delta: open findings 23 → 22 (B: 11 → 10); closed since last review 5 → 6;
  RP-* tracked unchanged (15); RP-* green unchanged (0).
- Open after closure: A=4, B=10, C=6, D=2. Next leading candidates remain the four open
  A-tier items.

### PR Gate Cycle 1 — 2026-06-02 (implementation follow-up, round 4)

Same-day autonomous closure of the only Codex-safe A-tier finding rooted in mechanical
Cargo.toml drift.

- Closed: `QF-2026-06-02-08` part (a) — pre-publish lint for path-deps without explicit
  version. Standalone `just release-deps-audit name` recipe added; wired into
  `release-preflight` as new section 5.5.
- Part (b) deferred: layering lint that rejects publishable → `publish = false` edges
  remains under `QF-2026-06-02-14` (commerce-rails publish-status decision). The lint is
  mechanical work that falls out once `QF-14` resolves; no separate finding was created.
- RP `Tracked-by` demoted: `RP-LAYERING` (−`-08`); `QF-14` continues to track the property.
- Cross-train sanity verified on five workspaces in dep-graph order (`converge → axiom →
  organism → helms` for bedrock, then `runway`):
  - `converge`, `axiom` — 0 issues each, exit 0 (recently released; clean).
  - `organism` — 2 dev-dep stragglers (`organism-runtime ↔ organism-catalog-seed`).
  - `helms` — 94 stragglers (unreleased workspace; expected — pinning is a release-prep step).
  - `runway` — 2 stragglers (`runway-storage → runway-storage-contract` dev,
    `converge-application → converge-llm` normal).
  Each non-zero result correctly propagated to `release-preflight`'s `fails` counter
  (verified end-to-end on `runway`: preflight ends `── ✗ preflight: 1 check(s) failed ──`).
- Authorization path: explicit user request ("close QF-08 autonomously"). Per `AGENTS.md >
  Autonomy Contract`, release-process changes require human review; recorded here.
- New findings created during implementation: 0. The dep-graph traversal *did* surface 98
  outstanding path-without-version pins in `organism`, `helms`, and `runway`. These are not
  filed as separate findings because they are the *symptoms* `QF-08` exists to detect; the
  natural home for fixing them is each affected workspace's own release prep, which the new
  recipe now automates.
- Standards promoted: 0 (one new pending follow-up recorded on the closed entry — proposed
  `KB/05-engineering/standards/path-dep-versioning.md`).
- Snapshot delta: open findings 22 → 21 (A: 4 → 3); closed since last review 6 → 7;
  RP-* tracked unchanged (15); RP-* green unchanged (0).
- Open after closure: A=3, B=10, C=6, D=2. Three A-tier items remain
  (`QF-...-04`, `-05`, `-07`).

### PR Gate Cycle 1 — 2026-06-02 (implementation follow-up, round 5)

Same-day autonomous closure of the two cleanup B-tier findings flagged as the next
"trivial cleanups that compound the discipline without new surface area" candidates.

- Closed: `QF-2026-06-02-22` (`set -uo` → `-euo` normalization with explicit
  `# permissive: <reason>` annotations where loose semantics are intentional) and
  `QF-2026-06-02-27` (tag-exists check now probes local + remote distinctly).
- Audit pass across every `set -uo pipefail` site in `Justfile`: 6 recipes switched to
  `set -euo pipefail` (`_dispatch`, `release-preflight-all`, `release`, `release-all`,
  `snapshot`, `status`); 3 recipes kept `-uo` with new `# permissive:` annotations
  (`release-deps-audit`, `release-migrate-deps`, `doctor`); 3 recipes unchanged because
  they already had the annotation (`release-preflight`, `quality-doctor`,
  `project-doctor`).
- Tag-check rewrite in `release-preflight` section 3: prepends `git fetch --tags --quiet`,
  probes local with `rev-parse` and remote with `ls-remote refs/tags/v$ver`, reports four
  distinct states. Informational only — no new ✗ failure mode (the original behavior was
  also informational); the question of "does v<next> already exist?" remains the release
  recipe's concern, not preflight's.
- Authorization path: explicit user request ("close QF-22 and QF-27").
- Verification: `release-preflight converge` now shows `tag v3.9.2 exists (local + remote)`
  instead of `tag v3.9.2 already exists locally`; `status`, `release-preflight`,
  `quality-doctor`, `release converge` all run end-to-end and propagate failures as
  before; `grep -nE 'set -uo' Justfile` returns exactly 6 lines, each preceded by a
  permissive annotation block.
- New findings created during implementation: 0. The "drift check for `# permissive:`
  annotation discipline" idea (quality-doctor could assert every `set -uo` line is
  preceded by an annotation within N lines) is noted on the QF-22 closed entry as a
  Bucket C-style follow-up; not filed separately because it is a refinement of the
  existing quality-doctor checks rather than a new property.
- Standards promoted: 0 (two new pending follow-ups, both folding into
  `KB/05-engineering/standards/release-preflight-discipline.md` proposed by `QF-...-20`).
- Snapshot delta: open findings 21 → 19 (B: 10 → 8); closed since last review 7 → 9;
  RP-* tracked unchanged (15); RP-* green unchanged (0).
- Open after closure: A=3, B=8, C=6, D=2. Cleanup tier still has plenty of B candidates;
  next leading targets are `QF-...-25` (cargo-metadata-based downstream detection — would
  also tighten `release-preflight` section 7) and `QF-...-09` (pre-publish crate-size
  budget — `project-doctor` check 3 partially covers this).

## Review Cycles

### Cycle 3 — 2026-07-02

- **Self-audit:** all 13 open findings re-verified against live state.
  Closed: `QF-2026-06-08-09` (commerce-rails CI complete + green),
  `QF-2026-06-02-14` (publishable decision implemented; ADR backfilled).
  Demoted: `QF-2026-06-26-01` D→C (decision executed through slice 2).
  History refreshed on `-08-07`, `-08-08` (CI bootstrap in flight on `next`,
  red) and `-07-02` (GTK/HiGHS landed; failure needs re-diagnosis).
- **Risk register:** RR-2026-06-07-01 revisit 2026-07-07 — not yet due.
- **Drift scan:** agents-doctor green fleet-wide (new this cycle);
  milestone-definition-of-done names a nonexistent check (`-02-04`);
  hermetic-audit workflow red (`-02-05`); RP-CI-PARITY rollout still 4/14
  thin-runner (tracked `-08-11`); fleet-status: FACTORY RED, 11 repos with a
  failing workflow on main (mostly Stability/Security lanes + the in-flight
  helms/arena/atelier bootstraps).
- **New findings:** `-02-06` fleet branch-hygiene/stranded-work debt,
  `-02-07` security-audit ignores lack expiry, `-02-04`, `-02-05` (above).
  A parallel session opened `-02-02` (helms→atelier path deps — overlaps
  `QF-2026-06-12-01`; reconcile next cycle) and `-02-03` (fmt-check sibling
  coupling) the same hour; IDs -02/-03 are theirs, this review renumbered
  its own entries to -06/-07.
- **Implemented this cycle (Autonomy Contract):** arena-tests stray
  `error.log` with account identifier deleted; ADR
  `2026-07-02-commerce-rails-publishable.md` backfilled; scorecard Cycle 3;
  Snapshot refreshed (clears the doctor stale-review red).
- **Scorecard delta vs Cycle 2:** open 15→18 (quality of accounting, not
  degradation: 4 stale entries closed/demoted while 4 real new debts
  surfaced); RP Enforced 8→15; standards 3→10; ADRs 4→7; CI green 1/1→2/5
  (more gates exist; two are red and now tracked).

Append one entry per recurring health review. The point is the trend.

### Cycle 2 — 2026-06-07 (first real revolution)

- Reviewer: Karl + Claude Opus 4.7 + Codex (collaborative, asynchronous over 2026-06-03 → 2026-06-07)
- Self-audit (step 1): 27 findings opened in Cycle 1 walked. 21 closed (-00,
  -01, -04, -06, -07, -08, -10, -13, -15, -16, -17, -20, -21, -22, -23, -24,
  -27, -28, -29, -30, -03-01-Wont-do). 15 remain open across A/B/C/D. None
  re-promoted or superseded this cycle; all closures cite a concrete
  artifact in `Linked PRs / commits`.
- Risk register sweep (step 2): 1 `RR-*` entry (`RR-2026-06-07-01`, Tauri
  GTK3 glib-0.18.5, accepted 2026-06-07, revisit 2026-07-07). Revisit date
  is 30 days out — no action this cycle. Risk register file
  (`KB/06-operations/risk-register.md`) bootstrapped this cycle from the
  `QF-2026-06-06-01` audit closure.
- Drift scan (step 3): `just doctor` runs 16 numbered checks (10
  quality-doctor + 6 project-doctor). Local + CI green at end of cycle.
  Run history: 2026-06-04 ed44065 first green, 2026-06-07 5403cfe latest
  green. No drift findings opened by the scan itself this cycle.
- New findings (step 4): 6 opened, 5 already closed.
  - Opened B: `QF-2026-06-06-02` (audit follow-up — open),
    `QF-2026-06-07-02` (FRESH-CLONE-GREEN scheduled CI matrix — open)
  - Opened C: `QF-2026-06-07-01` (audit follow-up — open).
  - All within Codex / Claude autonomy contract; no Bucket A surprises.
- Scorecard (step 5): `KB/06-operations/factory-scorecard.md` initialized
  this cycle. Cycle 2 row: open 15 (1/5/7/2), closed Δ +21, RP enforced
  8 / 14, risks 1 / 0 due, standards 3 / ADRs 4 cumulative, CI 1/1 green.
  See file for trend lens.
- Implemented this cycle (step 6):
  - `KB/05-engineering/standards/{doctor-recipe-pattern,publish-status-declaration,rustc-pinning}.md` (this commit)
  - `just release-undo <crate> <version> [reason]` recipe (closes QF-24)
  - `just project-doctor` check 6 (`RP-YANK-DISCOVERABLE` structural lint)
  - `just project-doctor` check 3 hardened to use `git ls-files` (no longer
    false-positives on gitignored caches like `.fastembed_cache`)
  - `just check-all-fresh` recipe (Codex; closes QF-13, mechanizes
    `RP-FRESH-CLONE-GREEN` operator half)
  - `just release-history-audit` recipe (Codex; crates.io cross-reference
    for release-history.md entries)
  - `KB/release-history.md` runbook + structural schema + 2 backfilled
    historical yanks
  - `KB/06-operations/risk-register.md` bootstrapped + `RR-2026-06-07-01`
  - `KB/06-operations/factory-scorecard.md` initialized
  - ADRs added: `2026-06-02-converge-runtime-retirement`,
    `2026-06-06-applet-runtime-boundaries`,
    `2026-06-07-tauri-gtk3-glib-risk` (3 of the 4 cumulative;
    `2026-05-23-runway-config-injection` predates this cycle)
- Standards promotion (step 7): 3 promoted from closed findings:
  - `doctor-recipe-pattern.md` from QF-28
  - `publish-status-declaration.md` from QF-29
  - `rustc-pinning.md` from QF-30
  - 9 pending follow-ups remain (`QF-2026-06-02-01`, `-08`, `-15`, `-16`,
    `-17`, `-20`, `-21`, `-22`, `-27`) — schedule for Cycle 3.
- Scorecard delta vs Cycle 1: RP enforced 0 → 8, standards 0 → 3,
  ADRs 0 → 4, CI green 0 → 1. The flywheel turned for the first time this
  cycle.
- Notes: Cycle 2 spanned 5 days (2026-06-03 → 2026-06-07) because three
  external constraints landed mid-cycle — GitHub org upgrade Free → Team
  (to unlock fine-grained PAT policy for private repos), then 4 train
  repos flipped to public to bypass that policy, then mosaic-extensions's
  nested-git structure required 8 sibling checkouts in `doctor.yml`. Each
  of these is a cycle artifact worth flagging: future "one Reflective-Lab
  org with mixed visibility + nested gits" decisions should remember the
  cost. Next cycle should (a) decide the residual 8 unmechanized RPs
  (`RP-SEMVER-GATED`, `RP-HERMETIC-UNIT`, `RP-DETERMINISM`,
  `RP-AUTO-BLESS-AUDITED`, `RP-TEST-CODE-ATTRIBUTION`,
  `RP-AI-EVIDENCE-CITED`, `RP-AI-SHORTCUT-DECLARED`), (b) drain the
  9 pending standards-from-closures, (c) close the residual yank reverse-
  discovery question (open finding or `Won't do`), (d) move from "single-
  developer flywheel" toward the multi-agent learning capture surface
  Karl asked about (see Cycle 2 thread: agents learning / definition of
  done / proof of delivery).

### Cycle 1 — 2026-06-02 (baseline)

- Reviewer: Codex (initial workspace audit + release rehearsal post-mortem)
- Self-audit: n/a (first cycle)
- Risk register sweep: n/a (no register yet)
- Drift scan: n/a (no automated drift checks yet — 14 Recurring System
  Properties declared, all `Aspired`, none enforced)
- New findings:
  - Closed: `QF-2026-06-02-00`
  - Opened A: `QF-2026-06-02-04`, `QF-2026-06-02-05` (in progress),
    `QF-2026-06-02-07`, `QF-2026-06-02-08`
  - Opened B: `QF-2026-06-02-01`, `QF-2026-06-02-02`, `QF-2026-06-02-06`,
    `QF-2026-06-02-09`, `QF-2026-06-02-10`, `QF-2026-06-02-11`
  - Opened C: `QF-2026-06-02-03`, `QF-2026-06-02-12`, `QF-2026-06-02-13`
  - Opened D: `QF-2026-06-02-14`
- Implemented this cycle: root `AGENTS.md`, root `QUALITY_BACKLOG.md`,
  `.gitignore` update, axiom v0.15.2 migration of `guide_heading`
  (RP-HERMETIC-UNIT progress on `QF-2026-06-02-05`), trybuild fixture
  repair in `bedrock-platform/organism` commit `3e1a7c8`
  (RP-SNAPSHOT-PORTABLE progress on `QF-2026-06-02-06`).
- Standards promoted: 0 (two pending follow-ups noted on
  `QF-2026-06-02-00`).
- ADRs added: 0 (one pending on `QF-2026-06-02-14`).
- Scorecard delta: baseline established; no prior cycle to compare.
- Notes: Cycle 1 also instantiated the 14-property `RP-*` invariant table.
  Next cycle should (a) populate Standard and Drift Check pointers on
  `QF-2026-06-02-00`, (b) draft the ADR for `QF-2026-06-02-14`, (c) run the
  first real drift scan across all 14 RP properties, (d) initialize
  `KB/06-operations/factory-scorecard.md`, and (e) close the two policy-gap
  blockers surfaced by PR Gate Cycle 1 (`QF-2026-06-02-15` quality-doctor,
  `QF-2026-06-02-16` root agent pointer files). See *PR Gate Cycle 1* above
  for the same-day Quality Gate that opened `QF-2026-06-02-15` through
  `QF-2026-06-02-27`.
