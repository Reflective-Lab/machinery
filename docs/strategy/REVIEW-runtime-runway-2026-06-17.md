# Runtime Runway — New Normal / Positioning Readiness Review

*Status: component review, v1, 2026-06-17. Produced per
`docs/strategy/00-REVIEW-TASK.md` against the strategy snapshots in this
repo. Repo: `runtime-runway`. Evidence commit: `e5c3e6c` (strategy package) +
`2754e95` (D1 stateful routes / ownership passthrough).*

## Executive summary

Runtime Runway is **ready to host the convened-burst reference shape**
(`quorum-sense`) and is **architecturally aligned** with the New Normal's
engineering posture (typed Rust substrate, app execution container, boundary
enforcement). It is **not ready** to carry the full governed commitment
substrate across all three commitment shapes.

| Verdict band | Count | Meaning |
|---|---|---|
| **Ready** | 9 | Burst hosting, core infra crates, boundary model |
| **Partial** | 11 | Exists but incomplete, burst-scoped, or not production-wired |
| **Missing** | 8 | Required by positioning; no RR implementation |
| **Wrong-layer** | 7 | Correctly owned by Converge, Axiom, Helm, Mosaic, or Commerce Rails |

**Top finding:** The stack one-pager assigns Runtime Runway *ambient operate*
machinery — event streams, resume, **scheduled or triggered ambient work** —
but RR today is primarily a **request/response Cloud Run host** with
in-process SSE. The inversion (continuous machine-driven action governed by a
sparse human core) demands a durable ambient runtime RR does not yet provide.
That gap blocks standing-governance shapes outright and limits async-burst /
degraded-connectivity cases for multi-sovereign.

**Second finding:** Multi-sovereign shapes break RR's implicit **single-org
Firebase identity model**. Cross-org signers, selective-disclosure-safe storage,
and symmetric receipt transport are absent here and must not be bolted onto
burst-only auth without an explicit platform RFC.

**Third finding:** Receipts, canonical core, projections, drift detection, and
reopen semantics are **correctly not in RR** — but RR must expose durable
event/provenance hooks those layers can bind to. Today `EventHub` is
process-local; `EventLog` is append-only but not wired as the cross-shape
ambient spine.

---

## 1. Requirement extraction

Requirements below are what the positioning demands of **Runtime Runway**
specifically — the operating substrate per `stack-one-pager.md` and the
shape lens in `00-REVIEW-TASK.md`. Universal substrate requirements (1)–(6)
are included only where RR is the natural owner.

### 1.1 All-shape requirements (any commitment mode)

| ID | Requirement | Source |
|---|---|---|
| AS-01 | Typed, compiled, observable core runtime (Rust-first posture) | `reflective-paradigm.md` §5 engineering posture; `stack-one-pager.md` Engineering posture |
| AS-02 | Standard app execution container — apps supply packets, not servers | `stack-one-pager.md` layers; `kb/Architecture/App Execution Container.md` |
| AS-03 | Auth with explicit authority context on every protected route | Six requirements #2; `runway-auth` |
| AS-04 | Multi-tenant storage (`org_id` / `app_id` paths) local + remote | Substrate for commitments, projections, receipts |
| AS-05 | Append-only event ledger for provenance and replay | Six requirements #4; Operate + Detect drift |
| AS-06 | Telemetry / tracing / structured logging at substrate boundary | Operate + Detect drift; observability of ambient action |
| AS-07 | Secrets and fail-fast bootstrap | Production substrate |
| AS-08 | Deployment paths (Cloud Run, Firebase Hosting URL model, deploy contracts) | Fleet of marquee apps |
| AS-09 | **Ambient operate runtime** — scheduled/triggered continuation between human returns | `stack-one-pager.md` RR layer; loop Operate phase |
| AS-10 | Durable event streams (not only in-process) for cross-instance ambient work | Burst async + standing autonomy |
| AS-11 | Resume / reconnect after disconnect (storage + transport) | `domain-fit-scan.md` connectivity axis |
| AS-12 | Commercial entitlement gating at the host boundary | Commerce Rails integration; `EntitlementProjection` |
| AS-13 | Manifest ↔ router strict enforcement (no silent boundary drift) | Marquee App Contract / D1 |
| AS-14 | Horizontal scale without corrupting mutating session state | Standing autonomy + burst scale-out |

