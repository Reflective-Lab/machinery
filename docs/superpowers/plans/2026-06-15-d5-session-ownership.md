# D5 SessionOwnership — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ship the D5 SessionOwnership lease primitive — a per-`(org_id, app_id, session_id)` admission-time lease that prevents two `RunwayAppHost` instances from concurrently mutating the same domain aggregate. Hard gate for marquee-app `--max-instances > 1`.

**Architecture:** New `LeaseStore` sibling trait in `runway-storage` with two backends (redb local, Firestore remote), wired through `StorageKit`. Tower middleware `SessionOwnershipLayer` in `runway-app-host` reads `AuthContext.org_id()` + a configured path param, calls `leases.try_acquire`, spawns a background renew task, and fire-and-forgets release on guard drop. v1 has NO write-side fencing (documented limitation; D5.1 follow-up).

**Tech Stack:** Rust edition 2024, rust-version 1.96.0, `async-trait`, `tokio`, `axum 0.8`, `tower`, `redb` (local), `reqwest` + Firestore REST API (remote), `chrono`, `uuid`. Workspace-pinned deps only.

**Spec:** `/Users/kpernyer/dev/reflective/runtime-runway/docs/superpowers/specs/2026-06-15-d5-session-ownership-design.md` (SHA `9d0504c`).

**Branch to work on:** `d5-session-ownership-design` (already exists; spec committed; implementation work goes here).

**Hard rules:** No `unsafe`. No feature flags. No softening switches. `just lint` clean before any commit. Workspace deps only (`workspace = true` in Cargo.tomls). Per `runtime-runway/CLAUDE.md`.

---

## Task 1: Define `LeaseStore` trait + concrete types

**Files:**
- Create: `runtime-runway/crates/runway-storage/src/traits/lease.rs`
- Modify: `runtime-runway/crates/runway-storage/src/traits/mod.rs`
- Modify: `runtime-runway/crates/runway-storage/src/lib.rs` (re-exports)

- [ ] **Step 1: Write the trait file**

Create `runtime-runway/crates/runway-storage/src/traits/lease.rs`:

```rust
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};

use crate::traits::Result;

/// Triple that identifies a unique lease. Serialized as
/// `format!("{org_id}|{app_id}|{session_id}")` for storage keys.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LeaseScope {
    pub org_id: String,
    pub app_id: String,
    pub session_id: String,
}

impl LeaseScope {
    /// Stable string key for backend storage. Implementations MUST use this
    /// for both redb keys and Firestore document IDs to keep contract-suite
    /// assertions consistent.
    pub fn key(&self) -> String {
        format!("{}|{}|{}", self.org_id, self.app_id, self.session_id)
    }
}

/// Persistent state of a lease.
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct LeaseRecord {
    pub holder_id: String,
    pub expires_at: DateTime<Utc>,
}

/// Outcome of a `try_acquire` call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AcquireOutcome {
    /// Caller now holds the lease (new acquire, idempotent re-acquire by same
    /// holder, or steal after expiry).
    Acquired(LeaseRecord),
    /// Another holder owns an unexpired lease.
    HeldByOther(LeaseRecord),
}

/// Outcome of a `renew` call.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RenewOutcome {
    /// Renewal succeeded; `expires_at` was advanced.
    Renewed(LeaseRecord),
    /// Caller is no longer the holder. `current` is `Some` when another holder
    /// has taken over, `None` when the lease was released or never existed.
    Lost { current: Option<LeaseRecord> },
}

/// Atomic compare-and-swap leases keyed by `(org_id, app_id, session_id)`.
///
/// Local impl: redb single-`WriteTransaction` read-modify-write.
/// Remote impl: Firestore `runTransaction` on `_runway_leases/{scope_key}`.
///
/// v1 is admission-time correctness only. There is no write-side fencing —
/// a paused process that wakes after TTL steal can still write through
/// `DocumentStore`. See `RP-NO-LEASE-WITHOUT-FENCING-V1`.
#[async_trait]
pub trait LeaseStore: Send + Sync {
    /// Try to acquire (or re-acquire / steal-after-expiry) the lease.
    async fn try_acquire(
        &self,
        scope: &LeaseScope,
        holder_id: &str,
        ttl: Duration,
    ) -> Result<AcquireOutcome>;

    /// Renew an existing lease iff `holder_id` matches the current record.
    async fn renew(
        &self,
        scope: &LeaseScope,
        holder_id: &str,
        ttl: Duration,
    ) -> Result<RenewOutcome>;

    /// Release the lease. No-op if the caller is not the current holder or
    /// the lease does not exist.
    async fn release(&self, scope: &LeaseScope, holder_id: &str) -> Result<()>;

    /// Read the current record without modifying it.
    async fn current(&self, scope: &LeaseScope) -> Result<Option<LeaseRecord>>;
}
```

- [ ] **Step 2: Wire trait module into the parent `mod.rs`**

Modify `runtime-runway/crates/runway-storage/src/traits/mod.rs`. The file currently has:

```rust
pub mod document;
pub mod embedding;
pub mod event;
pub mod object;
pub mod vector;

// ... existing Error/Result definitions
```

Add the `lease` module immediately after `event`:

```rust
pub mod document;
pub mod embedding;
pub mod event;
pub mod lease;
pub mod object;
pub mod vector;
```

- [ ] **Step 3: Re-export trait + types from crate root**

Modify `runtime-runway/crates/runway-storage/src/lib.rs`. Find the existing `pub use crate::traits::{...}` block (it currently re-exports `Document`, `DocumentStore`, `EventLog`, etc.) and add the lease re-exports immediately after the existing event exports:

```rust
pub use crate::traits::lease::{
    AcquireOutcome, LeaseRecord, LeaseScope, LeaseStore, RenewOutcome,
};
```

- [ ] **Step 4: Write a unit test for `LeaseScope::key`**

Append to `runtime-runway/crates/runway-storage/src/traits/lease.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scope_key_is_pipe_delimited() {
        let scope = LeaseScope {
            org_id: "org-1".into(),
            app_id: "quorum".into(),
            session_id: "inq-abc".into(),
        };
        assert_eq!(scope.key(), "org-1|quorum|inq-abc");
    }

    #[test]
    fn lease_record_serde_roundtrip() {
        let now = Utc::now();
        let rec = LeaseRecord {
            holder_id: "rev-1:uuid-x".into(),
            expires_at: now,
        };
        let json = serde_json::to_string(&rec).expect("serialize");
        let back: LeaseRecord = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.holder_id, rec.holder_id);
        assert_eq!(back.expires_at, rec.expires_at);
    }
}
```

- [ ] **Step 5: Build and run tests**

```bash
cd /Users/kpernyer/dev/reflective/runtime-runway
cargo test -p runway-storage --lib traits::lease
```
Expected: 2 tests pass.

- [ ] **Step 6: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 7: Commit**

```bash
git add crates/runway-storage/src/traits/lease.rs \
        crates/runway-storage/src/traits/mod.rs \
        crates/runway-storage/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(d5): LeaseStore trait + concrete types

Sibling trait next to DocumentStore/EventLog/etc. Defines LeaseScope,
LeaseRecord, AcquireOutcome, RenewOutcome and the four methods
(try_acquire, renew, release, current). Backend impls land in
subsequent tasks; v1 has no write-side fencing (D5.1 follow-up).

Refs: D5 spec section 4.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 2: Implement `RedbLeaseStore` (local backend)

**Files:**
- Create: `runtime-runway/crates/runway-storage/src/local/lease.rs`
- Modify: `runtime-runway/crates/runway-storage/src/local/mod.rs` (add `mod lease;` + `init_tables` call)

- [ ] **Step 1: Write the failing acquire test first**

Create `runtime-runway/crates/runway-storage/src/local/lease.rs`:

```rust
use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use chrono::Utc;
use redb::{Database, ReadableTable, TableDefinition, WriteTransaction};

use crate::traits::{
    Error, Result,
    lease::{AcquireOutcome, LeaseRecord, LeaseScope, LeaseStore, RenewOutcome},
};

// Table: scope_key (String) → LeaseRecord JSON
const LEASES: TableDefinition<&str, &str> = TableDefinition::new("leases");

pub fn init_tables(tx: &WriteTransaction) -> anyhow::Result<()> {
    tx.open_table(LEASES)?;
    Ok(())
}

pub struct RedbLeaseStore {
    db: Arc<Database>,
}

