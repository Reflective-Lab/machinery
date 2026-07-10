# Quality Backlog — runtime-runway

Per-repo finding ledger for runtime-runway. Append-only.

> **For implementors:** the active implementor brief is `(reflective-root)/HANDOFF_quorum-sense_2026-06-15.md` (signed by all three architects 2026-06-15). This backlog tracks RR-side commitments referenced from that handoff. If you are looking for "what to do this week" as a Quorum engineer, go to the handoff first.

Originating review: `(reflective-root)/REVIEW_quorum-sense_2026-06-15.md` (FROZEN; signed by all three architects 2026-06-15).
Workspace anchor: `(reflective-root)/BOUNDARY_REGISTRY.md`.
Active implementor handoff: `(reflective-root)/HANDOFF_quorum-sense_2026-06-15.md`.

## Conventions

- **Severity:** A = must fix now (correctness/security/production gate); B = should fix soon; C = strategic; D = needs human decision.
- **State:** Open → In progress → Done. Closed entries stay; never delete (per the workspace AGENTS.md).
- **Drift checks** ride alongside the standard each closed finding promotes.
- Cite findings by ID in PRs and ADRs.

## Active findings

### RR-STRAT-01 — Ambient job substrate
- **Severity:** A
- **State:** In progress — **v1 on `main`** (`runway-ambient` crate); Pub/Sub pull consumer + panel enqueue contract remain.
- **Owner:** `[RR-ARCH]`
- **Origin:** Strategy review 2026-06-17 (`docs/strategy/REVIEW-runtime-runway-2026-06-17.md`)
- **Scope:** Durable enqueue/claim/complete queue on `DocumentStore`, provenance on `EventLog`, optional Pub/Sub notify, background worker, HTTP enqueue surface, `runway-app-host` integration when packet declares jobs.
- **Acceptance (v1):** documented contract in `crates/runway-ambient/README.md`; quorum can enqueue preparatory work via HTTP and process via `AmbientJobHandler`; `just test-ambient` green.
- **Remaining:** Pub/Sub pull worker (wake without poll), Converge/Organism enqueue API panel sign-off, reference app adoption.
- **Progress log:**
  - 2026-06-17: Shipped `crates/runway-ambient` — `AmbientJobQueue`, `AmbientWorker`, `PubSubNotify`, `POST/GET /v1/ambient/jobs`, `RunwayAppHostBuilder::with_ambient_handler`, commit-bound phase gate. 9 unit tests + app-host integration unchanged (41 tests).
  - 2026-06-17: **`QuorumAmbientHandler`** reference (`runway-ambient/src/quorum.rs`) — `sensemap-refresh`, `mnemos-recall`, `drift-scan`; adoption guide `docs/adoption/quorum-ambient-handler.md`; app-host integration test `ambient_quorum_test.rs`. Quorum repo wiring is copy-paste from the guide (repo not mounted in cloud agent).
  - 2026-06-17: **Merged to `main`** with durable spine, D5/D6 verifier, and M4/app-shell branches. Quorum can depend on `main` path for `runway-ambient`.

### RR-STRAT-02 — Durable EventHub → EventLog spine
- **Severity:** A
- **State:** Done (v1) — **on `main`**
- **Owner:** `[RR-ARCH]`
- **Origin:** Strategy review 2026-06-17 (`docs/strategy/REVIEW-runtime-runway-2026-06-17.md`)
- **Scope:** `EventHub::with_event_log()` persists hub events to `EventLog` and replays on subscribe; `subscribe_with_cursor` is async; integration tests in `realtime.rs`.
- **Acceptance:** events survive process restart via `EventLog`; cursor replay works; `runway-app-host` tests green.
- **Progress log:**
  - 2026-06-17: Shipped on `cursor/durable-event-spine-b135`; merged to `main` after conflict resolution in `builder.rs` (preserved `with_ambient_handler`).

