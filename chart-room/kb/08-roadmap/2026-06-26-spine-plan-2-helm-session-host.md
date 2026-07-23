# Session Intelligence Spine — Plan 2: `helm-session-host`

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add `bedrock-platform/helms/crates/helm-session-host` — the server-side Session Helm that receives promoted `CoordinatorFinding`s, manages delivery state and the HITL gate lifecycle, and emits `SessionPush` over SSE to target participants. Built on **strengthened `runway-app-host`** (hub sequence stamping + `sse::event_stream`); **no duplicate transport**.

**Architecture:** Wire types live in `helm-session-contracts` (Plan 1). Client-side routing/projection lives in `helm-client` + `director-contracts`. `helm-session-host` is the **server mount surface**: a `HelmModule` (`helm.session-host`) that publishes session-scoped events on the shared `EventHubHandle` and exposes participant SSE streams filtered by `session_id`. It **does not** re-implement coordination's operator ledger or governed-jobs' truth runner — it composes alongside them on one hub when an app needs both operator coordination and decision-session pushes.

**Prerequisites (done):** Plan 1 (`helm-session-contracts`, `director-contracts`, `helm-client`); upstream SSE/sequencing consolidation (`runway-app-host` + helm migration, `QF-2026-06-26-01`, commit `b9aac85`).

**Spec references:**
- `KB/04-architecture/2026-06-26-session-intelligence-spine-design.md`
- `KB/04-architecture/decisions/2026-06-26-helm-session-host-vs-coordination.md` (Option 5)
- Plan 1: `KB/08-roadmap/2026-06-26-spine-plan-1-contracts-helm-client.md`

**Branch:** `feat/helm-session-host-slice-2` @ `c0df546` (helms); depends on `runtime-runway@fdc563c` (`EventHubHandle::publish` → `u64`); Quorum consumer on `quorum-sense@2af8d2b` (`feat/plan2-director-live-projection`).

