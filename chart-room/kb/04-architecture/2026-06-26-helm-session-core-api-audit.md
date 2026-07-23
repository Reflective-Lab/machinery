# helm-session-core — Pre-Extraction API Audit

- Date: 2026-06-26
- Status: Audit (read-only survey of current code) — satisfies ADR follow-up 1
- Authority: ADR `decisions/2026-06-26-helm-session-host-vs-coordination` (**Accepted**)
- Scope: define exactly which primitives move into `helm-session-core`, and the
  migration impact of moving each. **No code written.**
- Feeds: the implementation plan `KB/08-roadmap/2026-06-26-spine-plan-upstream-event-sse.md`

> **Redirection note (2026-06-26).** This audit's *analysis* stands (it correctly
> identified the shared primitives and that the gate primitive is Converge-free).
> Its *conclusion* — extract a `helm-session-core` crate — was superseded by ADR
> Option 5: the generic event/SSE mechanics belong **upstream in `runway-app-host`**
> (which already half-owns them), and the gate concept stays in `helm-governed-jobs`
> until `helm-session-host` is a real third consumer. See the rewritten plan above.

> Evidence convention (per `AGENTS.md` `RP-AI-EVIDENCE-CITED`): every claim cites
> a file:line or a command result. Inferences are marked.

---

## What the ADR asked

The ADR names four primitive families for the core and a hard guard:

1. heartbeat-leased session membership + presence
2. SSE stream fan-out / multiplex
3. the HITL gate lifecycle (pause → await human → resume)
4. the gate / decision-ledger primitive

> **Guard (ADR):** "the stem is defined by the **union of what real consumers
> already need**, not by anticipated needs. A primitive with exactly one consumer
> does not belong in the core yet."

The two in-tree consumers that justify the extraction are `helm-coordination` and
`helm-governed-jobs` (ADR Decision). This audit holds every public primitive
against that guard.

### Ownership correction (2026-06-26)

This audit originally classified the duplicated sequence-stamped publisher and
replay+live SSE loop as `helm-session-core` moves. On deeper workspace inspection,
`runtime-runway/crates/runway-app-host` already owns the upstream abstractions:
`EventHub`, `EventHubHandle`, `EventSubscription` (`realtime.rs`) and an SSE module
(`sse.rs`). Because Reflective owns both repos, the generic event mechanics should
strengthen `runway-app-host` first, then Helms should consume them. They remain
valid extraction work, but their **destination is upstream**, not
`helm-session-core`.

Revised destination split:

- `runway-app-host`: sequence-stamped event-draft publishing; generic replay+live
  SSE stream helper.
- `helm-session-core`: Helm-session-specific `GateDecision` and
  `GateWaiterRegistry`.

Do not duplicate or wrap generic event machinery locally in Helms unless a later
implementation pass finds a concrete reason the upstream API cannot carry it.

---

## Consumer map (evidence)

Searched the whole workspace for imports of the two crates
(`rg --no-ignore -g '*.toml'` and `-g '*.rs'`):

| Crate | In-tree consumers | External consumers (outside `bedrock-platform/helms`) |
|---|---|---|
| `helm-governed-jobs` | `helm-coordination` (`ledger.rs:16`, `service.rs:11`) | `quorum-server` (`main.rs:20`, mounts `GovernedJobsModule::new()` at `main.rs:1827`); `atlas-server` (`main.rs:18`) — **both import only `GovernedJobsModule`** |
| `helm-coordination` | none | **none** |

Findings that shape migration risk:

- **`helm-coordination` has zero external consumers** and is **not mounted by any
  app** — `quorum-server`/`atlas-server` mount `GovernedJobsModule` only, never
  `CoordinationModule` (`rg 'Coordination' marquee-apps/.../quorum-server/src/main.rs`
  → no matches). Its blast radius today is the helms workspace + its own tests.
