> **Archived 2026-07-02** — active milestone tracking moved to Linear (Reflective team).
> This file is kept for historical context only. Do not add new items here.

# Milestones - Commerce Rails

## M0 - Commercial Authority Home

- [x] Create `~/dev/reflective/commerce-rails/`
      with AGENTS, README, MILESTONES, and KB.
- [x] Define the first Commerce Rails contract surface: partner account,
      customer org, app listing, app installation, subscription, entitlement,
      revenue share, payout obligation, ledger entry, and webhook receipt.
- [x] Decide the first Stripe Connect adapter boundary without making Stripe
      the domain model.
- [x] Adopt movement terminology for the rail control model: mainspring,
      gear train, escapement, balance, caliber, and complication.
- [x] Specify executable command safety: idempotency, webhook verification,
      replay protection, reconciliation, audit events, Arbiter policy checks,
      and HITL gates for high-risk commerce actions.
- [x] Add command/result types for the first partner piggy-back loop.
- [x] Add tests for identifier stability, provider reference mapping, and
      webhook receipt replay keys.

## M1 - Partner Piggy-Back Proof

- [x] Partner app listing can be represented.
- [x] Customer app installation can be represented.
- [x] Subscription can grant an entitlement.
- [x] Revenue-share agreement can produce a payout obligation.
- [x] Stripe Connect adapter can map provider events into Commerce Rails
      receipts without replacing Commerce Rails IDs.

## M2a - Entitlement-Gate Proof
**Epic:** E6

**Driver:** quorum-sense.
**Origin:** panel review `(reflective-root)/REVIEW_quorum-sense_2026-06-15.md` (Round 1 finding by `[CR-ARCH]`, ratified by all three architects in Round 3). The original M2 named *Wolfgang* (now in `studio-apps/`); quorum-sense overtook as the first application proving Commerce Rails in a deployed Runtime Runway environment.

Done when:

- [x] quorum-sense is deployed through Runtime Runway and consumes Runtime Runway-owned user, organization, auth, secrets, telemetry, and runtime configuration.
- [x] quorum-sense reads entitlement state from Commerce Rails via `CommerceRails::is_entitled(firebase_uid, "quorum")` — no app-local Stripe or subscription state.
- [x] Stripe webhook mapped into Commerce Rails receipts and the typed `CommerceWebhookAction` set; `apply_webhook_action` updates `EntitlementStore`.
- [x] App shell shows Runtime Runway-backed identity plus Commerce Rails-backed entitlement gate.
- [x] End-to-end smoke passes: user signs in, Stripe webhook accepted, entitlement granted, gate flips.
- [ ] `EntitlementStore` is persistent (QF-CR-03) — currently in-memory; **hard gate before quorum-sense runs at `--max-instances > 1`**.
- [ ] Stripe IDs are not primary domain IDs (QF-CR-08) — currently `customer_ref` is a primary key in the store; refactor in flight.
- [ ] Apps import a Stripe-neutral CR client (QF-CR-09) — currently `commerce-rails-stripe`; refactor in flight.
- [ ] Public API exposes `Plan` enum, not Stripe `price_ref` (QF-CR-10) — refactor in flight.
- [ ] `entitlement_projection(uid, app_id)` returns the panel-locked `EntitlementProjection` schema (QF-CR-05).

See [`QUALITY_BACKLOG.md`](QUALITY_BACKLOG.md) for the active refactor ledger.

## M2b - Partner Piggy-Back Loop
**Epic:** E6

**Driver:** TBD (future). Not blocking M2a.

Done when Commerce Rails proves the full partner piggy-back loop end-to-end with the safety envelope:

- [ ] Commerce Rails exposes executable command handlers for the partner piggy-back loop with idempotency, replay protection, audit events, Arbiter policy checks, HITL gates, and reconciliation.
- [ ] Partner app listing → customer app installation → subscription → entitlement grant → revenue-share agreement → transfer intent → payout obligation flows end-to-end.
- [ ] Make.com (or equivalent) integration can trigger or observe approved commerce commands through a scoped webhook or API boundary with audit, idempotency, replay protection, and secret handling delegated to Runtime Runway.
- [ ] App shell shows Commerce Rails-backed subscription badge plus partner attribution.
- [ ] Documentation names the driver app and keeps the partner-piggy-back command surface separate from the M2a entitlement-gate surface.

**Historical note:** Wolfgang was the originally-named M2 integration driver but moved to `studio-apps/`. M2b inherits the partner piggy-back scope; quorum-sense is single-app and does not prove it.
