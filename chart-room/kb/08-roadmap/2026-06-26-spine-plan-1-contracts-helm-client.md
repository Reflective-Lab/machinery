# Session Intelligence Spine — Plan 1: Shared Contracts + Client Helm Core

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create three new Rust crates in `bedrock-platform/helms` — `helm-session-contracts` (server↔client wire types), `director-contracts` (the Rust→FFI/UI projection boundary: `DirectorFrame` + prompt/action vocabulary), and `helm-client` (headless coordination logic + the director projection) — forming the domain-agnostic foundation for the Session Intelligence Spine and the AI Director UX (`KB/04-architecture/2026-06-27-ai-director-mobile-ux-architecture.md`).

**Architecture:** `helm-session-contracts` holds every type that crosses the **server↔client wire** boundary: `UrgencyIntent`, `CoordinatorFinding<P>`, `SessionPush`, `GatedDecision`, `GateCondition`. `director-contracts` holds every type that crosses the **Rust→FFI/UI** boundary: `DirectorFrame`, `DirectorPrompt`, `DirectorIntent`, etc. — a *different* boundary, so it is a distinct crate (it depends on `helm-session-contracts` for shared ids like `GateId`; never the reverse). `helm-client` is a synchronous, pure-Rust coordination library — no network, no async, no Converge deps. The native layer (iOS, Android, Desktop) owns the SSE connection and feeds events in; `helm-client` returns typed actions and **projects** ordered session/gate/loop state into a versioned `DirectorFrame`. Because `helm-client` is domain-agnostic (payloads are opaque), the human-readable domain copy in a frame is supplied by the per-app FFI via a `DomainPresenter` seam — `helm-client` owns the frame *structure and lifecycle*, the app owns the *words*.

**Tech Stack:** Rust 1.96.0, `serde`/`serde_json`/`uuid`/`thiserror` (all workspace deps in `bedrock-platform/helms/Cargo.toml`).

## Global Constraints

- Rust toolchain: `1.96.0` — pinned in `bedrock-platform/helms/rust-toolchain.toml`
- Crate manifest style: copy `helm-module-contracts/Cargo.toml` — `version.workspace`, `edition.workspace`, `license.workspace`, `publish.workspace`
- `helm-session-contracts`: zero Converge deps, zero network deps
- `director-contracts`: zero Converge deps, zero network deps; pure serde types; depends only on `helm-session-contracts` (for shared ids). The wire crate must **never** depend on it (one-way: projection → wire).
- `helm-client`: zero async, zero network, zero Converge deps — synchronous library only
- No `unwrap()` / `expect()` in library code; propagate errors with `thiserror`
- `#[must_use]` on all constructors and methods returning a non-trivial value
- Id newtypes (`FindingId`, `GateId`, `LoopId`) name their string constructor `from_string`, **not** `from_str` — an inherent `from_str` trips `clippy::should_implement_trait` under `-D warnings` (verified 2026-06-27 building Task 1 + 1b)
- Every task ends with: `cd bedrock-platform/helms && cargo test --workspace --locked && cargo clippy --workspace -- -D warnings`
- Copyright header on every new `.rs` file: `// Copyright 2024-2026 Reflective Labs\n// SPDX-License-Identifier: MIT`
- Spec reference: `KB/04-architecture/2026-06-26-session-intelligence-spine-design.md`

---

## File Map

```
bedrock-platform/helms/
  Cargo.toml                           MODIFY — add three workspace members

  crates/helm-session-contracts/       CREATE (server↔client wire boundary)
    Cargo.toml
    src/
      lib.rs
      urgency.rs                       UrgencyIntent enum + Display
      finding.rs                       CoordinatorFinding<P>, FindingType, FindingId
      push.rs                          SessionPush, SessionContext
      gate.rs                          GatedDecision, GateCondition, GateId

  crates/director-contracts/           CREATE (Rust→FFI/UI projection boundary)
    Cargo.toml
    src/
      lib.rs
      frame.rs                         DirectorFrame, DirectorSnapshot, NowTask, WaitingFor, BlockingState
      prompt.rs                        DirectorPrompt, JudgmentPrompt, GatePrompt, ReviewPrompt, Choice
      action.rs                        PrimaryAction, SecondaryAction, DirectorIntent, GateVerdict, ReviewStance
      context.rs                       ContextLevel, PresenceHint

  crates/helm-client/                  CREATE
    Cargo.toml
    src/
      lib.rs
      ids.rs                           LoopId (newtype uuid)
      formation.rs                     SeedContext, FormationOutput, TemperatureReading, LocalFormationIntent
      registry.rs                      LoopRegistry, LoopEntry, LoopKind, LoopState, LoopEntryView
      router.rs                        SeverityRouter, RoutingDecision
      temperature.rs                   TemperatureQueue, PendingSubmission, TemperatureSignal
      gate_surface.rs                  GatedDecisionSurface, GatedDecisionView, PendingGateResponse
      budget.rs                        WallClockGuard — per-loop wall-clock budget (engine does not enforce time_limit)
      director.rs                      DomainPresenter trait + project state → DirectorSnapshot
      client.rs                        ClientHelm, ClientHelmAction
    tests/
      registry_tests.rs
      router_tests.rs
      client_integration_tests.rs
      director_tests.rs
```

---

### Task 1: `helm-session-contracts` — boundary types

**Files:**
- Create: `bedrock-platform/helms/crates/helm-session-contracts/Cargo.toml`
- Create: `bedrock-platform/helms/crates/helm-session-contracts/src/lib.rs`
- Create: `bedrock-platform/helms/crates/helm-session-contracts/src/urgency.rs`
- Create: `bedrock-platform/helms/crates/helm-session-contracts/src/finding.rs`
- Create: `bedrock-platform/helms/crates/helm-session-contracts/src/push.rs`
- Create: `bedrock-platform/helms/crates/helm-session-contracts/src/gate.rs`

**Interfaces:**
- Produces: `UrgencyIntent`, `CoordinatorFinding<P>`, `FindingType`, `FindingId`, `SessionPush`, `SessionContext`, `GatedDecision`, `GateCondition`, `GateId` — all `pub`, `Serialize`/`Deserialize`, `Clone`, `Debug`

- [ ] **Step 1: Create `Cargo.toml`**

```toml
# bedrock-platform/helms/crates/helm-session-contracts/Cargo.toml
[package]
name = "helm-session-contracts"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
thiserror.workspace = true
uuid = { workspace = true, features = ["serde", "v4"] }
```

- [ ] **Step 2: Write failing tests first**

```rust
// bedrock-platform/helms/crates/helm-session-contracts/src/lib.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

pub mod finding;
pub mod gate;
pub mod push;
pub mod urgency;

pub use finding::{CoordinatorFinding, FindingId, FindingType};
pub use gate::{GateCondition, GateId, GatedDecision};
pub use push::{SessionContext, SessionPush};
pub use urgency::UrgencyIntent;

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn urgency_intent_round_trips() {
        let variants = [
            UrgencyIntent::Informational,
            UrgencyIntent::Advisory,
            UrgencyIntent::Disruptive,
            UrgencyIntent::Preemptive,
        ];
        for v in variants {
            let json = serde_json::to_string(&v).unwrap();
            let back: UrgencyIntent = serde_json::from_str(&json).unwrap();
            assert_eq!(format!("{v:?}"), format!("{back:?}"));
        }
    }

    #[test]
    fn coordinator_finding_serializes_opaque_payload() {
        let finding = CoordinatorFinding {
            finding_id: FindingId::new(),
            finding_type: FindingType::HighConvictionDissent,
            payload: serde_json::json!({"hypothesis_id": "h-1", "dissent_count": 3}),
            urgency_intent: UrgencyIntent::Preemptive,
            requires_human: false,
            target_participants: vec!["alice".into(), "bob".into()],
        };
        let json = serde_json::to_string(&finding).unwrap();
        let back: CoordinatorFinding<serde_json::Value> =
            serde_json::from_str(&json).unwrap();
        assert_eq!(back.urgency_intent, finding.urgency_intent);
        assert_eq!(back.target_participants.len(), 2);
    }

    #[test]
    fn gated_decision_with_no_deadline() {
        let gate = GatedDecision {
            gate_id: GateId::new(),
            condition: GateCondition::AnyParticipant,
            payload: serde_json::json!({}),
            deadline: None,
        };
        let json = serde_json::to_string(&gate).unwrap();
        let back: GatedDecision = serde_json::from_str(&json).unwrap();
        assert!(back.deadline.is_none());
    }

    #[test]
    fn gate_condition_quorum_of_roles_round_trips() {
        let cond = GateCondition::QuorumOfRoles {
            roles: vec!["facilitator".into(), "lead".into()],
        };
        let json = serde_json::to_string(&cond).unwrap();
        let back: GateCondition = serde_json::from_str(&json).unwrap();
        match back {
            GateCondition::QuorumOfRoles { roles } => assert_eq!(roles.len(), 2),
            _ => panic!("wrong variant"),
        }
    }

    #[test]
    fn session_push_preserves_opaque_payload() {
        let push = SessionPush {
            finding_id: FindingId::new(),
            urgency_intent: UrgencyIntent::Disruptive,
            payload: serde_json::json!({"msg": "contradiction detected"}),
            session_context: SessionContext {
                session_id: "sess-1".into(),
                phase: "hypothesis".into(),
                cycle: 3,
                timestamp_ms: 1_700_000_000_000,
            },
        };
        let json = serde_json::to_string(&push).unwrap();
        let back: SessionPush = serde_json::from_str(&json).unwrap();
        assert_eq!(back.session_context.cycle, 3);
        assert_eq!(back.urgency_intent, UrgencyIntent::Disruptive);
    }
}
```

- [ ] **Step 3: Run tests — expect compile failure (types not defined yet)**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo test -p helm-session-contracts 2>&1 | head -20
```

Expected: `error[E0432]: unresolved import` or similar.

- [ ] **Step 4: Implement `urgency.rs`**

```rust
// bedrock-platform/helms/crates/helm-session-contracts/src/urgency.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// How urgently the server coordinator wants a participant to respond.
///
/// Derived by CoordinatorSuggestor from evidence topology changes.
/// Never assigned by Helms — always passed through from the promoted Fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UrgencyIntent {
    /// FYI — no action required; client app may surface as ambient state.
    Informational,
    /// Worth attention when convenient; do not interrupt active work.
    Advisory,
    /// Spawn a parallel local formation; surface prominently.
    Disruptive,
    /// Suspend the active local formation; on accept, spawn a FRESH formation
    /// seeded with its accumulated context + this context. Not a Converge resume.
    Preemptive,
}
```

- [ ] **Step 5: Implement `finding.rs`**

```rust
// bedrock-platform/helms/crates/helm-session-contracts/src/finding.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use crate::urgency::UrgencyIntent;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Opaque identifier for a coordinator finding.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct FindingId(String);

impl FindingId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    #[must_use]
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for FindingId {
    fn default() -> Self {
        Self::new()
    }
}

/// The class of finding the coordinator detected.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FindingType {
    ContradictionDetected,
    ConsensusEmerging,
    HighConvictionDissent,
    EvidenceGap,
    HypothesisReady,
    UncertaintyCluster,
}

/// A finding from the CoordinatorSuggestor, after promotion to Fact.
///
/// `P` is the domain payload — opaque to Helms, rendered by the client app.
/// Helms routes this; it never inspects `payload`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatorFinding<P> {
    pub finding_id: FindingId,
    pub finding_type: FindingType,
    /// Domain-specific payload — opaque to Helms and Client Helm.
    pub payload: P,
    /// Derived from evidence topology by CoordinatorSuggestor. Never set by Helms.
    pub urgency_intent: UrgencyIntent,
    /// When true, this finding paused the main formation (requires_human = true).
    pub requires_human: bool,
    /// ActorIds of participants who should receive this finding.
    pub target_participants: Vec<String>,
}
```

- [ ] **Step 6: Implement `push.rs`**

```rust
// bedrock-platform/helms/crates/helm-session-contracts/src/push.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use crate::finding::FindingId;
use crate::urgency::UrgencyIntent;
use serde::{Deserialize, Serialize};

/// Session-level context appended by Helms before routing a push.
/// Helms owns this; the domain coordinator does not set it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionContext {
    pub session_id: String,
    pub phase: String,
    pub cycle: u32,
    pub timestamp_ms: u64,
}

/// What Client Helm receives via SSE from the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionPush {
    pub finding_id: FindingId,
    /// Passed through unchanged from the CoordinatorFinding.
    pub urgency_intent: UrgencyIntent,
    /// Domain payload — opaque. The client app renders it; Client Helm routes it.
    pub payload: serde_json::Value,
    pub session_context: SessionContext,
}
```

- [ ] **Step 7: Implement `gate.rs`**

```rust
// bedrock-platform/helms/crates/helm-session-contracts/src/gate.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Opaque identifier for a HITL gate.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GateId(String);

