# D5 — SessionOwnership Lease Primitive

**Status:** Design approved 2026-06-15.
**Owner:** `[RR-ARCH]` (Runtime-Runway).
**Originating review:** `(reflective-root)/REVIEW_quorum-sense_2026-06-15.md` (panel finding HELMS F5; RR Round-2 accept).
**Implementor handoff row:** `(reflective-root)/HANDOFF_quorum-sense_2026-06-15.md` Section 2 — `D5`.
**RR backlog entry:** `runtime-runway/QUALITY_BACKLOG.md` → `D5`.
**Severity:** A — hard gate for `--max-instances > 1` on every marquee app (paired with `QF-CR-03` + `QF-CR-08`).
**Target window:** ~3–4 weeks (panel estimate; new scope).

## 1. Summary

D5 introduces a per-`(org_id, app_id, session_id)` lease primitive that prevents two `RunwayAppHost` instances from concurrently mutating the same domain aggregate. The lease is acquired at the HTTP boundary by a tower middleware (`SessionOwnershipLayer`) and released on response completion. v1 is **admission-time correctness only** — the storage layer enforces no fencing on writes already past the gate. The documented stale-writer gap (process pauses longer than TTL → woken process writes after losing the lease) is tracked as `D5.1 — lease fencing`.

**Lifetime clarification (amended 2026-06-15):** the lease is scoped to a single in-flight mutating request — acquired before the handler runs and released when the response completes (§5/§7/§8). It does **not** pin a session to an instance across sequential requests. Two sequential, non-overlapping mutating requests to the same scope — even from different instances — may both succeed; that is correct behavior behind a round-robin load balancer. The guarantee is only that two *overlapping* mutating handlers on the same scope cannot run concurrently. "SessionOwnership" means ownership of the session aggregate *for the duration of a mutating request*, not sticky session affinity.

## 2. Scope (in / out for v1)

**In scope:**
- `LeaseStore` sibling trait in `runway-storage::traits` with concrete types: `LeaseScope`, `LeaseRecord`, `AcquireOutcome`, `RenewOutcome`.
- Two backend implementations: `LocalLeaseStore` (redb) and `RemoteLeaseStore` (Firestore).
- Cross-backend contract suite in `runway-storage-contract::lease`.
- `SessionOwnershipLayer` tower layer in `runway-app-host`, with `for_app(...)`, `path_param(...)`, `ttl(...)`, `renew_interval(...)` builder.
- Internal `LeaseGuard` (RAII) with implicit background renew task and fire-and-forget release on drop.
- Two-host integration test against shared storage (both redb-shared and Firestore-emulator variants).

**Definition of "mutating" route (for v1):** any route whose method is `POST`, `PUT`, `PATCH`, or `DELETE` AND that mutates per-session domain state. Methods alone are not sufficient — see Section 6 for the route-grouping rule. `GET`, status, integrity, health, SSE-readonly streams, and routes mutating non-session-scoped state (org-scoped recompute, billing webhooks) are not in scope.

**Out of scope for v1, deferred to D5.1:**
- Write-side fencing (`LeasedDocumentStore` CAS wrapper or monotonic `fence: u64` token threaded through every write).
- Handler-visible `LeaseGuard` API (graceful-abort opt-in for SSE / long-streaming handlers).
- Cross-DC active-active write coherence (single-region failover only).

