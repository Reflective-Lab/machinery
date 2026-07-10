# Commerce Rails Agent Guide

This is the canonical agent entrypoint for Reflective Commerce Rails.

Commerce Rails is the Reflective-owned business layer for billing,
entitlements, partner commerce, revenue sharing, payouts, refunds, disputes,
ledger-sensitive audit, and payment-provider reconciliation.

## Software Factory

Software-factory policy and quality-gate semantics are owned by
`build-depot/`. This repo emits local evidence through its Just recipes, tests,
CI, findings, and docs; Build-Depot normalizes that evidence into the workspace
factory graph.

- Build-Depot cohort: `A`
- Linear label: `module:commerce-rails`
- Sentry: not applicable for this repo. Commerce Rails does not emit runtime
  incidents directly; Runtime Runway owns runtime telemetry and deployment
  observability for hosted services.

It is not Mosaic, Converge, Organism, Axiom, Helms, or Runtime Runway. It consumes
those layers and owns Reflective Labs commercial authority.

> **Before doing platform-boundary work**, read in this order:
> 1. `(reflective-root)/BOUNDARY_REGISTRY.md` — the four-layer authority matrix and the Marquee App Contract (ten hard rules).
> 2. `kb/Architecture/Operating Authority Boundary.md` — Commerce Rails' authoritative boundary doc. For the consumer-facing surface (what a marquee app actually imports and calls), also read `kb/Contracts/Apps Consuming Commerce Rails.md`.
> 3. The active handoff for the app you're working on (e.g. `(reflective-root)/HANDOFF_quorum-sense_2026-06-15.md`).
>
> Only then read code. If you are tempted to introduce something the Marquee App Contract forbids, stop and either prove it doesn't apply, or propose a contract revision through the three-architect panel. Do not silently route around the contract.

## Start Here

1. Read `README.md`.
2. Read `kb/Home.md`.
3. Read `kb/Architecture/Operating Authority Boundary.md`.
4. Read `kb/Architecture/Runtime Runway Commerce Rails Boundary.md`.
5. Read `kb/Architecture/Executable Command Safety.md`.
6. Read `kb/Contracts/Commerce Rail Surface.md`.
7. Read `kb/Adapters/Stripe Connect Boundary.md`.

## Commands

```bash
cargo check
cargo test
```

## Language

Use mechanical-watch terminology for Commerce Rails control flow:

- `movement` for the full rail operating together.
- `mainspring` for accumulated commercial force.
- `gear train` for ordered commerce sequencing.
- `escapement` for controlled release, idempotency, policy, HITL, replay, and
  state-transition gates.
- `balance` for regulation, invariants, ledger balance, and reconciliation.
- `caliber` for a named precision profile.
- `complication` for optional advanced commerce capabilities.

Do not rename clear business objects just to fit the metaphor. Keep names like
`Subscription`, `PayoutObligation`, `LedgerEntry`, and `WebhookReceipt` when
those are the actual commercial concepts.

## Boundaries

- Put Reflective billing, entitlements, marketplace, partner payouts, revenue
  share, refunds, disputes, and commercial audit here.
- Keep canonical users, auth, sessions, org membership, deployments,
  environments, runtime config, telemetry, and secrets in Runtime Runway.
- Model customer commercial orgs here as Commerce Rails projections of Runtime Runway orgs;
  do not make Commerce Rails the canonical tenant or identity source.
- Keep Stripe, Adyen, Klarna, and other providers behind adapter boundaries.
  Runtime Runway owns provider transport and runtime plumbing; Commerce Rails owns
  commercial provider semantics, receipts, idempotency, and reconciliation.
- Keep Mosaic as the specialist bench: evidence, policy, analytics, memory,
  solvers, and generic provider capabilities.
- Keep Converge as the proposal, promotion, receipt, fact, and replay engine.
- Keep Axiom as the Truth compiler, Organism as formation selection, Helms as
  trust-transfer surface, and Runtime Runway as deployment/secrets/telemetry.
- Keep customer-owned business-system writeback in the customer app,
  engagement, or deployment boundary.

## Rules

- Preserve `unsafe_code = "forbid"`.
- No product secrets, `.env` files, or payment-provider credentials in git.
- Provider object IDs are external references, not Commerce Rails primary IDs.
- Every consequential command needs idempotency, audit, tenant scoping, and
  explicit failure semantics before it becomes executable. Provider-originated
  commands also need webhook receipt verification and replay protection.
- Stripe Connect is the first adapter, not the domain model.
