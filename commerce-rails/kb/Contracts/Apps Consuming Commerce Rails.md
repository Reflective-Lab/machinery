---
tags: [contract, commerce, marquee-apps, consumer]
source: REVIEW_quorum-sense_2026-06-15
---
# Apps Consuming Commerce Rails

**Audience:** You are reading this if you are building a marquee app that needs subscription, entitlement, or commercial gating. This document is the **consumer contract** — the only Commerce-Rails surface you should depend on.

> **For implementors:** the action surface lives in the active handoff at [`(reflective-root)/HANDOFF_quorum-sense_2026-06-15.md`](../../../HANDOFF_quorum-sense_2026-06-15.md) (ACTIVE; signed by all three architects 2026-06-15). This contract defines *what* to consume; the handoff tells you *when* each piece ships and *what you do* at each call-site. Read this contract to understand the shape; read the handoff to plan your work.

**Workspace anchor:** [`(reflective-root)/BOUNDARY_REGISTRY.md`](../../../BOUNDARY_REGISTRY.md)
**Sibling boundary docs (cross-link):**
- Runtime-Runway: [`(reflective-root)/runtime-runway/kb/Architecture/App Execution Container.md`](../../../runtime-runway/kb/Architecture/App%20Execution%20Container.md)
- Helms: pending — see `BOUNDARY_REGISTRY.md`
**Frozen panel review:** [`(reflective-root)/REVIEW_quorum-sense_2026-06-15.md`](../../../REVIEW_quorum-sense_2026-06-15.md)
**Active implementor handoff:** [`(reflective-root)/HANDOFF_quorum-sense_2026-06-15.md`](../../../HANDOFF_quorum-sense_2026-06-15.md)

---

## The contract

You consume Commerce-Rails via **two function calls** and **one callback registration**. Nothing else.

### Hot path — `is_entitled`

```rust
let allowed: bool = commerce.is_entitled(firebase_uid, "your-app-id");
```

- Called per request on protected routes.
- Returns `bool`. Result is valid only for the lifetime of the JWT that produced `firebase_uid`.
- **Never cache past JWT validity.** Push refresh (post-apply callback, see below) + RR's `refresh-on-403` retry is the freshness contract. Caching past JWT expiry creates the *"user paid, app says deny"* failure mode.

### One-shot — `entitlement_projection`

```rust
let projection: EntitlementProjection = commerce.entitlement_projection(firebase_uid, "your-app-id");
```

- Called at app-shell init and on entitlement transitions (after a `refresh-on-403`).
- Returns the **panel-locked** schema below.
- Use this for rendering upgrade buttons, billing-portal links, plan labels — never call `is_entitled` repeatedly to populate UI state.

#### `EntitlementProjection` — panel-locked schema

```rust
pub struct EntitlementProjection {
    pub entitled: bool,
    pub checkout_url: Option<String>,
    pub portal_url: Option<String>,
    pub signup_url: Option<String>,
    pub next_renewal: Option<Timestamp>,
    pub plan_label: Option<String>,
}
```

Per the 2026-06-15 panel review (RR B2 amendment):
- **Adding new optional fields is non-breaking** and does not require panel re-review.
- **Renaming or removing a field requires a new dated panel review.** Do not assume field churn.

### Callback — `register_post_apply`

```rust
let _handle = commerce.register_post_apply(Arc::new(|action: &CommerceWebhookAction| {
    // refresh local cache / fire client SSE / etc.
}));
```

- Registered once at app startup. Called synchronously inside `apply_webhook_action` after the store mutation succeeds.
- Mechanism is in-process (no event bus). Cross-instance coherence handled by persistent store + RR's `refresh-on-403`.
- Apps that aren't `runway-accounts` usually do not need this; `runway-accounts` already wires Firebase claim refresh.

---

## The Marquee App Contract — apps' obligations

The eight hard rules from `BOUNDARY_REGISTRY.md`, restated with CR-specific elaboration:

1. **No app-local platform clones.** Specifically for CR: do not implement your own Stripe webhook handler, your own subscription state, your own entitlement cache. Use the platform webhook mount and the two API calls above.
2. **No commercial state outside commerce-rails.** No app-side `Subscription`, `Plan`, `Entitlement`, `Invoice`, or `Payment` types. Treat CR as the source of truth.
3. **No provider IDs in app or domain code.** No Stripe `cus_*`, `sub_*`, `price_*`, or invoice IDs surface to your app. If you find yourself touching a provider ID, you've reached around the contract.
4. **No test-only HTTP handlers in production.** RR's manifest verifier (D1) will fail your build.
5. **No Helm shells presented as live.** Unrelated to CR but flagged for completeness.
6. **No multi-writer scale until RR D5 + CR-03/CR-08 ship.** Pin `--max-instances=1`. CR's in-memory `EntitlementStore` v1 is per-process; scaling out before v2 means three independent caches re-hydrating from webhook traffic with no coherence.
7. **No feature flag to soften strict checks.** No `--lenient-stripe-ids`, no `STRIPE_LEGACY_KEYS_OK`. Fix the cause, not the check.
8. **No caching of `is_entitled` past JWT validity.** Documented above.

---

## CR API surface — today vs. target

This is honest disclosure of structural debt. Today's API does not yet match the contract above; three CR-side refactors are in flight to close the gap.

