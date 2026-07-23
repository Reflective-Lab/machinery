---
tags: [adr, commerce-rails, release]
source: llm
date: 2026-07-02
---

# ADR: commerce-rails crates are publishable (MIT)

**Status:** Accepted (backfill — decision was implemented in code before the
record was written)

## Question

`QF-2026-06-02-14`: commerce-rails carried `license = "UNLICENSED"` /
`publish = false`, blocking runtime-runway and app repos from consuming it
via crates.io and forcing path-dependencies across repo boundaries.

## Options considered

1. Keep private (path-deps or a private registry).
2. Make the crates publishable under MIT like the rest of the platform.

## Decision

Option 2. `commerce-rails/Cargo.toml` now declares `license = "MIT"`,
`publish = true` (shipped by v0.2.2); `runtime-runway/Cargo.toml` consumes
`commerce-rails-stripe` with a versioned, publishable dep spec.

## Consequences

- Commercial-authority crates follow the same release train and yank
  discipline (`KB/release-history.md`) as the rest of the fleet.
- Revenue-sensitive logic is public source; secrets and pricing stay in
  configuration, never in the crates.

## Revisit

If a future commercial reason requires closed source, a new ADR must
supersede this one; do not flip `publish` silently.

Originating finding: `QF-2026-06-02-14` (closed 2026-07-02).