impl GateId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    #[must_use]
    pub fn from_string(s: impl Into<String>) -> Self {
        Self(s.into())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for GateId {
    fn default() -> Self {
        Self::new()
    }
}

/// Condition that must be satisfied before the main formation resumes.
///
/// Compiled from Axiom Truth / Gherkin definitions. These variants are
/// the runtime representation of Axiom-generated validators.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum GateCondition {
    QuorumOfRoles { roles: Vec<String> },
    SpecificAuthority { actor_id: String },
    AnyParticipant,
    Unanimous,
}

/// A HITL gate event sent from Server Session Helm when requires_human = true.
///
/// The main formation is already paused at a Converge `RunResult::HitlPause`.
/// The user's response is sent to the server, which delivers it as a
/// `GateDecision` to `Engine::resume` (approve promotes the held proposal,
/// reject discards it). This is a verdict on the paused proposal — Client Helm
/// does not resume the formation directly, and this is NOT the admission path.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatedDecision {
    pub gate_id: GateId,
    pub condition: GateCondition,
    /// Domain payload — opaque. The client app renders it.
    pub payload: serde_json::Value,
    /// Unix timestamp ms — None means no deadline.
    pub deadline: Option<u64>,
}
```

- [ ] **Step 8: Run tests — expect pass**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo test -p helm-session-contracts --locked
```

Expected: all 5 tests pass.

- [ ] **Step 9: Clippy clean**

```bash
cargo clippy -p helm-session-contracts -- -D warnings
```

Expected: no warnings.

- [ ] **Step 10: Commit**

```bash
git add crates/helm-session-contracts/
git commit -m "feat(helms): add helm-session-contracts boundary types

Adds UrgencyIntent, CoordinatorFinding<P>, FindingType, FindingId,
SessionPush, SessionContext, GatedDecision, GateCondition, GateId.

These are the shared types that cross the server Session Helm ↔
Client Helm boundary. Zero Converge deps; zero network deps.

Part of: Session Intelligence Spine Plan 1."
```

---

### Task 1b: `director-contracts` — the projection boundary (`DirectorFrame`)

**Why:** The AI Director UX (`KB/04-architecture/2026-06-27-ai-director-mobile-ux-architecture.md`)
needs a single canonical `DirectorFrame` that `helm-client` *produces* and Swift /
Kotlin / Svelte *render*. It crosses the **Rust→FFI/UI** boundary — a different
boundary than `helm-session-contracts` (server↔client wire) — so it is its own
crate. `mobile-core` (in the `mobile-apps` repo) **consumes/re-exports** these
types and must **not** define a parallel `DirectorFrame`. This is the Helms-side
anchor for mobile milestone M3A.8.

**Files:**
- Create: `bedrock-platform/helms/crates/director-contracts/Cargo.toml`
- Create: `bedrock-platform/helms/crates/director-contracts/src/{lib,frame,prompt,action,context}.rs`

**Interfaces:**
- Consumes: `helm-session-contracts::{GateId, GateCondition}` (shared ids/condition).
- Produces: `DirectorFrame`, `DirectorSnapshot`, `NowTask`, `WaitingFor`,
  `BlockingState`, `DirectorPrompt`, `JudgmentPrompt`, `GatePrompt`, `ReviewPrompt`,
  `Choice`, `PrimaryAction`, `SecondaryAction`, `DirectorIntent`, `GateVerdict`,
  `ReviewStance`, `ContextLevel`, `PresenceHint` — all `pub`, `Serialize`/`Deserialize`, `Clone`, `Debug`.

- [ ] **Step 1: `Cargo.toml`**

```toml
# bedrock-platform/helms/crates/director-contracts/Cargo.toml
[package]
name = "director-contracts"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
helm-session-contracts = { path = "../helm-session-contracts" }
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
uuid = { workspace = true, features = ["serde", "v4"] }
```

- [ ] **Step 2: `context.rs`**

```rust
// bedrock-platform/helms/crates/director-contracts/src/context.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// How far out the user has chosen to look. Default surface is `Task`; the user
/// escapes outward only on request (the Now Principle).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextLevel {
    Task,
    LocalContext,
    Session,
    Formation,
    Organization,
    Everything,
}

/// Minimal "someone else is here" awareness. Opaque labels — the renderer does
/// not interpret them as identities.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceHint {
    pub actor_label: String,
    /// e.g. "viewing" | "deciding" | "away" — an opaque status label.
    pub status: String,
}
```

- [ ] **Step 3: `action.rs`**

```rust
// bedrock-platform/helms/crates/director-contracts/src/action.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use crate::context::ContextLevel;
use helm_session_contracts::gate::GateId;
use serde::{Deserialize, Serialize};

/// A gate verdict the human can return. **Intentionally `Approve` / `Reject`
/// only** — it mirrors `helm_session_contracts::GatedDecision` today. A
/// "defer / later" verdict must be added to the Helms gate contract FIRST; it
/// must never exist as a UI-only choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GateVerdict {
    Approve,
    Reject,
}

/// Stance on a focused evidence review. Maps to a temperature signal server-side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewStance {
    Agree,
    Disagree,
    NeedMoreContext,
}

/// The typed intent the native UI sends back. Every interactive director surface
/// maps a user choice to exactly one of these — the UI never invents action
/// strings, and there is no verdict here the Helms contracts cannot honor.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DirectorIntent {
    OpenTask { frame_id: String },
    SubmitJudgment { frame_id: String, choice_id: String },
    RespondGate { gate_id: GateId, verdict: GateVerdict },
    SubmitReview { frame_id: String, stance: ReviewStance },
    RequestContext { level: ContextLevel },
}

/// The single privileged action the UI should offer.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimaryAction {
    pub label: String,
    pub intent: DirectorIntent,
}

/// An escape hatch / secondary affordance.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecondaryAction {
    pub label: String,
    pub intent: DirectorIntent,
}
```

- [ ] **Step 4: `prompt.rs`**

```rust
// bedrock-platform/helms/crates/director-contracts/src/prompt.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use helm_session_contracts::gate::{GateCondition, GateId};
use serde::{Deserialize, Serialize};

/// One bounded choice in a prompt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Choice {
    pub choice_id: String,
    pub label: String,
}

/// A focused human judgment with bounded choices (≤3 on mobile-first surfaces).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JudgmentPrompt {
    pub question: String,
    pub body: String,
    pub choices: Vec<Choice>,
}

/// Render projection of `helm_session_contracts::GatedDecision` / `GateCondition`
/// — **not** a second gate model. `gate_id` correlates the user's
/// `DirectorIntent::RespondGate` back to the originating gate. Renderable
/// choices are limited to contract-backed verdicts (`GateVerdict`); there is no
/// "later" until the Helms gate contract gains it.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatePrompt {
    pub gate_id: GateId,
    pub reason: String,
    pub consequence: String,
    pub deadline_ms: Option<u64>,
    pub condition: GateCondition,
}

/// A focused evidence review. Resolves to `DirectorIntent::SubmitReview`, which
/// the projector maps to a temperature signal. Every stance it offers is
/// contract-backed (`ReviewStance`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewPrompt {
    pub title: String,
    pub primary_evidence: String,
}

/// Exactly one focused ask. Each variant maps to a concrete `DirectorIntent`,
/// so no prompt can present a verdict the contracts cannot honor.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DirectorPrompt {
    Judgment(JudgmentPrompt),
    Gate(GatePrompt),
    Review(ReviewPrompt),
}
```

- [ ] **Step 5: `frame.rs`**

```rust
// bedrock-platform/helms/crates/director-contracts/src/frame.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use crate::action::{PrimaryAction, SecondaryAction};
use crate::context::{ContextLevel, PresenceHint};
use crate::prompt::DirectorPrompt;
use serde::{Deserialize, Serialize};

/// Who the current scene is blocked on.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WaitingFor {
    Nobody,
    Participants { actor_labels: Vec<String> },
    Server,
}

/// How hard the current moment blocks progress.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BlockingState {
    NotBlocking,
    BlocksFormation,
    BlocksSession,
}

/// The one task requiring human attention, in human terms.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NowTask {
    pub objective: String,
    pub needed_from_user: Option<String>,
    pub estimated_minutes: Option<u32>,
}

/// The current scene the user should see. Computed by `helm-client` from ordered
/// SSE / session / gate / loop state; **rendered, never computed** by Swift /
/// Kotlin / Svelte. This is the Rust→FFI/UI projection boundary — distinct from
/// `helm-session-contracts` (the server↔client wire boundary).
///
/// Domain-readable fields (`title`, `subtitle`, `now`, prompt copy) are filled by
/// the per-app FFI via `helm-client`'s `DomainPresenter` seam, because
/// `helm-client` treats session payloads as opaque.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectorFrame {
    pub frame_id: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub now: Option<NowTask>,
    pub waiting_for: WaitingFor,
    pub primary: PrimaryAction,
    pub secondary: Vec<SecondaryAction>,
    pub prompt: Option<DirectorPrompt>,
    pub presence: Vec<PresenceHint>,
    pub context_trail: Vec<ContextLevel>,
    pub blocking: BlockingState,
}

/// An immutable, versioned snapshot. `version` is the upstream SSE `sequence` the
/// frame was computed at (the `runway-app-host` hub sequence consumed by
/// `helm-client`) — **not** a new mobile counter — so ordering and dedup are
/// consistent end-to-end. `helm-client` produces this; `mobile-core` may wrap or
/// re-export it as its FFI envelope.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectorSnapshot {
    pub version: u64,
    pub frame: DirectorFrame,
}
```

- [ ] **Step 6: `lib.rs` (with round-trip tests)**

```rust
// bedrock-platform/helms/crates/director-contracts/src/lib.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

pub mod action;
pub mod context;
pub mod frame;
pub mod prompt;

pub use action::{DirectorIntent, GateVerdict, PrimaryAction, ReviewStance, SecondaryAction};
pub use context::{ContextLevel, PresenceHint};
pub use frame::{BlockingState, DirectorFrame, DirectorSnapshot, NowTask, WaitingFor};
pub use prompt::{Choice, DirectorPrompt, GatePrompt, JudgmentPrompt, ReviewPrompt};

#[cfg(test)]
mod tests {
    use super::*;
    use helm_session_contracts::gate::{GateCondition, GateId};

    fn gate_frame() -> DirectorFrame {
        DirectorFrame {
            frame_id: "f-1".into(),
            title: "Legal approval required".into(),
            subtitle: None,
            now: None,
            waiting_for: WaitingFor::Server,
            primary: PrimaryAction {
                label: "Approve".into(),
                intent: DirectorIntent::RespondGate {
                    gate_id: GateId::from_string("g-1"),
                    verdict: GateVerdict::Approve,
                },
            },
            secondary: vec![SecondaryAction {
                label: "Reject".into(),
                intent: DirectorIntent::RespondGate {
                    gate_id: GateId::from_string("g-1"),
                    verdict: GateVerdict::Reject,
                },
            }],
            prompt: Some(DirectorPrompt::Gate(GatePrompt {
                gate_id: GateId::from_string("g-1"),
                reason: "Approve revised liability wording".into(),
                consequence: "Formation cannot claim success until resolved".into(),
                deadline_ms: Some(1_700_000_000_000),
                condition: GateCondition::AnyParticipant,
            })),
            presence: vec![],
            context_trail: vec![ContextLevel::Task, ContextLevel::Session],
            blocking: BlockingState::BlocksFormation,
        }
    }

    #[test]
    fn director_snapshot_round_trips_and_keeps_version() {
        let snap = DirectorSnapshot { version: 42, frame: gate_frame() };
        let json = serde_json::to_string(&snap).unwrap();
        let back: DirectorSnapshot = serde_json::from_str(&json).unwrap();
        assert_eq!(back.version, 42);
        assert!(matches!(back.frame.prompt, Some(DirectorPrompt::Gate(_))));
        assert!(matches!(back.frame.blocking, BlockingState::BlocksFormation));
    }

    #[test]
    fn gate_verdict_has_only_contract_backed_variants() {
        // Guards the "no UI-only verdict" rule at the type level.
        for v in [GateVerdict::Approve, GateVerdict::Reject] {
            let s = serde_json::to_string(&v).unwrap();
            let back: GateVerdict = serde_json::from_str(&s).unwrap();
            assert_eq!(v, back);
        }
        // "later"/"defer" is intentionally NOT a variant; adding it requires a
        // Helms gate-contract change first.
        assert!(serde_json::from_str::<GateVerdict>("\"later\"").is_err());
    }

    #[test]
    fn director_intent_round_trips() {
        let intent = DirectorIntent::RequestContext { level: ContextLevel::Formation };
        let json = serde_json::to_string(&intent).unwrap();
        let back: DirectorIntent = serde_json::from_str(&json).unwrap();
        assert!(matches!(
            back,
            DirectorIntent::RequestContext { level: ContextLevel::Formation }
        ));
    }
}
```

- [ ] **Step 7: Test + clippy + commit**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo test -p director-contracts --locked
cargo clippy -p director-contracts -- -D warnings
git add crates/director-contracts/
git commit -m "feat(helms): add director-contracts — the DirectorFrame projection boundary

