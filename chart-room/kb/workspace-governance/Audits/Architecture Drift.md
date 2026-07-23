---
tags: [audit]
source: llm
---
# Architecture & Systems Drift Tracker

## Post-retirement update (2026-06-07)

The 2026-06-02 retirement of the deployed `converge-runtime` service changes how prior DRIFT flags should be read:

- Any project listed as depending on the `converge-runtime` crate (wolfgang at line 41, epic-brand at line 44 in the 2026-04-13 baseline) is now depending on a **compatibility-only shell**, not a canonical runtime. The drift is still real (the crate is no longer the right dependency), but the remediation has shifted: instead of "migrate to public converge crates," the remediation is "migrate the runtime-shaped functionality to [[../../04-architecture/runtime-runway/Architecture - Overview|runtime-runway]] (`runway-auth`, `runway-middleware`, `runway-storage`, `runway-telemetry`) or to [[../../04-architecture/commerce-rails/Architecture - Overview|commerce-rails]] / [[../../04-architecture/bedrock-platform/Architecture - Helms|Helms]] / the app host as appropriate."

- The next baseline audit should re-classify `converge-runtime` usage from "DRIFT (uses internal crates)" to "DRIFT (uses retired compat shell)" and resolve via the [[../../04-architecture/decisions/2026-06-02-converge-runtime-retirement|2026-06-02 retirement ADR]]'s migration table.

See also the canonical [[../../04-architecture/current-system-map|boundary registry]].

## What We Check
- Rust edition/version alignment
- No unsafe code
- JS/TS stack (Bun, SvelteKit, Tauri)
- Converge dependency alignment (public crates only)
- Convention adherence (CLAUDE.md, checkpoint skills, kb structure)
- Layering (organism doesn't own axioms, saas-killer goes through organism)

## Audit History

### 2026-04-13 — Full Baseline

#### Rust Edition & Version

| Project | edition | rust-version | unsafe | Status |
|---------|---------|-------------|--------|--------|
| converge | 2024 | 1.94.0 | 2 files (FFI only) | Pass |
| organism | 2024 | 1.90 | Clean (false positive) | Pass |
| saas-killer | 2024 | 1.94.0 | Clean | Pass |
| hackathon | 2024 | 1.94 | Clean | Pass |
| epic-brand | 2024 | 1.94 | Clean | Pass |

**Spec updated:** ~/dev/CLAUDE.md now says rust-version 1.94 (aligned to converge).

**Unsafe code (justified FFI):**
- converge: ortools-sys/lib.rs (17 blocks, C FFI to OR-Tools), tool/codegen.rs (2 blocks, WASM boundary template)
- Original audit over-counted: 5 of 7 "hits" were string literals containing the word "unsafe"
- organism virtual_teams.rs: false positive — `"agents_unsafe_by_default"` is an invariant name

#### Converge Dependency Alignment

| Project | Method | Internal crates used | Status |
|---------|--------|---------------------|--------|
| organism | path | Clean (active code uses converge-kernel, converge-pack only) | Pass |
| wolfgang | path | **converge-core, converge-runtime, converge-provider** | DRIFT |
| saas-killer | path | **converge-core, converge-experience, converge-analytics, converge-domain, converge-knowledge, converge-optimization** | DRIFT |
| hackathon | crates.io + path override | **converge-core, converge-domain, converge-policy** | DRIFT |
| epic-brand | git rev + path | **converge-runtime, converge-core, converge-provider, converge-llm** | DRIFT |

#### Checkpoint Skills
All 8 projects: **PASS** — consistent, referencing MILESTONES.md + CHANGELOG.md + EPIC.md.

#### Layering
- organism: Pass (active code clean, legacy partitioned)
- saas-killer: **DRIFT** — imports converge crates directly, should go through organism

**Action items:**
- Update ~/dev/CLAUDE.md rust-version to 1.94.0 (align to converge)
- epic-brand: bump to edition 2024, add rust-version
- hackathon: bump rust-version to 1.94.0
- Audit and remove unsafe code from converge (7 files) and organism (1 file)
- Long-term: migrate wolfgang, saas-killer, hackathon, epic-brand from internal to public converge crates
