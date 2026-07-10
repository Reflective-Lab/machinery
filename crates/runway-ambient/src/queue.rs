use std::sync::Arc;

use chrono::Utc;
use runway_app_host::JobKey;
use runway_storage::traits::document::{Document, Filter, Order, Query};
use runway_storage::traits::event::StoredEvent;
use runway_storage::{DocumentStore, EventLog};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::error::{AmbientError, Result};
use crate::job::{AmbientJobPhase, AmbientJobRecord, AmbientJobRequest, AmbientJobStatus};

pub const JOBS_COLLECTION: &str = "_runway_ambient_jobs";

/// Durable ambient job queue backed by `DocumentStore` + provenance on `EventLog`.
#[derive(Clone)]
pub struct AmbientJobQueue {
    documents: Arc<dyn DocumentStore>,
    events: Arc<dyn EventLog>,
    max_attempts: u32,
}

impl AmbientJobQueue {
    pub fn new(
        documents: Arc<dyn DocumentStore>,
        events: Arc<dyn EventLog>,
        max_attempts: u32,
    ) -> Self {
        Self {
            documents,
            events,
            max_attempts,
        }
    }

    pub async fn enqueue(&self, request: AmbientJobRequest) -> Result<String> {
        if request.phase == AmbientJobPhase::CommitBound && request.commit_ref.is_none() {
            return Err(AmbientError::MissingCommitRef);
        }

        let job_id = Uuid::new_v4().to_string();
        let record = AmbientJobRecord::from_request(request, job_id.clone());
        self.put_record(&record).await?;
        self.append_event(
            &record,
            "ambient.job.enqueued",
            json!({ "job_key": record.job_key, "phase": record.phase }),
        )
        .await?;
        Ok(job_id)
    }

    pub async fn get(&self, job_id: &str) -> Result<Option<AmbientJobRecord>> {
        match self.documents.get(JOBS_COLLECTION, job_id).await {
            Ok(Some(doc)) => record_from_document(doc).map(Some),
            Ok(None) => Ok(None),
            Err(e) => Err(AmbientError::from(e)),
        }
    }

    /// Claim the oldest pending job matching one of `job_keys` for `app_id`.
    pub async fn claim_next(
        &self,
        worker_id: &str,
        app_id: &str,
        job_keys: &[JobKey],
    ) -> Result<Option<AmbientJobRecord>> {
        if job_keys.is_empty() {
            return Ok(None);
        }

        let pending = self
            .documents
            .query(
                JOBS_COLLECTION,
                Query::new()
                    .filter(Filter::Eq(
                        "status".into(),
                        serde_json::to_value(AmbientJobStatus::Pending)
                            .unwrap_or(Value::String("pending".into())),
                    ))
                    .order("enqueued_at", Order::Asc),
            )
            .await?;

        let mut candidates: Vec<AmbientJobRecord> = pending
            .into_iter()
            .filter_map(|doc| record_from_document(doc).ok())
            .filter(|r| r.app_id == app_id && job_keys.contains(&r.job_key))
            .collect();

        candidates.sort_by_key(|a| a.enqueued_at);

        for candidate in candidates {
            let Some(mut current) = self.get(&candidate.job_id).await? else {
                continue;
            };
            if current.status != AmbientJobStatus::Pending {
                continue;
            }
            current.status = AmbientJobStatus::Running;
            current.worker_id = Some(worker_id.to_string());
            current.attempts = current.attempts.saturating_add(1);
            current.updated_at = Utc::now();
            self.put_record(&current).await?;
            self.append_event(
                &current,
                "ambient.job.started",
                json!({
                    "worker_id": worker_id,
                    "attempt": current.attempts,
                }),
            )
            .await?;
            return Ok(Some(current));
        }

        Ok(None)
    }

    pub async fn complete(&self, job_id: &str, worker_id: &str, result: Value) -> Result<()> {
        let mut record = self
            .get(job_id)
            .await?
            .ok_or_else(|| AmbientError::NotFound(job_id.to_string()))?;
        if record.worker_id.as_deref() != Some(worker_id) {
            return Err(AmbientError::HeldByOther {
                job_id: job_id.to_string(),
            });
        }
        record.status = AmbientJobStatus::Succeeded;
        record.result = Some(result.clone());
        record.updated_at = Utc::now();
        self.put_record(&record).await?;
        self.append_event(
            &record,
            "ambient.job.completed",
            json!({ "result": result }),
        )
        .await?;
        Ok(())
    }

    pub async fn fail(
        &self,
        job_id: &str,
        worker_id: &str,
        error: &str,
        retryable: bool,
    ) -> Result<()> {
        let mut record = self
            .get(job_id)
            .await?
            .ok_or_else(|| AmbientError::NotFound(job_id.to_string()))?;
        if record.worker_id.as_deref() != Some(worker_id) {
            return Err(AmbientError::HeldByOther {
                job_id: job_id.to_string(),
            });
        }
        record.last_error = Some(error.to_string());
        record.worker_id = None;
        record.updated_at = Utc::now();

        let retry = retryable && record.attempts < self.max_attempts;
        record.status = if retry {
            AmbientJobStatus::Pending
        } else {
            AmbientJobStatus::DeadLetter
        };
        self.put_record(&record).await?;
        self.append_event(
            &record,
            "ambient.job.failed",
            json!({
                "error": error,
                "retryable": retryable,
                "will_retry": retry,
                "attempts": record.attempts,
            }),
        )
        .await?;
        Ok(())
    }

