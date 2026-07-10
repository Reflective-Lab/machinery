//! Validated ambient job key.

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Identifier for an ambient job declared in `runway.app.json` and executed by
/// the handler an app injects through [`crate::JobsRuntime`].
///
/// This key is the coupling point between three surfaces that must agree — the
/// app manifest, the enqueue endpoint, and the worker's dispatch table — so it
/// is a validated type rather than a bare `String`: lowercase kebab-case,
/// `[a-z0-9]+(-[a-z0-9]+)*`, e.g. `sensemap-refresh`.
///
/// Serializes as a plain JSON string (wire and persisted formats are
/// unchanged); deserialization validates, so a malformed key fails at manifest
/// parse instead of surfacing later as a job nothing can claim.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(try_from = "String", into = "String")]
pub struct JobKey(String);

impl JobKey {
    pub fn new(raw: impl Into<String>) -> Result<Self, InvalidJobKey> {
        let raw = raw.into();
        if is_kebab_case(&raw) {
            Ok(Self(raw))
        } else {
            Err(InvalidJobKey { offending: raw })
        }
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

fn is_kebab_case(s: &str) -> bool {
    !s.is_empty()
        && !s.starts_with('-')
        && !s.ends_with('-')
        && !s.contains("--")
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
}

impl fmt::Display for JobKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

impl AsRef<str> for JobKey {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<JobKey> for String {
    fn from(key: JobKey) -> Self {
        key.0
    }
}

impl TryFrom<String> for JobKey {
    type Error = InvalidJobKey;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::new(value)
    }
}

impl FromStr for JobKey {
    type Err = InvalidJobKey;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::new(s)
    }
}

/// Rejected [`JobKey`] input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InvalidJobKey {
    offending: String,
}

impl InvalidJobKey {
    /// The raw input that failed validation.
    pub fn offending(&self) -> &str {
        &self.offending
    }
}

impl fmt::Display for InvalidJobKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "invalid job key {:?}: expected lowercase kebab-case ([a-z0-9]+(-[a-z0-9]+)*)",
            self.offending
        )
    }
}

impl std::error::Error for InvalidJobKey {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_kebab_case_keys() {
        for ok in ["sensemap-refresh", "drift-scan", "a", "job2-v3"] {
            assert!(JobKey::new(ok).is_ok(), "{ok} should be valid");
        }
    }

    #[test]
    fn rejects_non_kebab_keys() {
        for bad in [
            "",
            "Sensemap-Refresh",
            "sensemap_refresh",
            "-leading",
            "trailing-",
            "double--dash",
            "spaced key",
        ] {
            assert!(JobKey::new(bad).is_err(), "{bad:?} should be rejected");
        }
    }

    #[test]
    fn serde_round_trips_as_plain_string() {
        let key: JobKey = serde_json::from_str("\"sensemap-refresh\"").unwrap();
        assert_eq!(key.as_str(), "sensemap-refresh");
        assert_eq!(serde_json::to_string(&key).unwrap(), "\"sensemap-refresh\"");
    }

    #[test]
    fn deserializing_invalid_key_fails() {
        let err = serde_json::from_str::<JobKey>("\"Not A Key\"").unwrap_err();
        assert!(err.to_string().contains("invalid job key"));
    }
}
