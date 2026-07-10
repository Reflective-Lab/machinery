---
tags: [adapters, stripe, commerce]
source: mixed
---
# Stripe Connect Boundary

Stripe Connect is the first payment adapter for Commerce Rails. It is not the
Commerce Rails domain model.

Stripe crosses the Runtime Runway / Commerce Rails boundary. Runtime Runway owns provider
transport, secret access, deployment config, and runtime observability.
Commerce Rails owns commercial Stripe semantics.

## Decision

Use Stripe Connect to implement provider-side account, customer, subscription,
charge, transfer, payout, refund, dispute, and webhook operations.

Keep Reflective-owned commercial state in Commerce Rails contracts:

- `PartnerAccount`
- `CustomerOrg`
- `AppInstallation`
- `Subscription`
- `EntitlementGrant`
- `RevenueShareAgreement`
- `TransferIntent`
- `PayoutObligation`
- `LedgerEntry`
- `WebhookReceipt`

Stripe IDs are stored only as `ProviderObjectRef` values.

## Adapter Owns

- Stripe API request construction.
- Stripe Connect account references.
- Stripe webhook signature verification.
- Stripe idempotency-key placement.
- Raw provider event capture.
- Mapping Stripe objects to Commerce Rails provider refs.
- Provider-specific error classification.

## Adapter Does Not Own

- Webhook ingress routing, deployment config, runtime telemetry, or secret
  storage authority; those belong in Runtime Runway.
- Partner commercial terms.
- Revenue-share rules.
- Entitlement decisions.
- Payout obligations.
- Refund or dispute policy.
- Ledger semantics.
- HITL placement.
- Arbiter policy decisions.

## Mapping Rules

| Stripe concept | Commerce Rails concept |
|---|---|
| Connected account | `PartnerAccount.provider_refs` |
| Customer | `CustomerOrg.provider_refs` later, if needed |
| Product / price | `AppListing`, `Plan`, `Price` |
| Subscription | `Subscription.provider_refs` |
| Payment intent / charge | future `ChargeIntent.provider_refs` |
| Transfer | `TransferIntent.provider_refs` later, if needed |
| Payout | `PayoutObligation.provider_refs` later, if needed |
| Event | `WebhookReceipt` |

## Webhook Rule

Every Stripe webhook must become a `WebhookReceipt` before it can influence
Commerce Rails state.

The receipt must record:

- provider,
- provider event ID,
- replay key,
- signature verification status,
- received timestamp,
- processing status.

Duplicate events should be accepted as duplicates, not re-run as new business
commands.

Runtime Runway gets the webhook request safely to the Commerce Rails surface. Commerce
Rails decides what the verified provider event means commercially.

## Command Rule

Every Stripe mutation must be driven by a Commerce Rails command with a
Commerce Rails idempotency key. The adapter may place that key into Stripe's
idempotency mechanism, but the key is owned by Commerce Rails.

Provider-originated commands must reference a `WebhookReceipt` and pass replay
protection before they can change `Subscription`, `EntitlementGrant`,
`LedgerEntry`, `TransferIntent`, or `PayoutObligation` state.
