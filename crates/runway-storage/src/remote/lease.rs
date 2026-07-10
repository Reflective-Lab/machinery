use std::time::Duration;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde_json::{Value, json};

use crate::remote::{BearerAuthExt, GcpToken};
use crate::traits::{
    Error, Result,
    lease::{AcquireOutcome, LeaseRecord, LeaseScope, LeaseStore, RenewOutcome},
};

/// Firestore document at `_runway_leases/{scope_key}` holds `LeaseRecord`.
const LEASE_COLLECTION: &str = "_runway_leases";

pub struct FirestoreLeaseStore {
    project_id: String,
    token: GcpToken,
    client: Client,
}

/// CAS precondition for conditional Firestore writes.
#[derive(Debug)]
enum Precondition {
    /// Document must not exist (atomic create).
    MustNotExist,
    /// Document's `updateTime` must equal this RFC3339 string (optimistic update).
    MustHaveUpdateTime(String),
}

impl FirestoreLeaseStore {
    pub fn new(project_id: String, token: GcpToken) -> Self {
        Self {
            project_id,
            token,
            // RP-HERMETIC-UNIT (Reflective QUALITY_BACKLOG.md →
            // QF-2026-06-02-05): production constructor for the Firestore
            // lease store; tests use the Firestore emulator at the test
            // harness level, not DI through this struct.
            #[allow(clippy::disallowed_methods)]
            client: crate::http::client(),
        }
    }

    /// Firestore-safe document ID for a scope.
    ///
    /// The raw key contains `|` and — under the contract suite's
    /// namespacing — `/`, which would silently turn the document path into
    /// a nested subcollection path. Percent-encoding the whole key yields a
    /// deterministic, reversible, slash-free ID, keeping the documented
    /// layout `_runway_leases/{scope_key}` regardless of key contents.
    fn doc_id(scope: &LeaseScope) -> String {
        urlencoding::encode(&scope.key()).into_owned()
    }

    /// Full Firestore resource name for a scope's lease document. Used in
    /// `:batchGet` / `:commit` request BODIES, where names are raw strings
    /// and no URI encoding applies.
    fn doc_name(&self, scope: &LeaseScope) -> String {
        format!(
            "projects/{}/databases/(default)/documents/{}/{}",
            self.project_id,
            LEASE_COLLECTION,
            Self::doc_id(scope)
        )
    }

    fn base_url(&self) -> String {
        // FIRESTORE_EMULATOR_HOST overrides for tests.
        match std::env::var("FIRESTORE_EMULATOR_HOST") {
            Ok(host) => format!("http://{host}/v1/"),
            Err(_) => "https://firestore.googleapis.com/v1/".to_string(),
        }
    }

    /// URL of the database-scoped RPC endpoints (`:commit`, `:batchGet`).
    fn rpc_url(&self, rpc: &str) -> String {
        format!(
            "{}projects/{}/databases/(default)/documents:{}",
            self.base_url(),
            self.project_id,
            rpc
        )
    }

    fn encode_record(rec: &LeaseRecord) -> Value {
        json!({
            "fields": {
                "holder_id": { "stringValue": rec.holder_id },
                "expires_at": { "timestampValue": rec.expires_at.to_rfc3339() }
            }
        })
    }

