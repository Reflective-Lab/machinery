# RFL-171 — Seam A: event/session/storage substrate → Bedrock-owned injection contracts

> Architect analysis 2026-07-08 (runway main 3e1fa59, helms main, atelier main, bedrock-consolidated). Full path:line analysis in the RFL-171 Linear comment / SDD transcript a4332cbd.

## Coupling reality
1. **Runway RFL-128 WIP branch is ALREADY MERGED to runway main** — no landing prerequisite; delete the stale local branch. runway-app-host already path-deps helm-module-contracts 0.3.0 and re-exports HelmModule/ModuleState — the dual-home + re-export pattern is proven in-tree and is Seam A's template.
2. **The seam is narrower than billed**: what moves is the event hub (EventHub/Handle/Envelope/Cursor/Subscription + sse), the lease/session-ownership CONTRACT (LeaseStore + LeaseScope/LeaseRecord/AcquireOutcome/RenewOutcome), and the event-ledger trait (EventLog/SyncableEventLog/StoredEvent/EventQuery) with SubstrateError. StorageKit (documents/vectors/objects), RunwayAppHost/builder/manifest types, and SessionOwnershipLayer STAY app-side.
3. **SessionOwnershipLayer cannot move yet** — bound to runway_auth::AuthContext + tower. The lease contract moves (that IS the ownership semantics: CAS + TTL + steal/renew); the axum middleware stays as a consumer. AuthContext→OrgIdentity neutralization = RFL-171b, not smuggled in.
4. **Zero source edits in helm-session-host/coordination/governed-jobs** — runway re-exports keep every import resolving. Seam A owns NO hunks in job_stream.rs → composes cleanly with Seam B.

## Decisions
- NEW dual-homed crate `contracts/crates/helm-event-substrate` (helms + Bedrock foundation/helm at Wave 2). NOT folded into helm-module-contracts (would push tokio/chrono/uuid/tokio-stream onto every contracts consumer). publish=true 0.1.0 (4.0.0 in Bedrock). Features: default=["sse"] (axum SSE); "memory" (off-default) = InMemoryEventLog + InMemoryLeaseStore — the honest second implementor, must satisfy the same contract assertions as runway's redb impls (contract-suite parity property test).
- Errors typed: SubstrateError (was runway_storage::traits::Error) — moved, re-exported from runway-storage so unmoved traits keep compiling.
- EventHub moves WHOLE (already backend-neutral over Option<Arc<dyn EventLog>>); the traits with two impls are EventLog/LeaseStore.
- crm-helm repatriation: atelier scenario deps = helm-module-contracts + helm-event-substrate(memory,sse) + application-kernel/storage; 7 one-line import repoints; NEW headless composition root (in-memory kernel store + EventHub::with_capacity + InMemoryLeaseStore, driven via tower oneshot — no TCP, no RunwayAppHost). The runway-backed main.rs moves to an app home (DESTINATION = owner decision, see risks); helms/showcase/ deleted.
- Staged atelier crm-helm copy is a stale partial byte-copy — regenerate from showcase then apply the diff (do not trust it).

## Tasks
T0 baseline green x4 repos | T1 substrate crate + event/lease/error moves + carried tests | T2 hub+sse moves (verbatim, tests carried) | T3 memory feature + parity property tests | T4 runway-storage rewire (impls target moved traits; re-exports) | T5 runway-app-host rewire (delete bodies, re-export; ownership/builder fixups) | T6 spine regression: 3 crates green with ZERO source edits (git diff --stat = Cargo.lock only) — if any source edit needed, the re-export design failed: STOP | T7 crm-helm repatriation + helms/showcase deletion (gated on destination decision) | T8 Bedrock import of substrate (or fold into Wave 2/B8) | T9 quality wave (negative/property/compile-fail/soak; rustdoc; KB one-story; CHANGELOGs; boundary registry).
Sequencing: T0→T1→T2→T3→{T4,T5}→T6→T7→{T8,T9}.

## Named risks
(1) RESOLVED: runway WIP branch already merged. (2) Seam-B overlap: none by design — repointing job_stream.rs line 68 to helm_event_substrate is FORBIDDEN in Seam A (post-window cleanup). (3) publish/version-table wiring for the new crate. (4) SessionOwnershipLayer stays (RFL-171b). (5) sequence/replay drift — carried hub tests are the equivalence oracle. (6) memory impls are new code, not re-exports — contract-parity tests required. (7) stale atelier copy. (8) T7 gated on the runway-backed main's destination repo (owner call).
