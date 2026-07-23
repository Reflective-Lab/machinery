---
type: architecture-module
source-path: studio-apps/wolfgang-chat/apps/desktop/
last-scanned: 2026-06-07
scanned-version: 1.2.0
tags: [architecture, studio-apps, wolfgang-chat]
---

# wolfgang-chat — Desktop

<!-- @generated:start -->

Part of [[Architecture - Overview|wolfgang-chat]]. Local-first Tauri v2 desktop app — runs the LLM/RAG stack in-process via Tauri commands. The user's content stays on the device.

## Shape

- **Tauri v2 shell:** `apps/desktop/src-tauri/`
  - Package: `wolfgang` v1.2.0
  - App identifier: `bot.wolfgang.desktop`
  - Bundle targets: `dmg`, `nsis`, `deb`, `appimage` (macOS + Windows + Debian + AppImage)
  - Window: 1200×800, min 600×400
- **SvelteKit frontend:** `apps/desktop/src/` — embedded into the Tauri shell at build (`../build` dist)
- **Dev URL:** `http://localhost:3000` (frontend dev server)

## Tauri commands (`#[tauri::command]` in `src-tauri/src/main.rs`)

15+ commands across `main.rs:73-1315`. Grouped by purpose:

**Chat / streaming:**
- `chat_stream()` — primary chat endpoint
- `complete_with_healthy_backend()` — backend health probe

**Deep research:**
- `start_deep_research_run()` / `resume_deep_research_run()`

**Team formations:**
- `generate_team_formations()` / `generate_panel_starters()` / `run_team_step()` — Organism formation integration

**Knowledgebase:**
- `list_knowledgebases()` / `import_knowledgebase()`
- `load_expert_images()`

**Conversations:**
- `list_conversations()` / `load_conversation()` / `save_conversation()` / `delete_conversation_db()`

**Brand customization:**
- `create_brand()`

## Tauri plugins (all v2.x)

- `tauri-plugin-shell` 2.3.5 — shell command execution
- `tauri-plugin-dialog` 2.6.0 — native file dialogs
- `tauri-plugin-fs` 2.4.5 — filesystem access
- `tauri-plugin-clipboard-manager` 2.3.2 — clipboard
- `tauri-plugin-opener` 2.5.3 — open URLs/files
- `tauri-plugin-os` 2.3.2 — OS info

## Direct dependencies (notable)

From `src-tauri/Cargo.toml`:

- `wolfgang-core`, `wolfgang-domain` (local path) — [[Architecture - Core|shared libraries]]
- `converge-*` (workspace; see workspace `[patch.crates-io]` for local-head pinning)
- `lancedb`, `arrow`, `parquet` — local vector store + columnar format
- `pdf-extract`, `scraper`, `tiktoken-rs` — document ingestion + tokenization
- `reqwest`, `tokio`, `uuid`

## Secondary binary: `ingest`

`apps/desktop/src-tauri/src/ingest.rs` — separate binary for batch knowledgebase ingestion. Workspace member declares both `src/main.rs` (default) and `src/ingest.rs` (ingest tool). Justfile recipe `just ingest` calls it.

## Boundary

Owns: native desktop shell + local-first compute (Tauri commands call `wolfgang-core` directly; no cloud round-trip).
Does NOT own: cloud backend (→ [[Architecture - Backend|Backend]]), shared LLM/RAG library code (→ [[Architecture - Core|Core]]), web product (→ [[Architecture - Web|Web]]).

## Cross-references

- [[Architecture - Overview|wolfgang-chat overview]]
- [[Architecture - Core|Core]] — the shared library this shell invokes
- [[Architecture - Web|Web]] — the parallel compute path

<!-- @generated:end -->
