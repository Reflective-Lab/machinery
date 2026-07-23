---
source: llm
---

# CI/CD inventory — 16-repo train, 2026-06-08

Baseline survey for `QF-2026-06-02-03` (security coverage inventory)
+ `QF-2026-06-02-11` (pre-commit hook / quality-gate inventory). One
document because both findings ask the same underlying question:
**which gates run where, and where do they drift?**

Purpose: surface the drift that today makes "verify locally, know it
goes through" unreliable. Precondition for the standardization work
under `RP-CI-PARITY` (proposed; see below).

## Method

Captured per repo:
- Files in `.github/workflows/`.
- Top-level `Justfile` recipe names.
- Recipe bodies for: `check`, `test`, `lint`, `fmt-check`, `security-audit`.
- Presence of `.github/dependabot.yml` and `deny.toml`.

Not captured (deferred):
- Pre-commit hook contents per repo's `.git/hooks/`.
- Full workflow trigger semantics, only the obvious patterns.
- Cargo-audit ignore lists per repo (drift exists but is a separate
  finding class).

## The 16 (+ container)

| # | Repo | CI workflows | Justfile recipes | dependabot | deny |
|---:|---|---|---:|:---:|:---:|
| 1 | `bedrock-platform/converge` | 6 (ci, coverage, security, stability, release, dependabot-automerge) | 40 | ✓ | ✓ |
| 2 | `bedrock-platform/axiom` | 7 (+ dependency-analysis) | 23 | ✓ | ✓ |
| 3 | `bedrock-platform/organism` | 5 (no stability) | 34 | ✓ | ✓ |
| 4 | `bedrock-platform/helms` | **NONE** | 24 | ✗ | ✗ |
| 5 | `mosaic-extensions/` (container) | NONE (expected) | 4 | n/a | n/a |
| 6 | `mosaic-extensions/arbiter-policy` | 4 (canonical mosaic) | 21 | ✓ | ✓ |
| 7 | `mosaic-extensions/crucible-models` | 4 | 14 | ✗ | ✓ |
| 8 | `mosaic-extensions/embassy-ports` | 4 | 21 | ✗ | ✓ |
| 9 | `mosaic-extensions/ferrox-solvers` | 6 (+ docker, dependabot-automerge) | 41 | ✓ | ✓ |
| 10 | `mosaic-extensions/manifold-adapters` | 4 | 23 | ✗ | ✓ |
| 11 | `mosaic-extensions/mnemos-knowledge` | 4 | 22 | ✓ | ✓ |
| 12 | `mosaic-extensions/prism-analytics` | 4 | 22 | ✓ | ✓ |
| 13 | `mosaic-extensions/soter-smt` | 4 | 22 | ✗ | ✗ |
| 14 | `atelier-showcase` | 4 | 42 | ✓ | ✓ |
| 15 | `arena-tests` | **NONE** | 10 | ✗ | ✗ |
| 16 | `runtime-runway` | 5 (ci, security, release, contract, contract-staging) | 49 | ✓ | ✗ |
| 17 | `commerce-rails` | **NONE** | 6 | ✗ | ✗ |

## Gap class A — repos with NO CI at all

Three repos have local Justfile recipes but no GitHub Actions
workflows:

- **`bedrock-platform/helms`** — 24 recipes, no CI. The local
  Justfile is desktop-focused (`build-desktop`, `dev-desktop-*`,
  `desktop-rust-fmt`, etc.) and lacks even `check`, `lint`,
  `security-audit`, `coverage` recipes; the recipe shape itself is an
  outlier in the train.
- **`arena-tests`** — 10 recipes (`report`, `build`, `test`,
  `lint`, `fmt-check`, `fmt`), no CI. Tests run only when an operator
  remembers.
- **`commerce-rails`** — 6 recipes (`check`, `test`, `fmt`,
  `fmt-check`, `lint`), no CI. The commercial-authority workspace has
  no PR safety net.

These three are the highest-priority migration targets — any PR is
shipping un-CI-validated code.

## Gap class B — Justfile recipe DOES different things in different repos

The same recipe name does dramatically different work depending on
which repo you `cd` into. Five samples:

| Recipe | converge | mosaic (prism) | atelier | runtime-runway | commerce-rails |
|---|---|---|---|---|---|
| `just check` | `cargo check --workspace` | `cargo check --workspace --all-targets` | `cargo check --workspace` | `cargo check --workspace` | `cargo check` |
| `just test` | `cargo test --all-targets` | `cargo test --workspace --all-targets` | `cargo test --workspace --all-targets` | `cargo test --all-targets` | `cargo test` |
| `just lint` | `cargo fmt --check` + `cargo clippy --all-targets -- -D warnings` | (split into `clippy` + `fmt-check`) | `cargo fmt --check` + `cargo clippy --workspace --all-targets -- -D warnings` | `cargo fmt --check` + `cargo clippy --all-targets -- -D warnings` | `cargo fmt --check` + `cargo clippy --all-targets -- -D warnings` |
| `just fmt-check` | (none — bundled in `lint`) | `cargo fmt --all -- --check` | (none — bundled in `lint`) | (none — bundled in `lint`) | `cargo fmt --check` |
| `just security-audit` | bash script with custom output dir | `cargo audit --deny warnings --ignore ...` | bash script with custom output dir | (none) | (none) |

Observations:

1. **`--workspace` is not universal.** Foundation repos and
   runtime-runway omit it on `cargo test`; commerce-rails omits it
   everywhere. An operator who runs `just check` in commerce-rails is
   checking a single package; in mosaic-extensions they're checking
   the whole workspace.
2. **`--all-targets` is not universal.** Only mosaic repos consistently
   pass `--all-targets` to `cargo check`. Foundation and atelier check
   src but not tests/benches/examples.
3. **`lint` is overloaded.** In some repos it's `fmt + clippy`; in
   others (mosaic) `fmt-check` and `clippy` are split recipes and
   `lint` is a third aggregator. Operators can't reason about what
   `just lint` runs without reading the Justfile each time.
4. **`security-audit` is non-existent in 3 of the train.** No local
   way to reproduce CI's `cargo audit` step in
   `bedrock-platform/helms`, `runtime-runway`, `arena-tests`,
   `commerce-rails`.

## Gap class C — CI runs things the Justfile doesn't expose

Mosaic sub-extensions' CI runs `cargo audit --deny warnings --ignore
RUSTSEC-...` with a curated ignore list (9 IDs for prism-analytics
sampled). Foundation runs a bash-wrapped audit script with custom
output paths. The two `cargo audit` invocations are not
interchangeable — they have different `--ignore` lists, different
output structures, different fail conditions.

CI also runs `gitleaks` (secret scanning) in mosaic security.yml.
**No Justfile recipe runs gitleaks.** An operator cannot reproduce
the secrets scan locally — they discover failures only at PR time.

Foundation `coverage.yml` and atelier `coverage.yml` use different
ignore patterns for `examples/` vs `tutorials/` vs `scenarios/` —
the 2026-06-08 atelier CI fix (`QF-2026-06-08-04`) was exactly this
class.

## Gap class D — repos that lack `cargo-deny` config

Five repos have CI workflows that may want to run `cargo deny` but
have no `deny.toml`:
- `bedrock-platform/helms` (no CI either)
- `mosaic-extensions/soter-smt`
- `mosaic-extensions/` (container)
- `arena-tests` (no CI either)
- `commerce-rails` (no CI either)
- `runtime-runway` (HAS CI but no deny.toml)

The `runtime-runway` case is the loudest — production runtime, has
CI, has security.yml, but missing dependency-license + advisory
config that the other train members rely on.

## Gap class E — dependabot.yml coverage

Eight repos lack `.github/dependabot.yml`:
- `bedrock-platform/helms`
- `mosaic-extensions/` (container — expected)
- `mosaic-extensions/crucible-models`
- `mosaic-extensions/embassy-ports`
- `mosaic-extensions/manifold-adapters`
- `mosaic-extensions/soter-smt`
- `arena-tests`
- `commerce-rails`

These repos get no automated dependency-update PRs. Drift accumulates
silently.

## What "verify locally, know it goes through" actually needs

The drift cannot be fixed by a single edit. The path forward, in
order:

### 1. Canonical `just ci` recipe (the recommendation)

Every train repo implements one recipe — `just ci` — that runs
EVERYTHING its CI workflow runs, in the same order, with the same
flags. The recipe is the single source of truth for "what's tested."
CI workflow becomes: install system deps, install Rust, install
sibling checkouts, run `just ci`.

A concrete canonical shape:

```
ci: fmt-check check lint test security-audit

