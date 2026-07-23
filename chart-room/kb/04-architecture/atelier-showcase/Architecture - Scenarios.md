---
type: architecture-module
source-path: atelier-showcase/scenarios/
last-scanned: 2026-06-07
tags: [architecture, atelier-showcase]
---

# atelier-showcase — Scenarios

<!-- @generated:start -->

Part of [[Architecture - Overview|atelier-showcase]]. **23 scenarios**, each its own Rust crate at `atelier-showcase/scenarios/<name>/`. All declare `publish = false`. Together 41 source files.

The "engagement seed gallery" — copy a scenario to start a real customer engagement (see [[../forge-templates/Architecture - Overview|forge-templates/converge-engagement]] for the parallel template-style path).

## Grouped by domain

**Policy / governance (5):**
- `arbiter-compliance` — uses [[../mosaic-extensions/Architecture - Arbiter-policy|Arbiter Policy]] for compliance checks
- `arbiter-ferrox-solver-gallery` — Arbiter + Ferrox solvers paired
- `arbiter-governance` — Arbiter policy in a governance loop
- `layered-governance` — multi-level governance composition
- `cedar-smt-analysis` — Cedar + SMT analysis combining [[../mosaic-extensions/Architecture - Arbiter-policy|Arbiter]] and [[../mosaic-extensions/Architecture - Soter-smt|Soter]]

**Business processes (7):**
- `expense-approval` — expense workflow
- `loan-application` — loan underwriting
- `meeting-scheduler` — calendar-style scheduling
- `vendor-selection` — vendor comparison
- `crm-helm` — CRM-shaped scenario; **only scenario with its own binary** (`crm-helm-showcase`)
- `sec-edgar-live-filing` — live SEC EDGAR filing through [[../mosaic-extensions/Architecture - Embassy-ports|embassy-ports/sec-edgar]]
- `high-risk-claim-portfolio` — high-risk claim handling

**Optimization (7):**
- `lp-diet`, `mip-facility-location`, `n-queens-cp-sat`, `sudoku-cp-sat`, `jobshop-ft06`, `network-flow-transport`, `vrptw-comparison` — all consume [[../mosaic-extensions/Architecture - Ferrox-solvers|Ferrox]] (OR-Tools / HiGHS).

**Formation / org (4):**
- `round-driven-formation-design`, `truth-driven-formation`, `multi-plan-allocation`, `solver-policy-allocation` — Organism formation-selection patterns.

## Pattern

Each scenario is a thin crate that wires Bedrock + Mosaic + the atelier-domain packs into a runnable end-to-end demo. Most are library-only (test-driven); only `crm-helm` has a binary.

## Cross-references

- [[Architecture - Overview|atelier-showcase overview]]
- [[Architecture - Crates|Crates]] — `atelier-domain` is the dependency these scenarios share
- [[../mosaic-extensions/Architecture - Overview|mosaic-extensions]] — capability families exercised by these scenarios

<!-- @generated:end -->
