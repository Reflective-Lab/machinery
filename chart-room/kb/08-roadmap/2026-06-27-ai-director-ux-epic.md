# EPIC — AI Director UX

- Date: 2026-06-27
- Status: Active direction
- Scope: Mobile first; desktop Svelte/Tauri and web app follow the same semantic model
- Reference app: `mobile-apps/apps/marquee/quorum-sense/`
- Architecture source: `KB/04-architecture/2026-06-27-ai-director-mobile-ux-architecture.md`

## Outcome

Reflective product surfaces stop behaving like dashboards and start behaving like
an AI Director:

> The system manages itself and asks the human only for the irreducible judgment
> needed now.

The user should not navigate formations, evidence stores, Business Truths,
queues, participants, gates, or timelines as the primary experience. The product
compresses that complexity into one scene, one question, and one primary action.

## Product Doctrine

Every primary screen answers:

> What should I do right now?

The AI Director continuously edits the scene:

- Cut to the relevant task.
- Show enough context to decide.
- Offer at most a small number of choices.
- Make blocking gates explicit.
- Keep presence visible but quiet.
- Let the user expand context only when needed.

This is not a super-app. It is a reduction engine.

## Platform Ownership

### Rust / Helms / Client Helm

Owns the canonical director contract and projection:

- current objective
- current task
- gate / blocking state
- allowed choices
- presence hints
- context escape levels
- immutable director snapshots

`director-contracts` owns the single `DirectorFrame` definition as the Rust→FFI/UI
projection boundary. `helm-client` projects `SessionPush`, `GatedDecision`,
urgency, loop registry state, server handles, and ordered SSE events into that
frame. Rust does not own native layout, typography, animation, haptics, or visual
style.

### `mobile-apps/crates/mobile-core`

Consumes or re-exports the Helms director contract. Owns mobile-facing immutable
snapshot envelopes, fixture replay harnesses, and mobile-specific adapters. It
must not define a parallel `DirectorFrame`.

### Quorum mobile

Proves the first real implementation:

- `apps/marquee/quorum-sense/ffi/` maps Quorum / Helms state into director snapshots.
- `apps/marquee/quorum-sense/ios/Views` renders SwiftUI director screens.
- `apps/marquee/quorum-sense/android/app` renders Compose director screens.
- `fixtures/` carries canonical spine input event streams and derived golden
  director frames for Rust, Swift, and Kotlin parity.

### Desktop Svelte/Tauri

Consumes the same semantic director model. Desktop may expose one adjacent context
panel, but the primary screen still starts with the Now task, not a tree,
dashboard, or admin console.

### Web App

Consumes the same semantic model for thin session surfaces. Web can be good at
review links, external participants, and low-friction gates, but should not fork
the director semantics.

## Inspiration To Borrow

- Apple: progressive disclosure, calm choices, max three primary options.
- Linear: one primary action and clean state transitions.
- Figma: minimal presence, confidence that others are here.
- Slido: focused participation, one question at a time.
- GitHub: explicit blocking gates and status checks.
- Uber: stage-based journey clarity and "what happens next."

The common thread is reduction, not collaboration chrome.

## Epic Milestones

### UX0 — Director Doctrine And Screen Inventory

Create the shared UX language and the first screen set.

Deliverables:

- AI Director UX guide in `KB/04-architecture/`.
- screen inventory for mobile, desktop, and web.
- borrow/avoid reference list.
- first five screen drafts:
  - Morning Director
  - Single Task
  - Focused Judgment
  - Gate Interrupt
  - Context Escape

Success: every product conversation can refer to the same vocabulary:
`DirectorFrame`, Now task, context escape, gate interrupt, focused judgment.

### UX1 — Quorum Mobile Director Slice

Build the first Quorum mobile slice around the Now Principle.

Deliverables:

- canonical spine event fixture plus derived `DirectorFrame` fixture for a Quorum
  decision checkpoint.
- iOS SwiftUI prototype for:
  - Morning Director
  - Single Task
  - Focused Judgment
  - Gate Interrupt
- Quorum FFI or preview bridge emits immutable director snapshots.
- UI sends intents, receives snapshots; no blocking main-thread FFI.

Success: Quorum mobile can show one live decision task and capture one judgment
without exposing session internals.

### UX2 — Shared Mobile Consumption Contract

Promote the proven director consumption shape into shared mobile foundations
without forking the canonical Helms type.

Deliverables:

- `mobile-core` snapshot envelope that carries Helms `DirectorFrame`.
- golden fixture replay tests: canonical spine event stream → `helm-client`
  projection → `DirectorFrame`.
- typed `DirectorIntent` inputs that map to Helms/client action vocabulary.
- UniFFI-compatible contract for Swift and Kotlin.

Success: a second mobile app can consume the Helms director contract without
copying Quorum-specific types or defining a second DirectorFrame.

### UX3 — Android Parity

Bring the same director semantics to Kotlin/Compose.

Deliverables:

- Compose Now screen.
- Compose focused judgment screen.
- Compose gate interrupt screen.
- Kotlin typed mapping from generated FFI values.
- Flow-based snapshot stream.

Success: Android has semantic parity with iOS, even if the platform presentation
differs.

### UX4 — Desktop Director Shell

Apply the same model to Svelte/Tauri.

Deliverables:

- desktop Now surface.
- optional adjacent context panel.
- keyboard-first focused judgment.
- gate interrupt surface.

Success: desktop benefits from more space without regressing into a dashboard.

### UX5 — Web Director Surface

Apply the same model to thin web participation.

Deliverables:

- one-link review / gate page.
- focused participation page.
- presence and status hints.
- no admin/navigation chrome in the primary flow.

Success: external or lightweight participants can act without learning the full
system.

### UX6 — Design System Hardening

Turn the successful patterns into reusable standards.

Deliverables:

- Director component naming rules.
- copy rules: short, active, single-action.
- max-choice rules.
- context escape rules.
- accessibility rules for SwiftUI, Compose, Svelte, and web.
- screenshot/reference board.

Success: new app surfaces inherit a strong UX floor before styling begins.

## Near-Term Development Plan

Start in Quorum mobile, but keep the shape shared:

1. Add canonical spine input fixture(s) plus derived `DirectorFrame` fixture(s) under
   `mobile-apps/apps/marquee/quorum-sense/fixtures/`.
2. Add SwiftUI `DirectorNowView`, `DirectorTaskCard`, `JudgmentPromptView`, and
   `GatePromptView` under `ios/Views`.
3. Wire those views to the existing preview bridge first.
4. Add `mobile-core` Rust snapshot envelope / replay harness once the fixture
   shape survives the first two screens; keep canonical DirectorFrame in Helms.
5. Add FFI snapshot output after the Rust shape is stable.
6. Mirror in Compose after iOS proves the interaction shape.
7. Port the same semantic model to Svelte/Tauri and web.

## Non-Goals

- Do not build a general enterprise dashboard.
- Do not start with app navigation, project trees, reports, or admin surfaces.
- Do not expose raw session internals to the mobile UI.
- Do not fork Quorum domain concepts into Swift/Kotlin.
- Do not build shared Swift/Kotlin frameworks before real repetition proves the
  abstraction.

## Definition Of Done

The epic is mature when:

- Quorum iOS ships a director-led decision task.
- Quorum Android reaches semantic parity.
- Helms owns the canonical DirectorFrame; `mobile-core` owns reusable mobile
  snapshot envelopes and fixtures.
- Desktop and web consume the same semantic model.
- app-specific UI maps domain state into director frames rather than inventing
  its own workflow chrome.
- users mostly live at the Task context level and only expand outward by choice.

