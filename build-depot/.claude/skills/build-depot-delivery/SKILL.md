---
name: build-depot-delivery
description: "Build-Depot reliable delivery workflow for CI/CD, Trigger.dev deploys, delivery preflight, smoke tests, rollback planning, and production runbooks. Use when changing GitHub Actions, Justfile delivery recipes, Trigger.dev deployment, Terraform delivery resources, or release operations."
---

# Build-Depot Delivery

Use this skill for delivery work in the Build-Depot repo.

## Workflow

1. Read `AGENTS.md`, `docs/operations/reliable-delivery.md`, and
   `docs/architecture/software-factory-build-depot.md`.
2. Before deploy-impacting changes are considered ready, run:
   - `just delivery-doctor`
   - `just delivery-preflight`
3. Keep delivery workflows thin:
   - install Bun and Just
   - run `bun install --frozen-lockfile`
   - call the appropriate Just recipe
4. For production deploys, use `just deploy` only after preflight passes.
5. Smoke test with known-safe Trigger payloads and verify task logs, graph
   ingest, and skipped/error counts.
6. If delivery fails, update Linear before continuing feature work.

## Review Checklist

- CI and local commands produce the same verdict.
- Manual deploys require preflight.
- Required secrets are documented by name.
- Webhook changes remain idempotent.
- Rollback path is documented and realistic.
- Production-affecting changes have a Linear issue.