Canonical Rust→FFI/UI types for the AI Director UX: DirectorFrame,
DirectorSnapshot (version = upstream SSE sequence), DirectorPrompt
(Judgment/Gate/Review), typed DirectorIntent, GateVerdict (approve/reject
only — defer must enter the Helms gate contract first). Distinct from the
server↔client wire crate; depends on helm-session-contracts one-way.
mobile-core consumes/re-exports these — it must not fork DirectorFrame.

Part of: Session Intelligence Spine Plan 1 (anchors mobile M3A.8)."
```

---

### Task 2: `helm-client` — scaffolding, `ids.rs`, `formation.rs`, `LoopRegistry`

**Files:**
- Create: `bedrock-platform/helms/crates/helm-client/Cargo.toml`
- Create: `bedrock-platform/helms/crates/helm-client/src/lib.rs`
- Create: `bedrock-platform/helms/crates/helm-client/src/ids.rs`
- Create: `bedrock-platform/helms/crates/helm-client/src/formation.rs`
- Create: `bedrock-platform/helms/crates/helm-client/src/registry.rs`
- Create: `bedrock-platform/helms/crates/helm-client/tests/registry_tests.rs`

**Interfaces:**
- Consumes: `helm-session-contracts::UrgencyIntent` (for future tasks)
- Produces: `LoopId`, `SeedContext`, `FormationOutput`, `TemperatureReading`, `LocalFormationIntent`, `LoopRegistry`, `LoopEntry`, `LoopKind`, `LoopState`, `LoopEntryView`

- [ ] **Step 1: Create `Cargo.toml`**

```toml
# bedrock-platform/helms/crates/helm-client/Cargo.toml
[package]
name = "helm-client"
version.workspace = true
edition.workspace = true
license.workspace = true
publish.workspace = true

[dependencies]
helm-session-contracts = { path = "../helm-session-contracts" }
serde = { workspace = true, features = ["derive"] }
serde_json.workspace = true
thiserror.workspace = true
uuid = { workspace = true, features = ["v4"] }

[dev-dependencies]
serde_json.workspace = true
```

- [ ] **Step 2: Write failing registry tests**

```rust
// bedrock-platform/helms/crates/helm-client/tests/registry_tests.rs
use helm_client::registry::{LoopKind, LoopRegistry, LoopState};
use helm_client::formation::SeedContext;

fn seed(desc: &str) -> SeedContext {
    SeedContext {
        facts: vec![],
        description: desc.into(),
    }
}

#[test]
fn new_registry_is_empty() {
    let r = LoopRegistry::new();
    assert!(r.running_entry().is_none());
    assert!(r.entries().is_empty());
}

#[test]
fn spawn_creates_running_entry() {
    let mut r = LoopRegistry::new();
    let id = r.spawn("personal-synthesis".into(), seed("think about hypothesis X"));
    assert!(r.running_entry().is_some());
    let entry = r.get(&id).unwrap();
    assert!(matches!(entry.state, LoopState::Running));
}

#[test]
fn at_most_one_running_at_a_time() {
    let mut r = LoopRegistry::new();
    r.spawn("synthesis".into(), seed("context a"));
    // spawning a second sequential loop is rejected while one is Running
    let result = r.try_spawn_sequential("synthesis".into(), seed("context b"));
    assert!(result.is_err());
}

#[test]
fn server_handle_does_not_block_local_slot() {
    let mut r = LoopRegistry::new();
    r.spawn("synthesis".into(), seed("primary"));
    // A ServerHandle tracks a formation running on the server (the "DD job
    // while I wait" case). It is an independent entry that must NOT occupy the
    // single local-running slot.
    let id2 = r.spawn_server_handle("srv-formation-1".into(), "dd-analysis".into(), seed("dd context"));
    assert!(r.get(&id2).is_some());
    assert_eq!(r.entries().len(), 2);
    // The local-running slot is still held by the original Local formation...
    let running = r.running_entry().expect("a local formation is running");
    assert!(matches!(running.kind, LoopKind::Local));
    // ...so a second *local* spawn is still rejected.
    assert!(r.try_spawn_sequential("synthesis".into(), seed("context b")).is_err());
}

#[test]
fn server_handle_alone_has_no_local_running() {
    let mut r = LoopRegistry::new();
    // A server handle with no local formation leaves the local-running slot free.
    r.spawn_server_handle("srv-1".into(), "dd-analysis".into(), seed("dd context"));
    assert!(r.running_entry().is_none());
    // A local formation may then be spawned sequentially.
    assert!(r.try_spawn_sequential("synthesis".into(), seed("local work")).is_ok());
}

#[test]
fn pause_running_entry() {
    let mut r = LoopRegistry::new();
    let id = r.spawn("synthesis".into(), seed("initial"));
    r.pause(&id, seed("injected from server")).unwrap();
    let entry = r.get(&id).unwrap();
    assert!(matches!(entry.state, LoopState::Paused { .. }));
    assert!(r.running_entry().is_none());
}

#[test]
fn resume_paused_entry() {
    let mut r = LoopRegistry::new();
    let id = r.spawn("synthesis".into(), seed("initial"));
    r.pause(&id, seed("injected")).unwrap();
    r.resume(&id).unwrap();
    let entry = r.get(&id).unwrap();
    assert!(matches!(entry.state, LoopState::Running));
    assert!(r.running_entry().is_some());
}

#[test]
fn complete_entry_stays_in_registry() {
    let mut r = LoopRegistry::new();
    let id = r.spawn("synthesis".into(), seed("task"));
    r.complete(&id, vec![serde_json::json!({"result": "done"})]).unwrap();
    assert!(r.running_entry().is_none());
    let entry = r.get(&id).unwrap();
    assert!(matches!(entry.state, LoopState::Completed(_)));
    assert_eq!(r.entries().len(), 1); // still present
}

#[test]
fn pause_nonexistent_loop_returns_error() {
    let mut r = LoopRegistry::new();
    let fake_id = helm_client::ids::LoopId::new();
    let result = r.pause(&fake_id, seed("ctx"));
    assert!(result.is_err());
}
```

- [ ] **Step 3: Run tests — expect compile failure**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo test -p helm-client 2>&1 | head -20
```

Expected: `error[E0432]: unresolved import`

- [ ] **Step 4: Implement `ids.rs`**

```rust
// bedrock-platform/helms/crates/helm-client/src/ids.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use uuid::Uuid;

/// Unique identifier for a local formation entry.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LoopId(String);

impl LoopId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for LoopId {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for LoopId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
```

- [ ] **Step 5: Implement `formation.rs`**

```rust
// bedrock-platform/helms/crates/helm-client/src/formation.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

/// Input context injected into a local formation as seed facts.
///
/// `facts` are opaque JSON values — the native layer knows how to
/// marshal them into ProposedFacts for the local Converge formation.
/// `helm-client` never inspects them.
#[derive(Debug, Clone)]
pub struct SeedContext {
    pub facts: Vec<serde_json::Value>,
    pub description: String,
}

/// Output from a completed local formation.
///
/// `proposals` are opaque JSON values submitted to the server admission
/// boundary by the native layer. `helm-client` never inspects them.
#[derive(Debug, Clone)]
pub struct FormationOutput {
    pub proposals: Vec<serde_json::Value>,
    /// Optional temperature reading derived from the formation's fixed point.
    /// If present, the native layer submits it as a TemperatureSignal.
    pub temperature: Option<TemperatureReading>,
}

/// Position + conviction derived from a local formation's output.
/// The native layer converts this into a server-bound ProposedFact.
#[derive(Debug, Clone)]
pub struct TemperatureReading {
    /// "agree" | "disagree" | "uncertain" | "need_more_evidence"
    pub position: String,
    /// "low" | "medium" | "high" | "critical"
    pub conviction: String,
    /// SubjectRef string — what this temperature is about.
    pub subject_ref: String,
}

/// Converge-free, opaque representation of a local formation's `RootIntent`.
///
/// The spec's `LoopEntry` carries a `TypesRootIntent`
/// (`converge_core::types::intent`). `helm-client` has **zero Converge deps**, so
/// it cannot hold that type. This carries exactly what Client Helm needs from the
/// intent: a `description` to display, and the two **engine-enforced** budgets
/// (`max_cycles`, `max_facts`) the native layer uses to configure the local
/// Converge formation. The full `TypesRootIntent` is reconstructed native-side
/// from this carrier. (The wall-clock budget is separate — Converge does not
/// enforce `time_limit`; see `WallClockGuard` in Task 4b.)
#[derive(Debug, Clone)]
pub struct LocalFormationIntent {
    pub description: String,
    pub max_cycles: u32,
    pub max_facts: u32,
}

impl LocalFormationIntent {
    /// Default light on-device budgets for a personal formation.
    #[must_use]
    pub fn new(description: String) -> Self {
        Self {
            description,
            max_cycles: 8,
            max_facts: 64,
        }
    }
}
```

- [ ] **Step 6: Implement `registry.rs`**

```rust
// bedrock-platform/helms/crates/helm-client/src/registry.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use crate::formation::{FormationOutput, LocalFormationIntent, SeedContext};
use crate::ids::LoopId;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum RegistryError {
    #[error("loop {0} not found")]
    NotFound(String),
    #[error("loop {0} is not in Running state")]
    NotRunning(String),
    #[error("loop {0} is not in Paused state")]
    NotPaused(String),
    #[error("a sequential loop is already Running")]
    AlreadyRunning,
}

/// Where a registered formation actually runs.
///
/// `Local` formations run Converge on this device and contend for the single
/// local-running slot. `ServerHandle` entries run on the **server** — the device
/// only tracks and surfaces them — so they never occupy the local slot. This is
/// the "DD job while I wait" case: heavy/parallel work is offloaded, not run on
/// device. `server_formation_id` is an opaque handle assigned by the server
/// (kept as a `String` here to preserve the crate's zero-Converge-deps rule).
#[derive(Debug, Clone)]
pub enum LoopKind {
    Local,
    ServerHandle { server_formation_id: String },
}

/// Current lifecycle state of a local formation.
#[derive(Debug, Clone)]
pub enum LoopState {
    Running,
    Paused { injected_context: SeedContext },
    Completed(Vec<serde_json::Value>),
    Failed(String),
}

/// A single formation entry in the registry.
#[derive(Debug, Clone)]
pub struct LoopEntry {
    pub loop_id: LoopId,
    /// Local (runs here) or ServerHandle (runs on the server, tracked here).
    pub kind: LoopKind,
    /// Opaque, Converge-free representation of the formation's RootIntent.
    pub intent: LocalFormationIntent,
    pub formation_type: String,
    pub seed_context: SeedContext,
    pub state: LoopState,
}

/// Read-only view of a LoopEntry for UI / FFI exposure.
#[derive(Debug, Clone)]
pub struct LoopEntryView {
    pub loop_id: String,
    pub formation_type: String,
    pub description: String,
    pub state_label: &'static str,
    /// "local" or "server_handle".
    pub kind_label: &'static str,
    /// Present only for ServerHandle entries.
    pub server_formation_id: Option<String>,
}

/// Manages the lifecycle of formations the participant is running.
///
/// Invariant: at most one `Local` entry is `Running` at a time. `ServerHandle`
/// entries run on the server and never occupy the local-running slot, so any
/// number may coexist with the single local formation.
pub struct LoopRegistry {
    entries: HashMap<String, LoopEntry>,
}

impl LoopRegistry {
    #[must_use]
    pub fn new() -> Self {
        Self {
            entries: HashMap::new(),
        }
    }

    /// Spawn a sequential **local** formation. Fails if one is already Running.
    pub fn try_spawn_sequential(
        &mut self,
        formation_type: String,
        seed_context: SeedContext,
    ) -> Result<LoopId, RegistryError> {
        if self.running_entry().is_some() {
            return Err(RegistryError::AlreadyRunning);
        }
        Ok(self.insert(formation_type, seed_context, LoopKind::Local))
    }

    /// Spawn a **local** formation unconditionally (use when no loop is running).
    #[must_use]
    pub fn spawn(&mut self, formation_type: String, seed_context: SeedContext) -> LoopId {
        self.insert(formation_type, seed_context, LoopKind::Local)
    }

    /// Record a **server-side** formation the participant spawned (e.g. the "DD
    /// job while I wait"). It runs on the server; the device only tracks it. It
    /// never occupies the local-running slot, so it is always allowed.
    /// `server_formation_id` is the opaque handle the server assigned.
    #[must_use]
    pub fn spawn_server_handle(
        &mut self,
        server_formation_id: String,
        formation_type: String,
        seed_context: SeedContext,
    ) -> LoopId {
        self.insert(
            formation_type,
            seed_context,
            LoopKind::ServerHandle { server_formation_id },
        )
    }

    fn insert(
        &mut self,
        formation_type: String,
        seed_context: SeedContext,
        kind: LoopKind,
    ) -> LoopId {
        let id = LoopId::new();
        let intent = LocalFormationIntent::new(seed_context.description.clone());
        self.entries.insert(
            id.as_str().to_string(),
            LoopEntry {
                loop_id: id.clone(),
                kind,
                intent,
                formation_type,
                seed_context,
                state: LoopState::Running,
            },
        );
        id
    }

    /// Pause a Running entry and inject server context.
    pub fn pause(
        &mut self,
        loop_id: &LoopId,
        injected_context: SeedContext,
    ) -> Result<(), RegistryError> {
        let entry = self
            .entries
            .get_mut(loop_id.as_str())
            .ok_or_else(|| RegistryError::NotFound(loop_id.to_string()))?;
        if !matches!(entry.state, LoopState::Running) {
            return Err(RegistryError::NotRunning(loop_id.to_string()));
        }
        entry.state = LoopState::Paused { injected_context };
        Ok(())
    }

    /// Resume a Paused entry.
    pub fn resume(&mut self, loop_id: &LoopId) -> Result<(), RegistryError> {
        let entry = self
            .entries
            .get_mut(loop_id.as_str())
            .ok_or_else(|| RegistryError::NotFound(loop_id.to_string()))?;
        if !matches!(entry.state, LoopState::Paused { .. }) {
            return Err(RegistryError::NotPaused(loop_id.to_string()));
        }
        entry.state = LoopState::Running;
        Ok(())
    }

    /// Mark a formation as completed.
    pub fn complete(
        &mut self,
        loop_id: &LoopId,
        proposals: Vec<serde_json::Value>,
    ) -> Result<(), RegistryError> {
        let entry = self
            .entries
            .get_mut(loop_id.as_str())
            .ok_or_else(|| RegistryError::NotFound(loop_id.to_string()))?;
        entry.state = LoopState::Completed(proposals);
        Ok(())
    }

    /// Mark a formation as failed.
    pub fn fail(&mut self, loop_id: &LoopId, reason: String) -> Result<(), RegistryError> {
        let entry = self
            .entries
            .get_mut(loop_id.as_str())
            .ok_or_else(|| RegistryError::NotFound(loop_id.to_string()))?;
        entry.state = LoopState::Failed(reason);
        Ok(())
    }

    /// The single `Local` Running entry, if any. `ServerHandle` entries are
    /// never counted — they run on the server, not the local-running slot.
    #[must_use]
    pub fn running_entry(&self) -> Option<&LoopEntry> {
        self.entries.values().find(|e| {
            matches!(e.kind, LoopKind::Local) && matches!(e.state, LoopState::Running)
        })
    }

    #[must_use]
    pub fn get(&self, loop_id: &LoopId) -> Option<&LoopEntry> {
        self.entries.get(loop_id.as_str())
    }

    /// All entries — for UI inspection.
    #[must_use]
    pub fn entries(&self) -> Vec<&LoopEntry> {
        self.entries.values().collect()
    }

    /// Read-only views for FFI / UI.
    #[must_use]
    pub fn entry_views(&self) -> Vec<LoopEntryView> {
        self.entries
            .values()
            .map(|e| LoopEntryView {
                loop_id: e.loop_id.as_str().to_string(),
                formation_type: e.formation_type.clone(),
                description: e.intent.description.clone(),
                state_label: match &e.state {
                    LoopState::Running => "running",
                    LoopState::Paused { .. } => "paused",
                    LoopState::Completed(_) => "completed",
                    LoopState::Failed(_) => "failed",
                },
                kind_label: match &e.kind {
                    LoopKind::Local => "local",
                    LoopKind::ServerHandle { .. } => "server_handle",
                },
                server_formation_id: match &e.kind {
                    LoopKind::Local => None,
                    LoopKind::ServerHandle { server_formation_id } => {
                        Some(server_formation_id.clone())
                    }
                },
            })
            .collect()
    }
}

impl Default for LoopRegistry {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 7: Stub `lib.rs` to make tests compile**

```rust
// bedrock-platform/helms/crates/helm-client/src/lib.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

