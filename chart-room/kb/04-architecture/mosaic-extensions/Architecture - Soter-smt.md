---
type: architecture-module
source-path: mosaic-extensions/soter-smt/
last-scanned: 2026-06-06
tags: [architecture, mosaic-extensions, soter-smt]
---

# Soter SMT — Module Architecture

<!-- @generated:start -->

Part of [[Architecture - Overview|mosaic-extensions]]. Per its own README:

> *"SMT-backed safety and policy assurance for Converge formations."*
> — `mosaic-extensions/soter-smt/README.md:14-20`

## Member crates (2)

Rust workspace at `soter-smt/` with members:

- `crates/cvc5-sys` — FFI bindings to the CVC5 SMT solver
- `crates/soter` — SMT Suggestor over the FFI crate

## Key dependencies

From `soter-smt/crates/soter/Cargo.toml:16-25`: `soter-cvc5-sys` (gated by `cvc5` feature), `converge-pack`, `tokio`, `serde`, `sha2`.

## Role

Safety and policy **assurance**: SMT-level checks that a candidate plan or assignment satisfies declared constraints. Sibling to [[Architecture - Arbiter-policy|Arbiter Policy]] (Cedar-style policy authorization) and [[Architecture - Ferrox-solvers|Ferrox]] (optimization-style assignment). Soter's role is provability under SMT, not optimality.

## Recent structural change

**2026-05-21 (`402e5c2`)** — Test/spec doubles renamed `Fake*` → `Scripted*` in Soter (and Manifold). Closed Soter + Manifold violation cases.

## Cross-references

- [[Architecture - Overview|mosaic-extensions overview]]
- [[Architecture - Arbiter-policy|Arbiter Policy]] — Cedar-policy sibling
- [[Architecture - Ferrox-solvers|Ferrox Solvers]] — optimization sibling

<!-- @generated:end -->
