---
type: architecture-module
source-path: bedrock-platform/helms/
last-scanned: 2026-06-07
scanned-version: 0.2.1
tags: [architecture, bedrock-platform, helms]
---

# Helms — Module Architecture

<!-- @generated:start -->

Per its own README:

> *"Helm is the operator-facing application layer and desktop workbench built on top of `../converge` and `../organism`."*
> — `bedrock-platform/helms/README.md:3-5`

Part of [[Architecture - Overview|bedrock-platform]]. Workspace version `0.2.1` in Cargo.toml (README header still reads `0.2.0`). Largest sub-project in bedrock by crate count: 35 Rust crates plus Tauri desktop app, Svelte web, and proto wire definitions.

## Workspace crates (35)

Grouped by role (group labels are inferred from crate names — `confidence: stated for crate names, speculation for grouping`):

**Application layer (7):**
- `apps/desktop/src-tauri` — Tauri desktop shell
- `crates/crm-contracts` — CRM-shaped contract types
- `crates/workbench-backend` — workbench HTTP backend
- `crates/application-kernel` — app kernel
- `crates/application-storage` — app-specific storage
- `crates/helm-truth-execution` — truth execution surface
- `crates/plugin-runtime` — plugin runtime

**Helm extraction (2):**
- `crates/helm-operator-control` — operator control plane
- `crates/helm-governed-jobs` — governed jobs surface

**Capability & truth (28):** `crates/capability-core`, `crates/capability-registry`, `crates/truth-catalog`, and 25 `crates/prio-*` crates covering catalog, identity, parties, conversations, opportunities, tasks, documents, expenses, apple-notes-cli, subscriptions, metering, ledger, entitlements, payments, workflow, approvals, policies, facts, audit, intents, memory, agent-ops.

**Primitives (1):** `crates/notes`.

**Utility (1):** `crates/seed-gen`.

## Top-level non-Rust assets

- `apps/` — desktop Tauri shell + web app
- `packages/` — shared operator console layer (`helm-console`)
- `proto/` — proto wire definitions, namespaced under `proto/prio/*/v1`
- `contracts/` — contract definitions
- `truths/` — truth specs + `.feature` files
- `data/`, `examples/`, `experiments/`, `kb/` (Obsidian knowledgebase)

## Truth state (from current-system-map)

The code-backed catalog in `crates/truth-catalog` registers **23 definitions** (18 cataloged job truths, 3 policy truths, 2 module-local invariants). Of 24 `.feature` files under `truths/`, all but `generate_data_transformer` are registered in `TRUTHS`.

The `workbench-backend` executable surface currently supports **4 truth keys**:

- `qualify-inbound-lead`
- `submit-expense-report`
- `activate-subscription`
- `refill-prepaid-ai-credits`

Other catalog entries may expose metadata, Converge bindings, or evaluators but are not executable unless `is_truth_supported` includes them or a registered `TruthBody` mounts them through `helm-truth-execution`.

## Entry points

- `outcome-workbench-desktop` binary at `apps/desktop/src-tauri/src/main.rs` — Tauri desktop app
- `seed-gen` binary at `crates/seed-gen/src/main.rs` — seed-generation utility

## Boundary

From [[../current-system-map|current-system-map]] §Boundaries:

- Owns: trust-transfer surfaces, app-specific business state, workbench views, user-facing consequence, manifest intake, operator review, truth-catalog binding, sandbox lifecycle, approval points, audit visibility.
- Does NOT own: applet authority/schema (→ [[Architecture - Axiom|Axiom]]), domain mutation in product apps (→ marquee/studio app repos), commercial state (→ [[../commerce-rails/Architecture - Overview|commerce-rails]]).

## Historical naming note

The `prio-*` crate prefix and the `crm-contracts` name predate the current `application-*` / `workbench-backend` / `capability-registry` / `truth-catalog` naming. Per [[../../LOG|KB/LOG.md]] 2026-05-31: legacy handoff notes under `outcome-workbench/` may still reference `crm-*` and `prio-truths`. Prefer the current crate layout when starting new work. Proto packages remain under `proto/prio/*/v1`; namespace migration is planned but incomplete.

## Cross-references

- [[../current-system-map|Current System Map]]
- [[../applet-runtime-boundaries|Applet Runtime Boundaries]]
- [[Architecture - Overview|bedrock-platform overview]]
- [[../../outcome-workbench/kb/Home|Helm workspace docs (legacy KB path)]]

<!-- @generated:end -->