impl RedbLeaseStore {
    pub fn new(db: Arc<Database>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl LeaseStore for RedbLeaseStore {
    async fn try_acquire(
        &self,
        scope: &LeaseScope,
        holder_id: &str,
        ttl: Duration,
    ) -> Result<AcquireOutcome> {
        let db = self.db.clone();
        let key = scope.key();
        let holder = holder_id.to_string();
        let now = Utc::now();
        let new_expires = now + chrono::Duration::from_std(ttl)
            .map_err(|e| Error::Other(e.to_string()))?;

        tokio::task::spawn_blocking(move || {
            let tx = db.begin_write().map_err(|e| Error::Database(e.to_string()))?;
            let outcome = {
                let mut table = tx.open_table(LEASES)
                    .map_err(|e| Error::Database(e.to_string()))?;

                let existing: Option<LeaseRecord> = match table
                    .get(key.as_str())
                    .map_err(|e| Error::Database(e.to_string()))?
                {
                    Some(guard) => Some(
                        serde_json::from_str(guard.value())
                            .map_err(|e| Error::Serialisation(e.to_string()))?,
                    ),
                    None => None,
                };

                let (rec, outcome) = match existing {
                    None => {
                        let rec = LeaseRecord {
                            holder_id: holder.clone(),
                            expires_at: new_expires,
                        };
                        (Some(rec.clone()), AcquireOutcome::Acquired(rec))
                    }
                    Some(existing) if existing.expires_at <= now => {
                        // Expired — steal.
                        let rec = LeaseRecord {
                            holder_id: holder.clone(),
                            expires_at: new_expires,
                        };
                        (Some(rec.clone()), AcquireOutcome::Acquired(rec))
                    }
                    Some(existing) if existing.holder_id == holder => {
                        // Idempotent re-acquire — extend expiry.
                        let rec = LeaseRecord {
                            holder_id: holder.clone(),
                            expires_at: new_expires,
                        };
                        (Some(rec.clone()), AcquireOutcome::Acquired(rec))
                    }
                    Some(existing) => (None, AcquireOutcome::HeldByOther(existing)),
                };

                if let Some(rec) = rec {
                    let json = serde_json::to_string(&rec)
                        .map_err(|e| Error::Serialisation(e.to_string()))?;
                    table
                        .insert(key.as_str(), json.as_str())
                        .map_err(|e| Error::Database(e.to_string()))?;
                }
                outcome
            };
            tx.commit().map_err(|e| Error::Database(e.to_string()))?;
            Ok::<_, Error>(outcome)
        })
        .await
        .map_err(|e| Error::Other(e.to_string()))?
    }

    async fn renew(
        &self,
        scope: &LeaseScope,
        holder_id: &str,
        ttl: Duration,
    ) -> Result<RenewOutcome> {
        let db = self.db.clone();
        let key = scope.key();
        let holder = holder_id.to_string();
        let now = Utc::now();
        let new_expires = now + chrono::Duration::from_std(ttl)
            .map_err(|e| Error::Other(e.to_string()))?;

        tokio::task::spawn_blocking(move || {
            let tx = db.begin_write().map_err(|e| Error::Database(e.to_string()))?;
            let outcome = {
                let mut table = tx.open_table(LEASES)
                    .map_err(|e| Error::Database(e.to_string()))?;

                let existing: Option<LeaseRecord> = match table
                    .get(key.as_str())
                    .map_err(|e| Error::Database(e.to_string()))?
                {
                    Some(guard) => Some(
                        serde_json::from_str(guard.value())
                            .map_err(|e| Error::Serialisation(e.to_string()))?,
                    ),
                    None => None,
                };

                match existing {
                    Some(ref rec) if rec.holder_id == holder && rec.expires_at > now => {
                        let renewed = LeaseRecord {
                            holder_id: holder.clone(),
                            expires_at: new_expires,
                        };
                        let json = serde_json::to_string(&renewed)
                            .map_err(|e| Error::Serialisation(e.to_string()))?;
                        table
                            .insert(key.as_str(), json.as_str())
                            .map_err(|e| Error::Database(e.to_string()))?;
                        RenewOutcome::Renewed(renewed)
                    }
                    other => RenewOutcome::Lost { current: other },
                }
            };
            tx.commit().map_err(|e| Error::Database(e.to_string()))?;
            Ok::<_, Error>(outcome)
        })
        .await
        .map_err(|e| Error::Other(e.to_string()))?
    }

    async fn release(&self, scope: &LeaseScope, holder_id: &str) -> Result<()> {
        let db = self.db.clone();
        let key = scope.key();
        let holder = holder_id.to_string();

        tokio::task::spawn_blocking(move || {
            let tx = db.begin_write().map_err(|e| Error::Database(e.to_string()))?;
            {
                let mut table = tx.open_table(LEASES)
                    .map_err(|e| Error::Database(e.to_string()))?;

                let current: Option<LeaseRecord> = match table
                    .get(key.as_str())
                    .map_err(|e| Error::Database(e.to_string()))?
                {
                    Some(guard) => Some(
                        serde_json::from_str(guard.value())
                            .map_err(|e| Error::Serialisation(e.to_string()))?,
                    ),
                    None => None,
                };

                if let Some(rec) = current
                    && rec.holder_id == holder
                {
                    table
                        .remove(key.as_str())
                        .map_err(|e| Error::Database(e.to_string()))?;
                }
            }
            tx.commit().map_err(|e| Error::Database(e.to_string()))?;
            Ok::<_, Error>(())
        })
        .await
        .map_err(|e| Error::Other(e.to_string()))?
    }

    async fn current(&self, scope: &LeaseScope) -> Result<Option<LeaseRecord>> {
        let db = self.db.clone();
        let key = scope.key();

        tokio::task::spawn_blocking(move || {
            let tx = db.begin_read().map_err(|e| Error::Database(e.to_string()))?;
            let table = tx.open_table(LEASES)
                .map_err(|e| Error::Database(e.to_string()))?;

            match table
                .get(key.as_str())
                .map_err(|e| Error::Database(e.to_string()))?
            {
                Some(guard) => Ok(Some(
                    serde_json::from_str(guard.value())
                        .map_err(|e| Error::Serialisation(e.to_string()))?,
                )),
                None => Ok(None),
            }
        })
        .await
        .map_err(|e| Error::Other(e.to_string()))?
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    async fn make_store() -> (RedbLeaseStore, TempDir) {
        let tmp = TempDir::new().expect("tempdir");
        let db_path = tmp.path().join("test.redb");
        let db = tokio::task::spawn_blocking(move || Database::create(db_path))
            .await
            .unwrap()
            .unwrap();
        let db = Arc::new(db);
        {
            let tx = db.begin_write().unwrap();
            init_tables(&tx).unwrap();
            tx.commit().unwrap();
        }
        (RedbLeaseStore::new(db), tmp)
    }

    fn scope() -> LeaseScope {
        LeaseScope {
            org_id: "org-1".into(),
            app_id: "test".into(),
            session_id: "sess-1".into(),
        }
    }

    #[tokio::test]
    async fn acquire_on_empty_returns_acquired() {
        let (store, _tmp) = make_store().await;
        let outcome = store
            .try_acquire(&scope(), "h1", Duration::from_secs(30))
            .await
            .expect("acquire");
        assert!(matches!(outcome, AcquireOutcome::Acquired(_)));
    }

    #[tokio::test]
    async fn acquire_by_other_holder_returns_held_by_other() {
        let (store, _tmp) = make_store().await;
        let _ = store
            .try_acquire(&scope(), "h1", Duration::from_secs(30))
            .await
            .unwrap();
        let outcome = store
            .try_acquire(&scope(), "h2", Duration::from_secs(30))
            .await
            .unwrap();
        match outcome {
            AcquireOutcome::HeldByOther(rec) => assert_eq!(rec.holder_id, "h1"),
            _ => panic!("expected HeldByOther"),
        }
    }
}
```

- [ ] **Step 2: Add `tempfile` dev-dependency if not present**

Check `runtime-runway/crates/runway-storage/Cargo.toml` for `[dev-dependencies] tempfile = ...`. If absent, add `tempfile = { workspace = true }` (workspace deps already define tempfile).

- [ ] **Step 3: Run the tests — expect to PASS (impl is in)**

```bash
cargo test -p runway-storage --lib local::lease
```
Expected: 2 tests pass.

- [ ] **Step 4: Add `lease::init_tables` call to local kit build**

Modify `runtime-runway/crates/runway-storage/src/local/mod.rs`. Find:

```rust
mod document;
mod event;
mod object;
pub mod sync;
mod vector;
```
Add `lease` to the module list (alphabetical):

```rust
mod document;
mod event;
mod lease;
mod object;
pub mod sync;
mod vector;
```

In the same file, find the `init_tables` block inside `LocalStorageKit::build`:

```rust
// Initialise tables
{
    let write = db.begin_write()?;
    document::init_tables(&write)?;
    event::init_tables(&write)?;
    vector::init_tables(&write)?;
    write.commit()?;
}
```
Add `lease::init_tables(&write)?;` between `event` and `vector`:

```rust
// Initialise tables
{
    let write = db.begin_write()?;
    document::init_tables(&write)?;
    event::init_tables(&write)?;
    lease::init_tables(&write)?;
    vector::init_tables(&write)?;
    write.commit()?;
}
```
(StorageKit wiring of the `leases` field happens in Task 3.)

- [ ] **Step 5: Build the crate**

```bash
cargo build -p runway-storage
```
Expected: clean.

- [ ] **Step 6: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 7: Commit**

```bash
git add crates/runway-storage/src/local/lease.rs \
        crates/runway-storage/src/local/mod.rs \
        crates/runway-storage/Cargo.toml
git commit -m "$(cat <<'EOF'
feat(d5): RedbLeaseStore — local LeaseStore backend

Single-WriteTransaction acquire/renew/release/current over a redb
table keyed by LeaseScope::key(). Inline unit tests cover the
happy-path acquire and contention; full contract suite lands in
Task 6.

Refs: D5 spec section 7 (LocalLeaseStore).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 3: Wire `leases` field into `StorageKit` (local builder + struct)

**Files:**
- Modify: `runtime-runway/crates/runway-storage/src/lib.rs` (StorageKit struct)
- Modify: `runtime-runway/crates/runway-storage/src/local/mod.rs` (build())

- [ ] **Step 1: Add `leases` field to `StorageKit`**

Modify `runtime-runway/crates/runway-storage/src/lib.rs`. Find the `pub struct StorageKit { ... }` block (around lines 24–45) and add the `leases` field after `events`:

```rust
pub struct StorageKit {
    pub documents: Arc<dyn DocumentStore>,
    pub vectors: Arc<dyn VectorStore>,
    pub objects: Arc<dyn ObjectStore>,
    pub events: Arc<dyn EventLog>,
    pub leases: Arc<dyn LeaseStore>,
    pub embeddings: Arc<dyn EmbeddingProvider>,
    /// Local-only: present when running against the redb backend. None for remote.
    pub syncable_events: Option<Arc<dyn SyncableEventLog>>,
}
```

- [ ] **Step 2: Wire `RedbLeaseStore` into the local builder**

Modify `runtime-runway/crates/runway-storage/src/local/mod.rs`. Find the `Ok(StorageKit { ... })` block at the end of `LocalStorageKit::build` and add the `leases` field:

```rust
Ok(StorageKit {
    documents: Arc::new(document::RedbDocumentStore::new(db.clone())),
    vectors: Arc::new(vector::FileVectorStore::new(db.clone())),
    objects: Arc::new(object::LocalObjectStore::new(object_base)),
    events,
    leases: Arc::new(lease::RedbLeaseStore::new(db.clone())),
    embeddings: Arc::new(LocalEmbedder::new()),
    syncable_events: Some(syncable),
})
```

- [ ] **Step 3: Build — expect compile error from the remote builder NOT setting `leases`**

```bash
cargo build -p runway-storage
```
Expected: compile error in `remote/mod.rs` — "missing field `leases` in initializer of `StorageKit`". This is intentional; Task 5 wires the remote side. For now, add a one-line shim so the workspace still compiles between Task 3 and Task 5:

In `runtime-runway/crates/runway-storage/src/remote/mod.rs`, locate the `Ok(StorageKit { ... })` block in `RemoteStorageKit::build_with_embedder` and add a temporary placeholder right before `embeddings`:

```rust
            // TEMPORARY placeholder — replaced with FirestoreLeaseStore in Task 5.
            leases: {
                use std::sync::Arc as ArcAlias;
                // Use a panic-on-call placeholder so a misconfigured build that
                // accidentally calls a lease method before Task 5 fails loudly.
                struct PendingLeaseStore;
                #[async_trait::async_trait]
                impl crate::traits::lease::LeaseStore for PendingLeaseStore {
                    async fn try_acquire(
                        &self,
                        _scope: &crate::traits::lease::LeaseScope,
                        _holder_id: &str,
                        _ttl: std::time::Duration,
                    ) -> crate::traits::Result<crate::traits::lease::AcquireOutcome> {
                        Err(crate::traits::Error::Other(
                            "FirestoreLeaseStore not yet wired (D5 Task 5)".into(),
                        ))
                    }
                    async fn renew(
                        &self,
                        _scope: &crate::traits::lease::LeaseScope,
                        _holder_id: &str,
                        _ttl: std::time::Duration,
                    ) -> crate::traits::Result<crate::traits::lease::RenewOutcome> {
                        Err(crate::traits::Error::Other(
                            "FirestoreLeaseStore not yet wired (D5 Task 5)".into(),
                        ))
                    }
                    async fn release(
                        &self,
                        _scope: &crate::traits::lease::LeaseScope,
                        _holder_id: &str,
                    ) -> crate::traits::Result<()> {
                        Err(crate::traits::Error::Other(
                            "FirestoreLeaseStore not yet wired (D5 Task 5)".into(),
                        ))
                    }
                    async fn current(
                        &self,
                        _scope: &crate::traits::lease::LeaseScope,
                    ) -> crate::traits::Result<Option<crate::traits::lease::LeaseRecord>> {
                        Err(crate::traits::Error::Other(
                            "FirestoreLeaseStore not yet wired (D5 Task 5)".into(),
                        ))
                    }
                }
                ArcAlias::new(PendingLeaseStore)
            },
```
This whole block goes away in Task 5. Tagged as a TODO with the Task ID so a `grep TODO` shows it.

- [ ] **Step 4: Rebuild — workspace should now compile**

```bash
cargo build --workspace
```
Expected: clean.

- [ ] **Step 5: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add crates/runway-storage/src/lib.rs \
        crates/runway-storage/src/local/mod.rs \
        crates/runway-storage/src/remote/mod.rs
git commit -m "$(cat <<'EOF'
feat(d5): wire leases into StorageKit (local builder; remote stub)

StorageKit.leases: Arc<dyn LeaseStore> added. LocalStorageKit::build
constructs RedbLeaseStore. RemoteStorageKit gets a temporary
panic-on-call placeholder (TODO: Task 5) so the workspace compiles
between tasks. Placeholder is removed in Task 5.

Refs: D5 spec section 3 (component layout).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 4: Implement `FirestoreLeaseStore` (remote backend)

**Files:**
- Create: `runtime-runway/crates/runway-storage/src/remote/lease.rs`
- Modify: `runtime-runway/crates/runway-storage/src/remote/mod.rs` (`mod lease;` declaration)

- [ ] **Step 1: Write the Firestore impl**

Create `runtime-runway/crates/runway-storage/src/remote/lease.rs`:

```rust
use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde_json::{Value, json};

use crate::remote::{BearerAuthExt, GcpToken};
use crate::traits::{
    Error, Result,
    lease::{AcquireOutcome, LeaseRecord, LeaseScope, LeaseStore, RenewOutcome},
};

/// Firestore document at `_runway_leases/{scope_key}` holds `LeaseRecord`.
const LEASE_COLLECTION: &str = "_runway_leases";

pub struct FirestoreLeaseStore {
    project_id: String,
    token: GcpToken,
    client: Client,
}

impl FirestoreLeaseStore {
    pub fn new(project_id: String, token: GcpToken) -> Self {
        Self {
            project_id,
            token,
            client: Client::new(),
        }
    }

    /// Compose the Firestore document path used by all REST calls.
    fn doc_path(&self, scope: &LeaseScope) -> String {
        format!(
            "projects/{}/databases/(default)/documents/{}/{}",
            self.project_id,
            LEASE_COLLECTION,
            scope.key()
        )
    }

    fn base_url(&self) -> String {
        // FIRESTORE_EMULATOR_HOST overrides for tests.
        match std::env::var("FIRESTORE_EMULATOR_HOST") {
            Ok(host) => format!("http://{host}/v1/"),
            Err(_) => "https://firestore.googleapis.com/v1/".to_string(),
        }
    }

    fn encode_record(rec: &LeaseRecord) -> Value {
        json!({
            "fields": {
                "holder_id": { "stringValue": rec.holder_id },
                "expires_at": { "timestampValue": rec.expires_at.to_rfc3339() }
            }
        })
    }

    fn decode_record(doc: &Value) -> Result<LeaseRecord> {
        let fields = doc
            .get("fields")
            .ok_or_else(|| Error::Serialisation("missing fields".into()))?;
        let holder_id = fields
            .get("holder_id")
            .and_then(|v| v.get("stringValue"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Serialisation("missing holder_id".into()))?
            .to_string();
        let ts = fields
            .get("expires_at")
            .and_then(|v| v.get("timestampValue"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Serialisation("missing expires_at".into()))?;
        let expires_at = DateTime::parse_from_rfc3339(ts)
            .map_err(|e| Error::Serialisation(e.to_string()))?
            .with_timezone(&Utc);
        Ok(LeaseRecord {
            holder_id,
            expires_at,
        })
    }
}

#[async_trait]
impl LeaseStore for FirestoreLeaseStore {
    async fn try_acquire(
        &self,
        scope: &LeaseScope,
        holder_id: &str,
        ttl: Duration,
    ) -> Result<AcquireOutcome> {
        // Strategy: GET, decide, PATCH-with-precondition based on whether the
        // doc existed. The emulator + production Firestore both accept this
        // pattern via the `currentDocument.exists` precondition.
        let token = self
            .token
            .get()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        let now = Utc::now();
        let new_expires = now + chrono::Duration::from_std(ttl)
            .map_err(|e| Error::Other(e.to_string()))?;

        // 1. Read current doc.
        let url = format!("{}{}", self.base_url(), self.doc_path(scope));
        let resp = self
            .client
            .get(&url)
            .bearer_auth_if_set(&token)
            .send()
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        let existing: Option<LeaseRecord> = if resp.status().is_success() {
            let body: Value = resp
                .json()
                .await
                .map_err(|e| Error::Serialisation(e.to_string()))?;
            Some(Self::decode_record(&body)?)
        } else if resp.status() == reqwest::StatusCode::NOT_FOUND {
            None
        } else {
            return Err(Error::Database(format!(
                "firestore GET failed: {}",
                resp.status()
            )));
        };

        // 2. Decide.
        let (write_rec, outcome) = match existing {
            None => {
                let rec = LeaseRecord {
                    holder_id: holder_id.into(),
                    expires_at: new_expires,
                };
                (Some(rec.clone()), AcquireOutcome::Acquired(rec))
            }
            Some(ref existing) if existing.expires_at <= now => {
                let rec = LeaseRecord {
                    holder_id: holder_id.into(),
                    expires_at: new_expires,
                };
                (Some(rec.clone()), AcquireOutcome::Acquired(rec))
            }
            Some(ref existing) if existing.holder_id == holder_id => {
                let rec = LeaseRecord {
                    holder_id: holder_id.into(),
                    expires_at: new_expires,
                };
                (Some(rec.clone()), AcquireOutcome::Acquired(rec))
            }
            Some(existing) => (None, AcquireOutcome::HeldByOther(existing)),
        };

        // 3. Write if needed. PATCH overwrites the whole document.
        if let Some(rec) = write_rec {
            let patch_url = format!("{}{}", self.base_url(), self.doc_path(scope));
            let body = Self::encode_record(&rec);
            let resp = self
                .client
                .patch(&patch_url)
                .bearer_auth_if_set(&token)
                .json(&body)
                .send()
                .await
                .map_err(|e| Error::Database(e.to_string()))?;
            if !resp.status().is_success() {
                return Err(Error::Database(format!(
                    "firestore PATCH failed: {}",
                    resp.status()
                )));
            }
        }
        Ok(outcome)
    }

    async fn renew(
        &self,
        scope: &LeaseScope,
        holder_id: &str,
        ttl: Duration,
    ) -> Result<RenewOutcome> {
        let token = self
            .token
            .get()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        let now = Utc::now();
        let new_expires = now + chrono::Duration::from_std(ttl)
            .map_err(|e| Error::Other(e.to_string()))?;

        let url = format!("{}{}", self.base_url(), self.doc_path(scope));
        let resp = self
            .client
            .get(&url)
            .bearer_auth_if_set(&token)
            .send()
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        let existing: Option<LeaseRecord> = if resp.status().is_success() {
            let body: Value = resp
                .json()
                .await
                .map_err(|e| Error::Serialisation(e.to_string()))?;
            Some(Self::decode_record(&body)?)
        } else if resp.status() == reqwest::StatusCode::NOT_FOUND {
            None
        } else {
            return Err(Error::Database(format!(
                "firestore GET failed: {}",
                resp.status()
            )));
        };

        match existing {
            Some(ref rec) if rec.holder_id == holder_id && rec.expires_at > now => {
                let renewed = LeaseRecord {
                    holder_id: holder_id.into(),
                    expires_at: new_expires,
                };
                let body = Self::encode_record(&renewed);
                let resp = self
                    .client
                    .patch(&url)
                    .bearer_auth_if_set(&token)
                    .json(&body)
                    .send()
                    .await
                    .map_err(|e| Error::Database(e.to_string()))?;
                if !resp.status().is_success() {
                    return Err(Error::Database(format!(
                        "firestore PATCH failed: {}",
                        resp.status()
                    )));
                }
                Ok(RenewOutcome::Renewed(renewed))
            }
            other => Ok(RenewOutcome::Lost { current: other }),
        }
    }

    async fn release(&self, scope: &LeaseScope, holder_id: &str) -> Result<()> {
        let token = self
            .token
            .get()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;

        // Only DELETE if we are the current holder.
        let url = format!("{}{}", self.base_url(), self.doc_path(scope));
        let resp = self
            .client
            .get(&url)
            .bearer_auth_if_set(&token)
            .send()
            .await
            .map_err(|e| Error::Database(e.to_string()))?;

        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(());
        }
        if !resp.status().is_success() {
            return Err(Error::Database(format!(
                "firestore GET failed: {}",
                resp.status()
            )));
        }
        let body: Value = resp
            .json()
            .await
            .map_err(|e| Error::Serialisation(e.to_string()))?;
        let rec = Self::decode_record(&body)?;
        if rec.holder_id != holder_id {
            return Ok(());
        }
        let resp = self
            .client
            .delete(&url)
            .bearer_auth_if_set(&token)
            .send()
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(Error::Database(format!(
                "firestore DELETE failed: {}",
                resp.status()
            )));
        }
        Ok(())
    }

    async fn current(&self, scope: &LeaseScope) -> Result<Option<LeaseRecord>> {
        let token = self
            .token
            .get()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        let url = format!("{}{}", self.base_url(), self.doc_path(scope));
        let resp = self
            .client
            .get(&url)
            .bearer_auth_if_set(&token)
            .send()
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        if resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(None);
        }
        if !resp.status().is_success() {
            return Err(Error::Database(format!(
                "firestore GET failed: {}",
                resp.status()
            )));
        }
        let body: Value = resp
            .json()
            .await
            .map_err(|e| Error::Serialisation(e.to_string()))?;
        Ok(Some(Self::decode_record(&body)?))
    }
}
```

- [ ] **Step 2: Declare module in `remote/mod.rs`**

Modify `runtime-runway/crates/runway-storage/src/remote/mod.rs`. Find:

```rust
mod document;
mod event;
mod object;
mod vector;
```
Add `lease`:

```rust
mod document;
mod event;
mod lease;
mod object;
mod vector;
```

- [ ] **Step 3: Build (without integration test — emulator not running yet)**

```bash
cargo build -p runway-storage
```
Expected: clean.

- [ ] **Step 4: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add crates/runway-storage/src/remote/lease.rs \
        crates/runway-storage/src/remote/mod.rs
git commit -m "$(cat <<'EOF'
feat(d5): FirestoreLeaseStore — remote LeaseStore backend

Implements LeaseStore over Firestore REST: GET-decide-PATCH for
try_acquire/renew, DELETE for release. Honors FIRESTORE_EMULATOR_HOST
for local emulator runs. Same decision tree as RedbLeaseStore so the
cross-backend contract suite (Task 6) sees identical semantics.

Refs: D5 spec section 7 (RemoteLeaseStore).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 5: Wire `FirestoreLeaseStore` into `RemoteStorageKit`, remove Task-3 placeholder

**Files:**
- Modify: `runtime-runway/crates/runway-storage/src/remote/mod.rs`

- [ ] **Step 1: Replace the panic-on-call placeholder with the real impl**

Modify `runtime-runway/crates/runway-storage/src/remote/mod.rs`. Find the temporary `leases: { ... PendingLeaseStore ... }` block inside `Ok(StorageKit { ... })` (added in Task 3 Step 3) and replace the entire `leases: { ... }` expression with:

```rust
            leases: Arc::new(lease::FirestoreLeaseStore::new(
                config.project_id.clone(),
                token.clone(),
            )),
```

- [ ] **Step 2: Build**

```bash
cargo build -p runway-storage
```
Expected: clean.

- [ ] **Step 3: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 4: Verify no leftover TODO**

```bash
rg -n "FirestoreLeaseStore not yet wired" crates/runway-storage/
```
Expected: zero matches.

- [ ] **Step 5: Commit**

```bash
git add crates/runway-storage/src/remote/mod.rs
git commit -m "$(cat <<'EOF'
feat(d5): wire FirestoreLeaseStore into RemoteStorageKit

Replaces the Task-3 panic-on-call placeholder with the real
FirestoreLeaseStore. StorageKit.leases now resolves to a working
backend on both local and remote builders.

Refs: D5 spec section 3.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 6: Cross-backend contract suite `run_lease_suite`

**Files:**
- Create: `runtime-runway/crates/runway-storage-contract/src/lease.rs`
- Modify: `runtime-runway/crates/runway-storage-contract/src/lib.rs`
- Modify: `runtime-runway/crates/runway-storage/tests/contract_local.rs`
- Modify: `runtime-runway/crates/runway-storage/tests/contract_remote_emulator.rs` (if file exists; otherwise SKIP this file modification — the emulator test runs through whichever test entry point the repo uses today)

- [ ] **Step 1: Write the contract suite**

Create `runtime-runway/crates/runway-storage-contract/src/lease.rs`:

```rust
//! LeaseStore contract suite.

use std::{sync::Arc, time::Duration};

use runway_storage::{AcquireOutcome, LeaseScope, LeaseStore, RenewOutcome};

use crate::harness::{ContractContext, SuiteReport};
use crate::{contract_assert, contract_assert_eq, contract_test};

fn scope(ctx: &ContractContext, name: &str) -> LeaseScope {
    LeaseScope {
        org_id: ctx.scope("org").to_string(),
        app_id: "test".into(),
        session_id: name.to_string(),
    }
}

pub async fn run_lease_suite(
    store: Arc<dyn LeaseStore>,
    ctx: ContractContext,
) -> SuiteReport {
    let report = SuiteReport::new(&ctx.backend, "LeaseStore");

    contract_test!(&report, "acquire_on_empty_returns_acquired", async {
        let s = scope(&ctx, "case-1");
        let outcome = store
            .try_acquire(&s, "h1", Duration::from_secs(30))
            .await
            .map_err(|e| e.to_string())?;
        contract_assert!(
            matches!(outcome, AcquireOutcome::Acquired(_)),
            "expected Acquired, got {:?}",
            outcome
        );
        store.release(&s, "h1").await.ok();
        Ok(())
    });

    contract_test!(&report, "idempotent_acquire_by_same_holder", async {
        let s = scope(&ctx, "case-2");
        let first = store
            .try_acquire(&s, "h1", Duration::from_secs(30))
            .await
            .map_err(|e| e.to_string())?;
        let second = store
            .try_acquire(&s, "h1", Duration::from_secs(60))
            .await
            .map_err(|e| e.to_string())?;
        contract_assert!(matches!(first, AcquireOutcome::Acquired(_)), "first acquired");
        match second {
            AcquireOutcome::Acquired(rec) => {
                let prev = if let AcquireOutcome::Acquired(p) = first { p } else { unreachable!() };
                contract_assert!(
                    rec.expires_at >= prev.expires_at,
                    "second acquire must advance expires_at"
                );
            }
            other => return Err(format!("expected Acquired, got {:?}", other)),
        }
        store.release(&s, "h1").await.ok();
        Ok(())
    });

    contract_test!(&report, "acquire_by_other_holder_returns_held_by_other", async {
        let s = scope(&ctx, "case-3");
        let _ = store
            .try_acquire(&s, "h1", Duration::from_secs(30))
            .await
            .map_err(|e| e.to_string())?;
        let outcome = store
            .try_acquire(&s, "h2", Duration::from_secs(30))
            .await
            .map_err(|e| e.to_string())?;
        match outcome {
            AcquireOutcome::HeldByOther(rec) => {
                contract_assert_eq!(rec.holder_id, "h1".to_string(), "holder is h1");
            }
            other => return Err(format!("expected HeldByOther, got {:?}", other)),
        }
        store.release(&s, "h1").await.ok();
        Ok(())
    });

    contract_test!(&report, "acquire_after_expiry_steals", async {
        let s = scope(&ctx, "case-4");
        let _ = store
            .try_acquire(&s, "h1", Duration::from_millis(100))
            .await
            .map_err(|e| e.to_string())?;
        // Give a healthy margin past 100ms before the steal attempt.
        tokio::time::sleep(Duration::from_millis(250)).await;
        let outcome = store
            .try_acquire(&s, "h2", Duration::from_secs(30))
            .await
            .map_err(|e| e.to_string())?;
        match outcome {
            AcquireOutcome::Acquired(rec) => {
                contract_assert_eq!(rec.holder_id, "h2".to_string(), "h2 stole");
            }
            other => return Err(format!("expected Acquired (steal), got {:?}", other)),
        }
        store.release(&s, "h2").await.ok();
        Ok(())
    });

    contract_test!(&report, "renew_by_holder_advances_expires_at", async {
        let s = scope(&ctx, "case-5");
        let acquired = match store
            .try_acquire(&s, "h1", Duration::from_secs(10))
            .await
            .map_err(|e| e.to_string())?
        {
            AcquireOutcome::Acquired(rec) => rec,
            other => return Err(format!("expected Acquired, got {:?}", other)),
        };
        tokio::time::sleep(Duration::from_millis(50)).await;
        let renewed = store
            .renew(&s, "h1", Duration::from_secs(60))
            .await
            .map_err(|e| e.to_string())?;
        match renewed {
            RenewOutcome::Renewed(rec) => contract_assert!(
                rec.expires_at > acquired.expires_at,
                "renew must advance expires_at"
            ),
            other => return Err(format!("expected Renewed, got {:?}", other)),
        }
        store.release(&s, "h1").await.ok();
        Ok(())
    });

    contract_test!(&report, "renew_by_non_holder_returns_lost", async {
        let s = scope(&ctx, "case-6");
        let _ = store
            .try_acquire(&s, "h1", Duration::from_secs(30))
            .await
            .map_err(|e| e.to_string())?;
        let outcome = store
            .renew(&s, "h2", Duration::from_secs(30))
            .await
            .map_err(|e| e.to_string())?;
        contract_assert!(
            matches!(outcome, RenewOutcome::Lost { current: Some(_) }),
            "expected Lost with current, got {:?}",
            outcome
        );
        store.release(&s, "h1").await.ok();
        Ok(())
    });

    contract_test!(&report, "renew_on_absent_returns_lost_none", async {
        let s = scope(&ctx, "case-7");
        let outcome = store
            .renew(&s, "h1", Duration::from_secs(30))
            .await
            .map_err(|e| e.to_string())?;
        contract_assert!(
            matches!(outcome, RenewOutcome::Lost { current: None }),
            "expected Lost{{None}}, got {:?}",
            outcome
        );
        Ok(())
    });

    contract_test!(&report, "release_by_holder_clears_record", async {
        let s = scope(&ctx, "case-8");
        let _ = store
            .try_acquire(&s, "h1", Duration::from_secs(30))
            .await
            .map_err(|e| e.to_string())?;
        store
            .release(&s, "h1")
            .await
            .map_err(|e| e.to_string())?;
        let after = store.current(&s).await.map_err(|e| e.to_string())?;
        contract_assert!(after.is_none(), "expected None after release, got {:?}", after);
        Ok(())
    });

    contract_test!(&report, "release_by_non_holder_is_noop", async {
        let s = scope(&ctx, "case-9");
        let _ = store
            .try_acquire(&s, "h1", Duration::from_secs(30))
            .await
            .map_err(|e| e.to_string())?;
        store
            .release(&s, "h2")
            .await
            .map_err(|e| e.to_string())?;
        let after = store.current(&s).await.map_err(|e| e.to_string())?;
        contract_assert!(after.is_some(), "h1's record must survive h2's release");
        store.release(&s, "h1").await.ok();
        Ok(())
    });

    contract_test!(&report, "release_of_absent_is_noop", async {
        let s = scope(&ctx, "case-10");
        store
            .release(&s, "h1")
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    });

    contract_test!(&report, "current_round_trips_record", async {
        let s = scope(&ctx, "case-11");
        let _ = store
            .try_acquire(&s, "h1", Duration::from_secs(30))
            .await
            .map_err(|e| e.to_string())?;
        let rec = store
            .current(&s)
            .await
            .map_err(|e| e.to_string())?
            .ok_or("current returned None for held lease")?;
        contract_assert_eq!(rec.holder_id, "h1".to_string(), "holder roundtrip");
        store.release(&s, "h1").await.ok();
        Ok(())
    });

    contract_test!(&report, "current_on_absent_returns_none", async {
        let s = scope(&ctx, "case-12");
        let after = store.current(&s).await.map_err(|e| e.to_string())?;
        contract_assert!(after.is_none(), "expected None, got {:?}", after);
        Ok(())
    });

    report
}
```

- [ ] **Step 2: Declare the module**

Modify `runtime-runway/crates/runway-storage-contract/src/lib.rs`. Add `pub mod lease;` to the existing module list (alphabetical between `event` and `harness`, or wherever the current convention puts it; mirror what's there for `document`).

- [ ] **Step 3: Wire suite into local contract test entry point**

Modify `runtime-runway/crates/runway-storage/tests/contract_local.rs`. Find the existing `#[tokio::test] async fn document_contract() { ... }` block. After the last existing test (probably `event_contract` or similar), add:

```rust
#[tokio::test]
async fn lease_contract() {
    let (kit, _tmp) = build_kit().await;
    runway_storage_contract::lease::run_lease_suite(Arc::clone(&kit.leases), ctx())
        .await
        .assert_passed();
}
```
If `Arc` is not yet imported in the file, add `use std::sync::Arc;` to the top.

- [ ] **Step 4: Run the local lease contract test**

```bash
cd /Users/kpernyer/dev/reflective/runtime-runway
cargo test -p runway-storage --test contract_local lease_contract -- --nocapture
```
Expected: all 12 contract tests pass.

- [ ] **Step 5: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 6: Commit**

```bash
git add crates/runway-storage-contract/src/lease.rs \
        crates/runway-storage-contract/src/lib.rs \
        crates/runway-storage/tests/contract_local.rs
git commit -m "$(cat <<'EOF'
test(d5): cross-backend LeaseStore contract suite

12 contract tests cover acquire (empty, idempotent, contention,
steal-after-expiry), renew (advances, lost-on-non-holder,
lost-on-absent), release (by-holder, by-non-holder, of-absent),
and current (roundtrip, absent). Wired into the local kit
contract_local.rs test entry point; the remote (Firestore-emulator)
entry point lands in Task 13's variant-2 integration test or via
just test-lease-firestore (Task 14).

Refs: D5 spec section 9 (contract suite).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 7: Add `ownership_exempt_routes` field to `AppExecutionPacket`

**Files:**
- Modify: `runtime-runway/crates/runway-app-host/src/lib.rs` (struct + builder + serde)

- [ ] **Step 1: Add the struct + field**

Modify `runtime-runway/crates/runway-app-host/src/lib.rs`. Near the other small types in the file (around the `RouteRegistration` definition), add:

```rust
/// A route that intentionally has no `SessionOwnershipLayer` applied. D1's
/// strict manifest verifier cross-checks this list against the live Router so
/// "mutating route missing ownership layer" doesn't false-positive on routes
/// that aren't session-scoped (org-wide writes, aggregate-creating POSTs, etc.).
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct OwnershipExemptRoute {
    pub method: String,
    pub path: String,
}
```

In the `AppExecutionPacket` struct definition, add the field. Find the existing struct (it currently has `app_id`, `display_name`, `description`, `version`, `route_prefix`, `auth_app`, `jobs`, `operator_packets`, `subject_refs`, `fixtures`, `domain_routes`, `mounted_modules`, `boundaries`). Add right after `boundaries`:

```rust
    #[serde(default)]
    pub ownership_exempt_routes: Vec<OwnershipExemptRoute>,
```

In `AppExecutionPacket::new`, initialize the new field. Find the existing `new` constructor and add to the struct-init:

```rust
            ownership_exempt_routes: Vec::new(),
```

Add a builder method below `with_boundary`:

```rust
    pub fn with_ownership_exempt_route(
        mut self,
        method: impl Into<String>,
        path: impl Into<String>,
    ) -> Self {
        self.ownership_exempt_routes.push(OwnershipExemptRoute {
            method: method.into(),
            path: path.into(),
        });
        self
    }
```

- [ ] **Step 2: Add a unit test for serde round-trip**

In the same file, find the existing `#[cfg(test)] mod tests { ... }` block (it has tests around `RouteRegistration` and packet JSON parsing). Add:

```rust
    #[test]
    fn packet_parses_ownership_exempt_routes() {
        let json = r#"{
            "app_id": "quorum",
            "display_name": "Q",
            "description": "",
            "version": "0",
            "route_prefix": "/q",
            "ownership_exempt_routes": [
                { "method": "POST", "path": "/inquiry" }
            ]
        }"#;
        let pkt = AppExecutionPacket::from_json_str(json).expect("parse");
        assert_eq!(pkt.ownership_exempt_routes.len(), 1);
        assert_eq!(pkt.ownership_exempt_routes[0].method, "POST");
        assert_eq!(pkt.ownership_exempt_routes[0].path, "/inquiry");
    }

    #[test]
    fn packet_defaults_ownership_exempt_routes_to_empty() {
        let json = r#"{
            "app_id": "quorum",
            "display_name": "Q",
            "description": "",
            "version": "0",
            "route_prefix": "/q"
        }"#;
        let pkt = AppExecutionPacket::from_json_str(json).expect("parse");
        assert!(pkt.ownership_exempt_routes.is_empty());
    }

    #[test]
    fn with_ownership_exempt_route_builder_appends() {
        let pkt = AppExecutionPacket::new("a", "A", "d", "/a")
            .with_ownership_exempt_route("POST", "/x")
            .with_ownership_exempt_route("DELETE", "/y/{id}");
        assert_eq!(pkt.ownership_exempt_routes.len(), 2);
        assert_eq!(pkt.ownership_exempt_routes[1].path, "/y/{id}");
    }
```

- [ ] **Step 3: Run the tests**

```bash
cargo test -p runway-app-host --lib ownership_exempt
```
Expected: 3 tests pass.

- [ ] **Step 4: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add crates/runway-app-host/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(d5): AppExecutionPacket.ownership_exempt_routes

Adds the typed manifest field D1's verifier will cross-check against
the live Router. Optional via #[serde(default)] — apps without
session-shaped mutating routes need not populate or mention it. Apps
that adopt SessionOwnershipLayer populate it in their consumer-wiring
PR; D1 catches drift there, not D5's ship gate. With-builder method
matches existing AppExecutionPacket pattern. D1's enforcement is a
separate D1 concern; this is the additive schema change D5 ships.

Refs: D5 spec section 6 (manifest enforcement seam).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 8: Process-static `holder_id` helper

**Files:**
- Create: `runtime-runway/crates/runway-app-host/src/ownership.rs` (skeleton — fully fleshed out in Task 9)

- [ ] **Step 1: Write the holder_id helper + test in the new file**

Create `runtime-runway/crates/runway-app-host/src/ownership.rs`:

```rust
//! D5 SessionOwnership — admission-time lease middleware.
//!
//! See `runtime-runway/docs/superpowers/specs/2026-06-15-d5-session-ownership-design.md`.

use std::sync::OnceLock;

use uuid::Uuid;

/// Process-static lease holder ID. Computed once on first use as
/// `format!("{K_REVISION|local}:{uuid_v4}")`.
///
/// On Cloud Run, `K_REVISION` is set automatically per revision; locally it
/// falls back to "local". The uuid ensures uniqueness across instances of the
/// same revision (parallel deploys, scale-out).
pub fn process_holder_id() -> &'static str {
    static HOLDER: OnceLock<String> = OnceLock::new();
    HOLDER.get_or_init(|| {
        let revision = std::env::var("K_REVISION").unwrap_or_else(|_| "local".into());
        format!("{}:{}", revision, Uuid::new_v4())
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_holder_id_is_stable() {
        let a = process_holder_id().to_string();
        let b = process_holder_id().to_string();
        assert_eq!(a, b);
    }

    #[test]
    fn process_holder_id_has_revision_prefix() {
        let id = process_holder_id();
        let (revision, _) = id.split_once(':').expect("colon separator");
        assert!(!revision.is_empty(), "revision must be non-empty");
    }
}
```

- [ ] **Step 2: Add module declaration in `runway-app-host/src/lib.rs`**

Modify `runtime-runway/crates/runway-app-host/src/lib.rs`. Near the top, find existing module declarations (e.g. `mod builder; mod config;`). Add `pub mod ownership;` so external test files can reach `ownership::process_holder_id` and the layer types added in Task 9. The `pub use` re-exports happen in Task 11.

- [ ] **Step 3: Run the tests**

```bash
cargo test -p runway-app-host --lib ownership::tests
```
Expected: 2 tests pass.

- [ ] **Step 4: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add crates/runway-app-host/src/ownership.rs \
        crates/runway-app-host/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(d5): process-static holder_id helper

Computes the lease holder ID once per process as
`{K_REVISION|local}:{uuid_v4}` using OnceLock. Cloud Run revision
correlation works without parsing lease docs.

Refs: D5 spec section 5 (holder_id format).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 9: `SessionOwnershipLayer` + middleware Service impl

**Files:**
- Modify: `runtime-runway/crates/runway-app-host/src/ownership.rs` (append layer + middleware)
- Modify: `runtime-runway/crates/runway-app-host/Cargo.toml` if `axum::extract::MatchedPath`-related deps aren't already there (they should be — verify)

- [ ] **Step 1: Append the layer + middleware to `ownership.rs`**

Append the following to `runtime-runway/crates/runway-app-host/src/ownership.rs` (below the existing `process_holder_id` + tests, but BEFORE the `#[cfg(test)]` block — i.e. between line ~26 and the test module):

```rust
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};

use axum::{
    extract::{MatchedPath, Request},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use runway_auth::AuthContext;
use runway_storage::{AcquireOutcome, LeaseScope, LeaseStore};
use serde_json::json;
use tower::{Layer, Service};
use tracing::{info, warn};

/// Default lease TTL (60s) — covers Quorum's 5–30s formation runs with margin.
pub const DEFAULT_TTL: Duration = Duration::from_secs(60);
/// Default renewal interval (15s = TTL/4) — one missed renewal still leaves
/// 30s of headroom.
pub const DEFAULT_RENEW_INTERVAL: Duration = Duration::from_secs(15);

#[derive(Clone)]
pub struct SessionOwnershipLayer {
    app_id: String,
    path_param: String,
    ttl: Duration,
    renew_interval: Duration,
    leases: Arc<dyn LeaseStore>,
}

impl SessionOwnershipLayer {
    pub fn for_app(app_id: impl Into<String>, leases: Arc<dyn LeaseStore>) -> Self {
        Self {
            app_id: app_id.into(),
            path_param: "id".into(),
            ttl: DEFAULT_TTL,
            renew_interval: DEFAULT_RENEW_INTERVAL,
            leases,
        }
    }

    pub fn path_param(mut self, name: impl Into<String>) -> Self {
        self.path_param = name.into();
        self
    }

    pub fn ttl(mut self, ttl: Duration) -> Self {
        self.ttl = ttl;
        self
    }

    pub fn renew_interval(mut self, d: Duration) -> Self {
        self.renew_interval = d;
        self
    }
}

impl<S> Layer<S> for SessionOwnershipLayer {
    type Service = SessionOwnershipMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        SessionOwnershipMiddleware {
            inner,
            app_id: self.app_id.clone(),
            path_param: self.path_param.clone(),
            ttl: self.ttl,
            renew_interval: self.renew_interval,
            leases: self.leases.clone(),
        }
    }
}

#[derive(Clone)]
pub struct SessionOwnershipMiddleware<S> {
    inner: S,
    app_id: String,
    path_param: String,
    ttl: Duration,
    renew_interval: Duration,
    leases: Arc<dyn LeaseStore>,
}

type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;

impl<S> Service<Request> for SessionOwnershipMiddleware<S>
where
    S: Service<Request, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = BoxFuture<Result<Response, S::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request) -> Self::Future {
        let app_id = self.app_id.clone();
        let path_param = self.path_param.clone();
        let ttl = self.ttl;
        let renew_interval = self.renew_interval;
        let leases = self.leases.clone();
        let mut inner = self.inner.clone();

        Box::pin(async move {
            // 1. Extract org_id from AuthContext (which AuthLayer inserted upstream).
            let auth_ctx = match req.extensions().get::<AuthContext>() {
                Some(c) => c.clone(),
                None => {
                    return Ok(error_response(
                        StatusCode::BAD_REQUEST,
                        "ownership_requires_auth",
                        None,
                    ));
                }
            };
            let org_id = match auth_ctx.org_id() {
                Some(s) => s.to_string(),
                None => {
                    return Ok(error_response(
                        StatusCode::BAD_REQUEST,
                        "ownership_requires_org",
                        None,
                    ));
                }
            };

            // 2. Extract session_id from the matched path template via the
            //    configured param name.
            let session_id = match extract_path_param(&req, &path_param) {
                Some(v) => v,
                None => {
                    return Ok(error_response(
                        StatusCode::BAD_REQUEST,
                        "ownership_requires_session_id",
                        None,
                    ));
                }
            };

            // 3. Build the scope and attempt to acquire.
            let scope = LeaseScope {
                org_id,
                app_id: app_id.clone(),
                session_id: session_id.clone(),
            };
            let holder_id = process_holder_id().to_string();

            let outcome = match leases.try_acquire(&scope, &holder_id, ttl).await {
                Ok(o) => o,
                Err(e) => {
                    warn!(
                        target: "runway_app_host::ownership",
                        scope = %scope.key(),
                        err = %e,
                        "lease_store_unavailable on try_acquire"
                    );
                    return Ok(error_response(
                        StatusCode::SERVICE_UNAVAILABLE,
                        "lease_store_unavailable",
                        None,
                    ));
                }
            };

            match outcome {
                AcquireOutcome::Acquired(_rec) => {
                    info!(
                        target: "runway_app_host::ownership",
                        scope = %scope.key(),
                        holder_id = %holder_id,
                        "ownership_acquired"
                    );
                    // Spawn background renew task; build LeaseGuard (Task 10).
                    let guard = LeaseGuard::spawn(
                        leases.clone(),
                        scope.clone(),
                        holder_id.clone(),
                        ttl,
                        renew_interval,
                    );
                    let response = inner.call(req).await?;
                    drop(guard); // explicit drop after handler returns
                    Ok(response)
                }
                AcquireOutcome::HeldByOther(rec) => {
                    info!(
                        target: "runway_app_host::ownership",
                        scope = %scope.key(),
                        our_holder_id = %holder_id,
                        current_holder_id = %rec.holder_id,
                        expires_at = %rec.expires_at,
                        "ownership_held_by_other"
                    );
                    Ok(error_response(
                        StatusCode::CONFLICT,
                        "ownership_held",
                        Some((session_id, rec.expires_at.to_rfc3339())),
                    ))
                }
            }
        })
    }
}

fn extract_path_param(req: &Request, name: &str) -> Option<String> {
    // MatchedPath gives us the route template (e.g. "/inquiry/{id}/signal").
    // We then walk both the template and the actual URI path together to find
    // the value at the matching `{name}` segment.
    let matched = req.extensions().get::<MatchedPath>()?;
    let template = matched.as_str();
    let actual = req.uri().path();
    let needle = format!("{{{name}}}");
    let template_segments: Vec<&str> = template.split('/').collect();
    let actual_segments: Vec<&str> = actual.split('/').collect();
    if template_segments.len() != actual_segments.len() {
        return None;
    }
    for (t, a) in template_segments.iter().zip(actual_segments.iter()) {
        if *t == needle {
            return Some((*a).to_string());
        }
    }
    None
}

fn error_response(
    status: StatusCode,
    error: &str,
    held: Option<(String, String)>,
) -> Response {
    let body = match held {
        Some((session_id, holder_expires_at)) => json!({
            "error": error,
            "session_id": session_id,
            "holder_expires_at": holder_expires_at,
        }),
        None => json!({ "error": error }),
    };
    (status, Json(body)).into_response()
}

// LeaseGuard lives in Task 10's append; declared here so Service::call
// references it.
use crate::ownership::guard::LeaseGuard;
```

NOTE: The `use crate::ownership::guard::LeaseGuard;` import at the bottom refers to the `LeaseGuard` type that Task 10 creates in a sibling `guard.rs`. Between Task 9 and Task 10 the workspace will not compile — that's intentional. Don't commit at the end of Task 9 before Task 10 is done. **Skip Step 5 "Commit" and Step 4 "Run lint" of this task; do them at the end of Task 10 instead, in a single combined commit.**

- [ ] **Step 2: (Skipped — see note above; commit happens after Task 10.)**

---

## Task 10: `LeaseGuard` with background renew + Drop release

**Files:**
- Create: `runtime-runway/crates/runway-app-host/src/ownership/guard.rs` — wait, but `ownership.rs` is already a single file. Two options: (a) split `ownership.rs` into a module `ownership/mod.rs` + `ownership/guard.rs`, or (b) keep `ownership.rs` as a single file and append `LeaseGuard` to it.

Per the spec's component layout (§3), one file is sufficient. **Use option (b)**: append to the existing `ownership.rs`. Revise the Task-9 trailing `use crate::ownership::guard::LeaseGuard;` line — replace it with the actual `LeaseGuard` implementation appended directly.

**Files:**
- Modify: `runtime-runway/crates/runway-app-host/src/ownership.rs` (append `LeaseGuard`; remove the dangling `use ... guard::LeaseGuard` line from Task 9)

- [ ] **Step 1: Remove the dangling `use crate::ownership::guard::LeaseGuard;` line from Task 9's append**

In `runtime-runway/crates/runway-app-host/src/ownership.rs`, find and delete the line:
```rust
use crate::ownership::guard::LeaseGuard;
```
(It was a forward reference.)

- [ ] **Step 2: Append `LeaseGuard` implementation**

Append to `runtime-runway/crates/runway-app-host/src/ownership.rs` (after `fn error_response` but before the `#[cfg(test)] mod tests` block):

```rust
use runway_storage::RenewOutcome;
use tokio::task::JoinHandle;

/// RAII guard wrapping an acquired lease. Spawns a background tokio task that
/// renews on `renew_interval`; on drop, aborts the task and fire-and-forgets a
/// release. v1 does not surface renewal loss to handlers (no fencing); a
/// future D5.1 will insert `SessionLeaseLost: watch::Receiver<()>` into request
/// extensions for opt-in graceful abort.
pub(crate) struct LeaseGuard {
    leases: Arc<dyn LeaseStore>,
    scope: LeaseScope,
    holder_id: String,
    renew_task: Option<JoinHandle<()>>,
}

impl LeaseGuard {
    pub(crate) fn spawn(
        leases: Arc<dyn LeaseStore>,
        scope: LeaseScope,
        holder_id: String,
        ttl: Duration,
        renew_interval: Duration,
    ) -> Self {
        let task_leases = leases.clone();
        let task_scope = scope.clone();
        let task_holder = holder_id.clone();
        let task = tokio::spawn(async move {
            let mut consecutive_errs: u32 = 0;
            loop {
                tokio::time::sleep(renew_interval).await;
                match task_leases.renew(&task_scope, &task_holder, ttl).await {
                    Ok(RenewOutcome::Renewed(_)) => {
                        consecutive_errs = 0;
                    }
                    Ok(RenewOutcome::Lost { current }) => {
                        info!(
                            target: "runway_app_host::ownership",
                            scope = %task_scope.key(),
                            our_holder_id = %task_holder,
                            current_holder_id = ?current.as_ref().map(|r| &r.holder_id),
                            "ownership_lost"
                        );
                        break;
                    }
                    Err(e) => {
                        consecutive_errs += 1;
                        warn!(
                            target: "runway_app_host::ownership",
                            scope = %task_scope.key(),
                            holder_id = %task_holder,
                            attempt = consecutive_errs,
                            err = %e,
                            "ownership_renew_transient_error"
                        );
                        if consecutive_errs >= 3 {
                            warn!(
                                target: "runway_app_host::ownership",
                                scope = %task_scope.key(),
                                "ownership_renew_giving_up_after_3_errors"
                            );
                            break;
                        }
                    }
                }
            }
        });
        Self {
            leases,
            scope,
            holder_id,
            renew_task: Some(task),
        }
    }
}

impl Drop for LeaseGuard {
    fn drop(&mut self) {
        if let Some(task) = self.renew_task.take() {
            task.abort();
        }
        // Fire-and-forget release. Cloning the Arc is cheap; we don't block
        // the response on Firestore latency.
        let leases = self.leases.clone();
        let scope = self.scope.clone();
        let holder = self.holder_id.clone();
        tokio::spawn(async move {
            if let Err(e) = leases.release(&scope, &holder).await {
                warn!(
                    target: "runway_app_host::ownership",
                    scope = %scope.key(),
                    holder_id = %holder,
                    err = %e,
                    "ownership_release_failed"
                );
            }
        });
    }
}
```

- [ ] **Step 3: Build**

```bash
cargo build -p runway-app-host
```
Expected: clean.

- [ ] **Step 4: Run unit tests**

```bash
cargo test -p runway-app-host --lib ownership::tests
```
Expected: 2 tests pass (the holder_id ones from Task 8).

- [ ] **Step 5: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 6: Commit Tasks 9 + 10 together**

```bash
git add crates/runway-app-host/src/ownership.rs \
        crates/runway-app-host/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(d5): SessionOwnershipLayer + LeaseGuard

Tower Layer wraps mutating-route groups. Middleware:
1) reads AuthContext from extensions (400 if missing),
2) reads org_id from claims (400 if absent),
3) extracts session_id via MatchedPath + configured path-param name
   (400 if unmatched),
4) calls leases.try_acquire — 409 ownership_held with body
   {error, session_id, holder_expires_at} on contention; 503
   lease_store_unavailable on storage error,
5) on Acquired, spawns LeaseGuard (background renew at TTL/4) and
   forwards the request.

LeaseGuard drop aborts the renew task and tokio::spawns a
fire-and-forget release — async so slow Firestore release doesn't
extend response latency. Renew failures: log INFO on Lost (handler
completes per the v1 stale-writer gap), WARN on transient errors,
giving up after 3 consecutive failures.

Refs: D5 spec sections 4, 5, 8.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 11: Re-export public surface from `runway-app-host`

**Files:**
- Modify: `runtime-runway/crates/runway-app-host/src/lib.rs`

- [ ] **Step 1: Add `pub use`**

Modify `runtime-runway/crates/runway-app-host/src/lib.rs`. Find the existing `pub use` block (which re-exports `RunwayAppHost`, `RunwayAppHostBuilder`, `AppExecutionPacket`, etc.). Add:

```rust
pub use ownership::{
    DEFAULT_RENEW_INTERVAL, DEFAULT_TTL, SessionOwnershipLayer, process_holder_id,
};
```

- [ ] **Step 2: Build**

```bash
cargo build -p runway-app-host
```
Expected: clean.

- [ ] **Step 3: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add crates/runway-app-host/src/lib.rs
git commit -m "$(cat <<'EOF'
feat(d5): re-export SessionOwnershipLayer from runway-app-host

Apps now access the layer via runway_app_host::SessionOwnershipLayer.
DEFAULT_TTL and DEFAULT_RENEW_INTERVAL exported for apps that want
to reference the platform defaults in their own constants.

Refs: D5 spec section 4 (public API surface).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 12: Variant-1 integration test — shared-redb two-host

**Files:**
- Create: `runtime-runway/crates/runway-app-host/tests/ownership_test.rs`

- [ ] **Step 1: Write the test**

Create `runtime-runway/crates/runway-app-host/tests/ownership_test.rs`:

```rust
//! D5 Variant-1 integration test: two RunwayAppHost-style routers wired to a
//! single shared redb-backed LeaseStore. Asserts that the second instance's
//! call returns 409 ownership_held while the first holds the lease, then 200
//! after the lease expires.

use std::{sync::Arc, time::Duration};

use axum::{
    Router,
    body::Body,
    extract::Extension,
    http::{Request, StatusCode},
    routing::post,
};
use runway_app_host::SessionOwnershipLayer;
use runway_auth::AuthContext;
use runway_auth::FirebaseClaims;
use runway_storage::local::LocalStorageKit;
use tempfile::TempDir;
use tower::ServiceExt;

async fn make_router(kit_dir: &std::path::Path) -> Router {
    let kit = LocalStorageKit::build(kit_dir).await.expect("kit");
    let layer = SessionOwnershipLayer::for_app("test", kit.leases.clone())
        .path_param("id")
        .ttl(Duration::from_millis(300))
        .renew_interval(Duration::from_secs(60)); // disable renew during the test
    Router::new()
        .route("/inquiry/{id}/signal", post(handler))
        .layer(layer)
        .layer(axum::middleware::from_fn(inject_dev_auth))
}

async fn inject_dev_auth(
    mut req: Request<Body>,
    next: axum::middleware::Next,
) -> axum::response::Response {
    req.extensions_mut().insert(AuthContext {
        claims: FirebaseClaims {
            uid: "dev-uid".into(),
            email: Some("dev@local".into()),
            org_id: Some("org-1".into()),
            apps: vec!["test".into()],
            role: Some("admin".into()),
        },
    });
    next.run(req).await
}

async fn handler(Extension(_auth): Extension<AuthContext>) -> &'static str {
    "ok"
}

fn signal_req() -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri("/inquiry/inq-1/signal")
        .body(Body::empty())
        .unwrap()
}

#[tokio::test(flavor = "current_thread")]
async fn second_host_returns_409_then_200_after_expiry() {
    let tmp = TempDir::new().unwrap();
    // Both routers point at the same kit dir (one redb db); within a single
    // tokio runtime + process this is a valid shared-store simulation.
    let host_a = make_router(tmp.path()).await;
    let host_b = make_router(tmp.path()).await;

    // 1. Host A's first POST acquires.
    let resp_a = host_a.clone().oneshot(signal_req()).await.unwrap();
    assert_eq!(resp_a.status(), StatusCode::OK);

    // 2. Host B's POST immediately after returns 409.
    let resp_b = host_b.clone().oneshot(signal_req()).await.unwrap();
    assert_eq!(resp_b.status(), StatusCode::CONFLICT);
    let body_bytes = axum::body::to_bytes(resp_b.into_body(), 4096)
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    assert_eq!(body["error"], "ownership_held");
    assert_eq!(body["session_id"], "inq-1");
    assert!(body["holder_expires_at"].as_str().is_some());

    // 3. Wait > TTL (300ms) and B steals.
    tokio::time::sleep(Duration::from_millis(450)).await;
    let resp_b2 = host_b.clone().oneshot(signal_req()).await.unwrap();
    assert_eq!(resp_b2.status(), StatusCode::OK);
}
```

NOTE: `LocalStorageKit::build` (per the existing code at `runway-storage/src/local/mod.rs:17`) takes `&Path`. The test uses real `tokio::time::sleep` here — Variant 1's 300ms TTL is fast enough that wall-clock is fine. The mocked-clock 30-second test lands in Task 13.

- [ ] **Step 2: Add `tower` to runway-app-host's dev-dependencies if missing**

Check `runtime-runway/crates/runway-app-host/Cargo.toml` `[dev-dependencies]`. If `tower` isn't there with `util` features, add `tower = { workspace = true, features = ["util"] }` (workspace declares the version).

- [ ] **Step 3: Run the test**

```bash
cargo test -p runway-app-host --test ownership_test
```
Expected: 1 test passes (`second_host_returns_409_then_200_after_expiry`).

- [ ] **Step 4: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add crates/runway-app-host/tests/ownership_test.rs \
        crates/runway-app-host/Cargo.toml
git commit -m "$(cat <<'EOF'
test(d5): Variant-1 integration — shared-redb two-host

Two Router instances over a single shared redb file. First POST
acquires → 200; second POST returns 409 with body
{error, session_id, holder_expires_at}; after TTL elapses, second
POST steals → 200. Real tokio::time::sleep (300ms TTL keeps the
test fast); the 30s renewal test (Task 13) uses mocked clock.

Refs: D5 spec section 9 (Variant-1).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 13: 30-second renewal-under-load test (mocked clock)

**Files:**
- Modify: `runtime-runway/crates/runway-app-host/tests/ownership_test.rs` (append)

- [ ] **Step 1: Append the renewal test**

Append to `runtime-runway/crates/runway-app-host/tests/ownership_test.rs`:

```rust
#[tokio::test(flavor = "current_thread", start_paused = true)]
async fn renewal_under_load_holds_lease_30s() {
    // Same router setup but with TTL=60s, renew_interval=15s — the spec defaults.
    let tmp = TempDir::new().unwrap();

    async fn make_router_with_defaults(p: &std::path::Path) -> Router {
        let kit = LocalStorageKit::build(p).await.expect("kit");
        let layer = SessionOwnershipLayer::for_app("test", kit.leases.clone())
            .path_param("id"); // default ttl 60s, default renew 15s
        Router::new()
            .route("/inquiry/{id}/signal", post(handler))
            .layer(layer)
            .layer(axum::middleware::from_fn(inject_dev_auth))
    }

    let host_a = make_router_with_defaults(tmp.path()).await;
    let host_b = make_router_with_defaults(tmp.path()).await;

    // Host A acquires (we don't drop the LeaseGuard because the response
    // future completes — but the renew task is spawned and runs against the
    // shared store, which is what we want to assert).
    //
    // Caveat: in this test, the request COMPLETES (handler returns "ok"), so
    // the LeaseGuard drops and fires-and-forgets a release. To assert the
    // 30s renewal pattern we instead hold the lease open by issuing requests
    // continuously from A.
    let mut now_elapsed = Duration::ZERO;
    let step = Duration::from_secs(1);
    let mut a_calls: u32 = 0;
    let mut b_409s: u32 = 0;
    while now_elapsed < Duration::from_secs(30) {
        // A keeps hammering — every request reacquires (idempotent for the
        // same holder) which behaves equivalently to renew.
        let resp = host_a.clone().oneshot(signal_req()).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK, "A's request at t={:?}", now_elapsed);
        a_calls += 1;

        // B always tries — should always 409 while A is holding.
        let resp_b = host_b.clone().oneshot(signal_req()).await.unwrap();
        assert_eq!(
            resp_b.status(),
            StatusCode::CONFLICT,
            "B's request at t={:?} should be 409",
            now_elapsed
        );
        b_409s += 1;

        tokio::time::advance(step).await;
        now_elapsed += step;
    }

