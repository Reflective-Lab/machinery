//! RP-LAYERING glue: plugs the ambient substrate into the publishable
//! `runway-app-host` through its `JobsRuntime` seam. Lives here — not in
//! app-host — so the publishable host never depends on this crate.

use std::collections::BTreeSet;
use std::sync::Arc;

use runway_app_host::{JobKey, JobsRuntime, MountedJobs};
use runway_storage::StorageKit;

use crate::{AmbientJobHandler, bootstrap, routes};

/// The ambient jobs runtime for `RunwayAppHostBuilder::with_jobs_runtime`.
/// Mounts `POST/GET /v1/ambient/jobs` when the packet declares jobs; when a
/// handler is set and `RUNWAY_AMBIENT_ENABLED=true`, spawns the background
/// worker for the packet's job keys.
pub struct AmbientJobs {
    handler: Option<Arc<dyn AmbientJobHandler>>,
}

impl AmbientJobs {
    /// Queue + HTTP surface only — jobs can be enqueued and inspected, but no
    /// worker runs in this process. `mount` skips the handler-coverage check;
    /// the deployment is responsible for running a covering worker elsewhere.
    pub fn new() -> Self {
        Self { handler: None }
    }

    /// Queue + HTTP surface + background worker driven by `handler`.
    pub fn with_handler(handler: Arc<dyn AmbientJobHandler>) -> Self {
        Self {
            handler: Some(handler),
        }
    }
}

impl Default for AmbientJobs {
    fn default() -> Self {
        Self::new()
    }
}

impl JobsRuntime for AmbientJobs {
    fn mount(
        self: Box<Self>,
        storage: &StorageKit,
        app_id: String,
        registered_job_keys: Vec<JobKey>,
    ) -> anyhow::Result<MountedJobs> {
        // Fail fast on registration/handler drift. The enqueue surface accepts
        // every registered key, but the worker claims only handler keys — a
        // registered key without a handler arm would be accepted and then sit
        // unclaimed forever. Refusing to start turns that silent stall into a
        // deploy-time error.
        if let Some(handler) = &self.handler {
            let handled: BTreeSet<&JobKey> = handler.job_keys().iter().collect();
            let registered: BTreeSet<&JobKey> = registered_job_keys.iter().collect();

            let uncovered: Vec<&str> = registered
                .difference(&handled)
                .map(|k| k.as_str())
                .collect();
            anyhow::ensure!(
                uncovered.is_empty(),
                "packet registers ambient job keys the handler cannot execute: \
                 [{}] — enqueued jobs for these keys would never be claimed",
                uncovered.join(", ")
            );

            let unregistered: Vec<&str> = handled
                .difference(&registered)
                .map(|k| k.as_str())
                .collect();
            anyhow::ensure!(
                unregistered.is_empty(),
                "handler executes ambient job keys the packet does not register: \
                 [{}] — they could never be enqueued",
                unregistered.join(", ")
            );
        }

        let (_queue, state, worker) = bootstrap(storage, app_id, registered_job_keys, self.handler);
        Ok(MountedJobs {
            router: routes(state),
            worker,
        })
    }
}