pub mod formation;
pub mod ids;
pub mod registry;
```

- [ ] **Step 8: Run registry tests — expect pass**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo test -p helm-client --test registry_tests --locked
```

Expected: 9 tests pass.

- [ ] **Step 9: Clippy + commit**

```bash
cargo clippy -p helm-client -- -D warnings
git add crates/helm-client/
git commit -m "feat(helms): add helm-client LoopRegistry

Manages formation lifecycle: spawn (local) / spawn_server_handle,
pause with injected context, resume, complete, fail. LoopKind
distinguishes Local (runs on device) from ServerHandle (runs on the
server, tracked here). Invariant: at most one Local formation Running
at a time; ServerHandle entries never occupy the local slot. Carries an
opaque LocalFormationIntent (Converge-free RootIntent representation).

Part of: Session Intelligence Spine Plan 1."
```

---

### Task 3: `SeverityRouter` — B/C routing decision

**Files:**
- Create: `bedrock-platform/helms/crates/helm-client/src/router.rs`
- Create: `bedrock-platform/helms/crates/helm-client/tests/router_tests.rs`

**Interfaces:**
- Consumes: `UrgencyIntent` from `helm-session-contracts`, `LoopId` from `ids.rs`
- Produces: `SeverityRouter`, `RoutingDecision` — used by `ClientHelm` in Task 5

- [ ] **Step 1: Write failing router tests**

```rust
// bedrock-platform/helms/crates/helm-client/tests/router_tests.rs
use helm_client::formation::SeedContext;
use helm_client::ids::LoopId;
use helm_client::router::{RoutingDecision, SeverityRouter};
use helm_session_contracts::urgency::UrgencyIntent;

fn seed(desc: &str) -> SeedContext {
    SeedContext { facts: vec![], description: desc.into() }
}

fn router() -> SeverityRouter {
    SeverityRouter::new()
}

// ── No active loop ──────────────────────────────────────────────────────────

#[test]
fn no_loop_informational_spawns_new() {
    let decision = router().decide(UrgencyIntent::Informational, None, seed("ctx"));
    assert!(matches!(decision, RoutingDecision::SpawnNew { .. }));
}

#[test]
fn no_loop_advisory_spawns_new() {
    let decision = router().decide(UrgencyIntent::Advisory, None, seed("ctx"));
    assert!(matches!(decision, RoutingDecision::SpawnNew { .. }));
}

#[test]
fn no_loop_disruptive_spawns_new() {
    let decision = router().decide(UrgencyIntent::Disruptive, None, seed("ctx"));
    assert!(matches!(decision, RoutingDecision::SpawnNew { .. }));
}

#[test]
fn no_loop_preemptive_spawns_new() {
    let decision = router().decide(UrgencyIntent::Preemptive, None, seed("ctx"));
    assert!(matches!(decision, RoutingDecision::SpawnNew { .. }));
}

// ── Active loop ─────────────────────────────────────────────────────────────

#[test]
fn active_loop_informational_queues_and_notifies() {
    let id = LoopId::new();
    let decision = router().decide(UrgencyIntent::Informational, Some(&id), seed("ctx"));
    assert!(matches!(decision, RoutingDecision::QueueAndNotify { .. }));
}

#[test]
fn active_loop_advisory_queues_and_notifies() {
    let id = LoopId::new();
    let decision = router().decide(UrgencyIntent::Advisory, Some(&id), seed("ctx"));
    assert!(matches!(decision, RoutingDecision::QueueAndNotify { .. }));
}

#[test]
fn active_loop_disruptive_offloads_to_server() {
    let id = LoopId::new();
    // A local formation is already running, so Disruptive work is offloaded to
    // the server (tracked as a ServerHandle) rather than spawning a second
    // local formation — at most one local formation runs at a time.
    let decision = router().decide(UrgencyIntent::Disruptive, Some(&id), seed("ctx"));
    assert!(matches!(decision, RoutingDecision::OffloadToServer { .. }));
}

#[test]
fn active_loop_preemptive_pauses_and_injects() {
    let id = LoopId::new();
    let decision = router().decide(UrgencyIntent::Preemptive, Some(&id), seed("ctx"));
    match decision {
        RoutingDecision::PauseAndInject { loop_id_to_pause, .. } => {
            assert_eq!(loop_id_to_pause.as_str(), id.as_str());
        }
        other => panic!("expected PauseAndInject, got {other:?}"),
    }
}

#[test]
fn preemptive_with_active_loop_carries_seed_context() {
    let id = LoopId::new();
    let ctx = seed("server contradiction context");
    let decision = router().decide(UrgencyIntent::Preemptive, Some(&id), ctx);
    match decision {
        RoutingDecision::PauseAndInject { injected_context, .. } => {
            assert_eq!(injected_context.description, "server contradiction context");
        }
        other => panic!("expected PauseAndInject, got {other:?}"),
    }
}
```

- [ ] **Step 2: Run tests — expect compile failure**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo test -p helm-client --test router_tests 2>&1 | head -15
```

- [ ] **Step 3: Implement `router.rs`**

```rust
// bedrock-platform/helms/crates/helm-client/src/router.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use crate::formation::SeedContext;
use crate::ids::LoopId;
use helm_session_contracts::urgency::UrgencyIntent;

/// The action Client Helm instructs the native layer to take.
#[derive(Debug)]
pub enum RoutingDecision {
    /// B — no active loop: spawn a new local formation with these seeds.
    SpawnNew { seed_context: SeedContext },
    /// B — a local formation is already running: offload this work to the
    /// **server** as a sub-formation (tracked as a `ServerHandle`). It does NOT
    /// spawn a second local formation — at most one local formation runs at a
    /// time, and heavy/parallel work belongs on the server. The native layer
    /// requests the sub-formation; when the server returns an id it calls
    /// `ClientHelm::server_formation_started` to record the handle.
    OffloadToServer { seed_context: SeedContext },
    /// Queue this push and show a UI notification; active loop continues.
    QueueAndNotify { urgency: UrgencyIntent, seed_context: SeedContext },
    /// C — suspend the active loop and capture server context. On user accept,
    /// the native layer spawns a FRESH formation seeded with the suspended
    /// loop's accumulated context + this injected context. This is local
    /// registry bookkeeping, NOT a Converge `Engine::resume`.
    PauseAndInject { loop_id_to_pause: LoopId, injected_context: SeedContext },
}

/// Stateless router: maps (urgency, active loop, context) → routing decision.
///
/// Routing table from spec Section 4:
/// - No running loop (any urgency)        → SpawnNew (local)
/// - Running + Informational / Advisory   → QueueAndNotify
/// - Running + Disruptive                 → OffloadToServer (server sub-formation,
///                                           tracked as a ServerHandle — never a
///                                           second local loop)
/// - Running + Preemptive                 → PauseAndInject (suspend + fresh spawn,
///                                           NOT a Converge Engine::resume)
pub struct SeverityRouter;

impl SeverityRouter {
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Decide what to do with an incoming SessionPush.
    ///
    /// `active_loop_id` is the LoopId of the currently Running sequential
    /// formation, if any.
    #[must_use]
    pub fn decide(
        &self,
        urgency: UrgencyIntent,
        active_loop_id: Option<&LoopId>,
        seed_context: SeedContext,
    ) -> RoutingDecision {
        let Some(running_id) = active_loop_id else {
            return RoutingDecision::SpawnNew { seed_context };
        };

        match urgency {
            UrgencyIntent::Informational | UrgencyIntent::Advisory => {
                RoutingDecision::QueueAndNotify { urgency, seed_context }
            }
            UrgencyIntent::Disruptive => RoutingDecision::OffloadToServer { seed_context },
            UrgencyIntent::Preemptive => RoutingDecision::PauseAndInject {
                loop_id_to_pause: running_id.clone(),
                injected_context: seed_context,
            },
        }
    }
}

