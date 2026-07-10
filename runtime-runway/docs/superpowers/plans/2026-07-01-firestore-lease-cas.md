# Firestore Lease CAS — Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the non-atomic GET→decide→PATCH sequence in `FirestoreLeaseStore` with Firestore conditional-write preconditions so that `try_acquire`, `renew`, and `release` are race-free under concurrent Cloud Run instances.

**Architecture:** Firestore's REST API accepts `?currentDocument.exists=false` (atomic create) and `?currentDocument.updateTime=<ts>` (optimistic CAS) query parameters on PATCH and DELETE. We capture the `updateTime` etag from every GET response and pass it as the precondition on the subsequent write. On precondition failure (HTTP 409 ABORTED or 400 FAILED_PRECONDITION) we re-read the current record and return the appropriate outcome rather than retrying — the middleware's 409 response handles that. `release` adds a conditional DELETE so a stolen lease cannot be deleted by the old holder.

**Tech Stack:** Rust edition 2024, `reqwest` (already a dep), Firestore REST API, Firestore emulator for tests.

## Global Constraints

- Rust edition 2024, rust-version 1.96.0.
- No new crate dependencies — `reqwest` and `serde_json` are already in the workspace.
- `just lint` must pass before every commit.
- No `unsafe`. No feature flags. Workspace deps only.
- All new tests that require the emulator are gated with `#[ignore = "requires FIRESTORE_EMULATOR_HOST"]` or `if std::env::var("FIRESTORE_EMULATOR_HOST").is_err() { return; }`.
- The existing 12-case contract suite (`contract_local.rs`) must still pass after every task.

---

## Files Changed

- **Modify:** `crates/runway-storage/src/remote/lease.rs` — all changes live here.
- **Create:** `crates/runway-storage/tests/firestore_lease_concurrent_test.rs` — concurrent mutual-exclusion test.

---

## Task 1: Extract `read_current` + `patch_conditional` + `delete_conditional` helpers

The three new private helpers replace the duplicated GET+PATCH/DELETE patterns in every method. Nothing in the public API changes. The existing contract tests remain green throughout.

**Files:**
- Modify: `crates/runway-storage/src/remote/lease.rs`

**Interfaces:**
- Produces:
  - `read_current(&self, scope: &LeaseScope, token: &str) -> Result<Option<(LeaseRecord, String)>>` — returns record + `updateTime` etag, or `None` for 404.
  - `enum Precondition { MustNotExist, MustHaveUpdateTime(String) }` — module-level private type.
  - `patch_conditional(&self, scope: &LeaseScope, token: &str, rec: &LeaseRecord, precondition: Precondition) -> Result<bool>` — `true` = written, `false` = precondition failed.
  - `delete_conditional(&self, scope: &LeaseScope, token: &str, update_time: &str) -> Result<()>` — no-ops on precondition failure.

- [ ] **Step 1: Write a unit test for `read_current` that will fail until the method exists**

Add a `#[cfg(test)]` block at the bottom of `crates/runway-storage/src/remote/lease.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precondition_variant_coverage() {
        // Compile-time proof that both variants exist and are pattern-matchable.
        let p = Precondition::MustNotExist;
        assert!(matches!(p, Precondition::MustNotExist));
        let q = Precondition::MustHaveUpdateTime("ts".into());
        assert!(matches!(q, Precondition::MustHaveUpdateTime(_)));
    }
}
```

- [ ] **Step 2: Run — expect compile error (Precondition not defined)**

```bash
cargo test -p runway-storage --lib remote::lease::tests 2>&1 | head -5
```
Expected: error — `cannot find type Precondition`.

- [ ] **Step 3: Add `Precondition` enum and the three helper methods**

Insert the following between the closing `}` of `impl FirestoreLeaseStore` (after `decode_record`) and the `#[async_trait]` line. Add the enum just above `impl FirestoreLeaseStore`:

```rust
/// CAS precondition for conditional Firestore writes.
enum Precondition {
    /// Document must not exist (atomic create).
    MustNotExist,
    /// Document's `updateTime` must equal this RFC3339 string (optimistic update).
    MustHaveUpdateTime(String),
}
```

Then add three private `async fn` methods inside `impl FirestoreLeaseStore`, before the closing `}` of that block:

```rust
    /// GET the document and return `(LeaseRecord, updateTime_etag)`, or `None` for 404.
    async fn read_current(
        &self,
        scope: &LeaseScope,
        token: &str,
    ) -> Result<Option<(LeaseRecord, String)>> {
        let url = format!("{}{}", self.base_url(), self.doc_path(scope));
        let resp = self
            .client
            .get(&url)
            .bearer_auth_if_set(token)
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
        let record = Self::decode_record(&body)?;
        let update_time = body["updateTime"]
            .as_str()
            .ok_or_else(|| Error::Serialisation("missing updateTime in Firestore response".into()))?
            .to_string();
        Ok(Some((record, update_time)))
    }

    /// PATCH the document with a precondition. Returns `true` if written, `false`
    /// if the precondition was rejected (another writer won the race).
    async fn patch_conditional(
        &self,
        scope: &LeaseScope,
        token: &str,
        rec: &LeaseRecord,
        precondition: Precondition,
    ) -> Result<bool> {
        let url = format!("{}{}", self.base_url(), self.doc_path(scope));
        let body = Self::encode_record(rec);
        let resp = match precondition {
            Precondition::MustNotExist => self
                .client
                .patch(&url)
                .bearer_auth_if_set(token)
                .query(&[("currentDocument.exists", "false")])
                .json(&body)
                .send()
                .await
                .map_err(|e| Error::Database(e.to_string()))?,
            Precondition::MustHaveUpdateTime(ref ts) => self
                .client
                .patch(&url)
                .bearer_auth_if_set(token)
                .query(&[("currentDocument.updateTime", ts.as_str())])
                .json(&body)
                .send()
                .await
                .map_err(|e| Error::Database(e.to_string()))?,
        };
        if resp.status().is_success() {
            return Ok(true);
        }
        // Firestore returns 409 ABORTED for updateTime violations and 400
        // FAILED_PRECONDITION / ALREADY_EXISTS for exists=false violations.
        if matches!(
            resp.status(),
            reqwest::StatusCode::CONFLICT | reqwest::StatusCode::BAD_REQUEST
        ) {
            let text = resp.text().await.unwrap_or_default();
            if text.contains("ABORTED")
                || text.contains("FAILED_PRECONDITION")
                || text.contains("ALREADY_EXISTS")
            {
                return Ok(false);
            }
        }
        Err(Error::Database(format!(
            "firestore PATCH failed: {}",
            resp.status()
        )))
    }

    /// DELETE with an `updateTime` precondition. No-ops on precondition failure
    /// (the doc was already modified — we must not delete the new holder's record).
    async fn delete_conditional(
        &self,
        scope: &LeaseScope,
        token: &str,
        update_time: &str,
    ) -> Result<()> {
        let url = format!("{}{}", self.base_url(), self.doc_path(scope));
        let resp = self
            .client
            .delete(&url)
            .bearer_auth_if_set(token)
            .query(&[("currentDocument.updateTime", update_time)])
            .send()
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        if resp.status().is_success() || resp.status() == reqwest::StatusCode::NOT_FOUND {
            return Ok(());
        }
        if matches!(
            resp.status(),
            reqwest::StatusCode::CONFLICT | reqwest::StatusCode::BAD_REQUEST
        ) {
            let text = resp.text().await.unwrap_or_default();
            if text.contains("ABORTED") || text.contains("FAILED_PRECONDITION") {
                // Doc was modified after our GET — no-op is correct.
                return Ok(());
            }
        }
        Err(Error::Database(format!(
            "firestore DELETE failed: {}",
            resp.status()
        )))
    }
```

- [ ] **Step 4: Run the compile test**

```bash
cargo test -p runway-storage --lib remote::lease::tests
```
Expected: 1 test passes (`precondition_variant_coverage`).

- [ ] **Step 5: Build the full crate**

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
git -C (reflective-root)/runtime-runway add crates/runway-storage/src/remote/lease.rs
git -C (reflective-root)/runtime-runway commit -m "$(cat <<'EOF'
refactor(d5): Firestore lease CAS helpers (read_current, patch_conditional, delete_conditional)

