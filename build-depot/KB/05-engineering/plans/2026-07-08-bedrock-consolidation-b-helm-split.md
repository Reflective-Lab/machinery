# Bedrock Consolidation Plan B — Helm Headless Split (two waves)

> Architect analysis 2026-07-08 over helms main (post-RFL-154), bedrock-consolidated consolidation/main, runtime-runway, arena-tests. Owner decision: **Wave 1 now; Seams A+B; then Wave 2.**

## ⚠️ Coupling reality — the spec's 15-crate spine is NOT import-closed

**Tier 1 (closed, 9 crates, importable now):** helm-module-contracts (from contracts/ mini-workspace), helm-session-contracts, director-contracts→helm-director-contracts, helm-client, application-kernel→helm-kernel, application-storage→helm-storage, capability-core→helm-capability-core, helm-truth-execution (AFTER trimming its DEAD truth-catalog dep — src has zero truth_catalog uses; Cargo.toml:28 vestige), helm-operator-control (RFL-154 cleaned).

**Tier 2 (blocked, 5 crates):** helm-session-host, helm-coordination, helm-governed-jobs — pervasive runtime-runway substrate use (runway_app_host::{EventHub, EventHubHandle, EventCursor, EventEnvelope, EventSubscription, sse}, SessionOwnershipLayer; runway_storage::{LeaseStore, StorageKit}); truth-catalog→helm-truth-catalog — mechanism + CRM content (qualify-inbound-lead, submit-expense-report, score-inbound-fit) + capability_registry::find_module app-reach (converge.rs:2).

**Spec corrections:**
- **capability-registry stays app-side** — it IS the 21 prio-* registrations (lib.rs:1-24 imports every prio MODULE), not a mechanism; the spec's spine list contradicted its own "prio-* stays out" rule. Only capability-core (serde-only mechanism) imports.
- Root Cargo.toml `# Plan B` block: DROP the stale prio-agent-ops line (arena-tests main already dropped that dep post-RFL-154; the consolidated arena copy is stale — re-sync before re-inclusion).
- truth-catalog's converge-manifold-adapters dep is feature-forcing only (no src use) — dissolves under workspace feature unification.

**Acceptance-gate reality:** cross-extension-smoke, helm-coordination-headless, helm-realtime-stem-headless need Tier 1 only → green in Wave 1. helm-multiuser-convergence-headless needs helm-coordination (Tier 2) → explicit tracked SKIP until Wave 2, expiry 2026-08-15.

## Seams gating Wave 2 (RFL-128 continuation, same discipline as RFL-154)

**Seam A — event/session substrate (preferred A1):** extract EventHub/EventHubHandle/EventCursor/EventEnvelope/EventSubscription/sse::event_stream/SessionOwnershipLayer (runway-app-host src/realtime.rs, sse.rs, ownership.rs) + the LeaseStore/StorageKit traits the spine uses into a foundation crate (new helm-event-substrate, or fold pure types into helm-module-contracts which runway already consumes); runway-app-host re-exports for compat (its RFL-128 pattern). A2 (import runway-app-host closure) REJECTED — drags runway-auth/middleware/telemetry, decision-14 violation.

**Seam B — truth-catalog split:** mechanism (truth-definition types, dispatch, catalog struct) → helm-truth-catalog (foundation); CRM content bodies + find_module lookup → app-side content crate depending on the mechanism. helm-governed-jobs then deps mechanism only.

## Dual-home decision (expiry 2026-08-15, aligned RFL-153)

Bedrock imports WITH history (filter-repo path-filter + path-rename; deterministic → incremental re-sync). Helms KEEPS its copies (desktop, workbench-backend, runway build from them during the window). Spine one-truth transfers to Bedrock by convention: helms-side spine edits forbidden except sync-fixes. Contracts version story: helms keeps 0.3.0 (runway path-deps it); Bedrock carries 4.0.0 lockstep; separate trees, zero interference; cutover repoints runway to registry 4.0.0.

## Mechanics

filter-repo (Plan A import-one.sh conventions: SHA-verify vs snapshot manifest, read-only clone, --allow-unrelated-histories merge) with:
`--path` per Tier-1 crate dir, `--path-rename contracts/crates:foundation/helm/contracts/crates`, `--path-rename crates:foundation/helm/crates`. Wave 2 re-runs with Tier-2 paths after seam SHAs land.

Renames via dep-key aliases (Plan A precedent) so intra-spine `use application_kernel::` idents don't churn: application-kernel={package="helm-kernel"}, application-storage={package="helm-storage"}, capability-core={package="helm-capability-core"}, director-contracts={package="helm-director-contracts"}. NO env!("CARGO_PKG_NAME") in any moving crate (verified — no replay-envelope hazard). Version literals → { workspace = true } at 4.0.0. Publish gating: helm-module-contracts earns publish (runway consumer); others earn via scenarios/smoke consumers + tests.

Workspace integration: members += foundation/helm/crates/* + contracts path; uncomment/corrected Plan-B dep block; re-sync arena from arena-tests main; re-include 3 of 4 excluded members (+cross-extension-smoke), repoint ../bedrock-platform/helms/... deps to workspace; delete imported sub-workspace roots; multiuser scenario stays excluded w/ tracked SKIP.

Arena dims un-SKIP: dim-layering classifier places foundation/helm/* at L3 (deps allowed: L1 converge, L2 axiom/organism contracts); any L3→runtime-runway edge = hard fail. axum in helm crates ALLOWED (HelmModule::router contract, RFL-154 verdict); tonic in truth-execution/governed-jobs must be feature-gated (risk 7).

## Tasks
B0 issue+manifest+branch | B1 helms sync-fix: trim truth-execution dead dep | B2 Tier-1 filter-repo import | B3 renames+aliases+rename-table | B4 workspace integration Wave 1 + arena re-sync + re-includes | B5 Wave-1 gate: workspace tests + 3/4 scenarios + arena dims (multiuser tracked SKIP) | B6 Seam A (own issue) | B7 Seam B (own issue) | B8 Tier-2 import | B9 4/4 gate + zero L3→app edges | B10 dual-home issue + doctrine + PR.
Sequencing: B0→B1→B2→B3→B4→B5→{B6,B7}→B8→B9→B10. Wave 1 (B0-B5) merges independently.

## Named risks
(1) runway substrate seam is the biggest gap — spec assumed it away; B6 gates B8. (2) capability-registry drags 21 prio crates if imported — verdict app-side. (3) truth-catalog CRM content must not enter foundation. (4) stale arena copy refs prio-agent-ops — re-sync first. (5) contracts dual-version 0.3.0/4.0.0 — safe while trees separate; cutover repoints runway. (6) feature-unification (_chat/strum) — dim-crate-footprint detects. (7) tonic in Tier crates — feature-gate. (8) sha256 receipt-id stability — property tests + snapshot check.

Full path:line analysis: Plan agent report (SDD ledger, RFL Plan-B issue comment).
