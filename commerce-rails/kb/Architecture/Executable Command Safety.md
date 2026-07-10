---
tags: [architecture, commands, safety, commerce]
source: mixed
---
# Executable Command Safety

Commerce Rails commands become executable only after they pass through the
escapement. The escapement controls when accumulated commercial force can move
from intent into state change, provider mutation, ledger entry, entitlement, or
payout.

## Required Envelope

Every consequential command must carry:

- a stable Commerce Rails command ID,
- a Commerce Rails idempotency key,
- an actor,
- tenant and commercial scope,
- command origin,
- requested timestamp,
- policy requirements,
- HITL requirement,
- reconciliation requirement,
- audit requirement.

Provider IDs may be attached as references, but they cannot replace the Commerce
Rails command ID or idempotency key.

## Safety Gates

| Gate | Rule |
|---|---|
| Idempotency | Re-running the same command with the same idempotency key returns the same accepted result or an explicit duplicate outcome. |
| Webhook verification | Provider-originated commands must reference a `WebhookReceipt`; unverified or rejected receipts cannot mutate commercial state. |
| Replay protection | Provider events must use `ReplayKey` and duplicate receipts must not re-run business commands. |
| Tenant scope | A command must name the customer org, partner account, app listing, installation, subscription, or payout scope it can affect. |
| Arbiter policy | Policy checks must be recorded before the command can execute. Denied policy blocks execution. |
| HITL | High-risk commands require an approval reference before provider mutation or irreversible state change. |
| Audit | Every accepted, rejected, duplicate, failed, or executed command emits a commercial audit event. |
| Reconciliation | Money-moving commands must declare how ledger and provider state will be reconciled. |

## Failure Semantics

Commands must return an explicit result state:

- `Accepted` means the command passed initial validation but has not necessarily
  completed all effects.
- `Duplicate` means the idempotency key or replay key was seen before.
- `RequiresApproval` means the command is blocked on HITL.
- `Rejected` means validation, policy, scope, or webhook verification blocked it.
- `Executed` means all declared effects completed.
- `Deferred` means execution was intentionally delayed, usually for provider,
  approval, or reconciliation follow-up.
- `Failed` means execution was attempted and did not complete.

Silent partial success is not allowed. If an adapter request succeeds but a
ledger write fails, the result must expose the failure and leave reconciliation
work visible.

## Partner Piggy-Back Loop

The first command loop is:

```text
ListPartnerApp
  -> InstallPartnerApp
  -> CreateSubscription
  -> GrantEntitlement
  -> RecordRevenueShare
  -> StagePartnerPayout
```

This loop is the first `gear train`: each command has a bounded commercial
scope, emits a typed result, and can be audited and reconciled independently.
