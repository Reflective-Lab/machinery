# Plan 4 — Reflective Labs Entitlement Store + `is_entitled` API

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans`. Steps use checkbox (`- [ ]`) syntax.

**Goal:** Close the Commerce Rails gap that Quorum (Plan 3b) will consume: a queryable `is_entitled(firebase_uid, app) -> bool` API backed by an in-memory `EntitlementStore` that applies the typed `CommerceWebhookAction` events the existing Stripe adapter already produces. v1: in-memory only (state lost on restart); v2 promotes to StorageKit-backed persistence. No new HTTP server; this stays a library API.

**Architecture:** The existing M1 work shipped `CommerceRails::accept_stripe_webhook` which returns a typed `CommerceWebhookAction` (one of: `LinkCustomerRef`, `ApplySubscriptionProjection`, `UpdateSubscriptionStatus`, `Ignored`). What was missing: somewhere to PUT that action and a way to QUERY it later. Plan 4 adds an `EntitlementStore` field on `CommerceRails` that:

1. Holds a `firebase_uid → customer_ref` mapping (set by `LinkCustomerRef`)
2. Holds a `customer_ref → SubscriptionProjection` mapping (set by `ApplySubscriptionProjection`, updated by `UpdateSubscriptionStatus`)
3. Exposes `is_entitled(firebase_uid, app) -> bool` that does the two-step lookup + checks the projection's status is active AND the plan grants the app

Also: `BillingPlan::apps()` currently returns `"marquee"` as the granted app key — a placeholder. Plan 4 changes that to `"quorum"` since Quorum is the actual first product. (When a second app ships, we extend the enum or the apps list — that's a follow-up, not v1.)

**Tech Stack:** Rust 1.96 / Edition 2024 (commerce-rails MSRV); existing crates (`commerce-rails-contracts`, `commerce-rails-stripe`); `std::collections::HashMap` + `std::sync::Mutex`.

**Spec:** `docs/superpowers/specs/2026-06-06-quorum-shippable-v1-design.md` (in marquee-apps/quorum-sense) — Track B / Plan 4 row. `MILESTONES.md` (in this repo) — M2 Wolfgang Deployed Integration Driver describes the same pattern; Quorum is the second consumer.

**Boundary check:**
- New `EntitlementStore` struct + methods → `commerce-rails-stripe` (alongside `CommerceRails`).
- Changed `BillingPlan::apps()` → returns `"quorum"` instead of `"marquee"`.
- No new crates, no new HTTP routes, no Stripe API changes, no breaking changes to existing `accept_stripe_webhook` semantics.
- Quorum-side consumption is **Plan 3b in the quorum-sense repo**, not this plan.

---

## File map

**Modified files:**
- `crates/commerce-rails-stripe/src/lib.rs` — add `EntitlementStore` struct, `CommerceRails::apply_webhook_action`, `CommerceRails::is_entitled`; update `BillingPlan::apps()` to return `"quorum"`
- `kb/LOG.md` (or `kb/History/CHANGELOG.md` — match existing convention) — Plan 4 wrap-up entry

**New files:**
- `crates/commerce-rails-stripe/tests/entitlement_store.rs` — integration test: simulate webhook actions → query → expect right entitlement decisions

**Not touched:**
- `crates/commerce-rails-contracts/src/lib.rs` (no new contract types — `EntitlementGrant` already exists if needed later)
- Stripe API calls (no new Stripe round-trips)
- The webhook signature verification or parsing logic (already shipped in M1)
- `MILESTONES.md` (the Wolfgang M2 line stays; Quorum-side notes go in the LOG entry)

---

## Task 1: Update `BillingPlan::apps()` to grant `"quorum"`

**Files:**
- Modify: `crates/commerce-rails-stripe/src/lib.rs` — the `BillingPlan::apps()` method around line 241

The current method returns `"marquee"` as a placeholder. v1's actual first product is Quorum. Update the string.

- [ ] **Step 1: Survey existing uses of `"marquee"` as an entitlement key**

```bash
grep -rnE '"marquee"' (reflective-root)/commerce-rails --include="*.rs" 2>/dev/null | head -10
```

If `"marquee"` appears in tests or other modules as an expected entitlement key, those tests need updating too. Count how many places need changes — informs whether Step 2 is a one-liner or wider sweep.

- [ ] **Step 2: Update the method**

In `crates/commerce-rails-stripe/src/lib.rs`, find:

```rust
    pub fn apps(self) -> Vec<String> {
        match self {
            Self::Free => Vec::new(),
            Self::Starter | Self::Team | Self::Enterprise => vec!["marquee".to_string()],
        }
    }
