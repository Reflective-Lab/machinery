---
type: architecture-module
source-path: mosaic-extensions/crucible-models/
last-scanned: 2026-06-06
tags: [architecture, mosaic-extensions, crucible-models]
---

# Crucible Models — Module Architecture

<!-- @generated:start -->

Part of [[Architecture - Overview|mosaic-extensions]]. Per its own README:

> *"Trained-model packs and training pipeline for the Converge Engine."*
> — `mosaic-extensions/crucible-models/README.md:14-18`

## Member crates (1)

Single crate `crates/crucible`. Path: `mosaic-extensions/crucible-models/crates/crucible/`.

## Key dependencies

From `crucible-models/crates/crucible/Cargo.toml:15-32`: `converge-pack`, `converge-optimization`, `burn` 0.20.0, `linfa` 0.8, `linfa-trees`, `ndarray`, `polars`, `calamine` (Excel, optional).

`burn` for neural-net work, `linfa` (+`linfa-trees`) for classical ML, `polars`/`ndarray` for data — same data stack as [[Architecture - Prism-analytics|Prism]] but oriented toward **training** rather than inference.

## Role

Training-side counterpart to [[Architecture - Prism-analytics|Prism]]. Produces trained-model packs the Converge Engine can load.

## Cross-references

- [[Architecture - Overview|mosaic-extensions overview]]
- [[Architecture - Prism-analytics|Prism Analytics]] — inference side

<!-- @generated:end -->
