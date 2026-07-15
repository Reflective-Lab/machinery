# Adopting the Factory

Build-Depot is a **reusable factory**, not a machinery-only setup. Any repo —
Bedrock, Applications, a new product — adopts it without copying scripts or CI
logic. Machinery hosts the kit; every other repo consumes it. This is the reusable
pattern; machinery is simply the first (and reference) consumer.

## What a consumer repo provides

Four small, repo-local things — nothing else:

1. **`release-train.yaml`** at the repo root — declares *this repo's* publishable
   projects and its Rust workspace roots + fresh-clone budgets:

   ```yaml
   version: 1
   projects:
     - name: my-lib
       dir: crates/my-lib
   fresh_workspaces:
     - dir: .
       fresh_check_budget_seconds: 900
       fresh_test_budget_seconds: 1200
   ```

2. **Caller workflows** in `.github/workflows/` — three lines each. They invoke the
   reusable factory workflows hosted in machinery:

   ```yaml
   # .github/workflows/factory.yml
   name: factory
   on: { push: { branches: [main] }, pull_request: { branches: [main] } }
   jobs:
     doctor:
       uses: Reflective-Lab/machinery/.github/workflows/factory-doctor.yml@main
     attribution:
       uses: Reflective-Lab/machinery/.github/workflows/factory-attribution.yml@main
   ```

   ```yaml
   # .github/workflows/factory-weekly.yml
   name: factory-weekly
   on: { schedule: [{ cron: "0 6 * * 1" }], workflow_dispatch: {} }
   jobs:
     hermetic:
       uses: Reflective-Lab/machinery/.github/workflows/factory-hermetic-audit.yml@main
     fresh-clone:
       uses: Reflective-Lab/machinery/.github/workflows/factory-fresh-clone.yml@main
   ```

   The reusable workflows check out the *caller's* repo and read the *caller's*
   `release-train.yaml`. No workspace is hardcoded — Bedrock gets bedrock's
   workspaces, Applications gets its own.

3. **A canonical `Justfile`** with the fleet gate surface (`ci` = `fmt-check →
   check → lint → test`, `security-audit`, `delivery-preflight`) and a **`deny.toml`**.
   Copy the reference from `chart-room/strategic/validator/` — it is the smallest
   complete adopter.

4. **Vendored standards** synced from build-depot (read-only), so the repo carries
   the rules it is held to. `just standards-sync` (see below) pulls them; do not
   hand-edit the vendored copies.

## What Build-Depot owns (the kit, in machinery)

| Piece | Path | Reusable how |
|---|---|---|
| `factory-doctor.yml` | `.github/workflows/` | `workflow_call` — release-train integrity everywhere; quality-doctor where build-depot is present |
| `factory-hermetic-audit.yml` | `.github/workflows/` | `workflow_call` — matrix from caller's `fresh_workspaces` |
| `factory-fresh-clone.yml` | `.github/workflows/` | `workflow_call` — matrix + budgets from caller's `fresh_workspaces` |
| `factory-attribution.yml` | `.github/workflows/` | `workflow_call` — fetches the canonical detector from build-depot |
| RP definitions | `build-depot/KB/…/recurring-properties.json` | vendored-sync |
| Standards (16 `.md`) | `build-depot/KB/…/standards/` | vendored-sync |
| Doctor scripts | `build-depot/scripts/` | referenced by the reusable workflows |
| Approved libraries | `build-depot/docs/operations/approved-libraries.md` | vendored-sync |

## Rules authored once, adopted everywhere

Build-Depot defines the RP set, gate shape, and standards **once**. Consumers adopt
them by reference (reusable workflows) and by vendored-sync (standards) — never by
forking logic. Pin to `@main` for live rules or a tag (`@factory-v1`) for stability.

## The one deliberate boundary

`factory-doctor`'s `project-doctor` layer (crate layering, seam checks) is still
partly fleet-shaped and runs only where build-depot is present. Making it fully
release-train-driven so every consumer runs it is the next step
(tracked in QUALITY_BACKLOG.md). Until then, consumers get release-train integrity,
hermetic-audit, fresh-clone, and attribution — the load-bearing gates.
