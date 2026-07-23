# Engineering Principles & Development Guide

This domain explains how to build in this workspace.

## Principles Demonstrated In Code

- Explicit boundaries over implicit sharing: crates and workspaces name ownership even before every service is independently deployable.
- Domain ownership: products own projections and consequences; foundations own reusable contracts.
- Catalogs are not execution: `truth-catalog` describes registered truths, while `workbench-backend` and `helm-truth-execution` decide what is executable.
- Provider/adapter separation: Stripe, LLMs, OCR engines, and external sources stay behind typed boundaries.
- Compatibility before churn: Helm keeps `prio.*` proto namespaces and `prio-*` leaf crates while neutral names are introduced in stages.
- Observability and audit by default: Converge, ExperienceStore concepts, audit modules, and ledger/reconciliation paths are first-class in the architecture.
- Rust safety baseline: current Rust workspaces use edition 2024 where possible, workspace dependencies, Clippy, and `unsafe_code = "forbid"`.
- Predictable runtime over convenience in the core: commitment, governance, runtime, billing, provenance, policy, and cross-layer boundary code should prefer typed, compiled, observable implementations where the runtime behaviour can be reasoned about. AI-assisted development reduces the cost of using sharper tools; it does not justify pushing late-bound uncertainty into the system core.

## Good Patterns

| Pattern | Why it exists | Examples |
|---------|---------------|----------|
| Capability crate | Gives a bounded business capability an owned API surface | `prio-ledger`, `prio-entitlements`, `runway-auth`, `commerce-rails-contracts` |
| Registry/catalog | Lets code distinguish available definitions from runnable implementations | `capability-registry`, `truth-catalog`, `plugin-runtime` |
| Foundation binding | Integrates Converge/Organism/Axiom without treating them as generic external ports | Helm `truth-catalog`, `workbench-backend` |
| Thin JTBD app | Keeps product domain flow in apps while pulling reusable behavior from Bedrock and Mosaic | Catalyst, Keystone, Tally, Vouch, Warden |
| Adapter boundary | Keeps provider semantics from becoming domain truth | Commerce Rails Stripe adapter, Manifold providers, Embassy ports |
| Workbench backend | Keeps UX-facing summaries and app state out of lower foundations | `workbench-backend` |
| Proof repo | Keeps tutorials, demonstrations, and cross-extension checks out of foundation crates | `atelier-showcase`, `arena-tests` |

## Anti-Patterns

- Do not describe planned architecture as implemented outside `08-roadmap`.
- Do not add product-specific UX assumptions to Converge, Organism, Mosaic, Runtime Runway, or Commerce Rails.
- Do not make provider IDs or Stripe object IDs the primary domain truth.
- Do not document a truth as executable unless a code path exposes it.
- Do not import source code wholesale into the KB; summarize boundaries and link to the owning repo.
- Do not introduce dynamic-language, VM-heavy, or high-churn UI/runtime layers across commitment boundaries unless the boundary is protected by typed contracts, adapters, tests, and explicit ownership. These tools are not forbidden; they must earn their place where truth, authority, obligation, money, or reopen decisions cross layers.
- Do not move fake-backed or mixed-resource contract-shape cases into Atelier
  to make a demo look complete; keep them in `arena-tests` until the
  live/local-real path exists.

## Canonical Links

- [[../business-tech-map|Business / Tech Map]]
- [[../outcome-workbench/kb/Architecture/Naming Migration Map|Helm Naming Migration Map]]
- [[../outcome-workbench/kb/Architecture/Application Layer Restructure|Application Layer Restructure]]
- [[../04-architecture/current-system-map|Current System Map]]