- **The only externally-imported symbol across both crates is
  `GovernedJobsModule`.** Because the migration keeps public APIs unchanged (ADR
  Consequences: "no change to their external behavior"), and `GovernedJobsModule`
  stays in `helm-governed-jobs`, **external migration impact is effectively
  zero**. `marquee-apps/`, `runtime-runway/`, and `mobile-apps/` carry no direct
  dependency on the primitives being moved. (`atelier-showcase` /
  `arena-tests` reference the showcase crate `scenario-helm-coordination-headless`,
  which depends on `helm-operator-control`, **not** on `helm-coordination` —
  `atelier-showcase/scenarios/helm-coordination-headless/Cargo.toml:17-19`.)
- **`helm-operator-control` does not actually depend on `helm-governed-jobs`** —
  the `lib.rs:30` comment ("for the operator-control approval handler") is
  aspirational; there is no dependency edge and no usage
  (`helm-operator-control/Cargo.toml` has neither dep; only a `pipeline.rs`
  comment references `RealtimeHub`).

---

## Correction to an ADR assumption (load-bearing for Item 3)

The ADR and spine §1a describe the target HITL as
`Engine::resume(GateDecision)` and conclude the core "will need `converge-core`
as a dep." **The current code does not work that way, and the extractable gate
primitive is Converge-free.** Evidence:

- The gate lifecycle in `job_stream.rs` is: emit `gate.paused`
  (`job_stream.rs:519`), block a `tokio::oneshot` waiter with a timeout
  (`job_stream.rs:532`), then on `Approved` **re-execute the truth**
  (`execute_truth`, `job_stream.rs:573`) — it never calls `Engine::resume`.
- The signalling primitive — `GateDecision` enum (`job_stream.rs:74-78`),
  `JobGateWaiter` (`:80-98`), `register_gate_waiter` / `take_gate_waiter` /
  `signal_gate` (`:149-188`) — touches `tokio::oneshot` only. **No `converge_core`
  import.**
- `converge_core` is used **only** in the truth-execution job driver
  (`run_job_task` → `ContextState`, `CriterionResult`, `ExperienceEvent`:
  `job_stream.rs:53, 484-489, 713-734`), which is `helm-governed-jobs`-specific
  and **stays out** of the core.

