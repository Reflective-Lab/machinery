# RP-MANIFEST-MATCHES-ROUTER

**Status:** Active (all three checks enforced). Promoted 2026-06-15 (Phase 1); Phase 2 (checks 1 & 3) enforced 2026-06-15.
**Originating finding:** `QUALITY_BACKLOG.md` → D1 (manifest verifier, strict-always).
**Source review:** `REVIEW_quorum-sense_2026-06-15.md` Round 1 (HELMS F3 Round 2); design in `docs/superpowers/specs/2026-06-15-d1-manifest-verifier-redesign.md`.

## What

An app's `runway.app.json` manifest must not lie about what the running host actually serves. `RunwayAppHost::serve()` rejects divergence in **every** environment (no flag — see `RP-NO-FEATURE-FLAG-SOFTENING`). Three reconciliations:

1. **Routes** — every `domain_routes` entry the manifest declares with `owner: AppDomain` must correspond to a route registered via `RunwayAppHostBuilder::route_*`, and vice versa (both directions). **Enforced.**
2. **Module state** — every module the manifest marks `mount_kind: "mounted"` must be mounted in the host and report `ModuleState::Live`. A module that is absent, or present but `Shell`, is the planned-vs-mounted lie. **Enforced.**
3. **No `_test` handlers** — no registered route resolves to a handler whose `std::any::type_name` contains `for_test` / `_test` / `test_only`. **Enforced.**

## Why

The manifest is the contract every other layer reads (deploy template, boundary registry, panel review). If it can claim a module is wired when it is a default shell — or claim routes it doesn't serve — the contract is fiction and downstream automation builds on sand. D2 surfaced exactly this: modules `.mount(...)`-ed in code while still shells, with the manifest implying live state. D1 makes the lie fail closed at boot.

## How to check (drift)

`runway-app-host::serve()` calls `verify_manifest(&packet, &modules, &routes)` before binding and returns `Err` on any divergence — in every environment, no flag. The route surface is introspectable because app-domain routes are registered as data via `route_get/route_post/route_put/route_patch/route_delete`, and the served router is built from those registrations (so the manifest and the live surface cannot drift). Covered by `builder::tests::verify_*`, `registered_routes_are_served_and_pass_verification`, `serve_rejects_test_handler_via_type_name`, and `serve_rejects_mounted_shell_lie_before_binding`.

`module_state()` defaults to `Shell` and registration derives `handler_id` from `std::any::type_name`, so both an unreported module and a test-only handler fail closed.

## Status of enforcement

| Check | Mechanism | State |
|---|---|---|
| 1 — AppDomain routes | registration set-diff (both directions) in `verify_manifest` | **Enforced** |
| 2 — module state | `module_state()` reconciliation in `verify_manifest` | **Enforced** |
| 3 — `_test` handlers | `type_name` token scan in `verify_manifest` | **Enforced** |

### Known scope limits (carried, not gaps)

- Check 1/3 cover `owner: AppDomain` routes registered via `route_*`. **Module** routes (`mounted_modules[].routes`) are governed by check 2 + the manifest's `mounted_modules` list; making module routes data-introspectable is future work if a module is found exposing a `_test` handler.
- `std::any::type_name` is a best-effort string (not guaranteed stable, and a **closure** handler yields `{{closure}}` without the fn name). It reliably catches the F4 case (a named `*_for_test` fn) but is defense-in-depth, never routing identity.

## Links

- Redesign note: `docs/superpowers/specs/2026-06-15-d1-manifest-verifier-redesign.md`.
- `RP-NO-FEATURE-FLAG-SOFTENING` — D1 is strict-always; this is its citable enforcement home.
- Marquee App Contract (`BOUNDARY_REGISTRY.md`).
