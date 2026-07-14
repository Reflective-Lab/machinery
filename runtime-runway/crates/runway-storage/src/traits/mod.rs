pub mod document;
pub mod embedding;
pub mod event;
pub mod lease;
pub mod object;
pub mod vector;

/// Canonical error type for storage contract implementations.
///
/// Re-exported from [`helm_event_substrate::SubstrateError`] (RFL-171).
/// Aliased here as `Error` so unmoved traits (documents/vectors/objects) keep
/// compiling without source edits.
pub use helm_event_substrate::{Result, SubstrateError as Error};