fmt-check:
    cargo fmt --all -- --check

check:
    cargo check --workspace --all-targets

lint:
    cargo clippy --workspace --all-targets -- -D warnings

test:
    cargo test --workspace --all-targets

security-audit:
    cargo audit --deny warnings $(< .audit-ignores)
```

Single `--workspace --all-targets` discipline. `cargo-audit` ignores
live in a separate file (`.audit-ignores`) so drift between repos is
trackable.

### 2. Canonical CI workflow

Once `just ci` exists everywhere, CI workflows shrink to ~30 lines:

```yaml
- uses: actions/checkout@v6
- uses: dtolnay/rust-toolchain@stable
  with: { toolchain: "1.96.0" }
- uses: Swatinem/rust-cache@v2
- uses: extractions/setup-just@v3
- run: bash scripts/ci/checkout-reflective-siblings.sh  # if applicable
- run: just ci
```

Everything else (coverage, stability, security depth) lives in
separate workflows that also call `just <recipe>` from the same
Justfile.

### 3. Migration order

Smallest-blast-radius first:
1. `commerce-rails` — 6 recipes, no CI. Build the canonical shape
   here; the workspace is small.
2. `arena-tests` — 10 recipes, no CI. Apply the canonical shape.
3. `bedrock-platform/helms` — 24 recipes, no CI, but desktop-focused
   Justfile needs adapter recipes (`just ci` calls into existing
   `desktop-check`, etc.).
4. `mosaic-extensions/*` — already close to canonical; need
   `fmt-check` / `clippy` consolidated under `just ci` and the
   `--workspace --all-targets` discipline.
5. `bedrock-platform/{converge, axiom, organism}` — foundation
   repos need migration of bundled `lint` recipe + `--all-targets`
   addition.
6. `runtime-runway` — has unique infra recipes; `just ci` builds
   on top of the existing structure without disrupting `firebase-*`
   / `gcp-*` / `infra-*` recipes.
7. `atelier-showcase` — closest to canonical; cleanup pass.

### 4. New `RP-CI-PARITY` property

> Every train repo's CI workflow consists of a thin runner around
> `just ci`. Running `just ci` locally gives the same verdict as CI
> on the same code.

Enforcement: a doctor check that walks each train repo's
`.github/workflows/ci.yml` and asserts it only invokes `just ci`
(plus toolchain / cache / sibling-checkout setup). Drift fails CI.

## Specific drift fixes recommended now (before canonicalization)

Three findings that should be filed regardless of when the canonical
work happens:

1. **`bedrock-platform/helms` has no CI** — every PR is shipping
   unvalidated. File a Bucket B finding.
2. **`arena-tests` has no CI** — same shape as helms.
3. **`commerce-rails` has no CI** — commercial-authority workspace
   with no PR safety net. File as Bucket B.

These are not blocked on canonical-shape work — adding ANY
GitHub Actions CI is better than none. They can be backfilled with
existing patterns (copy from a healthy mosaic repo) and migrated to
canonical later.

## Cross-references

- `QUALITY_BACKLOG.md` — `QF-2026-06-02-03` (security inventory) and
  `QF-2026-06-02-11` (hook inventory). Both findings are answered by
  this document; closure follow-ups likely separate per gap class.
- `KB/06-operations/factory-health.md` — root-repo doctor recipes
  that this document's CI-parity work would extend to the train.
- `KB/05-engineering/standards/hermetic-unit-tests.md` — the
  `RP-HERMETIC-UNIT` per-repo migration is the playbook this CI
  canonicalization would replay.
- `KB/05-engineering/standards/doctor-recipe-pattern.md` — the
  pattern future `RP-CI-PARITY` doctor checks would follow.
