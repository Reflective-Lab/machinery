---
title: Phase 2 Plan — Runtime-Runway to Bedrock 4.0.0
epic: RFL-195
date: 2026-07-10
---

# Phase 2: Runtime-Runway Migration to Bedrock 4.0.0

**Gate:** RFL-194 Phase 1 complete (48 crates published to reflective-labs registry v4.0.0)

**Duration:** ~2 days post-gate

## Overview

Runtime-runway has 10 dead converge deps and 2 helm contracts on path deps. Phase 2 is Cargo.toml surgery + CI rewiring. Zero Rust source edits.

## Step 1: Delete Dead Converge Declarations

### Workspace-level (delete from `runtime-runway/Cargo.toml` `[workspace.dependencies]`)

```toml
# REMOVE these 5:
converge-model = { version = "4.0.0", registry = "reflective-labs" }
converge-pack = { version = "4.0.0", registry = "reflective-labs" }
converge-protocol = { version = "4.0.0", registry = "reflective-labs" }  # ⚠️ tonic trap
converge-kernel = { version = "4.0.0", registry = "reflective-labs" }
converge-client = { version = "4.0.0", registry = "reflective-labs" }   # ⚠️ tonic trap
```

**Why:** These are never used in runway source. `converge-client` and `converge-protocol` present the tonic 0.12 → 0.14 trap (see migration guide).

### Crate-level (delete from `runtime-runway/crates/*/Cargo.toml`)

Search for and remove:
- `converge-provider-api` (workspace decl only; see guide)
- `converge-domain` (in crates/llm only; never imported)
- `converge-analytics` (in crates/application, optional feature, never used)
- `converge-knowledge` (in crates/application, optional feature, never used)
- `converge-policy` (workspace decl only)

**Gate:** `cargo check --workspace` must pass after removal.

## Step 2: Switch Helm Contracts to Registry

### `runtime-runway/Cargo.toml`

Identify all helm deps (currently path deps):

```toml
# BEFORE (path deps)
helm-module-contracts = { version = "0.3.0", path = "../../../bedrock-platform/helms/contracts/crates/helm-module-contracts" }
helm-event-substrate  = { version = "0.1.0", path = "../../../bedrock-platform/helms/contracts/crates/helm-event-substrate", features = ["sse"] }

# AFTER (registry deps)
helm-module-contracts = { version = "4.0.0", registry = "reflective-labs" }
helm-event-substrate  = { version = "4.0.0", registry = "reflective-labs", features = ["sse"] }
```

**Gate:** Implementations (RedbEventLog, FirestoreEventLog, RedbLeaseStore, FirestoreLeaseStore, SessionOwnershipLayer) compile unchanged.

## Step 3: Version Bumps

### Tonic + Prost (coordinated with runway workspace cleanup)

After deleting dead converge declarations, bump runway's direct tonic/prost:

```toml
# runtime-runway/Cargo.toml [workspace]
tonic = { version = "0.14" }
prost = { version = "0.14" }
```

**Why:** Bedrock 4.0.0 converge crates use 0.14; keeping 0.12 locks out their consumers.

**Gate:** `cargo tree` shows single tonic/prost copy.

### SHA2

```toml
# runtime-runway/Cargo.toml [workspace]
sha2 = { version = "0.11" }  # was 0.10
```

**Why:** helm-module-contracts declares 0.11; alignment avoids dual build.

### Reqwest (COORDINATED WAVE)

Bump to 0.13 in the SAME wave as quorum-server and commerce-rails:

```toml
# runtime-runway/Cargo.toml [workspace]
reqwest = { version = "0.13" }  # was 0.12
```

**Gate:** Coordinate with commerce-rails phase (verify commerce-rails-* clients still resolve).

## Step 4: CI Cleanup

### Remove helms sibling checkout

Delete from `.github/workflows/ci.yml` (or equivalent):

```yaml
# REMOVE this step:
- name: Checkout reflective-siblings
  run: checkout-reflective-siblings.sh
```

Delete the script if it exists in runway.

### Add Shipyard registry credential

Ensure `.cargo/config.toml` exists in runway root with:

```toml
[registries.reflective-labs]
protocol = "git"
```

Auth handled by CI secrets (SHIPYARD_SSH_KEY or token method).

**Gate:** CI proves registry resolution with NO sibling clone fallback.

## Step 5: Substance Tests

All 405 runway tests must pass:

```bash
cd runtime-runway
cargo test --workspace --all-targets
```

**Acceptance:** 
- ✓ No runtime-runway test changes needed
- ✓ Substrate implementor tests green (EventLog, LeaseStore, SessionOwnership)
- ✓ No WASM or FFI breakage

## Files to Change

```
runtime-runway/Cargo.toml              # dead deps removal, registry switch
runtime-runway/crates/*/Cargo.toml     # crate-level dead deps
runtime-runway/.cargo/config.toml      # registry config (create if missing)
runtime-runway/.github/workflows/*.yml # remove sibling checkout
```

## Rollback Trigger

If any test fails:
1. Revert Cargo.toml changes
2. Keep CI cleanup (that's safe)
3. Investigate source incompatibility (should not occur per scout report)

## Next Phase

Phase 3: commerce-rails reqwest 0.13 bump (coordinated)
Phase 4: quorum-sense registry migration (package aliases)
Phase 5: cutover walk with RFL-153 gap table

---

**Dependency:** RFL-194 Phase 1 complete (publish dry-run green)
**Blocking:** Phase 3 (commerce-rails) + Phase 4 (quorum-sense)
**Deadline:** 2026-08-15 (helms dual-home expiry)
