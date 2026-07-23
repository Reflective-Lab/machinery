# ADR: helm-session-host vs. existing Helms session machinery

- Date: 2026-06-26
- Status: **Accepted — revised same day** (the Option-4 "extract a `helm-session-core`
  stem now" choice is **superseded by Option 5**: strengthen the upstream we already
  own rather than wrap it. See the Decision section.)
- Decision type: architecture boundary
- Related design: `KB/04-architecture/2026-06-26-session-intelligence-spine-design.md` (§3, Repo Structure)
- Originating finding: `QF-2026-06-26-01` (D-tier; `QUALITY_BACKLOG.md`) — linked both ways
- Blocks: Session Intelligence Spine **Plan 2** (`helm-session-host` + Quorum server extensions)

## Question

The Session Intelligence Spine proposes a new server-side crate,
`helm-session-host`, that receives promoted `CoordinatorFinding` Facts, manages
delivery state and the HITL-gate lifecycle, and emits `SessionPush` via SSE.

Helms already ships two crates that overlap this surface:

- **`helm-coordination`** — `SessionRegistry` (heartbeat-leased), `PresenceRegistry`,
  `DecisionLedger` (optimistic gate ledger), and a coordination SSE stream
  (`/v1/coordination/stream`).
- **`helm-governed-jobs`** — a job-stream SSE (`/v1/jobs/{key}/stream`) with full
  HITL gate support (pre-gate execute → `gate.paused` → oneshot waiter → resume).

The Spine's own Platform Alignment Guard forbids standing up parallel
session/SSE/gate machinery. So: **does `helm-session-host` extend
`helm-coordination`, compose it, or supersede part of it — and is a "decision
session" the same concept as "multi-operator coordination"?**

## Context / evidence

- `helm-coordination/src/lib.rs:7-14` describes itself as "multi-operator
  coordination for Helm's headless surface" — `Sessions` = *who is connected,
  heartbeat-leased*; presence; attributed gate/job events on one workspace-scoped
  SSE stream.
- `helm-governed-jobs/src/lib.rs:5-13` owns the `/v1/jobs/{key}/stream` SSE route
  with HITL gate support (pause → waiter → resume).
- The Spine's HITL mechanism is settled (spine design §1a): a terminal suggestor
  proposes "criteria met" → `HitlPolicy` fires → `RunResult::HitlPause` →
  `Engine::resume(GateDecision)`. The gate *lifecycle* (pause → await human →
  resume) is structurally the same shape `helm-governed-jobs` already runs.
- **Semantic gap (the crux):** `helm-coordination`'s "session" is *operators of a
  workspace*. The Spine's "session" is *participants + AI in a long-running
  Converge decision loop*. They share primitives (membership, presence, SSE
  multiplex, gate lifecycle) but differ in purpose, lifetime, and membership
  semantics.

> Evidence depth caveat: the above is module-level evidence from a read-only
> survey, not a full API audit. A pre-implementation pass over the public APIs of
> both crates is a follow-up (see below) before committing to "extend" vs.
> "compose."

## Options considered

1. **Fold session-host responsibilities into `helm-coordination`.**
   Add finding routing, `SessionPush`, and the main-loop gate lifecycle to the
   existing crate.
   - Rejected (lead): conflates two genuinely different "session" concepts
     (operator coordination vs. decision deliberation) into one overloaded crate;
     couples the Spine's lifetime/membership model to operator semantics it does
     not want.

2. **New `helm-session-host` that _composes_ the existing primitives in place.**
   Distinct crate that reuses `helm-coordination`'s membership/presence/SSE and
   `helm-governed-jobs`' gate lifecycle without extracting a shared crate.
   - Not chosen: this was the conservative default, but it leaves the shared
     primitives duplicated/owned by `helm-coordination` while a third consumer
     reaches into them. With two real consumers already in tree (see Decision),
     deferring the extraction just accrues migration debt.

3. **New `helm-session-host` standalone (parallel SSE/session/gate).**
   - Rejected: directly violates the Alignment Guard; duplicates working
     infrastructure.

4. **Extract a shared `helm-session-core` stem** that `helm-coordination`,
   `helm-governed-jobs`, and `helm-session-host` all consume.
   - **Superseded by Option 5 (2026-06-26).** The API audit established that three
     of the four "shared primitives" (the sequence-stamping publisher, the SSE
     replay+live loop) are not new abstractions at all — they are duplicated
     mechanics that exist **only because the upstream `runway-app-host` hub we own
     leaves them half-built**. A new crate that wraps `EventHubHandle` and is then
     wrapped again by each consumer is two wrapper layers around code we own.
     Rejected on the "make upstream stronger; do not wrap what we own" principle.

