---
type: architecture-module
source-path: studio-apps/wolfgang-chat/crates/
last-scanned: 2026-06-07
tags: [architecture, studio-apps, wolfgang-chat]
---

# wolfgang-chat — Core

<!-- @generated:start -->

Part of [[Architecture - Overview|wolfgang-chat]]. Two Rust library crates shared between [[Architecture - Desktop|Desktop]] (Tauri commands invoke them in-process) and [[Architecture - Backend|Backend]] (Cloud Run service invokes them per-request).

## `crates/wolfgang-core/`

> *"Shared logic for Wolfgang: LLM clients, RAG, personas, knowledge"*
> — `crates/wolfgang-core/Cargo.toml` description, v0.1.0

### Public modules (`src/lib.rs`)

```rust
pub mod brand;          // brand customization (avatar, background, persona text)
pub mod client;         // LLM HTTP client abstractions
pub mod embedding;      // embedding generation
pub mod firestore_kb;   // Firestore knowledgebase access
pub mod knowledge;      // knowledge ingestion + retrieval policies
pub mod llm;            // LLM provider integration
pub mod persona;        // Professor Wolfgang persona modes
pub mod types;          // shared types
```

### Persona system

`src/persona.rs`:

- `enum PersonaMode` — different Wolfgang moods (the contrarian-scholar persona has multiple modes; specifics inferred from naming, `confidence: stated for the enum, speculation for individual variants`).
- `fn load_persona(mode: PersonaMode, personas_dir: Option<&Path>) -> String` — loads the persona prompt for a given mode.

### Storage

- **LanceDB** v0.30 — vector store backend, Arrow-based, no separate server needed. The desktop path uses LanceDB on local disk; the backend path uses HNSW (see [[Architecture - Backend|Backend]]).
- **Arrow** v58 (`arrow-array`, `arrow-schema`) — columnar format for vector data.
- **base64, dirs, futures-util, eventsource-stream, reqwest, tokio** — runtime infrastructure.

## `crates/wolfgang-domain/`

Domain models + Organism integration.

### Notable dependencies

- **Organism** crates: `organism-pack`, `organism-runtime` (workspace) — formation/intent integration for [[Architecture - Desktop|Desktop]]'s `generate_team_formations` / `generate_panel_starters` / `run_team_step` Tauri commands.
- **Converge** crates: `converge-model`, `converge-kernel`, `converge-pack`, `converge-provider`, `converge-manifold-adapters` (with features: `anthropic`, `openai`, `openrouter`, `brave`, `tavily`) — multi-provider LLM and search adapter surface.
- **chrono** (serde), **serde**, **serde_json**, **async-trait**, **tokio** — runtime infrastructure.

The `manifold-adapters` feature set (`anthropic`, `openai`, `openrouter`, `brave`, `tavily`) tells you Wolfgang's reachable LLM + search backends: three LLM providers + two web-search providers (Brave + Tavily) for grounded-research.

## Boundary

Owns: LLM client integrations + persona system + RAG primitives + Organism-shaped domain types. Shared between [[Architecture - Desktop|Desktop]] and [[Architecture - Backend|Backend]] — neither owns chat semantics; both delegate to these libraries.
Does NOT own: Tauri-specific commands (→ [[Architecture - Desktop|Desktop]]), HTTP/gRPC handlers (→ [[Architecture - Backend|Backend]]), web UI primitives (→ [[Architecture - Web|Web]]), persistent vector storage policy choice (LanceDB vs. HNSW is each compute path's call).

## Cross-references

- [[Architecture - Overview|wolfgang-chat overview]]
- [[Architecture - Desktop|Desktop]] — local-first consumer
- [[Architecture - Backend|Backend]] — cloud consumer
- [[../../mosaic-extensions/Architecture - Manifold-adapters|Manifold Adapters]] — the converge crate Wolfgang consumes for multi-provider LLM access

<!-- @generated:end -->
