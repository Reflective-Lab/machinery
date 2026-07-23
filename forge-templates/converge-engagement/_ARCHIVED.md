---
status: archived
archived_on: 2026-06-07
archived_reason: superseded by KB/04-architecture/reference-engagement.md pointing at studio-apps/folio-editor (the actual reference engagement, which is maintained)
sweep_adr: ../../KB/04-architecture/decisions/2026-06-07-retire-engagement-template.md
---

# Archived: forge-templates/converge-engagement/

This template is **archived as of 2026-06-07**. Do not use it to start a new engagement.

## Why archived

This was a hand-maintained scaffold for new Reflective Labs engagements. It carried:

- A 6-crate Cargo workspace skeleton (`{{project}}-{app,domain,kernel,platform,server,truths}`)
- Standard agent files (AGENTS.md / CLAUDE.md / README.md / etc.)
- A floor-version table for required platform crates

The floor table is the proximate forcing function: as of 2026-06-07 it preached Converge ≥ 3.8.1 (actual head 3.9.2), Organism ≥ 1.5.0 (actual 1.9.3), Axiom ≥ 0.7.0 (actual 0.15.2) — 1 to 8 minor versions behind. The drift-check script flagged this as critical.

Deeper reason: the actual reference engagement implementation, `studio-apps/folio-editor/` (the "Newspaper" project), is **maintained** because Karl uses it. This template is **not** maintained — it's a snapshot that decays. Two sources of truth → drift → wrong defaults preached to whoever bootstraps next.

## Use this instead

To start a new engagement, copy `studio-apps/folio-editor/` as the reference implementation. See [[../../KB/04-architecture/reference-engagement|KB/04-architecture/reference-engagement.md]] for the procedure.

## Why this file is here, not deleted

`forge-templates/` is gitignored by the main reflective repo and has no `.git` of its own — there is no git history to restore from. Soft-delete via this `_ARCHIVED.md` marker preserves the content as a historical reference (Karl can read what the template's shape was if needed) without keeping it as a live target.

## What's still useful in the same folder

Nothing actionable. If you're looking for the canonical engagement scaffold conventions, read `studio-apps/folio-editor/Cargo.toml` and `studio-apps/folio-editor/README.md` — those are the live source.

## Companion: converge-extension is NOT archived

The sibling `forge-templates/converge-extension/` template **is still active**. It ships CI workflows, the release-checklist enforcement script, and a criterion benchmark baseline extractor — concrete tooling that a new extension repo gets on day 1. Do not archive that one without a separate decision.

## Sweep

See [[../../KB/04-architecture/decisions/2026-06-07-retire-engagement-template|2026-06-07 retirement ADR]] for the full claim-sweep checklist (drift-check script updated, registry anchor narrowed, forge-templates architecture overview narrowed).
