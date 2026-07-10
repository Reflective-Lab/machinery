# RP-NO-FEATURE-FLAG-SOFTENING

**Status:** Active. Promoted 2026-06-15 alongside D5 SessionOwnership.
**Originating finding:** D5 implementation revealed `AuthLayer.local_dev` as the only existing softening switch in runway. D1's manifest verifier ships strict-always; this standard codifies the rule for future primitives.
**Source review:** `REVIEW_quorum-sense_2026-06-15.md` Round 2.5 self-correction §3; `BOUNDARY_REGISTRY.md` Marquee App Contract rule 7.

## What

No platform check ships with a feature flag, `--strict-mode` switch, env-var toggle, or any other mechanism that lets a caller weaken the check.

This applies to:
- `runway-app-host` manifest verifier (D1)
- `runway-app-host` SessionOwnershipLayer (D5)
- All future RR-owned strict checks

It does NOT apply to:
- Infrastructure-availability gating (e.g. `FIRESTORE_EMULATOR_HOST` env that skips a test when an emulator isn't running). The test assertions are unchanged when the env is set; the switch is whether the test runs at all, not whether the check is enforced.
- Configuration of behavior (e.g. `SessionOwnershipLayer::ttl(...)` accepting different durations, or `holder_id(...)` setting the lease identity). Tuning and identity configuration are not softening.

## Why

Softening switches are how strict checks become advisory: someone sets the flag "temporarily" to ship, the flag becomes permanent, and the check becomes a comment. The runway codebase had exactly one (`AuthLayer.local_dev`) at the time of D5 — we don't add a second.

## How to check (drift)

PR review rejects any new platform primitive that:
- Reads an env var to disable a check.
- Accepts a `bool` or enum that disables a check.
- Has a code path that returns "success" without running the check, controlled by config.

Standard exception: tests that need to mock backend availability may env-gate themselves, but the production code under test must not.

## Links

- Standard `RP-NO-LEASE-WITHOUT-FENCING-V1` (sibling standard for D5).
- Marquee App Contract rule 7 (`BOUNDARY_REGISTRY.md`).
