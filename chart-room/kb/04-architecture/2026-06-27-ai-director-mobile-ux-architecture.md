# AI Director UX Architecture — Mobile-First Cross-App Foundation

- Date: 2026-06-27
- Status: Draft
- Scope: Helms, Client Helm, Mobile apps, Desktop shells, Quorum as first proving app
- Related: `2026-06-26-session-intelligence-spine-design.md`,
  `2026-06-26-spine-plan-1-contracts-helm-client.md`,
  `../mobile-apps/docs/adr/0001-native-swift-kotlin-shared-rust-core.md`,
  `../mobile-apps/docs/adr/0002-mobile-platform-boundary.md`,
  `../mobile-apps/docs/adr/0003-responsiveness-and-snapshot-consistency.md`

---

## Thesis

Reflective apps should not expose the user to the full machinery of sessions,
formations, participants, evidence, Business Truths, gates, tasks, and policies.

The product surface is an **AI Director** between system complexity and human
judgment. The Session Spine may contain thousands of objects; the user should
experience one thing:

> What should I do right now?

This is not a super-app UI. It is a director-led decision surface. The system
manages itself and asks the human only for irreducible human judgment.

## Product Principle

Every screen answers exactly one question.

Default screen scope is the innermost useful context:

1. Task
2. Local context
3. Session
4. Formation
5. Organization
6. Everything

Most enterprise software starts at "Everything." Reflective starts at "Task" and
offers escape hatches outward only when the user asks for more context.

## Ownership Split

The AI Director pattern is cross-app and cross-platform, so it must not be
trapped inside Quorum mobile. Quorum can move first and prove the pattern, but the
shape should be backportable into shared foundations.

### Helms / Client Helm: semantic director model

Helms owns **what this moment means**, not how it looks.

`DirectorFrame` is a canonical Helms contract, not a mobile-owned DTO. **Decided
(2026-06-27):** it lives in its own crate, `director-contracts`, the Rust→FFI/UI
*projection* boundary — distinct from `helm-session-contracts`, which is the
server↔client *wire* boundary. The split is real because the two boundaries are
different; `director-contracts` depends one-way on `helm-session-contracts` (for
shared ids like `GateId`), never the reverse. `helm-client` consumes wire inputs
(`SessionPush`, `GatedDecision`, `UrgencyIntent`, `SessionContext`) and **projects**
them into a versioned `DirectorFrame` via `director_snapshot(version, presenter)`.
Both crates and the projection seam are specified in Plan 1
(`KB/08-roadmap/2026-06-26-spine-plan-1-contracts-helm-client.md`, Tasks 1b + 5b).

Rust defines the headless semantic model in `director-contracts`:

- `DirectorFrame` — the current scene the user should see.
- `DirectorSnapshot` — `{ version, frame }`; `version` = upstream SSE `sequence`.
- `NowTask` — one task requiring human attention.
- `JudgmentPrompt` — a focused human judgment with bounded choices.
- `GatePrompt` — a render projection of `GatedDecision` / `GateCondition`. Verdicts
  are contract-backed (`GateVerdict` = approve / reject); a "later/defer" choice
  must enter the Helms gate contract first — never a UI-only option.
- `DirectorIntent` — the typed action the UI sends back (no invented strings).
- `ContextLevel` — task, local context, session, formation, organization, everything.
- `PresenceHint` — minimal awareness that other people are involved.

Because `helm-client` is domain-agnostic (payloads are opaque), the human *copy* in
a frame comes from the per-app FFI via the `DomainPresenter` seam: `helm-client`
owns frame *structure and lifecycle*, the app owns the *words*.
- `PrimaryAction` — the single action the UI should privilege.
- `SecondaryAction` — escape hatch / defer / ask for context.

Client Helm is the headless Rust state machine that translates session pushes,
gates, local formations, server handles, pending work, and sequence-ordered SSE
events into the current `DirectorFrame`.

The Director is a **projection**, not a new runtime actor. The "what should I do
right now" ranking comes from the existing spine concepts: server-emitted
`UrgencyIntent`, gates, session context, and the client-side `SeverityRouter`.
Do not stand up a parallel Director actor that duplicates the coordinator /
suggestor role.

Rust must stay UI-free:

- no SwiftUI assumptions
- no Kotlin Compose assumptions
- no Svelte/Tauri assumptions
- no typography
- no colors
- no animation
- no layout grids

### Mobile apps: shared mobile director module

Because Quorum mobile is the first proving ground, it is acceptable to build the
first implementation there and backport. But the target home should be a shared
module under `mobile-apps/`, following that repo's current architecture:

- Native SwiftUI and Kotlin/Compose own UI and platform services.
- Rust owns shared application core, orchestration, deterministic logic,
  persistence contracts, schemas, and portable preprocessing.
- UniFFI is the preferred Swift/Kotlin bridge.
- Real-time views are projected from immutable, versioned snapshots emitted by an
  off-main Rust core.

Suggested target split:

```text
mobile-apps/
  crates/
    mobile-core/                  consumes/re-exports Helms Director contracts;
                                  owns mobile snapshot envelope + fixtures
    mobile-ai/                    native/platform AI routing policy
    shell-ffi/                    existing portfolio UniFFI shell facade

  schemas/
    quorum-mobile.udl             current Quorum UniFFI contract
    director-frame.udl            only if needed as a generated binding facade;
                                  canonical Rust type remains in Helms contracts

  apps/
    marquee/
      quorum-sense/
        ffi/                      Quorum-specific Rust UniFFI facade
        fixtures/                 DirectorFrame / field-signal golden fixtures
        ios/
          Views/                  Quorum SwiftUI composition + reusable director views
          CoreBridge/             async snapshot bridge to Rust / FFI
          PlatformAI/             Apple-native AI preprocessing
        android/
          app/                    Quorum Compose shell + reusable director screens
    studio/
      inkling-notes/
      wolfgang-chat/
```

The architectural rule:

- Helms contracts own the canonical `DirectorFrame` / prompt / action vocabulary.
- `helm-client` owns the projection from spine events into `DirectorFrame`.
- `mobile-core` consumes or re-exports the Helms director contracts and owns
  mobile-specific snapshot envelopes, replay harnesses, and fixtures. It must not
  define a parallel DirectorFrame shape.
- per-app `ffi/` crates map app/domain state to the shared Director model and
  expose it through UniFFI.
- iOS/Android render native screens from snapshots; they do not compute session
  semantics.
- Quorum can prove the first SwiftUI/Compose components locally under
  `apps/marquee/quorum-sense/ios/Views` and `android/app`; once two or more
  screens stabilize, lift generic components into a shared native module or
  template area under `mobile-apps` rather than forking them per app.

### Swift: iOS presentation

Swift owns the native iOS experience:

- SwiftUI screens and components
- iOS navigation and sheet behavior
- haptics
- Dynamic Type
- VoiceOver semantics
- iOS permission / interruption affordances
- local notification presentation

Swift should consume immutable `DirectorFrame` snapshots and render them using
native components. It should not re-implement session routing, gate state, or
formation lifecycle. The bridge must respect mobile ADR 0003: no synchronous FFI
call on the main actor; UI sends intents and receives snapshots through
`AsyncStream`.

Swift components should be generic:

- `DirectorNowView`
- `DirectorTaskCard`
- `JudgmentPromptView`
- `GatePromptView`
- `ContextTrailView`
- `PresenceStripView`

Quorum-specific views wrap these components with app-specific copy and data
mapping, not bespoke layouts for every flow.

### Kotlin: Android presentation

Kotlin owns the native Android experience:

- Jetpack Compose components
- Android navigation and back behavior
- Material-compatible interaction affordances where appropriate
- TalkBack semantics
- local notification presentation

Kotlin consumes the same `DirectorFrame` FFI model as Swift. It should not invent
parallel semantic state. The bridge must mirror ADR 0003: UI sends intents and
receives snapshots through a `Flow`; no blocking core calls on the UI thread.

Compose components should mirror Swift at the semantic level:

- `DirectorNowScreen`
- `DirectorTaskCard`
- `JudgmentPrompt`
- `GatePrompt`
- `ContextTrail`
- `PresenceStrip`

The components do not need pixel parity with iOS. They need semantic parity.

### Desktop: Svelte/Tauri presentation

Desktop can show a little more context than mobile, but it must obey the same
director principle. More screen space is not permission to become a dashboard.

Svelte/Tauri owns:

- desktop layout
- keyboard affordances
- side-by-side context when useful
- desktop notification style

Desktop still consumes the same semantic model. It may show the current task and
one adjacent context panel, but the primary question remains:

> What should I do right now?

## Shared Types

**These are now concrete**, defined in `director-contracts` and specified in Plan 1
(`KB/08-roadmap/2026-06-26-spine-plan-1-contracts-helm-client.md`, Task 1b). The
sketch below shows the canonical shape; treat Plan 1 as the source of truth.

```rust
pub struct DirectorFrame {
    pub frame_id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub now: Option<NowTask>,          // objective / needed_from_user / estimated_minutes
    pub waiting_for: WaitingFor,
    pub primary: PrimaryAction,
    pub secondary: Vec<SecondaryAction>,
    pub prompt: Option<DirectorPrompt>,
    pub presence: Vec<PresenceHint>,
    pub context_trail: Vec<ContextLevel>,
    pub blocking: BlockingState,
}

pub enum DirectorPrompt {
    Judgment(JudgmentPrompt),
    Gate(GatePrompt),
    Review(ReviewPrompt),
}

pub struct JudgmentPrompt {
    pub question: String,
    pub body: String,
    pub choices: Vec<Choice>, // ≤3 for mobile-first director surfaces
}

// Render projection of helm_session_contracts::GatedDecision / GateCondition.
// gate_id correlates the user's DirectorIntent::RespondGate back to the gate.
pub struct GatePrompt {
    pub gate_id: GateId,           // from helm-session-contracts
    pub reason: String,
    pub consequence: String,
    pub deadline_ms: Option<u64>,
    pub condition: GateCondition,  // from helm-session-contracts
}

// Verdicts are contract-backed only. No "later/defer" — adding it requires a
// Helms gate-contract change first; it must never be a UI-only choice.
pub enum GateVerdict { Approve, Reject }

// The typed action the UI sends back — never an invented string.
pub enum DirectorIntent {
    OpenTask { frame_id: String },
    SubmitJudgment { frame_id: String, choice_id: String },
    RespondGate { gate_id: GateId, verdict: GateVerdict },
    SubmitReview { frame_id: String, stance: ReviewStance },
    RequestContext { level: ContextLevel },
}

pub enum BlockingState {
    NotBlocking,
    BlocksFormation,
    BlocksSession,
}
```