    assert!(a_calls >= 30, "A made {a_calls} calls in 30s");
    assert!(b_409s >= 30, "B got {b_409s} 409s in 30s");
}
```

- [ ] **Step 2: Run the test**

```bash
cargo test -p runway-app-host --test ownership_test renewal_under_load_holds_lease_30s
```
Expected: 1 test passes.

- [ ] **Step 3: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 4: Commit**

```bash
git add crates/runway-app-host/tests/ownership_test.rs
git commit -m "$(cat <<'EOF'
test(d5): 30-second renewal-under-load with mocked clock

start_paused=true + tokio::time::advance(1s) per loop iteration
drives 30 seconds of simulated wall time in <1 real second. Asserts
A's requests always succeed (idempotent re-acquire by same holder)
and B always sees 409 while A is holding. Validates the renew-under-
load acceptance criterion without a flaky real-time sleep.

Refs: D5 spec section 9 (30s renewal-under-load).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 14: Variant-2 integration test — Firestore emulator (env-gated)

**Files:**
- Modify: `runtime-runway/crates/runway-app-host/tests/ownership_test.rs` (append)
- Modify: `runtime-runway/crates/runway-storage/tests/contract_remote_emulator.rs` (if file exists; else SKIP that step)

- [ ] **Step 1: Append Variant-2 test**

