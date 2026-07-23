# Current System Map

Last audited: 2026-05-31. Source order: running code, public manifests,
domain models, then existing docs.

> **This file is the canonical boundary registry.** Per-project READMEs and
> AGENTS.md files should quote their boundary row from `## Project Boundary
> Anchors` (below) verbatim with a link back to the anchor — not paraphrase
> and not re-state. If a README's quote drifts from the registry, the
> mismatch is the bug, not the registry. Update this file first; then sweep
> the citing README(s).

## Workspace Shape

The root repository is the coordination and knowledge layer. It tracks
`README.md`, `AGENTS.md`, `SKILLS.md`, `QUALITY_BACKLOG.md`, and `KB/**`;
implementation folders
at the root are nested projects with their own Git history and release cadence.

## Core Responsibilities

| Area | Home | Purpose | Public interfaces |
|------|------|---------|-------------------|
| Converge | `bedrock-platform/converge` | Governance engine, semantic model, pack contract, runtime/client/protocol/storage surfaces | 11 Rust crates at version `3.9.1` |
| Organism | `bedrock-platform/organism` | Intent, planning, adversarial review, simulation, learning, runtime, catalog, notes, and dynamics | 12 Rust crates at version `1.9.1` |
| Axiom | `bedrock-platform/axiom` | Truth/JTBD compilation, verifier reports, calibration, provenance, CLI | `axiom-truth` crate version `0.15.0`, `cz` binary |
| Helm / Helms | `bedrock-platform/helms` | Bedrock trust-transfer and operator-control surface for governed jobs. Operator/receipt vocabulary lives in `helm-module-contracts` (RFL-154) and the event/lease/session substrate contracts in `helm-event-substrate` (RFL-171) — both neutral contracts/ crates that Bedrock owns; runtime-runway implements and re-exports them. As of RFL-154: `helm-operator-control` carries no app-side deps (workbench-backend, prio-agent-ops, polars all cut); `prio-agent-ops` is a capability manifest only | 35 Rust crates, Svelte/Tauri desktop, proto packages under `proto/prio/*/v1` |
| Mosaic Extensions | `mosaic-extensions` | Reusable Converge specialists: policy, models, ports, solvers, adapters, memory, analytics, SMT | Independent Rust workspaces/crates |
| Atelier Showcase | `atelier-showcase` | Tutorial spine and live/local scenario gallery for stack composition | Rust workspace with scenario, tutorial, and domain crates |
| Arena Tests | `arena-tests` | Cross-extension integration and contract-shape pressure tests | Rust workspace using local Bedrock/Mosaic path dependencies |
| Runtime Runway | `runtime-runway` | Runtime operations, app host, accounts, auth, storage, secrets, telemetry, LLM/GPU paths | 11-crate Rust workspace at `3.4.1`, currently pinned to Converge `v3.4.0` release tags |
| Commerce Rails | `commerce-rails` | Commercial authority: contracts and Stripe adapter boundary | `commerce-rails-contracts`, `commerce-rails-stripe` |
| Chart-Room | `chart-room` (private) | Company operations: strategy alignment lenses, lens packs, strategy gates, drift checks, roadmap; fixture validator | Private repo; extracted from converge/kb/Governance 2026-07-09 (17 commits history) |
| Build-Depot | `build-depot` | Software-factory control plane: quality/security/delivery graph, PR gate, incident and repository health normalization | Bun/TypeScript Trigger.dev worker plus Omnigraph schema |
| Marquee Apps | `marquee-apps` | Thin JTBD apps and product workspaces built on Bedrock and Mosaic (a planned doctrinal category overlay lives in reflective-paradigm §6; not yet reflected in physical homes) | quorum-sense, plumb-execution, vouch-lending, atlas-integration, warden-compliance, scout-sourcing, tally-escrow, catalyst-biz, triage-keeper, fathom-narrative |
| Blueprint Apps | `blueprint-apps` | KB-first doctrine and meta apps, not shipped products | keystone-architecture, shoal-meta |
| Studio Apps | `studio-apps` | Creative, research, notes, writing, and presentation apps moved out of marquee where appropriate | Wolfgang, Inkling, Folio, Moosemen, Wykkid, and related studio/productivity workspaces |
| Web | `www` | Public web properties and app portal | Svelte/SvelteKit projects with Firebase Hosting targets |
| Mobile Apps | `mobile-apps` | Native Converge clients | Android Gradle project and iOS Swift package/project |

## Helm Truth State

The code-backed catalog in `bedrock-platform/helms/crates/truth-catalog`
contains 23 registered definitions:

