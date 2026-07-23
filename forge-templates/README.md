# Forge Templates

Project templates for spinning up new Converge-based extensions.

> **Local-only README.** This directory is gitignored at the main reflective repo level and has no `.git` of its own — it's a local meta-container for templates. This file exists so the boundary-registry drift check can find a Boundary block; the durable record is the architecture deep-dive in `KB/04-architecture/forge-templates/`.

## Boundary

> Owns: extension workspace skeleton + working CI workflows + release-ritual enforcement scripts for new Mosaic extensions (`converge-extension`). Does NOT own: live extension code (each copy detaches); engagement scaffolding (→ `studio-apps/folio-editor` as the live reference, per [[reference-engagement|reference-engagement.md]]).

— Canonical claim: [Forge Templates](https://github.com/Reflective-Lab/reflective/blob/main/KB/04-architecture/current-system-map.md#forge-templates) in the boundary registry. Update there first; this README quotes that source.

## Active template

- `converge-extension/` — workspace skeleton + 4 CI workflows + Extension Release Checklist enforcement script + criterion benchmark baseline extractor. Use this when starting a new Mosaic extension (Arbiter / Crucible / Ferrox / Manifold / Mnemos / Prism / Soter pattern).

## Archived

- `converge-engagement/` — **archived 2026-06-07**. See its `_ARCHIVED.md` for the archival reason. For new engagements, copy `studio-apps/folio-editor` (the live reference) per `KB/04-architecture/reference-engagement.md`.

## See also

- `KB/04-architecture/forge-templates/Architecture - Overview.md` — architecture overview.
- `KB/04-architecture/decisions/2026-06-07-retire-engagement-template.md` — the engagement-template retirement decision.
- `KB/04-architecture/reference-engagement.md` — current procedure for starting a new engagement.