### D1 — Manifest verifier (strict-always)
- **Severity:** A
- **State:** Done (RR implementation) — **all three checks enforced strict-always** in `serve()`. App adoption (register domain routes via `route_*` + declare them in `domain_routes`) tracked downstream in the Quorum handoff. See `docs/superpowers/specs/2026-06-15-d1-manifest-verifier-redesign.md`.
- **Owner:** `[RR-ARCH]`
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` Round 1, sharpened by HELMS F3 in Round 2.
- **Scope (intent preserved; mechanism revised):** `runway-app-host` rejects manifest/router divergence inside `BuiltHost::serve()` (no flag, every environment). Three checks:
  1. Every `RouteRegistration { owner: AppDomain }` in `packet.domain_routes` matches a **registered** route. *(Revised — `axum::Router` exposes no route introspection; replaced by a manifest-authoritative route-registration API so the served router is built from the registered set and the verifier is a set-diff. See redesign note §3.)*
  2. Every mounted `HelmModule` reports `module_state()` ≠ `ModuleState::Shell`. (Catches the planned-vs-mounted lie that D2 surfaced.) *(Requires adding `HelmModule::module_state()`, default `Shell`. See redesign note §5.)*
  3. No registered route's `handler_id` contains `for_test` / `_test` / `test_only`. *(Revised — stable Rust has no handler-symbol reflection; replaced by a build-time scan of the declared identifier. Catches `apply_proposals_for_test` HTTP exposure surfaced by HELMS F4. See redesign note §4.)*
- **No flag.** Per `runtime-runway/CLAUDE.md`: no feature flags. `serve()` returns `Err` on mismatch in every environment.
- **Acceptance:** see redesign note §6 — four directions (manifest-claims-unregistered, registered-undeclared, mounted-but-shell, `_test` handler) each force `serve()` to `Err`.
- **Promotes:** standard `RP-MANIFEST-MATCHES-ROUTER` in `runtime-runway/kb/05-engineering/standards/` (creates the folder).
- **Blocks/blocked by:** unblocks downgrade of D2 and HELMS F3 lies; should land while D5 is in flight so it catches regressions on D5's session-ownership wiring.
- **Progress log:**
  - 2026-06-15: Redesign note opened. Original runtime-introspection framing for checks 1 & 3 is infeasible (`axum::Router` has no route enumeration; no handler-symbol reflection in stable Rust). Note proposes: invert data flow so the manifest route list is authoritative — Phase 1 = `ModuleState` + `HelmModule::module_state()` + check 2 (low-risk, self-contained); Phase 2 = route-registration API + checks 1 & 3 as set-diff + identifier scan (app-facing migration). Recommends explicit registration API over a proc-macro. Open questions carried to the panel.
  - 2026-06-15: **Phase 1 shipped** on `next`. `ModuleState { Shell, Live }` + `HelmModule::module_state()` (default `Shell`, fails closed). `verify_manifest` runs inside `BuiltHost::serve()` before bind: any `mount_kind: "mounted"` module that is absent or reports `Shell` → `serve()` returns `Err`, every environment, no flag. Standard `RP-MANIFEST-MATCHES-ROUTER` promoted (`kb/05-engineering/standards/`).
  - 2026-06-15: **Phase 2 shipped** on `next`. Route-registration API (`RunwayAppHostBuilder::route_get/post/put/patch/delete`) records `(method, path, owner, handler_id)` as data and builds the served router from it, so manifest↔router cannot drift. `verify_manifest` now also runs **check 1** (AppDomain `domain_routes` ↔ registered routes, both directions → `Err`) and **check 3** (registered `handler_id` via `std::any::type_name` containing `_test`/`for_test`/`test_only` → `Err`; verified end-to-end that a real `*_for_test` fn is caught without an explicit id). 12 verifier tests total (unit + serve-level + served-route). `just lint` + `just test` green (310 + crate suites). Standard matrix updated to all-enforced; redesign-note §8 open questions resolved (Option A taken, type_name for check 3). RR implementation complete; app adoption is the remaining downstream step.

### D2 — `mount_kind` reconciliation (manifest vs code)
- **Severity:** A
- **State:** Open
- **Owner:** `[RR-ARCH]` (manifest schema), `[Quorum app]` (their specific reconciliation), `[HELMS-ARCH]` (live-state contract for the modules)
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` Round 1 D2; revised after HELMS F3.
- **Scope:** Either (a) back out the two `.mount(...)` calls in `quorum-server/src/main.rs:1900–1901` and keep `runway.app.json` `mount_kind: "planned"`, OR (b) HELMS wires the live `JobReadinessPacket` and quorum's manifest moves to `"mounted"`. No third option. D1 (above) makes this structurally enforceable going forward.
- **Acceptance:** for quorum specifically — `runway.app.json` `mount_kind` value matches what D1's verifier reports about module state. Reproducible by `just lint` returning success.
- **Closed by:** D1 verifier (structural) + quorum's reconciliation OR HELMS H-2026-06-15-01 wiring (case-specific).
- **Progress note (2026-06-15):** the structural half is now live — D1 check 2 is enforced in `serve()`, so a `mount_kind: "mounted"` module that is still a `Shell` fails at boot. D2 closes for quorum specifically once quorum either reports `ModuleState::Live` for those modules (HELMS wiring) or sets `mount_kind: "planned"`. RR side is done.

