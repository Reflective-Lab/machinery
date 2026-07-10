use std::sync::Arc;

use tokio::task::JoinHandle;
use tracing::{error, info, warn};

use crate::config::AmbientConfig;
use crate::handler::AmbientJobHandler;
use crate::queue::AmbientJobQueue;

/// Polls the durable queue and dispatches to a registered handler.
pub struct AmbientWorker {
    queue: AmbientJobQueue,
    handler: Arc<dyn AmbientJobHandler>,
    app_id: String,
    worker_id: String,
    config: AmbientConfig,
}

impl AmbientWorker {
    pub fn new(
        queue: AmbientJobQueue,
        handler: Arc<dyn AmbientJobHandler>,
        app_id: impl Into<String>,
        worker_id: impl Into<String>,
        config: AmbientConfig,
    ) -> Self {
        Self {
            queue,
            handler,
            app_id: app_id.into(),
            worker_id: worker_id.into(),
            config,
        }
    }

    /// Background poll loop. Returns immediately if `config.enabled` is false.
    pub fn spawn(self) -> Option<JoinHandle<()>> {
        if !self.config.enabled {
            info!("ambient worker disabled (RUNWAY_AMBIENT_ENABLED != true)");
            return None;
        }

        let poll = self.config.poll_interval;
        Some(tokio::spawn(async move {
            info!(
                worker_id = %self.worker_id,
                app_id = %self.app_id,
                job_keys = ?self.handler.job_keys(),
                "ambient worker started"
            );
            loop {
                match self
                    .queue
                    .claim_next(&self.worker_id, &self.app_id, self.handler.job_keys())
                    .await
                {
                    Ok(Some(job)) => {
                        let job_id = job.job_id.clone();
                        match self.handler.execute(&job).await {
                            Ok(result) => {
                                if let Err(e) =
                                    self.queue.complete(&job_id, &self.worker_id, result).await
                                {
                                    error!(job_id, error = %e, "ambient complete failed");
                                }
                            }
                            Err(e) => {
                                warn!(job_id, error = %e, "ambient job handler failed");
                                let retryable = e.is_retryable();
                                if let Err(fail_err) = self
                                    .queue
                                    .fail(&job_id, &self.worker_id, &e.to_string(), retryable)
                                    .await
                                {
                                    error!(job_id, error = %fail_err, "ambient fail recording failed");
                                }
                            }
                        }
                    }
                    Ok(None) => {
                        tokio::time::sleep(poll).await;
                    }
                    Err(e) => {
                        error!(error = %e, "ambient claim failed");
                        tokio::time::sleep(poll).await;
                    }
                }
            }
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::job::AmbientJobRequest;
    use crate::queue::AmbientJobQueue;
    use async_trait::async_trait;
    use runway_app_host::JobKey;
    use runway_storage::StorageKit;
    use serde_json::json;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::time::Duration;

    struct CountingHandler {
        keys: Vec<JobKey>,
        calls: Arc<AtomicUsize>,
    }

    #[async_trait]
    impl AmbientJobHandler for CountingHandler {
        fn job_keys(&self) -> &[JobKey] {
            &self.keys
        }

        async fn execute(
            &self,
            _job: &crate::job::AmbientJobRecord,
        ) -> Result<serde_json::Value, crate::error::AmbientError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            Ok(json!({ "done": true }))
        }
    }

    #[tokio::test]
    async fn worker_processes_enqueued_job() {
        let kit = StorageKit::local(tempfile::tempdir().unwrap().path())
            .await
            .unwrap();
        let queue = AmbientJobQueue::new(kit.documents, kit.events, 3);
        let calls = Arc::new(AtomicUsize::new(0));
        let handler = Arc::new(CountingHandler {
            keys: vec!["test-job".parse().unwrap()],
            calls: calls.clone(),
        });

        queue
            .enqueue(AmbientJobRequest::preparatory(
                "test-job".parse().unwrap(),
                "org-1",
                "my-app",
                json!({}),
            ))
            .await
            .unwrap();

        let worker = AmbientWorker::new(
            queue.clone(),
            handler,
            "my-app",
            "worker-test",
            AmbientConfig {
                enabled: true,
                poll_interval: Duration::from_millis(20),
                max_attempts: 3,
                pubsub_topic: None,
            },
        );
        let handle = worker.spawn().unwrap();

        tokio::time::timeout(Duration::from_secs(2), async {
            while calls.load(Ordering::SeqCst) == 0 {
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
        })
        .await
        .expect("worker should process job");

        handle.abort();
    }
}
