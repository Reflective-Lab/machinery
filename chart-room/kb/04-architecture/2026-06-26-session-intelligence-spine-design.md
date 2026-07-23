# Session Intelligence Spine — Design

- Date: 2026-06-26
- Status: Draft — Plan 1 written; main-loop mechanism decided (terminal-HITL suggestor, §1a); shared `helm-session-core` stem **Accepted** (ADR `decisions/2026-06-26-helm-session-host-vs-coordination`); Plan 2 (`helm-session-host`) sequenced after the Stem-extraction plan
- Scope: Helms, Quorum Sense (reference app), mobile-apps, desktop
- Relates to: [[current-system-map]], [[mobile-apps]], [[bedrock-platform]]

---

## Platform Alignment Guard

> Read this before extending the design or implementing from it.

Every concept below has a Converge, Organism, or Axiom analogue. Use those — do
not invent parallel machinery.

| Temptation | What already exists |
|---|---|
| Inventing a "loop" | A Converge **Formation** with explicit **Budgets**. `max_cycles` and `max_facts` are enforced by the engine loop; `time_limit` is declared on the intent but **not** enforced by the engine today (`StopReason::TimeBudgetExhausted` exists but the loop never checks wall-clock) — wall-clock bounding is the caller's responsibility. Formations run to a fixed point or declare failure. No loop is open-ended. |
| Inventing an "outcome anchor" | A **RootIntent**. The live entry point is `TypesRootIntent` (`converge_core::types::intent`), consumed by `Engine::run_with_types_intent`; the older `converge_core::root_intent::RootIntent` still exists but is legacy. Has typed success criteria, a `Vec<IntentConstraint>` (each carrying `ConstraintSeverity::Hard`/`Soft`), `Scope`, and budgets. The only entry point into a Converge runtime. Nothing may override it during execution. |
| Inventing a "proposed signal" | A **ProposedFact** submitted via the admission boundary (`converge_core::admission`). External actors submit observations with `ActorId` and provenance; Converge stages them as proposals. The promotion gate is the only path to governed Facts. |
| Inventing a "fact" | A promoted **Fact** — what the promotion gate produces after validating a `ProposedFact` against Truths. Facts are constructed only by the engine, never directly. |
| Inventing an "authority model" | **KernelPolicy** (`converge_core::kernel_boundary`): adapter selection, `requires_human`, `required_truths`. Routing is a **separate** type, `RoutingPolicy` (same module) — not a field on `KernelPolicy`. Authority is explicit, never emergent. |
| Inventing a "HITL gate" | Configure a Converge **`HitlPolicy`** (matches on proposal kind, confidence threshold, or agent id). When it matches, the engine returns `RunResult::HitlPause(HitlPause)` holding the proposal. The human verdict is delivered via `Engine::resume(pause, GateDecision)` — `GateDecision::approve(gate_id, decided_by)` promotes the held proposal, `reject()` discards it. This is a **verdict on a paused proposal, not free-form re-admission**. Note: `KernelProposal.requires_human` is kernel/LLM-boundary vocabulary and does **not** drive engine HITL. |
| Inventing a "coordinator agent" | Implement the **Suggestor** trait. A `CoordinatorSuggestor` struct exists in Quorum Sense (`adaptive_inquiry_coordinator.rs`), but today it emits `Proposal::CoordinatorProbeAssignment` (probe→actor routing) from decision/authority/presence context — it does **not** read the evidence topology and does **not** emit findings. This design adds a new output type, new topology inputs, and urgency-derivation logic: take the existing struct as a starting point, but expect a substantial new suggestor, not a small extension. |
| Inventing a "formation assembler" | **Organism** owns formation selection and intent decomposition. The App hands a problem to Organism; Organism selects the formation. |
| Inventing a "truth validator" | **Axiom** owns truth compilation and Rust codegen from `.truths` files. Gate conditions compile to Axiom-generated validators, not hand-written checks. |
| Reinventing session routing / SSE / a gate ledger in Helms | A shared **`helm-session-core`** stem owns the reusable primitives — heartbeat-leased session membership, presence, SSE fan-out, the gate / decision ledger, and the HITL gate lifecycle (resume via `Engine::resume(GateDecision)`). `helm-coordination` (operator coordination), `helm-governed-jobs` (governed jobs), and `helm-session-host` (decision sessions) all **consume the stem** — none stands up parallel machinery. See ADR `decisions/2026-06-26-helm-session-host-vs-coordination` (**Accepted**). |