impl Default for SeverityRouter {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 4: Add `router` to `lib.rs`**

```rust
// bedrock-platform/helms/crates/helm-client/src/lib.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

pub mod formation;
pub mod ids;
pub mod registry;
pub mod router;
```

- [ ] **Step 5: Run router tests — expect pass**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo test -p helm-client --test router_tests --locked
```

Expected: 10 tests pass.

- [ ] **Step 6: Clippy + commit**

```bash
cargo clippy -p helm-client -- -D warnings
git add crates/helm-client/src/router.rs crates/helm-client/tests/router_tests.rs crates/helm-client/src/lib.rs
git commit -m "feat(helms): add SeverityRouter — B/C routing decision

Stateless router maps (urgency_intent, active_loop_id, seed_context)
to RoutingDecision. Covers all 8 combinations from spec:
no-loop × 4 urgencies → SpawnNew (local); running × Informational/Advisory
→ QueueAndNotify; Disruptive → OffloadToServer (server sub-formation,
tracked as a ServerHandle); Preemptive → PauseAndInject.

Part of: Session Intelligence Spine Plan 1."
```

---

### Task 4: `TemperatureQueue` and `GatedDecisionSurface`

**Files:**
- Create: `bedrock-platform/helms/crates/helm-client/src/temperature.rs`
- Create: `bedrock-platform/helms/crates/helm-client/src/gate_surface.rs`

**Interfaces:**
- Consumes: `helm-session-contracts::{GatedDecision, GateId}`
- Produces: `TemperatureQueue`, `PendingSubmission`, `GatedDecisionSurface`, `GatedDecisionView`, `PendingGateResponse`

- [ ] **Step 1: Write failing tests (inline in source)**

```rust
// at the bottom of temperature.rs and gate_surface.rs — write #[cfg(test)] blocks
// Write them first, then implement the structs.
```

Inline test for `TemperatureQueue`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_and_drain() {
        let mut q = TemperatureQueue::new();
        q.enqueue(
            TemperatureSignal {
                position: "agree".into(),
                conviction: "high".into(),
                subject_ref: "quorum://hypothesis/h-1".into(),
            },
            "key-1".into(),
        );
        let drained = q.drain();
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].idempotency_key, "key-1");
    }

    #[test]
    fn drain_is_empty_after_call() {
        let mut q = TemperatureQueue::new();
        q.enqueue(
            TemperatureSignal {
                position: "disagree".into(),
                conviction: "critical".into(),
                subject_ref: "quorum://hypothesis/h-2".into(),
            },
            "key-2".into(),
        );
        q.drain();
        assert!(q.drain().is_empty());
    }

    #[test]
    fn duplicate_key_is_deduplicated() {
        let mut q = TemperatureQueue::new();
        let sig = TemperatureSignal {
            position: "agree".into(),
            conviction: "low".into(),
            subject_ref: "quorum://hypothesis/h-3".into(),
        };
        q.enqueue(sig.clone(), "key-dup".into());
        q.enqueue(sig, "key-dup".into());
        assert_eq!(q.drain().len(), 1);
    }
}
```

Inline test for `GatedDecisionSurface`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use helm_session_contracts::gate::{GateCondition, GateId, GatedDecision};

    fn make_gate(deadline: Option<u64>) -> GatedDecision {
        GatedDecision {
            gate_id: GateId::new(),
            condition: GateCondition::AnyParticipant,
            payload: serde_json::json!({}),
            deadline,
        }
    }

    #[test]
    fn add_and_list_pending_gates() {
        let mut s = GatedDecisionSurface::new();
        s.add_gate(make_gate(None));
        assert_eq!(s.pending_gates().len(), 1);
    }

    #[test]
    fn respond_to_gate_removes_it() {
        let mut s = GatedDecisionSurface::new();
        let gate = make_gate(None);
        let gate_id = gate.gate_id.clone();
        s.add_gate(gate);
        let response = s.respond(&gate_id, serde_json::json!({"approved": true}));
        assert!(response.is_some());
        assert!(s.pending_gates().is_empty());
    }

    #[test]
    fn respond_to_unknown_gate_returns_none() {
        let mut s = GatedDecisionSurface::new();
        let result = s.respond(&GateId::new(), serde_json::json!({}));
        assert!(result.is_none());
    }

    #[test]
    fn deadline_not_expired_when_now_is_before() {
        let s = GatedDecisionSurface::new();
        let gate = make_gate(Some(2_000_000_000_000));
        assert!(!s.is_deadline_expired(&gate, 1_000_000_000_000));
    }

    #[test]
    fn deadline_expired_when_now_is_after() {
        let s = GatedDecisionSurface::new();
        let gate = make_gate(Some(1_000_000_000_000));
        assert!(s.is_deadline_expired(&gate, 2_000_000_000_000));
    }

    #[test]
    fn no_deadline_never_expires() {
        let s = GatedDecisionSurface::new();
        let gate = make_gate(None);
        assert!(!s.is_deadline_expired(&gate, u64::MAX));
    }
}
```

- [ ] **Step 2: Run tests — expect compile failure**

```bash
cargo test -p helm-client --locked 2>&1 | head -15
```

- [ ] **Step 3: Implement `temperature.rs`**

```rust
// bedrock-platform/helms/crates/helm-client/src/temperature.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use std::collections::HashMap;

/// A participant's position and conviction on a specific subject.
/// Sent to the server admission boundary as a ProposedFact.
#[derive(Debug, Clone)]
pub struct TemperatureSignal {
    /// "agree" | "disagree" | "uncertain" | "need_more_evidence"
    pub position: String,
    /// "low" | "medium" | "high" | "critical"
    pub conviction: String,
    /// SubjectRef string — e.g. "quorum://hypothesis/h-1"
    pub subject_ref: String,
}

/// A temperature signal ready for submission to the server.
#[derive(Debug, Clone)]
pub struct PendingSubmission {
    pub signal: TemperatureSignal,
    pub idempotency_key: String,
}

/// Queue for outbound temperature signals.
/// Deduplicated by idempotency key; drain → submit → if fails, re-enqueue.
pub struct TemperatureQueue {
    pending: HashMap<String, TemperatureSignal>,
    order: Vec<String>, // preserve insertion order
}

impl TemperatureQueue {
    #[must_use]
    pub fn new() -> Self {
        Self {
            pending: HashMap::new(),
            order: Vec::new(),
        }
    }

    /// Enqueue a signal. If the key already exists, the existing entry is kept (idempotent).
    pub fn enqueue(&mut self, signal: TemperatureSignal, idempotency_key: String) {
        if self.pending.contains_key(&idempotency_key) {
            return;
        }
        self.order.push(idempotency_key.clone());
        self.pending.insert(idempotency_key, signal);
    }

    /// Consume all pending signals. Queue is empty after this call.
    #[must_use]
    pub fn drain(&mut self) -> Vec<PendingSubmission> {
        let mut out = Vec::with_capacity(self.order.len());
        for key in self.order.drain(..) {
            if let Some(signal) = self.pending.remove(&key) {
                out.push(PendingSubmission {
                    signal,
                    idempotency_key: key,
                });
            }
        }
        out
    }
}

impl Default for TemperatureQueue {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enqueue_and_drain() {
        let mut q = TemperatureQueue::new();
        q.enqueue(
            TemperatureSignal {
                position: "agree".into(),
                conviction: "high".into(),
                subject_ref: "quorum://hypothesis/h-1".into(),
            },
            "key-1".into(),
        );
        let drained = q.drain();
        assert_eq!(drained.len(), 1);
        assert_eq!(drained[0].idempotency_key, "key-1");
    }

    #[test]
    fn drain_is_empty_after_call() {
        let mut q = TemperatureQueue::new();
        q.enqueue(
            TemperatureSignal {
                position: "disagree".into(),
                conviction: "critical".into(),
                subject_ref: "quorum://hypothesis/h-2".into(),
            },
            "key-2".into(),
        );
        q.drain();
        assert!(q.drain().is_empty());
    }

    #[test]
    fn duplicate_key_is_deduplicated() {
        let mut q = TemperatureQueue::new();
        let sig = TemperatureSignal {
            position: "agree".into(),
            conviction: "low".into(),
            subject_ref: "quorum://hypothesis/h-3".into(),
        };
        q.enqueue(sig.clone(), "key-dup".into());
        q.enqueue(sig, "key-dup".into());
        assert_eq!(q.drain().len(), 1);
    }
}
```

- [ ] **Step 4: Implement `gate_surface.rs`**

```rust
// bedrock-platform/helms/crates/helm-client/src/gate_surface.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use helm_session_contracts::gate::{GateId, GatedDecision};
use std::collections::HashMap;

/// A pending gate response ready for submission to the server admission boundary.
#[derive(Debug, Clone)]
pub struct PendingGateResponse {
    pub gate_id: String,
    pub response: serde_json::Value,
}

/// Read-only view of a pending gate for FFI / UI.
#[derive(Debug, Clone)]
pub struct GatedDecisionView {
    pub gate_id: String,
    pub condition_label: String,
    pub deadline_ms: Option<u64>,
}

/// Tracks HITL gate events received from the server.
///
/// Gates are held until the user responds. The response is submitted
/// by the native layer via the server admission boundary.
pub struct GatedDecisionSurface {
    pending: HashMap<String, GatedDecision>,
}

impl GatedDecisionSurface {
    #[must_use]
    pub fn new() -> Self {
        Self {
            pending: HashMap::new(),
        }
    }

    /// Add a gate received from the server.
    pub fn add_gate(&mut self, gate: GatedDecision) {
        self.pending.insert(gate.gate_id.as_str().to_string(), gate);
    }

    /// Record a user response. Returns the pending response or None if gate unknown.
    pub fn respond(
        &mut self,
        gate_id: &GateId,
        response: serde_json::Value,
    ) -> Option<PendingGateResponse> {
        self.pending.remove(gate_id.as_str()).map(|_| PendingGateResponse {
            gate_id: gate_id.as_str().to_string(),
            response,
        })
    }

    /// All gates awaiting user response.
    #[must_use]
    pub fn pending_gates(&self) -> Vec<&GatedDecision> {
        self.pending.values().collect()
    }

    /// Read-only views for FFI / UI.
    #[must_use]
    pub fn gate_views(&self) -> Vec<GatedDecisionView> {
        use helm_session_contracts::gate::GateCondition;
        self.pending
            .values()
            .map(|g| GatedDecisionView {
                gate_id: g.gate_id.as_str().to_string(),
                condition_label: match &g.condition {
                    GateCondition::QuorumOfRoles { roles } => {
                        format!("quorum of: {}", roles.join(", "))
                    }
                    GateCondition::SpecificAuthority { actor_id } => {
                        format!("authority: {actor_id}")
                    }
                    GateCondition::AnyParticipant => "any participant".into(),
                    GateCondition::Unanimous => "unanimous".into(),
                },
                deadline_ms: g.deadline,
            })
            .collect()
    }

    /// True if the gate's deadline has passed at the given timestamp (ms).
    #[must_use]
    pub fn is_deadline_expired(&self, gate: &GatedDecision, now_ms: u64) -> bool {
        gate.deadline.map_or(false, |deadline| now_ms > deadline)
    }
}

impl Default for GatedDecisionSurface {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use helm_session_contracts::gate::{GateCondition, GateId, GatedDecision};

    fn make_gate(deadline: Option<u64>) -> GatedDecision {
        GatedDecision {
            gate_id: GateId::new(),
            condition: GateCondition::AnyParticipant,
            payload: serde_json::json!({}),
            deadline,
        }
    }

    #[test]
    fn add_and_list_pending_gates() {
        let mut s = GatedDecisionSurface::new();
        s.add_gate(make_gate(None));
        assert_eq!(s.pending_gates().len(), 1);
    }

    #[test]
    fn respond_to_gate_removes_it() {
        let mut s = GatedDecisionSurface::new();
        let gate = make_gate(None);
        let gate_id = gate.gate_id.clone();
        s.add_gate(gate);
        let response = s.respond(&gate_id, serde_json::json!({"approved": true}));
        assert!(response.is_some());
        assert!(s.pending_gates().is_empty());
    }

    #[test]
    fn respond_to_unknown_gate_returns_none() {
        let mut s = GatedDecisionSurface::new();
        let result = s.respond(&GateId::new(), serde_json::json!({}));
        assert!(result.is_none());
    }

    #[test]
    fn deadline_not_expired_when_now_is_before() {
        let s = GatedDecisionSurface::new();
        let gate = make_gate(Some(2_000_000_000_000));
        assert!(!s.is_deadline_expired(&gate, 1_000_000_000_000));
    }

    #[test]
    fn deadline_expired_when_now_is_after() {
        let s = GatedDecisionSurface::new();
        let gate = make_gate(Some(1_000_000_000_000));
        assert!(s.is_deadline_expired(&gate, 2_000_000_000_000));
    }

    #[test]
    fn no_deadline_never_expires() {
        let s = GatedDecisionSurface::new();
        let gate = make_gate(None);
        assert!(!s.is_deadline_expired(&gate, u64::MAX));
    }
}
```

- [ ] **Step 5: Add modules to `lib.rs`**

```rust
// bedrock-platform/helms/crates/helm-client/src/lib.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

pub mod formation;
pub mod gate_surface;
pub mod ids;
pub mod registry;
pub mod router;
pub mod temperature;
```

- [ ] **Step 6: Run all helm-client tests — expect pass**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo test -p helm-client --locked
```

Expected: all inline tests in `temperature.rs` and `gate_surface.rs` pass.

- [ ] **Step 7: Clippy + commit**

```bash
cargo clippy -p helm-client -- -D warnings
git add crates/helm-client/src/temperature.rs crates/helm-client/src/gate_surface.rs crates/helm-client/src/lib.rs
git commit -m "feat(helms): add TemperatureQueue and GatedDecisionSurface

TemperatureQueue: deduplicates by idempotency key, drains to
PendingSubmission for native layer to send to server admission.
GatedDecisionSurface: holds HITL gates, records user responses,
checks deadlines, exposes read-only views for FFI/UI.

Part of: Session Intelligence Spine Plan 1."
```

---

### Task 4b: `WallClockGuard` — on-device budget enforcement