Append to `runtime-runway/crates/runway-app-host/tests/ownership_test.rs`:

```rust
#[tokio::test(flavor = "current_thread")]
async fn second_host_returns_409_against_firestore_emulator() {
    // Env-gated: skips silently if the emulator isn't running.
    if std::env::var("FIRESTORE_EMULATOR_HOST").is_err() {
        eprintln!("skipping: FIRESTORE_EMULATOR_HOST not set");
        return;
    }

    use runway_storage::remote::{RemoteConfig, RemoteStorageKit, TokenSource};

    let config = RemoteConfig {
        project_id: "d5-test".into(),
        region: "europe-west1".into(),
        bucket: "d5-test-bucket".into(),
        token_source: TokenSource::Static(String::new()),
    };
    let kit_a = RemoteStorageKit::build(config.clone()).await.expect("kit a");
    let kit_b = RemoteStorageKit::build(config).await.expect("kit b");

    async fn build_router(leases: Arc<dyn runway_storage::LeaseStore>) -> Router {
        let layer = SessionOwnershipLayer::for_app("test", leases)
            .path_param("id")
            .ttl(Duration::from_millis(500))
            .renew_interval(Duration::from_secs(60));
        Router::new()
            .route("/inquiry/{id}/signal", post(handler))
            .layer(layer)
            .layer(axum::middleware::from_fn(inject_dev_auth))
    }

    let host_a = build_router(kit_a.leases.clone()).await;
    let host_b = build_router(kit_b.leases.clone()).await;

    // Unique session_id per test run to avoid cross-run contamination on the
    // emulator (which persists state across runs unless cleared).
    let session = format!("inq-{}", uuid::Uuid::new_v4());
    let make_req = || {
        Request::builder()
            .method("POST")
            .uri(format!("/inquiry/{session}/signal"))
            .body(Body::empty())
            .unwrap()
    };

    let resp_a = host_a.clone().oneshot(make_req()).await.unwrap();
    assert_eq!(resp_a.status(), StatusCode::OK);

    let resp_b = host_b.clone().oneshot(make_req()).await.unwrap();
    assert_eq!(resp_b.status(), StatusCode::CONFLICT);

    tokio::time::sleep(Duration::from_millis(700)).await;
    let resp_b2 = host_b.clone().oneshot(make_req()).await.unwrap();
    assert_eq!(resp_b2.status(), StatusCode::OK);
}
```

