---
tags: [standards, rust, toolchain, reproducibility]
source: llm
date: 2026-06-07
promoted-from: QF-2026-06-02-30
property: RP-RUSTC-DRIFT-CONTAINED, RP-AUTO-BLESS-AUDITED
---

# rustc Pinning

Every Cargo workspace in the release train pins rustc to an exact stable
release in its own `rust-toolchain.toml`. Bumps land in dedicated PRs with
classified snapshot diffs.

## Rule

Each train workspace root has a `rust-toolchain.toml` of this exact shape:

```toml
[toolchain]
channel = "1.X.Y"
components = ["rustfmt", "clippy"]
```

- `channel` is an exact stable version (`1.96.0`, not `stable`, not
  `nightly`). A dated nightly (`nightly-YYYY-MM-DD`) is allowed in
  workspaces that need a nightly-only feature; document why in the
  comment block at the top of the file.
- `components = ["rustfmt", "clippy"]` is required. Without them, `cargo
  fmt --check` and `cargo clippy` in CI silently skip the workspace.

A header comment block on every file points the reader at
`RP-RUSTC-DRIFT-CONTAINED` and `just project-doctor` check 5 so the
provenance survives.

## How to apply

For a new workspace:

```bash
cat > rust-toolchain.toml <<'EOF'
# Pin rustc to a specific stable release so trybuild fixtures, clippy
# lints, and macro-expansion output stay deterministic across contributor
# machines and CI runners. Enforced by RP-RUSTC-DRIFT-CONTAINED
# (`just project-doctor` check 5 at workspace root). Bump in a dedicated
# PR with classified snapshot diffs (RP-AUTO-BLESS-AUDITED).
[toolchain]
channel = "1.96.0"
components = ["rustfmt", "clippy"]
EOF
```

On a fresh machine, the named toolchain needs to be installed in rustup:

```bash
rustup toolchain install 1.96.0 --profile minimal --component rustfmt --component clippy
rustup component add cargo --toolchain 1.96.0
```

The second `rustup component add cargo` call is non-obvious but required:
`rustup toolchain install ... --profile minimal` sometimes ships rustc,
rustfmt, and clippy but not cargo, so `cargo metadata` against the pinned
toolchain fails with `'cargo' is not installed for the toolchain
'1.X.Y-...'`. Adding the `cargo` component explicitly is idempotent.

## Bump procedure

A rustc bump is its own PR. It does not piggyback on a feature commit.

1. Bump every train workspace's `rust-toolchain.toml` to the new exact
   version in one commit per workspace.
2. Run `cargo check --workspace --all-targets` and capture any snapshot
   diffs (`TRYBUILD=overwrite`, `cargo insta accept`).
3. Classify each diff line per `RP-AUTO-BLESS-AUDITED`:
   - **cosmetic** — rustc evolution (improved diagnostic wording, span
     refinement, formatter touch-up). Bless freely.
   - **semantic** — a behavior change. Stop. Open a finding. Do not bless
     until the contract delta is understood.
4. In the PR description, paste the classified diff table. Without it,
   the bump cannot land.

## Enforcement

`just project-doctor` check 5 (`RP-RUSTC-DRIFT-CONTAINED`, pinning half)
verifies each train workspace has `rust-toolchain.toml` pinning an exact
`1.X.Y` or dated nightly. CI-gated via `.github/workflows/doctor.yml`
`project-doctor` job — installs the pinned `1.X.Y` and `cargo` component
explicitly before any `cargo metadata` call.

The classified-diff half of `RP-RUSTC-DRIFT-CONTAINED` is still
convention-based (`RP-AUTO-BLESS-AUDITED`). Lint that proves the bump PR
contains a classification table is a future tightening.

## Exceptions

- Workspaces outside the publishable train (`marquee-apps/*` containers,
  scout-sourcing, mobile apps) are not currently gated. `RP-RUSTC-DRIFT-
  CONTAINED` applies to the 9 train workspaces only. Container workspaces
  that ship binaries to end users should adopt the same pattern, but the
  enforcement scope is intentionally narrow until the train is solid.
- `nightly-YYYY-MM-DD` is allowed but discouraged. If a workspace pins
  nightly, the `rust-toolchain.toml` header comment must state which
  nightly feature it depends on and the schedule for moving to stable.

## Why this rule

`channel = "stable"` is not a pin. It resolves to whatever `rustup` has
installed, which differs between contributors and between contributor and
CI. Effects:

- Trybuild fixtures bless cosmetically-different rustc output on one
  machine; CI on another rustc fails.
- `cargo update -w` plus a silent rustc bump can land in the same PR as
  code changes; the snapshot drift gets blamed on flakiness rather than
  the toolchain.
- Clippy lint sets drift; a PR that passes locally fails CI on a newer
  rustc with a stricter lint.
- `RP-AUTO-BLESS-AUDITED` (classified snapshot diffs) becomes
  unenforceable because there is no commit boundary that *is* the rustc
  bump.

Exact pinning makes the rustc bump a deliberate, classified, auditable
event. Everything else benefits.

## Provenance

Closing finding: `QF-2026-06-02-30`. Surfaced by the first run of
`just project-doctor` check 5; 8 of 9 train workspaces had no
`rust-toolchain.toml`, the 9th (`marquee-apps/scout-sourcing`, out of
train scope) pinned `channel = "stable"`. Closing commits (per train repo):
`converge@02d2648`, `axiom@5b19956`, `organism@1548d30`, `helms@5c43817`,
`atelier-showcase@dc7b415`, `arena-tests@abd6527`, `runtime-runway@012b81b`,
`commerce-rails@2e5680f`. Related: `KB/05-engineering/standards/
doctor-recipe-pattern.md`.