The mobile UI should not receive raw session internals and decide what matters.
That decision belongs in `helm-client`, which projects ordered wire state into a
`DirectorFrame` (filling domain copy via the `DomainPresenter` seam, since
payloads are opaque to it). In `mobile-apps`, the FFI-facing version is the
immutable, versioned `DirectorSnapshot` that carries the `DirectorFrame`, not a
mutable object graph read by Swift or Kotlin.

Snapshot versioning derives from the upstream event sequence (the
`runway-app-host` SSE sequence consumed by `helm-client`), not from a new mobile
counter. The chain is:

```text
runway-app-host SSE sequence
  -> helm-client off-main projection
  -> DirectorFrame snapshot
  -> UniFFI observer
  -> Swift AsyncStream / Kotlin Flow
  -> SwiftUI / Compose
```

Primary and secondary actions carry typed intent tokens from the Helms director
contract, not free-form UI strings. Native UI sends those typed intents back to
the bridge; it does not invent action vocabulary.

`GatePrompt` is the render projection of `GatedDecision` / `GateCondition`, not a
second gate model. If "Later" / defer is a real gate response, it must be added
to the contract vocabulary; it cannot exist only as a UI-only choice.

## Quorum Mobile as Proving App

Quorum mobile should implement the first version pragmatically:

1. Start inside the Quorum app if needed.
2. Keep the view model shape close to the future Helms `DirectorFrame`.
3. Avoid Quorum-only naming in reusable components.
4. Keep domain-specific words in mapping code, not component APIs.
5. Put mobile snapshot envelopes / replay harnesses in
   `mobile-apps/crates/mobile-core` once the shape stabilizes, but keep the
   canonical `DirectorFrame` type in Helms contracts.
6. Keep Quorum-specific FFI mapping in `apps/marquee/quorum-sense/ffi`.
7. Backport native components from `apps/marquee/quorum-sense/ios/Views` and
   `android/app` into a shared native module/template only after at least two
   director screens prove the pattern.

Example Quorum-specific mapping:

```text
CoordinatorFinding + SessionContext + UserRole
  -> helm-client projection
  -> Helms DirectorFrame
  -> immutable mobile snapshot via Quorum ffi / mobile-core
  -> SwiftUI / Compose Director screen
```

Quorum may know what a hypothesis, evidence gap, or dissent cluster is. The
generic director UI should not.

## First Screen Set

### 1. Morning Director

Purpose: show what needs the user today.

```text
Good morning Kenneth.

The procurement formation has reached
a decision checkpoint.

Two things need you today.

1
Review pricing anomaly
3 min

2
Approve legal wording
1 min

[Start]
```

### 2. Single Task

Purpose: focus one contribution.

```text
Current objective
Evaluate Vendor X's security claims

Needed from you
Review the encryption section

Estimated time
4 minutes

Waiting for
Nobody

[Open Review]
```

### 3. Focused Judgment

Purpose: capture one irreducible human signal.

```text
Maria needs your opinion.

Vendor X claims customer data is
encrypted at rest and in transit.

Do you accept this claim?

○ Yes
○ No
○ Unsure

[Submit]
```

### 4. Gate Interrupt

Purpose: make blocking explicit.

```text
Cannot continue

Legal approval required before the
formation can claim success.

Decision needed
Approve revised liability wording

Deadline
Today, 16:00

[Approve]
[Reject]
```

If deferral becomes a real gate response, it must be added to the Helms contract
and then may appear here. Until then, a gate prompt renders only contract-backed
choices.

### 5. Context Escape

Purpose: let the user expand context without starting there.

```text
Review pricing anomaly

This looks inconsistent with the
approved benchmark range.

Primary evidence
Vendor quote is 18% above comparable
Nordic enterprise contracts.

[Agree]
[Disagree]
[Need more context]

Context
Task > Session > Formation > Organization
```

## Boundary Rules

- Rust decides state and meaning.
- Swift/Kotlin/Svelte decide native rendering and interaction feel.
- App code maps domain concepts into director concepts.
- Shared mobile UI renders director concepts without domain knowledge.
- Quorum can move first, but reusable pieces must be named as director components,
  not Quorum components.
- The user never manages formations, queues, sessions, gates, or evidence stores
  directly from the primary UI.

## Open Decisions

1. Whether mobile shared UI is built as parallel Swift/Kotlin modules or generated
   from a shared schema plus native implementations.
2. How much desktop context is allowed before it violates the Now Principle.

