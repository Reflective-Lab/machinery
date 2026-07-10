//! Event ledger traits — moved to `helm-event-substrate` (RFL-171).
//!
//! Re-exported here so existing `crate::traits::event::*` import paths
//! continue to resolve without source edits in runway-storage callers.

pub use helm_event_substrate::{EventLog, EventQuery, StoredEvent, SyncableEventLog};
