---
type: architecture-overview
source-path: atelier-showcase/
last-scanned: 2026-06-07
scanned-commit: c33f175
scanned-version: 1.0.2
tags: [architecture, atelier-showcase]
---

# atelier-showcase — Architecture Overview

<!-- @generated:start -->

Per its own README:

> *"The Converge showcase — a numbered tutorial spine plus a gallery of end-to-end domain demos for builders new to Converge and the wider stack. Where Converge says what's possible, atelier says here's how it looks. Use it to learn the platform, seed a new engagement, or prove out an architectural idea."*
> — `atelier-showcase/README.md:14-19`

Rust workspace at version `1.0.2` (edition 2024, MSRV 1.94.0). The canonical place new domain packs and orchestration examples land per [[../current-system-map|current-system-map]].

## Stack composition

Scan at commit `c33f175`: Rust 112 files (64.0%), Markdown 62 (35.4%), Python 1.

Workspace shape (3 top-level kinds):

| Kind | Path | Count | Purpose |
|---|---|---|---|
| Library + tools | `crates/` | 3 crates | Reusable domain packs + dashboard tooling |
| Scenarios | `scenarios/` | 23 crates | End-to-end domain demos, each its own runnable crate |
| Tutorials | `tutorials/` | 19 crates | Numbered learning spine, `01-hello-convergence` → `19-collab-self-organizing` |
| Support | `scripts/` | 1 file | Out-of-band scripts (one Python file) |

## How the parts fit together

```mermaid
flowchart LR
    Learner([Learner / new builder])
    Engagement([New engagement seed])

    Tutorials["tutorials/<br/>(01..19 numbered)<br/>bins: example-*"]
    Scenarios["scenarios/<br/>(23 domain demos)<br/>all publish=false"]

    subgraph Crates["crates/"]
      direction TB
      AtelierDomain["atelier-domain<br/>(trust / money / delivery /<br/>data_metrics packs)"]
      OrganismDomain["organism-domain<br/>(org-pattern packs)"]
      QualityRender["quality-render<br/>(reads arena-tests/<br/>reports/history.jsonl)"]
    end

    Bedrock["bedrock-platform/<br/>+ mosaic-extensions/<br/>(via local path deps)"]
    Arena["arena-tests/<br/>reports/history.jsonl"]

    Learner --> Tutorials
    Engagement --> Scenarios
    Tutorials --> AtelierDomain
    Scenarios --> AtelierDomain & OrganismDomain
    AtelierDomain & OrganismDomain --> Bedrock
    Arena -. JSON .-> QualityRender
    QualityRender -. quality/dashboard.md .-> Arena
```

Atelier consumes [[../bedrock-platform/Architecture - Overview|bedrock-platform]] (Converge, Organism, Axiom, Helms) and [[../mosaic-extensions/Architecture - Overview|mosaic-extensions]] through local path dependencies. It produces (a) runnable example binaries that drive the platform, (b) reusable domain packs (`atelier-domain`, `organism-domain`), and (c) the markdown dashboard for arena-tests benchmark history.

## Personas

Quoted from the README + inferred from binary names; `confidence: stated` for first two, `speculation` for the rest.

- **Builder new to Converge** — works through `tutorials/01-19` in order. Each tutorial has a runnable `example-*` binary.
- **Engagement seed author** — copies a `scenarios/*` crate as the starting point for a real customer engagement.
- **Domain-pack author** — adds patterns to `crates/atelier-domain` or `crates/organism-domain` rather than re-implementing in every scenario.
- **Quality-bench maintainer** — runs `quality-render` to refresh the dashboard from [[../arena-tests/Architecture - Overview|arena-tests]] history.

## Module index

- [[Architecture - Crates|Crates]] — `atelier-domain`, `organism-domain`, `quality-render`
- [[Architecture - Scenarios|Scenarios]] — 23 end-to-end domain demos
- [[Architecture - Tutorials|Tutorials]] — the numbered 01..19 spine

## Entry points

- `quality-render` binary at `crates/quality-render/src/main.rs` — reads `arena-tests/reports/history.jsonl`, writes `arena-tests/quality/dashboard.md`.
- `crm-helm-showcase` binary at `scenarios/crm-helm/src/main.rs` — the single scenario with its own binary.
- 19 `example-*` binaries, one per tutorial crate.

## CI workflows

`atelier-showcase/.github/workflows/`:

- `ci.yml` — `cargo check --workspace`, `cargo test --workspace --all-targets`, clippy `-D warnings`, `rustfmt --check`. Plus a manual `solver-tests` job (ferrox + arbiter + prism end-to-end; OR-Tools install pending, `continue-on-error: true`).
- `coverage.yml` — `cargo-llvm-cov`; enforces 80% floor per [[../bedrock-platform/Architecture - Helms|Extension Release Checklist]] §4.
- `security.yml` — `cargo-audit --deny warnings`, `cargo-deny`, gitleaks v8.27.2.
- `stability.yml` — weekly (Mon 06:00 UTC): bench-compile, criterion bench-run uploading to `KB/Baselines/`, soak tests (1000 cycles, 100 concurrency, 200 iterations), security-blocking with RUSTSEC-2021-0141 ignored, deny.

## Boundary

Owns: reusable showcase domain packs + tutorial spine + scenario gallery + quality dashboard rendering.
Does NOT own: production app surfaces (→ [[../marquee-apps/Architecture - Overview|marquee-apps]] / [[../studio-apps/Architecture - Overview|studio-apps]]), cross-extension regression coverage (→ [[../arena-tests/Architecture - Overview|arena-tests]]), platform contracts (→ [[../bedrock-platform/Architecture - Overview|bedrock-platform]]).

## Cross-references

- [[../current-system-map|Current System Map]]
- [[../bedrock-platform/Architecture - Overview|bedrock-platform]]
- [[../mosaic-extensions/Architecture - Overview|mosaic-extensions]]
- [[../arena-tests/Architecture - Overview|arena-tests]] (consumer of the quality-render binary)
- [[../forge-templates/Architecture - Overview|forge-templates]] (parallel seed-style workspace)
- [[../README|04-architecture]] — domain hub

<!-- @generated:end -->
