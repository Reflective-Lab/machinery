use runway_app_host::JobKey;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AmbientError {
    #[error("job not found: {0}")]
    NotFound(String),
    #[error("commit-bound job requires commit_ref")]
    MissingCommitRef,
    /// Raw enqueue input that is not a packet-registered key. Carries `String`,
    /// not [`JobKey`]: the offending input may not even be a valid key.
    #[error("unregistered job_key: {0}")]
    UnregisteredJobKey(String),
    /// A registered key reached the handler without a dispatch arm. The
    /// startup coverage check in `AmbientJobs::mount` should make this
    /// unreachable; if it surfaces, registration and handler have drifted.
    #[error("no handler arm for registered job key: {0}")]
    UnhandledJobKey(JobKey),
    /// The job's payload does not parse into what the handler requires.
    /// Deterministic — never retried.
    #[error("invalid payload for job '{job_key}': {reason}")]
    InvalidPayload { job_key: JobKey, reason: String },
    #[error("job {job_id} held by another worker")]
    HeldByOther { job_id: String },
    #[error("storage error: {0}")]
    Storage(#[from] runway_storage::traits::Error),
    /// A job record failed to (de)serialize on its way to or from storage.
    #[error("job record codec error: {0}")]
    Codec(#[source] Box<dyn std::error::Error + Send + Sync>),
    #[error("pubsub publish failed: {0}")]
    Publish(String),
    /// App-side execution failure, source chain preserved.
    #[error("job execution failed: {0}")]
    Execution(#[source] Box<dyn std::error::Error + Send + Sync>),
}

impl AmbientError {
    /// Wrap an app-side execution failure without flattening it to a string.
    pub fn execution(err: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Self {
        Self::Execution(err.into())
    }

    /// Wrap a record (de)serialization failure without flattening it to a string.
    pub fn codec(err: impl Into<Box<dyn std::error::Error + Send + Sync>>) -> Self {
        Self::Codec(err.into())
    }

    /// Whether the worker should retry the job. Deterministic failures — a
    /// malformed payload, a missing handler arm, a commit-bound job without a
    /// commit ref — fail identically on every attempt and go straight to
    /// dead-letter.
    pub fn is_retryable(&self) -> bool {
        !matches!(
            self,
            Self::MissingCommitRef | Self::InvalidPayload { .. } | Self::UnhandledJobKey(_)
        )
    }
}

pub type Result<T> = std::result::Result<T, AmbientError>;