**Why:** Converge's engine enforces `max_cycles` and `max_facts` but **not**
`time_limit` — the convergence loop never checks wall-clock (spec must-fix 3;
`StopReason::TimeBudgetExhausted` exists but is unused). On-device formations
must be bounded for battery/thermal reasons, so Client Helm imposes its own
wall-clock guard. The guard is pure (holds no clock): callers supply timestamps.
The spawn time comes from `SessionPush.session_context.timestamp_ms`; periodic
ticks supply `now_ms` from the native layer.

**Files:**
- Create: `bedrock-platform/helms/crates/helm-client/src/budget.rs`

**Interfaces:**
- Consumes: `LoopId` from `ids.rs`
- Produces: `WallClockGuard` — used by `ClientHelm` in Task 5

- [ ] **Step 1: Write failing tests (inline in source)**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    fn lid() -> LoopId {
        LoopId::new()
    }

    #[test]
    fn new_guard_has_nothing_armed() {
        let mut g = WallClockGuard::new();
        assert!(g.expired(u64::MAX).is_empty());
    }

    #[test]
    fn armed_budget_not_expired_before_max() {
        let mut g = WallClockGuard::new();
        let id = lid();
        g.arm(&id, 1_000, 5_000); // started at 1000ms, 5s budget
        assert!(g.expired(3_000).is_empty()); // only 2s elapsed
        assert!(g.is_armed(&id));
    }

    #[test]
    fn armed_budget_expires_at_or_after_max() {
        let mut g = WallClockGuard::new();
        let id = lid();
        g.arm(&id, 1_000, 5_000);
        let expired = g.expired(6_000); // 5s elapsed → expired
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0].as_str(), id.as_str());
    }

    #[test]
    fn expired_loops_are_disarmed_once() {
        let mut g = WallClockGuard::new();
        let id = lid();
        g.arm(&id, 0, 1_000);
        assert_eq!(g.expired(2_000).len(), 1);
        assert!(g.expired(2_000).is_empty()); // already reported + disarmed
        assert!(!g.is_armed(&id));
    }

    #[test]
    fn disarm_removes_budget() {
        let mut g = WallClockGuard::new();
        let id = lid();
        g.arm(&id, 0, 1_000);
        g.disarm(&id);
        assert!(g.expired(u64::MAX).is_empty());
    }
}
```

- [ ] **Step 2: Implement `budget.rs`**

```rust
// bedrock-platform/helms/crates/helm-client/src/budget.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use crate::ids::LoopId;
use std::collections::HashMap;

/// Per-loop wall-clock budget guard.
///
/// Converge enforces `max_cycles`/`max_facts` but not `time_limit`; the engine
/// loop never inspects wall-clock. This guard bounds on-device formations.
/// It is pure — it holds no clock. Callers supply `started_at_ms` (from the
/// triggering `SessionPush.session_context.timestamp_ms`) and `now_ms` (from
/// the native layer) on each tick.
#[derive(Debug, Default)]
pub struct WallClockGuard {
    armed: HashMap<String, ArmedBudget>,
}

#[derive(Debug, Clone)]
struct ArmedBudget {
    loop_id: LoopId,
    started_at_ms: u64,
    max_ms: u64,
}

impl WallClockGuard {
    #[must_use]
    pub fn new() -> Self {
        Self {
            armed: HashMap::new(),
        }
    }

    /// Arm a wall-clock budget for a loop. `started_at_ms` is the spawn time.
    pub fn arm(&mut self, loop_id: &LoopId, started_at_ms: u64, max_ms: u64) {
        self.armed.insert(
            loop_id.as_str().to_string(),
            ArmedBudget {
                loop_id: loop_id.clone(),
                started_at_ms,
                max_ms,
            },
        );
    }

    /// Disarm a loop (completed, failed, or paused — no longer time-bounded).
    pub fn disarm(&mut self, loop_id: &LoopId) {
        self.armed.remove(loop_id.as_str());
    }

    /// Loop ids whose budget has elapsed at `now_ms`. Expired loops are
    /// disarmed, so each is reported at most once.
    #[must_use]
    pub fn expired(&mut self, now_ms: u64) -> Vec<LoopId> {
        let keys: Vec<String> = self
            .armed
            .iter()
            .filter(|(_, b)| now_ms.saturating_sub(b.started_at_ms) >= b.max_ms)
            .map(|(k, _)| k.clone())
            .collect();
        keys.iter()
            .filter_map(|k| self.armed.remove(k))
            .map(|b| b.loop_id)
            .collect()
    }

    #[must_use]
    pub fn is_armed(&self, loop_id: &LoopId) -> bool {
        self.armed.contains_key(loop_id.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lid() -> LoopId {
        LoopId::new()
    }

    #[test]
    fn new_guard_has_nothing_armed() {
        let mut g = WallClockGuard::new();
        assert!(g.expired(u64::MAX).is_empty());
    }

    #[test]
    fn armed_budget_not_expired_before_max() {
        let mut g = WallClockGuard::new();
        let id = lid();
        g.arm(&id, 1_000, 5_000);
        assert!(g.expired(3_000).is_empty());
        assert!(g.is_armed(&id));
    }

    #[test]
    fn armed_budget_expires_at_or_after_max() {
        let mut g = WallClockGuard::new();
        let id = lid();
        g.arm(&id, 1_000, 5_000);
        let expired = g.expired(6_000);
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0].as_str(), id.as_str());
    }

    #[test]
    fn expired_loops_are_disarmed_once() {
        let mut g = WallClockGuard::new();
        let id = lid();
        g.arm(&id, 0, 1_000);
        assert_eq!(g.expired(2_000).len(), 1);
        assert!(g.expired(2_000).is_empty());
        assert!(!g.is_armed(&id));
    }

    #[test]
    fn disarm_removes_budget() {
        let mut g = WallClockGuard::new();
        let id = lid();
        g.arm(&id, 0, 1_000);
        g.disarm(&id);
        assert!(g.expired(u64::MAX).is_empty());
    }
}
```

- [ ] **Step 3: Add `budget` to `lib.rs`**

```rust
pub mod budget;
```

(Insert in alphabetical order, before `client`.)

- [ ] **Step 4: Run tests + clippy + commit**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo test -p helm-client --locked
cargo clippy -p helm-client -- -D warnings
git add crates/helm-client/src/budget.rs crates/helm-client/src/lib.rs
git commit -m "feat(helms): add WallClockGuard for on-device formation budgets

Converge enforces max_cycles/max_facts but not time_limit (the engine
loop never checks wall-clock). Client Helm bounds on-device formations
with a pure, clock-free guard: callers supply started_at_ms and now_ms.

Part of: Session Intelligence Spine Plan 1."
```

> ClientHelm wires this guard in Task 5: arm on spawn (using the push's
> `session_context.timestamp_ms`), disarm on completion, and expose a `tick`
> that fails loops whose budget elapsed.

---

### Task 5: `ClientHelm` — top-level coordinator

**Files:**
- Create: `bedrock-platform/helms/crates/helm-client/src/client.rs`
- Create: `bedrock-platform/helms/crates/helm-client/tests/client_integration_tests.rs`

**Interfaces:**
- Consumes: all prior modules
- Produces: `ClientHelm`, `ClientHelmAction` — the public API for native layers and FFI

- [ ] **Step 1: Write failing integration tests**

```rust
// bedrock-platform/helms/crates/helm-client/tests/client_integration_tests.rs
use helm_client::client::{ClientHelm, ClientHelmAction};
use helm_client::formation::FormationOutput;
use helm_client::ids::LoopId;
use helm_session_contracts::{
    gate::{GateCondition, GateId, GatedDecision},
    push::{SessionContext, SessionPush},
    finding::FindingId,
    urgency::UrgencyIntent,
};

fn ctx() -> SessionContext {
    SessionContext {
        session_id: "sess-1".into(),
        phase: "hypothesis".into(),
        cycle: 1,
        timestamp_ms: 0,
    }
}

fn push(urgency: UrgencyIntent) -> SessionPush {
    SessionPush {
        finding_id: FindingId::new(),
        urgency_intent: urgency,
        payload: serde_json::json!({"msg": "test"}),
        session_context: ctx(),
    }
}

// ── handle_push ─────────────────────────────────────────────────────────────

#[test]
fn first_push_spawns_formation() {
    let mut helm = ClientHelm::new();
    let action = helm.handle_push(push(UrgencyIntent::Informational));
    assert!(matches!(action, ClientHelmAction::SpawnFormation { .. }));
}

#[test]
fn preemptive_push_while_running_pauses_active() {
    let mut helm = ClientHelm::new();
    // Start a formation
    let spawn_action = helm.handle_push(push(UrgencyIntent::Informational));
    let loop_id = match spawn_action {
        ClientHelmAction::SpawnFormation { loop_id, .. } => loop_id,
        other => panic!("expected SpawnFormation, got {other:?}"),
    };
    // Tell helm the formation is running
    helm.formation_started(&loop_id);
    // Now preemptive push
    let action = helm.handle_push(push(UrgencyIntent::Preemptive));
    match action {
        ClientHelmAction::PauseAndInject { paused_id, .. } => {
            assert_eq!(paused_id.as_str(), loop_id.as_str());
        }
        other => panic!("expected PauseAndInject, got {other:?}"),
    }
}

#[test]
fn disruptive_push_while_running_offloads_to_server() {
    let mut helm = ClientHelm::new();
    let spawn_action = helm.handle_push(push(UrgencyIntent::Informational));
    let loop_id = match spawn_action {
        ClientHelmAction::SpawnFormation { loop_id, .. } => loop_id,
        _ => panic!(),
    };
    helm.formation_started(&loop_id);
    // A local formation is already running → Disruptive offloads to the server
    // rather than spawning a second local formation.
    let action = helm.handle_push(push(UrgencyIntent::Disruptive));
    assert!(matches!(action, ClientHelmAction::RequestServerFormation { .. }));
}

#[test]
fn server_formation_started_records_handle_without_blocking_local() {
    let mut helm = ClientHelm::new();
    let spawn_action = helm.handle_push(push(UrgencyIntent::Informational));
    let local_id = match spawn_action {
        ClientHelmAction::SpawnFormation { loop_id, .. } => loop_id,
        _ => panic!(),
    };
    helm.formation_started(&local_id);
    // Disruptive → offload; native asks the server, the server assigns an id,
    // and the native layer reports it back so the device can track it.
    let _ = helm.handle_push(push(UrgencyIntent::Disruptive));
    let handle_id = helm.server_formation_started(
        "srv-formation-1".into(),
        "dd-analysis".into(),
        helm_client::formation::SeedContext { facts: vec![], description: "dd".into() },
    );
    assert_ne!(handle_id.as_str(), local_id.as_str());
    // The handle runs on the server — it is never wall-clock budgeted on device,
    // so even a far-future tick never reports it as expired.
    let expired = helm.tick(u64::MAX);
    assert!(!expired.iter().any(|id| id.as_str() == handle_id.as_str()));
}

#[test]
fn advisory_push_while_running_notifies() {
    let mut helm = ClientHelm::new();
    let spawn_action = helm.handle_push(push(UrgencyIntent::Informational));
    let loop_id = match spawn_action {
        ClientHelmAction::SpawnFormation { loop_id, .. } => loop_id,
        _ => panic!(),
    };
    helm.formation_started(&loop_id);
    let action = helm.handle_push(push(UrgencyIntent::Advisory));
    assert!(matches!(action, ClientHelmAction::Notify { .. }));
}

// ── formation_completed ──────────────────────────────────────────────────────

#[test]
fn formation_completed_queues_temperature_for_submission() {
    let mut helm = ClientHelm::new();
    let spawn_action = helm.handle_push(push(UrgencyIntent::Informational));
    let loop_id = match spawn_action {
        ClientHelmAction::SpawnFormation { loop_id, .. } => loop_id,
        _ => panic!(),
    };
    helm.formation_started(&loop_id);
    helm.formation_completed(
        &loop_id,
        FormationOutput {
            proposals: vec![serde_json::json!({"conclusion": "agree"})],
            temperature: Some(helm_client::formation::TemperatureReading {
                position: "agree".into(),
                conviction: "high".into(),
                subject_ref: "quorum://hypothesis/h-1".into(),
            }),
        },
    );
    let submissions = helm.drain_submissions();
    assert!(!submissions.is_empty());
}

// ── handle_gate ─────────────────────────────────────────────────────────────

#[test]
fn gate_surfaces_as_pending() {
    let mut helm = ClientHelm::new();
    let gate = GatedDecision {
        gate_id: GateId::new(),
        condition: GateCondition::AnyParticipant,
        payload: serde_json::json!({}),
        deadline: None,
    };
    helm.handle_gate(gate);
    assert_eq!(helm.pending_gates().len(), 1);
}

#[test]
fn gate_response_produces_pending_submission() {
    let mut helm = ClientHelm::new();
    let gate = GatedDecision {
        gate_id: GateId::new(),
        condition: GateCondition::AnyParticipant,
        payload: serde_json::json!({}),
        deadline: None,
    };
    let gate_id = gate.gate_id.clone();
    helm.handle_gate(gate);
    helm.respond_to_gate(&gate_id, serde_json::json!({"approved": true}));
    assert!(helm.pending_gates().is_empty());
    let submissions = helm.drain_submissions();
    assert!(!submissions.is_empty());
}

// ── wall-clock budget ────────────────────────────────────────────────────────

#[test]
fn formation_exceeding_wall_clock_budget_is_failed() {
    // 1s budget; ctx() timestamp is 0, so the formation starts at t=0.
    let mut helm = ClientHelm::with_budget_ms(1_000);
    let action = helm.handle_push(push(UrgencyIntent::Informational));
    let loop_id = match action {
        ClientHelmAction::SpawnFormation { loop_id, .. } => loop_id,
        other => panic!("expected SpawnFormation, got {other:?}"),
    };
    // t=2000ms is past the 1s budget → the loop is reported and failed.
    let expired = helm.tick(2_000);
    assert_eq!(expired.len(), 1);
    assert_eq!(expired[0].as_str(), loop_id.as_str());
    // Reported once: a second tick is empty.
    assert!(helm.tick(9_999).is_empty());
}

#[test]
fn completed_formation_is_not_budget_failed() {
    let mut helm = ClientHelm::with_budget_ms(1_000);
    let action = helm.handle_push(push(UrgencyIntent::Informational));
    let loop_id = match action {
        ClientHelmAction::SpawnFormation { loop_id, .. } => loop_id,
        _ => panic!(),
    };
    helm.formation_completed(
        &loop_id,
        FormationOutput { proposals: vec![], temperature: None },
    );
    // Disarmed on completion → never reported as expired.
    assert!(helm.tick(u64::MAX).is_empty());
}
```

