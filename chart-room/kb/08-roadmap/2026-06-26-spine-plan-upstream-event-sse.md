# Session Intelligence Spine — Upstream Event/SSE Consolidation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

> **Supersedes** the retired `2026-06-26-spine-stem-plan-helm-session-core.md`.
> Per ADR `decisions/2026-06-26-helm-session-host-vs-coordination` **Option 5**,
> there is **no `helm-session-core` crate in this pass**. The duplication that
> motivated a stem is removed by *strengthening the upstream we already own*
> (`runway-app-host`) and migrating the two helm consumers onto it.

**Goal:** Move the two genuinely-generic event primitives — monotonic sequence
stamping and the SSE replay+live loop — into `runway-app-host` (which already owns
the event hub and half-implements both), then delete the duplicated copies in
`helm-coordination` and `helm-governed-jobs`. **No new crate. No behavior change.**

**Why this shape (the holistic boundary):** sequencing and SSE framing are the
*event-transport* concept; their home is `runway-app-host`. They leaked into the
two helm crates only because the in-memory hub path was left half-built
(`EventHub::with_capacity` sets `next_sequence: None`, `realtime.rs:156`). The HITL
**gate** concept (`GateDecision`, the waiter registry/`signal_gate`) is *not*
transport — it stays in `helm-governed-jobs` this pass and earns its own clean home
only when `helm-session-host` becomes a real third consumer (ADR §Concept layering).

**Scope boundary — what this pass does NOT touch:**
- No `helm-session-core` crate.
- No move of `GateDecision` / `GateWaiterRegistry` / `signal_gate` — they stay in
  `helm-governed-jobs`. `helm-coordination` keeps consuming them across the existing
  edge (it still needs `JobStreamState` for `signal_gate`, `service.rs:251`).
- **No `GateDecisionKind` ↔ `GateDecision` unification.** `GateDecisionKind`
  (kebab-case HTTP wire vocabulary, `ledger.rs:25`) and `GateDecision` (internal
  signal vocabulary) are a healthy wire/domain split; leave the `to_gate_decision()`
  bridge intact.
- No `SessionRegistry` / `PresenceRegistry` / `DecisionLedger` changes.

## Architecture & cross-workspace note

This plan spans **two Cargo workspaces / git roots**, linked by an existing path
dependency (`helm-governed-jobs/Cargo.toml:21` →
`../../../../runtime-runway/crates/runway-app-host`):

- `runtime-runway/` — owns `runway-app-host` (Tasks 1–3). Build/test here.
- `bedrock-platform/helms/` — owns `helm-coordination` + `helm-governed-jobs`
  (Tasks 4–5). Build/test here; picks up the upstream change via the path dep.

Commits therefore land in two repos. The KB/ADR/ledger docs live in the **root**
repo (a third git root). Keep the three commit sets separate; cite
`QF-2026-06-26-01` in each.

**Tech Stack:** Rust (runtime-runway + helms workspace toolchains). `runway-app-host`
already deps `axum`, `futures`, `tokio`, `tokio-stream`, `serde`, `chrono`, `uuid`.
The SSE combinator adds `async-stream` to the `runtime-runway` workspace if absent.

## Global Constraints

- **No new crate.** The only structural change is to `runway-app-host`'s public API
  (additive) and the internal wiring of the two helm crates.
- **Behavior-preserving sequence values.** Today the helm in-memory path stamps
  `1, 2, 3, …` (counter `AtomicU64::new(1)` + `fetch_add` with no `+1`,
  `job_stream.rs:145,248`; `service.rs:48`, `events.rs:92`). The hub durable path
  stamps the same values (`AtomicU64::new(high_water=0)` + `fetch_add(1) + 1`,
  `realtime.rs:190,220`). After migration the in-memory hub uses the **same**
  `fetch_add(1)+1`-from-`0` logic → first event is still `1`. Verify this with the
  existing helm SSE/sequence assertions.
- **Same-hub invariant.** The fix is correct only because coordination and jobs
  publish to the **same** hub when wired live (`with_job_state` does
  `self.hub = job_state.hub.clone()`, `service.rs:87`). Preserve that adoption; just
  drop the now-redundant `next_sequence` sharing.
- **Characterization tests first** (Task 1), green and unedited through Tasks 2–6.
- **Incremental.** Each task compiles and passes its workspace's `cargo test`.
- **Public-API of both helm crates unchanged** — verified by diff in Task 6.
- Copyright header on every new `.rs` file:
  `// Copyright 2024-2026 Reflective Labs` / `// SPDX-License-Identifier: MIT`.

---

## File Map

