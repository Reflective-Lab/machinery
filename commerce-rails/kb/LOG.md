---
tags: [log]
source: mixed
---
# KB Mutation Log

| Date | File | Change | Author |
|---|---|---|---|
| 2026-05-30 | README.md, AGENTS.md, kb/Architecture/Runtime Runway Commerce Rails Boundary.md, kb/Home.md, kb/INDEX.md | Renamed the sibling platform authority from Runway to Runtime Runway after the workspace moved to `runtime-runway/` | codex |
| 2026-05-29 | README.md, AGENTS.md, MILESTONES.md, kb/Architecture/*, kb/Home.md, kb/INDEX.md | Moved the repo to `~/dev/reflective/commerce-rails/` and removed the extra layer indirection from current docs | codex |
| 2026-05-28 | crates/commerce-rails-stripe/src/lib.rs, README.md, INDEX.md | Moved Stripe provider config, API calls, webhook signature mechanics, receipts, and event mapping out of Runtime Runway and into Commerce Rails | codex |
| 2026-05-17 | MILESTONES.md | Prepared M2 for Wolfgang as the deployed Runtime Runway application driver using Commerce Rails with Stripe and Make.com integrations | mixed |
| 2026-05-17 | crates/commerce-rails-contracts/src/lib.rs, MILESTONES.md | Completed M1 contract proof for installation, subscription entitlement, revenue-share payout obligation, and Stripe event receipt mapping | mixed |
| 2026-05-17 | crates/commerce-rails-contracts/src/lib.rs, MILESTONES.md | Started M1 with a contract test proving partner app listing, plan, price, and revenue-share representation | mixed |
| 2026-05-17 | crates/commerce-rails-contracts/src/lib.rs, MILESTONES.md | Added contract tests for CommerceId stability, provider refs, webhook replay keys, provider webhook command gates, command effects, and payout reconciliation | mixed |
| 2026-05-17 | Architecture/Executable Command Safety.md, README.md, AGENTS.md, Contracts/Commerce Rail Surface.md, Adapters/Stripe Connect Boundary.md, Home.md, INDEX.md, MILESTONES.md | Specified executable command safety and first partner piggy-back command loop | mixed |
| 2026-05-17 | Architecture/Runtime Runway Commerce Rails Boundary.md, README.md, AGENTS.md, Architecture/Operating Authority Boundary.md, Adapters/Stripe Connect Boundary.md, Contracts/Commerce Rail Surface.md, Home.md, INDEX.md | Documented Runtime Runway identity/runtime authority vs Commerce Rails commercial authority | mixed |
| 2026-05-17 | Architecture/Rail Terminology.md, README.md, AGENTS.md, Home.md, INDEX.md, Contracts/Commerce Rail Surface.md, MILESTONES.md | Added mechanical-watch terminology for the Commerce Rails control model | mixed |
| 2026-05-17 | Architecture/Operating Authority Boundary.md | Created Commerce Rails stack-placement boundary | mixed |
| 2026-05-17 | Contracts/Commerce Rail Surface.md | Created first contract surface | mixed |
| 2026-05-17 | Adapters/Stripe Connect Boundary.md | Decided Stripe Connect adapter boundary | mixed |
| 2026-05-17 | Home.md, INDEX.md | Created KB entrypoint and catalog | mixed |

## 2026-06-08 — Plan 4 (Track B): EntitlementStore + is_entitled API landed on `next`

Closes the Commerce Rails gap that Quorum will consume (Plan 3b, in
the quorum-sense repo). The existing M1 work shipped
`accept_stripe_webhook` which returns a typed `CommerceWebhookAction`,
but nothing actually persisted those actions or answered "is this
user entitled?" Plan 4 adds:

- `EntitlementStore` — in-memory mappings of `firebase_uid` →
  `customer_ref` and `customer_ref` → `SubscriptionProjection`,
  updated by the 3 concrete `CommerceWebhookAction` variants
  (`LinkCustomerRef`, `ApplySubscriptionProjection`,
  `UpdateSubscriptionStatus`).
- `CommerceRails::apply_webhook_action(&action) -> bool` — the
  webhook handler's persist call.
- `CommerceRails::is_entitled(firebase_uid, app) -> bool` — active
  subscription + plan grants the app. Active = `subscription_status`
  in {`"active"`, `"trialing"`}.
- `BillingPlan::apps()` updated to return `"quorum"` instead of the
  `"marquee"` placeholder. v1 has a single paid product; all paid
  plans (Starter, Team, Enterprise) grant Quorum. When app #2 ships,
  extend the per-plan list.

The store is `Arc<EntitlementStore>` on `CommerceRails` (the service
derives `Clone` and `Mutex<HashMap>` isn't `Clone`-safe — Arc keeps
shared state across clones, matching how `reqwest::Client` works
internally).

5 integration tests cover the lifecycle: fresh service denies;
LinkCustomerRef alone is insufficient (no subscription); active
Starter + link grants Quorum but not other apps; cancellation
revokes; Free plan never grants Quorum.

In-memory only. v2 promotes the store to StorageKit-backed
persistence so state survives restarts. For Karl's friends/family v1
audience, in-memory + signup-loop testing is acceptable — production
deployment + persistence is a separate plan.

Held for follow-ups:
- Persistent storage (StorageKit-backed)
- Per-plan app lists (Wolfgang and other future apps)
- Webhook replay from Stripe for cold-start state rehydration
- HTTP wrapper service (today this stays a library; the consumer
  embeds it OR builds its own HTTP wrapper)

Spec: `marquee-apps/quorum-sense/docs/superpowers/specs/2026-06-06-quorum-shippable-v1-design.md`.
Plan: `docs/superpowers/plans/2026-06-08-plan-4-labs-entitlement-store.md`.

Unblocks Plan 3b (in quorum-sense repo): Quorum's
`/api/session/start` adds `commerce-rails-stripe` as a dep and calls
`is_entitled(firebase_uid, "quorum")` as a gate before opening a
session. Without this plan, that gate had nowhere to call.

## 2026-06-15 — v0.2.0: persistent EntitlementStore + CR-internal CustomerId (QF-CR-03 + QF-CR-08)

Closes the panel-review acceptance criteria for both QF items in one
landing. Replaces the in-memory `Mutex<HashMap>` store with a persistent
backend over `runway-storage::DocumentStore` (redb local, Firestore
remote) and introduces CR-owned `CustomerId` identity. Source: the
frozen three-architect review
`/Users/kpernyer/dev/reflective/REVIEW_quorum-sense_2026-06-15.md` and
the implementor handoff at
`/Users/kpernyer/dev/reflective/HANDOFF_quorum-sense_2026-06-15.md`.

Changes:

- `commerce-rails-contracts`: new `CustomerId(CommerceId)` newtype —
  opaque CR-owned customer identity, prevents accidental cross-entity
  substitution at API boundaries.
- `commerce-rails-stripe`:
  - `EntitlementStore` rewritten over `Arc<dyn DocumentStore>` with
    three collections (`commerce.firebase_to_customer`,
    `commerce.customer_projections`,
    `commerce.provider_to_customer`). Keys by `CustomerId`, never
    Stripe `cus_*`. Resolves provider IDs at the adapter boundary via
    `resolve_or_mint_customer_id`.
  - `CommerceRails::new` adds `store: Arc<dyn DocumentStore>`
    parameter. **Breaking API change.**
  - `CommerceRails::apply_webhook_action` and `is_entitled` are now
    `async fn`. Storage errors logged via tracing; fail-closed
    (returns `false`). The webhook handler retains the choice to
    return 5xx so Stripe retries.
  - `SubscriptionProjection`, `BillingPlan` get `Serialize` /
    `Deserialize` derives so `Document::new(id, &projection)` works
    directly.
  - New `CommerceRailsError::Storage` variant.
  - Workspace deps add `runway-storage` (path), `anyhow`,
    `async-trait`, `tempfile` (dev-only).

Acceptance criteria met:

- **QF-CR-03**: integration test `entitlement_survives_process_restart`
  in `tests/entitlement_store.rs` proves state survives process
  restart without webhook replay. Phase 1 sets up entitlement, drops
  the rails (releases the redb file handle); Phase 2 opens fresh
  against same path, asserts entitled.
- **QF-CR-08**: `grep -rn 'customer_ref'
  commerce-rails/crates/commerce-rails-stripe/src/` finds matches only
  in the adapter layer (`CommerceWebhookAction` variants + the
  `StripeAdapter` HTTP client); none in domain code. The
  `EntitlementStore` keys by `CustomerId`.

Tests passing: 12 contracts unit tests, 3 stripe unit tests, 6 stripe
integration tests (5 migrated to `tokio::test` + redb backing, 1 new
restart-survival). All green at v0.2.0.

Sub-phase 2b cancelled: originally scoped to migrate
`CommerceWebhookAction` variants to carry `CustomerId` at the action
level. 2a's resolution-in-store design satisfies both QF acceptance
criteria and keeps the adapter boundary clean. The action variants
stay Stripe-shaped because they ARE the adapter layer; consumers
(`runway-accounts`, quorum, future apps) never see them.

Consumer impact (downstream — handled in implementor follow-up):

- quorum-server: 3 call-site changes — pass
  `storage.documents.clone()` to `CommerceRails::new`; add `.await` to
  `is_entitled` and `apply_webhook_action` (handlers already async);
  `require_quorum_entitlement` becomes `async fn`. Tracked in
  `marquee-apps/quorum-sense` against the active HANDOFF.
- runway-accounts: same pattern on the shared webhook handler.

Scale-out gate status (BOUNDARY_REGISTRY Marquee App Contract rule 6):

- ✓ QF-CR-03 (CR persistence)
- ✓ QF-CR-08 (CR-internal CustomerId)
- ☐ RR `D5` (SessionOwnership lease) — still open

quorum-sense remains pinned at `--max-instances=1` until RR `D5`
lands.

Held for follow-ups (separate QF items, not blocking this ship):

- QF-CR-04: deploy recipes for `runway.app.json:deploy_contracts`
- QF-CR-05: `EntitlementProjection` schema +
  `entitlement_projection()` endpoint
- QF-CR-09: extract `commerce-rails-client` (Stripe-free)
- QF-CR-10: `Plan` enum on public API
- QF-CR-11: per-plan apps configured mapping

## 2026-06-15 — v0.2.1: `register_post_apply` callback hook (QF-CR-06)

Adds the in-process callback mechanism for downstream consumers — most
notably `runway-accounts` — to react to entitlement-state mutations
without an event bus or polling loop. Panel-agreed mechanism from the
2026-06-15 review (RR-OQ4 answer: push, not poll; CR Round 2.5
concretization: in-process callback, not Pub/Sub).

Changes:

- `commerce-rails-stripe`:
  - New public type alias `PostApplyCallback = Arc<dyn Fn(&CommerceWebhookAction) + Send + Sync>`.
  - New public struct `CallbackHandle` — guard with `Drop` impl that
    deregisters the callback when dropped. Consumers hold the handle
    for their lifetime (e.g. `runway-accounts` stores it in the
    webhook-handler state).
  - New method `CommerceRails::register_post_apply(callback) ->
    CallbackHandle`.
  - Internal `PostApplyRegistry` keyed by `AtomicU64` IDs, locked
    `HashMap<u64, PostApplyCallback>`. Lock is dropped before
    invoking callbacks to avoid deadlock if a callback re-enters
    `CommerceRails`.
  - `apply_webhook_action` now invokes registered callbacks
    synchronously after a successful store mutation. Callbacks are
    skipped for `Ignored`, `UpdateSubscriptionStatus` against an
    unknown customer, and any storage error — there is nothing to
    refresh in those cases.

Acceptance criteria met:

- Integration test `register_post_apply_fires_callback_after_mutation`
  proves the callback fires exactly once on a successful
  `LinkCustomerRef` apply.
- Three companion tests cover the contract surface:
  `post_apply_callback_does_not_fire_on_no_op_actions` (Ignored +
  unknown-customer no-op suppression),
  `post_apply_callback_deregisters_on_handle_drop` (guard semantics),
  `multiple_post_apply_callbacks_all_fire` (multi-listener fanout).
- The RR-side `runway-accounts` claim-refresh integration test is a
  separate cross-repo deliverable handled by `[RR-ARCH]`.

Tests passing: 12 contracts unit tests, 3 stripe unit tests, 10 stripe
integration tests (6 from v0.2.0 + 4 new callback tests). All green at
v0.2.1.

API surface change (additive — patch bump):

- v0.2.0 → v0.2.1. No breaking changes; existing call-sites compile
  unchanged. Consumers that want the new callback hook opt in by
  calling `register_post_apply`.

Consumer impact:

- `runway-accounts`: opt in by registering a closure at startup that
  calls the Firebase Admin SDK to refresh the custom claim for the
  affected `firebase_uid`. The closure receives the
  `CommerceWebhookAction` reference; it can pattern-match on the
  variant to decide what to refresh.
- quorum-server and other apps: no change. The push refresh is
  invisible to app code; apps continue calling `is_entitled`.

Scale-out gate status (BOUNDARY_REGISTRY Marquee App Contract rule 6):
unchanged from v0.2.0. `D5` still the open leg.

Held for follow-ups (separate QF items):

- QF-CR-04: deploy recipes for `runway.app.json:deploy_contracts`
- QF-CR-09: extract `commerce-rails-client` (Stripe-free)
- QF-CR-10: `Plan` enum on public API
- QF-CR-11: per-plan apps configured mapping

## 2026-06-15 — v0.2.2: `EntitlementProjection` schema + `entitlement_projection()` endpoint (QF-CR-05)

Adds the projection-later read path agreed in the 2026-06-15 panel
review. `is_entitled(uid, app) -> bool` stays as the per-request
hot-path gate; the new `entitlement_projection(uid, app) ->
EntitlementProjection` returns the rich shape for app-shell
initialization and after entitlement transitions (e.g. after a
`refresh-on-403` retry). The field set is panel-locked per RR's B2
amendment.

Changes:

- `commerce-rails-stripe`:
  - New public struct `EntitlementProjection` with panel-locked
    fields: `{ entitled, checkout_url?, portal_url?, signup_url?,
    next_renewal?, plan_label? }`. Optional fields use
    `#[serde(skip_serializing_if = "Option::is_none")]` so absent
    values are not serialized as `null`.
  - New method `CommerceRails::entitlement_projection(uid, app)
    -> EntitlementProjection`. Storage errors are logged and treated
    as "no projection found" — the response still returns the
    configured static URLs so the app shell can render an unentitled
    state correctly.
  - `CommerceRailsConfig` gains three optional URL fields:
    `signup_url`, `checkout_url`, `portal_url`. Constructed via
    builder methods (`with_signup_url`, `with_checkout_url`,
    `with_portal_url`) or read from env (`CR_SIGNUP_URL`,
    `CR_CHECKOUT_URL`, `CR_PORTAL_URL`). Empty strings treated as
    unset.
- `commerce-rails/kb/Contracts/EntitlementProjection.schema.json`:
  - New JSON Schema (draft 2020-12) for the locked field set. Includes
    type constraints, `format: uri` on URL fields, `format:
    date-time` on `next_renewal`, and `enum` on `plan_label`
    (`free` / `starter` / `team` / `enterprise`).
  - `additionalProperties: false` enforces the field-set lock on
    the wire.

Acceptance criteria met:

- `EntitlementProjection` exported in `cargo doc` with the locked field
  set verbatim.
- JSON Schema published at
  `commerce-rails/kb/Contracts/EntitlementProjection.schema.json`.
- `entitlement_projection(uid, app)` callable from the CR client.
- Integration tests at `tests/entitlement_store.rs`:
  - `entitlement_projection_entitled_includes_plan_label_and_next_renewal`
    proves the entitled path including plan label, renewal
    round-trip, and all three static URLs.
  - `entitlement_projection_not_entitled_returns_static_urls` proves
    the unentitled path still surfaces the configured signup URL.
  - `entitlement_projection_omits_unconfigured_urls` proves absent
    config yields `None` (not empty strings).
  - `entitlement_projection_serializes_with_locked_field_set` proves
    the JSON wire shape matches the lock — including
    `skip_serializing_if` behaviour on absent fields and round-trip
    deserialization.

Tests passing: 12 contracts unit tests, 3 stripe unit tests, 14 stripe
integration tests (10 from v0.2.1 + 4 new projection tests). All green
at v0.2.2.

API surface change (additive — patch bump):

- v0.2.1 → v0.2.2. No breaking changes; existing call-sites compile
  unchanged. Consumers that want the rich projection opt in by
  calling `entitlement_projection`. Apps already calling
  `is_entitled` see no impact.

Consumer impact:

- `runway-app-shell` (RR D3b): widget pane is unblocked. Builds the
  shell against the published schema; consumes
  `entitlement_projection` at shell init and on transitions.
- quorum-server and other apps: no immediate change. The HANDOFF
  Section-5 row for QF-CR-05 says: stop hardcoding signup/portal
  URLs in the SPA once the widget moves to `runway-app-shell` (which
  is gated on D3b, not on this ship).

Scale-out gate status (BOUNDARY_REGISTRY Marquee App Contract rule 6):
unchanged from v0.2.1. `D5` still the open leg.

Held for follow-ups (separate QF items):

- QF-CR-04: deploy recipes for `runway.app.json:deploy_contracts`
- QF-CR-09: extract `commerce-rails-client` (Stripe-free)
- QF-CR-10: `Plan` enum on public API
- QF-CR-11: per-plan apps configured mapping