- [ ] **Step 2: Run tests — expect compile failure**

```bash
cargo test -p helm-client --test client_integration_tests 2>&1 | head -15
```

- [ ] **Step 3: Implement `client.rs`**

```rust
// bedrock-platform/helms/crates/helm-client/src/client.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use crate::budget::WallClockGuard;
use crate::formation::{FormationOutput, SeedContext};
use crate::gate_surface::{GatedDecisionSurface, GatedDecisionView, PendingGateResponse};
use crate::ids::LoopId;
use crate::registry::{LoopEntry, LoopRegistry};
use crate::router::{RoutingDecision, SeverityRouter};
use crate::temperature::{PendingSubmission, TemperatureQueue, TemperatureSignal};
use helm_session_contracts::{
    gate::{GateId, GatedDecision},
    push::SessionPush,
    urgency::UrgencyIntent,
};
use uuid::Uuid;

/// Action that Client Helm instructs the native layer to execute.
#[derive(Debug)]
pub enum ClientHelmAction {
    /// B — spawn a new **local** formation. `loop_id` is already registered as a
    /// `Local` Running entry. (At most one local formation runs at a time.)
    SpawnFormation { loop_id: LoopId, seed_context: SeedContext },
    /// B — a local formation is already running: ask the **server** to spawn a
    /// sub-formation with these seeds (the "DD job while I wait"). The native
    /// layer issues the server request; when the server returns an id it calls
    /// `server_formation_started` to register the tracked `ServerHandle`. This
    /// runs on the server, so it is NOT wall-clock budgeted on device.
    RequestServerFormation { seed_context: SeedContext },
    /// C — suspend the active formation; the native layer spawns a fresh
    /// formation seeded with its accumulated context + injected_context.
    /// Not a Converge Engine::resume.
    PauseAndInject { paused_id: LoopId, injected_context: SeedContext },
    /// Surface a notification without interrupting the active formation.
    Notify { urgency: UrgencyIntent, message: String },
    /// Nothing required from the native layer right now.
    NoAction,
}

/// Pending submission item — temperature signal or gate response.
#[derive(Debug)]
pub enum ClientSubmission {
    Temperature(PendingSubmission),
    GateResponse(PendingGateResponse),
}

/// The headless Client Helm coordinator.
///
/// Synchronous and pure — no network, no async, no Converge deps.
/// The native layer (iOS, Android, Desktop) owns the SSE connection,
/// feeds events in via `handle_push` / `handle_gate`, executes the
/// returned `ClientHelmAction`, and calls back with lifecycle events.
pub struct ClientHelm {
    registry: LoopRegistry,
    router: SeverityRouter,
    temperature_queue: TemperatureQueue,
    gate_surface: GatedDecisionSurface,
    gate_responses: Vec<PendingGateResponse>,
    budget: WallClockGuard,
    default_budget_ms: u64,
}

/// Default on-device formation wall-clock budget: 5 minutes.
/// Converge does not enforce `time_limit`; Client Helm bounds formations itself.
const DEFAULT_FORMATION_BUDGET_MS: u64 = 5 * 60 * 1_000;

impl ClientHelm {
    #[must_use]
    pub fn new() -> Self {
        Self::with_budget_ms(DEFAULT_FORMATION_BUDGET_MS)
    }

    /// Construct with an explicit per-formation wall-clock budget (ms).
    #[must_use]
    pub fn with_budget_ms(default_budget_ms: u64) -> Self {
        Self {
            registry: LoopRegistry::new(),
            router: SeverityRouter::new(),
            temperature_queue: TemperatureQueue::new(),
            gate_surface: GatedDecisionSurface::new(),
            gate_responses: Vec::new(),
            budget: WallClockGuard::new(),
            default_budget_ms,
        }
    }

    // ── Inbound events from native layer ──────────────────────────────────

    /// Call when an SSE SessionPush arrives from the server.
    #[must_use]
    pub fn handle_push(&mut self, push: SessionPush) -> ClientHelmAction {
        let active_id = self.registry.running_entry().map(|e| e.loop_id.clone());
        // The push carries the session clock; use it as the formation start time.
        let started_at_ms = push.session_context.timestamp_ms;
        let seed = SeedContext {
            facts: vec![push.payload.clone()],
            description: format!("server push: {:?}", push.urgency_intent),
        };
        let decision = self.router.decide(push.urgency_intent, active_id.as_ref(), seed);
        let action = self.apply_routing_decision(decision);
        // Arm a wall-clock budget for any newly spawned formation — Converge
        // does not enforce time_limit, so Client Helm must.
        if let ClientHelmAction::SpawnFormation { loop_id, .. } = &action {
            self.budget.arm(loop_id, started_at_ms, self.default_budget_ms);
        }
        action
    }

    /// Call when a GatedDecision event arrives from the server.
    pub fn handle_gate(&mut self, gate: GatedDecision) {
        self.gate_surface.add_gate(gate);
    }

    /// Call when the native layer has started a spawned local formation.
    pub fn formation_started(&mut self, loop_id: &LoopId) {
        // Formation was already inserted as Running by handle_push.
        // This call is a no-op in the current implementation; reserved
        // for future state tracking (e.g. start timestamp, resource tracking).
        let _ = loop_id;
    }

    /// Call when the server has accepted an offloaded sub-formation (the result
    /// of a prior `RequestServerFormation` action) and assigned it an id.
    /// Records a `ServerHandle` the device tracks but does not run. It does not
    /// occupy the local-running slot and is not wall-clock budgeted here.
    /// Returns the `LoopId` of the tracking entry.
    #[must_use]
    pub fn server_formation_started(
        &mut self,
        server_formation_id: String,
        formation_type: String,
        seed_context: SeedContext,
    ) -> LoopId {
        self.registry
            .spawn_server_handle(server_formation_id, formation_type, seed_context)
    }

    /// Call when a local formation completes. Queues temperature + proposals.
    pub fn formation_completed(&mut self, loop_id: &LoopId, output: FormationOutput) {
        let _ = self.registry.complete(loop_id, output.proposals.clone());
        self.budget.disarm(loop_id);
        if let Some(temp) = output.temperature {
            self.temperature_queue.enqueue(
                TemperatureSignal {
                    position: temp.position,
                    conviction: temp.conviction,
                    subject_ref: temp.subject_ref,
                },
                Uuid::new_v4().to_string(),
            );
        }
    }

    /// Call when the user responds to a gate. Removes gate from surface.
    pub fn respond_to_gate(&mut self, gate_id: &GateId, response: serde_json::Value) {
        if let Some(gate_response) = self.gate_surface.respond(gate_id, response) {
            self.gate_responses.push(gate_response);
        }
    }

    /// Periodic tick from the native layer (which owns the clock). Returns the
    /// loop ids whose wall-clock budget elapsed at `now_ms`; those loops are
    /// marked `Failed`. The native layer must abort the corresponding on-device
    /// formations. This is the enforcement Converge's engine does not provide
    /// (it ignores `time_limit`).
    #[must_use]
    pub fn tick(&mut self, now_ms: u64) -> Vec<LoopId> {
        let expired = self.budget.expired(now_ms);
        for id in &expired {
            let _ = self.registry.fail(id, "wall-clock budget exhausted".into());
        }
        expired
    }

    // ── State inspection for native UI ───────────────────────────────────

    /// Drain all pending submissions (temperature signals + gate responses).
    /// Call after `formation_completed` or `respond_to_gate` to get items to send.
    #[must_use]
    pub fn drain_submissions(&mut self) -> Vec<ClientSubmission> {
        let mut out = Vec::new();
        for t in self.temperature_queue.drain() {
            out.push(ClientSubmission::Temperature(t));
        }
        for g in self.gate_responses.drain(..) {
            out.push(ClientSubmission::GateResponse(g));
        }
        out
    }

    #[must_use]
    pub fn registry_state(&self) -> Vec<&LoopEntry> {
        self.registry.entries()
    }

    #[must_use]
    pub fn pending_gates(&self) -> Vec<GatedDecisionView> {
        self.gate_surface.gate_views()
    }

    // ── Internal ─────────────────────────────────────────────────────────

    fn apply_routing_decision(&mut self, decision: RoutingDecision) -> ClientHelmAction {
        match decision {
            RoutingDecision::SpawnNew { seed_context } => {
                let loop_id = self.registry.spawn(
                    seed_context.description.clone(),
                    seed_context.clone(),
                );
                ClientHelmAction::SpawnFormation { loop_id, seed_context }
            }
            RoutingDecision::OffloadToServer { seed_context } => {
                // The work runs on the server, not on device — do NOT touch the
                // local registry here. The native layer issues the server
                // request; when the server returns an id it calls
                // `server_formation_started` to register the tracked handle.
                ClientHelmAction::RequestServerFormation { seed_context }
            }
            RoutingDecision::QueueAndNotify { urgency, seed_context: _ } => {
                ClientHelmAction::Notify {
                    urgency,
                    message: format!("Server update: {urgency:?}"),
                }
            }
            RoutingDecision::PauseAndInject {
                loop_id_to_pause,
                injected_context,
            } => {
                let _ = self.registry.pause(&loop_id_to_pause, injected_context.clone());
                ClientHelmAction::PauseAndInject {
                    paused_id: loop_id_to_pause,
                    injected_context,
                }
            }
        }
    }
}

impl Default for ClientHelm {
    fn default() -> Self {
        Self::new()
    }
}
```

- [ ] **Step 4: Add `client` to `lib.rs`**

```rust
// bedrock-platform/helms/crates/helm-client/src/lib.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

pub mod budget;
pub mod client;
pub mod formation;
pub mod gate_surface;
pub mod ids;
pub mod registry;
pub mod router;
pub mod temperature;
```

- [ ] **Step 5: Run all tests — expect pass**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo test -p helm-client --locked
```

Expected: all tests across registry, router, inline, and integration pass.

- [ ] **Step 6: Clippy + commit**

```bash
cargo clippy -p helm-client -- -D warnings
git add crates/helm-client/src/client.rs crates/helm-client/src/lib.rs crates/helm-client/tests/client_integration_tests.rs
git commit -m "feat(helms): add ClientHelm top-level coordinator

Synchronous event-driven coordinator: handle_push → ClientHelmAction,
handle_gate, formation_completed, respond_to_gate, drain_submissions,
server_formation_started. Local formations run on device (single
running slot, wall-clock budgeted); Disruptive work while busy is
offloaded to the server via RequestServerFormation and tracked as a
ServerHandle. Integrates LoopRegistry + SeverityRouter +
TemperatureQueue + GatedDecisionSurface. No network, no async, no
Converge deps.

