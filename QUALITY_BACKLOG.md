# Quality Backlog — commerce-rails

Per-repo finding ledger for commerce-rails. Append-only.

> **For implementors:** the active implementor brief is `/Users/kpernyer/dev/reflective/HANDOFF_quorum-sense_2026-06-15.md` (signed by all three architects 2026-06-15). This backlog tracks CR-side commitments referenced from that handoff. If you are looking for "what to do this week" as a Quorum engineer, go to the handoff first.

Originating review: `/Users/kpernyer/dev/reflective/REVIEW_quorum-sense_2026-06-15.md` (FROZEN; signed by all three architects 2026-06-15).
Workspace anchor: `/Users/kpernyer/dev/reflective/BOUNDARY_REGISTRY.md`.
Active implementor handoff: `/Users/kpernyer/dev/reflective/HANDOFF_quorum-sense_2026-06-15.md`.

## Conventions

- **Severity:** A = must fix now (correctness/security/production gate); B = should fix soon; C = strategic; D = needs human decision.
- **State:** Open → In progress → Done. Closed entries stay; never delete (per workspace AGENTS.md).
- **Drift checks** ride alongside the standard each closed finding promotes.
- Cite findings by ID in PRs and ADRs.

## Active findings

### QF-CR-02 — Split M2 charter into M2a + M2b
- **Severity:** B
- **State:** Done (2026-06-15, this Wave-1 landing)
- **Owner:** `[CR-ARCH]`
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` Round 1 finding; M2 named *Wolfgang*, reality is quorum-sense.
- **Scope:** Rewrite `MILESTONES.md:32+` from a single M2 to **M2a — Entitlement-gate proof** (driver = quorum-sense) and **M2b — Partner piggy-back loop** (driver TBD, future). Explicit historical note: Wolfgang moved to `studio-apps/`; quorum-sense overtook as integration driver.
- **Acceptance:** `MILESTONES.md` shows both milestones; no Wolfgang reference outside the historical note; cross-link to BOUNDARY_REGISTRY entry naming quorum-sense as the M3 reference app for RR.
- **Promotes:** nothing; documentation-only.

### QF-CR-03 — `EntitlementStore` v2 (persistent)
- **Severity:** A (hard gate for `--max-instances > 1`)
- **State:** **Done (2026-06-15, v0.2.0 landing — see `kb/LOG.md`)**
- **Owner:** `[CR-ARCH]`
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` Round 1; v1 documented as in-memory in `kb/LOG.md` (Plan 4, 2026-06-08).
- **Scope:** Replace `commerce-rails-stripe/src/lib.rs:265-327` in-memory `Mutex<HashMap>` store with a persistent backend over `runway-storage::DocumentStore` (Firestore in prod, redb local). Webhook re-hydration not needed — state survives restart.
- **Acceptance:** integration test runs `apply_webhook_action` against the persistent store; kills the process; new process starts; `is_entitled(uid, "quorum")` returns the same answer without re-receiving the webhook. **Met by `tests/entitlement_store.rs::entitlement_survives_process_restart` (2026-06-15).**
- **Blocks/blocked by:** sequenced together with QF-CR-08 (same store refactor); CR-side legs of the scale-out gate done. Quorum still pinned at `--max-instances=1` until RR `D5` lands.

