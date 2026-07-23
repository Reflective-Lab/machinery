---
type: architecture-module
source-path: mosaic-extensions/prism-analytics/
last-scanned: 2026-06-06
tags: [architecture, mosaic-extensions, prism-analytics]
---

# Prism Analytics — Module Architecture

<!-- @generated:start -->

Part of [[Architecture - Overview|mosaic-extensions]]. Per its own README:

> *"Closed-form analytics and inference Suggestors for Converge formations."*
> — `mosaic-extensions/prism-analytics/README.md:14-18`

## Member crates (2)

Rust workspace at `prism-analytics/` with members:

- `crates/fuzzy` — fuzzy-inference Suggestor
- `crates/prism` — closed-form analytics Suggestor

## Key dependencies

From `prism-analytics/crates/prism/Cargo.toml:15-34`: `polars` 0.51.0, `burn` 0.21.0, `ndarray`, `converge-pack`, `converge-optimization`, `converge-fuzzy-inference`, `calamine` (Excel ingestion).

The Polars + Burn + ndarray stack signals dataframe-style analytics + neural-net inference. `calamine` indicates explicit Excel-as-source support. `converge-fuzzy-inference` integration is shared with [[Architecture - Manifold-adapters|Manifold]].

## Role

Closed-form analytics + inference suggestors that a formation can call on for **evidence**, not for action. Suggestor outputs flow back through Converge for promotion review (see [[../bedrock-platform/Architecture - Converge|Converge]]).

## Cross-references

- [[Architecture - Overview|mosaic-extensions overview]]
- [[Architecture - Crucible-models|Crucible Models]] — sibling for the training side (Burn/linfa) vs. Prism's inference side
- [[../bedrock-platform/Architecture - Converge|Converge]]

<!-- @generated:end -->