**Consequence:** for a *no-behavior-change* extraction (the migration's hard
constraint), `helm-session-core` should ship **without** `converge-core`. The
`Engine::resume(GateDecision)` integration is a forward step that arrives with
the **third** consumer (`helm-session-host`), at which point the core may gain
`converge-core`. Pulling it in now would add an unused heavy dep and overshoot
the ADR guard. (See Item 3 Global Constraints — this overrides the original
brief's "converge dep is expected" note, on the evidence above.)

---

## Primitive-by-primitive audit

Legend: **Shared?** = would `helm-session-host` (decision sessions) plausibly
need it. **2 consumers now?** = do *both* in-tree crates use it *today* (the ADR
guard). **Clean?** = extractable without dragging operator/job-domain semantics.

### Genuinely shared, multi-consumer, clean → **MOVE to core**

| Primitive | Current owner | Shared? | 2 consumers now? | Clean? | Migration notes |
|---|---|---|---|---|---|
| **Sequence-stamping event publisher** — `CoordinationPublisher` (`events.rs:62-107`) ≈ `Publisher` (`job_stream.rs:235-262`) | both (duplicated) | yes | **yes — duplicated in both** | yes (thin wrapper over `runway_app_host::EventHubHandle` + `Arc<AtomicU64>`) | This is the clearest extraction: two near-identical `Arc<AtomicU64>`-stamps-`EventEnvelope` wrappers. Extract `SessionEventPublisher { hub, seq, app_id }` with `emit(type, actor, payload)`. `CoordinationPublisher` and `Publisher` become thin shims (or are deleted). Core depends on `runway-app-host` (both consumers already do). |
| **SSE replay+live stream builder** — `build_stream` (`http.rs:199-231`) ≈ `build_run_sse_stream` (`job_stream.rs:796-832`) | both (duplicated) | **yes — duplicated in both** | yes (generic over a filter predicate) | Both run the same loop: drain `subscription.replay`, dedup by `sequence`, then `recv()` live, skip `Lagged`, break on `Closed`. The only difference is the per-event filter (`include`/workspace vs `run_id`) and a terminal predicate (`is_terminal`, jobs only). Extract a generic `sse_stream(subscription, filter: Fn(&EventEnvelope)->bool, terminal: Fn(&EventEnvelope)->bool)`. Consumers supply their own predicate. |
| **HITL gate-wait lifecycle** — `GateDecision` (`job_stream.rs:74`), `JobGateWaiter` (`:80`), `register_gate_waiter`/`take_gate_waiter`/`restore_gate_waiter`/`signal_gate` (`:149-188`) | helm-governed-jobs (owns), helm-coordination (drives via `signal_gate`, `service.rs:251`) | yes | **yes — owned by jobs, called by coordination** | yes — **Converge-free** (see correction above) | Extract the waiter registry (`HashMap<String, GateWaiter>` + oneshot + signal) into core as `GateWaiterRegistry`. **Do not** move the truth re-execution (`run_job_task`). `JobStreamState` keeps a `GateWaiterRegistry` field instead of inlining the map (`job_stream.rs:119`). `gate_timeout` policy (`:127`) stays with the consumer (it is a job-run policy). |
| **Gate-decision vocabulary** — `GateDecisionKind {Approve,Reject}` (`ledger.rs:26`) + `GateDecision {Approved,Rejected}` (`job_stream.rs:75`) + the `to_gate_decision()` bridge (`ledger.rs:34`) | both | **yes** | yes | Two near-duplicate two-variant enums bridged by a hand-written mapping — a smell. Unify into one core `GateDecision` (approve/reject). `GateDecisionKind`'s serde (`kebab-case`, `ledger.rs:25`) is the wire contract on the coordination HTTP body (`http.rs:135`), so the core type must preserve that serialization to keep behavior unchanged. |

### Named in the ADR as core, but **single-consumer today** → guard says **DEFER** (move with `helm-session-host`)

| Primitive | Current owner | Shared (future)? | 2 consumers now? | Clean? | Migration notes |
|---|---|---|---|---|---|
| `SessionRegistry` / `Session` / `DEFAULT_SESSION_LEASE` (`session.rs:20-134`) | helm-coordination | yes (host needs membership) | **no — only coordination** | mostly (depends on `OperatorPrincipal`) | The ADR lists "heartbeat-leased membership" as core, but `helm-governed-jobs` has **no** sessions (jobs key off `run_id`). Per the guard ("exactly one consumer → not in core yet"), this **does not move in the first pass**. It moves when `helm-session-host` lands as the second consumer — *and* it must first be decoupled from `OperatorPrincipal` (see Stays-out). Recommend: generic `Session<M>` over a membership-identity type, or a `Member` trait. |
| `PresenceRegistry` / `PresenceEntry` / `PresenceChange` (`presence.rs:19-133`) | helm-coordination | yes | **no — only coordination** | mostly (depends on `OperatorPrincipal` + `SubjectRef`) | Same verdict as sessions: single-consumer today, defer to the `helm-session-host` step. "Soft-claims never lock" (`presence.rs:5`) is operator-coordination flavored; decision-session presence may differ. Do not speculatively generalize. |
| `DecisionLedger` / `DecisionOutcome` / `DecisionRecord` (`ledger.rs:44-122`) | helm-coordination | partially | **no — only coordination** | no — see notes | The ADR calls this "the gate / decision-ledger primitive," but the implemented ledger is the **optimistic multi-operator** model: first-writer-wins, idempotent-repeat, divergent-conflict (`ledger.rs:1-10, 82-112`). That conflict semantic is **operator-coordination-specific** — a single-loop decision session has one terminal gate, not racing operators. **Do not move as-is.** What the gate *lifecycle* needs (the waiter + signal) is already extracted above. Revisit whether `helm-session-host` wants an append-once ledger when it is built. |

### **STAYS OUT** — domain/role-specific or single-consumer composition roots

| Primitive | Owner | One-line reason it stays out |
|---|---|---|
| `OperatorPrincipal`, `PrincipalClaim`, `PrincipalResolver`, `RequestActorResolver` (`principal.rs`) | coordination | Operator-identity semantics; decision sessions have *participants* (`ActorId`), not workspace operators. Single-consumer. Blocks `Session`/`Presence` extraction until decoupled. |
| `AuthorityResolver`, `PermissiveAuthority` (`ledger.rs:128-140`) | coordination | "Who may decide a gate" authority seam — coordination policy; single-consumer. |
| `SubjectRef` (`subject.rs`) | coordination | Advisory `kind:id` pointer for presence/claims; only coordination uses it; generic-looking but single-consumer. |
| Event-type consts + `is_coordination_type` / `is_job_type` (`events.rs:20-55`) | coordination | The coordination event *vocabulary* and its multiplex policy (`stream_includes`, `service.rs:281`). The generic stream *loop* moves; the vocabulary is consumer-specific. |
| `CoordinationError` (`error.rs`) | coordination | Coordination-specific error enum + HTTP mapping. |
| `CoordinationService`, `CoordinationModule` (`service.rs`, `module.rs`) | coordination | The consumer's composition root + `HelmModule` mount — by definition the consumer, not the stem. |
| `GovernedJobsModule` (+ readiness re-exports) (`lib.rs:49-147`) | governed-jobs | Consumer composition root + `HelmModule` mount; **the one externally-imported symbol** — must stay put and unchanged. |
| `JobStreamState` (`job_stream.rs:108-125`) | governed-jobs | Bundles `store`/`runtime_stores`/`truths` (truth-execution wiring) with the shared hub+waiters. It **stays**, but is refactored to *hold* the extracted `SessionEventPublisher` + `GateWaiterRegistry` rather than inline them. |
| `JobRunTask`, `run_job_task` (`job_stream.rs:424-619`) | governed-jobs | The truth-execution driver — depends on `converge-core`, `truth-catalog`, `helm-truth-execution`, `organism-pack`. The Converge coupling lives here and stays here. |
| `gate_timeout` (`job_stream.rs:117,127`) | governed-jobs | A job-run wall-clock policy (10-min default), not a shared primitive. |

---

## Net: what the first extraction pass moves

Only the four **MOVE** rows above — all four are used by **both** in-tree
consumers today (publisher + SSE loop are literally duplicated; gate-wait +
gate-decision span both), all are **cleanly** extractable, and all are
**Converge-free**:

1. `SessionEventPublisher` (sequence-stamped `EventHubHandle` wrapper)
2. `sse_stream(...)` generic replay+live builder (filter + terminal predicate)
3. `GateWaiterRegistry` (oneshot waiter + `signal`/`take`/`restore`)
4. unified `GateDecision` (preserving the `kebab-case` wire contract)

**Deferred to the `helm-session-host` step** (single-consumer today, per the
guard): `SessionRegistry`/`PresenceRegistry` (and their `OperatorPrincipal`
decoupling), and a decision on whether the optimistic `DecisionLedger` belongs in
core or stays a coordination specialty.

**Core dependency set (first pass):** `runway-app-host`, `tokio`, `serde`,
`serde_json`, `uuid`, `chrono`, `thiserror`, `async-stream`, `tokio-stream`.
**No `converge-core`** (see the ADR-assumption correction).

This keeps the first extraction minimal, evidence-backed, and faithful to the
ADR guard, while still collapsing the two genuine code duplications (publisher,
SSE loop) that motivated the stem.