### 1.2 Burst-scoped requirements (convened burst adds; other shapes may reuse)

| ID | Requirement | Source |
|---|---|---|
| BS-01 | Realtime transport (SSE) with cursor replay for live/async rooms | `quorum-sense-substrate-scoping.md` async + realtime |
| BS-02 | Per-session admission lease for mutating routes | D5 / convened burst concurrency |
| BS-03 | Helm module mount surface (operator-control, governed jobs) | App Execution Container |
| BS-04 | SPA static hosting with history-API deep links | D3a / fleet UX |
| BS-05 | Single-org Firebase identity + custom-claim app entitlements | Burst default authority model |

### 1.3 Multi-sovereign requirements (assumption-breakers for RR)

| ID | Requirement | Source |
|---|---|---|
| MS-01 | Cross-org identity and authority for signers and arbiters | `00-REVIEW-TASK.md` shape lens; `quorum-sense-substrate-scoping.md` |
| MS-02 | Selective-disclosure-safe storage and transport | Same |
| MS-03 | Tamper-evident **symmetric** receipts verifiable by all parties | Six requirements #4; `tally-escrow` proof path |
| MS-04 | Obligation projection storage paths per party (not only per org) | `domain-fit-scan.md` Shape B |
| MS-05 | Dispute / arbiter surfaces at the transport + hosting layer | Shape B |
| MS-06 | Async / intermittent connectivity deployment profile | Maritime, clinical async cases |

### 1.4 Standing-governance-of-autonomy requirements (assumption-breakers for RR)

| ID | Requirement | Source |
|---|---|---|
| SG-01 | Long-horizon delegation scope enforcement at runtime | `00-REVIEW-TASK.md` shape lens |
| SG-02 | Runtime telemetry ingestion compared against mandate | Operate → Detect drift |
| SG-03 | Policy gates + intervention thresholds (host-level hooks) | `triage-keeper` / `vigil-care` |
| SG-04 | Kill / reopen under degraded connectivity | `quorum-sense-substrate-scoping.md` Vigil open questions |
| SG-05 | Executable (machine-actionable) projection delivery | Projection-to-machine |
| SG-06 | Long-running workers separate from request-bounded Cloud Run | `quorum-sense-substrate-scoping.md` needs-human |

### 1.5 Explicitly wrong-layer (RR must not build; must integrate)

| ID | Requirement | Owner |
|---|---|---|
| WL-01 | Canonical commitment / mandate semantics | Axiom |
| WL-02 | Governed projections + integrity contract (8 distortion classes) | Axiom + apps |
| WL-03 | Admission, promotion, refusal, reopen events | Converge |
| WL-04 | Merkle / Lamport receipt chain | Converge (promoted from Quorum) |
| WL-05 | Drift detection engines (reality vs projection vs mandate) | Mosaic / Fathom class |
| WL-06 | Formation selection (humans, agents, dissenters) | Organism |
| WL-07 | Billing, payouts, commercial disputes | Commerce Rails |

---

## 2. Component mapping

