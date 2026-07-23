# Retirement ADR: `converge-runtime` deployed service

- Date: 2026-06-02
- ADR written: 2026-06-07 (retroactive — the decision landed 2026-06-02, this ADR codifies the sweep that was completed retroactively on the same day)
- Status: **swept** (2026-06-07; see Sweep evidence section)
- Decision type: retirement / replacement
- Related:
  - [[../../LOG|KB/LOG.md]] 2026-06-02 entry "Converge runtime retirement" — the underlying change
  - [[templates/retirement-adr|Retirement ADR Template]] — codifies the sweep protocol this ADR is the first instance of

## What's being retired

The deployed standalone `converge-runtime` Cloud Run service has been retired as the canonical deployed runtime for the Converge stack. The `converge-runtime` Rust crate binary at `bedrock-platform/converge/crates/runtime/src/main.rs` still exists but is now a **compatibility-only internal shell** — it is no longer the production runtime path.

## Why

Runtime responsibilities were split across better-fitting homes as the stack matured:

- App hosting, auth, storage, secrets, telemetry → [[../runtime-runway/Architecture - Overview|runtime-runway]]
- In-process distributed consensus (Organism HITL quorum), orchestration, persistence services → [[../lattice-mesh/Architecture - Overview|lattice-mesh]] (declared boundary, no source yet)
- Commercial state, billing, entitlement → [[../commerce-rails/Architecture - Overview|commerce-rails]]
- Operator surfaces and governed-jobs runtime → [[../bedrock-platform/Architecture - Helms|Helms]]
- App-specific runtime composition → individual app repos (marquee/studio/mobile)

Continuing to deploy `converge-runtime` would have meant: (a) duplicating runtime work that already exists in `runtime-runway`, (b) inverting the dependency direction (a "runtime" inside a "platform-as-library" project), and (c) preaching a deployment shape we no longer use.

Forcing function: the 2026-06-02 commit recorded in `KB/LOG.md` removed the active `converge.zone` Firebase Hosting `/api/**` rewrite to the Cloud Run service.

## Old → New (the migration table)

| What it was | Where it lived | What it is now | Where it lives now |
|---|---|---|---|
| `converge-runtime` Cloud Run service | `converge.zone` Firebase Hosting `/api/**` rewrite → Cloud Run | **removed** | n/a — no replacement at this URL |
| `converge-runtime` deployed runtime | Cloud Run, as the canonical app/agent host | split across runtime-runway, lattice-mesh, commerce-rails, helms, and app hosts | see anchors in [[../current-system-map\|current-system-map]] |
| `converge-runtime` Rust crate binary | `bedrock-platform/converge/crates/runtime/src/main.rs` | **compat-only shell** (binary still builds; not the production path) | same path; mark with deprecation comment |
| Documentation framing "Converge as a deployed runtime" | many KB pages + READMEs | "Converge is a pure library; deployment lives in sibling repos" | per-anchor description in [[../current-system-map\|current-system-map]] |

## Claim sweep checklist

Original grep at 2026-06-07 returned **33 files** mentioning `converge-runtime` or `converge_runtime` in `KB/`. The sweep below is **semantic**, not deletion-based — the grep count stays the same after the sweep because: (a) this ADR, the registry, the LOG entry, and the architecture notes are the new sources of authoritative claim and they correctly name the retired thing; (b) bulk-archive markers preserve legacy folders' contents under an `_ARCHIVED.md` flag rather than rewriting 21 files; (c) 7 already-correct files were swept by Karl pre-2026-06-07 and explicitly say "retired" / "compatibility-only" / "was removed". What matters is whether any file still **preaches the wrong reality** — that count is now zero in the active KB.

### Registry

- [x] `KB/04-architecture/current-system-map.md` — Converge anchor row updated 2026-06-07 to note `converge-runtime` is compat-only; full Project Boundary Anchors section added.
- [x] `KB/04-architecture/bedrock-platform/Architecture - Converge.md` — generated note marks it as compatibility shell.
- [x] `KB/LOG.md` — retirement landed under 2026-06-02 entry; retroactive ADR landed under 2026-06-07.

