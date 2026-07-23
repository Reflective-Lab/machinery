---
type: architecture-module
source-path: mosaic-extensions/mnemos-knowledge/
last-scanned: 2026-06-06
tags: [architecture, mosaic-extensions, mnemos-knowledge]
---

# Mnemos Knowledge — Module Architecture

<!-- @generated:start -->

Part of [[Architecture - Overview|mosaic-extensions]]. Per its own README:

> *"Knowledge, recall, retrieval, and memory for Converge formations."*
> — `mosaic-extensions/mnemos-knowledge/README.md:14-18`

## Member crates (1)

Single crate `crates/mnemos`. Path: `mosaic-extensions/mnemos-knowledge/crates/mnemos/`.

## Key dependencies

From `mnemos-knowledge/crates/mnemos/Cargo.toml:18-49`: `converge-pack`, `tokio`, `tonic` 0.14 + `tonic-prost` + `prost` (gRPC), `pulldown-cmark` (Markdown parsing), `uuid`, `chrono`, `reqwest` (OpenAI embeddings).

The gRPC surface signals Mnemos exposes its capability over the network as well as in-process; OpenAI embeddings + Markdown parsing point to text-knowledge ingestion.

## Role

Long-term memory + retrieval for formations. The retention layer that complements [[Architecture - Embassy-ports|Embassy Ports]] (fetch) and [[Architecture - Prism-analytics|Prism]] (analyze).

## Cross-references

- [[Architecture - Overview|mosaic-extensions overview]]
- [[Architecture - Embassy-ports|Embassy Ports]] — fetch side
- [[Architecture - Prism-analytics|Prism Analytics]] — analytics side

<!-- @generated:end -->
