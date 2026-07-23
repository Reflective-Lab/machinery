# RFL-154 — Cut the helm-operator-control seam — Implementation Plan

> Architect analysis 2026-07-07 (read-only over helms main e6b9c0a). Execution: subagent-driven, scratch clone, branch e12/rfl-154-cut-operator-seam, PR to helms main.

## ⚠️ Coupling reality — materially SMALLER than the two-edges assumption

1. **The prio-agent-ops edge is a clean file-internal split.** `prio-agent-ops/src/lib.rs` is two disjoint concerns: the capability manifest (lines 9–42, depends on capability-core — STAYS, capability-registry consumes `MODULE`) and the operator/receipt vocabulary (lines 44–703 + tests 705–990, depends only on serde + sha2 — MOVES). operator-control re-exports exactly 18 items = the entire vocabulary section, none of the manifest.
2. **The workbench-backend edge is largely vestigial.** operator-control holds `OperatorApp<S>` (http_api.rs:29,39) but never invokes it; its own preview methods implement live-feed logic directly. `OperatorApp` is dead weight dragging in application-kernel, capability-registry, organism-domain, organism-runtime, truth-catalog, chrono, uuid. Only genuinely-used items: three thin serde view structs (views.rs:22-113). Cutting requires (a) moving three view structs, (b) deleting a never-called field and dead error plumbing.

## Verdicts
- **axum: KEEP** — `HelmModule::router() -> axum::Router` is the seam contract itself (helm-module-contracts/src/lib.rs:130). Constraint: new vocab submodules must be pure (serde+sha2), no axum import, so pure consumers don't pull transport.
- **polars: REMOVE** — only used by two Parquet seed-loaders (pipeline.rs:546-660) called from one HTTP handler. Relocate seed-loading to the app/seed-gen layer; mounting app supplies `ShowcasePipelineInput` (mirrors the existing readiness-feed injection).

## Ownership end state
- `helm-module-contracts`: OWNS vocabulary (new `operator_receipts` module: all 18 types + hashing/validation helpers + OperatorControlError) + preview views (new `operator_preview` module: OperatorControlPreview/Backing, OperatorReceiptFamilyView, operator_receipt_families()). Adds sha2 = "0.11" dep; proptest + trybuild as dev-deps.
- `prio-agent-ops`: manifest-only (keeps lines 1–42; deps shrink to capability-core).
- `workbench-backend`: consumer of contracts (not prio-agent-ops); keeps OperatorApp/dashboard for its own concerns.
- `helm-operator-control`: depends on contracts ONLY (drops workbench-backend, prio-agent-ops, polars); deletes OperatorApp field; single typed OperatorControlError→ApiError mapper covering only reachable variants; removes the pub-use re-export bridges (no shims).
- arena `cross-extension-smoke`: dep swap prio-agent-ops → helm-module-contracts; fuzzy_operator_stack.rs:22 imports repoint (all 10 types are in the moving set).

## Tasks
| Task | Scope | Gate |
|---|---|---|
| T1 | Vocabulary → contracts/operator_receipts.rs (verbatim incl. tests 705-990); sha2 dep | cargo test -p helm-module-contracts |
| T2 | Preview views → contracts/operator_preview.rs, wired to T1 | cargo test -p helm-module-contracts |
| T3 | prio-agent-ops → manifest-only; Cargo deps shrink | cargo test -p prio-agent-ops -p capability-registry |
| T4 | workbench-backend repoints to contracts | cargo test -p workbench-backend |
| T5a | operator-control: drop both deps, delete OperatorApp field, typed error mapper, fix module_test imports | cargo test -p helm-operator-control |
| T5b | operator-control: remove polars, relocate seed loaders, inject ShowcasePipelineInput | cargo test -p helm-operator-control; cargo tree grep polars = 0 |
| T6 | arena cross-extension-smoke dep+use swap | cargo test -p cross-extension-smoke |
| T7 | Property (proptest: packet_id determinism, single-field mutation changes id, sha256: format, AuthorityEffect::None invariant, serde roundtrips) + compile-fail (trybuild: constructor-gate + no-resolve regression guards) + soak (#[ignore], 100k operator flows, no id/hash drift) + seam rustdoc + helms KB (Foundation Contracts, Operator Control Common Module, Module Map) + boundary registry (current-system-map.md Helm anchor) + CHANGELOG + drift-check.py exit 0 | full helms test + drift check |

Sequencing: T1 → T2 → {T3, T4, T5a, T6 parallel} → T5b → T7.

## Named risks
1. Duplicate-symbol window: T1/T2 must land before T3/T4/T5; trybuild guard catches shim reintroduction.
2. `sha256:` id stability: move hashing verbatim; property tests assert determinism; check runway/arena snapshots.
3. workbench-backend residual prio_agent_ops imports: grep after T4.
4. run_pipeline HTTP contract changes with polars removal: injection preserves behavior; document moved responsibility.
5. contracts crate is published (0.2.1, runway consumes): minor-version surface expansion — version bump; dim-semver flags it.
6. Error-variant surface: every reachable error maps to sane HTTP status; unreachable arms removed, not silenced.
7. Downstream repos importing vocab through operator-control re-exports: grep sibling repos before T5a lands; repoint to contracts.

Full analysis with path:line citations: RFL-154 Linear comment / SDD report a27eeec.