| Requirement | Primary RR mapping | Evidence |
|---|---|---|
| AS-01 | Workspace Rust crates, `edition = 2024`, no `unsafe` | `AGENTS.md`; all `crates/runway-*` |
| AS-02 | `runway-app-host`, `AppExecutionPacket`, `RunwayAppHostBuilder` | `crates/runway-app-host/src/lib.rs`, `kb/Architecture/App Execution Container.md` |
| AS-03 | `runway-auth::AuthLayer`, `AuthContext` | `crates/runway-auth/src/middleware.rs` |
| AS-04 | `runway-storage::StorageKit` local/remote | `crates/runway-storage/src/lib.rs`, `kb/Architecture/Crate Map.md` |
| AS-05 | `EventLog` trait, Firestore/redb backends | `crates/runway-storage/src/traits/event.rs` |
| AS-06 | `runway-telemetry::init` → OTel, Sentry, JSON logs | `crates/runway-telemetry/src/lib.rs` |
| AS-07 | `runway-secrets::Secrets` | `crates/runway-secrets/src/lib.rs` |
| AS-08 | `ops/templates/`, `just api-deploy`, Firebase hosting docs | `kb/Building/Deployment.md`, `ops/templates/cloud-run-deploy.sh` |
| AS-09 | — | **No crate** |
| AS-10 | `EventHub` (in-process), Pub/Sub Terraform only | `crates/runway-app-host/src/realtime.rs`; `ops/infra/terraform/modules/pubsub/main.tf` |
| AS-11 | `SyncEngine` (local ↔ remote) | `crates/runway-storage/src/local/sync.rs` |
| AS-12 | `runway-accounts`, `EntitlementProjection` schema, CR adapter | `crates/runway-accounts/`; `kb/Architecture/App Execution Container.md` |
| AS-13 | `verify_manifest` in `BuiltHost::serve()` | `crates/runway-app-host/src/builder.rs` |
| AS-14 | `SessionOwnershipLayer` + `LeaseStore` | `crates/runway-app-host/src/ownership.rs`, `crates/runway-storage/src/traits/lease.rs` |
| BS-01 | `EventHub`, `sse::router`, `EventCursor` | `crates/runway-app-host/src/sse.rs`, `realtime.rs` |
| BS-02 | D5 middleware | `ownership.rs`; `QUALITY_BACKLOG.md` D5 |
| BS-03 | `HelmModule` mount + `ModuleState` | `crates/runway-app-host/src/module.rs` |
| BS-04 | `SpaConfig`, `with_spa` | `crates/runway-app-host/src/builder.rs` |
| BS-05 | Firebase + `requiring_app` | `crates/runway-auth/src/middleware.rs` |
| MS-01 – MS-05 | — | **No implementation** |
| SG-01 – SG-06 | Partial hooks only (telemetry, events, leases) | See §4 |
| WL-01 – WL-07 | Integration points only | Boundaries in `App Execution Container.md` |

**Milestone cross-reference (`MILESTONES.md`):**

| Milestone | Status | Positioning relevance |
|---|---|---|
| Immediate — App execution container | ✅ Done | AS-02, BS-03, AS-13 |
| M1 — Shared infra compiles | ✅ Done | AS-01, AS-03–07 |
| M2 — GCP production ready | ❌ Open | AS-08 production gate |
| M3 — Reference app wired end-to-end | ❌ Open | AS-05, AS-06, AS-12 proof |
| M4 — Tauri offline-first | ❌ Open | AS-11, SG-04 |
| M5 — Marquee fleet online | ❌ Open | AS-08, AS-12 fleet |

**Quality backlog cross-reference (`QUALITY_BACKLOG.md`):**

| Item | State | Positioning relevance |
|---|---|---|
| D1 manifest verifier | Done | AS-13 |
| D2 mount_kind reconciliation | Open (app/Helms) | BS-03 honesty |
| D3a `with_spa` | Done | BS-04 |
| D3b `runway-app-shell` | Open | AS-12 fleet UX |
| D4 deploy template + math-base tags | Done (RR); ops adoption pending | AS-08 |
| D5 SessionOwnership | In progress (Tier-2 pending) | BS-02, AS-14 partial |
| D6 `deploy_contracts` | Partial (field + materialization; verifier leg open) | AS-08 |

---

## 3. Readiness verdict

### 3.1 All-shape