```

Replace with:

```rust
    pub fn apps(self) -> Vec<String> {
        match self {
            Self::Free => Vec::new(),
            // v1: all paid plans grant Quorum (Reflective Labs single-app
            // subscription). When a second app ships, extend the variant
            // discriminant or the apps list per plan.
            Self::Starter | Self::Team | Self::Enterprise => vec!["quorum".to_string()],
        }
    }
```

- [ ] **Step 3: Update any tests that asserted on `"marquee"`**

If Step 1 found uses in test files, update them to expect `"quorum"`. If `"marquee"` appears in non-test code, leave it (it's likely a Stripe metadata key or a comment string).

- [ ] **Step 4: Verify**

```bash
cargo check --workspace 2>&1 | tail -5
cargo test --workspace 2>&1 | grep "test result" | head -10
cargo clippy --workspace -- -D warnings 2>&1 | tail -5
```

Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add crates/commerce-rails-stripe/src/lib.rs
git commit -m "feat(stripe): BillingPlan grants \"quorum\" instead of \"marquee\" placeholder

Quorum is the actual first product in Reflective Labs; \"marquee\"
was a planning-time placeholder. v1 has a single paid product, so
all paid plans grant the same app. When app #2 lands, extend per
plan or per variant."
```

## Report

- Status: DONE / BLOCKED
- Count of `"marquee"` uses found and which were updated
- Test + clippy result
- Commit SHA

---

## Task 2: Add `EntitlementStore` struct + integration into `CommerceRails`

**Files:**
- Modify: `crates/commerce-rails-stripe/src/lib.rs`

The store holds the two mappings needed to answer "is firebase_uid X entitled to app Y." In-memory + thread-safe via `Mutex`.

- [ ] **Step 1: Add the struct + impl**

Near the `CommerceRails` struct (around line 157), add:

```rust
use std::collections::HashMap;
use std::sync::Mutex;

/// In-memory entitlement state. Holds two mappings:
/// - firebase_uid → customer_ref (set by LinkCustomerRef)
/// - customer_ref → SubscriptionProjection (set/updated by ApplySubscriptionProjection
///   and UpdateSubscriptionStatus)
///
/// v1 only; state is lost on restart. A future plan promotes this to
/// StorageKit-backed persistence. Quorum consumes this via
/// `CommerceRails::is_entitled` (not direct store access).
#[derive(Default)]
pub struct EntitlementStore {
    /// firebase_uid → Stripe customer_ref
    customer_refs: Mutex<HashMap<String, String>>,
    /// customer_ref → current subscription projection
    projections: Mutex<HashMap<String, SubscriptionProjection>>,
}

impl EntitlementStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the customer_ref for a firebase_uid, if linked.
    pub fn customer_ref_for(&self, firebase_uid: &str) -> Option<String> {
        self.customer_refs.lock().ok()?.get(firebase_uid).cloned()
    }

    /// Returns the subscription projection for a customer_ref, if any.
    pub fn projection_for(&self, customer_ref: &str) -> Option<SubscriptionProjection> {
        self.projections.lock().ok()?.get(customer_ref).cloned()
    }

    /// Updates the store from a typed webhook action. Returns true if the
    /// action mutated state; false for `Ignored` or no-op cases. Lock
    /// failures (mutex poisoned) are treated as no-op + false.
    pub fn apply(&self, action: &CommerceWebhookAction) -> bool {
        match action {
            CommerceWebhookAction::LinkCustomerRef { firebase_uid, customer_ref } => {
                if let Ok(mut map) = self.customer_refs.lock() {
                    map.insert(firebase_uid.clone(), customer_ref.clone());
                    return true;
                }
                false
            }
            CommerceWebhookAction::ApplySubscriptionProjection { customer_ref, projection } => {
                if let Ok(mut map) = self.projections.lock() {
                    map.insert(customer_ref.clone(), projection.clone());
                    return true;
                }
                false
            }
            CommerceWebhookAction::UpdateSubscriptionStatus { customer_ref, subscription_status } => {
                if let Ok(mut map) = self.projections.lock() {
                    if let Some(p) = map.get_mut(customer_ref) {
                        p.status = subscription_status.clone();
                        return true;
                    }
                }
                false
            }
            CommerceWebhookAction::Ignored => false,
        }
    }
}
```

