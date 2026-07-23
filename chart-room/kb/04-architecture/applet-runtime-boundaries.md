# Applet Runtime Boundaries

Applets are repeatable JTBD contracts, not small versions of big apps. The
Intent Codec manifest is the boundary: it names the functional, emotional, and
relational job; binds the job to evidence, authority, and non-goals; and gives
Helm, app repos, Arena, and Atelier the same object to validate before any
runtime projection.

Decision record:
`KB/04-architecture/decisions/2026-06-06-applet-runtime-boundaries.md`.

## Boundary Model

The stable frame stays in Rust, Axiom, and Helm:

- Axiom owns the typed applet manifest schema, manifest validation, Truth
  Package compilation, IntentPacket compilation, verifier specs, WASM artifact
  contracts, run reports, lineage, and calibration.
- Helm owns manifest intake, operator review, truth-catalog binding, workbench
  projection, sandbox lifecycle, approval points, and audit visibility.
- Product apps own their domain state machines, app-specific adapters, and
  emitted observations. Quorum, for example, can map a validated applet
  manifest into an inquiry contract without owning the manifest schema.
- Runtime Runway owns auth, app hosting, storage, secrets, telemetry, and
  deployment runtime.
- Commerce Rails owns billing, entitlement, marketplace, payout, and
  reconciliation authority.

The dynamic zone is intentionally narrower:

- TypeScript and Svelte own user-facing projections, workflow panels, local UI
  state, optimistic interaction, and calls into Helm, Runway, Commerce, or app
  APIs. They should not redefine applet authority, evidence requirements, or
  Truth semantics.
- WASM owns portable, sandboxed, deterministic applet logic such as predicate
  checks, transforms, verifiers, and small executable policies compiled from
  Rust or generated from Truth artifacts. WASM should not hold secrets, mutate
  source-of-truth domain state, or promote outcomes.
- App adapters own translation between product events and Axiom observations.
  They are dynamic at the edge, but their output must validate against Axiom's
  shared contracts.

## Decision Rules

Use these rules when deciding where applet behavior belongs:

- If it defines the job, authority, evidence, or non-goals, put it in the
  applet manifest and validate it through Axiom.
- If it compiles Gherkin, Truth, JTBD, or governance into a package or
  IntentPacket, keep it in Axiom.
- If it presents, reviews, approves, runs, or audits an applet for operators,
  keep it in Helm.
- If it mutates product state or emits product events, keep it in the owning
  app repo.
- If it renders a workflow, displays status, or collects operator input, use
  TypeScript or Svelte against typed contracts.
- If it is a pure, deterministic executable check or transform, make it a WASM
  module hosted by Helm and described by the applet manifest.
- If it changes billing, entitlement, credit balances, or reconciliation, keep
  it in Commerce Rails and expose only the commercial fact or observation back
  to the applet flow.

## Manifest Consumption Path

The first high-leverage path is direct manifest consumption rather than
fixture-only validation:

1. Axiom exposes the shared JSON validator and typed Rust shape for
   `intent-codec-applet.v1`.
2. Root applet manifests under `KB/02-product/applets/` validate through Axiom.
3. Helm truth-catalog tests load those manifests directly, bind them to
   `TruthSeed` entries, and compile the bound truth into IntentPackets.
4. Quorum consumes the same Axiom validator in its domain tests before mapping
   a fixture into its own inquiry contract.
5. Atelier examples and Arena tests can now treat applet manifests as portable
   contracts instead of bespoke fixtures.

This makes the applet contract reusable across web, desktop, Tauri/Svelte,
Rust services, and future WASM applet execution without forcing every app to
become its own framework.

## Next Interfaces

The next reusable interfaces should be generated from the Axiom-owned manifest
shape:

- Axiom-owned JSON Schema for editor validation and fixture linting:
  `bedrock-platform/axiom/schema/intent-codec-applet.v1.schema.json`.
- Axiom-owned TypeScript declarations for web and Svelte surfaces:
  `bedrock-platform/axiom/schema/intent-codec-applet.v1.d.ts`.
- WASM guest ABI for deterministic applet checks and transforms.
- Helm manifest registry and review API.
- Arena and Atelier harnesses that load real manifests and assert the same
  validation, binding, projection, and observation contracts.