### Ownership contract

The split is surgical. "Helms owns orchestration" must **not** erode into Helms
running the loop or selecting the formation.

| Layer | Owns | Does NOT own |
|---|---|---|
| **Converge** | The loop *mechanic*: admission, promotion, facts, criteria evaluation, fixed-point convergence, budgets, HITL pause/resume, storage contracts. Generic; reused by every formation. | The session's purpose; which suggestors run; what a "hypothesis" is. |
| **Organism** | The *formation template* — `Huddle`, `Competition`, `Deliberation`, etc. Dynamic / LLM-shaped inside, but every template runs under Converge's exact contract and only ever emits `ProposedFact`s. | The loop mechanic; the domain payload. |
| **$APP (Quorum)** | The *instrumentation*: this session's `RootIntent` + success criteria, suggestor selection from Mosaic (LLM / websearch / math / policy), gate truths, starting context (settled facts, curiosity level, allowed nudges). | The loop mechanic; transport; session hosting. |
| **Helms** | The *session host*: holds the long loop open, routes findings, runs gates as operator surfaces, owns the device↔server hybrid sync, trust transfer. The reusable operator protecting the stem of all apps. | Running the loop; selecting the formation; interpreting `domain_payload`. |

Rule of thumb: the **mechanism** is never app-specific; the **configuration**
always is. The App supplies the `RootIntent` + suggestor/gate config; Organism
chose the template; Converge runs it; Helms hosts the running session.

---

## Purpose

This design describes the **Session Intelligence Spine**: the reusable runtime
backbone for governed, multi-participant decision sessions across the Reflective
app suite. It is the runtime *body* of Shoal's doctrine — *governed reasoning
composes across the lifecycle of a decision* — not a restatement of it. Quorum
Sense (the elicitation stage of the composed-decision stack) is the reference
implementation; the same spine serves the other stages.

The organizing split is **individual amplification vs. collective intelligence**:

- **On device (Client Helm) — "make me the best version of myself."** Personal,
  mostly-sequential local formations: the AI-assisted guidance a participant
  asks for. Desktop and mobile are identical in this role; the desktop is *not* a
  web mirror of the session.
- **On the server — "make us better."** Coordination, collaboration, collective
  wisdom: many people and many AI, the group effort.

Within that split the spine runs:

- **One long-running main formation** holding the session's `RootIntent` — the
  raison d'être and the fixed desired outcome that must not drift. It reaches a
  fixed point via a terminal HITL suggestor (§1a), not by running forever.
- **Server sub-formations** in parallel on sub-problems (hypotheses, evidence
  gaps, AI workers, sub-groups).
- **Local formations** on each device — plus *handles* to server sub-formations a
  participant spawned (the "DD job while I wait" case; §4).
- Participants express **temperature** (position + conviction) as `ProposedFact`s
  that extend the session's evidence topology.
- The `CoordinatorSuggestor` reads the topology and emits `CoordinatorFinding`s
  (as `ProposedFact`s, promoted by the gate) that route intelligence back to
  participants via the Server Session Helm.
- Client Helm applies severity-based routing (B or C) to incoming pushes.

The spine is defined in Helms and reusable across the suite. At this stage no app
delegates to another — each solves one problem well and trusts the suite to cover
the rest; cross-app composition via `atlas://` links is later, high-level
integration. Quorum Sense is built self-contained.

---

## Section 1 — System Architecture

```
App (domain: problem definition)
  │  provides RootIntent (SuccessCriteria = fixed desired outcome, must not drift)
  ▼
Server Session Helm
  ├── MAIN FORMATION  (Converge, all participants; converges via terminal HITL suggestor — §1a)
  │     Budgets declared upfront; meeting = wall-clock budget; declares failure rather than drifting
  │     Sub-formations for hypotheses, evidence gaps, AI workers, sub-groups
  │     CoordinatorSuggestor reads evidence topology → emits CoordinatorFindings (as ProposedFacts)
  │     findings flagged requires_human → matching HitlPolicy fires → HITL pause → GatedDecision surface
  │
  └── Routing layer (domain-agnostic)
        Receives promoted CoordinatorFinding Facts
        Emits SessionPush via SSE to target participants
        Manages HITL gate lifecycle (delivers GateDecision to Engine::resume on response)

          │  SSE push + temperature ProposedFacts
          ▼

Client Helm  (Rust core, UniFFI — same crate for mobile and desktop)
  ├── SSE subscription
  ├── Loop Registry  (local formations + handles to server sub-formations)
  ├── Severity Router  (B or C based on urgency + local formation state)
  └── Temperature queue  (ProposedFacts submitted back to server admission)

        Mobile:   Rust + UniFFI → Swift / Kotlin (native UI)
        Desktop:  Rust → Tauri + Svelte (individual amplification, not web mirror)

Web client (thin):
  SSE rendering + temperature submission
  No local formations
```

