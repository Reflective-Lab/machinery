---
tags: [contract, commerce, marquee-apps, seam]
source: mixed
verified-against: quorum-sense 2026-07-14
---
# Marquee App Seam — Commerce Rails

**Design status:** Done — see [Apps Consuming Commerce Rails](Apps%20Consuming%20Commerce%20Rails.md) (authoritative consumer contract).

This page is the **bootstrap checklist** for App #2.

**Umbrella:** [Bedrock marquee-app-machinery-seam.md](../../../../framework/bedrock/kb/06-consumption/marquee-app-machinery-seam.md)  
**Workspace:** [`BOUNDARY_REGISTRY.md`](../../../../BOUNDARY_REGISTRY.md)  
**Reference app:** `applications/marquee-apps/quorum-sense/crates/quorum-server/src/main.rs`

---

## What Commerce Rails owns (crisp test)

*"Who pays, what is granted, what must be reconciled."*

Subscriptions, entitlements, payouts, ledger-sensitive audit, webhook receipts,
plan→app mapping (target). **Not** Firebase identity (Runway), domain product
logic, or Stripe adapter reach-around from app code.

---

## Consumer surface

| Phase | Crate | Rule |
|-------|-------|------|
| **Today** | `commerce-rails-stripe` | Grandfathered — Quorum pattern |
| **Target** (QF-CR-09) | `commerce-rails-client` | Stripe-free app import |

App workspace pin:

```toml
commerce-rails-stripe = "0.2.0"

[patch.crates-io]
commerce-rails-stripe = { path = "../../../machinery/commerce-rails/crates/commerce-rails-stripe" }
```

---

## Three API calls (the whole contract)

```rust
// Hot path — per protected request
cr.is_entitled(firebase_uid, "your-app-id")  // bool — JWT lifetime only

// Shell / UI — init + after refresh-on-403
cr.entitlement_projection(firebase_uid, "your-app-id")

// Optional — cache invalidation hook
cr.register_post_apply(|action| { /* refresh */ })
```

No other Commerce APIs from marquee apps. No direct `EntitlementStore` access.

---

## Composition root wiring (pattern)

```rust
let commerce = Arc::new(CommerceRails::new(client, CommerceRailsConfig::from_env(local_dev)?));

// Share with runway-accounts so Stripe webhook updates the same store the gate reads
let accounts = AccountsState::with_commerce(/* ... */, commerce.clone());

router
    .merge(runway_accounts::public_routes(accounts.clone()))    // webhook
    .merge(runway_accounts::protected_routes(accounts.clone()));

// App gate (enum lives in {app}-server — Commerce does not own this type)
match (entitlement_mode, commerce.as_ref()) {
    (Bypass, _) => Ok(()),
    (Enforce, Some(cr)) if cr.is_entitled(uid, APP_ID) => Ok(()),
    _ => Err(entitlement_required),
}
```

**One** `CommerceRails` instance per process. Clone shares inner store.

---

## `runway.app.json` integration

Declare commercial recipes in `deploy_contracts` — not Stripe env vars in app:

```json
"deploy_contracts": [{ "key": "commerce-rails-deploy", "version": "0.2.0" }]
```

Materialization is Runway deploy template + Commerce deploy recipes.

---

## Anti-patterns (Marquee App Contract)

- App-local Stripe webhook handler or subscription types
- Stripe `cus_*` / `sub_*` / `price_*` in domain code
- Inferring entitlement from Firebase claims alone (stale up to 1h)
- Inferring entitlement from `invoice.paid` events
- Hardcoded plan→app mapping in app repo
- Caching `is_entitled` past JWT expiry

Full list: [Apps Consuming Commerce Rails](Apps%20Consuming%20Commerce%20Rails.md).

---

## Today vs target (do not extend violations)

| Surface | Today | Target |
|---------|-------|--------|
| Customer ref | Stripe id internal to CR | `CustomerId` |
| Checkout | Stripe `price_*` | `Plan` enum |
| Store | In-memory per process | `runway-storage` persistent |
| Plan→apps | Hardcoded in CR | Configured data |

Track: `commerce-rails/QUALITY_BACKLOG.md` (QF-CR-08–11).

---

## New app checklist

- [ ] `commerce-rails-stripe` only in `{app}-server` `Cargo.toml`
- [ ] Single `Arc<CommerceRails>` at startup; shared with `runway-accounts`
- [ ] `EntitlementMode` for local dev bypass vs enforce
- [ ] `is_entitled(uid, "your-app-id")` on protected domain routes
- [ ] `deploy_contracts` in `runway.app.json`
- [ ] `boundary-manifest.toml` forbids commerce crates in tier 0–3
- [ ] No provider IDs in `{app}-domain` / `{app}-app`
- [ ] `--max-instances=1` until CR-03/08 persistent coherence ships

---

## Related

- [Apps Consuming Commerce Rails](Apps%20Consuming%20Commerce%20Rails.md)
- [Operating Authority Boundary](../Architecture/Operating%20Authority%20Boundary.md)
- [Runtime Runway Marquee App Seam](../../runtime-runway/kb/Contracts/Marquee%20App%20Seam.md)
- [Stripe Connect Boundary](../Adapters/Stripe%20Connect%20Boundary.md) — adapter authors only
