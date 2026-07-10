---
source: llm
---

# Standard: CI parity (`RP-CI-PARITY`)

**Property:** `RP-CI-PARITY` in `QUALITY_BACKLOG.md`.

**Status:** Enforced (convention + standard + first pilot landed in
`commerce-rails`).

## The rule

> *Every train repo's CI workflow is a thin runner around a single
> `just ci` recipe. Running `just ci` locally gives the same verdict
> as GitHub Actions on the same commit.*

When this property is green, the operator's question "will CI pass?"
is answered by running one command locally. Push surprises stop.

## What this standard solves

The 2026-06-08 16-repo CI/CD inventory
(`KB/06-operations/ci-cd-inventory.md`) documented systemic drift:

- The same recipe name does dramatically different work per repo
  (`just check` ranged from `cargo check` to `cargo check
  --workspace --all-targets`).
- CI workflows ran tooling the Justfile didn't expose (cargo-audit
  ignore lists, gitleaks).
- 3 of 16 repos had no CI at all.

The atelier CI repairs the same day (`QF-2026-06-08-03`,
`QF-2026-06-08-04` from a parallel session) hit four different drift
modes in a single workspace.

The fix: stop maintaining two parallel descriptions of "what gets
checked" — one in the Justfile, one in the workflow. Make the
Justfile the single source. The workflow is a runner.

## The canonical shape

### Justfile

**The file is named `Justfile` — capital J, everywhere.** This is not
cosmetic: the root repo's orchestration (`_dispatch`, `ws-check`, …)
tests `[[ -f $d/Justfile ]]`, which on case-sensitive filesystems
(every CI runner) silently misses a lowercase `justfile` and falls back
to raw cargo — the repo's own recipes never run. Four repos carried the
lowercase form until 2026-07-02; macOS's case-insensitive filesystem
hid the drift locally (and even let a commit stage `Justfile` while the
real file was `justfile`, stranding the change).

```just
default:
    @just --list

# `just ci` — single CI entry point per RP-CI-PARITY.
# Runs every gate CI runs, in CI order. Local exit code matches CI's.
# Order: fmt-check (fastest fail) → check (compile) → lint (clippy
# needs compile) → test.
#
# security-audit is deliberately NOT in `ci` (revised 2026-07-02):
# advisory verdicts change with the WORLD, not the commit. A new
# RUSTSEC entry must not turn a code push red — that conflates
# world-red with code-red (see "The feedback loop"). The audit runs
# in the Security/Stability workflows on schedule, and locally on
# demand via `just security-audit`.
ci: fmt-check check lint test
    @echo "✓ just ci: all gates passed"

check:
    cargo check --workspace --all-targets

test:
    cargo test --workspace --all-targets

fmt:
    cargo fmt --all

fmt-check:
    cargo fmt --all -- --check

lint:
    cargo clippy --workspace --all-targets -- -D warnings

security-audit:
    #!/usr/bin/env bash
    set -uo pipefail
    if [[ ! -f .audit-ignores ]]; then
        cargo audit --deny warnings
    else
        ignores=""
        while IFS= read -r line; do
            line="${line%%#*}"
            line="${line## }"
            line="${line%% }"
            [[ -z "$line" ]] && continue
            ignores="$ignores --ignore $line"
        done < .audit-ignores
        cargo audit --deny warnings $ignores
    fi
```

Non-negotiable invariants:

1. **`just ci` exists** and runs every CI gate.
2. **`--workspace --all-targets`** on `check`, `test`, and `lint`.
   Half-`--workspace` setups are what caused `QF-2026-06-08-03`
   (`fmt --check` formatting siblings, not just the workspace).
3. **`fmt-check` and `lint` are separate recipes**, both invoked by
   `just ci`. Bundling `fmt` into `lint` (as foundation crates
   historically did) is forbidden — it makes the verdict hard to
   reason about.
4. **`.audit-ignores`** is the single per-workspace ignore source.
   Embedded `--ignore` flags in the Justfile recipe are forbidden;
   drift between repos must be readable from one file per repo.

### `.audit-ignores`

```
# RUSTSEC IDs to ignore at `just security-audit` time.
#
# Format: one RUSTSEC ID per line. Blank lines and `#` comments are
# ignored.
#
# Each ignore MUST be justified inline:
#   # Reason: <transitive crate>, <why we can't fix>, <exposure surface>
#   RUSTSEC-YYYY-NNNN
#
# Removing an ignore is preferred over keeping it.
```

### `.github/workflows/ci.yml`

```yaml
name: CI

# Canonical RP-CI-PARITY shape (KB/05-engineering/standards/ci-parity.md).
# Thin runner around `just ci`. To add or modify a check, edit the
# Justfile — never add checks here.