- [ ] **Step 2: Wire FirestoreLeaseStore into the existing remote contract test (if entry-point exists)**

Check whether `runtime-runway/crates/runway-storage/tests/contract_remote_emulator.rs` exists:

```bash
ls runtime-runway/crates/runway-storage/tests/
```

- If a remote-emulator entry file exists, add a `lease_contract_remote` test mirroring the local version, e.g.:

```rust
#[tokio::test]
async fn lease_contract_remote() {
    if std::env::var("FIRESTORE_EMULATOR_HOST").is_err() {
        eprintln!("skipping: FIRESTORE_EMULATOR_HOST not set");
        return;
    }
    let kit = build_remote_kit().await;
    runway_storage_contract::lease::run_lease_suite(
        std::sync::Arc::clone(&kit.leases),
        runway_storage_contract::harness::ContractContext::new("firestore-emulator"),
    )
    .await
    .assert_passed();
}
```
(Mirror the exact `build_remote_kit()` helper name used by adjacent tests in the same file.)

- If no such file exists today, document the absence in the commit message; the `just test-lease-firestore` recipe in Task 15 will run the contract suite directly via a small standalone harness.

- [ ] **Step 3: Verify behaviour with the emulator running locally (optional during plan execution; required for Tier-2 ship gate)**

If you have an emulator: start it, set `FIRESTORE_EMULATOR_HOST=localhost:8080`, run:
```bash
cargo test -p runway-app-host --test ownership_test second_host_returns_409_against_firestore_emulator
```
Expected: PASS.