```
runtime-runway/crates/runway-app-host/
  Cargo.toml                    MODIFY — + async-stream (if not already a workspace dep)
  src/realtime.rs               MODIFY — in-memory hub owns its sequence counter
  src/sse.rs                    MODIFY — add replay+live event_stream combinator
  tests/ (or inline)            CREATE — characterization for hub sequencing + combinator

bedrock-platform/helms/crates/helm-governed-jobs/
  src/job_stream.rs             MODIFY — drop next_sequence + pre-stamp; delegate SSE
  tests/characterization.rs     CREATE (Task 1)

bedrock-platform/helms/crates/helm-coordination/
  src/service.rs                MODIFY — drop next_sequence sharing in new/with_job_state
  src/events.rs                 MODIFY — CoordinationPublisher drops its seq counter
  src/http.rs                   MODIFY — build_stream delegates to upstream combinator
  tests/characterization.rs     CREATE (Task 1)
```

---

### Task 1: Characterization tests — pin the observable contract

The contract that must survive is **at the helm boundary**: events emerge with
monotonic sequences starting at 1, and the SSE streams frame/replay/terminate
exactly as today. (The `runway-app-host` in-memory `publish` contract *does* change
— that is the point — so it is pinned with a *new* test in Task 2, not here.)

**Files:**
- Create: `bedrock-platform/helms/crates/helm-governed-jobs/tests/characterization.rs`
- Create: `bedrock-platform/helms/crates/helm-coordination/tests/characterization.rs`

- [ ] **Step 1: Pin governed-jobs sequencing + SSE behavior.** Assert that a fresh
  `JobStreamState::default()` publishing N events yields sequences `1..=N` on a
  cursor subscription, and that `build_run_sse_stream` replays then terminates on a
  terminal event. (Reuse the shapes already asserted in `gate_test.rs`; if those
  already cover sequence + terminal framing, cite them instead of duplicating and
  skip this file.)
- [ ] **Step 2: Pin coordination sequencing.** With `CoordinationService::new(hub,…)`
  and again with `.with_job_state(job_state)`, assert emitted events carry monotonic
  sequences and (live mode) interleave with job events on one globally-ordered
  stream. (Reuse `coordination_test.rs` assertions if they already cover this.)
- [ ] **Step 3: Run — expect green** in the helms workspace:

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo test -p helm-governed-jobs --locked
cargo test -p helm-coordination --locked
```

- [ ] **Step 4: Snapshot helm public APIs (baseline for Task 6):**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo public-api -p helm-coordination > /tmp/helm-coordination.api.before.txt
cargo public-api -p helm-governed-jobs > /tmp/helm-governed-jobs.api.before.txt
# Fallback if cargo-public-api is unavailable:
rg -n '^\s*pub (use|fn|struct|enum|trait|const|type|mod) ' \
   crates/helm-coordination/src/lib.rs crates/helm-governed-jobs/src/lib.rs \
   > /tmp/helm-stem.exports.before.txt
```

- [ ] **Step 5: Commit** (helms repo).

---

### Task 2 (upstream): in-memory hub owns its sequence counter

The durable hub already auto-stamps (`realtime.rs:190,218-222`); the in-memory
constructors just opt out (`realtime.rs:156`). Make them own a counter too, so
`EventHubHandle::publish` stamps for **every** hub.

**Files:** `runtime-runway/crates/runway-app-host/src/realtime.rs`

- [ ] **Step 1: Initialize the counter in the in-memory constructors.** In
  `EventHub::with_capacity` (`realtime.rs:150-159`) set
  `next_sequence: Some(Arc::new(AtomicU64::new(0)))` (matching the durable path's
  `fetch_add(1)+1` convention so the first event is `1`). `new()` delegates to
  `with_capacity`, so it inherits the change.
- [ ] **Step 2: Add a characterization test for the new contract** (in `hub_tests`):

```rust
#[tokio::test]
async fn in_memory_hub_owns_and_stamps_sequence() {
    let hub = EventHub::with_capacity(8);
    let h = hub.handle();
    let mut rx = h.subscribe();
    // Caller supplies sequence 0; the hub overwrites with its own counter.
    h.publish(sample(0, "a"));
    h.publish(sample(0, "b"));
    assert_eq!(rx.recv().await.unwrap().sequence, 1);
    assert_eq!(rx.recv().await.unwrap().sequence, 2);
}
```

- [ ] **Step 3: Reconcile existing `hub_tests`.** The current tests publish in
  ascending order starting at 1, so auto-stamping yields the same values and they
  pass unchanged. Any test that asserts a *caller-supplied, non-sequential* sequence
  is now a **contract update** (the hub owns sequencing): adjust its expectation and
  note it in the commit body. Audit `approvals.rs:108` (`EventHub::new()`) — if it
  relies on caller sequences, treat likewise; if it only publishes, no change.
- [ ] **Step 4: Run — expect green** in runtime-runway:

```bash
cd /Users/kpernyer/dev/reflective/runtime-runway
cargo test -p runway-app-host --locked
```