| Surface | Today (v1) | Target (post-QF-CR-08/09/10) |
|---|---|---|
| Crate to import | `commerce-rails-stripe` | `commerce-rails-client` (Stripe-free) |
| Customer ref type | Stripe `cus_*` string is a primary key in `EntitlementStore` | CR-internal `CustomerId`; `ProviderObjectRef` carries Stripe ref |
| Checkout API | `create_checkout_session(customer_ref, price_ref, ...)` takes Stripe `price_*` ID | `create_checkout_session(customer_ref, Plan, ...)` |
| Config env vars | `STRIPE_PRICE_TEAM_MONTHLY`, `STRIPE_PRICE_STARTER_MONTHLY`, etc. read by `CommerceRailsConfig::from_env` | CR deploy recipe; app declares `deploy_contracts: [{key, version}]` in `runway.app.json` |
| Plan → apps mapping | Hardcoded `vec!["quorum"]` in `BillingPlan::apps()` | Configured data; blocks app #2 today |
| Entitlement response | `is_entitled -> bool` only | `is_entitled -> bool` + `entitlement_projection -> EntitlementProjection` |
| Webhook coherence | In-memory `Mutex<HashMap>`; lost on restart | Persistent `runway-storage::DocumentStore` |

**If you are writing a new app today**, you import `commerce-rails-stripe` (grandfathered). After QF-CR-09 lands, you migrate to `commerce-rails-client` — single line in `Cargo.toml`.

**If you are reviewing a PR**, the `Today` column is the violation surface, not the contract. Don't extend the existing violations; the refactors are tracked in `QUALITY_BACKLOG.md`.

---

## Anti-patterns — CR will not support these

- **Reading the `EntitlementStore` directly.** The store is an implementation detail. The contract is the function. Reaching around `is_entitled` bypasses every invariant CR enforces.
- **Inferring entitlement from Firebase custom claims.** The `apps: [...]` claim is a hint for UI fast-paths. It is up to 1 hour stale. The server-side authoritative check is `is_entitled`. Treating the claim as authoritative recreates the failure mode push-refresh exists to prevent.
- **Inferring entitlement from invoice or webhook events.** *"They got an `invoice.paid` — they're entitled"* is a category error. CR's webhook handler maps events to state; the state is what matters, not the event.
- **Treating `customer_ref` (Stripe `cus_*`) as a domain ID.** Even though the current implementation does this internally (QF-CR-08 fixes), no consumer code should ever know the Stripe customer ID.
- **Hardcoding plan→app mappings in app code.** The current `BillingPlan::apps()` does this in CR itself (QF-CR-11 fixes). Apps must never re-implement that mapping locally.
- **Asking for write-side commerce APIs.** CR exposes read-side gates (`is_entitled`, `entitlement_projection`) and the webhook ingestion path (`apply_webhook_action`, called by the platform handler). Apps do not mutate commercial state. If you need *"grant this user an entitlement,"* that is a CR command handler (M2b territory), not an app concern.

---

## How to consume — the seam quorum-sense uses

This is the **reference shape**. Copy it for a new marquee app.

```rust
// Single CommerceRails shared across the platform webhook handler and the app's gate.
let commerce = CommerceRails::new(client, CommerceRailsConfig::from_env(local_dev)?);

// Share with runway-accounts so the canonical Stripe webhook updates the same store the gate reads.
let accounts_state = AccountsState::with_commerce(/* ... */, commerce.clone());

// Mount runway-accounts' protected routes (including the public Stripe webhook).
let router = router
    .merge(runway_accounts::public_routes(accounts_state.clone()))   // webhook
    .merge(runway_accounts::protected_routes(accounts_state.clone()).layer(bare_auth_layer));  // /v1/accounts/me etc.

// Gate your domain routes via the EntitlementMode policy enum (app-side; CR doesn't care where you put it).
fn require_app_entitlement(ctx: &EntitlementContext, uid: &str) -> Result<(), AppError> {
    match (ctx.mode, ctx.commerce_rails.as_ref()) {
        (EntitlementMode::Bypass, _) => Ok(()),
        (EntitlementMode::Enforce, Some(cr)) => {
            if cr.is_entitled(uid, "your-app-id") { Ok(()) } else { Err(AppError::EntitlementRequired) }
        }
        _ => Err(AppError::CommerceUnavailable),
    }
}
```

Reference implementation: [`marquee-apps/quorum-sense/crates/quorum-server/src/main.rs:1469-1480`](../../../marquee-apps/quorum-sense/crates/quorum-server/src/main.rs) (entitlement gate) and `:1821-1879` (shared `CommerceRails` wiring) — verified by the panel as the canonical seam.

---

## Cross-references

- [`Commerce Rail Surface.md`](Commerce%20Rail%20Surface.md) — full domain vocabulary (commands, entities, receipts).
- [`Stripe Connect Boundary.md`](../Adapters/Stripe%20Connect%20Boundary.md) — provider-adapter rules; not consumer-facing.
- [`Operating Authority Boundary.md`](../Architecture/Operating%20Authority%20Boundary.md) — what CR owns and does not own.
- [`(reflective-root)/commerce-rails/QUALITY_BACKLOG.md`](../../QUALITY_BACKLOG.md) — open structural-debt items (CR-08/09/10/11) that this contract assumes will land.

— Maintained by `[CR-ARCH]`. Changes require panel review when they alter the contract surface; non-breaking additions can land via PR with `[CR-ARCH]` sign-off.
