---
type: architecture-module
source-path: mosaic-extensions/ferrox-solvers/
last-scanned: 2026-06-06
tags: [architecture, mosaic-extensions, ferrox-solvers]
---

# Ferrox Solvers — Module Architecture

<!-- @generated:start -->

Part of [[Architecture - Overview|mosaic-extensions]]. Per its own README:

> *"Constraint solving as a Converge Suggestor."*
> — `mosaic-extensions/ferrox-solvers/README.md:14`

## Member crates (4)

Rust workspace at `ferrox-solvers/` with members:

- `crates/ortools-sys` — FFI bindings to Google OR-Tools (C++)
- `crates/highs-sys` — FFI bindings to HiGHS solver (C++/C)
- `crates/ferrox` — solver Suggestor over the FFI crates
- `crates/ferrox-server` — server surface for hosted solver use

The C++ and C source files in the mosaic scan (3 each) live in these `*-sys` crates.

## Key dependencies

From `ferrox-solvers/crates/ferrox/Cargo.toml:22-27`: `ferrox-ortools-sys` (gated by `ortools` feature), `ferrox-highs-sys` (gated by `highs` feature), `converge-pack`, `converge-model`, `converge-provider`.

## Role

Solver-as-Suggestor: a formation hands a problem to Ferrox, gets a constraint-satisfying or optimal assignment back, and **proposes** it as evidence to Converge. Selection between OR-Tools and HiGHS is a feature flag at the workspace level.

## Cross-references

- [[Architecture - Overview|mosaic-extensions overview]]
- [[Architecture - Soter-smt|Soter SMT]] — sibling for SMT-style assurance (CVC5) vs. Ferrox's optimization-style assignment

<!-- @generated:end -->