| ID | Verdict | Notes |
|---|---|---|
| AS-01 | **Ready** | Rust workspace, clippy-clean contract, aligns with paradigm engineering posture |
| AS-02 | **Ready** | `runway-app-host` proven via quorum-sense panel review (`MILESTONES.md`) |
| AS-03 | **Partial** | Firebase single-org only; no cross-org federation (blocks MS-01) |
| AS-04 | **Partial** | StorageKit solid; remote EventLog query not production-proven (M3) |
| AS-05 | **Partial** | Append-only log exists; not promoted as cross-shape provenance spine |
| AS-06 | **Partial** | Crate ready; M3 unchecked — traces not proven flowing in reference app |
| AS-07 | **Partial** | Crate ready; M2 secrets population open |
| AS-08 | **Partial** | Deploy template shipped (D4); Terraform audit, releases CDN, CI/CD open (M2, M5) |
| AS-09 | **Missing** | No scheduler, job runner, Cloud Tasks/Pub/Sub consumer in RR |
| AS-10 | **Partial** | In-process `EventHub` only; Pub/Sub provisioned in Terraform, no RR runtime |
| AS-11 | **Partial** | `SyncEngine` scaffolded; M4 checklist largely open; no transport-level resume |
| AS-12 | **Partial** | Schema locked; `runway-app-shell` missing (D3b); Stripe webhook path open (M2) |
| AS-13 | **Ready** | D1 strict-always enforced in `serve()` |
| AS-14 | **Partial** | D5 admission lease ships; write-side fencing explicitly deferred (`RP-NO-LEASE-WITHOUT-FENCING-V1`) |

### 3.2 Burst-scoped

| ID | Verdict | Notes |
|---|---|---|
| BS-01 | **Ready** | SSE + cursor replay in `runway-app-host` |
| BS-02 | **Partial** | D5 Tier-1 done; Firestore Tier-2 + fencing gap |
| BS-03 | **Partial** | Mount surface ready; D2 shell-vs-mounted reconciliation open |
| BS-04 | **Ready** | D3a shipped with history-API fix |
| BS-05 | **Ready** | Sufficient for intra-org burst; not for multi-sovereign |

### 3.3 Multi-sovereign

| ID | Verdict | Notes |
|---|---|---|
| MS-01 | **Missing** | `AuthContext` is `uid` + `org_id` + app claims — no cross-org party model |
| MS-02 | **Missing** | No field-level disclosure, encrypted views, or trust-boundary storage |
| MS-03 | **Wrong-layer** | Converge / Quorum integrity spine — RR must expose transport, not own Merkle logic |
| MS-04 | **Missing** | Firestore paths are `orgs/{orgId}/apps/{appId}/...` — single-tenant assumption |
| MS-05 | **Wrong-layer** | Dispute semantics in Helm/Converge; RR hosts routes only |
| MS-06 | **Partial** | Async SSE + offline sync direction exists; not validated for hostile latency |

### 3.4 Standing governance of autonomy

| ID | Verdict | Notes |
|---|---|---|
| SG-01 | **Missing** | No delegation-scope runtime primitive in RR |
| SG-02 | **Partial** | Telemetry crate exists; no mandate-comparison pipeline in RR |
| SG-03 | **Wrong-layer** | Policy gates belong in Mosaic/Converge; RR provides hooks only |
| SG-04 | **Partial** | Offline sync + local storage direction (M4); kill/reopen semantics absent |
| SG-05 | **Wrong-layer** | Axiom projection contract + app actuation |
| SG-06 | **Missing** | Cloud Run request model insufficient for continuous operate phase |

### 3.5 Wrong-layer confirmations (integration health)

| ID | Verdict | RR integration today |
|---|---|---|
| WL-01 – WL-02 | **Wrong-layer** ✅ | Apps mount domain routes; no Axiom types in RR |
| WL-03 – WL-04 | **Wrong-layer** ✅ | No Converge promotion logic in RR — correct |
| WL-05 | **Wrong-layer** ✅ | No drift engine in RR — correct |
| WL-06 | **Wrong-layer** ✅ | `converge-llm` is distribution, not Organism |
| WL-07 | **Wrong-layer** ✅ | Stripe logic relocated to Commerce Rails per MILESTONES boundary debt |

---

## 4. Shape-specific gaps (assumption-breakers)

### 4.1 Multi-sovereign — what burst never needed

