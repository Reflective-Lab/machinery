# Approved Libraries (Golden Path)

Build-Depot is the factory authority: **repositories own local truth, Build-Depot
owns judgment and coordination.** This catalog is that judgment for dependencies.
It names one blessed library per job and the version line the factory standardises
on. Repos still declare their own manifests — this doc does not own anyone's
`Cargo.toml` or `package.json`. It sets the policy that a drift detector and
`cargo-deny` make observable.

## Why this exists

Two failure modes it guards against:

1. **Library sprawl** — two libraries for almost the same job (two HTTP clients,
   two date libraries, two error crates). Every duplicate is a second thing to
   learn, patch, and audit, for no gain.
2. **Version drift** — the same library pinned at different versions across
   projects. Coordinated *inside* a Rust workspace via `[workspace.dependencies]`;
   **not** coordinated *across* projects today. Concrete example at time of
   writing: `sha2` is `0.11` in `runtime-runway` but `0.10` in `commerce-rails`.

## What this does NOT do

- It does **not** introduce a machinery-wide Cargo workspace. Projects stay
  autonomous with independent release cadence (`runtime-runway` at `3.6.0`,
  `commerce-rails` at `0.2.2`). Catalog + detector gives consistency without
  coupling release trains.
- It does **not** freeze versions forever. Bumps are welcome — they update the
  catalog first, then roll to repos, so the catalog stays the source of intent.

## How to use it

- **Starting new work?** Pick from the catalog. Need something not listed? See
  *Adding or changing an entry* below — do not just add a crate.
- **A job here already has a blessed choice?** Use it. Reaching for an
  alternative requires a logged exception.
- **Bumping a shared version?** Change it here, then align every repo's
  `[workspace.dependencies]` / `package.json` to match.

## Rust golden path

One blessed crate per concern. Versions are the major/minor line the workspaces
standardise on; features are per-repo.

| Concern | Blessed crate | Version line | Notes |
|---|---|---|---|
| Async runtime | `tokio` | `1.48` | Only async runtime. No `async-std`, `smol`. |
| HTTP client | `reqwest` | `0.13` | Construct via DI (`with_http_client`) per RP-HERMETIC-UNIT. No second client. |
| Serialization | `serde` + `serde_json` | `1` / `1` | `preserve_order` feature where map order is load-bearing. |
| Library errors | `thiserror` | `2` | Typed errors on library surfaces. |
| Binary/app errors | `anyhow` | `1` | Top-level bins only — never on a library's public API. |
| Date/time | `chrono` | `0.4` | Only time library. No `time` crate alongside it. |
| UUID | `uuid` | `1.11` | — |
| CLI parsing | `clap` | `4.5` | `derive` + `env`. |
| Tracing/logging | `tracing` + `tracing-subscriber` | `0.1` / `0.3` | No `log` + `env_logger` alongside. |
| Web framework | `axum` | `0.8` | `tower` `0.5` / `tower-http` `0.6` for middleware. |
| Config | `config` | `0.14` | — |
| Cryptographic hash | `sha2` | **RESOLVE** | Drift: `0.11` (runtime-runway) vs `0.10` (commerce-rails). Converge to one. |
| MAC | `hmac` | `0.12` | — |
| Randomness | `rand` | `0.8` | Gated in tests by RP-DETERMINISM. |
| Property testing | `proptest` | `1.5` | — |
| Temp files | `tempfile` | `3` | — |
| Telemetry (OTel) | `opentelemetry` stack | `0.24` | `opentelemetry-otlp` `0.17`, `_sdk` `0.24`, `tracing-opentelemetry` `0.25`. |
| Error reporting | `sentry` | `0.33` | — |
| gRPC | `tonic` + `prost` | `0.12` / `0.13` | — |
| Secrets in memory | `secrecy` + `zeroize` | `0.8` / `1` | — |
| JWT | `jsonwebtoken` | `10.3` | `aws_lc_rs` backend pinned explicitly (see runtime-runway note). |
| Embedded store | `redb` | `2` | — |

## TypeScript / Bun golden path

| Concern | Blessed package | Version line | Notes |
|---|---|---|---|
| Runtime + package manager | Bun | `1.3.14` | `bun.lock` is the lockfile. No npm/pnpm/yarn. |
| Schema validation | `zod` | `3.25` | Validate all runtime inputs before use. |
| LLM SDK | `@anthropic-ai/sdk` | `0.110` | — |
| Job orchestration | `@trigger.dev/sdk` | `4.5` | — |

## Approved exceptions

A catalog without an escape hatch rots. When a repo genuinely needs an
off-catalog library or a divergent version, log it here rather than diverge
silently. Keep it short; link the ledger finding.

| Repo | Library / version | Instead of | Reason | Ledger |
|---|---|---|---|---|
| _(none yet)_ | | | | |

## Adding or changing an entry

1. Open a `QF-*` finding in `QUALITY_BACKLOG.md` proposing the addition/bump/swap.
2. Update this catalog (or the exceptions table).
3. Roll repos to match and update their `deny.toml`.

## Enforcement

This doc is policy. Three layers make it observable:

1. **Catalog (this file)** — the human-readable golden path. Answers *sprawl*.
2. **`cargo-deny` `[bans]` per Rust workspace** — `multiple-versions = "deny"`
   kills within-workspace drift; a `deny = [...]` list bans off-catalog
   alternatives. `commerce-rails/deny.toml` exists; **`runtime-runway` is
   missing one** (tracked in the backlog) — that gap is the first thing to close.
3. **`RP-DEP-CATALOG` drift detector (Build-Depot)** — parses each repo's
   `[workspace.dependencies]` and `package.json` and flags (a) a library absent
   from this catalog and (b) a version diverging from the catalog line. This is
   the cross-project judgment layer, consistent with the factory doctrine — no
   repo manifest is owned centrally; drift is merely surfaced.
