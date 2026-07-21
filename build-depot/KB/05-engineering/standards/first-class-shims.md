---
source: llm
date: 2026-07-03
---

# First-class shims

## What

No workaround lands silently. A shim — linker/compiler leniency, an
`#[ignore]`d or commented-out test, a cfg-gated escape hatch, CI-only
configuration that changes what a gate actually verifies — either doesn't
land (the root cause is fixed first, even if that delays a release) or it
lands as **first-class debt**:

- Inline marker at the site, in the host file's comment syntax:
  `SHIM(QF-YYYY-MM-DD-NN, expires: YYYY-MM-DD): <one-line reason>`
- A ledger finding (the marker's ID) with root cause, root fix, and owner.
- Expiry is a promise: remove the shim by that date or re-justify in the
  finding's history — never silently renew.
- Bare `#[ignore]` is banned; `#[ignore = "reason"]` must cite a ledger or
  Linear ID. Commented-out tests are deleted or tracked, never parked.
- A release train does not leave with an unexpired shim on shipped cargo.

Gate *configuration* that does not change what is verified (e.g.
`CARGO_PROFILE_DEV_DEBUG=0` — same tests, smaller binaries) is not a shim.
The test is: would the gate pass a defect it previously caught? If yes,
it's a shim.

## Why

Karl, 2026-07-03: "I want ALL shims, conditional compiles, commented out
tests become a first class citizen. In general I want to frontload work and
delay releases rather than come up with workarounds or tell a system is
working with green tests that don't test the real thing." The factory's
agents act on CI verdicts autonomously; a green produced by a workaround
corrupts the one signal everything depends on. First instance:
`QF-2026-07-03-01` (ferrox `--allow-shlib-undefined` for the
OR-Tools/HiGHS version skew). Same family: `QF-2026-07-02-07`
(audit-ignore entries without expiry).

## How to check

`just shim-doctor` (`scripts/factory/shim-doctor.sh`), wired into
`just doctor` and `.github/workflows/doctor.yml`:

1. Every `SHIM(` marker parses, its `QF-*` ID exists in
   `QUALITY_BACKLOG.md`, and its expiry is not past.
2. A maintained smell list (`allow-shlib-undefined`, bare `#[ignore]`,
   commented-out `#[test]`) requires a `SHIM(` marker within 5 lines.

Scope v1: root-repo tracked files. Fleet-repo rollout is the standard's
residual (same pattern as `test-code-attribution`, `QF-2026-06-08-05`).

Enforces `RP-SHIM-FIRST-CLASS`.