- [ ] **Step 5: Commit** (runtime-runway repo). Classify any adjusted hub test as
  *Contract update* per the test/code-attribution policy.

---

### Task 3 (upstream): SSE replay+live combinator in `runway-app-host::sse`

The existing `sse::stream` (`sse.rs:20-32`) is live-only. Add a generic replay+live
combinator over the existing `EventSubscription`, so the two helm crates stop
hand-rolling the same loop.

**Files:** `runtime-runway/crates/runway-app-host/src/sse.rs`, `Cargo.toml`

- [ ] **Step 1: Ensure `async-stream` is available.** If `runtime-runway`'s workspace
  `Cargo.toml` doesn't already list `async-stream`, add it there and reference it as
  `async-stream.workspace = true` in `runway-app-host/Cargo.toml`. (It is a small,
  standard crate already used by the helm crates.)
- [ ] **Step 2: Implement the combinator:**

```rust
use std::convert::Infallible;
use axum::response::sse::Event;
use tokio::sync::broadcast;
use crate::realtime::{EventEnvelope, EventSubscription};

/// Encode an envelope as an SSE frame (id = sequence, data = JSON). `None` if it
/// cannot be serialized (the prior behavior at both helm call sites).
#[must_use]
pub fn encode_frame(env: &EventEnvelope) -> Option<Event> {
    serde_json::to_string(env)
        .ok()
        .map(|data| Event::default().id(env.sequence.to_string()).data(data))
}

/// Replay the subscription's buffer, then stream live events — deduped by
/// sequence, tolerant of lag. `filter` selects which envelopes are yielded;
/// `terminal` (checked only on yielded envelopes) decides when to stop.
pub fn event_stream<F, T>(
    subscription: EventSubscription,
    filter: F,
    terminal: T,
) -> impl tokio_stream::Stream<Item = Result<Event, Infallible>>
where
    F: Fn(&EventEnvelope) -> bool,
    T: Fn(&EventEnvelope) -> bool,
{
    async_stream::stream! {
        let mut last_sequence = 0u64;
        for env in subscription.replay {
            last_sequence = env.sequence;
            if filter(&env) {
                let stop = terminal(&env);
                if let Some(frame) = encode_frame(&env) { yield Ok(frame); }
                if stop { return; }
            }
        }
        let mut live = subscription.receiver;
        loop {
            match live.recv().await {
                Ok(env) => {
                    if env.sequence <= last_sequence { continue; }
                    last_sequence = env.sequence;
                    if filter(&env) {
                        let stop = terminal(&env);
                        if let Some(frame) = encode_frame(&env) { yield Ok(frame); }
                        if stop { break; }
                    }
                }
                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                Err(broadcast::error::RecvError::Closed) => break,
            }
        }
    }
}
```

> **Behavior-equivalence note (the subtle part).** Jobs' replay is pre-filtered by
> `run_id` via the cursor, so its `filter` is always true on replay → `terminal` is
> effectively checked on every replay envelope, matching today's
> `build_run_sse_stream` (`job_stream.rs:804-811`). Coordination passes
> `terminal = |_| false` (its stream never self-terminates), matching `build_stream`
> (`http.rs:205-228`). This combinator reproduces both. Task 6's `gate_test.rs` /
> `coordination_test.rs` are the contract check.

- [ ] **Step 3: Unit-test the combinator** (replay filters + terminates; live dedup
  skips `<= last_sequence`). Export `event_stream` + `encode_frame` from the module.
- [ ] **Step 4: Run + Step 5: Commit** (runtime-runway repo).

---

### Task 4 (helms): migrate `helm-governed-jobs` onto upstream

**Files:** `bedrock-platform/helms/crates/helm-governed-jobs/src/job_stream.rs`

- [ ] **Step 1: Drop the counter.** Remove the `next_sequence` field
  (`job_stream.rs:120-124`) and its init in `new` (`:145`) and `Default` (`:221`).
- [ ] **Step 2: Stop pre-stamping in `Publisher`.** Remove the `seq` field
  (`:199`/struct) and the `fetch_add` line (`:248`); `emit` builds the envelope with
  `sequence: 0` and calls `self.hub.publish(env)` — the hub now stamps. The
  `publisher(...)` helper (`:190-205`) drops `seq: Arc::clone(&self.next_sequence)`.
  Remove now-unused `AtomicU64`/`Ordering` imports if nothing else uses them.
- [ ] **Step 3: Delegate the SSE loop.** Replace `build_run_sse_stream`
  (`job_stream.rs:796-832`) and its local `encode_frame` (`:838-842`) with:

```rust
fn build_run_sse_stream(
    subscription: EventSubscription,
    run_id: String,
) -> impl tokio_stream::Stream<Item = Result<Event, Infallible>> {
    runway_app_host::sse::event_stream(
        subscription,
        move |env: &EventEnvelope| env.run_id.as_deref() == Some(run_id.as_str()),
        is_terminal, // keep job_stream.rs:834-836
    )
}
```

  Drop now-unused `broadcast` / `async_stream` imports.
- [ ] **Step 4: Run — expect green** (helms workspace; `gate_test.rs` asserts the
  job SSE sequence + terminal) + **Commit** (helms repo). Classify as
  *Contract update* (tests unchanged; production delegates upstream).

---

### Task 5 (helms): migrate `helm-coordination` onto upstream

**Files:** `service.rs`, `events.rs`, `http.rs`

- [ ] **Step 1: Drop the standalone counter.** In `CoordinationService::new`
  (`service.rs:46-61`) remove `let seq = Arc::new(AtomicU64::new(1));` and build the
  publisher without it.
- [ ] **Step 2: Simplify `with_job_state`** (`service.rs:86-95`): keep
  `self.hub = job_state.hub.clone()` (the same-hub adoption is what makes sequencing
  monotonic) and rebuild the publisher from that hub — **drop**
  `job_state.next_sequence.clone()`. Remove the `AtomicU64` import if now unused.
- [ ] **Step 3: `CoordinationPublisher` drops its counter** (`events.rs:62-107`):
  remove the `seq` field and the `fetch_add` (`:92`); `emit` keeps its
  workspace/principal payload enrichment and calls `hub.publish(env)` with
  `sequence: 0`. `CoordinationPublisher::new` drops the `seq` parameter (update both
  call sites in `service.rs`).
- [ ] **Step 4: Delegate the SSE loop.** Replace `build_stream`
  (`http.rs:199-231`) + local `encode_frame` (`:238-242`) with a call to
  `runway_app_host::sse::event_stream(subscription, move |e| include(e, &workspace_id), |_| false)`.
  Keep `include` (`http.rs:233-236`); drop now-unused `broadcast`/`async_stream`.
- [ ] **Step 5: Run — expect green** (`coordination_test.rs` asserts coordination
  framing + the interleaved live stream) + **Commit** (helms repo).
  *Contract update.*

---

### Task 6: Verify no behavior change + unchanged helm public APIs

- [ ] **Step 1: Both workspaces green.**

```bash
cd /Users/kpernyer/dev/reflective/runtime-runway && cargo test -p runway-app-host --locked && cargo clippy -p runway-app-host -- -D warnings
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms && cargo test --workspace --locked && cargo clippy --workspace -- -D warnings
```

- [ ] **Step 2: Helm public-API diff — empty.**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo public-api -p helm-coordination  > /tmp/helm-coordination.api.after.txt
cargo public-api -p helm-governed-jobs > /tmp/helm-governed-jobs.api.after.txt
diff -u /tmp/helm-coordination.api.before.txt  /tmp/helm-coordination.api.after.txt
diff -u /tmp/helm-governed-jobs.api.before.txt /tmp/helm-governed-jobs.api.after.txt
```

  Expected: empty diffs. `CoordinationPublisher::new` losing its `seq` parameter is
  the one intended public change — if it is part of the public surface, record it in
  the commit body as the single accepted delta; otherwise the diff is empty.
- [ ] **Step 3: External apps build untouched** (they import only
  `GovernedJobsModule`, which never moved):

```bash
cd /Users/kpernyer/dev/reflective/marquee-apps/quorum-sense && cargo build -p quorum-server --locked
cd /Users/kpernyer/dev/reflective/marquee-apps/atlas-integration && cargo build -p atlas-server --locked
```

- [ ] **Step 4: Update the ledger.** Move `QF-2026-06-26-01` to Done with the
  closing commit SHAs (runtime-runway + helms), and note that gate/session-core
  extraction remains deferred to the `helm-session-host` step.

---

## Plan Complete

**Deliverable:** `runway-app-host` owns event sequencing (all hubs) and provides a
reusable SSE replay+live combinator; `helm-coordination` and `helm-governed-jobs`
consume them and drop their duplicated counters and SSE loops — **with zero new
crates and unchanged helm public APIs**.

**Explicitly deferred (ADR Option 5 + Concept layering):** the HITL gate concept
(`GateDecision`, the waiter registry, `signal_gate`) stays in `helm-governed-jobs`.
It earns its own clean home when `helm-session-host` lands as a real third consumer
and the `Engine::resume(GateDecision)` HITL evolution (spine §1a) is implemented —
the point at which `converge-core` legitimately enters that home.

## Execution Handoff

1. **Subagent-Driven (recommended)** — dispatch a fresh subagent per task, review
   between tasks (REQUIRED SUB-SKILL: superpowers:subagent-driven-development).
2. **Inline Execution** — execute with checkpoints (REQUIRED SUB-SKILL:
   superpowers:executing-plans).