on:
  push:
    branches: ["main"]
  pull_request:
    branches: ["main"]

env:
  CARGO_TERM_COLOR: always
  RUST_VERSION: "1.96.0"

permissions:
  contents: read

jobs:
  ci:
    name: just ci
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v6
      - uses: dtolnay/rust-toolchain@stable
        with:
          toolchain: ${{ env.RUST_VERSION }}
          components: clippy, rustfmt
      - uses: Swatinem/rust-cache@v2
      - uses: extractions/setup-just@v3
      - run: just ci
```

Non-negotiable invariants:

1. **Exactly one job per workflow**, named `ci` (matches the recipe).
   No per-gate separate jobs (`check`, `test`, `lint`, `format` as
   separate jobs the way mosaic-extensions historically did) — that
   duplicates setup and drifts.
2. **No `run:` step other than `just ci`** (and dep installation).
   If a workflow runs `cargo` directly, the parity is broken.
3. **`Swatinem/rust-cache@v2`** for cache; `extractions/setup-just@v3`
   for just; `taiki-e/install-action@v2` for cargo-audit. Standardised
   action versions keep CI behaviour comparable across repos.
4. **`env:`-block pattern** for any attacker-controlled input
   (per the GitHub Actions security guidance). Today's canonical CI
   doesn't need any, but extensions like the test/code-attribution
   workflow do.

## Sibling checkout (cross-repo path deps)

App and runtime workspaces path-dep on sibling repos
(`../../runtime-runway/...`, `[patch.crates-io]`, or `package.json`
`file:` deps). CI mirrors the local layout with
`scripts/ci/checkout-reflective-siblings.sh`, run as the **first step
after checkout** (before `rust-cache` — the cache action's
`cargo metadata` errors on unresolvable path deps). Lessons from the
first live runs (2026-07-03, RFL-132):

1. **Clone transitively.** Cargo loads the *workspace manifest* of any
   path dep, so every sibling that the sibling's own workspace
   path-deps on must exist too. Example: anything cloning
   runtime-runway must also clone commerce-rails.
2. **mosaic-extensions is an umbrella.** Locally it is one directory;
   on GitHub each extension (arbiter-policy, ferrox-solvers, soter-smt,
   manifold-adapters, embassy-ports, prism-analytics) is its own repo.
   Clone each needed extension into
   `../../mosaic-extensions/<name>` to mirror the local layout — the
   umbrella repo itself has no crates.
3. **Private siblings need a token.** bedrock-platform is private:
   pass `SIBLINGS_TOKEN` (from the `REFLECTIVE_SIBLINGS_TOKEN` secret —
   a fine-grained PAT with `contents: read`, never a personal oauth
   token) via an `env:` block on the step. Public siblings clone
   anonymously; the script only injects the token when set.
4. **JS deps count too.** catalyst-biz's `bun install` resolves
   `@reflective/helm-flow` from
   `file:../../bedrock-platform/helms/packages/helm-flow` — sibling
   checkout is not a Cargo-only concern.

## Multi-job exceptions

Some workspaces legitimately need multiple CI workflows beyond `ci.yml`:

- **`security.yml`** — runs `just security-audit` on a different
  schedule (weekly) plus secret scanning (gitleaks). The audit half
  is `just security-audit`; the secret scan is workflow-local because
  no Justfile equivalent exists yet (`QF-2026-06-08-10` tracks adding
  `just security-secrets`).
- **`coverage.yml`** — runs `cargo tarpaulin` or equivalent.
  Currently workspace-specific; canonical shape TBD.
- **`stability.yml`** — long-running soak tests. Workspace-specific.

The rule: **every** workflow file's main job should call `just <recipe>`
where `<recipe>` is the equivalent of what the job does. If no such
recipe exists, file a finding to add one rather than diverging.

## Migration path

Smallest-blast-radius first (from the 16-repo inventory):

1. **`commerce-rails`** — pilot landed 2026-06-08 (`QF-2026-06-08-09`).
   First-ever CI for the workspace. Validates the canonical shape
   against real feature code.
2. **`arena-tests`** — 10 existing recipes; needs `check`,
   `security-audit`, `coverage` added (`QF-2026-06-08-08`).
3. **`bedrock-platform/helms`** — 24 existing recipes but
   desktop-focused; `just ci` needs adapter recipes wrapping
   `cargo check` over the Rust workspace + the desktop bits
   (`QF-2026-06-08-07`).
4. **`mosaic-extensions/*`** (8 sub-repos) — close to canonical;
   collapse 4-job workflows into the single `ci` job; promote
   `clippy` + `fmt-check` into `lint` aggregator; align audit
   ignore lists into `.audit-ignores`.
5. **`bedrock-platform/{converge, axiom, organism}`** — foundation
   repos need `lint` recipe restructure (split `fmt-check` and
   `clippy`; unbundle from current `lint` body), `--all-targets`
   additions, and audit-ignores migration.
6. **`runtime-runway`** — unique infra recipes (`firebase-*`,
   `gcp-*`, `infra-*`) coexist with the canonical surface; `just ci`
   wraps the existing structure without disrupting them.
7. **`atelier-showcase`** — closest to canonical; cleanup pass on
   the existing 4-job workflow + `lint` consolidation.

Each migration is its own paired session because each workspace has
its own quirks. Mosaic sub-repos can be batched if their structure
proves uniform after the first one.

## Enforcement

Three layers:

1. **`just ci` exists and is green** — local invariant; reviewer
   checks at PR time.
2. **Workflow shape** — `just project-doctor` will gain a check that
   parses each train repo's `.github/workflows/ci.yml` and asserts:
   - One job named `ci`.
   - One `run:` step is `just ci` (plus setup steps).
   - No bare `cargo` invocations.
   Drift fails CI in the root repo's doctor workflow.
3. **Recipe shape** — at minimum, each repo's Justfile defines
   `ci`, `check`, `test`, `lint`, `fmt-check`, `security-audit`
   recipes. The `quality-doctor` recipe in the root repo could add
   a check that grep-validates per train repo.

The mechanical check is itself follow-up work (`QF-2026-06-08-11`
will be filed when the recipe migration reaches the foundation
repos — earlier is too soon because the canonical shape may evolve
during early migrations).

## What "verify locally, know it goes through" means in practice

Before this standard: an operator runs `just lint` or `just test` in
some workspace, gets green, pushes, and discovers CI ran a
different `cargo` invocation that fails. The most expensive class of
CI surprise.

After this standard:

```
$ cd commerce-rails
$ just ci
... fmt-check ...
... cargo check --workspace --all-targets ...
... cargo clippy --workspace --all-targets -- -D warnings ...
... cargo test --workspace --all-targets ...
... cargo audit --deny warnings ...
✓ just ci: all gates passed

$ git push origin next
$ # GitHub Actions runs the SAME just ci.
$ # If local was green, CI will be green.
```

The operator's mental model becomes: "if `just ci` is green, CI is
green." One command. One verdict.

## The feedback loop (andon)

Parity makes local and CI verdicts agree; the loop below makes a red
verdict impossible to miss. Added 2026-07-02 when the fleet survey found
repos that had been red on `main` since June 13 with nobody noticing.

- **`just factory-status`** (root repo) — the fleet board. Latest
  main-branch conclusion per workflow for every train repo, in train
  order. Exits non-zero when any repo is red. Logic lives in
  `scripts/factory/fleet-status.sh`.
- **`factory-alert.yml`** (root repo, daily cron) — runs the same
  script; when red, opens or updates a "Factory red" issue in the root
  repo. Requires the `FACTORY_STATUS_PAT` secret (fine-grained PAT,
  Actions: read on Reflective-Lab repos).
- **Stop the line** (AGENTS.md operating rule) — a red `main` in any
  fleet repo preempts feature work.
- **"Dependabot Updates" is excluded from the verdict** — those runs
  track dependabot's own resolution failures, not the state of `main`.
  An andon light that cries wolf gets ignored.

Two red classes need different reflexes:

1. **Code red** (CI/Coverage red after a push) — a parity or test
   defect; fix forward on `next` immediately.
2. **World red** (scheduled audit catches a new RUSTSEC advisory —
   e.g. RUSTSEC-2026-0186 memmap2 painted 8 repos red on 2026-06-29
   with zero commits) — `cargo update` the affected crate fleet-wide,
   upstream-first in train order; if no fixed version exists, add a
   justified entry to `.audit-ignores`.

## Cross-references

- `QUALITY_BACKLOG.md` — `RP-CI-PARITY`, `QF-2026-06-08-09` (pilot),
  `QF-2026-06-08-10` (this standard).
- `KB/06-operations/ci-cd-inventory.md` — the 16-repo audit that
  motivated this standard.
- `KB/06-operations/factory-health.md` — root-repo doctor recipes;
  the future per-train doctor check would extend that pattern.
- `KB/05-engineering/standards/doctor-recipe-pattern.md` — the
  pattern the project-doctor CI-parity check would follow.
- Pilot:
  - `commerce-rails/Justfile`
  - `commerce-rails/.github/workflows/ci.yml`
  - `commerce-rails/.github/workflows/security.yml`
  - `commerce-rails/.audit-ignores`
  - `commerce-rails/deny.toml`
