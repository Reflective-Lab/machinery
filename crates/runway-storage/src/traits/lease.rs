//! Lease / session-ownership contracts — moved to `helm-event-substrate` (RFL-171).
//!
//! Re-exported here so existing `crate::traits::lease::*` import paths
//! continue to resolve without source edits in runway-storage callers.
//! Unit tests for the moved types live in `helm-event-substrate::lease`.

pub use helm_event_substrate::{AcquireOutcome, LeaseRecord, LeaseScope, LeaseStore, RenewOutcome};
