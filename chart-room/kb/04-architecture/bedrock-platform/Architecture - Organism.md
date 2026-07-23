---
type: architecture-module
source-path: bedrock-platform/organism/
last-scanned: 2026-06-07
scanned-version: 1.9.3
tags: [architecture, bedrock-platform, organism]
---

# Organism — Module Architecture

<!-- @generated:start -->

Per its own README:

> *"Organizational intelligence runtime. The reasoning layer between human intent and governed execution."*
> — `bedrock-platform/organism/README.md:12`

Part of [[Architecture - Overview|bedrock-platform]]. Workspace version `1.9.3`. Libraries-only — no binaries declared in the workspace.

## Workspace crates (12)

All members at `bedrock-platform/organism/crates/`:

| Crate path | Crate name | Purpose (from name + boundary doc) |
|---|---|---|
| `intent/` | `organism-intent` | Intent contracts |
| `planning/` | `organism-planning` | Planning strategies |
| `adversarial/` | `organism-adversarial` | Adversarial review |
| `simulation/` | `organism-simulation` | Simulation harness |
| `learning/` | `organism-learning` | Learning loop |
| `runtime/` | `organism-runtime` | Runtime composition |
| `intelligence/` | `organism-intelligence` | Intelligence primitives |
| `pack/` | `organism-pack` | Pack contract for Organism |
| `notes/` | `organism-notes` | Note primitives |
| `catalog/` | `organism-catalog` | Capability/formation catalog |
| `catalog-seed/` | `organism-catalog-seed` | Catalog seed fixtures |
| `dynamics/` | `organism-dynamics` | Dynamics primitives |

## Boundary

From [[../current-system-map|current-system-map]] §Boundaries:

- Owns: intent contracts, planning, adversarial review, simulation, formation selection.
- Does NOT own: promotion authority (→ [[Architecture - Converge|Converge]]), truth compilation (→ [[Architecture - Axiom|Axiom]]).

Domain pack and orchestration examples for Organism live in sibling `atelier-showcase/`, not in this repo.

## Cross-references

- [[../current-system-map|Current System Map]]
- [[Architecture - Overview|bedrock-platform overview]]
- [[../../organism-business/README|Organism business KB]]

<!-- @generated:end -->