Part of: Session Intelligence Spine Plan 1."
```

---

### Task 5b: `helm-client` — director projection (`DirectorFrame` from session state)

**Why:** `DirectorFrame` is a *projection*, not a new runtime actor — and
`helm-client` is the one place that already holds ordered loop + gate state, so it
owns the projection. Because `helm-client` is domain-agnostic (payloads are
opaque), the human words come from the per-app FFI via a `DomainPresenter` seam:
`helm-client` decides the frame **structure, prompt kind, blocking, ids, and
deadlines**; the app supplies the **copy**. The `version` is the upstream SSE
`sequence` the native layer passes in — never a counter invented here.

> This is a deliberately minimal first projection (gate › running task › idle). The
> rich mapping is co-developed with mobile milestone M3A against real fixtures; the
> seam (trait + `director_snapshot`) is what Plan 1 fixes now.

**Files:**
- Modify: `bedrock-platform/helms/crates/helm-client/Cargo.toml` (add `director-contracts` dep)
- Create: `bedrock-platform/helms/crates/helm-client/src/director.rs`
- Modify: `bedrock-platform/helms/crates/helm-client/src/client.rs` (add `director_snapshot`)
- Modify: `bedrock-platform/helms/crates/helm-client/src/lib.rs` (add `pub mod director;`)
- Create: `bedrock-platform/helms/crates/helm-client/tests/director_tests.rs`

**Interfaces:**
- Consumes: `director-contracts::*`, `helm-session-contracts::gate::GatedDecision`, `crate::formation::LocalFormationIntent`
- Produces: `DomainPresenter` (trait), `GateCopy`, `ProjectionInputs`, `project`, and `ClientHelm::director_snapshot`

- [ ] **Step 1: Add the dep**

```toml
# in bedrock-platform/helms/crates/helm-client/Cargo.toml [dependencies]
director-contracts = { path = "../director-contracts" }
```

- [ ] **Step 2: Write the failing test**

```rust
// bedrock-platform/helms/crates/helm-client/tests/director_tests.rs
use director_contracts::{BlockingState, DirectorPrompt, NowTask};
use helm_client::client::ClientHelm;
use helm_client::director::{DomainPresenter, GateCopy};
use helm_client::formation::LocalFormationIntent;
use helm_session_contracts::gate::{GateCondition, GateId, GatedDecision};

struct TestPresenter;
impl DomainPresenter for TestPresenter {
    fn now_task(&self, intent: &LocalFormationIntent) -> NowTask {
        NowTask { objective: intent.description.clone(), needed_from_user: None, estimated_minutes: Some(2) }
    }
    fn gate_copy(&self, _gate: &GatedDecision) -> GateCopy {
        GateCopy { reason: "Approve the revised wording".into(), consequence: "Formation stays blocked until resolved".into() }
    }
    fn idle_title(&self) -> String { "Nothing needs you right now".into() }
}

fn gate() -> GatedDecision {
    GatedDecision {
        gate_id: GateId::from_string("g-1"),
        condition: GateCondition::AnyParticipant,
        payload: serde_json::json!({}),
        deadline: Some(1_700_000_000_000),
    }
}

#[test]
fn idle_helm_projects_idle_frame() {
    let helm = ClientHelm::new();
    let snap = helm.director_snapshot(7, &TestPresenter);
    assert_eq!(snap.version, 7);
    assert!(snap.frame.prompt.is_none());
    assert!(matches!(snap.frame.blocking, BlockingState::NotBlocking));
}

#[test]
fn pending_gate_becomes_the_scene() {
    let mut helm = ClientHelm::new();
    helm.handle_gate(gate());
    let snap = helm.director_snapshot(9, &TestPresenter);
    assert_eq!(snap.version, 9);
    assert!(matches!(snap.frame.prompt, Some(DirectorPrompt::Gate(_))));
    assert!(matches!(snap.frame.blocking, BlockingState::BlocksFormation));
}
```

- [ ] **Step 3: Implement `director.rs`**

```rust
// bedrock-platform/helms/crates/helm-client/src/director.rs
// Copyright 2024-2026 Reflective Labs
// SPDX-License-Identifier: MIT

use crate::formation::LocalFormationIntent;
use director_contracts::{
    BlockingState, ContextLevel, DirectorFrame, DirectorIntent, DirectorPrompt, DirectorSnapshot,
    GatePrompt, GateVerdict, NowTask, PrimaryAction, SecondaryAction, WaitingFor,
};
use helm_session_contracts::gate::GatedDecision;

/// Human copy for a gate, read by the app from the gate's opaque payload.
pub struct GateCopy {
    pub reason: String,
    pub consequence: String,
}

/// `helm-client` is domain-agnostic — session payloads are opaque. The per-app
/// FFI implements this to supply the human words for a frame. `helm-client` owns
/// the frame STRUCTURE and lifecycle; the app owns the WORDS.
pub trait DomainPresenter {
    fn now_task(&self, intent: &LocalFormationIntent) -> NowTask;
    fn gate_copy(&self, gate: &GatedDecision) -> GateCopy;
    fn idle_title(&self) -> String;
}

/// Plain inputs the projector reads from `ClientHelm` state, kept separate so
/// `project` is pure and unit-testable.
pub struct ProjectionInputs<'a> {
    pub running_intent: Option<&'a LocalFormationIntent>,
    pub pending_gate: Option<&'a GatedDecision>,
}

/// First projection: an unresolved HITL gate is the scene; else the running local
/// formation; else idle. Refined against real fixtures during mobile M3A.
#[must_use]
pub fn project(
    version: u64,
    inputs: ProjectionInputs<'_>,
    presenter: &dyn DomainPresenter,
) -> DirectorSnapshot {
    if let Some(gate) = inputs.pending_gate {
        let copy = presenter.gate_copy(gate);
        let frame = DirectorFrame {
            frame_id: gate.gate_id.as_str().to_string(),
            title: copy.reason.clone(),
            subtitle: None,
            now: None,
            waiting_for: WaitingFor::Server,
            primary: PrimaryAction {
                label: "Approve".into(),
                intent: DirectorIntent::RespondGate {
                    gate_id: gate.gate_id.clone(),
                    verdict: GateVerdict::Approve,
                },
            },
            secondary: vec![SecondaryAction {
                label: "Reject".into(),
                intent: DirectorIntent::RespondGate {
                    gate_id: gate.gate_id.clone(),
                    verdict: GateVerdict::Reject,
                },
            }],
            prompt: Some(DirectorPrompt::Gate(GatePrompt {
                gate_id: gate.gate_id.clone(),
                reason: copy.reason,
                consequence: copy.consequence,
                deadline_ms: gate.deadline,
                condition: gate.condition.clone(),
            })),
            presence: vec![],
            context_trail: vec![ContextLevel::Task],
            blocking: BlockingState::BlocksFormation,
        };
        return DirectorSnapshot { version, frame };
    }

    if let Some(intent) = inputs.running_intent {
        let now = presenter.now_task(intent);
        let frame = DirectorFrame {
            frame_id: "now".into(),
            title: now.objective.clone(),
            subtitle: None,
            now: Some(now),
            waiting_for: WaitingFor::Nobody,
            primary: PrimaryAction {
                label: "Open".into(),
                intent: DirectorIntent::OpenTask { frame_id: "now".into() },
            },
            secondary: vec![],
            prompt: None,
            presence: vec![],
            context_trail: vec![ContextLevel::Task],
            blocking: BlockingState::NotBlocking,
        };
        return DirectorSnapshot { version, frame };
    }

    DirectorSnapshot {
        version,
        frame: DirectorFrame {
            frame_id: "idle".into(),
            title: presenter.idle_title(),
            subtitle: None,
            now: None,
            waiting_for: WaitingFor::Nobody,
            primary: PrimaryAction {
                label: "Refresh".into(),
                intent: DirectorIntent::RequestContext { level: ContextLevel::Session },
            },
            secondary: vec![],
            prompt: None,
            presence: vec![],
            context_trail: vec![ContextLevel::Task],
            blocking: BlockingState::NotBlocking,
        },
    }
}
```

- [ ] **Step 4: Add `director_snapshot` to `ClientHelm` (in `client.rs`)**

```rust
// add `use crate::director::{self, DomainPresenter, ProjectionInputs};` to client.rs imports
// add `use director_contracts::DirectorSnapshot;`

impl ClientHelm {
    /// Project current session/gate/loop state into a versioned `DirectorFrame`.
    /// `version` is the upstream SSE `sequence` the native layer last applied —
    /// NOT a counter invented here — so frames order/dedup consistently with the
    /// rest of the spine. `presenter` supplies domain copy (payloads are opaque).
    #[must_use]
    pub fn director_snapshot(
        &self,
        version: u64,
        presenter: &dyn DomainPresenter,
    ) -> DirectorSnapshot {
        let running_intent = self.registry.running_entry().map(|e| &e.intent);
        // First projection picks any one pending gate. Refine to earliest-deadline
        // ordering during M3A once multi-gate scenes are exercised.
        let pending_gate = self.gate_surface.pending_gates().into_iter().next();
        director::project(
            version,
            ProjectionInputs { running_intent, pending_gate },
            presenter,
        )
    }
}
```

- [ ] **Step 5: Add `pub mod director;` to `lib.rs`** (alphabetical — after `client`, before `formation`).

- [ ] **Step 6: Test + clippy + commit**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo test -p helm-client --locked
cargo clippy -p helm-client -- -D warnings
git add crates/helm-client/Cargo.toml crates/helm-client/src/director.rs crates/helm-client/src/client.rs crates/helm-client/src/lib.rs crates/helm-client/tests/director_tests.rs
git commit -m "feat(helms): project session state into DirectorFrame (helm-client)

helm-client owns the DirectorFrame projection (it already holds ordered
loop + gate state). DomainPresenter seam keeps helm-client domain-agnostic:
it decides frame structure/prompt/blocking/ids/deadlines; the per-app FFI
supplies the words. version = upstream SSE sequence (not a new counter).
First projection: gate > running task > idle; refined with mobile M3A.

Part of: Session Intelligence Spine Plan 1."
```

---

### Task 6: Register crates in Helms workspace

**Files:**
- Modify: `bedrock-platform/helms/Cargo.toml`

- [ ] **Step 1: Add workspace members**

In `bedrock-platform/helms/Cargo.toml`, find the `[workspace]` `members` array and add:

```toml
    "crates/helm-session-contracts",
    "crates/director-contracts",
    "crates/helm-client",
```

Add them after the existing `helm-*` crates to keep the grouping consistent.

- [ ] **Step 2: Verify full workspace builds and tests pass**

```bash
cd /Users/kpernyer/dev/reflective/bedrock-platform/helms
cargo build --workspace --locked
cargo test --workspace --locked
cargo clippy --workspace -- -D warnings
```

Expected: all pass with no new warnings.

- [ ] **Step 3: Commit**

```bash
git add Cargo.toml Cargo.lock
git commit -m "chore(helms): register helm-session-contracts and helm-client in workspace

Both crates compile and test clean within the workspace.
Prerequisite for Plan 2 (Quorum server extensions) and
Plan 3 (mobile FFI + desktop integration).

Part of: Session Intelligence Spine Plan 1."
```

---

## Plan Complete

**Deliverable:** Three new Rust crates in `bedrock-platform/helms/`:
- `helm-session-contracts` — 5 source files, 5 tests, zero heavy deps (server↔client wire boundary)
- `director-contracts` — 5 source files, 3 tests, zero heavy deps (Rust→FFI/UI projection boundary; `DirectorFrame` + vocabulary)
- `helm-client` — 9 source files (incl. `budget.rs`, `director.rs`), 27+ tests, synchronous and portable

**What this unblocks:**
- **AI Director UX (mobile M3A):** `director-contracts` is the canonical
  `DirectorFrame` home and `helm-client::director_snapshot` is its producer. Mobile
  milestone M3A.8 consumes/re-exports these via `mobile-core` — it must **not** fork
  `DirectorFrame`. The `DomainPresenter` seam is where Quorum's per-app FFI maps
  opaque payloads to human copy. (Spec: `KB/04-architecture/2026-06-27-ai-director-mobile-ux-architecture.md`; epic `KB/08-roadmap/2026-06-27-ai-director-ux-epic.md`.)
- The **upstream event/SSE consolidation plan**
  (`2026-06-26-spine-plan-upstream-event-sse.md`): strengthen `runway-app-host`
  (sequence stamping + replay/live SSE) and migrate `helm-coordination` /
  `helm-governed-jobs` onto it. No new crate this pass (ADR
  `2026-06-26-helm-session-host-vs-coordination`, Option 5).
- Plan 2: `helm-session-host` and Quorum server extensions can import
  `helm-session-contracts`.
- Plan 3: `helm-client-ffi` wraps `helm-client` via UniFFI and re-exports
  `director-contracts`. It belongs in `mobile-apps/crates/` (portfolio-level,
  following the existing `shell-ffi` precedent), **not** the per-app `apps/*/ffi`
  pattern.

**Sequencing.** Both prior Plan 2 blockers are now resolved: the HITL mechanism is
settled (terminal-HITL suggestor → `Engine::resume(GateDecision)`, spine §1a), and
the `helm-coordination` reconciliation is decided (strengthen `runway-app-host`
upstream, no new crate — ADR 2026-06-26, Option 5). Plan 2 now sits behind the
**upstream event/SSE consolidation plan**
(`2026-06-26-spine-plan-upstream-event-sse.md`), which ships first.

**Plan 3** is drafted after Plan 1's crates are merged and the mobile M2–M4
milestones are complete (all currently open).