- 18 cataloged job truths
- 3 policy truths
- 2 module-local invariants

There are 24 `.feature` files under `truths/`; `generate_data_transformer` is
present as a feature file but is not registered in `TRUTHS` today.

The `workbench-backend` executable surface inside Helm currently supports four truth keys:

- `qualify-inbound-lead`
- `submit-expense-report`
- `activate-subscription`
- `refill-prepaid-ai-credits`

Other catalog entries may expose metadata, Converge bindings, or evaluators, but
should not be documented as executable unless `is_truth_supported`
includes them or a registered `TruthBody` mounts them through
`helm-truth-execution`.

## Boundaries

- Bedrock owns Helm, Axiom, Organism, and Converge as the core system base.
- Products own thin JTBD app surfaces, domain projections, UX state, and
  application-specific consequences.
- Axiom translates jobs and truths into runtime contracts; it does not select
  formations or promote facts.
- Organism selects formations and planning strategy; it does not own promotion
  authority.
- Converge owns admission, governance, promotion, facts, criteria evaluation,
  protocol, runtime, and storage contracts.
- Mosaic specialists propose evidence or capabilities through Converge-shaped
  contracts; they do not own product consequence.
- Runtime Runway owns operational plumbing: app host, auth, storage, secrets,
  telemetry, deployment, LLM/GPU runtime paths.
- Commerce Rails owns commercial state and obligations; Stripe is an adapter.
- Chart-Room owns company-operations governance: strategy lenses, gates,
  drift, roadmap. It governs Reflective's decisions, not the software or the
  platform runtime.
- Build-Depot owns software-factory observations, graph facts, and scorecard
  inputs. It records and routes drift; it does not take over product, runtime,
  commercial, or Helm authority.

## Recurring Patterns

- Rust workspaces use edition 2024, `rust-version` around `1.94`, workspace
  dependencies, and `unsafe_code = "forbid"`.
- Capability boundaries are explicit crates before they are fully independent
  services.
- Catalogs describe what exists; separate execution modules decide what is
  currently runnable.
- Human-facing web surfaces are Svelte/SvelteKit, usually deployed through
  Firebase Hosting with project-local `Justfile` recipes.
- Runtime code increasingly separates reusable foundations from app-specific
  projections: foundations stay in Converge/Organism/Mosaic, while products
  compose them.

## Review Notes

- Deployment status here is derived from local Firebase config and workflows,
  not live DNS or hosting checks.
- Runtime and cross-layer injection boundaries are diagrammed in
  [[runtime-injection-boundaries|Runtime and Injection Boundary Diagrams]].
- Runtime Runway still depends on Converge `v3.4.0` release tags even though the
  local Converge workspace is `3.9.2`; treat that as an intentional lag until
  code or release notes say otherwise.
- Several historical handoff notes under the legacy `KB/outcome-workbench` path still reference old names
  (`crm-*`, `prio-truths`, `prio-modules`). Prefer the migration map and current
  Cargo workspace when starting new work.

## Project Boundary Anchors

Each H3 below is a stable anchor a per-project README quotes verbatim. The
blockquote IS the canonical claim; everything else is metadata. Update the
blockquote here first when a boundary changes; then sweep the citing READMEs
([[decisions/templates/retirement-adr|retirement-ADR template]] codifies the
sweep checklist).

Format per anchor: blockquote (the copyable claim) + Home + Version + Last
reviewed + deep-dive link. Workspace versions reflect the latest
`/obsidian-architect` scan; bump on refresh, not on every commit.

### Converge

> Owns: admission, promotion, facts, criteria evaluation, protocol, runtime
> contracts, storage contracts. Does NOT own: formation selection (→ Organism);
> truth compilation (→ Axiom); product consequence (→ Helms).

- Home: `bedrock-platform/converge/`
- Version: v3.9.2 (workspace; the deployed `converge-runtime` binary is a
  compatibility-only shell — see [[decisions/2026-06-02-converge-runtime-retirement|2026-06-02 retirement ADR]])
- Last reviewed: 2026-06-07
- Deep dive: [[bedrock-platform/Architecture - Converge]]

### Organism

> Owns: intent contracts, planning, adversarial review, simulation, formation
> selection. Does NOT own: promotion authority (→ Converge); truth compilation
> (→ Axiom).

- Home: `bedrock-platform/organism/`
- Version: v1.9.3 (workspace, libraries-only)
- Last reviewed: 2026-06-07
- Deep dive: [[bedrock-platform/Architecture - Organism]]

### Axiom

