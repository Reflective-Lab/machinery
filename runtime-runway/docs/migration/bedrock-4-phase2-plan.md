---
title: Phase 2 — Runtime-Runway on Bedrock 4.0.0 (executed)
epic: RFL-195
date: 2026-07-10
status: executed 2026-07-10 (git-tag interim; registry flip pending RFL-194)
---

# Phase 2: Runtime-Runway Migration to Bedrock 4.0.0

**Interim strategy:** Bedrock 4.0.0 is released on GitHub
(`Reflective-Lab/bedrock-platform` tag `v4.0.1`) but not yet published to the
Shipyard was dropped (owner decision 2026-07-10); the registry endgame is a
local Kellnr service on the build server, planned for August 2026. Until
then, all Bedrock deps use
`git = "ssh://git@github.com/Reflective-Lab/bedrock-platform", tag = "v4.0.1"`.
When the local registry is live, each line flips to
`{ version = "4.0.1", registry = "..." }` — nothing else changes (registry name decided with the Kellnr setup).

The same source URL must be used by every repo in the wave (runway,
commerce-rails via transitive chain, quorum-sense): Cargo only unifies types
when deps resolve from the same source.

## What was done

### Dead dep removal (10 crates)
Deleted from `[workspace.dependencies]`: `converge-model`, `converge-pack`,
`converge-protocol`, `converge-kernel`, `converge-client`,
`converge-provider-api`, `converge-domain`, `converge-analytics`,
`converge-knowledge`, `converge-policy`. Crate-level references removed from
`crates/llm` (domain) and `crates/application` (knowledge/analytics + their
feature gates; `full` feature narrowed). Deleting `converge-client`/`protocol`
retires the tonic 0.12↔0.14 trap: none of the live 4.0 crates pull tonic, so
runway's own tonic 0.12 (app-host/llm grpc features) no longer risks a dual
copy.

### Live deps repointed (5 + 2 + 1)
- `converge-core`, `converge-provider`, `converge-experience`,
  `converge-optimization`, `converge-storage` → git+tag v4.0.1.
- `helm-module-contracts`, `helm-event-substrate` (runway-app-host,
  runway-storage): fragile `../../../../framework/bedrock` path deps → git+tag.
- **New:** `manifold-adapters` (lib name `manifold`) — env-driven chat backend
  selection (`select_chat_backend`) moved there from converge-provider at 4.0.

### Version alignment
- `sha2` 0.10 → 0.11 (matches helm-module-contracts).
- `reqwest` 0.12 → 0.13, coordinated with commerce-rails in the same wave.
  reqwest 0.13 feature-gates `form`/`query` — both added where used.
- tonic/prost stay 0.12/0.13 (runway-internal only after dead-dep removal).

### CI cleanup
- `scripts/ci/checkout-reflective-siblings.sh` deleted; all five workflows now
  use `webfactory/ssh-agent` with `secrets.SHIPYARD_SSH_KEY` (legacy name; repurposed for the deploy-key at the August registry setup) instead of
  sibling clones, so CI proves genuine git-tag resolution.
- `.cargo/config.toml` is now checked in (`net.git-fetch-with-cli = true`,
  required for the private repo). `just use-local-converge` appends a
  `[patch]` section for local framework/bedrock work;
  `just use-released-converge` restores the tracked file.

## Scout-report correction (important)

The scout verdict "Cargo.toml surgery, not a rewrite / zero source edits"
was **wrong for runway**. converge 3.4 → 4.0 carries real API drift:

- `ContextView` trait deleted → `Context` trait (pack), same shape.
- Concrete `Context` struct → `ContextState` (implements the `Context` trait).
- `Fact` → `ContextFact`, fields now private (accessors).
- `ProposedFact` struct-literal construction → typed constructor
  `ProposedFact::new(key, id: Into<ProposalId>, payload: FactPayload,
  provenance: Into<Provenance>)` + `.with_confidence(UnitInterval)`;
  confidence is `UnitInterval`, not `f64` (typed-contracts standard, RFL-129).
- `ContextKey::iter()` removed.
- `converge_storage` lost the `local` feature (core types now unconditional).
- `select_chat_backend`/`SelectedChatBackend` moved converge-provider → manifold.
- `SelectionCriteria` moved core::model_selection → converge-provider selection.

Affected production code: `crates/llm` (provider.rs, agent.rs,
execution_plan.rs), `crates/runway-accounts` (reqwest unification with
commerce-rails). Affected demo code: `crates/application` (all of it —
agents, main, ui, evals, streaming, llm_backend).

## Gates

- `cargo check --workspace --all-targets` green.
- Full test suite green (405-test baseline; substrate implementor tests
  EventLog/SyncableEventLog/LeaseStore/SessionOwnershipLayer must pass).
- commerce-rails workspace green at reqwest 0.13 with runway-storage
  transitive chain (29 tests). ✅ done 2026-07-10.

## Registry flip (follow-up: local Kellnr registry, August 2026)

1. Replace every `git = "ssh://git@github.com/Reflective-Lab/bedrock-platform", tag = "v4.0.1"`
   with `version = "4.0.1", registry = "reflective-labs"` (workspace +
   runway-app-host + runway-storage + commerce-rails transitive).
2. Add registry config + credentials to `.cargo/config.toml` and CI.
3. Keep `git-fetch-with-cli` (harmless with a git-index registry).

Deadline anchor: helms dual-home expiry **2026-08-15** (RFL-153).
