---
tags: [architecture, storage, boundary, data, forward-looking]
source: mixed
status: deferred-design
epic: RFL E15 — Syncable App Data & Storage Boundaries
epic-url: https://linear.app/reflective-labs/project/e15-syncable-app-data-and-storage-boundaries-a71b7dd28bf8
captured: 2026-07-15
---
# Data & Sync Boundaries (forward-looking design)

> **Status: DEFERRED design capture — not a shipped contract.** No work is
> scheduled. This records the intended boundary story so that when a marquee or
> studio app (Quorum Sense, Atlas Integration, …) first needs **syncable data
> that persists across a session or a series of sessions** — so agents don't
> start from scratch each run — we build against a considered model instead of
> improvising per app. Tracked as Linear **E15**.

## Decision (summary)

Settled direction — **redb is engine implementation; Turso is app / distributed
data; managed cloud is the long-term authority.** Engine per tier:

- **Agent working memory** → **redb**, one file per agent (RAM for pure scratch).
  Engine/kernel-internal, **never synced**. Per-agent files sidestep redb's
  single-writer limit and give write-parallelism.
- **Individual app data** (session / multi-session; mobile · Tauri · CLI) →
  **Turso** (embedded relational, local-first push/pull). Owned by the Runtime
  Runway storage kit, injected into the app.
- **Shared converging context** → **Turso or the long-term tier** (per
  formation); sync mode still open.
- **Long-term / cross-user** → **managed cloud (Spanner / AlloyDB)**,
  server-shared consistency.

Core calls:

1. **Keep redb — do not replace it.** It is the engine primitive (typed keys,
   deterministic, crash-safe). Turso is a *separate persistence boundary*, not a
   redb replacement.
2. **Turso for local-first sync**, treated as a **rebuildable projection** so its
   0.x maturity is not a durability risk; `libsql` is the battle-tested fallback,
   and managed cloud can later swap in as a new projector without touching the
   kernel.
3. **Sync boundary = promotion boundary** — only promoted ("important enough")
   data leaves the device; promotion is a typed, governed migration, not opaque
   replication.
4. **Local-first sync ≠ multi-instance server consistency** — lifting
   `--max-instances=1` needs a shared authoritative backend (managed cloud /
   libsql-server), a different mode.