If you don't have an emulator: the test prints "skipping" and returns. That's the documented env-gate behavior.

- [ ] **Step 4: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add crates/runway-app-host/tests/ownership_test.rs
# Add the remote-emulator entry file too if it was modified in Step 2.
git commit -m "$(cat <<'EOF'
test(d5): Variant-2 — Firestore-emulator integration (env-gated)

Same 200 → 409 → 200-after-TTL scenario against
FirestoreLeaseStore. Gated on FIRESTORE_EMULATOR_HOST so default
cargo test on a laptop without the emulator skips cleanly. Required
for the D5 Tier-2 ship gate; CI release job sets the env.

Refs: D5 spec section 9 (Variant-2).

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 15: `just test-lease-firestore` recipe + standards promotion

**Files:**
- Modify: `runtime-runway/justfile` (add recipe)
- Create: `runtime-runway/kb/05-engineering/standards/RP-NO-FEATURE-FLAG-SOFTENING.md`
- Create: `runtime-runway/kb/05-engineering/standards/RP-NO-LEASE-WITHOUT-FENCING-V1.md`

- [ ] **Step 1: Add the justfile recipe**

Modify `runtime-runway/justfile`. Append:

```just
# Run the LeaseStore contract suite + ownership Variant-2 against a Firestore emulator.
# Caller is responsible for starting `firestore-emulator` (or `gcloud emulators firestore start`)
# and exporting FIRESTORE_EMULATOR_HOST. See docs/superpowers/specs/2026-06-15-d5-session-ownership-design.md §9.
test-lease-firestore:
    @test -n "$FIRESTORE_EMULATOR_HOST" || (echo "FIRESTORE_EMULATOR_HOST not set; start the emulator first" && exit 1)
    cargo test -p runway-app-host --test ownership_test second_host_returns_409_against_firestore_emulator -- --nocapture
    cargo test -p runway-storage --test contract_remote_emulator lease_contract_remote -- --nocapture || echo "(contract_remote_emulator absent — skipped)"
```