    async fn put_record(&self, record: &AmbientJobRecord) -> Result<()> {
        let doc = Document::new(&record.job_id, record).map_err(AmbientError::codec)?;
        self.documents
            .put(JOBS_COLLECTION, doc)
            .await
            .map_err(AmbientError::from)
    }

    async fn append_event(
        &self,
        record: &AmbientJobRecord,
        event_type: &str,
        payload: Value,
    ) -> Result<()> {
        let event = StoredEvent {
            event_id: Uuid::new_v4().to_string(),
            org_id: record.org_id.clone(),
            app_id: record.app_id.clone(),
            event_type: event_type.to_string(),
            context_id: Some(record.job_id.clone()),
            fact_id: record.commit_ref.clone(),
            payload,
            occurred_at: Utc::now(),
            synced_at: None,
        };
        self.events.append(event).await.map_err(AmbientError::from)
    }
}

fn record_from_document(doc: Document) -> Result<AmbientJobRecord> {
    let value = serde_json::to_value(doc.data).map_err(AmbientError::codec)?;
    serde_json::from_value(value).map_err(AmbientError::codec)
}

#[cfg(test)]
mod tests {
    use super::*;
    use runway_storage::StorageKit;
    use serde_json::json;

    async fn test_queue() -> AmbientJobQueue {
        let kit = StorageKit::local(tempfile::tempdir().unwrap().path())
            .await
            .unwrap();
        AmbientJobQueue::new(kit.documents, kit.events, 3)
    }

    #[tokio::test]
    async fn enqueue_preparatory_without_commit_ref() {
        let q = test_queue().await;
        let id = q
            .enqueue(AmbientJobRequest::preparatory(
                "sensemap-refresh".parse().unwrap(),
                "org-1",
                "test-app",
                json!({ "scope": "inquiry-1" }),
            ))
            .await
            .unwrap();
        let record = q.get(&id).await.unwrap().unwrap();
        assert_eq!(record.status, AmbientJobStatus::Pending);
        assert_eq!(record.phase, AmbientJobPhase::Preparatory);
    }

    #[tokio::test]
    async fn commit_bound_requires_commit_ref() {
        let q = test_queue().await;
        let err = q
            .enqueue(AmbientJobRequest {
                job_key: "apply-decision".parse().unwrap(),
                org_id: "org-1".into(),
                app_id: "test-app".into(),
                correlation_id: None,
                commit_ref: None,
                phase: AmbientJobPhase::CommitBound,
                payload: json!({}),
            })
            .await
            .unwrap_err();
        assert!(matches!(err, AmbientError::MissingCommitRef));
    }

    #[tokio::test]
    async fn claim_complete_lifecycle() {
        let q = test_queue().await;
        let id = q
            .enqueue(AmbientJobRequest::preparatory(
                "mnemos-recall".parse().unwrap(),
                "org-1",
                "test-app",
                json!({}),
            ))
            .await
            .unwrap();

        let claimed = q
            .claim_next("worker-1", "test-app", &["mnemos-recall".parse().unwrap()])
            .await
            .unwrap()
            .unwrap();
        assert_eq!(claimed.job_id, id);
        assert_eq!(claimed.status, AmbientJobStatus::Running);

        q.complete(&id, "worker-1", json!({ "ok": true }))
            .await
            .unwrap();
        let done = q.get(&id).await.unwrap().unwrap();
        assert_eq!(done.status, AmbientJobStatus::Succeeded);
    }

    #[tokio::test]
    async fn fail_retries_then_dead_letters() {
        let q = test_queue().await;
        let id = q
            .enqueue(AmbientJobRequest::preparatory(
                "drift-scan".parse().unwrap(),
                "org-1",
                "test-app",
                json!({}),
            ))
            .await
            .unwrap();

        for _ in 0..3 {
            let job = q
                .claim_next("w", "test-app", &["drift-scan".parse().unwrap()])
                .await
                .unwrap()
                .unwrap();
            q.fail(&job.job_id, "w", "transient", true).await.unwrap();
        }

        let dead = q.get(&id).await.unwrap().unwrap();
        assert_eq!(dead.status, AmbientJobStatus::DeadLetter);
    }

    #[tokio::test]
    async fn enqueued_event_written_to_log() {
        let kit = StorageKit::local(tempfile::tempdir().unwrap().path())
            .await
            .unwrap();
        let q = AmbientJobQueue::new(kit.documents.clone(), kit.events.clone(), 3);
        let _ = q
            .enqueue(AmbientJobRequest::preparatory(
                "x".parse().unwrap(),
                "org-1",
                "app",
                json!({}),
            ))
            .await
            .unwrap();

        let events = kit
            .events
            .query(runway_storage::EventQuery {
                event_type: Some("ambient.job.enqueued".into()),
                ..Default::default()
            })
            .await
            .unwrap();
        assert_eq!(events.len(), 1);
    }
}
