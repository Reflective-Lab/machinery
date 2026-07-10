---
tags: [contracts, commerce]
source: mixed
---
# Commerce Rail Surface

The first Commerce Rails contract surface defines Reflective-owned commercial
objects. These names are intentionally provider-neutral.

The Rust contract crate is `commerce-rails-contracts`.

Rail terms describe the control model around these contracts. They do
not replace the plain domain names below.

## Accounts

`ReflectiveAccount` is the internal commercial account for Reflective-operated
commerce.

`CustomerOrg` is the customer commercial organization buying, installing, or
using apps. It is a Commerce Rails projection of a canonical Runtime Runway organization,
not the tenant identity source.

`BuilderAccount` is a builder identity that can create app listings.

`PartnerAccount` is a partner that can receive revenue share and payouts.
Payment-provider account IDs are stored as external `ProviderObjectRef` values.

## Apps And Entitlements

`AppListing` is the market-facing app offer.

`AppInstallation` is the customer-specific installation of a listed app.

`Plan` and `Price` define what can be purchased.

`Subscription` records a customer commitment owned by Commerce Rails.

`EntitlementGrant` records what the customer can use after a commercial event.
Apps consume entitlements by contract; they do not infer them from Stripe,
email, invoices, or provider webhooks.

## Partner Commerce

`RevenueShareAgreement` records the commercial split between Reflective and a
partner.

`TransferIntent` records the intent to move value to a partner under an
agreement.

`PayoutObligation` records the business obligation to pay or hold funds for a
partner.

## Audit And Reconciliation

`LedgerEntry` records commercial audit entries in deterministic minor units.

`WebhookReceipt` records provider events for signature verification, replay
protection, duplicate handling, and reconciliation.

`CommercialPolicy` records named commercial policy used by the governing layer.

## Commands

`CommercialCommand` wraps every consequential command with command ID,
idempotency key, actor, scope, origin, policy, HITL, audit, and reconciliation
requirements.

`PartnerPiggyBackCommand` defines the first executable loop for listing a
partner app, installing it for a customer, creating a subscription, granting an
entitlement, recording revenue share, and staging a partner payout.

## First Loop

```text
partner app listed
  -> customer installs app
  -> subscription created
  -> entitlement granted
  -> revenue-share obligation recorded
  -> payout staged
  -> payout executed or held
  -> ledger and webhook receipts reconciled
```

## Rule

Commerce Rails IDs are primary. Provider object IDs are references. A provider
may report state, but Commerce Rails decides which commercial state is accepted
under Reflective policy.