| Gap | Why burst didn't need it | What RR must add |
|---|---|---|
| **Cross-org identity** | Quorum assumes intra-org good faith (`quorum-sense-substrate-scoping.md`) | Federated signer identities, arbiter roles, party-scoped JWT claims — likely a new auth mode or adapter, not a tweak to `requiring_app` |
| **Selective-disclosure storage** | All participants see the same org-scoped Firestore tree | Party-scoped document views, encrypted fields, or split stores with proof hooks; storage contract extension |
| **Symmetric receipt transport** | Central org owns the receipt chain | RR must carry **verifiable** event envelopes across parties without becoming the Merkle owner — e.g. signed webhook/outbox pattern bound to Converge receipts |
| **Obligation paths** | Projections target human roles inside one org | Storage paths and authz model for per-party obligation views on a shared canonical agreement |
| **Dispute hosting** | HITL synthesis gate inside Helm | Neutral route namespace + auth for arbiter surfaces; still Wrong-layer for dispute semantics |
| **Intermittent connectivity** | Live SSE assumes mostly-online clients | Durable outbox, offline queue, conflict rules beyond "remote wins on status" (`sync.rs` today) |

**RR verdict for multi-sovereign:** **Not ready.** The execution container can host routes, but identity, storage, and transport models are burst-shaped.

### 4.2 Standing governance of autonomy — what burst never needed

| Gap | Why burst didn't need it | What RR must add |
|---|---|---|
| **Operate is the product** | Burst is episodic; Cloud Run cold-start acceptable | Long-running workers, schedulers, stream processors — likely GPU/worker paths (`ops/deploy/gpu/`) plus Pub/Sub consumers |
| **Delegation scope provability** | Session lease covers one mutating inquiry | Mandate-bound scope tokens checked on every ambient action; escalation on out-of-scope — new primitive, not D5 session lease |
| **Telemetry vs mandate** | Drift is session-scoped in Quorum | Continuous metric/event pipeline into Detect-drift layer; RR owns ingestion + retention, not comparison logic |
| **Intervention / kill / reopen** | Human is in the room | Host-level circuit breakers, mandate revocation propagation, graceful agent shutdown — needs durable control plane |
| **Degraded connectivity** | Desktop burst clients reconnect to SSE | Tauri offline-first (M4) + authoritative kill that works when cloud is unreachable |
| **Executable projections** | Humans read projections | Machine-consumable policy artifacts at the host boundary — Wrong-layer for semantics; RR delivers signed blobs + actuation hooks |
| **Scale-out safety** | `--max-instances=1` pin | D5 v1 explicitly lacks write fencing (`RP-NO-LEASE-WITHOUT-FENCING-V1`) — insufficient for continuous multi-writer autonomy |

**RR verdict for standing autonomy:** **Not ready.** Request-bounded Cloud Run + in-process `EventHub` is the wrong runtime shape for continuous governed operation.

### 4.3 What convened burst still lacks (honest partials)

Even the reference shape is not production-complete on RR:

- M3 end-to-end wiring unchecked (telemetry, secrets, remote EventLog).
- D3b shared app shell not built.
- D5 Tier-2 (Firestore emulator CI) pending; fencing deferred.
- M2 production infra (Terraform apply, Firebase rules, billing webhook) open.

Burst is **architecturally ready**, not **operationally ready**.

---

## 5. Gap-ordered work plan

Sequencing respects: burst proof first (already in flight), then substrate
promotion for shape two, then ambient runtime for shape three. RR work only;
sibling items noted.

### Phase 0 — Finish burst production path (weeks, not quarters)

*Unblocks convened burst as commercial reference.*

| # | Work | Repo | Depends on |
|---|---|---|---|
| 0.1 | Close M3 reference wiring (telemetry, secrets, remote EventLog, auth on all routes) | RR + quorum-sense | M2 secrets |
| 0.2 | Complete D5 Tier-2 + document `--max-instances=1` until D5.1 fencing | RR | — |
| 0.3 | Ship `runway-app-shell` (D3b) | RR | Commerce Rails QF-CR-05 |
| 0.4 | Close M2 Terraform audit + Firebase rules deploy | RR `ops/` | — |
| 0.5 | Quorum adopts deploy template; delete app-owned `STRIPE_*` | quorum-sense | D4 (RR done) |

### Phase 1 — Durable ambient spine (all shapes)