    fn decode_record(doc: &Value) -> Result<LeaseRecord> {
        let fields = doc
            .get("fields")
            .ok_or_else(|| Error::Serialisation("missing fields".into()))?;
        let holder_id = fields
            .get("holder_id")
            .and_then(|v| v.get("stringValue"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Serialisation("missing holder_id".into()))?
            .to_string();
        let ts = fields
            .get("expires_at")
            .and_then(|v| v.get("timestampValue"))
            .and_then(|v| v.as_str())
            .ok_or_else(|| Error::Serialisation("missing expires_at".into()))?;
        let expires_at = DateTime::parse_from_rfc3339(ts)
            .map_err(|e| Error::Serialisation(e.to_string()))?
            .with_timezone(&Utc);
        Ok(LeaseRecord {
            holder_id,
            expires_at,
        })
    }

    /// Read the document via `:batchGet` and return
    /// `(LeaseRecord, updateTime_etag)`, or `None` if missing.
    ///
    /// Body-based RPC (like the official SDKs) rather than a GET on the
    /// document URI: resource names in bodies need no URI encoding, and the
    /// Firestore emulator implements the RPC surface far more faithfully
    /// than the URI-addressed REST shim.
    async fn read_current(
        &self,
        scope: &LeaseScope,
        token: &str,
    ) -> Result<Option<(LeaseRecord, String)>> {
        let body = json!({ "documents": [self.doc_name(scope)] });
        let resp = self
            .client
            .post(self.rpc_url("batchGet"))
            .bearer_auth_if_set(token)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        if !resp.status().is_success() {
            return Err(Error::Database(format!(
                "firestore batchGet failed: {}",
                resp.status()
            )));
        }
        let results: Value = resp
            .json()
            .await
            .map_err(|e| Error::Serialisation(e.to_string()))?;
        let Some(first) = results.as_array().and_then(|a| a.first()) else {
            return Err(Error::Database("firestore batchGet: empty response".into()));
        };
        let Some(found) = first.get("found") else {
            return Ok(None); // "missing" entry
        };
        let record = Self::decode_record(found)?;
        let update_time = found["updateTime"]
            .as_str()
            .ok_or_else(|| Error::Serialisation("missing updateTime in Firestore response".into()))?
            .to_string();
        Ok(Some((record, update_time)))
    }

    /// Detect a precondition rejection in a `:commit` error body.
    fn is_precondition_rejection(status: reqwest::StatusCode, text: &str) -> bool {
        matches!(
            status,
            reqwest::StatusCode::CONFLICT | reqwest::StatusCode::BAD_REQUEST
        ) && (text.contains("ABORTED")
            || text.contains("FAILED_PRECONDITION")
            || text.contains("ALREADY_EXISTS"))
    }

    /// Conditionally write the record via `:commit`. Returns `true` if
    /// written, `false` if the precondition was rejected (another writer
    /// won the race).
    ///
    /// The precondition travels in the commit BODY (`writes[].currentDocument`)
    /// — the SDK wire shape. The previous PATCH with a
    /// `currentDocument.updateTime` QUERY PARAM was parsed by the emulator as
    /// version 0 ("required base version (0)"), rejecting every legitimate
    /// CAS: steal, renew, release, and idempotent re-acquire all failed.
    async fn patch_conditional(
        &self,
        scope: &LeaseScope,
        token: &str,
        rec: &LeaseRecord,
        precondition: Precondition,
    ) -> Result<bool> {
        let current_document = match precondition {
            Precondition::MustNotExist => json!({ "exists": false }),
            Precondition::MustHaveUpdateTime(ref ts) => json!({ "updateTime": ts }),
        };
        let mut update = Self::encode_record(rec);
        update["name"] = json!(self.doc_name(scope));
        let body = json!({
            "writes": [{ "update": update, "currentDocument": current_document }]
        });
        let resp = self
            .client
            .post(self.rpc_url("commit"))
            .bearer_auth_if_set(token)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        let status = resp.status();
        if status.is_success() {
            return Ok(true);
        }
        let text = resp.text().await.unwrap_or_default();
        if Self::is_precondition_rejection(status, &text) {
            return Ok(false);
        }
        Err(Error::Database(format!(
            "firestore commit failed: {status} {text}"
        )))
    }

    /// Conditional delete via `:commit`. No-ops on precondition failure
    /// (the doc was already modified — we must not delete the new holder's record).
    async fn delete_conditional(
        &self,
        scope: &LeaseScope,
        token: &str,
        update_time: &str,
    ) -> Result<()> {
        let body = json!({
            "writes": [{
                "delete": self.doc_name(scope),
                "currentDocument": { "updateTime": update_time }
            }]
        });
        let resp = self
            .client
            .post(self.rpc_url("commit"))
            .bearer_auth_if_set(token)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::Database(e.to_string()))?;
        let status = resp.status();
        if status.is_success() || status == reqwest::StatusCode::NOT_FOUND {
            return Ok(());
        }
        let text = resp.text().await.unwrap_or_default();
        if Self::is_precondition_rejection(status, &text) {
            // Doc was modified after our read — no-op is correct.
            return Ok(());
        }
        Err(Error::Database(format!(
            "firestore commit-delete failed: {status} {text}"
        )))
    }
}

