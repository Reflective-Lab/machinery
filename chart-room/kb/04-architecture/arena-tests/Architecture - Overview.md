---
type: architecture-overview
source-path: arena-tests/
last-scanned: 2026-06-07
scanned-commit: 44796f2
tags: [architecture, arena-tests]
---

# arena-tests — Architecture Overview

<!-- @generated:start -->

Per its own README:

> *"Cross-extension integration and arena tests for the Reflective stack. This repo wires Bedrock and Mosaic crates together in one test-only workspace, so it can catch regressions and composition gaps that single-repo tests miss."*
> — `arena-tests/README.md:1-5`

Test-only Rust workspace. 19 Rust source files at commit `44796f2`.

## Why arena-tests is its own repo

Quoted verbatim from `arena-tests/README.md:7-13`:

> *"Bedrock crates must not depend on Mosaic extensions. Putting cross-extension tests inside Converge, Organism, Axiom, or Helm would invert the dependency direction. This repo sits at the workspace root and consumes Bedrock plus Mosaic through local path dependencies, so the platform stays clean while still getting integration coverage."*

This is the **dependency-direction firewall**: keeps Bedrock pure, lets the test layer reach across both Bedrock and Mosaic.

## Stack composition

- Rust workspace, all members at `crates/`. All `publish = false` (`README.md:52`).
- Source counts: Rust 19 files (82.6%), Markdown 4 files (17.4%).
- No CI workflows in this scan (signals.ci: false).

## Member crates

From `arena-tests/README.md:17-23`:

| Crate | Role |
|---|---|
| `cross-extension-smoke` | Smoke + composition tests; most logic lives in the crate's `tests/` directory rather than `src/`. |
| `intent-cases` | Shared business-intent fixtures used by Organism routing tests. |
| `counterparty-kyc-convergence` | **Live-by-default arena binary** for counterparty identity, sanctions, and procurement evidence. |

## Current coverage

From `arena-tests/README.md:25-31`, the claim-portfolio coverage today:

- Expense non-finance high-value commit exemplar
- Strict HITL rejection when approval would not change the Cedar decision
- Vendor due-diligence gate
- Flow phase commit gate
- Data-classification PII block

## How a test composes

```mermaid
flowchart LR
    Bedrock["bedrock-platform/<br/>(Converge, Organism, Axiom, Helms)"]
    Mosaic["mosaic-extensions/<br/>(arbiter, ferrox, manifold, etc.)"]
    Atelier["atelier-showcase/<br/>(domain packs)"]

    subgraph Arena["arena-tests/crates/"]
      Smoke["cross-extension-smoke"]
      Intent["intent-cases"]
      KYC["counterparty-kyc-convergence<br/>(live-by-default)"]
    end

    Bedrock -. local path .-> Arena
    Mosaic -. local path .-> Arena
    Atelier -. local path .-> Arena

    Arena -. history.jsonl .-> QualityRender["atelier-showcase/<br/>quality-render"]
```

The flow of dependencies is one-way: Arena pulls Bedrock + Mosaic + Atelier from local paths. None of those repos depend on Arena.

## Personas

Inferred from README + crate names; `confidence: speculation`.

- **Platform contributor** — runs `cargo test --workspace` before submitting cross-repo PRs; catches composition regressions.
- **KYC engagement author** — uses `counterparty-kyc-convergence` as the live arena demo.

## Contract (from the repo's own rules)

Quoted from `arena-tests/README.md:50-54`:

- No production code. Test-only repo (`publish = false` everywhere).
- Don't pull from crates.io for any Bedrock or Mosaic crate — always use the local path. If a `[patch.crates-io]` entry is missing for a transitive dep, add it.
- Keep tests deterministic. Wall-clock comparisons must strip timestamps (see the pattern in `engine_converges_deterministically` in `converge-core`'s test).

## Cross-references

- [[../current-system-map|Current System Map]]
- [[../atelier-showcase/Architecture - Crates|atelier-showcase/quality-render]] — consumer of `arena-tests/reports/history.jsonl`
- [[../bedrock-platform/Architecture - Overview|bedrock-platform]]
- [[../mosaic-extensions/Architecture - Overview|mosaic-extensions]]
- [[../README|04-architecture]] — domain hub

<!-- @generated:end -->
