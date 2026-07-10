---
tags: [architecture, terminology, commerce]
source: mixed
---
# Rail Terminology

Commerce Rails uses mechanical-watch language for the rail control model. The
terms are allowed when they map to real commerce behavior; they should not
replace clear domain object names.

## Core Terms

| Term | Commerce Rails meaning |
|---|---|
| `movement` | The whole Reflective-owned commerce rail: contracts, commands, policy gates, receipts, ledger, adapters, and reconciliation working together. |
| `mainspring` | Accumulated commercial force waiting to be released, such as authorized value, subscription commitment, receivables, transfer intent, or payout obligation. |
| `gear train` | The deterministic sequence that transmits commercial work from one state to the next. |
| `escapement` | The controlled-release mechanism for consequential work: idempotency, replay protection, policy checks, HITL gates, and explicit state transitions. |
| `balance` | The regulator that keeps the rail stable: reconciliation cadence, invariant checks, ledger balance, timeout handling, and drift detection. |
| `caliber` | A named, versioned precision profile for a rail path: supported capabilities, tolerances, command semantics, adapter requirements, and reconciliation rules. |
| `complication` | An optional advanced commerce behavior layered onto the core movement, such as subscriptions, revenue share, refunds, disputes, taxes, installments, or split payouts. |

## Working Model

```text
mainspring stores commercial force
  -> gear train sequences the work
  -> escapement releases each consequential step
  -> balance regulates timing, invariants, and reconciliation
  -> caliber defines the precision profile for the path
```

## Naming Rule

Use movement terminology for orchestration, safety, reconciliation, and rail
profiles. Keep customer, partner, listing, entitlement, ledger, webhook, and
provider references named as the plain business concepts they represent.

Good examples:

- `Caliber` for a versioned rail profile.
- `Escapement` for the command gate that controls execution.
- `GearTrain` for the ordered commerce step plan.
- `Balance` for reconciliation and invariant regulation.
- `Mainspring` for accumulated commercial force before release.

Avoid extending the metaphor where it hides the business meaning. A refund is
still a refund, a subscription is still a subscription, and a provider event is
still a provider event.