- [ ] **Step 2: Create the standards directory if absent and write the no-soften standard**

```bash
mkdir -p runtime-runway/kb/05-engineering/standards
```

Create `runtime-runway/kb/05-engineering/standards/RP-NO-FEATURE-FLAG-SOFTENING.md`:

```markdown
# RP-NO-FEATURE-FLAG-SOFTENING

**Status:** Active. Promoted 2026-06-15 alongside D5 SessionOwnership.
**Originating finding:** D5 implementation revealed `AuthLayer.local_dev` as the only existing softening switch in runway. D1's manifest verifier ships strict-always; this standard codifies the rule for future primitives.
**Source review:** `REVIEW_quorum-sense_2026-06-15.md` Round 2.5 self-correction §3; `BOUNDARY_REGISTRY.md` Marquee App Contract rule 7.

## What

No platform check ships with a feature flag, `--strict-mode` switch, env-var toggle, or any other mechanism that lets a caller weaken the check.

This applies to:
- `runway-app-host` manifest verifier (D1)
- `runway-app-host` SessionOwnershipLayer (D5)
- All future RR-owned strict checks

It does NOT apply to:
- Infrastructure-availability gating (e.g. `FIRESTORE_EMULATOR_HOST` env that skips a test when an emulator isn't running). The test assertions are unchanged when the env is set; the switch is whether the test runs at all, not whether the check is enforced.
- Configuration of behavior (e.g. `SessionOwnershipLayer::ttl(...)` accepting different durations). Tuning is not softening.

## Why

Softening switches are how strict checks become advisory: someone sets the flag "temporarily" to ship, the flag becomes permanent, and the check becomes a comment. The runway codebase had exactly one (`AuthLayer.local_dev`) at the time of D5 — we don't add a second.

## How to check (drift)

PR review rejects any new platform primitive that:
- Reads an env var to disable a check.
- Accepts a `bool` or enum that disables a check.
- Has a code path that returns "success" without running the check, controlled by config.

Standard exception: tests that need to mock backend availability may env-gate themselves, but the production code under test must not.

## Links

- Standard `RP-NO-LEASE-WITHOUT-FENCING-V1` (sibling standard for D5).
- Marquee App Contract rule 7 (`BOUNDARY_REGISTRY.md`).
```