### D3a — `RunwayAppHostBuilder::with_spa(...)`
- **Severity:** B
- **State:** Done (RR primitive) — Quorum adoption tracked in Quorum's repo.
- **Owner:** `[RR-ARCH]`
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` Round 1 D3, escalated by HELMS F6.
- **Scope:** Add `with_spa(SpaConfig { dist_dir: PathBuf, route_prefix: String, fallback_index: PathBuf })` to `RunwayAppHostBuilder`. Single env contract `RUNWAY_SPA_DIST` (via `SpaConfig::from_env`). Apps drop `tower-http/fs` from their Cargo.toml.
- **Acceptance:** quorum-server's `main.rs:1893–1908` `tower_http::services::{ServeDir, ServeFile}` import + `nest_service` block becomes a single `.with_spa(...)` call.
- **Blocks/blocked by:** depends on nothing; can land any time.
- **Progress log:**
  - 2026-06-15: Shipped on `next`. `SpaConfig` + `RunwayAppHostBuilder::with_spa`; `tower-http` promoted to a normal dep with `fs`. Sub-prefix uses `nest_service`, root prefix uses `fallback_service` so app/host routes keep precedence. Two builder integration tests (asset serving, deep-link fallback, route precedence).
  - 2026-06-15: **Correctness improvement over the hand-rolled block.** The pattern the apps copied (`ServeDir::not_found_service(ServeFile)`) serves the SPA shell **but preserves HTTP 404** — that breaks history-API deep links (it only worked for Quorum because Quorum uses hash routing). `with_spa` uses `ServeDir::fallback(ServeFile)` instead, which returns **200** for unknown deep links. Verified by `with_spa_*` tests. Apps adopting `with_spa` get history-API routing for free.

### D3b — `runway-app-shell` crate
- **Severity:** B
- **State:** In progress — **Rust scaffold on `main`**; Svelte/TS components remain
- **Owner:** `[RR-ARCH]`; widget contract from `[CR-ARCH]` (QF-CR-05)
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` HELMS F6 escalation of D3.
- **Scope:** New crate `runway-app-shell` packaging the Svelte/TS components for shared topbar, Firebase auth bootstrap, entitlement redirect handling, and the entitlement widget against CR's `EntitlementProjection` schema (panel-locked, see `kb/Architecture/App Execution Container.md`). Plus Rust handlers for the auth-bootstrap endpoints.
- **Acceptance:** quorum's `apps/desktop/src/lib/{MarqueeTopbar,DesktopShell,firebase-client,product-api}.ts` can be deleted; replaced by imports from the package.
- **Blocks/blocked by:** Widget pane depends on QF-CR-05 (EntitlementProjection schema published). Auth-bootstrap pane independent.
- **Open question carried to next review:** does CR want to absorb the widget once `commerce-rails-client` (CR-09) stabilizes and a third marquee app ships? Defer.
- **Progress log:**
  - 2026-06-17: **Rust scaffold shipped** on `main` (`crates/runway-app-shell`). `EntitlementProjection`, auth bootstrap payloads, `ShellState`/`ShellConfig`, Axum routes (`/v1/shell/*`). See `crates/runway-app-shell/README.md`. Frontend widget layer is a follow-up.