**Status (2026-06-29):** Slice 2 **merged** to quorum-sense `main@371c728` (hotfix #8 restored D1 `wire_domain_routes` dropped by #7). Merge train complete on all three repos.

**Hotfix #8 (2026-06-29):** PR #7 regressed boot — inline `QuorumDomainModule` router bypassed D1 registration. Fix: restore `domain_host::wire_domain_routes`, director on D1 registry + `SessionHostService` Extension, empty module router; also restored PR #5 boot (`mount_live_modules`, ambient, helm feed).

## Slice 2 — Live state + Quorum director projection (done, pending merge)

**Goal:** Session push/gate updates in-memory mirror; Quorum `GET /api/director/snapshot` projects live `{ version, frame }` from hub sequence while preserving M3A.12 fixture fallback.

| Repo | Branch | Commit | What |
|------|--------|--------|------|
| runtime-runway | `feat/hub-publish-returns-sequence` | `fdc563c` | `EventHubHandle::publish()` returns assigned sequence |
| helms | `feat/helm-session-host-slice-2` | `c0df546` | `store.rs`, `presenter.rs`, `service.rs`, `events.rs` |
| quorum-sense | `feat/plan2-director-live-projection` | `2af8d2b` | Shared hub + `SessionHostModule`; `resolve_director_snapshot()` |

**Behavior:**
- Cold server → committed fixture (version **1844**); mobile M3A.12 contract unchanged.
- After `SessionHostService::publish_push` / `publish_gate` → live projection; `version` = hub sequence.
- Optional `?session_id=`; else most recently active session.

**Tests (green locally):** `helm-session-host` push/gate → snapshot; `quorum-server/director` fixture + live + fallback.

**Merge order:** runtime-runway PR first (path dep), then helms, then quorum-sense.

## Global Constraints

- **No duplicate transport.** Use `runway_app_host::{EventHubHandle, EventCursor, EventSubscription}` and `runway_app_host::sse::event_stream` only.
- **Wire boundary:** import `SessionPush`, `SessionContext`, `CoordinatorFinding`, `GatedDecision`, … from `helm-session-contracts` — do not fork types.
- **No Converge deps** in `helm-session-host` for slice 1–3 (routing/transport only). Converge/Organism wiring arrives with Quorum server extensions (Session 1 / marquee-apps).
- Copyright header on every new `.rs` file.
- Each task ends with: `cd bedrock-platform/helms && cargo test -p helm-session-host --locked && cargo clippy -p helm-session-host --all-targets --no-deps -- -D warnings`

## File Map

```
bedrock-platform/helms/
  Cargo.toml                           MODIFY — add workspace member

  crates/helm-session-host/            CREATE
    Cargo.toml
    src/
      lib.rs
      types.rs                         DecisionSessionId, SessionHostState
      events.rs                        session.push event type + publish helper
      service.rs                       SessionHostService (hub-facing core)
      http.rs                          GET /v1/sessions/{session_id}/stream
      module.rs                        SessionHostModule (HelmModule + readiness)
      host.rs                          mount_session_host helper
    tests/
      host_mount_test.rs               RunwayAppHost mount + SSE reachable
      characterization.rs              (slice 2+) sequence/filter pins
```

## Routes (full plan — slice 1 implements stream only)

| Method | Path | Slice | Purpose |
|--------|------|-------|---------|
| GET | `/v1/sessions/{session_id}/stream` | **1** | SSE: `session.push` (+ later gate events) for one decision session |
| POST | `/v1/sessions` | 2 | Open decision session (membership) |
| POST | `/v1/sessions/{session_id}/findings` | 3 | Ingest promoted `CoordinatorFinding` → route → `SessionPush` |
| POST | `/v1/sessions/{session_id}/gates/{gate_id}/decision` | 4 | Deliver gate verdict → resume hook (Quorum-owned engine) |

## Event vocabulary (on shared hub)

| `EventEnvelope.type` | Payload | Consumer |
|----------------------|---------|----------|
| `session.push` | serialized `SessionPush` | Client Helm SSE (`helm-client`) |
| `session.gate.opened` | `GatedDecision` + context | slice 4 |
| `session.gate.resolved` | gate id + verdict | slice 4 |

Filter key: `payload.session_context.session_id` (matches `SessionPush` wire shape).

---

### Task 1: Types + mount test (this slice)

**Files:** crate scaffold, `types`, `events`, `service`, `http`, `module`, `host`, `host_mount_test.rs`

**Produces:**
- `SessionHostService::publish_push(&SessionPush)` → hub event `session.push`
- `SessionHostModule` (`module_id = "helm.session-host"`) mounted on `RunwayAppHost`
- `GET /v1/sessions/{session_id}/stream` using upstream `sse::event_stream`

- [x] **Step 1:** Create crate + workspace member
- [x] **Step 2:** Implement types + publish + SSE route (no hand-rolled SSE loop)
- [x] **Step 3:** `host_mount_test` — host builds, stream returns `200` + `text/event-stream`
- [x] **Step 4:** Unit test — `publish_push` + `stream_includes` filter by `session_id`
- [x] **Step 5:** `cargo test -p helm-session-host` + clippy

**Commit:** `feat(helms): helm-session-host slice 2 — live store and director projection` @ `c0df546`

---

### Task 2a: Live store + director projection (slice 2 — done, pending merge)

**Files:** `store.rs`, `presenter.rs`, `service.rs`, `events.rs`; quorum `director.rs` + `main.rs` mount.

**Produces:**
- Per-session in-memory `ClientHelm` mirror; hub sequence as `version`
- `QuorumDomainPresenter` implements `DomainPresenter`
- `SessionHostService::publish_push` / `publish_gate` → hub + store update; `quorum_director_snapshot(session_id)`
- Quorum `GET /api/director/snapshot` → `resolve_director_snapshot()` (live or fixture v1844)

- [x] **Step 1:** `EventHubHandle::publish` returns sequence (`runtime-runway@fdc563c`)
- [x] **Step 2:** Store + presenter + service projection path
- [x] **Step 3:** Quorum mounts shared hub + `SessionHostModule`; `Extension<SessionHostService>`
- [x] **Step 4:** Director tests — fixture contract, live vs fixture, fallback
- [x] **Step 5:** Branches pushed (see status table above)

**Commit:** helms `c0df546`, quorum-sense `2af8d2b`

---

### Task 2: Session membership (deferred)

Heartbeat-leased decision-session registry (distinct from `helm-coordination` operator sessions). Routes: `POST /v1/sessions`, heartbeat, close.

---

### Task 3: Finding ingestion + routing (deferred)

Accept `CoordinatorFinding<P>` (JSON), derive target participants, call `publish_push` per target (or fan-out envelope). **No payload inspection beyond routing fields.**

---

### Task 4: Gate lifecycle (deferred)

Surface `GatedDecision` on SSE; accept gate verdict POST; hand off to Quorum's Converge resume path (Session 1 owns engine wiring).

---

### Task 5: Quorum server mount (slice 2 — done on branch, pending merge)

Mount `SessionHostModule` beside coordination/jobs on quorum-server's shared hub. **Implemented** on `quorum-sense@2af8d2b` — see slice 2 status table.

## Handoff reference (quorum-server — implemented on branch)

Branch `feat/plan2-director-live-projection` @ `2af8d2b`. Merge after helms slice 2.


**Mount pattern:**

```rust
use helm_session_host::{mount_session_host, SessionHostModule};
use runway_app_host::{EventHub, RunwayAppHost};

let hub = EventHub::with_capacity(512);
let session_host = mount_session_host(hub.handle(), "quorum.sense");
// share hub with coordination/jobs when co-mounted

RunwayAppHost::builder(packet)
    .mount(session_host)
    .build()
    .await?;
```

**Example `SessionPush` shape** (from `helm-session-contracts`):

```json
{
  "finding_id": "550e8400-e29b-41d4-a716-446655440000",
  "urgency_intent": "disruptive",
  "payload": {"msg": "contradiction detected"},
  "session_context": {
    "session_id": "sess-quorum-1",
    "phase": "hypothesis",
    "cycle": 3,
    "timestamp_ms": 1700000000000
  }
}
```

**SSE:** `GET /v1/sessions/{session_id}/stream` — replays then live; filter matches `session_context.session_id` in envelope payload.

**Do not duplicate:** sequence counters or SSE loops in quorum-server; publish via `SessionHostService::publish_push` or shared hub only.

---

## Validation (full plan)

- Slice 1: mount test + unit filter test green
- Slice 3+: two-participant push routing integration test
- Before Quorum wiring: characterization test pins monotonic sequence on shared hub (mirror `helm-coordination/tests/characterization.rs`)

**Tier:** Approval-path / session push surface → human review before production mount (Autonomy Contract Tier 2).
