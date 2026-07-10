use std::{sync::Arc, time::Duration};

use async_trait::async_trait;
use chrono::Utc;
use redb::{Database, ReadableTable, TableDefinition, WriteTransaction};

use crate::traits::{
    Error, Result,
    lease::{AcquireOutcome, LeaseRecord, LeaseScope, LeaseStore, RenewOutcome},
};

// Table: scope_key (&str) → LeaseRecord-as-JSON (&str)
const LEASES: TableDefinition<&str, &str> = TableDefinition::new("leases");

pub fn init_tables(tx: &WriteTransaction) -> anyhow::Result<()> {
    tx.open_table(LEASES)?;
    Ok(())
}

fn read_record(
    table: &impl ReadableTable<&'static str, &'static str>,
    key: &str,
) -> Result<Option<LeaseRecord>> {
    match table.get(key).map_err(|e| Error::Database(e.to_string()))? {
        Some(guard) => Ok(Some(
            serde_json::from_str(guard.value()).map_err(|e| Error::Serialisation(e.to_string()))?,
        )),
        None => Ok(None),
    }
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
        let new_expires =
            now + chrono::Duration::from_std(ttl).map_err(|e| Error::Other(e.to_string()))?;

        tokio::task::spawn_blocking(move || {
            let tx = db
                .begin_write()
                .map_err(|e| Error::Database(e.to_string()))?;
            let outcome = {
                let mut table = tx
                    .open_table(LEASES)
                    .map_err(|e| Error::Database(e.to_string()))?;

                let existing = read_record(&table, key.as_str())?;

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
        let new_expires =
            now + chrono::Duration::from_std(ttl).map_err(|e| Error::Other(e.to_string()))?;

        tokio::task::spawn_blocking(move || {
            let tx = db
                .begin_write()
                .map_err(|e| Error::Database(e.to_string()))?;
            let outcome = {
                let mut table = tx
                    .open_table(LEASES)
                    .map_err(|e| Error::Database(e.to_string()))?;

                let existing = read_record(&table, key.as_str())?;

                match existing {
                    // An expired same-holder lease is intentionally Lost: force the holder
                    // back through try_acquire (steal path) rather than silently extending
                    // a lease whose hold may have lapsed.
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
            let tx = db
                .begin_write()
                .map_err(|e| Error::Database(e.to_string()))?;
            {
                let mut table = tx
                    .open_table(LEASES)
                    .map_err(|e| Error::Database(e.to_string()))?;

                let current = read_record(&table, key.as_str())?;

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
            let tx = db
                .begin_read()
                .map_err(|e| Error::Database(e.to_string()))?;
            let table = tx
                .open_table(LEASES)
                .map_err(|e| Error::Database(e.to_string()))?;

            read_record(&table, key.as_str())
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
