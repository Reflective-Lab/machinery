---
type: architecture-module
source-path: bedrock-platform/converge/
last-scanned: 2026-06-07
scanned-version: 3.9.2
tags: [architecture, bedrock-platform, converge]
---

# Converge — Module Architecture

<!-- @generated:start -->

Governance engine for the Reflective stack. Per its own README:

> *"A correctness-first, context-driven multi-suggestor runtime built in Rust. Converge owns the convergence loop, the promotion gate, runtime invariants, HITL pauses, and the integrity proof of a run."*
> — `bedrock-platform/converge/README.md:13-15`

Part of [[Architecture - Overview|bedrock-platform]]. Workspace version `3.9.2` (the 3.4.0 git tag is what [[Architecture - Overview|runtime-runway]] pins against — see [[../runtime-runway/Architecture - Overview|runtime-runway]]).

## Workspace crates (11)

All members at `bedrock-platform/converge/crates/`:

| Crate path | Crate name | Purpose (from name + boundary doc) |
|---|---|---|
| `model/` | `converge-model` | Domain types and value objects |
| `pack/` | `converge-pack` | Domain pack contract (the deliverable unit) |
| `protocol/` | `converge-protocol` | Wire protocol / contract surfaces |
| `kernel/` | `converge-kernel` | Convergence loop, promotion gate |
| `client/` | `converge-client` | Client-side API |
| `core/` | `converge-core` | Cross-cutting primitives |
| `provider/` | `converge-provider` | Provider trait + adapter contract |
| `experience/` | `converge-experience` | Experience / interaction primitives |
| `optimization/` | `converge-optimization` | Optimization helpers |
| `runtime/` | `converge-runtime` | Compatibility shell — see "Runtime status" below |
| `storage/` | `converge-storage` | Storage contracts |

Workspace also tracks `schema/` (protocol JSON definitions), `scripts/`, `experiments/`, and `kb/`.

## Runtime status

Per [[../../LOG|KB/LOG.md]] entry 2026-06-02 (`Converge runtime retirement`): the `converge-runtime` binary at `crates/runtime/src/main.rs` is now a **compatibility-only internal shell**. Canonical deployed runtime moved to [[../runtime-runway/Architecture - Overview|runtime-runway]], [[../../01-platform/README|Lattice Mesh]], [[../commerce-rails/Architecture - Overview|commerce-rails]], [[Architecture - Helms|Helms]], and app hosts. The active `converge.zone` Firebase Hosting `/api/**` rewrite to Cloud Run service `converge-runtime` was removed in the same change.

## Boundary

From [[../current-system-map|current-system-map]] §Boundaries:

- Owns: admission, governance, promotion, facts, criteria evaluation, protocol, runtime contracts, storage contracts.
- Does NOT own: formation selection (→ [[Architecture - Organism|Organism]]), truth compilation (→ [[Architecture - Axiom|Axiom]]), product consequence (→ [[Architecture - Helms|Helms]]).
- Mosaic specialists ([[../mosaic-extensions/Architecture - Overview|mosaic-extensions]]) propose evidence/capabilities through Converge-shaped contracts.

## Entry points

- `converge-runtime` binary at `crates/runtime/src/main.rs` — compatibility shell only (see Runtime status).

## Cross-references

- [[../current-system-map|Current System Map]]
- [[Architecture - Overview|bedrock-platform overview]]
- [[../../converge-business/README|Converge business KB]]

<!-- @generated:end -->