**The closed loop:**

```
Participant submits temperature ProposedFact
  → server admission boundary stages it
  → promotion gate validates against Truths
  → becomes Fact in evidence topology (alignment + conviction dimensions)
  → CoordinatorSuggestor reads topology change
  → emits CoordinatorFinding { urgency_intent, requires_human? } as a ProposedFact
  → if requires_human: matching HitlPolicy fires → engine RunResult::HitlPause, GatedDecision pushed to participants
  → if auto-promotable: becomes Fact → routing action → SessionPush via SSE
  → Client Helm receives push → Severity Router applies B or C
  → local formation runs → output: new temperature ProposedFact + proposals
  → submitted back to server → closes the loop
```

**Client Helm scope — individual amplification:**

The Client Helm is not the session. It manages one participant's local formations.
Desktop and mobile are identical in this role. Server handles the group.
Desktop UI is oriented toward individual support, not a web mirror of the session.

---

## Section 1a — Main-Loop Semantics

A live session does not reach a static fixed point while people are still talking
— so "runs to a fixed point" needs a precise mechanic. The main formation
converges the way every Converge formation does: it reaches a fixed point when **a
terminal suggestor proposes that the success criteria are met**, and that proposal
is **gated to a human**.

- **Normal cycle:** suggestors emit `ProposedFact`s; the promotion gate validates
  them against Truths; the topology updates.
- **Terminal suggestor:** watches the `RootIntent`'s `SuccessCriteria` against the
  promoted facts. When it judges them satisfiable, it emits a terminal
  `ProposedFact` ("we can claim success with these results").
- **Terminal gate:** a `HitlPolicy` gates that terminal proposal. The engine
  pauses (`RunResult::HitlPause`); a human (per the `GateCondition`) either
  **claims success** (`approve` → promoted → formation converges) or **honestly
  declares it unmet** (`reject` → the loop continues, or fails on budget).

This is how a long-running formation stays *rigid in structure* while its interior
is *dynamic*: the terminal gate is fixed even when the formation is a `Competition`
of LLM agents. The discipline that keeps "Converge does not invent results" true
is unchanged — dynamic formations may be creative in *proposing*, but only the
promotion gate plus the terminal HITL turn proposals into a claimed outcome.

**Where it lives.** The *mechanism* (terminal suggestor + HITL gate + budgets) is
Converge plus an Organism formation template (a `Deliberation`-style template). The
*purpose* (what this session is for, its success criteria, its starting context) is
`$APP` configuration, instrumented into the Helms session host. None of this is new
kernel machinery — it composes primitives that already exist.

**Budget = the meeting.** The session's wall-clock bound is the meeting itself.
Because the engine does not enforce `time_limit` (see the guard table), the Helms
session host owns the wall-clock budget for the main loop — exactly as Client Helm
does for local formations.

