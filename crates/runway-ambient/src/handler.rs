use async_trait::async_trait;
use runway_app_host::JobKey;
use serde_json::Value;

use crate::error::AmbientError;
use crate::job::AmbientJobRecord;

/// App-provided executor for one or more registered [`JobKey`] values.
///
/// `job_keys()` must exactly match the packet-registered keys —
/// `AmbientJobs::mount` refuses to start on drift in either direction.
#[async_trait]
pub trait AmbientJobHandler: Send + Sync {
    fn job_keys(&self) -> &[JobKey];

    async fn execute(&self, job: &AmbientJobRecord) -> std::result::Result<Value, AmbientError>;
}