### D4 — Math-base semantic tags + Cloud Run deploy template
- **Severity:** B (semantic-tag publish); A (`deploy_contracts: [{key, version}]` materialization, because D6 depends on it)
- **State:** Done (RR implementation) — registry push + Quorum adoption are downstream ops/Quorum steps (see progress log + Quorum handoff).
- **Owner:** `[RR-ARCH]`
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` Round 1 D4 + D6 deploy split.
- **Scope:**
  1. Publish `kenneth-backend-math-base` with semantic tags (`v1.96.0-ortools-9.14.0-highs-1.14.0`). Ban `:latest` for downstream pulls.
  2. Ship Cloud Run deploy template under `runtime-runway/ops/templates/` that materializes `runway.app.json:deploy_contracts: [{key, version}]` by reading CR-published recipes (matched by `key + version`). App never writes provider env-var names.
- **Acceptance:** quorum's `deploy/cloud-run-provision.sh` becomes a thin caller of the RR template; `STRIPE_*` env names appear nowhere in quorum's repo.
- **Promotes:** standard `RP-NO-MATH-BASE-LATEST` in `runtime-runway/kb/05-engineering/standards/`. Drift check: `just math-base-audit` (the runnable check the root `just project-doctor` should call) greps sibling Dockerfiles for `math-base:latest` / `math-base@sha256` and reports.
- **Blocks/blocked by:** ~~template depends on CR's recipe format (QF-CR-04)~~ — **unblocked 2026-06-15**: QF-CR-04 published `commerce-rails-stripe@0.1.yaml` (`recipe_format_version: "1.0"`).
- **Progress log:**
  - 2026-06-15: **RR implementation shipped** on `next`. Two pieces, both verified locally:
    - **Deploy template** (`ops/templates/`). `materialize-deploy-contracts.sh` reads `runway.app.json:deploy_contracts` (the D6 field) + the CR recipe `<key>@<version>.yaml` and emits eval-able `RUNWAY_DEPLOY_ENV_VARS` / `RUNWAY_DEPLOY_SECRETS`: `source: secret` → `--set-secrets NAME=<SECRET_PREFIX>-<lower(name|_->->)>:latest` (convention reproduces quorum's `quorum-stripe-secret-key` etc.); `source: config` → `--set-env-vars NAME=<value-from-deploy-env>`, where required+unset is a **hard error** and optional+unset is skipped. `cloud-run-deploy.sh` is the thin wrapper apps call. `DRY_RUN=1` verified it reproduces quorum's exact `gcloud run deploy` invocation with `STRIPE_*` materialized from the recipe — so quorum's `cloud-run-provision.sh` STRIPE_* lines (49–50, 54–55) delete. No `yq` dependency: the recipe parser is `awk`, keyed to `recipe_format_version: "1.0"` with a version guard. `just deploy-materialize`.
    - **math-base semantic tag.** `ops/scripts/build-math-base.sh` derives `v<rust>-ortools-<ortools>-highs-<highs>` (today `v1.96-ortools-9.14-highs-1.14.0`) from the `Dockerfile.math-base` ARGs and refuses any `:latest`-shape tag; `cloudbuild.math-base.yaml` default flipped from `latest` to a non-valid sentinel so an unparameterized build fails loudly; Dockerfile header rewritten to the script path. `just math-base-tag` / `math-base-build`. Standard `RP-NO-MATH-BASE-LATEST` promoted (`kb/05-engineering/standards/`) + `just math-base-audit` drift check (rg with VCS-ignore override; catches the 2 live `:latest` pins in `quorum-sense/deploy/backend/Dockerfile.cloudrun:54,125`).
  - 2026-06-15: **Not executed in this env (no docker/gcloud):** the actual `gcloud builds submit` push of the semantic-tagged image, and Quorum's downstream adoption (FROM → semantic tag; declare `deploy_contracts` + delete STRIPE_*). RR tooling/scheme/standard/drift-check complete; those are the remaining ops + Quorum-repo steps.

### D5 — `SessionOwnership` lease primitive
- **Severity:** A (hard gate for `--max-instances > 1`)
- **State:** Done (RR implementation) — Tier-1 + Tier-2 (Firestore-emulator in `.github/workflows/contract.yml`) on `main`.
- **Owner:** `[RR-ARCH]`
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` HELMS F5; accepted by `[RR-ARCH]` in Round 2.
- **Scope:** Primitive in `runway-app-host` or `runway-storage`: lease per `(org_id, app_id, session_id)`, with TTL, renewal, and steal semantics. Backing store via `runway-storage::DocumentStore` (Firestore in prod, redb local). Exposed as a tower middleware that applies to mutating routes; apps mark routes via attribute or layer.
- **Acceptance:** integration test runs two `RunwayAppHost` instances against shared Firestore; one acquires lease, the other's mutating-route call returns `409 ownership_held`. Renewal under load passes a 30-second steady-state test.
- **Blocks/blocked by:** unblocks marquee apps from `--max-instances=1` pin. Compounds with QF-CR-03 (EntitlementStore v2 persistence) — both required before scaling.
- **Progress log:**
  - 2026-06-15: Spec committed (`docs/superpowers/specs/2026-06-15-d5-session-ownership-design.md`, SHA 9d0504c).
  - 2026-06-15: Implementation complete on branch `d5-session-ownership-design` (Tasks 1–16). `LeaseStore` trait + redb/Firestore backends + cross-backend contract suite; `SessionOwnershipLayer`/`LeaseGuard` middleware; `AppExecutionPacket.ownership_exempt_routes`. Tier-1 acceptance passes locally (lease contract suite × redb, Variant-1 two-host, 30s renewal-under-load, manifest field, `just lint`).
  - 2026-06-15: **Contract clarification (panel/consumer decision).** D5 is a *per-request admission* lease (acquired before the handler, released on response completion — spec §5/§7/§8), NOT sticky session affinity. Sequential non-overlapping mutating requests from different instances may both succeed; only overlapping handlers are serialized. The original spec §9 sequential `200→409→TTL-steal` assertions were wrong for non-overlapping traffic and were rewritten to overlap-based contention (Classification: Contract update). Added `SessionOwnershipLayer::holder_id(...)` identity override (non-softening) so two instances can contend inside one test process.
  - 2026-06-15: Standards promoted to `kb/05-engineering/standards/`: `RP-NO-FEATURE-FLAG-SOFTENING`, `RP-NO-LEASE-WITHOUT-FENCING-V1`.
  - Tier-2 (Firestore-emulator: `just test-lease-firestore`) wired into `.github/workflows/contract.yml` (ownership Variant-2 + `lease_contract` via emulator suite). Local emulator run not executed in this env (no docker); CI is the D5 ship gate.