5. **Owned by Runtime Runway, injected into apps; Bedrock stays storage-free** —
   no app-local clones (Marquee Contract #1).
6. **Heavy deps stay out of this path** — Polars / Burn / SurrealDB belong in
   production Docker sidecars. SurrealDB's graph/multi-model role is **not**
   covered by Turso (relational) — confirm usage before assuming replacement.

## Why this exists

Today the fleet is storage-thin: `runway-storage` is used narrowly (Quorum uses
it for events) and the fleet is pinned to `--max-instances=1` until a persistent
store lands (Commerce Rails QF-CR-11, Runway RR D5). Bedrock itself owns **no**
storage — the `bedrock_application` facade has no persistence surface, by design.

The trigger for this epic is not "add a database." It is the moment an app's
**many in-app agents** (10 / 100 / 1000 lightweight agents) generate data that is
worth keeping across sessions, and some of it is worth **syncing** to the cloud so
a user on mobile / Tauri / CLI resumes where they left off.

## The boundary model (four tiers)

The core claim: these are **different layers of database with different owners,
lifecycles, and sync semantics** — and conflating them is the failure mode.

| Tier | Owns | Lifecycle | Engine fit | Sync |
|------|------|-----------|-----------|------|
| **Agent temporary store** | Per-agent working memory (observations, scratch, intermediate reasoning) | Ephemeral — dies with the agent unless promoted | RAM for pure scratch; **redb** file per agent (or sharded per pool) for durable candidates. By-key access; per-file topology gives write-parallelism across agents and isolation | **Never synced** |
| **Individual app data** | Session / multi-session state for one user (mobile, Tauri, CLI) — the "don't start from scratch" data | Persists across sessions; local-first | **Turso** (embedded relational, local-first push/pull sync) — single-user, single-device-first, offline-capable | Local-first push/pull of the **promoted** slice only |
| **Shared converging context** | Context shared across the convergence loops / across agents within a formation | Lives as long as the formation/loop; queryable across agents | Relational + cross-agent query (Turso, or the long-term store) | Depends — settled context may promote to long-term |
| **Long-term store** | Durable, cross-user, queryable business/knowledge records | Authoritative, survives everything | **Managed cloud** — Google Cloud Spanner / AlloyDB / similar | Managed replication (server-shared consistency, not local-first) |

## The load-bearing rules

1. **The sync boundary is the promotion boundary.** Volatile agent memory never
   leaves the device. Only data that passes an "important enough" promotion gate
   is written to a syncable/long-term tier. You sync curated, settled data — not
   churn. Cheap, and little to conflict on because promotion already resolved
   truth. Promotion is a **typed, governed migration**, not opaque DB replication.

2. **Local-first sync ≠ multi-instance server consistency.** Turso's local-first
   push/pull solves *one user across their devices / offline*. Lifting
   `--max-instances=1` (many server instances) needs a **shared authoritative**
   backend (managed cloud / libsql-server mode), a different mode. Same engine
   family, two jobs — do not let one masquerade as the other.

3. **Bedrock stays storage-free.** Storage authority lives in **Runtime Runway's
   storage kit**, not in the Bedrock facade and not as app-local clones (Marquee
   Contract rule #1: no app-local platform clones). redb, if used, is
   engine-internal only.

4. **Injected, not imported.** This capability lives in **Runtime Runway** and is
   **injected into the Marquee and Studio apps** via the host composition root
   (`{app}-server`, `runway.app.json`). App domain code (`{app}-app`,
   `{app}-domain`) never imports a storage engine directly.

## Engine notes (from evaluation)

- **redb** — embedded ordered KV, copy-on-write B+ trees, ACID, effectively
  single-writer per file, application-owned typed encoding, stable file format.
  Best for engine-internal state, caches, indexes, and per-agent working memory.
- **Turso** — Rust reimplementation of SQLite; async, MVCC concurrent writes,
  optional local-first push/pull sync. Best for user-facing app data and
  local-first sync. The new `turso` crate is 0.x — keep it as a *rebuildable
  projection* of authoritative state so its maturity is not a correctness risk;
  `libsql` remains the battle-tested fallback.
- **Managed cloud (Spanner / AlloyDB / …)** — long-term authoritative,
  server-shared consistency, external inspection, scale. If Turso is only ever a
  projection/read model, swapping in managed cloud later is a new *projector*, not
  a kernel migration.
- Heavy compute deps (Polars, Burn, SurrealDB) are **not** part of this story —
  they belong in production Docker sidecars. Note: SurrealDB's graph/multi-model
  role is **not** covered by Turso (relational); confirm what it's doing before
  assuming replacement.

## Open questions for E15 (when it activates)

- Durability split: how much agent memory is truly transient (RAM-only) vs must
  survive restart? Drives whether we have thousands of DB files or a handful.
- Topology: per-agent redb file vs sharded keyspace (file-handle / mmap / GC cost
  at 1000 agents), and a reaper for dead-agent stores.
- Where the **promotion gate** lives (agent decision vs formation governance) and
  whether promoted items are modeled as typed facts.
- Which store backs "shared converging context" — app-local Turso vs the
  long-term tier — and its sync mode.
- Concrete `runway-storage` surface: what gets injected into `{app}-server` and
  how apps declare storage needs in `runway.app.json`.

## Related

- [[App Execution Container]] — Runway/Helm/app host split (where storage injects)
- [[Commerce Rails Boundary]] — sibling authority split; QF-CR-11 persistent store
- Bedrock seam: `framework/bedrock/kb/06-consumption/marquee-app-seam.md`
- Bedrock machinery seam: `framework/bedrock/kb/06-consumption/marquee-app-machinery-seam.md`
