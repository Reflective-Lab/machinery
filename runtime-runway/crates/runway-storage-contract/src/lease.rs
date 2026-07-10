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

pub async fn run_lease_suite(store: Arc<dyn LeaseStore>, ctx: ContractContext) -> SuiteReport {
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
        contract_assert!(
            matches!(first, AcquireOutcome::Acquired(_)),
            "first acquired"
        );
        match second {
            AcquireOutcome::Acquired(rec) => {
                let prev = if let AcquireOutcome::Acquired(p) = first {
                    p
                } else {
                    unreachable!()
                };
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

    contract_test!(
        &report,
        "acquire_by_other_holder_returns_held_by_other",
        async {
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
        }
    );

    contract_test!(&report, "acquire_after_expiry_steals", async {
        let s = scope(&ctx, "case-4");
        let _ = store
            .try_acquire(&s, "h1", Duration::from_millis(100))
            .await
            .map_err(|e| e.to_string())?;
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
        store.release(&s, "h1").await.map_err(|e| e.to_string())?;
        let after = store.current(&s).await.map_err(|e| e.to_string())?;
        contract_assert!(
            after.is_none(),
            "expected None after release, got {:?}",
            after
        );
        Ok(())
    });

    contract_test!(&report, "release_by_non_holder_is_noop", async {
        let s = scope(&ctx, "case-9");
        let _ = store
            .try_acquire(&s, "h1", Duration::from_secs(30))
            .await
            .map_err(|e| e.to_string())?;
        store.release(&s, "h2").await.map_err(|e| e.to_string())?;
        let after = store.current(&s).await.map_err(|e| e.to_string())?;
        contract_assert!(after.is_some(), "h1's record must survive h2's release");
        store.release(&s, "h1").await.ok();
        Ok(())
    });

    contract_test!(&report, "release_of_absent_is_noop", async {
        let s = scope(&ctx, "case-10");
        store.release(&s, "h1").await.map_err(|e| e.to_string())?;
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
