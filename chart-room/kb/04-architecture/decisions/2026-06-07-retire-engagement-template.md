# Retirement ADR: `forge-templates/converge-engagement/` scaffold template

- Date: 2026-06-07
- Status: **swept**
- Decision type: retirement / replacement
- Replaces: [[2026-06-02-converge-runtime-retirement|2026-06-02 converge-runtime retirement]] (similar shape; second worked example of the retirement-ADR protocol)
- Related: [[templates/retirement-adr|Retirement ADR Template]]; [[../forge-templates/Architecture - Overview|forge-templates overview]]; [[../reference-engagement|reference-engagement.md]]

## What's being retired

The `forge-templates/converge-engagement/` template — a hand-maintained 6-crate Cargo workspace scaffold (`{{project}}-{app,domain,kernel,platform,server,truths}`) with placeholder syntax, standard agent files (AGENTS / CLAUDE / README / CHANGELOG / MILESTONES / CAPABILITIES), Justfile, and a floor-version table for required Converge / Organism / Axiom / Mosaic crates.

The directory still exists on disk (forge-templates/ is gitignored and has no .git of its own — no soft-delete via git history is possible). A `_ARCHIVED.md` marker has been added inside the folder; treat that as the canonical signal that the template is no longer a live target.

The sibling `forge-templates/converge-extension/` template is **NOT retired**. It ships working CI workflows + release-checklist enforcement + criterion benchmark baseline extractor and is still the recommended starting point for new Mosaic extensions.

## Why

Two reasons:

1. **Stale floor versions.** The template's README floor table preached Converge ≥ 3.8.1 (actual 3.9.2; 1 minor behind), Organism ≥ 1.5.0 (actual 1.9.3; 4 minor behind), Axiom ≥ 0.7.0 (actual 0.15.2; 8 minor behind). The 2026-06-07 drift-check script flagged these as critical. A template that preaches stale defaults is worse than no template — it actively misleads.

2. **Duplicates the live reference.** The README explicitly says *"Patterns lifted from `engagements/newspaper` — the reference implementation"*. That repo is now `studio-apps/folio-editor/` and Karl actively maintains it for the Sölvesborg Lede pilot. Folio-editor has 7 crates (a superset of the template's 6), tracks current platform head, and follows current naming conventions — strictly more correct than the template at any given moment.

Forcing function: the [[../scripts/drift-check|drift-check.py]] script's first run, today, surfaced the floor drift as critical (exit code 2) and made the maintenance liability concrete.

## Old → New (the migration table)

| What it was | Where it lived | What it is now | Where it lives now |
|---|---|---|---|
| `forge-templates/converge-engagement/` (live template) | not in version control (gitignored at main repo) | archived (still on disk, `_ARCHIVED.md` marker) | same path; marker flags don't-use |
| README "## Floor versions" table | `forge-templates/converge-engagement/README.md:8-21` | implicit: whatever folio-editor's Cargo.toml resolves to | `studio-apps/folio-editor/Cargo.toml` |
| 6-crate scaffold | `forge-templates/converge-engagement/crates/{{project}}-*/` | 7-crate reference implementation | `studio-apps/folio-editor/crates/newspaper-*` |
| Engagement bootstrap procedure | implicit in template's README ("copy + replace `{{project}}`") | explicit in [[../reference-engagement|reference-engagement.md]] | KB |
| Floor-drift check in drift-check.py | `check_forge_floors()` walked `converge-engagement/README.md` | check now skips when `_ARCHIVED.md` marker present | `KB/scripts/drift-check.py` |

## Claim sweep checklist

### Registry

- [x] `KB/04-architecture/current-system-map.md` — Forge Templates anchor narrowed to mention only `converge-extension` template + reference-engagement.md pointer. Floor-drift call-out removed (no longer applicable; folio-editor's Cargo.toml is the new source of truth).

### Architecture notes

- [x] `KB/04-architecture/forge-templates/Architecture - Overview.md` — converge-engagement section rewritten as "(archived 2026-06-07 — see retirement ADR)"; converge-extension content unchanged. The Mermaid simplified to show only extension template.
- [x] `KB/04-architecture/reference-engagement.md` — created. Documents folio-editor as the reference; 6-step bootstrap procedure for new engagements.

### Scripts

- [x] `KB/scripts/drift-check.py` — `check_forge_floors()` now checks for the `_ARCHIVED.md` marker and skips with an info entry. Floor drift no longer reports as critical for the engagement template.

### KB cross-references

- [x] `KB/04-architecture/README.md` — added reference-engagement.md to a new "Reference implementations" section under Boundary registry; added this ADR to the Decisions list.
- [x] `KB/LOG.md` — entry added under 2026-06-07.

### Code-side markers

- [x] `forge-templates/converge-engagement/_ARCHIVED.md` — added. Explains the archival, points at reference-engagement.md, and explicitly says converge-extension is NOT archived.

### Out of scope (intentional)

- [ ] `KB/04-architecture/forge-templates/Architecture - Overview.md` is NOT moved or renamed despite covering only one template now — keep the directory and filename stable; the content describes what's at `forge-templates/` (a single-template directory now).
- [ ] Hard `rm -rf` of `forge-templates/converge-engagement/` — deferred. The directory has no git history; soft-delete via `_ARCHIVED.md` is reversible and matches the existing `_ARCHIVED.md` pattern used for KB/converge-business/ and KB/outcome-workbench/ earlier today.

## Consequences

- Anyone (Karl, Claude in a future session) bootstrapping a new engagement is now redirected to folio-editor. Procedure documented in [[../reference-engagement|reference-engagement.md]].
- The drift-check script no longer reports critical findings against the archived template. Drift-check exit code on the next run should drop from 2 to 1 (only the 10 missing-Boundary-blocks remain as warnings).
- `forge-templates/` becomes a single-template directory (just `converge-extension/`). If a future decision retires that too, this folder can be removed entirely (see the orphan-folder check in any future health audit).
- No code or downstream caller breaks. The template was a manual-copy scaffold; nothing imported from it.

## Follow-Ups

- When folio-editor's conventions change (e.g. a new sub-crate is added, the KB skeleton is restructured), update folio-editor itself — it IS the reference now.
- The 5-step engagement bootstrap procedure in reference-engagement.md could become a `just bootstrap-engagement <name>` recipe later if engagement creation gets frequent. Not needed today.
- The drift-check script could grow a generic "is this folder retired?" check (look for `_ARCHIVED.md`) rather than having a hardcoded `FORGE_ENG_ARCHIVED` path. Defer until a third retirement uses the marker.

## Sweep evidence

- Sweep landed: 2026-06-07 (same day as decision)
- Commits: TBD (will be added on next commit; this ADR + the sweep land together)
