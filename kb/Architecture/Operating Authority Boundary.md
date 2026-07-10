---
tags: [architecture, authority, commerce]
source: mixed
---
# Operating Authority Boundary

> **Workspace anchor:** [`/Users/kpernyer/dev/reflective/BOUNDARY_REGISTRY.md`](../../../BOUNDARY_REGISTRY.md)
> **Sibling boundary docs (read these for non-CR concerns):**
> - Runtime-Runway: [`/Users/kpernyer/dev/reflective/runtime-runway/kb/Architecture/App Execution Container.md`](../../../runtime-runway/kb/Architecture/App%20Execution%20Container.md)
> - Helms: pending — see `BOUNDARY_REGISTRY.md` for the live link
>
> **Consumer-facing contract for marquee apps:** [`../Contracts/Apps Consuming Commerce Rails.md`](../Contracts/Apps%20Consuming%20Commerce%20Rails.md)
> **Frozen panel review:** [`/Users/kpernyer/dev/reflective/REVIEW_quorum-sense_2026-06-15.md`](../../../REVIEW_quorum-sense_2026-06-15.md)
> **Active implementor handoff:** [`/Users/kpernyer/dev/reflective/HANDOFF_quorum-sense_2026-06-15.md`](../../../HANDOFF_quorum-sense_2026-06-15.md) — ACTIVE since 2026-06-15.

Commerce Rails owns Reflective Labs commercial authority.

It is a business layer above Bedrock and Mosaic. It consumes the stack, but it
does not push Reflective business semantics downward into reusable platform
machinery.

## Owns

- Reflective billing and subscriptions.
- Customer org commercial state as a projection of a canonical Runtime Runway
  organization.
- Partner accounts and builder accounts.
- App listings and app installations.
- Entitlements across Reflective-hosted apps.
- Revenue-share agreements and partner payouts.
- Refunds, disputes, ledger entries, webhook receipts, and reconciliation.

## Does Not Own

- Canonical users, authentication, sessions, invites, roles, org membership, or
  tenant identity; those belong in Runtime Runway.
- Deployment, environment, runtime config, telemetry, and secret-storage
  authority; those belong in Runtime Runway.
- Source-specific evidence ports; those belong in Embassy.
- Generic provider, fetch, search, storage, vector, LLM, and tool capabilities;
  those belong in Manifold or Converge provider/tool contracts.
- Product-domain workflows such as escrow, lending, sourcing, or SMB ops.
- Customer-owned writeback to CRM, accounting, HR, support, signing, commerce,
  or identity systems.
- Deployment topology, secret-storage implementation, or cloud resources.

## Stack Use

| Layer | Commerce Rails use |
|---|---|
| Axiom | Commerce Truths, invariants, and compile-time checks |
| Organism | Formation selection for commercial intents |
| Converge | Proposals, promoted facts, receipts, audit, replay |
| Helms | HITL approvals, review, redirect, and operator visibility |
| Mosaic / Arbiter | Cedar policy, delegation, approval requirements |
| Mosaic / Embassy | Source-specific evidence where needed |
| Mosaic / Manifold | Generic provider and storage capabilities |
| Runtime Runway | Deployment, secrets, auth, storage, telemetry |

## Runtime Runway Rule

Runtime Runway answers who can act, where code runs, how secrets are accessed, and how
the runtime is operated.

Commerce Rails answers who pays, what is owed, what is granted, what is
refundable, what must be reconciled, and which commercial state is accepted.

## Rule

If Reflective bears the commercial consequence, Commerce Rails owns the
contract. Providers implement parts of the flow, but they do not define the
business model.

## Structural debts in flight (2026-06-15 review)

The panel review surfaced three places where the current CR code **does not yet honour the rules above**. These are tracked in [`../../QUALITY_BACKLOG.md`](../../QUALITY_BACKLOG.md) and listed here so the next AI session does not pattern-match against an aspirational contract.

| Debt | Today | After refactor | QF-ID |
|---|---|---|---|
| Stripe `customer_ref` is a primary key in `EntitlementStore` | `Mutex<HashMap<String, SubscriptionProjection>>` keyed by Stripe ID (`commerce-rails-stripe/src/lib.rs:267-269`) | CR-internal `CustomerId`; `ProviderObjectRef` carries Stripe ref | QF-CR-08 |
| Apps import a Stripe-named crate | `commerce-rails-stripe` is both contract AND adapter | Apps import `commerce-rails-client` (Stripe-free); `commerce-rails-stripe` becomes behind-the-trait adapter | QF-CR-09 |
| Public API takes Stripe price IDs | `create_checkout_session(customer_ref, price_ref, ...)` and `STRIPE_PRICE_*` env vars in `CommerceRailsConfig::from_env` | `Plan` enum on public surface; CR-internal `Plan → provider price_ref` map | QF-CR-10 |
| `Plan → Vec<AppId>` hardcoded | `BillingPlan::apps()` returns `vec!["quorum"]` for every paid plan (`lib.rs:348-356`) | Configured mapping; blocks app #2 today | QF-CR-11 |

**Implication for the rule "Providers implement parts of the flow, but they do not define the business model":** today, the public API surface and configuration leak Stripe vocabulary. The rule is the target; the refactors are the path. Do not extend the existing violations.

## Sibling-authority cross-references

For concerns that aren't CR's:

- **Identity, auth, secrets, telemetry, runtime, deploy, storage, app shell, session ownership** → see Runtime-Runway's `kb/Architecture/App Execution Container.md`.
- **Trust-transfer surfaces, operator workbench, HITL approvals, truth catalog binding, governed-job ledger shape** → see Helms's boundary doc (linked from `BOUNDARY_REGISTRY.md`).
- **Domain semantics, product flows, app-specific subject refs, process receipts** → see the relevant marquee-app's own `kb/`.

If a concern doesn't fit any of the four layers, that's a sign you've found a gap, not a sign you should put it here. Propose a contract revision through the panel.