> Owns: JTBD/truth translation, verifier reports, calibration, intent
> compilation, applet manifest schema. Does NOT own: formation selection (→
> Organism); promotion (→ Converge).

- Home: `bedrock-platform/axiom/` (extracted to its own repo 2026-05-12; the
  bedrock-platform path is a working-tree convention)
- Version: `axiom-truth` v0.15.2 (single crate + `cz` CLI binary)
- Last reviewed: 2026-06-07
- Deep dive: [[bedrock-platform/Architecture - Axiom]]

### Helms

> Owns: trust-transfer surfaces, workbench views, operator-facing consequence,
> manifest intake, operator review, truth-catalog binding, sandbox lifecycle,
> approval points, audit visibility. Does NOT own: applet authority/schema (→
> Axiom); domain mutation in product apps (→ marquee/studio repos); commercial
> state (→ Commerce Rails).

- Home: `bedrock-platform/helms/`
- Version: v0.2.1 (35 crates + Tauri desktop + Svelte web + `proto/prio/*/v1`)
- Last reviewed: 2026-06-07
- Deep dive: [[bedrock-platform/Architecture - Helms]]

### Mosaic Extensions

> Owns: reusable specialist capability families (policy, models, ports,
> solvers, adapters, memory, analytics, SMT) that propose evidence/capabilities
> through Converge-shaped contracts. Does NOT own: product consequence; admission
> or promotion (→ Converge).

- Home: `mosaic-extensions/` — 8 capability workspaces + 1 integration harness
- Capability families: embassy-ports, manifold-adapters, prism-analytics,
  ferrox-solvers, mnemos-knowledge, arbiter-policy, crucible-models, soter-smt
- Last reviewed: 2026-06-07
- Deep dive: [[mosaic-extensions/Architecture - Overview]]

### Runtime Runway

> Owns: auth, app host, storage, secrets, telemetry, deployment runtime,
> LLM/GPU paths, **managed-service wrappers** (Pub/Sub, Spanner, Memorystore).
> Does NOT own: governance (→ Converge); commercial state (→ Commerce Rails);
> in-process distributed consensus (→ Lattice Mesh).

- Home: `runtime-runway/`
- Version: v3.4.2 workspace, pinned to Converge `v3.4.0` git tag (intentional
  lag — see [[decisions/2026-05-23-runway-config-injection|2026-05-23 ADR]])
- Last reviewed: 2026-06-07
- Deep dive: [[runtime-runway/Architecture - Overview]]

### Commerce Rails

> Owns: commercial state, billing, entitlement, marketplace, payout,
> reconciliation; commercial-authority contracts and provider adapters. Does
> NOT own: runtime operations (→ Runtime Runway); product consequence (→
> marquee/studio apps).

- Home: `commerce-rails/`
- Version: v0.1.2 — 2 crates (contracts + Stripe adapter)
- Last reviewed: 2026-06-07
- Deep dive: [[commerce-rails/Architecture - Overview]]

### Lattice Mesh

> Owns: distributed execution mesh — orchestration (Docker Compose,
> container lifecycle, service mesh), consensus (in-process Rust, Organism HITL
> quorum), self-contained solver services, persistence (databases, vector
> stores, object storage), node provisioning. Does NOT own: cognitive engine
> (→ Converge); domain packs (→ Organism); **managed-service wrappers** (→
> Runtime Runway).

- Home: `lattice-mesh/` (planning stage — AGENTS.md is canonical, no source yet)
- Status: declared boundary, no implementation. The "in-process consensus"
  responsibility currently has no implementing code.
- Last reviewed: 2026-06-07
- Deep dive: [[lattice-mesh/Architecture - Overview]]

### Atelier Showcase

> Owns: tutorial spine (numbered 01–19), scenario gallery (23 demo crates),
> reusable showcase domain packs (`atelier-domain`, `organism-domain`), and the
> `quality-render` dashboard tool. Does NOT own: production app surfaces (→
> marquee/studio); platform contracts (→ Converge); cross-extension regression
> coverage (→ Arena Tests).

- Home: `atelier-showcase/`
- Version: v1.0.2
- Last reviewed: 2026-06-07
- Deep dive: [[atelier-showcase/Architecture - Overview]]

### Arena Tests

> Owns: cross-extension integration + contract-shape pressure tests. **Test
> code only** (`publish = false` everywhere) — provides the dependency-direction
> firewall (Bedrock must not depend on Mosaic). Does NOT own: any production
> code; CI for Bedrock or Mosaic themselves.

- Home: `arena-tests/`
- Composition: pulls Bedrock + Mosaic + Atelier via local path dependencies
- Last reviewed: 2026-06-07
- Deep dive: [[arena-tests/Architecture - Overview]]