*Highest-leverage RR gap for the New Normal inversion.*

| # | Work | Repo | Depends on |
|---|---|---|---|
| 1.1 | **Ambient job substrate** — Pub/Sub (or Cloud Tasks) consumer crate in RR: trigger → `StorageKit` + outbound events; document contract for Converge/Organism callbacks | RR | M2 Pub/Sub live |
| 1.2 | Promote `EventHub` → **durable event stream** backed by `EventLog` + optional Pub/Sub fan-out for cross-instance SSE | RR | 1.1 |
| 1.3 | Outbox pattern on `EventLog.append` for at-least-once ambient delivery | RR | 1.1 |
| 1.4 | Platform RFC: ambient operate API (what apps vs Converge vs Organism enqueue) | KB / panel | — |

### Phase 2 — Multi-sovereign substrate (RR slice)

*Do not start before Phase 1 event spine exists — symmetric receipts need durable transport.*

| # | Work | Repo | Depends on |
|---|---|---|---|
| 2.1 | Platform RFC: cross-org identity model | Bedrock / RR auth | Panel |
| 2.2 | `runway-auth` federation adapter (multi-party claims) | RR | 2.1 |
| 2.3 | Storage contract: party-scoped paths + selective disclosure fields | RR `runway-storage-contract` | 2.1 |
| 2.4 | Signed event envelope transport for cross-party verification (RR carries; Converge signs) | RR + Converge | 1.3, WL-04 |
| 2.5 | Prove with `tally-escrow` / `concord-supply` | App repos | 2.2–2.4 |

### Phase 3 — Standing autonomy runtime (RR slice)

*Panel decision (bucket D) whether this shares Converge loop or needs a long-horizon worker tier.*

| # | Work | Repo | Depends on |
|---|---|---|---|
| 3.1 | **Needs-human:** separate long-horizon runtime vs shared loop? | Panel | `quorum-sense-substrate-scoping.md` §Decisions |
| 3.2 | Delegation-scope admission primitive (distinct from session lease) | RR `runway-app-host` | 3.1 |
| 3.3 | D5.1 write-side fencing for multi-writer safety | RR | D5 Tier-2 |
| 3.4 | Worker deploy path (Cloud Run jobs / GPU workers / Modal) wired to ambient substrate | RR `ops/` | 1.1 |
| 3.5 | Offline kill/reopen — extend M4 sync with mandate revocation checkpoint | RR | M4 |
| 3.6 | Prove with `triage-keeper` or `vigil-care` | App repos | 3.2–3.5 |

### Sibling work (not RR; block RR promotion)

| Work | Owner |
|---|---|
| Promote Merkle receipt chain from Quorum → Converge | Converge / Bedrock |
| Promote drift detection spine (Mnemos / SenseMap class) → Mosaic | Mosaic |
| EntitlementStore v2 + push refresh (QF-CR-03, CR-06) | Commerce Rails |
| Helm live module state (H-2026-06-15-01) | Helms |
| Axiom canonical core + projection contract types | Axiom / Bedrock |

---

## 6. Findings for the ledger

Append to `QUALITY_BACKLOG.md` when promoted. Format per `AGENTS.md` +
`QUALITY_BACKLOG.md` conventions.

### RR-STRAT-01 — No ambient operate substrate

- **Severity:** A
- **State:** Open
- **Owner:** `[RR-ARCH]`
- **Origin:** Strategy review 2026-06-17 (`00-REVIEW-TASK.md`); `stack-one-pager.md` RR layer
- **Scope:** RR claims scheduled/triggered ambient work but has no job runner, scheduler integration, or Pub/Sub consumer. Blocks standing-autonomy shapes and weakens async burst.
- **Acceptance:** Documented ambient enqueue + execute contract; at least one reference worker consuming Pub/Sub and writing `EventLog`; quorum or plumb can trigger ambient continuation without a human HTTP request.
- **Effort:** L
- **Codex-safe now?:** Partial — can scaffold consumer crate + contract tests against emulator; needs panel sign-off on API boundary.
- **Evidence:** `stack-one-pager.md:104-106`; no consumer in `crates/`; Pub/Sub only in `ops/infra/terraform/modules/pubsub/main.tf`

