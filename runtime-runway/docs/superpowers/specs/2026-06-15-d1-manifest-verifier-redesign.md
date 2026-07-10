# D1 — Manifest Verifier: Redesign Note

**Status:** **Implemented 2026-06-15** (Phase 1 + Phase 2) — supersedes the runtime-introspection framing in the original D1 backlog entry. All three checks enforced strict-always in `RunwayAppHost::serve()`. The §8 open questions were resolved by implementation: **Option A** (explicit registration API) was taken; the Phase-1/Phase-2 split shipped as designed; check 3 derives `handler_id` from `std::any::type_name` (reviewer may later upgrade to a proc-macro compile-error form if a closure-handler `_test` exposure is ever found). App adoption (apps register domain routes via `route_*` and declare them in `domain_routes`) is the remaining downstream step, tracked in the Quorum handoff.
**Owner:** `[RR-ARCH]` (Runtime-Runway).
**Originating review:** `(reflective-root)/REVIEW_quorum-sense_2026-06-15.md` Round 1 (sharpened by HELMS F3 Round 2).
**RR backlog entry:** `runtime-runway/QUALITY_BACKLOG.md` → `D1`.
**Severity:** A — strict-always gate; `serve()` must `Err` on manifest/router divergence in every environment (no flag, per `CLAUDE.md`).

## 1. Why a redesign

The original D1 entry specifies three checks inside `verify_manifest(&packet, &router) -> Result<()>`, run from `BuiltHost::serve()`. Two of the three are **not implementable as specified** because they assume runtime reflection that neither `axum` nor stable Rust provides:

| Check | Original framing | Feasible at runtime? |
|---|---|---|
| 1 | Every `RouteRegistration { owner: AppDomain }` in `packet.domain_routes` matches a **live route in the `Router`** | **No.** `axum::Router` (0.8) exposes no public API to enumerate registered paths/methods. The router is an opaque `Service`; there is no `routes()` accessor. We cannot ask the router "what paths do you serve?" |
| 2 | Every mounted `HelmModule` reports `module_state() != Shell::Default` | **Partially.** Implementable, but `HelmModule` has **no `module_state()` method today** — it must be added to the trait. |
| 3 | No route handler resolves to a symbol containing `for_test` / `_test` / `test_only` | **No.** Stable Rust has no handler-symbol reflection. A boxed `axum` handler erases its source identifier; there is no portable way to recover `apply_proposals_for_test` from the live `MethodRouter`. |

The intent behind all three is sound — catch (1) routes the manifest claims but the app doesn't serve, (2) the planned-vs-mounted lie D2 surfaced, (3) `_test` handlers reaching production (HELMS F4's `apply_proposals_for_test`). The intent survives; the **runtime-introspection mechanism does not**. This note replaces the mechanism.

## 2. Root-cause framing

The original spec tried to verify the router **after** it was assembled by an opaque builder — i.e. reconcile two independently-produced artifacts (the JSON manifest and the `axum::Router`) after the fact. That reconciliation is only possible if one of them is introspectable, and the `Router` is not.

The fix is to **invert the data flow**: make the manifest's route list the *single source from which the router is built* (or against which registration is checked), so divergence is caught **by construction or at build time**, never needing to interrogate a finished `Router`.

## 3. Replacement for check 1 (route ↔ manifest)

Two viable designs. Both make the **manifest-declared route list authoritative** and remove runtime router introspection.

### Option A — Explicit route-registration API (recommended)

Routes are not handed to the host as a pre-built `axum::Router`. Instead the app/module registers each route through a typed API that records `(method, path, owner, handler)` in a registry the host owns. The host:

1. Builds the `axum::Router` **from** the registry (so the served router is a pure function of the registered set).
2. Runs `verify_manifest` by comparing the **registry's** `(method, path, owner)` set against `packet.domain_routes` / `packet.mounted_modules[].routes`. Both sides are plain data; the diff is exact.

```rust
// Illustrative — not final API.
pub struct RouteSpec {
    pub method: Method,
    pub path: &'static str,
    pub owner: RouteOwner,
}

impl RunwayAppHostBuilder {
    pub fn route<H, T>(self, spec: RouteSpec, handler: H) -> Self
    where H: Handler<T, ()> + ...;
}
```

- **Pro:** manifest↔router divergence becomes structurally impossible to *miss* — the verifier compares two `Vec`s, no reflection. Also gives us a natural home for check 3 (see §4) and for D5's `ownership_exempt_routes` cross-check (already a manifest field).
- **Con:** apps stop passing a free-form `Router`; migration touches every app's wiring. `with_spa` (D3a, shipped) and `modify_router` remain the escape hatches for non-registered static/fallback services, which are *not* domain routes and are correctly out of the manifest.

### Option B — Build-time proc-macro emits the route list

A `#[runway::route(method = "POST", path = "/inquiry/{id}/signal", owner = AppDomain)]` attribute on each handler emits, at compile time, both the `axum` registration and a `const`/`inventory`-collected `RouteSpec`. The verifier compares the collected specs against the manifest.

