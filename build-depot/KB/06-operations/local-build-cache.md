# Local Build Cache & Disk Management

How the reflective tree's Rust build artifacts are managed on the dev workstation. Covers disk reclaim patterns and the shared compile cache (sccache) that reduces redundant compilation across the 16+ workspaces.

## Disk pressure history

- **2026-06-01 (approx)** — root volume hit 99% (178 MB free). Authorized emergency `cargo clean` on `quorum-sense`, `runtime-runway`, `atelier-showcase`. Freed ~85 GB.
- **2026-06-09** — Data volume at 83% (154 GiB free, 738 GiB used). Identified `target/` dirs as the dominant cost (~493 GiB across the top 15 workspaces).
- **2026-06-09 (later)** — second cleanup pass: 7 inactive marquee-apps + arena-tests via `cargo clean`. Freed 218 GiB. Volume now at 59% (372 GiB free). See "Cleanup pattern" below.

## sccache (shared compile cache)

Installed 2026-06-09 to reduce redundant compilation of shared platform crates (`converge-*`, `organism-*`, `axiom-*`, `helm-*`, common deps like `tokio`/`serde`/`axum`) across the 16 Rust workspaces.

**Config (in `~/.zshrc`):**

```sh
export RUSTC_WRAPPER=sccache
export SCCACHE_DIR="$HOME/.cache/sccache"
export SCCACHE_CACHE_SIZE=50G
```

**Server:** `sccache --start-server` (auto-started on first cargo build). Listens on `127.0.0.1:4226`.

**Verify it's working:**

```sh
sccache --show-stats          # compile requests, cache hit rate
sccache --show-adv-stats      # per-crate breakdown
```

The benefit compounds: each new workspace that consumes a previously-cached crate-version-with-features hits the cache instead of recompiling.

**Caveats:**

- `cargo check` produces `.rmeta` only and historically has spotty cache-hit behavior. The big wins come from `cargo build`, `cargo test`, and Cloud Build runs.
- Each workspace still keeps its own `target/` — sccache caches at the rustc invocation level, not the workspace artifact level. You still need to clean inactive workspaces periodically.
- `[patch.crates-io]` blocks pointing at different local paths invalidate the cache for that crate. Working in one marquee-app vs another with different patches counts as different inputs.

## Cleanup pattern

When disk pressure hits ~80%, this is the safe first sweep — apps not under active iteration:

```sh
for app in atlas-integration scout-sourcing tally-escrow plumb-execution fathom-narrative vouch-lending; do
  cargo clean --manifest-path "/Users/kpernyer/dev/reflective/marquee-apps/$app/Cargo.toml"
done
cargo clean --manifest-path /Users/kpernyer/dev/reflective/arena-tests/Cargo.toml
```

Typical reclaim: **200-250 GiB**.

**Apps to keep clean-aware:**

| App | Typical `target/` | Notes |
|---|---|---|
| `bedrock-platform/helms` | 134 GB | Largest single offender. Clean ONLY when not about to build any platform-dependent marquee-app — it has hot dependencies for everything. |
| `marquee-apps/quorum-sense` | ~36 GB | Active (Plan 5/5a). Keep. |
| `marquee-apps/atlas-integration` | ~55 GB | Step 2a / Atlas cross-app spike pending — clean only if cold. |
| `marquee-apps/scout-sourcing` | ~43 GB | Safe to clean — not currently active. |
| `marquee-apps/tally-escrow` | ~36 GB | Safe to clean. |
| `marquee-apps/plumb-execution` | ~31 GB | Safe to clean. |
| `marquee-apps/fathom-narrative` | ~16 GB | Safe to clean. |
| `marquee-apps/vouch-lending` | ~13 GB | Safe to clean. |
| `studio-apps/folio-editor` | ~24 GB | Newspaper deploy work — check before cleaning. |
| `studio-apps/wolfgang-chat/backend` | ~16 GB | Active per CLAUDE.md goals (Stage 1 billing/QA). Keep. |
| `arena-tests` | ~24 GB | Safe to clean — regenerated on next test run. |

## Open follow-ups

- **Pin a sccache size budget for CI / Cloud Build.** sccache supports a `--dist` mode that could share the cache across machines, including the Cloud Build runners we use for wolfgang + (soon) quorum-sense. Not configured yet.
- **The 134 GB `bedrock-platform/helms/target` problem.** Periodic cleanup is the only current answer; sccache reduces but doesn't eliminate it because incremental linking + debug symbols dominate the size.
- **Disable incremental compilation in `release-fast` profile.** Halves target/ size in some workspaces at the cost of slower iterative rebuilds. Worth measuring on a high-pressure day.

## Related

- Plan 5 (Cloud Run sandbox): `marquee-apps/quorum-sense/docs/superpowers/plans/2026-06-08-plan-5-cloud-run-sandbox.md` — references the wolfgang shared base image + Artifact Registry; consider whether Cloud Build should also wrap with sccache `--dist`.
- Wolfgang deploy docs: `studio-apps/wolfgang-chat/deploy/` — already uses a `kenneth-backend-base` shared image as a caching layer for cargo deps in the Cloud Build pipeline.
