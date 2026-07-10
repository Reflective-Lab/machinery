//! Shared reqwest client for the remote storage backends.
//!
//! `reqwest::Client::new()` has NO request timeout: a server that accepts
//! the TCP connection and never answers blocks the caller forever. That is
//! exactly how the contract-emulator CI job hung for its entire history
//! (QF-2026-07-02-08 in the root repo's QUALITY_BACKLOG.md) — two Firestore
//! suites awaited responses that never came, and the runner killed the job
//! at the cap with no verdict. With deadlines, a wedged endpoint becomes a
//! named `Error::Network` carrying the URL instead of silence.

use std::time::Duration;

/// Deadline for establishing the TCP/TLS connection.
const CONNECT_TIMEOUT: Duration = Duration::from_secs(10);

/// Deadline for the whole request (connect + send + response body).
/// Generous enough for a slow GCS object round-trip; small enough that a
/// hung emulator fails the suite instead of the CI job timeout firing.
const REQUEST_TIMEOUT: Duration = Duration::from_secs(30);

/// Build the standard client for remote GCP/emulator backends.
///
/// This is the crate's single sanctioned construction site (the per-store
/// `Client::new()` calls it replaced were the unsanctioned ones). Tests
/// stub these backends at the emulator level per RP-HERMETIC-UNIT, so the
/// disallowed-methods escape hatch is deliberate and localized here.
#[allow(clippy::disallowed_methods)]
pub(crate) fn client() -> reqwest::Client {
    reqwest::Client::builder()
        .connect_timeout(CONNECT_TIMEOUT)
        .timeout(REQUEST_TIMEOUT)
        .build()
        .expect("reqwest client construction cannot fail with static config")
}
