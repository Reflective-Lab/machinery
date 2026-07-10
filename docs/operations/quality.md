# Quality Setup

Build-Depot quality controls are intentionally small and repeatable. The local
verdict and CI verdict should match on the same commit.

## Local Commands

```bash
just check
just test
just ci
just quality-doctor
```

`just ci` is the default code gate. It runs strict TypeScript typechecking and
Bun tests. `just quality-doctor` is the structural drift check for the factory
setup itself.

## Quality Invariants

- Bun is the only project package manager. `bun.lock` is the lockfile.
- TypeScript stays strict. Do not relax compiler flags to land a change.
- Runtime inputs are validated before use.
- Graph-facing records stay aligned with `build-depot.pg`.
- CI calls `just ci` instead of reimplementing the gate in YAML.
- Tests cover normalizers, graph record shape, and failure behavior.

## Drift Checks

`just quality-doctor` checks:

- package manager and lockfile shape
- required package scripts
- strict TypeScript options
- Justfile recipe surface
- test presence
- architecture and operations documentation presence

The doctor is read-only. It reports drift and exits non-zero if a check fails.

## Review Lens

When reviewing a Build-Depot change, classify quality impact in these areas:

- correctness of event parsing and normalization
- test coverage for malformed payloads and skipped records
- graph schema/query alignment
- CI parity with local commands
- operational observability of task outcomes
- whether the change improves or weakens the software factory

## Evidence

Quality state should be traceable to one of:

- a passing local command
- a CI run
- a graph query result
- a Linear issue
- a documented standard or architecture note

Avoid relying on chat memory for durable factory state.