### RR-STRAT-02 — Event stream is process-local

- **Severity:** A
- **State:** Open
- **Owner:** `[RR-ARCH]`
- **Origin:** Strategy review 2026-06-17
- **Scope:** `EventHub` is in-memory `broadcast` — events do not survive instance restart or reach other Cloud Run instances. Insufficient for cross-instance ambient operate or durable SSE catch-up at scale.
- **Acceptance:** SSE replay sourced from durable log; multi-instance fan-out verified under load test.
- **Effort:** M
- **Codex-safe now?:** Yes — wire `EventHub` publish to `EventLog.append` + replay on subscribe.
- **Evidence:** `crates/runway-app-host/src/realtime.rs:119-189`

### RR-STRAT-03 — Cross-org identity absent

- **Severity:** A
- **State:** Open
- **Owner:** `[RR-ARCH]` + panel
- **Origin:** Strategy review 2026-06-17; `quorum-sense-substrate-scoping.md:75-76`
- **Scope:** `runway-auth` is Firebase single-org. Multi-sovereign shapes cannot authenticate independent parties or arbiters.
- **Acceptance:** Platform RFC approved; `AuthContext` carries party identity across trust boundary; contract tests for two-org scenario.
- **Effort:** L
- **Codex-safe now?:** No — needs panel RFC before implementation.
- **Evidence:** `crates/runway-auth/src/middleware.rs:39-50`; `MILESTONES.md` multi-tenancy path

### RR-STRAT-04 — Selective-disclosure storage absent

- **Severity:** A
- **State:** Open
- **Owner:** `[RR-ARCH]`
- **Origin:** Strategy review 2026-06-17; paradigm §6 multi-sovereign machinery
- **Scope:** `StorageKit` paths assume full org visibility. No party-scoped views or disclosure-safe fields.
- **Acceptance:** `runway-storage-contract` extension + contract tests; Concord can store per-party obligation projection without leaking canonical core fields.
- **Effort:** L
- **Codex-safe now?:** No — needs RFC aligned with RR-STRAT-03.
- **Evidence:** `kb/Architecture/Crate Map.md` Firestore path model; `crates/runway-storage-contract/`

### RR-STRAT-05 — D5 admission lease without write fencing

- **Severity:** B
- **State:** Open (documented; D5.1 follow-up)
- **Owner:** `[RR-ARCH]`
- **Origin:** Strategy review 2026-06-17; `RP-NO-LEASE-WITHOUT-FENCING-V1`
- **Scope:** Standing autonomy and multi-instance burst require stale-writer prevention. D5 v1 is admission-only.
- **Acceptance:** D5.1 ships; standard updated; multi-instance integration test proves stale writer rejected at storage layer.
- **Effort:** M
- **Codex-safe now?:** Yes — design in `docs/superpowers/specs/2026-06-15-d5-session-ownership-design.md` lineage.
- **Evidence:** `kb/05-engineering/standards/RP-NO-LEASE-WITHOUT-FENCING-V1.md`; `crates/runway-storage/src/traits/lease.rs:58-60`

### RR-STRAT-06 — Delegation-scope runtime primitive absent

- **Severity:** B
- **State:** Open
- **Owner:** `[RR-ARCH]`
- **Origin:** Strategy review 2026-06-17; standing-autonomy shape lens
- **Scope:** Session lease ≠ mandate delegation. No "outside scope escalates" enforcement at RR boundary.
- **Acceptance:** Typed delegation scope checked on ambient job enqueue and mutating routes; escalation event to Converge reopen path.
- **Effort:** L
- **Codex-safe now?:** No — depends on RR-STRAT-01 + panel loop factoring.
- **Evidence:** `docs/strategy/quorum-sense-substrate-scoping.md:82-90`

### RR-STRAT-07 — Production substrate incomplete (M2/M3)

