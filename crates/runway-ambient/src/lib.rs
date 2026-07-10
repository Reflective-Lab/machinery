mod config;
mod error;
mod handler;
mod http;
mod job;
mod jobs_runtime;
mod pubsub;
mod queue;
mod worker;

pub use config::AmbientConfig;
pub use error::{AmbientError, Result};
pub use handler::AmbientJobHandler;
pub use http::{AmbientState, routes};
pub use job::{AmbientJobPhase, AmbientJobRecord, AmbientJobRequest, AmbientJobStatus};
pub use jobs_runtime::AmbientJobs;
pub use pubsub::PubSubNotify;
pub use queue::{AmbientJobQueue, JOBS_COLLECTION};
pub use worker::AmbientWorker;

use std::sync::Arc;

use runway_app_host::JobKey;
use runway_storage::StorageKit;

/// Build queue + optional worker from a [`StorageKit`] and app packet job keys.
pub fn bootstrap(
    storage: &StorageKit,
    app_id: impl Into<String>,
    registered_job_keys: Vec<JobKey>,
    handler: Option<Arc<dyn AmbientJobHandler>>,
) -> (
    AmbientJobQueue,
    AmbientState,
    Option<tokio::task::JoinHandle<()>>,
) {
    let config = AmbientConfig::from_env();
    let app_id = app_id.into();
    let queue = AmbientJobQueue::new(
        storage.documents.clone(),
        storage.events.clone(),
        config.max_attempts,
    );
    let state = AmbientState {
        queue: queue.clone(),
        app_id: app_id.clone(),
        registered_job_keys: Arc::new(registered_job_keys),
        config: config.clone(),
    };
    let worker_handle = handler.and_then(|h| {
        AmbientWorker::new(
            queue.clone(),
            h,
            app_id,
            format!(
                "{}:{}",
                std::env::var("K_REVISION").unwrap_or_else(|_| "local".into()),
                uuid::Uuid::new_v4()
            ),
            config,
        )
        .spawn()
    });
    (queue, state, worker_handle)
}