**The outcome must not shift — so reframing is an App-level event, not a loop
event.** If a session discovers it is solving the wrong problem ("we came to decide
X; the real question is Y"), the main loop does **not** silently re-aim its
`RootIntent` (that would be drift). It reaches **failure** on the original intent,
and the App opens a *new* session with a *new* `RootIntent`. Drift stays impossible
inside the loop; reframing happens above it.

---

## Section 2 — Temperature as Domain Evidence

Temperature is a participant observation submitted through the normal Converge
admission boundary as a `ProposedFact`. It is not a separate protocol.

```
// In the app domain (Quorum Sense or any blueprint app)
ParticipantTemperature {
    actor_id:    ActorId,          // the participant — maps to AdmissionActorKind::Human
    subject_ref: SubjectRef,       // what this is about: a hypothesis, signal, synthesis
                                   // temperature without a target is noise
    position:    Stance,
    conviction:  ConvictionWeight,
}

enum Stance         { Agree, Disagree, Uncertain, NeedMoreEvidence }
enum ConvictionWeight { Low, Medium, High, Critical }
```

`subject_ref` is the critical field. A `Critical / Disagree` on a specific
hypothesis is actionable. Without a target, it has no meaning in the topology.

### Evidence topology extensions

The existing `quorum.evidence-topology.v1` holds: diversity, independence,
dissent, recency, role coverage. Temperature adds two dimensions.

Be precise about shape: the existing dimensions are **not** profiles — each is a
single `score: Confidence` scalar plus `required`, `satisfied`, `evidence_count`,
and `rationale` (`EvidenceTopologyDimension`). `alignment` and `conviction` are
genuinely *richer* than any existing dimension (per-`subject_ref` distributions,
broken down by role), so they are justified on their own merits, not by analogy
to the current dimensions. Adding them changes the topology's serialized shape:
treat it as a schema version bump (`quorum.evidence-topology.v2`) because
consumers (e.g. the desktop SSE event union) parse it.

| New dimension | What it captures |
|---|---|
| `alignment` | Distribution of stances per `subject_ref`, broken down by role |
| `conviction` | Aggregate importance weight across participants per `subject_ref` |

Together: not just *that* there is dissent, but *how strongly held* it is and
*whether it clusters by role*.

### How the CoordinatorSuggestor reads it

In the target state, the `CoordinatorSuggestor` (implementing the `Suggestor`
trait) reads the full evidence topology on each cycle. When `alignment` or
`conviction` shift, it evaluates significance and emits a `CoordinatorFinding`
as a `ProposedFact` (promoted to a Fact by the gate, then routed). (Today this
suggestor reads decision/authority/presence context and emits probe
assignments — adding topology reads + findings is the new work; see the guard
table.)

```
CoordinatorFinding<QuorumPayload> {
    // emitted as a ProposedFact; promoted to a Fact by the gate
    finding_type:        FindingType,
    domain_payload:      QuorumPayload,   // opaque to Helms
    urgency_intent:      UrgencyIntent,   // derived from topology, never assigned
    requires_human:      bool,            // true → HITL pause in main formation
    target_participants: Vec<ActorId>,
}

enum FindingType {
    ContradictionDetected,
    ConsensusEmerging,
    HighConvictionDissent,
    EvidenceGap,
    HypothesisReady,
    UncertaintyCluster,
}

enum UrgencyIntent { Informational, Advisory, Disruptive, Preemptive }
```

`urgency_intent` is derived from topology state using domain knowledge
(inquiry phase, role weights, hypothesis importance). The derivation lives
entirely inside `CoordinatorSuggestor` — Helms never interprets it.

| Topology state | Derived urgency |
|---|---|
| High-conviction dissent from a key role on a near-ready hypothesis | `Preemptive` |
| Emerging consensus, high alignment, high conviction | `Advisory` |
| Evidence gap, low conviction overall | `Informational` |
| Contradiction detected, mixed conviction | `Disruptive` |

> The coordinator's bias policy — whether to amplify dissent or nudge toward a
> direction, and how hard — is **Quorum-domain configuration driven by the
> `RootIntent` starting context** (settled facts not to re-litigate, the
> encouraged curiosity level, where nudging is allowed), not a spine concern. Not
> all input weighs equally, and not all of it must be surfaced. The spine carries
> `urgency_intent` opaquely; Quorum decides what earns it.

When `requires_human = true`, a matching `HitlPolicy` fires and the engine
returns `RunResult::HitlPause(HitlPause)`, holding the proposal. Helms surfaces
this as a `GatedDecision`. The human verdict returns to the engine via
`Engine::resume(pause, GateDecision)`: `GateDecision::approve(gate_id,
decided_by)` promotes the held proposal; `reject()` discards it. This is a
**verdict on the paused proposal**, not re-admission of a new fact through the
admission boundary. (Temperature and other participant observations *do* flow
through admission — but HITL resume is a distinct, verdict-only channel.)

---

## Section 3 — Server Session Helm: Routing

Server Session Helm sits between the promotion gate and participants. It is
domain-agnostic: it routes `CoordinatorFinding` Facts; it never interprets the
`domain_payload`.

### Normal delivery — SessionPush via SSE

```
SessionPush {
    finding_id:      FindingId,
    urgency_intent:  UrgencyIntent,   // passed through from the promoted Fact
    payload:         DomainPayload,   // opaque — rendered by the client app
    session_context: SessionContext,  // phase, formation cycle, timestamp — Helm owns this
}
```

Delivery by urgency:

| UrgencyIntent | Helm behavior |
|---|---|
| `Informational` | Deliver when connected; safe to drop if offline (client re-derives from session state on reconnect) |
| `Advisory` | Deliver when connected; queue briefly if offline |
| `Disruptive` | Deliver immediately; queue with TTL if offline; surface prominently on reconnect |
| `Preemptive` | Deliver immediately; retry until acknowledged; escalate if unacknowledged |

### HITL gates — GatedDecision

When a `CoordinatorFinding` has `requires_human = true`, a matching `HitlPolicy`
has fired and the engine is already holding the proposal at
`RunResult::HitlPause`. Helms surfaces this:

```
GatedDecision {
    gate_id:           GateId,
    condition:         GateCondition,
    payload:           DomainPayload,
    deadline:          Option<Timestamp>,
}

enum GateCondition {
    QuorumOfRoles(Vec<Role>),
    SpecificAuthority(ActorId),
    AnyParticipant,
    Unanimous,
}
```

> **Note on SessionManifest:** Gate conditions will be compiled from `.truths` /
> Gherkin by Axiom, JTBD-framed. The `GateCondition` variants are the runtime
> representation of Axiom-generated validators. The SessionManifest design
> (how the App hands a `RootIntent` + gate definitions to Server Session Helm)
> is deferred — define it when the first real app session is wired end-to-end.

The human verdict returns to the engine via `Engine::resume(pause,
GateDecision)`: `approve` promotes the held proposal, `reject` discards it. This
is a verdict on the paused proposal, not re-admission of a new fact. Helms
constructs and delivers the `GateDecision`; it does not mutate or resume the
formation directly.

If `deadline` passes without `GateCondition` satisfied, Helms emits a
`GateExpired` event. The App's Truth definitions decide what this means (hard
abort, soft escalation, or continue).

