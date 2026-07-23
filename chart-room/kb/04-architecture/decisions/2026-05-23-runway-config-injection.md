# ADR: Runway Config Injection (env via constructors, not direct reads)

- Date: 2026-05-23
- Status: Accepted
- Decision type: runtime configuration pattern
- Scope: `runtime-runway/crates/*`
- Source-of-truth commits: `09ff7cb`, `63740d2`, `a4f8fd6`, `7be86c0`, `488ed78` (2026-05-23) and `3d28915` (2026-05-24)
- Related: [[../runtime-runway/Architecture - Overview|runtime-runway overview]], [[../runtime-runway/Architecture - Crates|runtime-runway crates]]

## Question

How should Runtime Runway crates obtain their runtime configuration (Stripe keys, Firebase project, local-dev flag, middleware tunables, telemetry endpoints)?

## Decision

Each `runway-*` crate exposes a typed `*Config` struct (e.g. `AccountsConfig`, `MiddlewareConfig`, `TelemetryConfig`, `HostConfig`). The binary (`api-server` or any embedding host) owns env reading via a top-level `RunwayConfig` and constructs each crate's config struct from it. Libraries never read environment variables directly.

- Env is read **once**, in the binary, into `RunwayConfig` at `crates/api-server/src/config.rs:12`.
- Each crate's `*Config` is built by the binary and passed via constructor.
- Crate-local compile-time `const`s stay at the top of the crate that owns them. No global `constants.rs`.

## Options Considered

1. **Read env inside each crate.**
   Rejected. Couples library testability to process env, makes per-crate test setup require env juggling, and scatters knowledge of env var names across the workspace.
2. **Single shared `constants.rs` crate.**
   Rejected. Becomes a junk drawer; couples unrelated crates and hides which value a crate actually needs.
3. **Typed `*Config` struct per crate, constructed by the binary from a top-level `RunwayConfig`.**
   Chosen. Libraries depend on their own typed config, not on env. Binary is the sole env reader.

## Consequences

- All five env-injection refactors landed on **2026-05-23**:
  - `09ff7cb` `runway-accounts`: inject `AccountsConfig` instead of reading env
  - `63740d2` `runway-auth`: take `local_dev` via constructor instead of env
  - `a4f8fd6` `runway-middleware`: inject `MiddlewareConfig` instead of reading env
  - `7be86c0` `runway-accounts`: inject `stripe_webhook_secret` instead of reading env
  - `488ed78` `runway-accounts`: inject remaining Stripe config instead of reading env
- Follow-up `3d28915` (2026-05-24): `runway-storage` consolidated `googleapis` base URLs into an `endpoints` module — same pattern (named constants in the crate that owns them, no global).
- Tests can construct a config directly without setting environment variables.
- Adding a new env var is a binary-level concern (extend `RunwayConfig`, plumb to the right `*Config`). It is not a library concern.

## Follow-Ups

- Apply the same pattern to any new `runway-*` crate from inception.
- Confirm the pattern applies to `application/`, `llm/`, and `runway-app-host/` configs (each already exposes a typed config struct — `AppConfig` / `LlmConfig` / `HostConfig` — so this is largely already in place; scan their constructors for any straggler `std::env::var` calls).
- When [[../commerce-rails/Architecture - Overview|commerce-rails]] grows runtime hosts, mirror this pattern (commerce-rails-stripe currently uses `reqwest` + secrets; the same boundary applies).