- **Pro:** keeps handler and its route declaration co-located; no manual registry calls.
- **Con:** proc-macro + `inventory`-style global collection is heavier machinery; harder to debug; introduces a build-time codegen dependency. Distributed collection (`inventory`/`linkme`) has known footguns under static linking and test harnesses.

**Recommendation:** **Option A.** It needs no new macro/codegen dependency, the verifier is a trivial set-diff, and it composes with the manifest fields we already added (`domain_routes`, `mounted_modules`, `ownership_exempt_routes`, `deploy_contracts`). Option B can layer on later as ergonomic sugar over the same registry if hand-registration proves noisy.

## 4. Replacement for check 3 (`_test` handler scan)

Once routes are registered as data (Option A) or declared via attribute (Option B), the `_test` check stops being "introspect a live handler symbol" and becomes "inspect a declared identifier at build time":

- **With Option A:** require each `RouteSpec` to carry a stable `handler_id: &'static str` (or derive it from the registration callsite via `stringify!`/`std::any::type_name` of the handler fn). The verifier rejects any registered `handler_id` containing `for_test`, `_test`, or `test_only`. This runs over **declared data**, not a boxed handler — deterministic and portable.
- **With Option B:** the proc-macro sees the handler's `fn` identifier at expansion time and can either reject the forbidden tokens directly (compile error) or emit the identifier into the collected `RouteSpec` for the same build-time check.

Either way the check moves from *runtime handler reflection* (impossible) to *build-time inspection of a declared identifier* (trivial). `type_name`/`stringify!` are best-effort but sufficient: the failure mode we're guarding against (`apply_proposals_for_test` wired to a public route) is exactly a named function whose identifier carries the token.

> Note: `std::any::type_name` is documented as not guaranteed stable across compiler versions and not for programmatic identity. We use it only as a **defense-in-depth string heuristic** for the `_test` token, never for routing identity. The authoritative identity is the explicitly-registered `(method, path, owner)`.

## 5. Check 2 stays — but needs a trait method

Check 2 is kept as originally intended and is the smallest piece. It requires adding state reporting to the module trait:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModuleState {
    /// Default shell — module mounted but not wired to live state.
    Shell,
    /// Live state wired.
    Live,
}

#[async_trait]
pub trait HelmModule: Send + Sync + 'static {
    // ... existing ...
    fn module_state(&self) -> ModuleState { ModuleState::Shell } // default = honest
}
```

The verifier asserts: for every module the manifest marks `mount_kind: "mounted"`, the live `module.module_state()` must be `Live`; a module reporting `Shell` while the manifest says `"mounted"` fails `serve()`. This is the structural enforcement D2 needs (HELMS H-2026-06-15-01 wiring flips Quorum's modules from `Shell` to `Live`).

Defaulting `module_state()` to `Shell` makes silence honest: a module that forgot to report state is treated as a shell, so the lie fails closed.

## 6. Revised acceptance

Integration test in `runway-app-host/tests/` constructs a host that:

1. **Declares a domain route in the manifest that is never registered** → `serve()` returns `Err` (manifest claims a route the registry lacks).
2. **Registers a domain route absent from the manifest** → `serve()` returns `Err` (registry serves a route the manifest doesn't declare). *(New symmetric direction — only possible because the registry is now introspectable; the original spec could only check one direction.)*
3. **Marks a module `mount_kind: "mounted"` while its `module_state()` returns `Shell`** → `serve()` returns `Err`.
4. **Registers a handler whose `handler_id` contains `_test`** → `serve()` returns `Err`.

No flag; all four hold in every environment. Passing tests = D1 done. Promotes `RP-MANIFEST-MATCHES-ROUTER` (+ drift check) and gives `RP-NO-FEATURE-FLAG-SOFTENING` a citable enforcement home.

## 7. Sequencing & blast radius

- **Phase 1 (low-risk, lands first):** add `ModuleState` + `HelmModule::module_state()` (default `Shell`) and wire **check 2** through `serve()`. Self-contained; no app-API change.
- **Phase 2 (the redesign):** introduce the route-registration API (Option A), build the served router from the registry, wire **checks 1 + 3** as set-diff + identifier scan. This is the app-facing migration — every marquee app moves from "pass a `Router`" to "register routes." Coordinate with the active Quorum handoff; `with_spa`/`modify_router` remain for non-domain services.
- D1 should land **while D5 is in flight** so it catches regressions on D5's session-ownership wiring (per the original entry's blocks/blocked-by). The `ownership_exempt_routes` cross-check folds naturally into the Phase-2 registry diff.

## 8. Open questions for the panel

1. Option A vs B — confirm Option A (registration API) over the proc-macro, or request a spike comparing ergonomics on Quorum's real route set.
2. Is the Phase-1/Phase-2 split acceptable, i.e. can check 2 ship and close the D2 lie before the larger registration migration lands?
3. Does `handler_id` derivation via `type_name`/`stringify!` satisfy the F4 `_test`-exposure guard, or does the panel want the stricter proc-macro compile-error form for check 3?
