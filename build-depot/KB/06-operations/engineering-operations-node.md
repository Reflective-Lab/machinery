---
source: mixed
---

# Engineering Operations Node — the continuous software factory machine

**Decision (Karl, 2026-07-02):** the newly bought 12-core Intel Mac Pro
(96 GB RAM, 7 TB storage) becomes a dedicated **Engineering Operations
Node** — a continuous software factory — rather than an AI workstation.
Factory workloads are CPU-, memory-, and I/O-bound, not GPU-bound; an
Intel Mac Pro is still very capable engineering infrastructure. Larger
reasoning tasks stay on the GX10 or cloud providers.

## What it runs continuously

- **CI builds** — compile every Rust crate; build iOS/Android components
  where applicable; documentation generation; verify examples compile.
- **Quality gates** — `cargo test`, `clippy`, `fmt --check`, `audit`,
  `deny`, `llvm-cov`; mutation testing where appropriate.
- **Security** — secret scanning, dependency vulnerability checks, SBOM
  generation, license compliance, static analysis, binary inspection.
- **Performance** — criterion benchmarks per crate, regression detection,
  memory/allocation profiling, startup latency tracking.
- **Architecture health** — dependency graph generation, cycle detection,
  API compatibility checks, layering-rule enforcement, crate-size and
  compile-time monitoring.
- **Documentation** — verify READMEs, check code examples compile, detect
  stale docs, generate architecture diagrams, keep the Obsidian KB
  synchronized with the codebase.
- **Repository intelligence** — index every repo, cross-repository symbol
  graph, duplicate-code detection, unused-module detection, refactoring
  suggestions.
- **AI-assisted engineering** — small local models for code
  classification, summarization, documentation; larger reasoning tasks
  delegated to the GX10 or cloud.

## The bigger idea: guardian of software quality

Not CI as a pass/fail gate, but continuous evaluation of system
properties: security, performance, reliability, scalability,
maintainability, documentation quality, API consistency, test coverage,
build reproducibility, technical debt, dependency health, architectural
drift, developer experience.

Each becomes a **typed quality signal that feeds into Converge**. Over
time, define Business Truths such as:

- "No critical security vulnerabilities."
- "Compile time must not regress by more than 10%."
- "Public APIs require documentation."
- "All architectural invariants must hold."
- "Every release produces a signed SBOM."

The Convergence Kernel then reasons over these signals the same way it
reasons over business evidence. **The dogfooding story: the first
"organization" Converge helps manage is the development of Converge
itself.**

`build-depot/` is the software implementation of this control plane: a
Bun/TypeScript Trigger.dev worker plus Omnigraph schema that turns GitHub,
Linear, Sentry, and scheduled fleet observations into typed quality,
security, delivery, repository, and incident facts.

## Where this connects to what exists today

The 2026-07-02 green-main program built the seeds this node would take
over and extend:

- `just factory-status` + `factory-alert.yml` (root repo) — the fleet
  andon light. The node would run the board continuously instead of on a
  daily cron, and could execute the whole fleet's `just ci` locally
  instead of waiting for GitHub runners.
- `KB/05-engineering/standards/ci-parity.md` — the canonical `just ci`
  shape means the node runs the exact same gates as GitHub Actions;
  parity extends from "laptop vs CI" to "factory node vs CI".
- World-red vs code-red triage (andon section of ci-parity.md) — advisory
  waves like today's (quick-xml, anyhow, proc-macro-error2) are exactly
  the class of signal the node should catch on ITS schedule, fleet-wide,
  in one sweep — instead of 16 repos discovering them one scheduled
  workflow at a time.
- `QUALITY_BACKLOG.md` QF findings + RP properties — today's findings
  (e.g. `QF-2026-07-02-02` layering inversion, `QF-2026-07-02-03`
  fmt-check sibling coupling) are early, hand-written instances of the
  "architectural drift" and "quality signal" classes the node would emit
  mechanically.
- `build-depot/` — graph-backed implementation of those signals, with Trigger
  tasks for PR review and debt tracking plus the `build-depot.pg` graph schema.
- Fresh-clone budgets in `release-train.yaml` — already the compile-time
  regression baseline the node would enforce continuously.

## Open questions

- Runner form: self-hosted GitHub Actions runner (keeps GitHub as the
  system of record) vs. independent scheduler pushing status to Converge
  (keeps the factory sovereign)? Likely both, phased.
- Signal schema: what does a typed quality signal look like as a Converge
  fact — and which existing RP-* properties translate directly into the
  first Business Truths?
- Intel Mac constraints: macOS on Intel is off Apple's newest OS support
  path; evaluate Linux on the Mac Pro vs. staying on macOS for the
  iOS-build capability.

## Cross-references

- [[ci-parity|Standard: CI parity (RP-CI-PARITY)]] —
  `KB/05-engineering/standards/ci-parity.md`
- `KB/06-operations/ci-cd-inventory.md` — the 16-repo audit
- `KB/06-operations/factory-health.md` — root-repo doctor recipes
- `build-depot/docs/operations/software-factory-quality-system.md` — quality/security/delivery loop
- `build-depot/docs/architecture/software-factory-build-depot.md` — Build-Depot architecture
- `QUALITY_BACKLOG.md` — QF ledger and RP properties
- `release-train.yaml` — fleet membership, order, fresh-clone budgets
