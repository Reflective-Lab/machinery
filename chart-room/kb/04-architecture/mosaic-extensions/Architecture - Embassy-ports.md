---
type: architecture-module
source-path: mosaic-extensions/embassy-ports/
last-scanned: 2026-06-06
tags: [architecture, mosaic-extensions, embassy-ports]
---

# Embassy Ports — Module Architecture

<!-- @generated:start -->

Part of [[Architecture - Overview|mosaic-extensions]]. Per its own README:

> *"Source-specific connector ports for Converge extensions."*
> — `mosaic-extensions/embassy-ports/README.md:14-19`

The largest mosaic family by source-file count (140 files). One crate per external data source.

## Member crates (24)

Rust workspace at `embassy-ports/`, all members under `crates/`:

`pack`, `linkedin`, `sec-edgar`, `bolagsverket`, `gleif`, `vies`, `ofac-sls`, `eu-sanctions`, `commerce-csl`, `sam-gov`, `usaspending`, `ted`, `skatteverket`, `uspto`, `crunchbase`, `github`, `pubmed`, `arxiv`, `openalex`, `wikidata`, `companies-house`, `scb`, `epo`.

Sources span: business identity registries (Bolagsverket, GLEIF, Companies House, VIES, SAM.gov), sanctions lists (OFAC SLS, EU sanctions, Commerce CSL), procurement (USASpending, TED), tax (Skatteverket, EPO), patents (USPTO, EPO), scientific literature (PubMed, arXiv, OpenAlex), public knowledge (Wikidata), professional graph (LinkedIn), code graph (GitHub), startup data (Crunchbase), national statistics (SCB).

## Key dependencies

From `embassy-ports/Cargo.toml:45-69`: `converge-manifold-adapters` 1.1.0, `tokio` 1.48, `reqwest` 0.12, `serde`, `serde_json`, `async-trait`, `thiserror`, `sha2`.

Each port crate is an async HTTPS adapter against its source's public API. The shared dependency on [[Architecture - Manifold-adapters|manifold-adapters]] suggests embassy ports normalize their output through manifold's generic adapter contract before handing evidence back to a Converge formation.

## Boundary

Embassy ports own **fetching and normalizing** from an external source. They do not own retention (→ [[Architecture - Mnemos-knowledge|Mnemos]]), policy (→ [[Architecture - Arbiter-policy|Arbiter]]), or evidence scoring (→ [[Architecture - Prism-analytics|Prism]]).

## Cross-references

- [[Architecture - Overview|mosaic-extensions overview]]
- [[Architecture - Manifold-adapters|Manifold Adapters]] — shared adapter contract

<!-- @generated:end -->