- **Severity:** B
- **State:** Open
- **Owner:** `[RR-ARCH]`
- **Origin:** `MILESTONES.md`; strategy review confirms burst operational gap
- **Scope:** Terraform audit, Firebase rules, telemetry flowing, secrets fail-fast, release CDN — all open.
- **Acceptance:** M2 + M3 checklists closed for quorum-sense reference.
- **Effort:** L
- **Codex-safe now?:** Partial — Terraform/Firebase script work is codifiable; needs GCP credentials for apply.
- **Evidence:** `MILESTONES.md` M2–M3 sections

### RR-STRAT-08 — `runway-app-shell` missing (D3b)

- **Severity:** B
- **State:** Open (existing D3b)
- **Owner:** `[RR-ARCH]`
- **Origin:** `QUALITY_BACKLOG.md` D3b; strategy fleet requirement
- **Scope:** Shared auth bootstrap + entitlement widget — every shape needs entitled access to substrate surfaces.
- **Acceptance:** Per D3b acceptance; quorum deletes local shell copies.
- **Effort:** M
- **Codex-safe now?:** Partial — widget depends on CR EntitlementProjection stability (published).
- **Evidence:** `QUALITY_BACKLOG.md` D3b; `kb/Architecture/App Execution Container.md` EntitlementProjection

### RR-STRAT-09 — Offline-first sync incomplete (M4)

- **Severity:** B
- **State:** Open
- **Owner:** `[RR-ARCH]`
- **Origin:** `MILESTONES.md` M4; SG-04 degraded connectivity
- **Scope:** `SyncEngine` exists but M4 checklist (re-embedding, Tauri hook, conflict rules) open. Needed for kill/reopen under disconnect.
- **Acceptance:** M4 checklist closed.
- **Effort:** M
- **Codex-safe now?:** Yes — sync engine work is RR-scoped.
- **Evidence:** `crates/runway-storage/src/local/sync.rs`; `MILESTONES.md` M4

### RR-STRAT-10 — Cloud Run request model vs continuous operate

- **Severity:** D (needs-human)
- **State:** Open
- **Owner:** Panel
- **Origin:** `quorum-sense-substrate-scoping.md` §Decisions; strategy review
- **Scope:** Standing autonomy may need a separate long-horizon runtime rather than extending Cloud Run request hosts.
- **Acceptance:** Panel decision recorded; RR architecture doc updated.
- **Effort:** —
- **Codex-safe now?:** No
- **Evidence:** `docs/strategy/quorum-sense-substrate-scoping.md:118-119`

---

## 7. Bottom line

**Is Runtime Runway ready to support the New Normal positioning?**

- **As the execution container for convened-burst apps:** **Yes, architecturally;
  not yet operationally.** The app-host boundary, infra crates, manifest
  enforcement, SSE, and deploy-contract materialization are the right
  substrate. M2/M3/D3b/D5 completion is the remaining burst path.

- **As the operating substrate for all three commitment shapes:** **No.** The
  inversion toward continuous autonomous action requires an **ambient operate
  runtime** RR has not built. Multi-sovereign requires **cross-org identity and
  disclosure-safe storage** RR has not built. Standing autonomy likely requires
  **long-horizon workers and delegation-scope enforcement** beyond today's
  Cloud Run + session lease model.

- **Layer discipline:** RR correctly **does not** own canonical core,
  projections, receipts, drift, or commercial authority. The risk is not
  overreach — it is **under-building the shared ambient spine** that those
  layers need to run the loop across shapes.

**Recommended next RR investment:** Phase 0 (burst production) in parallel with
Phase 1.1 (ambient job substrate). That single addition does more for
substrate credibility than any burst-only polish.

---

## Canonical links

- `docs/strategy/00-REVIEW-TASK.md` — review brief
- `docs/strategy/the-new-normal.md` — worldview
- `docs/strategy/reflective-paradigm.md` — substrate + shapes
- `docs/strategy/stack-one-pager.md` — RR layer charter
- `docs/strategy/quorum-sense-substrate-scoping.md` — shape promotion seams
- `kb/Architecture/App Execution Container.md` — RR boundary authority
- `MILESTONES.md` — delivery state
- `QUALITY_BACKLOG.md` — active RR findings