### QF-CR-04 — CR deploy recipes referenced by `runway.app.json:deploy_contracts`
- **Severity:** A
- **State:** Open — **v0.1 recipe stable** (published 2026-06-15, upgraded from stub same day per RR-ARCH ask). RR `D4` may build the template against it. Concrete source taxonomy is `secret` (managed secret store; never literal in deploy manifests) vs `config` (plain env var). Recipe stays Open until end-to-end materialization closes the acceptance criterion below.
- **Owner:** `[CR-ARCH]` (recipe definition + publishing format), `[RR-ARCH]` (manifest schema D6 + deploy template D4)
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` Round 2 D6 (RR-ARCH filed) → CR ownership of the recipe format per Round 2 reconciliation.
- **Scope:** Publish CR-owned deploy recipes keyed by `(key, version)` (e.g. `key = "commerce-rails-stripe", version = "0.1"`). Recipe lists the env vars CR owns (`STRIPE_*` + `CR_*` URLs from v0.2.2) so the app's `runway.app.json` declares `deploy_contracts: [{key, version}]` only — never the raw env names. RR's deploy template materializes the env slots by looking up the published recipe.
- **v0.1 recipe contents:** 4 required env vars (`STRIPE_WEBHOOK_SECRET`/`STRIPE_SECRET_KEY` as `source: secret`; `STRIPE_PRICE_TEAM_MONTHLY`/`STRIPE_PRICE_STARTER_MONTHLY` as `source: config` since price IDs are public identifiers); 3 optional CR_* URLs (`source: config`); `forbidden_in_app_deploy` list mirroring the above. Each entry carries `source`, `consumer` (CR call-site), and `semantics`. Recipe also declares `recipe_format_version: "1.0"` so the recipe file format can evolve independently of any individual recipe's `version`. Bump path documented inline: 0.1 → 0.2 (drops STRIPE_PRICE_* when QF-CR-10 lands a CR-internal map) → 1.0 (re-baseline on QF-CR-09 client extraction).
- **Acceptance:** quorum-sense's `deploy/cloud-run-provision.sh` no longer mentions `STRIPE_*`/`CR_*` env vars; the recipe materialization handles them. `just project-doctor` reports zero such strings in any marquee-app deploy script.
- **Blocks/blocked by:** depends on RR's D1 (manifest verifier) + D4 (deploy template). Stub now exists for D4/D6 validation. Pairs with QF-CR-10 (`Plan` enum) — when CR-10 lands, the two `STRIPE_PRICE_*` entries collapse into a CR-internal map and the recipe shrinks (planned bump path v0.1 → v0.2 → v1.0 documented in the stub).
- **Promotes:** standard `RP-NO-PROVIDER-VARS-IN-APP-DEPLOY` in `commerce-rails/kb/05-engineering/standards/` (creates folder).

### QF-CR-05 — Publish `EntitlementProjection` schema + widget contract
- **Severity:** B
- **State:** **Done (2026-06-15, v0.2.2 landing — see `kb/LOG.md`)**
- **Owner:** `[CR-ARCH]` (schema + contract); widget *component* lives in `runway-app-shell` (RR-ARCH, D3b).
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` Round 2 (initially over-committed as "create `commerce-rails-shell` package"; pulled back to contract-only in CR Round 2.5).
- **Scope:** Publish the `EntitlementProjection` Rust type and its JSON Schema. Field set is **panel-locked** at:

  ```
  EntitlementProjection {
      entitled: bool,
      checkout_url: Option<String>,
      portal_url: Option<String>,
      signup_url: Option<String>,
      next_renewal: Option<DateTime<Utc>>,
      plan_label: Option<String>,
  }
  ```

  Per RR B2: adding new optional fields is non-breaking and does not require panel re-review. Renaming or removing a field requires a new dated panel review.
- **Acceptance:** type exported from `commerce-rails-client` (after QF-CR-09 lands; until then, lives in `commerce-rails-stripe`). JSON Schema published under `commerce-rails/kb/Contracts/`. `runway-app-shell` widget builds against the published schema with no back-channel checks. **Met by v0.2.2 (2026-06-15)** — type exported from `commerce-rails-stripe::EntitlementProjection`; JSON Schema at `kb/Contracts/EntitlementProjection.schema.json`; `CommerceRails::entitlement_projection(uid, app) -> EntitlementProjection` callable; four tests cover the contract surface (entitled+plan_label+renewal, not-entitled+static URLs, omit-unconfigured-URLs, locked-field-set serialization round-trip).
- **Blocks/blocked by:** Widget pane in RR's D3b depends on this schema being published — **unblocked**.

### QF-CR-06 — `register_post_apply` callback hook on `CommerceRails`
- **Severity:** A
- **State:** **Done (2026-06-15, v0.2.1 landing — see `kb/LOG.md`)**
- **Owner:** `[CR-ARCH]`
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` Round 2 OQ4 (claim refresh: push vs poll). CR Round 2.5 concretized the mechanism — in-process callback, no event bus.
- **Scope:** Add `CommerceRails::register_post_apply(callback: Arc<dyn Fn(&CommerceWebhookAction) + Send + Sync>) -> CallbackHandle`. Called synchronously inside `apply_webhook_action` after the store mutation succeeds. `runway-accounts` registers a closure that refreshes the Firebase custom claim for the affected `firebase_uid`.
- **Acceptance:** integration test asserts that calling `apply_webhook_action(LinkCustomerRef { firebase_uid, customer_ref })` triggers the registered callback exactly once with the same action variant. `runway-accounts` claim refresh integration test passes. **Met by `tests/entitlement_store.rs::register_post_apply_fires_callback_after_mutation` plus three companion tests (no-op suppression, handle-drop deregistration, multi-callback fanout); RR-side `runway-accounts` claim-refresh test is a separate cross-repo integration deliverable.**
- **Cross-instance coherence:** handled by QF-CR-03 (persistent store) + RR's `refresh-on-403` pattern. Multi-region webhook fan-out is YAGNI.
- **Blocks/blocked by:** independent; landed alongside QF-CR-03/08 ship. Unblocks `runway-accounts` claim-refresh wiring.

### QF-CR-07 — (Closed/folded into QF-CR-05)
- **State:** Closed 2026-06-15.
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` Round 2 — initially filed as "extend entitlement response with `checkout_url`/`portal_url`/`signup_url`."
- **Resolution:** Per RR's CR-OQ4 answer (projection-later), `is_entitled(uid, app_id) -> bool` stays as the hot-path gate; the URL set lives on the separate `entitlement_projection(uid, app_id) -> EntitlementProjection` endpoint. URLs are fields on the panel-locked projection schema (QF-CR-05). No separate work item.