Private helpers that will replace the non-atomic GET→PATCH sequences in
try_acquire/renew/release. No public-API change; existing behaviour unchanged
until Task 2 rewires the methods to use them.

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
EOF
)"
```

---

## Task 2: Rewrite `try_acquire`, `renew`, and `release` to use CAS

Replaces the three GET→PATCH/DELETE bodies with calls to the helpers from Task 1. The external contract is identical; the race is closed.

**Files:**
- Modify: `crates/runway-storage/src/remote/lease.rs`

**Interfaces:**
- Consumes: `read_current`, `patch_conditional`, `delete_conditional`, `Precondition` from Task 1.
- Public signatures of `try_acquire`, `renew`, `release`, `current` are unchanged.

- [ ] **Step 1: Confirm the local contract suite passes before touching anything**

```bash
cargo test -p runway-storage --test contract_local -- --nocapture
```
Expected: all 12 lease contract tests + all other contract tests pass.

- [ ] **Step 2: Replace `try_acquire`**

Find and replace the entire `async fn try_acquire` body inside the `#[async_trait] impl LeaseStore for FirestoreLeaseStore` block. The new body is:

```rust
    async fn try_acquire(
        &self,
        scope: &LeaseScope,
        holder_id: &str,
        ttl: Duration,
    ) -> Result<AcquireOutcome> {
        let token = self
            .token
            .get()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        let now = Utc::now();
        let new_expires = now
            + chrono::Duration::from_std(ttl).map_err(|e| Error::Other(e.to_string()))?;
        let new_rec = LeaseRecord {
            holder_id: holder_id.into(),
            expires_at: new_expires,
        };

        match self.read_current(scope, &token).await? {
            None => {
                // Atomic create: precondition ensures only one winner when two
                // instances race on an absent scope.
                if self
                    .patch_conditional(scope, &token, &new_rec, Precondition::MustNotExist)
                    .await?
                {
                    Ok(AcquireOutcome::Acquired(new_rec))
                } else {
                    // Lost the race — re-read who won and report them as the holder.
                    match self.read_current(scope, &token).await? {
                        Some((current, _)) => Ok(AcquireOutcome::HeldByOther(current)),
                        None => Err(Error::Database(
                            "lease vanished immediately after concurrent create".into(),
                        )),
                    }
                }
            }
            Some((ref existing, ref etag)) if existing.expires_at <= now => {
                // Steal expired lease: CAS on updateTime prevents two stealers both winning.
                if self
                    .patch_conditional(
                        scope,
                        &token,
                        &new_rec,
                        Precondition::MustHaveUpdateTime(etag.clone()),
                    )
                    .await?
                {
                    Ok(AcquireOutcome::Acquired(new_rec))
                } else {
                    match self.read_current(scope, &token).await? {
                        Some((current, _)) => Ok(AcquireOutcome::HeldByOther(current)),
                        None => Err(Error::Database(
                            "lease vanished immediately after concurrent steal".into(),
                        )),
                    }
                }
            }
            Some((ref existing, ref etag)) if existing.holder_id == holder_id => {
                // Idempotent re-acquire: extend our own TTL atomically.
                if self
                    .patch_conditional(
                        scope,
                        &token,
                        &new_rec,
                        Precondition::MustHaveUpdateTime(etag.clone()),
                    )
                    .await?
                {
                    Ok(AcquireOutcome::Acquired(new_rec))
                } else {
                    // Another instance wrote between our GET and PATCH.
                    match self.read_current(scope, &token).await? {
                        Some((current, _)) if current.holder_id == holder_id => {
                            // A concurrent renewal by us won; still acquired.
                            Ok(AcquireOutcome::Acquired(current))
                        }
                        Some((current, _)) => Ok(AcquireOutcome::HeldByOther(current)),
                        None => Err(Error::Database(
                            "lease vanished immediately after concurrent re-acquire".into(),
                        )),
                    }
                }
            }
            Some((existing, _)) => Ok(AcquireOutcome::HeldByOther(existing)),
        }
    }
```

- [ ] **Step 3: Replace `renew`**

Find and replace the entire `async fn renew` body:

```rust
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
        let new_expires = now
            + chrono::Duration::from_std(ttl).map_err(|e| Error::Other(e.to_string()))?;

        match self.read_current(scope, &token).await? {
            Some((ref rec, ref etag))
                if rec.holder_id == holder_id && rec.expires_at > now =>
            {
                let renewed = LeaseRecord {
                    holder_id: holder_id.into(),
                    expires_at: new_expires,
                };
                if self
                    .patch_conditional(
                        scope,
                        &token,
                        &renewed,
                        Precondition::MustHaveUpdateTime(etag.clone()),
                    )
                    .await?
                {
                    Ok(RenewOutcome::Renewed(renewed))
                } else {
                    // Stolen between our GET and PATCH.
                    let current = self
                        .read_current(scope, &token)
                        .await?
                        .map(|(rec, _)| rec);
                    Ok(RenewOutcome::Lost { current })
                }
            }
            other => Ok(RenewOutcome::Lost {
                current: other.map(|(rec, _)| rec),
            }),
        }
    }
```

- [ ] **Step 4: Replace `release`**

Find and replace the entire `async fn release` body:

```rust
    async fn release(&self, scope: &LeaseScope, holder_id: &str) -> Result<()> {
        let token = self
            .token
            .get()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        match self.read_current(scope, &token).await? {
            None => Ok(()),
            Some((rec, _)) if rec.holder_id != holder_id => Ok(()),
            Some((_, etag)) => self.delete_conditional(scope, &token, &etag).await,
        }
    }
```

- [ ] **Step 5: Simplify `current` to use `read_current`**

The existing `current` implementation duplicates the GET + decode logic. Replace it:

```rust
    async fn current(&self, scope: &LeaseScope) -> Result<Option<LeaseRecord>> {
        let token = self
            .token
            .get()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(self.read_current(scope, &token).await?.map(|(rec, _)| rec))
    }
```

- [ ] **Step 6: Build**

```bash
cargo build -p runway-storage
```
Expected: clean.

- [ ] **Step 7: Run local contract suite**

```bash
cargo test -p runway-storage --test contract_local -- --nocapture
```
Expected: all tests still pass (contract unchanged; only implementation internals changed).

- [ ] **Step 8: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 9: Commit**

```bash
git -C (reflective-root)/runtime-runway add crates/runway-storage/src/remote/lease.rs
git -C (reflective-root)/runtime-runway commit -m "$(cat <<'EOF'
fix(d5): FirestoreLeaseStore — atomic CAS via Firestore precondition writes

Replaces the non-atomic GET→decide→PATCH sequence with conditional writes:
- try_acquire on absent scope: PATCH with ?currentDocument.exists=false
- try_acquire steal/re-acquire: PATCH with ?currentDocument.updateTime=<etag>
- renew: PATCH with ?currentDocument.updateTime=<etag>
- release: DELETE with ?currentDocument.updateTime=<etag>

On precondition failure (409 ABORTED / 400 FAILED_PRECONDITION) we re-read
and return HeldByOther/Lost rather than retrying. The middleware's 409
response handles the retry at the HTTP level.

Closes the TOCTOU race where two Cloud Run instances could both believe they
held the same session lease simultaneously. Unblocks --max-instances > 1.

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
EOF
)"
```

---

## Task 3: Concurrent mutual-exclusion test against the Firestore emulator

Adds a test that demonstrates the invariant the CAS fix enforces: N concurrent `try_acquire` callers on the same empty scope produce exactly one `Acquired`. The test is env-gated; it runs automatically in CI when `FIRESTORE_EMULATOR_HOST` is set.

**Files:**
- Create: `crates/runway-storage/tests/firestore_lease_concurrent_test.rs`
- Modify: `crates/runway-storage/Cargo.toml` — add `[[test]]` entry if needed (check first).

**Interfaces:**
- Consumes: `FirestoreLeaseStore` (through `RemoteStorageKit`), `LeaseScope`, `AcquireOutcome`, all from `runway-storage`.

- [ ] **Step 1: Check whether a `[[test]]` entry is needed**

```bash
grep -n 'firestore_lease\|\[\[test\]\]' (reflective-root)/runtime-runway/crates/runway-storage/Cargo.toml
```
If no `[[test]]` sections exist (Cargo discovers test files in `tests/` automatically), no Cargo.toml change is needed. If the file uses an explicit list, add the entry.

- [ ] **Step 2: Create the test file**

Create `crates/runway-storage/tests/firestore_lease_concurrent_test.rs`:

```rust
//! Concurrent mutual-exclusion test for FirestoreLeaseStore.
//!
//! Requires a running Firestore emulator:
//!   gcloud emulators firestore start --host-port=localhost:8080
//!   export FIRESTORE_EMULATOR_HOST=localhost:8080
//!
//! Run with:
//!   cargo test -p runway-storage --test firestore_lease_concurrent_test -- --nocapture

use std::{sync::Arc, time::Duration};

use runway_storage::{
    AcquireOutcome, LeaseScope,
    remote::{RemoteConfig, RemoteStorageKit, TokenSource},
};
use uuid::Uuid;

fn emulator_config() -> RemoteConfig {
    RemoteConfig {
        project_id: "cas-test".into(),
        region: "europe-west1".into(),
        bucket: "cas-test-bucket".into(),
        token_source: TokenSource::Static(String::new()),
    }
}

/// N independent RemoteStorageKit instances all race to acquire the same empty scope.
/// Exactly one must win; all others must see HeldByOther.
///
/// With the pre-CAS implementation this test is flaky (sometimes >1 wins).
/// With CAS precondition writes it always passes.
#[tokio::test]
async fn concurrent_acquire_on_empty_scope_has_exactly_one_winner() {
    if std::env::var("FIRESTORE_EMULATOR_HOST").is_err() {
        eprintln!("skipping: FIRESTORE_EMULATOR_HOST not set");
        return;
    }

    const CONCURRENCY: usize = 10;

    // Unique scope per run — emulator persists state across test runs.
    let scope = LeaseScope {
        org_id: "org-cas".into(),
        app_id: "test".into(),
        session_id: Uuid::new_v4().to_string(),
    };

    // Build CONCURRENCY independent kit instances (each simulates a separate Cloud Run instance).
    let mut kits = Vec::with_capacity(CONCURRENCY);
    for _ in 0..CONCURRENCY {
        let kit = RemoteStorageKit::build(emulator_config())
            .await
            .expect("kit");
        kits.push(Arc::new(kit));
    }

    // Fan out: all CONCURRENCY instances call try_acquire simultaneously.
    let scope_arc = Arc::new(scope.clone());
    let handles: Vec<_> = kits
        .iter()
        .enumerate()
        .map(|(i, kit)| {
            let leases = kit.leases.clone();
            let scope = scope_arc.clone();
            let holder = format!("holder-{i}");
            tokio::spawn(async move {
                leases
                    .try_acquire(&scope, &holder, Duration::from_secs(30))
                    .await
            })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|join| join.expect("task panicked").expect("lease store error"))
        .collect();

    let acquired: Vec<_> = results
        .iter()
        .filter(|r| matches!(r, AcquireOutcome::Acquired(_)))
        .collect();
    let held_by_other: Vec<_> = results
        .iter()
        .filter(|r| matches!(r, AcquireOutcome::HeldByOther(_)))
        .collect();

    assert_eq!(
        acquired.len(),
        1,
        "exactly one holder must win; got {} Acquired and {} HeldByOther",
        acquired.len(),
        held_by_other.len()
    );
    assert_eq!(
        held_by_other.len(),
        CONCURRENCY - 1,
        "all other callers must see HeldByOther"
    );
}

/// Same as above but for a scope that has an expired lease — concurrent steal
/// must also produce exactly one winner.
#[tokio::test]
async fn concurrent_steal_of_expired_scope_has_exactly_one_winner() {
    if std::env::var("FIRESTORE_EMULATOR_HOST").is_err() {
        eprintln!("skipping: FIRESTORE_EMULATOR_HOST not set");
        return;
    }

    const CONCURRENCY: usize = 10;

    let scope = LeaseScope {
        org_id: "org-cas".into(),
        app_id: "test".into(),
        session_id: Uuid::new_v4().to_string(),
    };

    // Seed an already-expired record.
    let seeder = RemoteStorageKit::build(emulator_config()).await.expect("seeder");
    seeder
        .leases
        .try_acquire(&scope, "seed-holder", Duration::from_millis(1))
        .await
        .expect("seed");
    tokio::time::sleep(Duration::from_millis(50)).await; // let it expire

    let mut kits = Vec::with_capacity(CONCURRENCY);
    for _ in 0..CONCURRENCY {
        kits.push(Arc::new(
            RemoteStorageKit::build(emulator_config()).await.expect("kit"),
        ));
    }

    let scope_arc = Arc::new(scope);
    let handles: Vec<_> = kits
        .iter()
        .enumerate()
        .map(|(i, kit)| {
            let leases = kit.leases.clone();
            let scope = scope_arc.clone();
            let holder = format!("stealer-{i}");
            tokio::spawn(async move {
                leases
                    .try_acquire(&scope, &holder, Duration::from_secs(30))
                    .await
            })
        })
        .collect();

    let results: Vec<_> = futures::future::join_all(handles)
        .await
        .into_iter()
        .map(|join| join.expect("task panicked").expect("lease store error"))
        .collect();

    let acquired_count = results
        .iter()
        .filter(|r| matches!(r, AcquireOutcome::Acquired(_)))
        .count();

    assert_eq!(
        acquired_count, 1,
        "exactly one stealer must win; got {acquired_count}"
    );
}
```

