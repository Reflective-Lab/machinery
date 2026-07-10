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
    let seeder = RemoteStorageKit::build(emulator_config())
        .await
        .expect("seeder");
    seeder
        .leases
        .try_acquire(&scope, "seed-holder", Duration::from_millis(1))
        .await
        .expect("seed");
    tokio::time::sleep(Duration::from_millis(50)).await; // let it expire

    let mut kits = Vec::with_capacity(CONCURRENCY);
    for _ in 0..CONCURRENCY {
        kits.push(Arc::new(
            RemoteStorageKit::build(emulator_config())
                .await
                .expect("kit"),
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
    let held_by_other_count = results
        .iter()
        .filter(|o| matches!(o, AcquireOutcome::HeldByOther(_)))
        .count();

    assert_eq!(acquired_count, 1, "exactly one racer should win the steal");
    assert_eq!(
        held_by_other_count,
        CONCURRENCY - 1,
        "all other racers must see HeldByOther"
    );
}
