---
type: architecture-module
source-path: atelier-showcase/crates/
last-scanned: 2026-06-07
scanned-version: 1.0.2
tags: [architecture, atelier-showcase]
---

# atelier-showcase — Crates

<!-- @generated:start -->

Part of [[Architecture - Overview|atelier-showcase]]. Three crates at `atelier-showcase/crates/`. 52 Rust source files total.

## atelier-domain

Path: `crates/atelier-domain/`. Library crate: `atelier_domain`.

Built-in domain packs and reference domain agents shipped with Converge for builders. ~6,300 LOC across modules: `ask_converge`, `domain_invariants`, `drafting`, `drafting_llm`, `eval_agent`, `evals`, `flow_governance`, `form_filler`, `llm_utils`, `lib.rs`. Domain packs cover **trust**, **money**, **delivery**, and **data_metrics**.

Published. The reusable spine that scenarios and tutorials depend on.

## organism-domain

Path: `crates/organism-domain/`. Not published (`publish = false`).

Organizational domain packs — reusable workflow patterns for autonomous organizations. Depends on `converge-pack`, `organism-pack`, `organism-runtime` (see [[../bedrock-platform/Architecture - Organism|Organism]]).

## quality-render

Path: `crates/quality-render/`. Not published. Binary: `quality-render`.

Reads `arena-tests/reports/history.jsonl` and regenerates `arena-tests/quality/dashboard.md`. Bridges the criterion-style benchmark history from [[../arena-tests/Architecture - Overview|arena-tests]] into a human-readable dashboard.

## Cross-references

- [[Architecture - Overview|atelier-showcase overview]]
- [[Architecture - Scenarios|Scenarios]] — consume the atelier-domain packs
- [[Architecture - Tutorials|Tutorials]] — consume the atelier-domain packs

<!-- @generated:end -->