### D6 — `deploy_contracts` in `runway.app.json`
- **Severity:** A
- **State:** Done (RR implementation) — schema + materialization (D4) + verifier (check 4) on `main`. Quorum adoption (declare contracts, delete STRIPE_* lines) is downstream.
- **Owner:** `[RR-ARCH]` (manifest schema), `[CR-ARCH]` (recipe publish, QF-CR-04)
- **Origin:** `REVIEW_quorum-sense_2026-06-15.md` Round 2 D6.
- **Scope:** Add `deploy_contracts: [{key: String, version: String}]` field to `AppExecutionPacket`. D1's verifier validates the field is well-formed and that every declared `key+version` has a CR-published recipe. RR's deploy template (D4) materializes env-var slots from the recipe.
- **Acceptance:** quorum's `runway.app.json` declares `[{key: "commerce-rails-stripe", version: "X.Y"}]`; `STRIPE_*` env vars in `quorum-sense/deploy/cloud-run-provision.sh` removed; deploy still functions end-to-end.
- **Blocks/blocked by:** depends on QF-CR-04 (CR recipe format).
- **Progress log:**
  - 2026-06-15: **Schema field shipped** on `next`. `DeployContract { key, version }` + `AppExecutionPacket.deploy_contracts` (`#[serde(default)]`) + `with_deploy_contract(...)` builder + parse/default/builder tests. The field is now declarable by apps. **Still open:** (1) D1 verifier validation of `key+version` against published recipes — blocked on D1; (2) materialization into env-var slots — blocked on D4 template + QF-CR-04 recipe format. Field addition is the unblocking prerequisite for both.
  - 2026-06-15: **Materialization leg landed** via D4. The deploy template (`ops/templates/materialize-deploy-contracts.sh`) now consumes this field end-to-end and turns each `{key, version}` into Cloud Run env/secret flags from the CR recipe. Remaining D6 leg: D1's manifest verifier validating each `key+version` resolves to a published recipe (deferred — the verifier currently has no recipe-directory handle; tracked with D1 app-adoption).
  - 2026-06-17: **Verifier leg shipped.** `verify_manifest` check 4 validates non-empty `key`/`version` and, when `CR_RECIPES_DIR` is set, requires `<key>@<version>.yaml` for each declared contract (same convention as `materialize-deploy-contracts.sh`). `serve()` fails closed before bind. Unit + serve-level tests in `builder::tests`.

## Closed / folded

(None yet — this backlog opens 2026-06-15.)

## Standards to promote on close

- `RP-MANIFEST-MATCHES-ROUTER` (promoted Phase 1 2026-06-15; check 2 enforced, checks 1 & 3 reviewer-enforced until D1 Phase 2).
- `RP-NO-FEATURE-FLAG-SOFTENING` (already a CLAUDE.md rule; promoted to a standards page when D1 ships so it has a citable home).
- `RP-NO-MATH-BASE-LATEST` (**promoted 2026-06-15** with D4 — `kb/05-engineering/standards/RP-NO-MATH-BASE-LATEST.md`; drift check `just math-base-audit`).

## Cross-references

- Workspace registry: `(reflective-root)/BOUNDARY_REGISTRY.md`
- Commerce-Rails backlog: `(reflective-root)/commerce-rails/QUALITY_BACKLOG.md`
- Helms backlog: pending — link here when filed
- Architectural doc: `kb/Architecture/App Execution Container.md`