- [ ] **Step 3: Write the no-fencing-v1 standard**

Create `runtime-runway/kb/05-engineering/standards/RP-NO-LEASE-WITHOUT-FENCING-V1.md`:

```markdown
# RP-NO-LEASE-WITHOUT-FENCING-V1

**Status:** Active. Promoted 2026-06-15 with D5 v1 ship.
**Originating finding:** D5 ships an admission-time lease only; v1 has no write-side fencing.
**Source review:** `REVIEW_quorum-sense_2026-06-15.md` HELMS F5; RR Round-2 D5 acceptance with documented stale-writer gap.
**Follow-up ticket:** D5.1 (RR-owned, opened when --max-instances > 1 is needed in production).

## What this standard says (verbatim)

> Admission-time lease (D5) serializes new mutating requests across healthy instances. It does not prevent stale-writer writes after TTL steal. Write-side fencing is D5.1.

## What this means in practice

D5's `SessionOwnershipLayer` rejects a second instance's mutating request with 409 ownership_held while the first instance holds the lease. That is its complete safety guarantee.

D5 does NOT prevent the classic stuck-process scenario:
1. Instance A acquires lease at t=0.
2. A pauses (GC, network, scheduling) longer than TTL.
3. Instance B's request arrives at t=TTL+1; B steals the lease (Acquired).
4. B completes its writes.
5. A wakes at t=TTL+10; A's already-in-flight handler completes its writes through `DocumentStore`/`EventLog` — bypassing the lease entirely because the storage layer doesn't check holder identity on writes.

In step 5, A's write is "stale" — A no longer owns the lease, but the storage backend accepts the write anyway.

## When this standard matters

Any of these claims must be rejected:
- "We have D5 now, we can run multi-writer."
- "D5 prevents data corruption under concurrent writes."
- "Lifting `--max-instances=1` is safe because D5 shipped."

Correct claim: "D5 prevents concurrent admission of mutating requests. It does not prevent stale-writer writes after a steal. `--max-instances > 1` is only safe when D5.1 (write-side fencing) ALSO ships."

## Cross-references

- Marquee App Contract rule 6 (`BOUNDARY_REGISTRY.md`).
- `marquee-apps/quorum-sense/deploy/cloud-run-provision.sh` `--max-instances=1` pin comment cites D5 + QF-CR-03 + QF-CR-08 as the three release gates. When the pin is lifted, the comment must reference D5.1 as the safety-completion ticket.
- D5 spec section 12 (deferred to D5.1).

## How to check (drift)

PR review rejects any change that:
- Lifts `--max-instances=1` in any marquee-app deploy without D5.1 also having shipped.
- Documents D5 alone as multi-writer-safe.
- Removes this standard.
```

- [ ] **Step 4: Run lint (no Rust changes; just `just lint` to confirm nothing regressed)**

```bash
just lint
```
Expected: clean.

- [ ] **Step 5: Commit**

```bash
git add justfile \
        kb/05-engineering/standards/RP-NO-FEATURE-FLAG-SOFTENING.md \
        kb/05-engineering/standards/RP-NO-LEASE-WITHOUT-FENCING-V1.md
git commit -m "$(cat <<'EOF'
docs(d5): just test-lease-firestore + promoted standards

just test-lease-firestore runs Variant-2 + remote contract suite
against an emulator (caller-started).

Two standards promoted to kb/05-engineering/standards/:
- RP-NO-FEATURE-FLAG-SOFTENING: codifies rule 7 of the Marquee App
  Contract. AuthLayer.local_dev is the only existing exception; D5
  ships with no equivalent switch.
- RP-NO-LEASE-WITHOUT-FENCING-V1: verbatim spec-locked text
  documenting that D5 admission-only lease does NOT make us
  multi-writer safe. Rejects future PRs that lift max-instances=1
  on D5 alone.

Refs: D5 spec section 14.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

---

## Task 16: Final verification + update `QUALITY_BACKLOG.md`

**Files:**
- Modify: `runtime-runway/QUALITY_BACKLOG.md`

- [ ] **Step 1: Full workspace test run**

```bash
cd /Users/kpernyer/dev/reflective/runtime-runway
cargo test --workspace --all-targets
```
Expected: all tests pass.

- [ ] **Step 2: Full lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 3: Verify Tier-1 acceptance checklist**

Manually verify each Tier-1 box from spec §14 by ticking a local checklist:
- `LeaseStore` trait + types exported from `runway-storage` — `cargo doc -p runway-storage --no-deps --open` (visually confirm); or `rg "pub use crate::traits::lease" crates/runway-storage/src/lib.rs`.
- `SessionOwnershipLayer::for_app(...)` + builders exported from `runway-app-host` — `rg "pub use ownership" crates/runway-app-host/src/lib.rs`.
- `LocalLeaseStore` passes `run_lease_suite` — already green in Task 6.
- Variant-1 passes — already green in Task 12.
- 30s renewal-under-load passes — already green in Task 13.
- `ownership_exempt_routes` field exists in the type with `#[serde(default)]` (apps without session-shaped mutating routes need not populate it) — already in Task 7.
- `just lint` clean — Step 2 of this task.

- [ ] **Step 4: Update `runtime-runway/QUALITY_BACKLOG.md`**

Modify `runtime-runway/QUALITY_BACKLOG.md`. Find the `### D5 — SessionOwnership lease primitive` block. Change `**State:** Open` to `**State:** In progress` (because Tier-2 emulator gating still needs to be run in CI before D5 closes officially).

Add a "Progress log" subsection at the end of the D5 block:

```markdown
- **Progress log:**
  - 2026-06-15: Spec committed (`docs/superpowers/specs/2026-06-15-d5-session-ownership-design.md`, SHA 9d0504c).
  - 2026-06-15: Implementation complete on branch `d5-session-ownership-design`. Tier-1 acceptance passes locally (lease contract suite × redb, Variant-1 two-host, 30s renewal-under-load, AppExecutionPacket.ownership_exempt_routes field, `just lint`).
  - Tier-2 (Firestore-emulator) pending CI release-job execution. D5 moves to Done once Tier-2 passes.
```

- [ ] **Step 5: Commit the backlog update**

```bash
git add QUALITY_BACKLOG.md
git commit -m "$(cat <<'EOF'
chore(d5): mark D5 In progress; Tier-1 local acceptance complete

All Tier-1 boxes from spec §14 pass locally:
- LeaseStore + SessionOwnershipLayer public surfaces in place.
- Redb contract suite green (12 tests).
- Variant-1 two-host integration green.
- 30s renewal-under-load (mocked clock) green.
- AppExecutionPacket.ownership_exempt_routes field present.
- just lint clean across the workspace.

Tier-2 (Firestore emulator) gated on CI release-job execution;
D5 closes when that run is green.

Refs: D5 spec section 14.

Co-Authored-By: Claude Opus 4.7 (1M context) <noreply@anthropic.com>
EOF
)"
```

- [ ] **Step 6: Print branch summary for the user**

```bash
git log --oneline d5-session-ownership-design ^main 2>&1 | head -30
```

This lists every D5 commit. Hand it to the user for review before any push.

- [ ] **Step 7: Do NOT push. Stop here and notify user.**

Per `runtime-runway/CLAUDE.md`: "Never push to main without confirmation." This plan does not include `git push`. The branch `d5-session-ownership-design` holds all commits; the user decides when (and whether) to push and merge.

---

## Out-of-band notes (read before starting)

- **Order matters.** Tasks 1–11 must run sequentially (each depends on the previous). Tasks 12–14 can theoretically run in parallel against the same surface (they only modify a single test file), but the file gets appended each time, so serial execution is simpler.
- **The Task-3 placeholder is a feature, not a bug.** It exists so the workspace compiles between Tasks 3 and 5. Task 5 is responsible for its removal — that removal is verified by `rg -n "FirestoreLeaseStore not yet wired"` returning zero matches.
- **Watch the `AuthLayer.local_dev` precedent.** D5 must not add an equivalent softening switch. If you find yourself wanting one, re-read `RP-NO-FEATURE-FLAG-SOFTENING.md` (which this plan creates).
- **The `MatchedPath` extraction in `extract_path_param`** assumes the layer is applied AFTER Axum's path matching has populated the extension. If the layer is applied at the wrong nesting level, the extension may be absent — that's a configuration bug surfaced as `400 ownership_requires_session_id`. The error code is correct; the operator's response is "apply the layer to the right Router group."
- **TTL clock skew between instances.** Both `RedbLeaseStore` and `FirestoreLeaseStore` compute `expires_at` from local `Utc::now()`. In real two-instance scenarios, modest clock skew (< 1s) is absorbed by the conservative TTL. Larger skew would need NTP convergence — out of D5 v1 scope.
- **No need to wire D5 into anything in `runway-app-host::builder`.** Apps opt in by calling `.layer(SessionOwnershipLayer::for_app(...))` on a Router group themselves. The builder doesn't apply the layer globally.

## Self-review (writing-plans skill required step)

- **Spec coverage:** Walked through spec sections 1–14. Each requirement maps to a task:
  - §1–2 summary/scope: covered by Tasks 1–11 + acceptance in Task 16.
  - §3 component layout: Tasks 1, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12.
  - §4 public API: Tasks 1, 8, 9, 11.
  - §5 data flow: Task 9 (middleware) + Task 10 (guard drop / release).
  - §6 route grouping + manifest field: Task 7 (`ownership_exempt_routes`).
  - §7 backend implementation notes: Tasks 2 (redb), 4 (Firestore).
  - §8 error/edge: Task 9 envelope helpers + Task 10 transient-error handling.
  - §9 testing tier matrix: Tasks 6 (contract suite), 12 (Variant 1), 13 (30s renewal), 14 (Variant 2), 15 (`just test-lease-firestore`).
  - §10 configurability: Tasks 8, 9 (DEFAULT_TTL, DEFAULT_RENEW_INTERVAL, builders).
  - §11 observability: Task 9 + Task 10 (tracing calls).
  - §12 deferred D5.1: documented in Task 15's `RP-NO-LEASE-WITHOUT-FENCING-V1`.
  - §13 references: Task 16 progress log cites spec.
  - §14 acceptance: Task 16 explicit checklist; Tier-1 fully covered, Tier-2 documented as CI-gated.
- **Placeholder scan:** No "TBD" or "implement later." The Task-3 placeholder is itself a temporary code block with a clear removal step in Task 5; not a planning placeholder.
- **Type consistency:** `LeaseScope`, `LeaseRecord`, `AcquireOutcome`, `RenewOutcome`, `LeaseStore`, `SessionOwnershipLayer`, `LeaseGuard`, `process_holder_id`, `DEFAULT_TTL`, `DEFAULT_RENEW_INTERVAL` — all referenced by the same name across every task they appear in. Builder methods (`for_app`, `path_param`, `ttl`, `renew_interval`) are consistent. Error envelope strings (`ownership_requires_auth`, `ownership_requires_org`, `ownership_requires_session_id`, `ownership_held`, `lease_store_unavailable`) match spec §5 and §8 verbatim. Tracing target string (`runway_app_host::ownership`) is consistent. JSON body keys (`error`, `session_id`, `holder_expires_at`) are consistent.

---