5. **Strengthen the upstream we own (`runway-app-host`) + de-dupe within the
   existing dependency edge; defer any `helm-session-core` crate.**
   - **Chosen.** Evidence (audit + `runway-app-host/src/realtime.rs`,
     `src/sse.rs`): the hub *already* owns a monotonic sequence counter and
     auto-stamps on the durable `with_event_log` path (`realtime.rs:132,190,218-222`);
     the in-memory `with_capacity`/`new` path just sets `next_sequence: None`
     (`realtime.rs:156`), which is the **sole reason** both helm crates carry their
     own `Arc<AtomicU64>` and pre-stamp (`job_stream.rs:124,248`;
     `events.rs:92`; the `with_job_state` counter-sharing hack at `service.rs:90`).
     `runway-app-host` already depends on `axum` and ships an `sse.rs` module — the
     natural home for a replay+live SSE combinator. The gate-decision enum and the
     gate-wait rendezvous are HITL concepts (not transport) shared by exactly the
     two helm crates, and `helm-coordination` **already depends on**
     `helm-governed-jobs` — so they de-duplicate within that existing edge, no new
     crate. A `helm-session-core` crate is reconsidered only when `helm-session-host`
     lands and introduces session primitives (membership/presence/ledger,
     `Engine::resume`) that have **no upstream home** and ≥2 consumers.

## Decision

> **Revised 2026-06-26 (supersedes the Option-4 text below).** Under the
> "we own what we own — make upstream stronger, do not wrap" directive, **Option 5
> is chosen**: there is **no `helm-session-core` crate in the first pass**. The
> duplication that motivated a stem is removed by finishing the upstream we own and
> de-duplicating within the existing `helm-coordination → helm-governed-jobs` edge:
>
> 1. **`runway-app-host` (upstream) — finish the sequencing it half-owns.** Make the
>    in-memory hub (`with_capacity`/`new`) own its sequence counter exactly as the
>    durable path already does, so `EventHubHandle::publish` auto-stamps for *every*
>    hub. Both helm crates then **delete** their `next_sequence` field, their
>    `fetch_add` pre-stamping, and the `with_job_state` counter-sharing hack — one
>    hub, one counter, monotonic by construction. Blast radius is tiny and fully
>    in-tree (audit: only `job_stream.rs:212` is a production in-memory hub; the rest
>    are `runway-app-host`'s own tests + helm tests; no app consumes `EventHub`
>    directly). Gate behind characterization tests on `runway-app-host`.
> 2. **`runway-app-host::sse` (upstream) — add the replay+live combinator.** A
>    `stream(subscription, filter, terminal)` over the existing `EventSubscription`
>    type, returning `impl Stream<Item = Result<sse::Event, Infallible>>`. `axum` is
>    already a dep here; this is the natural home. `build_stream` /
>    `build_run_sse_stream` in the two helm crates collapse to calls into it with
>    their own predicates, then are deleted.
> 3. **`GateDecision` + gate-wait rendezvous — de-dupe within the existing edge.**
>    These are HITL primitives, not transport, so they do **not** go in
>    `runway-app-host`. `helm-coordination` already depends on `helm-governed-jobs`,
>    so the canonical `GateDecision` (kebab-case wire contract preserved) and the
>    gate-wait registry **stay in `helm-governed-jobs`** (where the registry already
>    lives) and `helm-coordination` consumes them (it already does, via
>    `signal_gate` / `to_gate_decision`). `GateDecisionKind` collapses into the one
>    enum. No new crate.
>
> Net: the first pass ships **zero new crates and zero wrapper layers** — it makes
> `runway-app-host` stronger (and thereby every future hub consumer) and removes two
> real code duplications. A `helm-session-core` crate is revisited only if/when
> `helm-session-host` introduces session-shaped primitives with no upstream home and
> ≥2 consumers. The semantic boundary conclusion below (decision sessions ≠ operator
> coordination; `helm-session-host` is its own crate) **still holds** — only the
> "extract a shared stem now" mechanism is superseded.

---

**[Superseded] Option 4 — extract a shared `helm-session-core` stem now.** The
load-bearing justification is **two real consumers already in tree** — enough to
satisfy the guard below (a primitive needs ≥2 consumers to belong in the core):

- `helm-coordination` — operator coordination (**in tree**)
- `helm-governed-jobs` — governed job streams with HITL gates (**in tree**)

