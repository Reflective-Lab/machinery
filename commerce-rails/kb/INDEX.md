---
tags: [index]
source: mixed
---
# Entity Catalog

## Architecture

- [Operating Authority Boundary](Architecture/Operating%20Authority%20Boundary.md) - Commerce Rails ownership and stack placement
- [Runtime Runway Commerce Rails Boundary](Architecture/Runtime%20Runway%20Commerce%20Rails%20Boundary.md) - users, orgs, DevOps, subscriptions, billing, and Stripe authority split
- [Executable Command Safety](Architecture/Executable%20Command%20Safety.md) - idempotency, replay, webhook verification, policy, HITL, audit, and reconciliation gates
- [Rail Terminology](Architecture/Rail%20Terminology.md) - movement, mainspring, gear train, escapement, balance, caliber, and complication language

## Contracts

- [Commerce Rail Surface](Contracts/Commerce%20Rail%20Surface.md) - account, listing, installation, subscription, entitlement, revenue share, payout, ledger, and webhook contracts

## Adapters

- [Stripe Connect Boundary](Adapters/Stripe%20Connect%20Boundary.md) - provider adapter decision and mapping rules

## Crates

- `commerce-rails-contracts` - Rust contract vocabulary for the first rail surface
- `commerce-rails-stripe` - Stripe provider adapter for config, API requests, webhook signature mechanics, receipts, and event mapping
