---
tags: [architecture, authority, runway, commerce]
source: mixed
---
# Runtime Runway Commerce Rails Boundary

Runtime Runway owns platform identity and runtime authority. Commerce Rails owns
Reflective commercial authority.

The boundary is decided by who has authority over the consequence, not by which
system first receives a request or event.

## Ownership

| Area | Owner | Rule |
|---|---|---|
| Users | Runtime Runway | Canonical identity, authentication, sessions, invites, roles, and membership. |
| Organizations | Runtime Runway | Canonical tenant and organization container. |
| Customer commercial org | Commerce Rails | Commercial buyer/account projection of a Runtime Runway organization. |
| DevOps | Runtime Runway | Deployments, secrets, environments, runtime config, telemetry, and operational substrate. |
| Subscriptions | Commerce Rails | Plans, prices, subscription state, billing state, and entitlement grants. |
| Billing | Commerce Rails | Invoices, charges, refunds, revenue share, payout obligations, ledger, and reconciliation. |
| Stripe transport | Runtime Runway | Secret access, webhook ingress plumbing, deployment config, and runtime observability. |
| Stripe commerce adapter | Commerce Rails | Provider mapping, idempotency, webhook receipts, commercial state transitions, and reconciliation semantics. |

## Organization Model

Runtime Runway owns the login and tenancy container:

```text
RunwayOrg
  id
  name
  members
  auth and security configuration
```

Commerce Rails owns the commercial projection:

```text
CustomerOrg
  id
  runway_org_id
  legal or commercial name
  billing status
  provider refs
```

Runtime Runway answers who can act for an organization. Commerce Rails answers what that
organization can buy, owes, receives, or is entitled to use.

## Stripe Split

Stripe crosses the boundary, but the responsibilities are not shared
ambiguously.

```text
Stripe webhook HTTP request
  -> Runtime Runway routes it, provides secret access, and observes runtime health
  -> Commerce Rails Stripe adapter verifies provider semantics and records receipt
  -> Commerce Rails gates apply idempotency, replay, policy, and HITL checks
  -> Commerce Rails updates Subscription, EntitlementGrant, LedgerEntry, or payout state
```

Runtime Runway gets the Stripe event safely to the application. Commerce Rails decides
what the Stripe event means commercially.

## Rule

If the question is who can log in, where code runs, where secrets live, or how
the runtime is operated, it belongs to Runtime Runway.

If the question is who pays, what is owed, what is granted, what is refundable,
what must be reconciled, or what commercial state is accepted, it belongs to
Commerce Rails.
