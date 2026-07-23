---
tags: [index]
source: llm
---
# Entity Index

Curated catalog of major entities across the workspace. One-line description + location.

## Projects

| Entity | Description | Location |
|--------|-------------|----------|
| Converge | Governance engine and public platform crate train | `bedrock-platform/converge/` |
| Organism | Intent, formation, planning, adversarial review, simulation, and learning runtime | `bedrock-platform/organism/` |
| Axiom | Truth/JTBD compiler, verifier, calibration, and CLI | `bedrock-platform/axiom/` |
| Outcome Workbench | Entrepreneur workbench application layer and desktop surface | `bedrock-platform/helms/` |
| Mosaic Extensions | Reusable specialists: Arbiter, Crucible, Embassy, Ferrox, Manifold, Mnemos, Prism, Soter | `mosaic-extensions/` |
| Atelier Showcase | Tutorial spine and live/local scenario gallery | `atelier-showcase/` |
| Arena Tests | Cross-extension integration and contract-shape pressure tests | `arena-tests/` |
| Runtime Runway | Runtime operations, app host, accounts, auth, storage, secrets, telemetry, LLM/GPU paths | `runtime-runway/` |
| Commerce Rails | Commercial authority contracts and Stripe adapter boundary | `commerce-rails/` |
| Wolfgang | Research companion with desktop/web apps and backend | `studio-apps/wolfgang-chat/` |
| Marquee Apps | End-user application proofs and app workspaces | `marquee-apps/` |

## Key Crates (Converge public API)

| Entity | Description | Location |
|--------|-------------|----------|
| converge-model | Governed semantic types | `bedrock-platform/converge/crates/model/` |
| converge-pack | Pack authoring contract | `bedrock-platform/converge/crates/pack/` |
| converge-protocol | Generated protobuf and gRPC contract | `bedrock-platform/converge/crates/protocol/` |
| converge-kernel | In-process engine API | `bedrock-platform/converge/crates/kernel/` |
| converge-client | Remote runtime client SDK | `bedrock-platform/converge/crates/client/` |
| converge-core | Core convergence runtime and governance model | `bedrock-platform/converge/crates/core/` |
| converge-provider | Capability/backend selection contract | `bedrock-platform/converge/crates/provider/` |
| converge-experience | Experience store observer bridge and test support | `bedrock-platform/converge/crates/experience/` |
| converge-optimization | Optimization algorithms and examples | `bedrock-platform/converge/crates/optimization/` |
| converge-runtime | Compatibility-only HTTP/gRPC service shell; retired as canonical stack runtime | `bedrock-platform/converge/crates/runtime/` |
| converge-storage | Object storage contracts and Parquet/Polars bridge | `bedrock-platform/converge/crates/storage/` |

## Key Crates (Organism)

| Entity | Description | Location |
|--------|-------------|----------|
| organism-intent | Intent packets, admission control, decomposition | `organism/crates/intent/` |
| organism-planning | Huddle (multi-model collaborative planning), debate loop | `organism/crates/planning/` |
| organism-adversarial | Assumption breakers, constraint checkers, skeptics | `organism/crates/adversarial/` |
| organism-simulation | 5-dimension simulation swarm | `organism/crates/simulation/` |
| organism-intelligence | OCR, vision, web, social, patent, billing | `organism/crates/intelligence/` |
| organism-notes | Vault management, source adapters, enrichment | `organism/crates/notes/` |
| organism-learning | Recall and learning primitives | `bedrock-platform/organism/crates/learning/` |
| organism-runtime | Runtime orchestration | `bedrock-platform/organism/crates/runtime/` |
| organism-pack | Pack-level planning contract | `bedrock-platform/organism/crates/pack/` |
| organism-catalog | Formation/catalog model | `bedrock-platform/organism/crates/catalog/` |
| organism-dynamics | Dynamics and formation behavior | `bedrock-platform/organism/crates/dynamics/` |

## Services

| Entity | Description | Location |
|--------|-------------|----------|
| api.wolfgang.bot | Wolfgang backend API (axum/optional gRPC) | `studio-apps/wolfgang-chat/backend/` |
| Wolfgang web | SvelteKit web app, currently embedded into apps.reflective.se | `studio-apps/wolfgang-chat/apps/web/` |
| Wolfgang Desktop | Tauri desktop app | `studio-apps/wolfgang-chat/apps/desktop/` |
| apps.reflective.se | Marquee app portal and Wolfgang web host path | `beacon-sites/apps.reflective.se/` |

## Infrastructure

| Entity | Description | Location |
|--------|-------------|----------|
| Firebase `converge-369ad` | Public sites and apps.reflective.se portal | `beacon-sites/*/.firebaserc` |
| Firebase/GCP `wolfgang-kb-prod` | Wolfgang product infrastructure | `studio-apps/wolfgang-chat/deploy/`, `infra/` |
| GitHub Actions | Site and product deploy workflows | project-local `.github/workflows/` |

## People & Orgs

| Entity | Description | Context |
|--------|-------------|---------|
| Reflective Labs | The company | reflective.se |
| Epidemic Sound | Target B2B customer for Epic demo | epic-brand |

## Domain Concepts

| Entity | Description | Where defined |
|--------|-------------|--------------|
| Truth | A governed assertion — Observation → DraftProposal → ValidatedProposal → Fact | converge ADR-002 |
| Pack | Bundle of agents: Suggestor + Validator + Invariant + CriterionEvaluator | converge ADR-003 |
| Huddle | Multi-model collaborative planning session | organism-planning |
| Axiom | Non-negotiable convergence rule (9 total, owned by Converge) | converge-kernel |
| Cedar Policy | Authorization policy language — scopes agent authority | converge-policy |
| Nudge | Accountability prompt for writers | moosemen-writer |
