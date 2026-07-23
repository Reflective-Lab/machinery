---
type: architecture-module
source-path: mosaic-extensions/manifold-adapters/
last-scanned: 2026-06-06
tags: [architecture, mosaic-extensions, manifold-adapters]
---

# Manifold Adapters — Module Architecture

<!-- @generated:start -->

Part of [[Architecture - Overview|mosaic-extensions]]. Per its own README:

> *"Generic adapter implementations for Converge contracts."*
> — `mosaic-extensions/manifold-adapters/README.md:14-18`

## Member crates (1)

Single crate `crates/manifold` (workspace declares one member). Path: `mosaic-extensions/manifold-adapters/crates/manifold/`.

## Key dependencies

From `manifold-adapters/crates/manifold/Cargo.toml:15-48`: `converge-core`, `converge-experience`, `converge-fuzzy-inference`, `converge-pack`, `converge-provider`, `converge-storage`, `object_store`, `reqwest`, `lancedb`, `surrealdb`, `hf-hub`.

Dependency shape tells the story: this crate is a one-stop set of **generic** adapter implementations covering object storage (`object_store`), vector store (`lancedb`), document/graph store (`surrealdb`), model hub access (`hf-hub`), HTTP (`reqwest`), against the full Converge contract surface (`converge-{core,experience,fuzzy-inference,pack,provider,storage}`).

## Role in the whole

Provides the **default Converge adapter implementations**. Other Mosaic capabilities depend on these adapters (e.g. [[Architecture - Embassy-ports|Embassy Ports]] depends on `converge-manifold-adapters`), so manifold is the bottom of the Mosaic adapter stack.

## Boundary

Owns: generic adapter implementations. Does NOT own: source-specific port logic (→ [[Architecture - Embassy-ports|Embassy Ports]]), domain-specific analytics (→ [[Architecture - Prism-analytics|Prism]]), Converge contract definitions themselves (→ [[../bedrock-platform/Architecture - Converge|Converge]]).

## Cross-references

- [[Architecture - Overview|mosaic-extensions overview]]
- [[../bedrock-platform/Architecture - Converge|Converge]] — contract surfaces this crate implements

<!-- @generated:end -->
