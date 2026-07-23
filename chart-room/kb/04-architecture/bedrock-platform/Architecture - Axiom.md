---
type: architecture-module
source-path: bedrock-platform/axiom/
last-scanned: 2026-06-07
scanned-version: 0.15.2
tags: [architecture, bedrock-platform, axiom]
---

# Axiom — Module Architecture

<!-- @generated:start -->

Per its own README:

> *"The truth layer — JTBD decoding, Truth Packages, validation, simulation, intent compilation, run verification, and decoder calibration for the Reflective stack."*
> — `bedrock-platform/axiom/README.md:15`

Part of [[Architecture - Overview|bedrock-platform]]. Workspace version `0.15.2`.

## Shape

Axiom is a **single Rust package**, not a workspace. The `axiom-truth` crate is declared at the root `Cargo.toml`. This is structurally different from its three siblings ([[Architecture - Converge|Converge]] 11, [[Architecture - Organism|Organism]] 12, [[Architecture - Helms|Helms]] 35).

Top-level layout:

- `axiom-truth/` library — the typed truth layer (declared in root `Cargo.toml`)
- `src/bin/cz/main.rs` — `cz` CLI binary (workspace orchestrator)
- `architecture/` — ADR and design docs (in-repo, not the KB)
- `kb/` — Axiom's own knowledge base
- `tests/` — test fixtures

## Entry points

- `cz` binary at `src/bin/cz/main.rs` — Axiom CLI / workspace orchestrator. Used directly from operator workflows and from CI.

## Boundary

From [[../current-system-map|current-system-map]] §Boundaries:

- Owns: JTBD/truth translation, verifier reports, calibration, intent compilation.
- Does NOT own: formation selection (→ [[Architecture - Organism|Organism]]), promotion (→ [[Architecture - Converge|Converge]]).

Per [[../applet-runtime-boundaries|applet-runtime-boundaries]]: Axiom also owns the applet manifest schema, Truth Package and IntentPacket compilation, verifier specs, WASM artifact contracts, run reports, lineage, and calibration. The applet manifest is the shared boundary object between Axiom, Helm, app repos, Arena, and Atelier.

## Recent structural change

**2026-05-12 (`af63388`, in bedrock-platform commit log)** — Axiom was extracted to its own repository. Bedrock's `axiom/` is now excluded via `.gitignore` and referenced by sibling working tree only. Treat path references to `bedrock-platform/axiom/` as a working-tree convention, not a git submodule.

## Cross-references

- [[../current-system-map|Current System Map]]
- [[../applet-runtime-boundaries|Applet Runtime Boundaries]]
- [[../decisions/2026-06-06-applet-runtime-boundaries|ADR — Applet Runtime Boundaries]]
- [[Architecture - Overview|bedrock-platform overview]]

<!-- @generated:end -->
