# ADR: Applet Runtime Boundaries

- Date: 2026-06-06
- Status: Accepted
- Decision type: architecture boundary
- Related guide: `KB/04-architecture/applet-runtime-boundaries.md`

## Question

Where should Applet responsibility live when applets are expressed as
JTBD/Intent Codec manifests, rendered by TypeScript or Svelte surfaces, and
optionally executed or verified through Rust/WASM?

## Decision

Treat an Applet as a repeatable JTBD contract, not as a small standalone app.
The applet manifest is the shared boundary object.

- Axiom owns the applet manifest type, JSON validation, Truth Package and
  IntentPacket compilation, verifier contracts, WASM artifact contracts,
  lineage, run reports, and calibration.
- Helm owns manifest intake, operator review, truth-catalog binding,
  workbench projection, sandbox lifecycle, approval points, and audit
  visibility.
- Product apps own domain state machines, app-specific adapters, emitted
  observations, and any product mutations.
- Runtime Runway owns app hosting, auth, storage, secrets, telemetry, and
  deployment runtime.
- Commerce Rails owns billing, entitlement, credit balances, payout, and
  reconciliation authority.
- TypeScript and Svelte own user-facing projection and interaction against
  typed contracts, not authority or Truth semantics.
- WASM owns deterministic sandboxed checks and transforms. It must not hold
  secrets, mutate source-of-truth domain state, or promote outcomes.

## Options Considered

1. Put most applet logic in TypeScript app surfaces.
   - Rejected because it would duplicate authority, evidence, and Truth
     semantics across web, desktop, and app repos.
2. Put applet logic directly in Helm.
   - Rejected because Helm would become a second runtime and domain framework.
3. Put the reusable contract in Axiom, operator consumption in Helm, domain
   projection in apps, and deterministic execution in WASM.
   - Chosen because it keeps the factory repeatable while letting each app
     stay focused on its domain job.

## Consequences

- Applet manifests must validate through Axiom before Helm, apps, Arena, or
  Atelier treat them as executable or reviewable.
- Helm and app repos consume the Axiom validator rather than carrying their
  own fixture-only manifest checks.
- Dynamic UI can change quickly in TypeScript/Svelte, but it cannot redefine
  applet authority, evidence contracts, or non-goals.
- WASM applet modules remain portable and deterministic because they execute
  pure checks/transforms under Helm-controlled sandbox policy.
- Commercial actions stay outside applet execution and flow back as facts,
  receipts, or observations.

## Follow-Ups

- Keep the Axiom-owned JSON Schema for `intent-codec-applet.v1` packaged with
  `axiom-truth`.
- Keep TypeScript declarations packaged with the same Axiom-owned contract for
  web and Svelte applet projections.
- Add Helm manifest registry and review surfaces that load manifests directly.
- Extend Arena and Atelier harnesses to consume real applet manifests.