### What Helms does not own

- Does not interpret `domain_payload`
- Does not change `urgency_intent` (set by `CoordinatorSuggestor`)
- Does not change `target_participants` (set by `CoordinatorSuggestor`)
- Does not resume the formation directly (constructs a `GateDecision` and calls `Engine::resume`)
- Does not know what a hypothesis, signal, or synthesis is

---

## Section 4 — Client Helm: Rust Core

Client Helm is a Rust library crate exposed via UniFFI. Identical logic on
mobile (Swift/Kotlin) and desktop (Tauri/Svelte). Native UI drives it;
coordination logic is shared.

### Local formations and server sub-formation handles

On-device Converge is real but deliberately **light**: cheap suggestors only, no
power-hungry compute. Anything heavy (LLM-heavy synthesis, a due-diligence job)
runs as a **server sub-formation** and appears on the device only as a *handle*.
So the Loop Registry holds two kinds of entry:

- **Local formation** — actually runs Converge on this device (the personal task).
- **Server sub-formation handle** — a server formation the participant spawned;
  the device tracks and surfaces it but does not run it.

The "DD job while I wait" is a *handle*, not an on-device loop — it runs on the
server where the compute lives.

Each local formation is a Converge formation instance, light-weight. It has:
- A `RootIntent` — use the live `TypesRootIntent` (the personal task: "help me
  formulate my position on X")
- A formation scoped to personal suggestors (no Organism formation selection,
  no full Mosaic stack — the light-weight converge already established)
- `Budgets` — local formations must terminate. `max_cycles`/`max_facts` are
  engine-enforced; `time_limit` is **not** enforced by the engine loop, so
  Client Helm imposes its own wall-clock guard (see Plan 1, Task 5). No budget
  = not a formation.
- Output: `ProposedFact`s — personal conclusions, refined positions

Local formation output is NOT automatically promoted to Facts in the server
session. It enters the server through the admission boundary as a participant
`ProposedFact` — same path as temperature, same promotion gate.

### Loop Registry

```rust
struct LoopEntry {
    loop_id:         LoopId,
    kind:            LoopKind,          // runs here, or tracked from the server
    root_intent:     TypesRootIntent,   // the personal task (live entry point)
    formation_type:  FormationType,
    state:           LoopState,
}

enum LoopKind {
    Local,                                              // runs Converge on this device
    ServerHandle { server_formation_id: FormationId },  // runs on the server; tracked here
}

enum LoopState {
    Running,
    Paused { injected_context: DomainPayload },  // C: suspended; superseded by a fresh spawn
    Completed(Vec<ProposedFact>),                // output ready for submission
    Failed(ConvergeFailure),
}
```

At most one **local** formation is `Running` at a time. Parallel work (the "DD job
while I wait" case) is a `ServerHandle` entry — an independent server sub-formation
with its own `RootIntent`; it runs on the server, not on device, and does not
contend for the single local-running slot. Entries do not share context.

> **Implementation note — `root_intent` representation.** `TypesRootIntent` above
> is the *conceptual* type. The `helm-client` crate carries **zero Converge
> dependencies** (it is the headless core that compiles to FFI for mobile), so it
> cannot hold a `TypesRootIntent` directly. The plan represents it as an **opaque
> `LocalFormationIntent { description: String, max_cycles: u32, max_facts: u32 }`**
> (`helm-client/src/formation.rs`) — exactly what Client Helm needs from the intent:
> a `description` to display, and the two engine-enforced budget values the
> `WallClockGuard` and the `Local` loop slot reason about. The full
> `TypesRootIntent` is reconstructed on the device's Converge side (or server side
> for a `ServerHandle`) from this opaque carrier; the client core never depends on
> `converge_core`. See Plan 1, Task 2.

### Severity Router

```
incoming SessionPush (urgency_intent, payload)
        │
  any formation Running?
   ├── No  →  B: spawn new local formation, payload as seed facts in RootIntent
   └── Yes
        ├── Informational / Advisory
        │     →  queue; surface as notification; Running formation continues
        ├── Disruptive
        │     →  B: offload to the server as a sub-formation (a ServerHandle entry,
        │            payload as seed facts) — NOT a second local formation. At most
        │            one local formation runs at a time; parallel/heavy work lives
        │            on the server (the "DD job while I wait" case).
        └── Preemptive
              →  C: transition Running → Paused { injected_context: payload }
                     surface to user with explicit accept / dismiss
                     on accept: spawn a FRESH formation seeded with the suspended
                                formation's accumulated context + injected_context
                                (NOT a Converge Engine::resume — resume is a HITL
                                verdict only and does not re-seed a formation)
                     the fresh formation may converge differently — that is the intent
```

The `Preemptive` / C path does not silently drop the suspended work. On accept
it spawns a **fresh formation** seeded with the suspended formation's accumulated
context plus the injected server context — it is **not** a Converge
`Engine::resume` (resume delivers a HITL verdict on a held proposal; it does not
restart a formation with new seed facts). A different output than the
uninterrupted run is the correct behavior. The local `LoopState::Paused` →
`resume()` transition is Client-Helm registry bookkeeping; the actual Converge
work is a new formation run.

### Temperature submission

Temperature `ProposedFact`s are queued with idempotency keys and submitted to
the server's admission endpoint. Same queue as other participant proposals.
Client Helm owns retry; the server admission boundary owns validation.

When a local formation completes, its output `ProposedFact`s (refined position +
proposals) are enqueued with the pending temperature signals. One submission
path for all client output.

### GatedDecision surface

`GatedDecision` events are held separately from the push queue and surfaced as
first-class interrupts to native UI. The user's response is submitted via the
admission boundary. Client Helm tracks deadline and emits a local timeout event
if the user does not respond before the server deadline.

### Director projection (`DirectorFrame`)

The native UI does not read the Loop Registry, push queue, and GatedDecision
surface directly. Client Helm **projects** that ordered state into a single
versioned `DirectorFrame` — "what should I do right now?" — via
`director_snapshot(version, presenter)`. `DirectorFrame` and its vocabulary live in
the `director-contracts` crate (the Rust→FFI/UI boundary, §5); Swift / Kotlin /
Svelte *render* it and never compute it. Two boundaries hold this honest:

- **`version` is the upstream SSE `sequence`** the frame was computed at — not a new
  counter — so snapshots order and dedup consistently with the rest of the spine
  (immutable, versioned snapshots via AsyncStream/Flow, never blocking calls).
- **`helm-client` stays domain-agnostic.** It owns the frame *structure, prompt
  kind, blocking, ids, and deadlines*; the per-app FFI supplies the human *copy*
  from opaque payloads via the `DomainPresenter` seam. Prompts can only present
  contract-backed verdicts (e.g. `GateVerdict` is approve/reject only — a
  "later/defer" choice must enter the Helms gate contract first, never as UI-only).

The Director is a projection, not a new runtime actor. Full UX architecture:
`KB/04-architecture/2026-06-27-ai-director-mobile-ux-architecture.md`.

### Runway and Commerce Rails

| Concern | Source | How Client Helm gets it |
|---|---|---|
| Auth | Runtime Runway | Firebase Auth client SDK → JWT bearer token |
| App host URL | Runtime Runway | Config or discovery manifest from RunwayAppHost |
| Entitlements | Commerce Rails | Custom claims in Firebase JWT, refreshed on subscription change |
| Session SSE URL | Server app | Carried in session join response |

No separate Commerce Rails API. No `runway-storage`, `runway-secrets`, or GPU
paths on device.

---

## Section 5 — Repo Structure

### New crates

```
bedrock-platform/helms/crates/
  helm-session-contracts/     server↔client WIRE boundary — no deps, built first
                                CoordinatorFinding<P>, SessionPush, GatedDecision,
                                UrgencyIntent, GateCondition, SessionContext

  director-contracts/         Rust→FFI/UI PROJECTION boundary — depends only on
                                helm-session-contracts (one-way: projection → wire)
                                DirectorFrame, DirectorSnapshot, DirectorPrompt,
                                DirectorIntent, NowTask, PrimaryAction, ...
                                (AI Director UX — see 2026-06-27 mobile UX doc)

  helm-client/                headless Client Helm Rust core
                                pure library — no Tauri, no UniFFI, no app deps
                                SSE client, Loop Registry, Severity Router,
                                temperature queue, GatedDecision surface,
                                director projection (DomainPresenter seam)

  helm-session-host/          server-side session routing — receives promoted
                                CoordinatorFinding Facts, manages delivery state +
                                HITL gate lifecycle, emits SessionPush via SSE.
                                Built on STRENGTHENED runway-app-host (below), not
                                a new session-core crate (ADR 2026-06-26, Option 5).

runtime-runway/crates/
  runway-app-host/            UPSTREAM (strengthened, not a new crate): in-memory
                                hub auto-stamps sequence; SSE gains a replay+live
                                combinator. helm-coordination + helm-governed-jobs
                                migrate onto it, dropping duplicated copies.

mobile-apps/crates/
  helm-client-ffi/            UniFFI wrapper around helm-client; re-exports
                                director-contracts. thin — exposes Loop Registry
                                state, push callbacks, temperature submission, gate
                                response, and DirectorSnapshot to Swift/Kotlin
  mobile-core/                consumes/re-exports director-contracts (must NOT fork
                                DirectorFrame); owns the mobile snapshot envelope
```

**The wire boundary and the projection boundary are different crates on purpose.**
`helm-session-contracts` is what crosses the server↔client *wire*;
`director-contracts` is what crosses the Rust→*FFI/UI* seam (`DirectorFrame` and
its vocabulary). The dependency is one-way (`director-contracts` →
`helm-session-contracts`, for shared ids like `GateId`); the wire crate never
depends on the projection crate. `helm-client` *produces* a versioned
`DirectorFrame` (`version` = the upstream SSE `sequence`), filling domain copy via
a `DomainPresenter` seam so it stays domain-agnostic. See Plan 1
(`KB/08-roadmap/2026-06-26-spine-plan-1-contracts-helm-client.md`) and the AI
Director UX architecture (`KB/04-architecture/2026-06-27-ai-director-mobile-ux-architecture.md`).

**`helm-session-host` is built on a strengthened `runway-app-host`, not a new
`helm-session-core` crate** (ADR `decisions/2026-06-26-helm-session-host-vs-coordination`,
**Option 5 supersedes Option 4**). Generic event/SSE mechanics move *upstream* into
the owned `runway-app-host` (sequence stamping + replay/live SSE);
`helm-coordination` and `helm-governed-jobs` migrate to consume them rather than
carrying their own copies (a Tier-1+ refactor behind characterization tests, no
external-behavior change — see the upstream event/SSE consolidation plan,
`KB/08-roadmap/2026-06-26-spine-plan-upstream-event-sse.md`). That ships first and is
the one remaining prerequisite for Plan 2. The HITL gate concept stays in
`helm-governed-jobs` until `helm-session-host` makes it a real third consumer. The
HITL mechanism itself is settled — §1a: terminal HITL suggestor →
`RunResult::HitlPause` → `Engine::resume(GateDecision)`.

**`helm-client-ffi` placement** is deliberately portfolio-level
(`mobile-apps/crates/`), following the existing `mobile-apps/crates/shell-ffi`
precedent — not the per-app `apps/*/ffi` pattern (e.g. the current
`apps/marquee/quorum-sense/ffi`). It wraps the domain-agnostic `helm-client` and
re-exports `director-contracts`, so it belongs with the shared crates, not under a
single app.

### Extended (not forked)

```
marquee-apps/quorum-sense/
  quorum-evidence/            +alignment, +conviction dimensions in evidence topology
  quorum-domain/              +ParticipantTemperature signal type
  CoordinatorSuggestor        reads extended topology, emits CoordinatorFinding<QuorumPayload>
  SSE transport               +SessionPush, +GatedDecision event variants
  apps/desktop/               reoriented: individual amplification via helm-client
                              (not a web mirror of the server session)
```

### Unchanged

Converge **kernel**, Axiom, Mosaic, Runtime Runway, Commerce Rails are unchanged.
Organism gains a `Deliberation` formation template (long loop + terminal HITL
suggestor, §1a) that runs the existing Converge contract — a new template, **no
kernel change**. Mobile-apps M2–M4 milestones (FFI seam, consent, queue,
submission) are prerequisites. Client Helm builds on top of them.

### Build order

```
1. helm-session-contracts          (no deps — wire boundary)
   director-contracts              (depends on helm-session-contracts — projection boundary)
2. runway-app-host (upstream)      (strengthen: seq stamping + replay/live SSE; migrate
                                    helm-coordination + helm-governed-jobs onto it — ADR 2026-06-26, Option 5)
3. quorum-evidence / quorum-domain (extend topology + temperature type)
4. CoordinatorSuggestor            (reads topology, emits finding)
   helm-session-host               (built on strengthened runway-app-host; routes findings, emits push)
5. helm-client                     (consumes push, manages local formations, projects DirectorFrame)
6. helm-client-ffi                 (UniFFI — re-exports director-contracts; requires mobile M4 + step 5)
   desktop Tauri integration       (direct Rust dep on step 5)
```

---

## Section 6 — ExperienceStore and the Learning Loops

Session 24 should be better than session 1 — participants get fluent in the way of
working, and the system learns from prior engagements: individual activity,
tendency to agree, innovation / curiosity, and most importantly **group dynamics**.
The server-side converging loops capture these dimensions; Converge already
integrates an ExperienceStore (via `application-storage`), so capture is cheap from
session 1.

**Two distinct learning loops — do not conflate them:**

| Loop | Cadence | Feeds |
|---|---|---|
| **Within-session** | live, per cycle | the `CoordinatorSuggestor` and the live topology — fast coordination *now* |
| **Across-session** | slow, between sessions | formation/suggestor *configuration* and coordinator *priors* — via the ExperienceStore and, over time, `mosaic-extensions/mnemos-knowledge` |

**Start capturing now; enrich later.** Record the raw group-dynamics dimensions in
the ExperienceStore from session 1. `mnemos-knowledge` is not yet mature — so the
spine must **not depend on its maturity**. mnemos enriches the captured data later;
it is not on the critical path. Capturing the raw signal early is what makes the
moat start compounding.

**Governance boundary (Quorum policy, Tier 2).** Across-session learning profiles
individuals (who concedes, who dissents, who drives novelty). That is sensitive
data and a potential **anchoring weapon**: a coordinator tuned on "this person
usually concedes under pressure" could nudge harder and manufacture the very
groupthink the product exists to prevent. The boundary on what across-session
priors may feed back into *live* coordination is a deliberate Quorum policy
decision — not a default. Capture is allowed; weaponizing it against a
participant's own deliberation is not.

---

## Non-Goals

- Do not run a promotion-authoritative Converge formation on a mobile Marquee app
- Do not put Mosaic credentials or live specialist adapters on device
- Do not implement Stripe semantics on mobile (consume server entitlements only)
- Do not fork Quorum domain types for mobile
- Do not implement a separate coordination channel alongside the admission boundary —
  temperature flows through the same admission path as all other participant signals
- Do not build the SessionManifest until the first real app session is wired end-to-end
- Do not build the desktop reorientation before helm-client is stable
- Do not re-aim a running main loop's `RootIntent` — reframing is a new session, not loop drift (§1a)
- Do not run power-hungry compute on device — heavy work is a server sub-formation surfaced as a handle (§4)
- Do not make the spine depend on `mnemos-knowledge` maturity — capture raw dimensions now, enrich later (§6)
- Do not feed across-session participant profiles into live coordination without an explicit Quorum policy (§6)
- Do not design cross-app delegation now — `atlas://` composition is later, high-level integration
