# ONE-KB consolidation plan (owner direction 2026-07-09)

> Architect analysis in SDD transcript a281a1ce; full per-file merge map: 2026-07-09-kb-merge-map.md (this dir).

## Honesty header
563 kb-classified md files; ~148 are the converge Governance validator SUBSYSTEM (code+cedar+personas under kb/ by accident — relocate as code, separate task); real kb ≈ 415. kb/mosaic (52) = complete federation-era duplicate → zero survivors. atelier/docs/strategy duplicates kb/mosaic/docs/strategy. 3 ADR homes (converge 9 real, axiom 6 dangling stubs pointing at a nonexistent root path, +1 clash on ADR-006). 8× duplicated Release-Commands/Getting-Started; Workflow trip-duplicated; 13 EXP files with colliding IDs (EXP-001 ×6). One-story drift severe: root INDEX/Home describe the federation (every Location wrong); 114 files carry sibling-path references; 16 files stale v3.x; converge skills say "main + next". Anchors of current truth: README.md (4.0.0, 142 crates, 6 layers) + kb/consolidation/* (2026-07-08/09).

## Target taxonomy (numbered, narrative order)
kb/{Home,INDEX,LOG,Observations} + 01-platform-story (doctrine/concepts/strategy/philosophy) | 02-architecture (layer-model, boundaries, subsystems/<13>) | 03-contracts-and-seams | 04-quality (standards, arena-dimensions, soak) | 05-operations (build/stack/workflow/storage) | 06-consumption (registry guide, scenarios, engagement) | 07-decisions (adr/, consolidation/ verbatim) | 08-history (audits/changelogs/experiments/releases).

## Waves
W0 scaffold+anchors (INDEX/Home rewritten from README truth) → W1 per-subsystem merges (parallel; disambiguate basenames on move; Governance excluded) → W2 kill duplicates (kb/mosaic, ADR stubs, 8× dupes, EXP renamespace) → W3 one-story reconciliation (federation paths, v3.x, main+next; grep-zero gate with 08-history whitelist) → W4 links+index (wikilink basename uniqueness; zero unresolved [[...]]/](…md); INDEX complete).

## Owner calls: governance destination; philosophy one-thesis-vs-voices; atelier/quality fate; doorstep files. Doctrine-vs-code verification (Fuzzy Substrate Boundaries) = W3 verification task with mismatch report.

## Risks: basename-collision link rewiring (disambiguate first); governance tree pollution; ADR stub dangling; history-vs-grep-gate tension; MILESTONES uniqueness check before delete.