**Explicit non-goals (v1 must NOT do):**
- Modify `DocumentStore`, `EventLog`, `VectorStore`, `ObjectStore`, `EmbeddingProvider` traits.
- Introduce a `--local-dev` softening switch (forbidden by Marquee App Contract Rule 7; local-dev safety comes from redb's single-process serialization).
- Apply to non-mutating routes per the definition above.

## 3. Architecture — component layout

| Path | New / Edit | Purpose |
|---|---|---|
| `runway-storage/src/traits/lease.rs` | NEW | `LeaseStore` trait + concrete types. |
| `runway-storage/src/traits/mod.rs` | EDIT | `+ pub mod lease;` |
| `runway-storage/src/local/lease.rs` | NEW | `LocalLeaseStore` (redb `WriteTransaction`). |
| `runway-storage/src/remote/lease.rs` | NEW | `RemoteLeaseStore` (Firestore `runTransaction`). |
| `runway-storage/src/local/mod.rs` | EDIT | `+ pub mod lease;` |
| `runway-storage/src/remote/mod.rs` | EDIT | `+ pub mod lease;` |
| `runway-storage/src/lib.rs` | EDIT | Re-export `LeaseStore` and types; `StorageKit { ..., leases: Arc<dyn LeaseStore> }`. |
| `runway-storage-contract/src/lease.rs` | NEW | `run_lease_suite(...)` contract suite. |
| `runway-storage-contract/src/lib.rs` | EDIT | `+ pub mod lease;` |
| `runway-storage/tests/contract_local.rs` | EDIT | `+ lease_contract` test. |
| `runway-storage/tests/contract_remote_emulator.rs` | EDIT (or NEW if absent) | `+ lease_contract` test. |
| `runway-app-host/src/ownership.rs` | NEW | `SessionOwnershipLayer`, `SessionOwnershipMiddleware`, internal `LeaseGuard`. |
| `runway-app-host/src/lib.rs` | EDIT | `pub use ownership::*;` |
| `runway-app-host/tests/ownership_test.rs` | NEW | Two-host integration test (shared redb fixture + Firestore-emulator variant). |

No changes to `runway-auth`, `runway-middleware`, `runway-secrets`, `runway-telemetry`, `runway-accounts`, or `api-server`. No changes to the existing `RunwayAppHostBuilder` signature. No changes to `DocumentStore` or any other storage trait.

## 4. Public API surface

```rust
// runway-storage/src/traits/lease.rs

pub struct LeaseScope {
    pub org_id: String,
    pub app_id: String,
    pub session_id: String,
}

pub struct LeaseRecord {
    pub holder_id: String,
    pub expires_at: DateTime<Utc>,
}

pub enum AcquireOutcome {
    Acquired(LeaseRecord),
    HeldByOther(LeaseRecord),
}

pub enum RenewOutcome {
    Renewed(LeaseRecord),
    Lost { current: Option<LeaseRecord> },
}

#[async_trait]
pub trait LeaseStore: Send + Sync {
    async fn try_acquire(
        &self,
        scope: &LeaseScope,
        holder_id: &str,
        ttl: Duration,
    ) -> Result<AcquireOutcome>;

    async fn renew(
        &self,
        scope: &LeaseScope,
        holder_id: &str,
        ttl: Duration,
    ) -> Result<RenewOutcome>;

    async fn release(&self, scope: &LeaseScope, holder_id: &str) -> Result<()>;

    async fn current(&self, scope: &LeaseScope) -> Result<Option<LeaseRecord>>;
}
```

```rust
// runway-app-host/src/ownership.rs

pub struct SessionOwnershipLayer { /* private */ }

impl SessionOwnershipLayer {
    pub fn for_app(app_id: impl Into<String>) -> Self;

    /// Path-parameter name to extract session_id from. Default: "id".
    pub fn path_param(self, name: impl Into<String>) -> Self;

    /// Lease TTL. Default: Duration::from_secs(60).
    pub fn ttl(self, ttl: Duration) -> Self;

    /// Background renewal interval. Default: Duration::from_secs(15).
    pub fn renew_interval(self, d: Duration) -> Self;

    /// Override the lease holder identity. Default: the process-static
    /// `process_holder_id()` (correct in production — one identity per process).
    /// Override only to model multiple instances inside a single process, e.g.
    /// an in-process two-host integration test. This is identity configuration,
    /// not a softening switch (RP-NO-FEATURE-FLAG-SOFTENING): the lease check is
    /// enforced identically regardless of the value.
    pub fn holder_id(self, id: impl Into<String>) -> Self;
}

impl<S> Layer<S> for SessionOwnershipLayer { /* ... */ }
```

`LeaseGuard` is internal to the middleware in v1 — handlers do not see it. The type name `SessionLeaseLost` (a `tokio::sync::watch::Receiver<()>`) is reserved for D5.1 to deliver renewal-loss notification to opt-in handlers, but is not inserted into request extensions in v1.

## 5. Data flow per mutating request

```
1. AuthLayer runs  →  AuthContext inserted into request extensions.
2. SessionOwnershipLayer middleware runs:
     a. Read AuthContext from extensions.
        If absent → 400 BAD_REQUEST "ownership_requires_auth"
        (matches Quorum's existing missing-org_id convention).
     b. Read AuthContext.org_id().
        If absent → 400 BAD_REQUEST "ownership_requires_org".
     c. Read MatchedPath from request → extract configured path_param value.
        If path has no matching param → 400 BAD_REQUEST "ownership_requires_session_id".
     d. Build LeaseScope { org_id, app_id (from layer config), session_id }.
     e. Use process-static holder_id (computed once at first use).
     f. Call leases.try_acquire(scope, holder_id, ttl).
        - Ok(Acquired(rec)) → spawn renew task; build LeaseGuard; proceed.
        - Ok(HeldByOther(rec)) → 409 CONFLICT with body:
            {
              "error": "ownership_held",
              "session_id": "<value>",
              "holder_expires_at": "<rec.expires_at ISO-8601>"
            }
        - Err(_) → 503 SERVICE_UNAVAILABLE with body:
            { "error": "lease_store_unavailable" }
3. Handler runs (no lease awareness).
4. Response complete → LeaseGuard drops → renew task aborted →
   `tokio::spawn` fires-and-forgets release(scope, holder_id).
   Errors from release logged at WARN only; lease will TTL-expire if release fails.
```

### holder_id format

Computed once per process via `OnceLock<String>`:
```rust
format!("{}:{}", env::var("K_REVISION").unwrap_or_else(|_| "local".into()), Uuid::new_v4())
```
Logged at INFO on every `try_acquire` (Acquired or HeldByOther) and on every `Lost` renewal outcome, so Cloud Run log correlation works without parsing lease docs.

## 6. Route grouping (consumer guidance for Quorum)

The middleware is applied per route-group, not globally by HTTP method. Three groups for Quorum:

| Group | Layer config | Routes (representative) |
|---|---|---|
| Inquiry-scoped mutating | `.path_param("id")` | `POST /inquiry/{id}/signal`, `POST /inquiry/{id}/probes/allocate`, `POST /inquiry/{id}/rounds/{round_id}/phase`, `POST /inquiry/{id}/consent`, `POST /inquiry/{id}/presence`, formation paths, **plus the DELETE on this group: `DELETE /inquiry/{id}/consent/{participant_id}`**. ~90% of mutating traffic. |
| Session-join | `.path_param("inquiry_id")` | `POST /api/sessions/{inquiry_id}/join` (same lease key, different param name). |
| **Exempt — no layer applied** | n/a | See exempt list below. |

### Exempt list (explicit, v1)

| Route | Reason exempt |
|---|---|
| `POST /inquiry` | Aggregate-creating; no session_id exists yet. |
| `POST /inquiry/contracted` | Aggregate-creating; no session_id exists yet. |
| `POST /api/session/start` | Aggregate-creating; no session_id exists yet. |
| `POST /sensemap/recompute` | Org-scoped, not session-scoped — would need a different lease key. |
| `POST /sensemap/anticipatory-signals/detect` | Org-scoped sibling of `/sensemap/recompute`. |
| `POST /inquiry/intent/compile` | No `{id}` in path. Long-running. |
| `POST /inquiry/intent/compile/stream` | No `{id}` in path. Long-running SSE. |
| `POST /acquisition/unresolved-questions/originate` | Aggregate-creating; org-wide spike path; no session_id. |
| `POST /acquisition/unresolved-questions/{id}/signal` | `{id}` is an acquisition-question id, not an inquiry session — different aggregate semantics. Acquisition-question leasing is tracked as separate future work, not part of D5 v1. |

### Footnotes (non-Quorum-domain routes, exempt by construction)

- **Helm mounted modules** (`POST /v1/jobs/{key}/stream`, operator-control previews): out of Quorum consumer scope. `{key}` is a job key, not an inquiry session. Either treated as a platform-deferred mount or exempted at module-router level — D1 must not flag these as missing inquiry ownership.
- **`runway-accounts` routes** (billing/org provisioning): org-scoped, not session-scoped. Exempt by construction — they sit on a different router merge and never see the `SessionOwnershipLayer`.
- **Stripe webhook** (`runway_accounts::public_routes`): public, HMAC-verified, no auth context, no session — never session-leased.

The exempt list is **explicit and declared** so D1's manifest verifier cross-checks a known set rather than hardcoding Quorum paths.

### Manifest enforcement seam (for D1)

`runway.app.json` gains a parallel field:

```json
"ownership_exempt_routes": [
  { "method": "POST", "path": "/inquiry" },
  { "method": "POST", "path": "/inquiry/contracted" },
  { "method": "POST", "path": "/api/session/start" },
  { "method": "POST", "path": "/sensemap/recompute" },
  { "method": "POST", "path": "/sensemap/anticipatory-signals/detect" },
  { "method": "POST", "path": "/inquiry/intent/compile" },
  { "method": "POST", "path": "/inquiry/intent/compile/stream" },
  { "method": "POST", "path": "/acquisition/unresolved-questions/originate" },
  { "method": "POST", "path": "/acquisition/unresolved-questions/{id}/signal" }
]
```

D1 (the strict manifest verifier) enforces, **keyed off session-shaped mutating `domain_routes`** (not universal manifest presence): for every `RouteRegistration { owner: AppDomain, method ∈ {POST, PUT, PATCH, DELETE} }` whose template contains a path parameter that could plausibly be a session/aggregate id, the route is either (a) declared in `ownership_exempt_routes`, or (b) sits under a `Router` group that has `SessionOwnershipLayer` applied. Drift in either direction fails the build. Routes with no session-shaped path parameter (e.g. `POST /sensemap/recompute`) are NOT subject to this check and don't need to appear in the exempt list. Apps that ship zero session-shaped mutating routes (e.g. `api-server`) need not populate the field at all. The field is added to `AppExecutionPacket` as part of D5 (small additive schema change with `#[serde(default)]`); D1's enforcement is a D1 concern.

## 7. Backend implementation notes

### LocalLeaseStore (redb)

Single `WriteTransaction` per operation. One `leases` table keyed by serialized `(org_id, app_id, session_id)` tuple (concrete format: `format!("{}|{}|{}", org_id, app_id, session_id)`). Acquire logic inside the transaction:

1. Read existing record at key (if any).
2. Decide:
   - No record → insert `{ holder_id, expires_at }` → `Acquired`.
   - Record present and `expires_at <= now` → overwrite → `Acquired` (steal-after-expiry).
   - Record present and `holder_id == me` → extend `expires_at` → `Acquired` (idempotent re-acquire / renew through acquire).
   - Otherwise → return the existing record → `HeldByOther`.
3. Commit.

Renew: read; if `holder_id != me` return `Lost { current }`; else write new `expires_at`; commit.
Release: read; if `holder_id != me`, no-op (Ok); else delete; commit.

### RemoteLeaseStore (Firestore)

One document per scope at collection `_runway_leases`, document id `format!("{}|{}|{}", org_id, app_id, session_id)`. Acquire wraps a Firestore `runTransaction` with up to 3 retries on transient `ABORTED`. Same decision tree as redb. Renew and release likewise transactional.

Field shape:
```json
{
  "holder_id": "<string>",
  "expires_at": "<ISO-8601 timestamp>"
}
```

No `fence` field in v1 — reserved for D5.1.

## 8. Error and edge handling

| Scenario | v1 behavior |
|---|---|
| `try_acquire` returns `Ok(HeldByOther)` | 409 with body shape in section 5. |
| `try_acquire` returns `Err` (storage unreachable, transient Firestore error after retries) | 503 `lease_store_unavailable`. Distinct from 409 so client can retry vs. failover. |
| Background renew returns `Ok(Lost)` mid-handler | Task stops renewing. Logged at INFO with `holder_id`, `scope`, `current.holder_id`. Handler **completes normally** in v1 (documented stale-writer gap). The lost event is observable via logs and (in D5.1) via the `SessionLeaseLost` watch receiver. |
| Background renew returns `Err` (transient storage) | Retry on next interval; log at WARN; do not abort handler. After 3 consecutive errors, treat as lost (same as above) to bound staleness. |
| `release` on `Drop` returns `Err` | Log at WARN only. Lease will TTL-expire. Do not block response. |
| Path lookup yields no matching param (e.g. layer applied to wrong group) | 400 BAD_REQUEST `ownership_requires_session_id`. This is a configuration bug, not a runtime condition — surfaced at the first request to the misconfigured route. |
| AuthContext missing from extensions | 400 BAD_REQUEST `ownership_requires_auth`. Means the layer was applied without `AuthLayer` upstream; programmer error. |
| `org_id` claim absent on `AuthContext` | 400 BAD_REQUEST `ownership_requires_org`. Matches Quorum's existing org-required convention. |

### Long-handler safety margin

With defaults (TTL=60s, renew_interval=15s), a single missed renewal still leaves ~30s of headroom before steal — sufficient for Quorum's worst-case formation runs (5–30s) plus Firestore transaction retries. Tuning: per route-group `ttl()` / `renew_interval()` overrides are public on the builder.

## 9. Testing

### Contract suite (`runway-storage-contract::lease::run_lease_suite`)

Runs against both `LocalLeaseStore` (redb) and `RemoteLeaseStore` (Firestore emulator), using the existing `contract_test!` harness. Test cases:

1. `try_acquire_on_empty_returns_acquired`
2. `try_acquire_by_same_holder_is_idempotent_and_extends_expiry`
3. `try_acquire_by_other_holder_returns_held_by_other`
4. `try_acquire_after_expiry_steals_and_returns_acquired`
5. `renew_by_holder_advances_expires_at`
6. `renew_by_non_holder_returns_lost_with_current`
7. `renew_after_expiry_returns_lost`
8. `release_by_holder_clears_record`
9. `release_by_non_holder_is_noop`
10. `release_of_absent_record_is_noop`
11. `current_round_trips_record`
12. `current_on_absent_returns_none`

### `runway-app-host` integration test (`tests/ownership_test.rs`)

Two variants of the same scenario, tiered by where they run:

**Variant 1 — shared redb (every PR, CI-fast, default `cargo test`):**
- Two `RunwayAppHost` instances sharing a single redb-backed `LeaseStore` handle (single-process fixture; redb takes an exclusive file lock, so a single process cannot open the same file twice — the shared `Arc<dyn LeaseStore>` is the faithful in-process realization of cross-instance shared storage). Each instance is given a distinct `holder_id` via the layer builder so contention is observable in one process.
- Both expose `POST /inquiry/{id}/signal` with `SessionOwnershipLayer::for_app("test").path_param("id")` applied.
- **Contention is tested with an overlapping handler lifetime, not sequential requests** (see the §1 lifetime clarification): instance-A's handler is held open (holding the lease) while instance-B attempts the same scope. Asserts: while A is in-flight, B → 409 with body containing `holder_expires_at` and `session_id`; after A's response completes (lease released), B's retry → 200. A separate steal subtest holds A stuck past the TTL with renewal disabled (so the record expires without being released) and asserts B → 200 (steal of the expired record). Sequential non-overlapping requests are **not** asserted to 409 — by design they may both succeed.

**Variant 2 — Firestore emulator (D5 ship gate, env-gated locally):**
- Same scenario, using `RemoteLeaseStore` against a local Firestore emulator.
- Same assertions plus: Firestore transaction retry behavior under simulated `ABORTED` returns.
- Runs when `FIRESTORE_EMULATOR_HOST` is set. Skipped silently when the env is absent (default local `cargo test` flow).
- Exposed as `just test-lease-firestore` recipe that starts the emulator and runs Variant 2 + the remote contract suite.

**30-second renewal-under-load:**
- Instance A holds the lease continuously for the window via a single long-running handler (held open over mocked time) — under release-on-drop semantics this is how the lease stays held across the whole 30s, since short fire-and-forget requests would leave gaps. The background renew task keeps the record alive.
- Instance B's parallel attempts return 409 every time across the 30-second window.
- The holder's `expires_at` advances across renew intervals (renewal ran).
- Uses `tokio::time::pause`/`advance` to drive the renew schedule in <1 real second. Note: `expires_at` is computed from `chrono::Utc::now()` (wall clock), not the tokio clock, so advancing mocked time fires the renew *timer* while expiry math stays on real time; TTL-steal subtests therefore use a short real-time TTL rather than mocked advance.

### Where each test runs

| Test | Every PR / merge | D5 ship gate (RR CI release) | Default local `cargo test` |
|---|---|---|---|
| `run_lease_suite` against `LocalLeaseStore` | ✓ | ✓ | ✓ |
| `run_lease_suite` against `RemoteLeaseStore` (emulator) | env-gated | ✓ **required** | env-gated |
| `ownership_test.rs` Variant 1 (shared redb) | ✓ | ✓ | ✓ |
| `ownership_test.rs` Variant 2 (emulator) | env-gated | ✓ **required** | env-gated |
| 30s renewal-under-load (mocked clock) | ✓ | ✓ | ✓ |

The env-gating is infrastructure availability, **not** a Rule-7 softening switch: the test code path runs identical assertions whether on emulator or against production Firestore in the release pipeline. The switch is whether the emulator is running, not whether the check is enforced.

### No-flake guarantee

All timing-sensitive tests use `tokio::time::pause`/`advance` so the 60s TTL doesn't translate to a 60s wall-clock test. The Firestore-emulator variant is the one exception — emulator-bound `tokio::time::sleep` cannot be mocked.

## 10. Configurability defaults

| Knob | Default | Rationale |
|---|---|---|
| `ttl` | 60s | Covers Quorum's 5–30s formation runs with margin; fast enough for Cloud Run cold-failover. |
| `renew_interval` | 15s | TTL/4. One missed renewal still leaves 30s headroom. |
| `path_param` name | `"id"` | Quorum's predominant convention. Override per group when route uses `"inquiry_id"` etc. |
| Firestore transaction retries | 3 | Standard for transient `ABORTED`. |
| Renew failure-tolerance before treat-as-lost | 3 consecutive | Bounds staleness without thrashing on transient blips. |

All defaults are panel-locked sane values, not feature flags. Apps tune per route group when domain shape demands; the layer never silently changes behavior based on env.

## 11. Observability

- `INFO ownership_acquired scope={...} holder_id={...}` on every `Acquired`.
- `INFO ownership_held_by_other scope={...} our_holder_id={...} current_holder_id={...} expires_at={...}` on every `HeldByOther`.
- `INFO ownership_lost scope={...} our_holder_id={...} current_holder_id={...}` on every `Lost` renewal outcome.
- `WARN ownership_release_failed scope={...} holder_id={...} err={...}` on `Drop`-time release errors.
- `WARN ownership_renew_transient_error scope={...} holder_id={...} attempt={n} err={...}` on transient renew failures.

No metrics-emission infrastructure added in v1; metrics piggyback on existing `runway-telemetry` log scraping. Dedicated counters reserved for D5.1.

## 12. Deferred to D5.1

When `--max-instances=1` lifts and real two-instance load arrives, open `D5.1 — lease fencing`:

- `LeasedDocumentStore` wrapper that performs CAS-against-current-holder on every write (option 3 from the brainstorming round — handlers stay unchanged, +1 Firestore RTT per mutating write).
- `SessionLeaseLost: watch::Receiver<()>` inserted into request extensions, letting handlers opt into graceful-abort on renewal loss.
- Counters: lease_acquired_total, lease_held_by_other_total, lease_lost_total per `(app_id, route_group)`.

D5.1 is a follow-up RR ticket, not a v1 blocker. v1 ships when section 14's acceptance gate passes — the per-tier matrix in Section 9 governs which tests must run where (every PR vs. ship gate).

## 13. References

- Originating panel finding: `REVIEW_quorum-sense_2026-06-15.md` HELMS F5; RR Round-2 accept.
- Implementor row: `HANDOFF_quorum-sense_2026-06-15.md` Section 2, `D5` row.
- RR backlog: `runtime-runway/QUALITY_BACKLOG.md` → `D5`.
- Marquee App Contract Rule 6 (no multi-writer scale until D5 + CR-03 + CR-08 ship): `BOUNDARY_REGISTRY.md`.
- Marquee App Contract Rule 7 (no softening switches): `BOUNDARY_REGISTRY.md`.
- Reference middleware shape: `runtime-runway/crates/runway-auth/src/middleware.rs:88` (`AuthLayer`).
- Reference trait shape: `runtime-runway/crates/runway-storage/src/traits/event.rs:43,52` (`EventLog` / `SyncableEventLog`).
- Reference contract harness: `runtime-runway/crates/runway-storage-contract/src/document.rs` (`run_document_suite`).
- Consumer ownership-trait stub: `marquee-apps/quorum-sense/crates/quorum-app/src/scale_seams.rs:176-182` (`SessionOwnership` app-side trait, retained for app-internal use).

## 14. Acceptance criteria (mirrors `HANDOFF` Section 5 ship-signal)

D5 ships when ALL of the following pass. Tier-1 items must pass on every PR; tier-2 items are the additional gates the RR CI release job must run before publishing D5 done.

**Tier 1 — every PR / merge:**

- [ ] `LeaseStore` trait + types exported from `runway-storage`.
- [ ] `SessionOwnershipLayer::for_app(...)` + `.path_param(...)` + `.ttl(...)` + `.renew_interval(...)` exported from `runway-app-host`.
- [ ] `LocalLeaseStore` passes the `run_lease_suite` contract harness.
- [ ] `runway-app-host/tests/ownership_test.rs` Variant-1 (shared-redb two-host) passes: 200 → 409 with `holder_expires_at` body → 200 after TTL elapses (mocked clock).
- [ ] 30-second renewal-under-load test passes: `expires_at` strictly advances; B's parallel attempts return 409 throughout.
- [ ] `AppExecutionPacket::ownership_exempt_routes` field exists in the type with `#[serde(default)]` — apps without session-shaped mutating routes need not populate or even mention it. Apps that adopt `SessionOwnershipLayer` populate the field in the same consumer-wiring PR, asserted by D1's session-shaped-routes check (not by this acceptance criterion). The empty list is **not** required for app-onboarding parity.
- [ ] `just lint` clean (no `unsafe`, no feature flags, no softening switches).

**Tier 2 — D5 ship gate (RR CI release job, required before D5 is marked done in the HANDOFF):**

- [ ] `RemoteLeaseStore` passes `run_lease_suite` against the Firestore emulator (`just test-lease-firestore`).
- [ ] `ownership_test.rs` Variant-2 (Firestore emulator) passes.
- [ ] Standards promoted to `runtime-runway/kb/05-engineering/standards/`:
  - [ ] `RP-NO-FEATURE-FLAG-SOFTENING` (general; promoted with D5 as the first finding to ship it).
  - [ ] **`RP-NO-LEASE-WITHOUT-FENCING-V1`** (ships with D5, NOT D5.1). Documents the admission-time-only correctness gap so future code reviews reject any claim that D5 makes us multi-writer safe. Verbatim text the standard must include: *"Admission-time lease (D5) serializes new mutating requests across healthy instances. It does not prevent stale-writer writes after TTL steal. Write-side fencing is D5.1."* Cross-referenced from `marquee-apps/quorum-sense/deploy/cloud-run-provision.sh` (the `--max-instances=1` pin comment) when the pin is eventually lifted.

### Why `RP-NO-LEASE-WITHOUT-FENCING-V1` ships with D5

D5 is exactly when an engineer will be tempted to lift `--max-instances=1`. The standard exists to prevent that lift from being justified by "D5 shipped." Promoting it with D5.1 instead of D5 would leave a window where the deploy pin can come off before the review guardrail exists. The standard predates the temptation.

— end —