The exact field name `p.status` assumes `SubscriptionProjection` has a `status: String` field. Verify with `grep -nA 12 "pub struct SubscriptionProjection" crates/commerce-rails-stripe/src/lib.rs | head -16` and adjust the field name if it's different (likely something like `status: String` based on the `UpdateSubscriptionStatus` action's field name).

If `SubscriptionProjection` doesn't `#[derive(Clone)]` already, add it — the store needs to return owned copies. Same for `BillingPlan` if it isn't `Copy + Clone` already (the `apps()` method already shows it's `Copy` since `self` works without ref).

**Step 2: Verify build**

```bash
cargo check -p commerce-rails-stripe 2>&1 | tail -5
```

Expected: clean. Common issues:
- `SubscriptionProjection.status` field name — verify
- Missing `Clone` derive on SubscriptionProjection — add it
- `Mutex` import — `use std::sync::Mutex`

- [ ] **Step 3: Commit**

```bash
git add crates/commerce-rails-stripe/src/lib.rs
git commit -m "feat(stripe): EntitlementStore — in-memory webhook-action sink

Holds firebase_uid → customer_ref and customer_ref → SubscriptionProjection
mappings updated by the three concrete CommerceWebhookAction variants.
v1: in-memory, no persistence. is_entitled query lands in the next task.

Quorum (Plan 3b in quorum-sense repo) will consume the query, not
the raw store."
```

## Report

- Status: DONE / BLOCKED
- `SubscriptionProjection` field name for status (and any derive additions you made)
- `cargo check` result
- Commit SHA

---

## Task 3: Add `apply_webhook_action` and `is_entitled` on `CommerceRails`

**Files:**
- Modify: `crates/commerce-rails-stripe/src/lib.rs`

Wire `EntitlementStore` into `CommerceRails` as a field; expose the apply + query methods on the service itself.

- [ ] **Step 1: Add the store field**

In the `CommerceRails` struct:

```rust
pub struct CommerceRails {
    // ... existing fields ...
    entitlements: EntitlementStore,
}
```

If the struct's fields are all listed in `impl CommerceRails::new(...)`, add `entitlements: EntitlementStore::new()` to the initializer. If it uses `..Default::default()` (rare for a service struct), the new field's Default::default works automatically.

- [ ] **Step 2: Add the public methods**

```rust
impl CommerceRails {
    /// Apply a typed webhook action to the entitlement store. Returns true
    /// if state was mutated. Call this from the webhook HTTP handler after
    /// `accept_stripe_webhook` returns an AcceptedWebhook.
    pub fn apply_webhook_action(&self, action: &CommerceWebhookAction) -> bool {
        self.entitlements.apply(action)
    }

    /// Returns true if the firebase_uid has an active subscription whose
    /// plan grants the named app entitlement.
    ///
    /// Lookup: firebase_uid → customer_ref → SubscriptionProjection.
    /// Active = subscription status is one of {"active", "trialing"}.
    /// Apps come from BillingPlan::apps() — currently every paid plan
    /// grants Quorum.
    pub fn is_entitled(&self, firebase_uid: &str, app: &str) -> bool {
        let Some(customer_ref) = self.entitlements.customer_ref_for(firebase_uid) else {
            return false;
        };
        let Some(projection) = self.entitlements.projection_for(&customer_ref) else {
            return false;
        };
        // Status must be active.
        if !matches!(projection.status.as_str(), "active" | "trialing") {
            return false;
        }
        // Plan must grant the app.
        projection.plan.apps().iter().any(|a| a == app)
    }
}
```

If `projection.status` isn't a `String` (e.g. typed enum), adjust the match. If `projection.plan` doesn't exist (the field is named differently), adapt. Verify with the struct read in Task 2 Step 1.

- [ ] **Step 3: Verify**

```bash
cargo check --workspace 2>&1 | tail -5
cargo clippy --workspace -- -D warnings 2>&1 | tail -5
```

Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add crates/commerce-rails-stripe/src/lib.rs
git commit -m "feat(stripe): CommerceRails::apply_webhook_action + is_entitled

Wires EntitlementStore into the service. apply_webhook_action is the
HTTP webhook handler's persist call; is_entitled is the query for
apps (Quorum, future) to check before granting access.

Active subscription = status in {active, trialing}. App grants come
from BillingPlan::apps() — extended per-plan when app #2 lands."
```

## Report

- Status: DONE
- Whether `projection.status` was `String` or typed (and any adaptation)
- Whether `projection.plan` access path matched expectation
- `cargo check` + clippy result
- Commit SHA

---

## Task 4: Integration test for the full webhook → entitlement flow

**Files:**
- Create: `crates/commerce-rails-stripe/tests/entitlement_store.rs`

End-to-end: construct a `CommerceRails`, simulate the 3 webhook actions in order, query entitlement, assert.

- [ ] **Step 1: Survey existing tests for setup pattern**

```bash
ls (reflective-root)/commerce-rails/crates/commerce-rails-stripe/tests/ 2>/dev/null
find (reflective-root)/commerce-rails -name "*.rs" -path "*/tests/*" 2>/dev/null | head -5
grep -rnE "CommerceRails::new\b|CommerceRailsConfig::" (reflective-root)/commerce-rails --include="*.rs" 2>/dev/null | head -10
```

Find existing test patterns. If there are no tests in `crates/commerce-rails-stripe/tests/`, the unit-test idiom is `#[cfg(test)] mod tests { ... }` inside `lib.rs` — but the plan calls for an integration test, so create one.

- [ ] **Step 2: Write the test**

Create `crates/commerce-rails-stripe/tests/entitlement_store.rs`:

```rust
use commerce_rails_stripe::{
    BillingPlan, CommerceRails, CommerceRailsConfig, CommerceWebhookAction,
    SubscriptionProjection,
};
use reqwest::Client;

fn make_service() -> CommerceRails {
    let config = CommerceRailsConfig::local();
    CommerceRails::new(Client::new(), config)
}

#[test]
fn fresh_service_says_not_entitled() {
    let service = make_service();
    assert!(!service.is_entitled("user-1", "quorum"));
}

#[test]
fn link_customer_alone_does_not_entitle() {
    let service = make_service();
    service.apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
        firebase_uid: "user-1".to_string(),
        customer_ref: "cus_abc".to_string(),
    });
    // No subscription projection yet → not entitled.
    assert!(!service.is_entitled("user-1", "quorum"));
}

#[test]
fn link_plus_active_starter_grants_quorum() {
    let service = make_service();
    service.apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
        firebase_uid: "user-1".to_string(),
        customer_ref: "cus_abc".to_string(),
    });

    let projection = SubscriptionProjection {
        plan: BillingPlan::Starter,
        status: "active".to_string(),
        // ... other fields default. If the struct has required fields
        // without defaults, fill them in.
        ..Default::default()
    };
    service.apply_webhook_action(&CommerceWebhookAction::ApplySubscriptionProjection {
        customer_ref: "cus_abc".to_string(),
        projection,
    });

    assert!(service.is_entitled("user-1", "quorum"));
    // Other apps: not entitled (only Quorum is granted in v1).
    assert!(!service.is_entitled("user-1", "wolfgang"));
}

#[test]
fn canceled_subscription_revokes_entitlement() {
    let service = make_service();
    service.apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
        firebase_uid: "user-1".to_string(),
        customer_ref: "cus_abc".to_string(),
    });
    service.apply_webhook_action(&CommerceWebhookAction::ApplySubscriptionProjection {
        customer_ref: "cus_abc".to_string(),
        projection: SubscriptionProjection {
            plan: BillingPlan::Starter,
            status: "active".to_string(),
            ..Default::default()
        },
    });
    assert!(service.is_entitled("user-1", "quorum"));

    service.apply_webhook_action(&CommerceWebhookAction::UpdateSubscriptionStatus {
        customer_ref: "cus_abc".to_string(),
        subscription_status: "canceled".to_string(),
    });
    assert!(!service.is_entitled("user-1", "quorum"));
}

#[test]
fn free_plan_does_not_grant_quorum() {
    let service = make_service();
    service.apply_webhook_action(&CommerceWebhookAction::LinkCustomerRef {
        firebase_uid: "user-1".to_string(),
        customer_ref: "cus_abc".to_string(),
    });
    service.apply_webhook_action(&CommerceWebhookAction::ApplySubscriptionProjection {
        customer_ref: "cus_abc".to_string(),
        projection: SubscriptionProjection {
            plan: BillingPlan::Free,
            status: "active".to_string(),
            ..Default::default()
        },
    });
    assert!(!service.is_entitled("user-1", "quorum"));
}
```

**Adaptations expected:**
- `SubscriptionProjection { ..Default::default() }` — only works if the struct has `#[derive(Default)]`. If it doesn't, add the derive in Task 2 OR construct the struct with all required fields. Check with `grep -nA 20 "pub struct SubscriptionProjection" crates/commerce-rails-stripe/src/lib.rs`.
- `CommerceRailsConfig::local()` exists at line 61; use it.
- `Client::new()` from reqwest needs reqwest as a dev-dependency on commerce-rails-stripe. Check `crates/commerce-rails-stripe/Cargo.toml` `[dev-dependencies]`; if reqwest isn't there, add it.

- [ ] **Step 3: Run**

```bash
cargo test -p commerce-rails-stripe --test entitlement_store 2>&1 | tail -15
```

Expected: 5 PASS.

- [ ] **Step 4: Lint**

```bash
cargo clippy -p commerce-rails-stripe -- -D warnings 2>&1 | tail -5
```

Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add crates/commerce-rails-stripe/tests/entitlement_store.rs
# If you had to add derives or fix imports in lib.rs/Cargo.toml, add those too:
# git add crates/commerce-rails-stripe/src/lib.rs crates/commerce-rails-stripe/Cargo.toml
git commit -m "test(stripe): integration test for EntitlementStore + is_entitled

5 tests: fresh service denies; LinkCustomerRef alone insufficient;
LinkCustomerRef + active Starter projection grants quorum (but not
other apps); subscription cancellation revokes; Free plan never
grants quorum.

Proves the end-to-end webhook → store → query loop without hitting
Stripe."
```

## Report

- Status: DONE
- Adaptations made (Default derive, Cargo dep additions, field name changes)
- Test count + result (should be 5 PASS)
- Clippy result
- Commit SHA

---

## Task 5: Final check + LOG

- [ ] **Step 1: Workspace check**

```bash
cargo check --workspace --locked 2>&1 | tail -3
cargo test --workspace --tests 2>&1 | grep "test result" | head -10
cargo clippy --workspace -- -D warnings 2>&1 | tail -5
```

Expected: all green.

- [ ] **Step 2: LOG entry**

`head -10 kb/LOG.md` to verify convention. Then append:

```markdown
## 2026-06-08 — Plan 4 (Track B): EntitlementStore + is_entitled API landed on `next`

Closes the Commerce Rails gap that Quorum will consume (Plan 3b, in
the quorum-sense repo). The existing M1 work shipped
`accept_stripe_webhook` which returns a typed `CommerceWebhookAction`,
but nothing actually persisted those actions or answered "is this
user entitled?" Plan 4 adds:

- `EntitlementStore` — in-memory mappings of firebase_uid →
  customer_ref and customer_ref → SubscriptionProjection, updated by
  the 3 concrete `CommerceWebhookAction` variants
  (`LinkCustomerRef`, `ApplySubscriptionProjection`,
  `UpdateSubscriptionStatus`).
- `CommerceRails::apply_webhook_action` — the webhook handler's
  persist call.
- `CommerceRails::is_entitled(firebase_uid, app) -> bool` — active
  subscription + plan grants the app. Active = status in
  {"active", "trialing"}.
- `BillingPlan::apps()` updated to return `"quorum"` instead of
  `"marquee"` placeholder. v1 has a single paid product; all paid
  plans (Starter, Team, Enterprise) grant Quorum. When app #2 ships,
  extend the per-plan list.

5 integration tests cover the full lifecycle: fresh service denies;
LinkCustomerRef alone is insufficient (no subscription); active
Starter + link grants Quorum but not other apps; cancellation
revokes; Free plan never grants Quorum.

In-memory only. v2 promotes the store to StorageKit-backed
persistence so state survives restarts. For v1's friends/family
audience (Karl's target), in-memory + a fresh signup loop is fine —
the store is rehydrated from Stripe's source of truth on restart
by replaying webhooks (when the webhook-replay machinery lands as a
separate task).

Held for follow-ups:
- Persistent storage (StorageKit-backed)
- Per-plan app lists (Wolfgang and other apps)
- Webhook replay from Stripe for cold-start state rehydration
- An HTTP wrapper service (today this is a library; the consumer
  embeds it OR builds its own HTTP wrapper)

Spec: `marquee-apps/quorum-sense/docs/superpowers/specs/2026-06-06-quorum-shippable-v1-design.md`.
Plan: `docs/superpowers/plans/2026-06-08-plan-4-labs-entitlement-store.md`.

Unblocks Plan 3b (in quorum-sense repo): Quorum's
`/api/session/start` now has a concrete `is_entitled` call to gate
on once it imports commerce-rails as a dep.
```

- [ ] **Step 3: Commit (kb/LOG.md ONLY)**

```bash
git add kb/LOG.md
git diff --cached --name-only  # MUST be only kb/LOG.md
git commit -m "docs(log): record Plan 4 (EntitlementStore + is_entitled) landing"
```

- [ ] **Step 4: Confirm**

```bash
git log --oneline main..HEAD | head -10
```

## Report

- Status: DONE
- Workspace check result
- LOG commit SHA
- `git log --oneline main..HEAD` output

---

## Self-review checklist

1. `BillingPlan::apps()` returns `"quorum"` for paid plans.
2. `EntitlementStore` correctly applies all 3 CommerceWebhookAction variants + skips Ignored.
3. `is_entitled` returns true ONLY when both link + active subscription + plan-grants-app are all satisfied.
4. The 5 integration tests pass.
5. No new HTTP routes; this stays a library API.
6. No changes to existing M1 webhook parsing logic.
7. Clippy clean.

---

## What unblocks after this plan

- **Plan 3b** (in quorum-sense): Quorum's `/api/session/start` adds `commerce-rails-stripe` as a dep, wires `is_entitled(firebase_uid, "quorum")` as a gate before opening a session. Without Plan 4, that gate had nowhere to call.
- **Labs frontend** (separate plan): a small landing/signup at `apps.reflective.se/` that hits Stripe Checkout via the existing `CommerceRails::create_checkout_session` API, then trusts the webhook to grant entitlement asynchronously. Today the user must run their own Stripe setup; that's polish.
- **Persistent storage** (separate plan): swap `Mutex<HashMap>` for StorageKit so state survives restarts.

---

## Open follow-ups (named, not solved)

1. **In-memory state is lost on restart.** For v1's friends/family scope, acceptable; once the service is deployed and accepting real subscriptions, the next plan adds persistence and webhook replay.
2. **Plan-to-app mapping is hardcoded.** All paid plans grant Quorum because Quorum is the only app. When app #2 ships, refactor `BillingPlan::apps()` to a per-plan list or move to a registry.
3. **Mutex lock contention.** With one writer (webhooks) and many readers (entitlement checks), an `RwLock` would be slightly better. v1 traffic is well below the level where this matters.
4. **`active` and `trialing` are the only "active" statuses.** Stripe has additional states (`past_due`, `unpaid`, etc.) that may or may not gate access. v1 lets a `past_due` subscription lose entitlement — strict but safe. Loosen if real-world friction shows up.
