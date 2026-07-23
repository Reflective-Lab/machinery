---
type: architecture-module
source-path: atelier-showcase/tutorials/
last-scanned: 2026-06-07
tags: [architecture, atelier-showcase]
---

# atelier-showcase — Tutorials

<!-- @generated:start -->

Part of [[Architecture - Overview|atelier-showcase]]. **19 numbered tutorials**, each its own Rust crate at `atelier-showcase/tutorials/<NN-name>/`. All `publish = false`. Each ships a runnable `example-*` binary plus a `README.md` describing its learning goal.

The numbered learning spine. Read in order to onboard onto Converge.

## The spine

| # | Crate / binary | Learning goal (from crate name) |
|---|---|---|
| 01 | `01-hello-convergence` / `example-hello-convergence` | First convergence loop |
| 02 | `02-custom-agent` / `example-custom-agent` | Custom agent definition |
| 03 | `03-custom-provider` / `example-custom-provider` | Custom provider |
| 04 | `04-intent-codec-loop` / `example-intent-codec-loop` | Intent Codec round trip |
| 05 | `05-fixed-point-vs-budget` / `example-fixed-point-vs-budget` | Fixed-point convergence vs. budget cutoff |
| 06 | `06-reconciliation-loop` / `example-reconciliation-loop` | Reconciliation between proposed and admitted state |
| 07 | `07-adaptive-gap-loop` / `example-adaptive-gap-loop` | Adaptive gap closure |
| 08 | `08-live-formation` / `example-live-formation` | Live formation execution |
| 09 | `09-formation-mixed` / `example-formation-mixed` | Mixed-specialist formation |
| 10 | `10-formation-compiler` / `example-formation-compiler` | Formation compiler |
| 11 | `11-charter-from-intent` / `example-charter-from-intent` | Charter derived from intent |
| 12 | `12-shape-competition` / `example-shape-competition` | Shape competition |
| 13 | `13-topology-transition` / `example-topology-transition` | Topology transition |
| 14 | `14-debate-loop` / `example-debate-loop` | Debate-style adversarial review |
| 15 | `15-resolution-showcase` / `example-resolution-showcase` | Resolution patterns |
| 16 | `16-collab-discussion` / `example-collab-discussion` | Collaborative discussion |
| 17 | `17-collab-huddle` / `example-collab-huddle` | Collaborative huddle |
| 18 | `18-collab-panel` / `example-collab-panel` | Collaborative panel |
| 19 | `19-collab-self-organizing` / `example-collab-self-organizing` | Self-organizing collaboration |

Trajectory (inferred from titles, `confidence: speculation`): single-agent basics (01-04) → loop dynamics (05-07) → formations (08-10) → derived behaviors (11-13) → multi-agent collaboration (14-19).

## Cross-references

- [[Architecture - Overview|atelier-showcase overview]]
- [[Architecture - Crates|Crates]] — `atelier-domain` is the shared dependency
- [[../bedrock-platform/Architecture - Overview|bedrock-platform]] — the system these tutorials onboard onto

<!-- @generated:end -->