### Marquee Apps

> Owns: thin JTBD product surfaces, per-app domain state machines, app-specific
> adapters that emit observations. Does NOT own: governance contracts (→
> Converge); commercial state (→ Commerce Rails); runtime plumbing (→ Runtime
> Runway); specialist capabilities (→ Mosaic).

- Home: `marquee-apps/`
- Sub-apps (physically resident): catalyst-biz, scout-sourcing,
  fathom-narrative, atlas-integration, quorum-sense, tally-escrow,
  plumb-execution, warden-compliance, triage-keeper, vouch-lending
- Planned doctrinal category overlay (not yet reflected in physical homes):
  see [[../01-platform/reflective-paradigm|reflective-paradigm]] §6 for the
  proposed cut — Marquee drivers plus Applied apps sorted by commitment shape
  (convened burst / multi-sovereign / standing governance), truth-preserving
  projection, Reframe-watch, and Platform-component. This registry continues
  to track physical homes until repos move.
- Last reviewed: 2026-06-17
- Deep dive: [[marquee-apps/Architecture - Overview]]

### Studio Apps

> Owns: creative, research, notes, writing, and presentation product surfaces;
> per-app domain state; local-first storage where applicable. Does NOT own:
> thin JTBD commercial surfaces (→ Marquee Apps); governance (→ Converge);
> runtime plumbing (→ Runtime Runway).

- Home: `studio-apps/`
- Sub-apps (physically resident): wolfgang-chat (flagship), wykkid-preso,
  inkling-notes, folio-editor ("Newspaper"), moosemen-writer
- Planned doctrinal category overlay (reflective-paradigm §6): wolfgang-chat
  is category-adjacent; folio-editor is a truth-preserving projection fit.
  Not yet a physical move.
- Last reviewed: 2026-06-17
- Deep dive: [[studio-apps/Architecture - Overview]]

### Blueprint Apps

> Owns: KB-first doctrine and meta artifacts that explain how the portfolio
> composes as one decision lifecycle. Does NOT own: shipped product surfaces
> (→ Marquee/Studio); governance (→ Converge); runtime plumbing (→ Runtime
> Runway).

- Home: `blueprint-apps/`
- Sub-apps: keystone-architecture (constraint-driven structure search across
  the decision lifecycle), shoal-meta (composed-decision stack map)
- Note: moved from `marquee-apps/` on 2026-06-09; doctrine, not products.
- Last reviewed: 2026-06-17
- Deep dive: [[../01-platform/reflective-paradigm|reflective-paradigm]] §6

### Mobile Apps

> Owns: native iOS (SwiftUI) + Android (Compose) capture surfaces, on-device AI
> preprocessing, consent gates, structured-packet handoff to the platform via
> UniFFI/Rust. Does NOT own: governance, fact promotion, billing semantics,
> product invariants — "It must not silently decide, promote facts, run
> product invariants, or bypass consent" (`mobile-apps/README.md:120-121`).

- Home: `mobile-apps/` (intentionally early — product lab + native foundation)
- Target candidates: 1:1 with Marquee (10) + Studio (5)
- Last reviewed: 2026-06-07
- Deep dive: [[mobile-apps/Architecture - Overview]]

### Beacon Sites

> Owns: public-facing web content, app portal hosting, Firebase Hosting +
> Firestore rules **per site**. Does NOT own: app product code (→ studio/marquee);
> backend logic (→ Runtime Runway).

- Home: `beacon-sites/`
- 6 live sites: www.converge.zone / www.axioms.zone / www.helms.zone /
  www.organism.zone / www.reflective.se / apps.reflective.se (portal, embeds
  Wolfgang web at `/wolfgang-chat/**`); www.wolfgang.bot scaffolded but empty.
- Last reviewed: 2026-06-07
- Deep dive: [[beacon-sites/Architecture - Overview]]

### Forge Templates

> Owns: extension workspace skeleton + working CI workflows + release-ritual
> enforcement scripts for new Mosaic extensions (`converge-extension`). Does
> NOT own: live extension code (each copy detaches); engagement scaffolding
> (→ `studio-apps/folio-editor` as the live reference, per
> [[reference-engagement|reference-engagement.md]]).

- Home: `forge-templates/` (one active template: `converge-extension/`;
  `converge-engagement/` archived 2026-06-07 — see
  [[decisions/2026-06-07-retire-engagement-template|retirement ADR]])
- Last reviewed: 2026-06-07
- Deep dive: [[forge-templates/Architecture - Overview]]