### Repo-level READMEs (already swept by Karl pre-2026-06-07; verified clean)

- [x] `bedrock-platform/converge/README.md:33` — already states *"The standalone `converge-runtime` crate is retired as the canonical deployed service. It remains in the workspace as an internal compatibility shell while historical scripts and downstream references drain."*
- [x] `runtime-runway/README.md` — three correct mentions: line 189–190 declares retirement; line 227 marks the Cloud Run target "Retired compatibility only" gated by `ALLOW_LEGACY_CONVERGE_RUNTIME_DEPLOY=true`; line 249 labels `just dev-up` as "legacy local converge-runtime compatibility shell."

### KB — top-level cross-cutting (already swept; verified clean)

- [x] `KB/deployment-and-infrastructure.md:20` — already states *"former converge.zone `/api/**` -> `converge-runtime` route was retired on 2026-06-02"*.
- [x] `KB/tech-stack.md:38` — already states *"converge.zone no longer routes `/api/**` to standalone `converge-runtime`"*.
- [x] `KB/06-operations/README.md:20` — already states *"The standalone `converge-runtime` Cloud Run route is retired; current app runtime ownership belongs to Runtime Runway and app hosts."*

### KB — workspace-governance

- [x] `KB/workspace-governance/INDEX.md:38` — already states *"Compatibility-only HTTP/gRPC service shell; retired as canonical stack runtime"*.
- [x] `KB/workspace-governance/Audits/Architecture Drift.md` — added a "Post-retirement update (2026-06-07)" section explaining how the 2026-04-13 baseline DRIFT flags for wolfgang (line 41) and epic-brand (line 44) should be re-read now that `converge-runtime` is a compat shell. Remediation path redirected to runtime-runway / commerce-rails / Helms / app host as appropriate.

### KB — converge-business (bulk-archived)

- [x] `KB/converge-business/_ARCHIVED.md` written 2026-06-07. Folder marker explicitly flags the 19 files under this domain (specs, packs, flows, the-guarantee/architecture, the-business/competitive-positioning, go/gtm-archive/mvp-plan) as predating the 2026-06-02 retirement. Rule: do not cite this folder as current architecture; copy out and adapt if anything timeless needs promoting back.

### KB — outcome-workbench (bulk-archived)

- [x] `KB/outcome-workbench/_ARCHIVED.md` written 2026-06-07. Folder marker flags both the pre-rename naming (`crm-*` / `prio-truths`) issue and the converge-runtime issue (specifically `Operations/Coordinator Handoff.md:40` and `Integrations/Integration Plan.md:96`). Points at current Helm docs in `bedrock-platform/helms/kb/`.

### KB — knowledge / wolfgang

- [x] `KB/07-knowledge/stack-reconciliation-2026-05-31.md` — already correct (has a "2026-06-02 Runtime Retirement Update" section at line 109 that explicitly handles the change).
- [x] `KB/wolfgang-business/02-architecture/system-overview.md:45` — edited 2026-06-07: the stale claim `auth boundary through converge_runtime::http_auth` now carries a `**stale as of 2026-06-02**` flag, points Wolfgang's backend at [[../runtime-runway/Architecture - Crates|runtime-runway/runway-auth]], and links this ADR.

### Deployment / infra

- [x] `converge.zone` Firebase Hosting `/api/**` rewrite to Cloud Run service `converge-runtime` — **removed** per LOG.md 2026-06-02.
- [x] `runtime-runway/ops/scripts/deploy-cloud-run.sh` and `runtime-runway/ops/scripts/dev-up.sh` still reference `converge-runtime` — intentionally retained as compat-only paths (gated behind `ALLOW_LEGACY_CONVERGE_RUNTIME_DEPLOY=true` per the README). Do not delete until the last legacy consumer migrates.
- [x] `forge-templates/converge-engagement/` floor table — verified does not list `converge-runtime`. Note: floor versions there are 3+ behind current platform; separate concern, flagged in [[../forge-templates/Architecture - Overview|forge-templates overview]].

### Code-side markers

