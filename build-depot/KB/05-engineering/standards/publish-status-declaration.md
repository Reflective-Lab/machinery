---
tags: [standards, cargo, release-engineering, layering]
source: llm
date: 2026-06-07
promoted-from: QF-2026-06-02-29
property: RP-LAYERING
---

# Publish-Status Declaration

Every Cargo crate in this workspace declares its publish intent explicitly
in its own `Cargo.toml`. Inheritance from a workspace root is not enough,
because `cargo metadata` does not propagate workspace-level `publish` to
members.

## Rule

Each `Cargo.toml` with a `[package]` section sets `publish` to one of:

- `publish = false` — internal crate (apps, binaries, fixtures, test
  scaffolds, anything UNLICENSED). Never goes to crates.io.
- `publish = ["crates-io"]` *(optional, equivalent to the default of `true`)*
  — public crate that can ship.

Omitting the field is forbidden in any workspace whose root is
`publish = false`. Even though Cargo's default is "publish = true," the
omission makes the crate look like an oversight in `cargo metadata` and
trips `RP-LAYERING` (a publishable-by-default app cannot path-dep an
unpublishable library).

## How to apply

1. Every new crate: add `publish = false` to `[package]` if it is internal,
   or leave it implicit only if you genuinely intend to publish. If
   intentional-to-publish, also fill in `description`, `license`,
   `repository`, and `readme` — the publish gate trips fast on missing
   metadata, and discovering that during a release-train run is painful.
2. Tauri / Electron / Dioxus apps and any `apps/*/src-tauri/` crate:
   always `publish = false`. The Tauri build never produces a crates.io
   artifact; the omission is a metadata classification bug, not a release
   option.
3. Internal `prio-*` / `*-backend` / `application-*` crates: always
   `publish = false` until and unless a deliberate publish decision lands
   in an ADR.

## Enforcement

`just project-doctor` check 2 (`RP-LAYERING`) walks `cargo metadata
--no-deps` per train workspace and rejects any path-dep from a publishable
crate to a `publish = false` crate. The original failure mode the check
caught was `outcome-workbench-desktop` (a Tauri app missing the
declaration) showing up as publishable and triggering 4 false-positive
layering violations against `prio-expenses`, `workbench-backend`,
`application-kernel`, `application-storage`.

CI-gated via `.github/workflows/doctor.yml` `project-doctor` job.

## Exceptions

- Examples crates (`examples/<name>/Cargo.toml`) are exempt from the rule
  only if Cargo's default member-discovery already excludes them from the
  workspace and from `cargo publish`. If the example is a workspace member,
  the rule applies.
- Test-fixture crates that exist only to be compiled by `trybuild` follow
  the rule (`publish = false`) — the cargo metadata visibility is what
  matters, not the test framework's intent.

## Why this rule

Cargo treats `[workspace.package].publish = false` as a default-for-members
that members can override but does not propagate it to `cargo metadata`'s
`publish` field on individual packages unless the member opts in via
`publish.workspace = true`. Most workspaces in this repo don't opt in.
Result: members of an "internal-only" workspace look publishable from the
outside. That confuses every downstream tool — `cargo publish`, `cargo
public-api`, the layering check, and any human auditing the release train.

Explicit per-crate `publish = false` is two seconds of work and removes
the entire class of confusion at the source.

## Provenance

Closing finding: `QF-2026-06-02-29`. Surfaced by the first run of
`just project-doctor` check 2. Closing commit:
`Reflective-Lab/helms@72ad5a0`. Related: `KB/05-engineering/standards/
doctor-recipe-pattern.md` for how the check itself is structured.
