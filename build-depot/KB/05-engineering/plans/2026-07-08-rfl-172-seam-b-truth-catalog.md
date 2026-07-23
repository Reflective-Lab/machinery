# RFL-172 — Seam B: truth-catalog split (mechanism → foundation-bound, CRM content app-side)

> Architect analysis 2026-07-08 (helms main). Full path:line analysis in RFL-172 Linear comment / SDD transcript a07fb536. Execution: scratch clone, branch e12/rfl-172-truth-catalog-seam, PR to helms main.

## Coupling reality
1. The mechanism/content line is an INVERSION, not a file boundary: every src file mixes both; the crate bottoms out in a global `TRUTHS` const read by free functions (find_truth/admit_truth_intent/converge_binding_for_truth/...). Core work = invert the global into injected `TruthCatalog<'a>` + `PackResolver` + `IntentOverlay`; content crate re-binds them.
2. helm-truth-execution confirmed clean (B1 trim was the last edge). capability-* enters via exactly two content-side places → mechanism ends with ZERO capability edges.
3. axiom-truth stays mechanism (truth-compile IS mechanism); converge-manifold-adapters feature-force follows it (dissolves at Wave-2 unification, NOT removed in-window).

## Topology
- Mechanism keeps crate name `truth-catalog` (born-as, aliased to helm-truth-catalog at Wave-2 import — application-kernel precedent; no env!(CARGO_PKG_NAME) hazard).
- Content = NEW `crates/crm-truths` (app-side FOREVER; never filtered to foundation). Gets: TRUTHS const, 24 .feature files, 13 CRM evaluators, CrmPackResolver (suite_pack_id + prio pack consts + find_module reach), CrmIntentOverlay, CrmOrganismRecipes, catalog-bound free fns, integrity/snapshot tests, example + applet fixtures.
- Edge: crm-truths → truth-catalog (app→foundation). converge.rs:606 panic → Err(UnknownModule).

## Typed boundary
`TruthKey` newtype (kebab-case grammar, parse-don't-validate) at the runtime string crossing ONLY (HTTP keys, literal lookups); const definitions keep &'static str + integrity test enforces grammar. `TruthCatalog::find(&TruthKey)`. Trait signatures in the architect analysis.

## Consumers
- helm-governed-jobs → MECHANISM ONLY (owner constraint): JobStreamState gains injected catalog+overlay (pattern of the existing injected TruthExecutionModule); mounting binary injects CRM instances. Gate: cargo tree shows zero crm-truths/capability-registry.
- workbench-backend → both (types/admission from truth_catalog; catalog-bound fns from crm_truths).
- showcase/crm-helm truth files are ORPHANED (no cargo edge today) — flag, not a gate.

## Tasks
T1 scaffold crm-truths | T2 mechanism refactor 1: TruthKey+TruthCatalog+registry API | T3 mechanism refactor 2: PackResolver+IntentOverlay, capability-* edges deleted, panic→Err | T4 content move | T5 governed-jobs mechanism-only injection | T6 workbench repoint | T7 mechanism quality suite (property/negative/trybuild/soak, fixture catalog) | T8 docs one-story (incl. current-system-map Helm Truth State counts unchanged) + drift-check.
T1→T2→T3→T4→{T5,T6}→T7→T8.

## Named risks
(1) Seam-A overlap: disjoint job_stream.rs hunks (B: 55-61, 329, 605-625, state 106-130; A: zero lines by re-export design); single Cargo.toml rebase point. (2) Overlay inversion behavior drift — mounting binary MUST inject CrmIntentOverlay; before/after compiled-intent snapshot for qualify-inbound-lead. (3) panic→Result failure-mode change — negative test + message preserved. (4) pack-id vector stability — snapshot guard. (5) orphaned showcase truths — flag only. (6) Wave-2: crm-truths must never reach foundation — dim-layering hard-fail edge. (7) manifold feature-force stays in mechanism Cargo.toml in-window.