- [x] Top-of-file doc comment on `bedrock-platform/converge/crates/runtime/src/main.rs` rewritten 2026-06-07 to declare "COMPATIBILITY SHELL — RETIRED 2026-06-02," list where production runtime responsibilities moved, and link this ADR. `#[deprecated]` attribute deferred — the doc comment is more discoverable for the actual readers (humans + Claude reading the source) and a `#[deprecated]` on `main` doesn't fire on binary callers anyway.

## Consequences

- **Callers:** no remote `/api/**` endpoint at `converge.zone` for the old Cloud Run service. Anything pointing there will get 404. Verify mobile-apps / studio-apps / marquee-apps don't ship code that calls that URL.
- **Deployers:** no Cloud Run service to deploy under that name. Existing deploy scripts referencing it should be removed or repointed.
- **Library users:** the Converge Rust workspace is unaffected — `converge-{model,pack,protocol,kernel,client,core,provider,experience,optimization,storage}` all keep working as embedded libraries.
- **Compat shell sunset:** the `converge-runtime` binary can be deleted from `bedrock-platform/converge/crates/runtime/` once a sweep confirms no caller depends on it. Today: keep it; tag this ADR `archived` only when the crate is also deleted.
- **Arena-tests:** any test asserting `converge-runtime` is reachable on a port should be removed. Tests asserting in-process runtime behaviour move to runtime-runway or stay as library tests on Converge.

## Follow-Ups

- A drift-check script (Move 2 from the boundary-coherence plan) should learn this pair: `converge-runtime (Cloud Run service)` → retired; `converge-runtime` crate → compat shell. Any future README claim that names `converge-runtime` as a live runtime should be flagged.
- If the `KB/converge-business/specs/` files are kept as historical records, add a folder-level `_ARCHIVED.md` rather than sweeping each spec. That's faster, honest, and respects the user's pattern of preserving history.

## Sweep evidence

- Sweep landed: 2026-06-07
- Commits:
  - `reflective@b63834a` — *"kb: architecture coverage + boundary registry + converge-runtime sweep"* (28 files; 1702 insertions, 15 deletions)
  - `converge@577857f` — *"docs(runtime): mark crate as compatibility shell post-2026-06-02 retirement"* (rebased on top of Dependabot `c6e329a` polars bump + `dfebe75` async-nats bump; pre-commit hook ran cargo fmt + clippy + test coverage gate; all passed)
- Files written or edited as part of the sweep:
  - **Created:** `KB/converge-business/_ARCHIVED.md` (folder marker; 19 files covered).
  - **Created:** `KB/outcome-workbench/_ARCHIVED.md` (folder marker; 2 files covered).
  - **Edited:** `KB/wolfgang-business/02-architecture/system-overview.md` — line 45 stale-flagged + redirected.
  - **Edited:** `KB/workspace-governance/Audits/Architecture Drift.md` — added 2026-06-07 post-retirement update section.
  - **Edited:** `bedrock-platform/converge/crates/runtime/src/main.rs` — top-of-file doc comment now declares the compat-shell status.
  - **Cross-referenced:** the 7 KB pages already correctly framed (deployment-and-infrastructure, tech-stack, 06-operations/README, workspace-governance/INDEX, 07-knowledge/stack-reconciliation, plus the two repo READMEs) were verified and ticked, not re-edited.

### Why the grep count didn't drop

The sweep is semantic. The original 33-file grep over `KB/` for `converge-runtime` still returns 33 files because:

- The new authoritative documents (this ADR, the registry's Converge anchor, the LOG entry, the architecture notes) **must** name the retired thing — that's their job.
- Bulk-archive markers preserve legacy folders intact; the `_ARCHIVED.md` adds context without rewriting 21 files individually.
- 7 already-correct files describe `converge-runtime` accurately as retired / compatibility-only.
- The 2 files I edited still mention `converge-runtime` — but now with explicit stale-flags pointing at the correct owner.

What did drop to **zero**: the count of files in the active KB that still preach `converge-runtime` as a live deployed runtime without context. That was the actual goal.