#[async_trait]
impl LeaseStore for FirestoreLeaseStore {
    async fn try_acquire(
        &self,
        scope: &LeaseScope,
        holder_id: &str,
        ttl: Duration,
    ) -> Result<AcquireOutcome> {
        let token = self
            .token
            .get()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        let now = Utc::now();
        let new_expires =
            now + chrono::Duration::from_std(ttl).map_err(|e| Error::Other(e.to_string()))?;
        let new_rec = LeaseRecord {
            holder_id: holder_id.into(),
            expires_at: new_expires,
        };

        match self.read_current(scope, &token).await? {
            None => {
                // Atomic create: precondition ensures only one winner when two
                // instances race on an absent scope.
                if self
                    .patch_conditional(scope, &token, &new_rec, Precondition::MustNotExist)
                    .await?
                {
                    Ok(AcquireOutcome::Acquired(new_rec))
                } else {
                    // Lost the race — re-read who won and report them as the holder.
                    // v1: if the winner released before this re-read, the caller sees a transient HeldByOther denial and must retry.
                    match self.read_current(scope, &token).await? {
                        Some((current, _)) => Ok(AcquireOutcome::HeldByOther(current)),
                        None => Err(Error::Database(
                            "lease vanished immediately after concurrent create".into(),
                        )),
                    }
                }
            }
            Some((ref existing, ref etag)) if existing.expires_at <= now => {
                // Steal expired lease: CAS on updateTime prevents two stealers both winning.
                if self
                    .patch_conditional(
                        scope,
                        &token,
                        &new_rec,
                        Precondition::MustHaveUpdateTime(etag.clone()),
                    )
                    .await?
                {
                    Ok(AcquireOutcome::Acquired(new_rec))
                } else {
                    // v1: if the winner released before this re-read, the caller sees a transient HeldByOther denial and must retry.
                    match self.read_current(scope, &token).await? {
                        Some((current, _)) => Ok(AcquireOutcome::HeldByOther(current)),
                        None => Err(Error::Database(
                            "lease vanished immediately after concurrent steal".into(),
                        )),
                    }
                }
            }
            Some((ref existing, ref etag)) if existing.holder_id == holder_id => {
                // Idempotent re-acquire: extend our own TTL atomically.
                if self
                    .patch_conditional(
                        scope,
                        &token,
                        &new_rec,
                        Precondition::MustHaveUpdateTime(etag.clone()),
                    )
                    .await?
                {
                    Ok(AcquireOutcome::Acquired(new_rec))
                } else {
                    // Another instance wrote between our GET and PATCH.
                    match self.read_current(scope, &token).await? {
                        Some((current, _)) if current.holder_id == holder_id => {
                            // A concurrent renewal by us won; still acquired.
                            Ok(AcquireOutcome::Acquired(current))
                        }
                        Some((current, _)) => Ok(AcquireOutcome::HeldByOther(current)),
                        None => Err(Error::Database(
                            "lease vanished immediately after concurrent re-acquire".into(),
                        )),
                    }
                }
            }
            Some((existing, _)) => Ok(AcquireOutcome::HeldByOther(existing)),
        }
    }

    async fn renew(
        &self,
        scope: &LeaseScope,
        holder_id: &str,
        ttl: Duration,
    ) -> Result<RenewOutcome> {
        let token = self
            .token
            .get()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        let now = Utc::now();
        let new_expires =
            now + chrono::Duration::from_std(ttl).map_err(|e| Error::Other(e.to_string()))?;

        match self.read_current(scope, &token).await? {
            Some((ref rec, ref etag)) if rec.holder_id == holder_id && rec.expires_at > now => {
                let renewed = LeaseRecord {
                    holder_id: holder_id.into(),
                    expires_at: new_expires,
                };
                if self
                    .patch_conditional(
                        scope,
                        &token,
                        &renewed,
                        Precondition::MustHaveUpdateTime(etag.clone()),
                    )
                    .await?
                {
                    Ok(RenewOutcome::Renewed(renewed))
                } else {
                    // Stolen between our GET and PATCH.
                    let current = self.read_current(scope, &token).await?.map(|(rec, _)| rec);
                    Ok(RenewOutcome::Lost { current })
                }
            }
            other => Ok(RenewOutcome::Lost {
                current: other.map(|(rec, _)| rec),
            }),
        }
    }

    async fn release(&self, scope: &LeaseScope, holder_id: &str) -> Result<()> {
        let token = self
            .token
            .get()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        match self.read_current(scope, &token).await? {
            None => Ok(()),
            Some((rec, _)) if rec.holder_id != holder_id => Ok(()),
            Some((_, etag)) => self.delete_conditional(scope, &token, &etag).await,
        }
    }

    async fn current(&self, scope: &LeaseScope) -> Result<Option<LeaseRecord>> {
        let token = self
            .token
            .get()
            .await
            .map_err(|e| Error::Other(e.to_string()))?;
        Ok(self.read_current(scope, &token).await?.map(|(rec, _)| rec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn precondition_variant_coverage() {
        // Compile-time proof that both variants exist and are pattern-matchable.
        let p = Precondition::MustNotExist;
        assert!(matches!(p, Precondition::MustNotExist));
        let q = Precondition::MustHaveUpdateTime("ts".into());
        assert!(matches!(q, Precondition::MustHaveUpdateTime(_)));
    }
}