The remaining consumers are **planned, not yet in tree**. They reinforce the
direction but are not what justifies the extraction today:

- `helm-session-host` — decision sessions for the Spine / Quorum (new, this design)
- the rest of the composed-decision suite — keystone, tally-escrow,
  plumb-execution, … are **KB-only today** (no Rust workspace yet); each stage is
  a governed session that will consume the stem once built
- Client Helm hybrid sync, which leans on the same membership/presence model

`helm-session-core` holds **only the genuinely shared primitives**. The
pre-extraction API audit (`2026-06-26-helm-session-core-api-audit.md`, follow-up 1)
sharpened this against the code — **the first pass moves only the four primitives
that both in-tree consumers use today**, all of them Converge-free:

- the **sequence-stamping SSE publisher** (duplicated in both crates)
- the **SSE replay+live stream builder** (duplicated in both crates)
- the **HITL gate-wait rendezvous** (`oneshot` waiter + `signal`/`take`/`restore`)
- a **unified `GateDecision`** vocabulary (preserving the kebab-case wire contract)

Two corrections to this ADR's first sketch, both evidence-backed by the audit:

- **The core is Converge-free (first pass).** The implemented gate primitive is a
  Converge-free `oneshot` rendezvous; the Converge coupling lives only in the
  truth-execution driver (`run_job_task`), which is a *consumer*. So the resume
  step — `Engine::resume(GateDecision)` — lives in the **consumer**
  (`helm-session-host`), mirroring how `run_job_task` (not the waiter) owns
  Converge today. The core does **not** depend on `converge-core` in the first
  pass; that arrives (if at all) with the third consumer.
- **Membership, presence, and the optimistic ledger are deferred.** Each has
  exactly one in-tree consumer today (`helm-coordination`), so the guard keeps
  them out of the first pass. They move with `helm-session-host` as the second
  consumer — *and* the optimistic first-writer-wins/idempotent/conflict ledger is
  **operator-coordination-specific** (racing operators), not obviously what a
  single-terminal-gate decision session needs; whether it belongs in the core is
  re-evaluated when `helm-session-host` is built.

Everything domain- or role-specific stays **out** of the core and lives in the
consumer:

- operator-coordination semantics → `helm-coordination`
- decision-session semantics (finding routing, `SessionPush`, urgency-based
  delivery, the terminal-HITL main loop) → `helm-session-host`

Guard: the stem is defined by the **union of what real consumers already need**,
not by anticipated needs. A primitive with exactly one consumer does not belong in
the core yet.

## Concept layering (holistic view)

The redirection is an application of *separation of concerns*: strong concepts
that play together through well-defined APIs, each depending only downward. Read
top-to-bottom, each layer knows nothing about the layers above it.

| Layer | Concept | Home | Knows nothing about |
|---|---|---|---|
| 1 | **Event transport** — ordered, replayable, fan-out event stream; sequencing; SSE framing | `runway-app-host` (hub, `EventEnvelope`, `EventSubscription`, `sse`) | gates, sessions, operators, Converge |
| 2 | **HITL gate** — a decision point that blocks on a human: pause → await verdict → resume; `GateDecision` | today `helm-governed-jobs` (incidental); clean home is its own concept once it has a third consumer | *what* is being decided, *who* decides, transport, Converge |
| 3 | **Session / membership** — who is participating, heartbeat leases, presence, recorded decisions | `helm-coordination` (operator-flavored today) | the domain loop running on top |
| 4 | **Domain loops** — `helm-governed-jobs` (truth execution + Converge driver), `helm-coordination` (operator coordination), `helm-session-host` (decision deliberation, finding routing, `SessionPush`) | their own crates; **compose** layers 1–3; Converge lives here | each other |
| 5 | **Apps** — Quorum, Atlas | `marquee-apps/*` | each other's internals |

What went wrong and what the fix restores: layer-1 sequencing was left half-built
(in-memory hub returns `next_sequence: None`), so layer-4 crates grew their own
copies of a layer-1 responsibility — the boundary blurred and the code duplicated.
Strengthening `runway-app-host` pushes the responsibility back to the concept that
owns it; the duplication disappears as a *consequence* of the boundary being
correct, not as the goal.

