---
name: build-depot-quality
description: "Build-Depot quality workflow for Bun/TypeScript factory work. Use when checking or improving local quality gates, graph schema alignment, tests, docs, quality drift, or PR review quality for the Build-Depot repo."
---

# Build-Depot Quality

Use this skill for quality work in the Build-Depot repo.

## Workflow

1. Read `AGENTS.md`, `README.md`, and
   `docs/architecture/software-factory-build-depot.md`.
2. Run the narrowest useful local gate:
   - `just quality-doctor` for setup drift.
   - `just check` for TypeScript type safety.
   - `just test` for behavior.
   - `just ci` before code changes are considered ready.
3. When graph-facing behavior changes, update these together:
   - `build-depot.pg`
   - `queries/*.gq`
   - `scripts/seed.ts`
   - `trigger.dev/debt-tracker.ts`
   - tests
   - docs
4. Treat malformed external payloads as first-class cases. Validate with `zod`,
   skip unsupported shapes explicitly, and test the skip reason.
5. Keep CI parity: workflows install tools and call Just recipes.

## Review Checklist

- TypeScript strictness remains enabled.
- Runtime boundaries validate unknown input.
- Tests cover success, skip, and malformed input paths.
- Graph records match `build-depot.pg`.
- New quality policy is backed by a doctor, test, or workflow check.
- Linear is referenced for active implementation work.