### QF-CR-08 — CR-internal `CustomerId`; `ProviderObjectRef` for Stripe `customer_ref`
- **Severity:** A (boundary debt; current code violates CR's own Stripe boundary rule)
- **State:** **Done (2026-06-15, v0.2.0 landing — see `kb/LOG.md`)**
- **Owner:** `[CR-ARCH]`
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` CR Round 2.5 self-correction #1. `EntitlementStore` today keys `projections: Mutex<HashMap<String, SubscriptionProjection>>` by the Stripe `cus_*` ID directly (`commerce-rails-stripe/src/lib.rs:267-269`). Violates `kb/Adapters/Stripe Connect Boundary.md:57-69` — *"Stripe IDs are external references, not domain IDs."*
- **Scope:** Introduce CR-internal `CustomerId` (opaque, CR-owned, UUID-like). `ProviderObjectRef { provider, external_id }` carries the Stripe `customer_ref`. `EntitlementStore` keys by `CustomerId`. A side mapping `ProviderObjectRef → CustomerId` lets the webhook handler resolve the Stripe ID. `firebase_uid → CustomerId` replaces `firebase_uid → customer_ref`.
- **Acceptance:** `grep -rn 'customer_ref' commerce-rails-stripe/src/` returns zero matches in domain code; provider refs appear only in the Stripe adapter and the `ProviderObjectRef` resolution layer. Existing tests pass. **Met by v0.2.0 landing (2026-06-15)** — adapter-layer matches in `CommerceWebhookAction` variants are correct per the architectural split; domain code (`EntitlementStore` internals, `is_entitled`, `projection_for_customer`) is `customer_ref`-free.
- **Blocks/blocked by:** shipped together with QF-CR-03. Pre-condition for QF-CR-09 (clean API for `commerce-rails-client` extraction) — that extraction is now unblocked.
- **Promotes:** standard `RP-NO-PROVIDER-IDS-IN-DOMAIN` in `commerce-rails/kb/05-engineering/standards/` (file to be added when CR-09 lands).

### QF-CR-09 — Extract `commerce-rails-client` crate (Stripe-free)
- **Severity:** B (build-graph hygiene; closes HELMS F2 framing)
- **State:** Open — NEW SCOPE (does not exist in commerce-rails today)
- **Owner:** `[CR-ARCH]`
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` CR Round 2.5 self-correction #2. App `Cargo.toml` currently reads `commerce-rails-stripe = ...` because there is no Stripe-neutral CR client crate. The import name couples apps to Stripe at the build-graph level even though the code surface is `CommerceRails`.
- **Scope:** New crate `commerce-rails-client` exposing `CommerceRails`, `EntitlementStore` (trait), `is_entitled`, `entitlement_projection`, `Plan` enum (QF-CR-10), `SubscriptionProjection`, `EntitlementProjection` (QF-CR-05), and the `register_post_apply` hook (QF-CR-06). Zero Stripe imports. `commerce-rails-stripe` becomes a behind-the-trait adapter the binary wires at startup.
- **Acceptance:** marquee-app `Cargo.toml` reads `commerce-rails-client = ...` only. `cargo tree -p commerce-rails-client` shows no dependency on `reqwest` or any Stripe crate. quorum-sense's `Cargo.toml:21` migrates from `commerce-rails-stripe` to `commerce-rails-client`.
- **Blocks/blocked by:** depends on QF-CR-08 (clean internal IDs) and QF-CR-10 (`Plan` enum on the public surface).
- **Promotes:** standard `RP-NO-PROVIDER-CRATE-IMPORT-IN-APP` in `commerce-rails/kb/05-engineering/standards/`.

### QF-CR-10 — `Plan` enum on public API; CR-internal `Plan → provider price_ref` mapping
- **Severity:** A (current public API takes Stripe `price_*` IDs)
- **State:** Open
- **Owner:** `[CR-ARCH]`
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` CR Round 2.5 self-correction #3. `CommerceRails::create_checkout_session(customer_ref, price_ref, ...)` at `commerce-rails-stripe/src/lib.rs:184-202` takes a Stripe `price_*` ID as a public-API parameter. `CommerceRailsConfig::new` / `::from_env` (lines 46-106) require `STRIPE_PRICE_TEAM_MONTHLY` / `STRIPE_PRICE_STARTER_MONTHLY` env vars. RR's D6 deploy fix is necessary but not sufficient — the API itself leaks.
- **Scope:** Public API accepts `Plan { Starter | Team | Enterprise }`. CR-internal config maps `Plan → provider price_ref` (read from the deploy recipe published per QF-CR-04). Apps never see Stripe price IDs in API or env.
- **Acceptance:** `grep -rn 'price_ref\|price_team_monthly\|price_starter_monthly' commerce-rails/crates/commerce-rails-client/` returns zero matches; provider price IDs appear only in `commerce-rails-stripe` adapter config. quorum-sense's `runway.app.json` declares `deploy_contracts: [{key: "commerce-rails-stripe", version: "X.Y"}]`; no `STRIPE_PRICE_*` env names anywhere in the app repo.
- **Blocks/blocked by:** pairs with QF-CR-04 (deploy recipe). Pre-condition for QF-CR-09 (clean API surface).

### QF-CR-11 — `Plan → Vec<AppId>` becomes configured mapping
- **Severity:** B (blocks app #2)
- **State:** Open
- **Owner:** `[CR-ARCH]`
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` CR Round 2.5 self-correction #7. `BillingPlan::apps()` at `commerce-rails-stripe/src/lib.rs:348-356` returns hardcoded `vec!["quorum".to_string()]` for every paid plan. Pretending CR is "the platform's commercial layer" while the entitlement model is hardcoded to one app is overclaiming.
- **Scope:** Replace enum-coded `apps()` with a configured mapping. Open question to resolve in design: YAML config alongside Stripe price config? JSON in `runway.app.json`? CR-published JSON Schema consumed at config-load time? Pick one; lock it; document.
- **Acceptance:** adding a second marquee-app to a plan requires only a config change, no code change. Integration test: a `Plan::Team` configured to grant `["quorum", "atlas"]` returns `true` for both apps via `is_entitled`.
- **Blocks/blocked by:** blocks any second marquee-app integration. Must land before app #2 starts implementation.
- **Open question carried:** config shape — see CR-OQ-2026-06-15-A in the frozen review.

## Closed / folded

### QF-CR-07 (closed 2026-06-15)
Folded into QF-CR-05. See above.

### QF-CR-02 (closed 2026-06-15 — Wave-1 landing)
M2 split landed. See above.

## Standards to promote on close

- `RP-NO-PROVIDER-VARS-IN-APP-DEPLOY` — closes with QF-CR-04.
- `RP-NO-PROVIDER-IDS-IN-DOMAIN` — closes with QF-CR-08.
- `RP-NO-PROVIDER-CRATE-IMPORT-IN-APP` — closes with QF-CR-09.

## Sequencing summary

**Wave 2 (unblocks `--max-instances > 1`):** QF-CR-03 + QF-CR-08 (sequenced together, same store refactor) + RR's D5.

**Wave 4 (contract evolution):** QF-CR-04 (with RR D1+D4) → QF-CR-10 → QF-CR-09 → HELMS rehoming.

**Wave 5 (shell + projection):** QF-CR-05 schema → QF-CR-06 callback → RR D3b widget consumes.

**Wave 6 (pre-app-#2):** QF-CR-11.

## Cross-references

- Workspace registry: `/Users/kpernyer/dev/reflective/BOUNDARY_REGISTRY.md`
- Frozen review: `/Users/kpernyer/dev/reflective/REVIEW_quorum-sense_2026-06-15.md`
- Runtime-Runway backlog: `/Users/kpernyer/dev/reflective/runtime-runway/QUALITY_BACKLOG.md`
- Helms backlog: pending — link here when filed
- Architectural docs: `kb/Architecture/Operating Authority Boundary.md`, `kb/Contracts/Commerce Rail Surface.md`, `kb/Contracts/Apps Consuming Commerce Rails.md`
