---
source: llm
---

# Factory Health

Canonical source: `build-depot/docs/operations/quality-gates.md`.

This page is retained as a parent-workspace pointer for older links. The
software-factory health gate is now defined by Build-Depot.

Important correction: the current root `Justfile` aggregate `just doctor` runs
four checks, not two:

- `quality-doctor`
- `agents-doctor`
- `shim-doctor`
- `project-doctor`

Do not add new factory-health semantics here. Update Build-Depot instead.