- [ ] **Step 3: Check that `futures` is available as a dev-dependency**

```bash
grep 'futures' (reflective-root)/runtime-runway/crates/runway-storage/Cargo.toml
```
If absent, add to `[dev-dependencies]`:
```toml
futures = { workspace = true }
```
Check `Cargo.toml` at workspace root to confirm `futures` is in `[workspace.dependencies]`. If it's not there either, add `futures = "0.3"` to the workspace and reference it with `{ workspace = true }` in the crate.

- [ ] **Step 4: Confirm the test compiles (skips cleanly without the emulator)**

```bash
cargo test -p runway-storage --test firestore_lease_concurrent_test -- --nocapture
```
Expected output includes `skipping: FIRESTORE_EMULATOR_HOST not set` for each test; exit 0.

- [ ] **Step 5: Run the Firestore emulator and confirm the tests pass (if emulator is available)**

If `gcloud` and the emulator are installed:
```bash
# In a separate terminal:
gcloud emulators firestore start --host-port=localhost:8080

# In this terminal:
FIRESTORE_EMULATOR_HOST=localhost:8080 \
  cargo test -p runway-storage --test firestore_lease_concurrent_test -- --nocapture
```
Expected: both tests pass.

Also re-run the existing emulator-gated ownership test to confirm the fix didn't break it:
```bash
FIRESTORE_EMULATOR_HOST=localhost:8080 \
  cargo test -p runway-app-host --test ownership_test variant2 -- --nocapture
```
Expected: passes.

- [ ] **Step 6: Run lint**

```bash
just lint
```
Expected: clean.

- [ ] **Step 7: Commit**

```bash
git -C (reflective-root)/runtime-runway add \
    crates/runway-storage/tests/firestore_lease_concurrent_test.rs \
    crates/runway-storage/Cargo.toml
git -C (reflective-root)/runtime-runway commit -m "$(cat <<'EOF'
test(d5): concurrent mutual-exclusion tests for FirestoreLeaseStore CAS

Two emulator-gated tests (FIRESTORE_EMULATOR_HOST required):
- 10 concurrent try_acquire on empty scope → exactly 1 Acquired
- 10 concurrent steal of expired scope → exactly 1 Acquired

These tests document the invariant the CAS fix enforces and would be
flaky against the pre-CAS GET→PATCH implementation.

Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>
EOF
)"
```

---

## Self-Review

**1. Spec coverage:**
- TOCTOU on `try_acquire` (new + steal + re-acquire): covered by Task 2 Step 2.
- TOCTOU on `renew`: covered by Task 2 Step 3.
- TOCTOU on `release` (old holder deletes new holder's record): covered by Task 2 Step 4.
- `current` cleanup: Task 2 Step 5 (no behaviour change, removes duplication).
- Test demonstrating the race is closed: Task 3.

**2. Placeholder scan:** None found.

**3. Type consistency:**
- `Precondition` defined in Task 1, used identically in Task 2. ✓
- `read_current` signature defined in Task 1 Step 3, consumed in Task 2 Steps 2–4. ✓
- `patch_conditional` and `delete_conditional` defined in Task 1, consumed in Task 2. ✓
- `AcquireOutcome`, `RenewOutcome`, `LeaseRecord`, `LeaseScope` unchanged throughout. ✓