The one subtlety this lens exposes: the **HITL gate (layer 2)** is a genuinely
distinct concept whose current residence inside `helm-governed-jobs` is incidental.
Its clean home is *itself*, not "jobs." But there is **no rendezvous duplication
today** — it lives once in `helm-governed-jobs` and `helm-coordination` consumes it
across the existing edge; only the `GateDecision`/`GateDecisionKind` *enum* is
duplicated, which de-dupes trivially in place. So the principled call is: **unify
the enum now, leave the rendezvous where it is, and give the gate concept its own
clean home when `helm-session-host` makes it a true three-consumer concept** — at
which point a small dedicated crate is *separation of concerns*, not a speculative
wrapper. Extracting it earlier would move working, non-duplicated code purely for
tidiness, which the ownership directive cautions against.

Decisive evidence for the *timing*: `helm-coordination` couples to
`helm-governed-jobs` through `JobStreamState`, not through the gate types —
`service.rs:39` holds `Option<Arc<JobStreamState>>`, `:86` `with_job_state(...)`,
`:251` `job_state.signal_gate(...)`. That coupling exists for **two** reasons,
sequence-sharing (`job_state.next_sequence.clone()`, `service.rs:90`) and
`signal_gate`. **Step 1 (upstream sequencing) removes the first reason entirely**
— once the hub owns the counter, nothing reaches into `JobStreamState` for it.
Only `signal_gate` remains. So a gate-only `helm-session-core` extracted *now*
would add a crate without severing the `coordination → governed-jobs` edge (still
needed for `JobStreamState`), whereas extracting the gate concept *after* the
upstream fix — at the `helm-session-host` step — has a single, clean purpose and
actually severs the sibling edge. Order is load-bearing: strengthen upstream
first, then re-scope the gate extraction against what coupling actually remains.

## Consequences

- A new crate `helm-session-core` is created; `helm-session-host` is built on top
  of it. The spine design's Repo Structure section names the stem and the
  dependency.
- `helm-coordination` and `helm-governed-jobs` are **migrated to consume the
  stem** rather than carrying their own copies of membership/presence/SSE/gate
  primitives. This is a refactor of working crates: do it incrementally, behind
  characterization tests, with **no change to their external behavior**.
- The extraction is a **prerequisite for Plan 2** — `helm-session-host` composes
  the stem, so the stem lands first (new "Stem-extraction" plan; see follow-ups).
- Migrating shipped crates is a Tier-1+ change: capture characterization tests
  before and after, and stage the migration so each crate stays green throughout.

## Follow-ups

1. **API audit (pre-extraction, required):** _done 2026-06-26._ Canonical record:
   `KB/04-architecture/2026-06-26-helm-session-core-api-audit.md`. Headline
   results: external migration impact ≈ zero (the only externally-imported symbol
   across both crates is `GovernedJobsModule`; `helm-coordination` has no external
   consumers and is mounted by no app). First pass moves the four Converge-free,
   two-consumer primitives (publisher, SSE stream-builder, gate-wait rendezvous,
   unified `GateDecision`); membership / presence / optimistic ledger are deferred
   (one in-tree consumer each). See the Decision section for the two ADR
   corrections the audit forced (core is Converge-free; ledger is operator-specific
   and not assumed core).
2. **Write the implementation plan** — _done 2026-06-26; rewritten same day for
   Option 5:_ `KB/08-roadmap/2026-06-26-spine-plan-upstream-event-sse.md`
   (strengthen `runway-app-host` sequencing + SSE combinator; migrate both helm
   crates onto it; **no new crate**; characterization-tests-first; ends with a
   public-API diff proving both helm crates' surfaces unchanged; sequenced before
   Plan 2). The earlier `…spine-stem-plan-helm-session-core.md` draft is retired —
   it described the superseded Option-4 crate extraction.
3. **Open the originating ledger finding** — done: `QF-2026-06-26-01` (D-tier),
   linked both ways with this ADR.
4. **Separate decision (not this ADR):** the §6 across-session → live-coordination
   governance boundary is a distinct Quorum-policy decision; track it on its own.

## Revisit

- Note for future scrutiny: the justification rests on the **two in-tree
  consumers** (`helm-coordination`, `helm-governed-jobs`). The composed-decision
  suite stages (keystone, tally-escrow, plumb-execution) are **KB-only today** —
  they motivate the direction but do not, by themselves, justify the extraction.
  The two existing crates already clear the ≥2-consumer guard, so the decision
  holds on in-tree evidence alone.
- Re-examine after the API audit (follow-up 1): if the existing primitives prove
  not cleanly extractable, revisit the migration *sequencing* (not the decision).
  Otherwise revisit by **2026-09-26**, once `helm-session-host` has landed as the
  third in-tree consumer.
