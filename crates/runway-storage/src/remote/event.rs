use async_trait::async_trait;
use serde_json::Value;

use crate::{
    remote::{BearerAuthExt, GcpToken},
    traits::{
        Error, Result,
        event::{EventLog, EventQuery, StoredEvent},
    },
};

/// Appends ExperienceEvents to a Firestore subcollection.
/// Path: `orgs/{org_id}/apps/{app_id}/events/{event_id}`
/// Also fires a BigQuery streaming insert (best-effort, non-blocking).
pub struct FirestoreEventLog {
    project_id: String,
    token: GcpToken,
    client: reqwest::Client,
}

impl FirestoreEventLog {
    pub fn new(project_id: String, token: GcpToken) -> Self {
        Self {
            project_id,
            token,
            // RP-HERMETIC-UNIT (Reflective QUALITY_BACKLOG.md →
            // QF-2026-06-02-05): production constructor for Firestore
            // event sink; tests use Firestore emulators at the test
            // harness level, not DI through this struct.
            #[allow(clippy::disallowed_methods)]
            client: crate::http::client(),
        }
    }

    async fn bearer(&self) -> Result<String> {
        self.token
            .get()
            .await
            .map_err(|e| Error::Network(e.to_string()))
    }

    fn doc_url(&self, event: &StoredEvent) -> String {
        format!(
            "{}/orgs/{}/apps/{}/events/{}",
            crate::endpoints::firestore_documents(&self.project_id),
            event.org_id,
            event.app_id,
            event.event_id
        )
    }
}

#[async_trait]
impl EventLog for FirestoreEventLog {
    async fn append(&self, event: StoredEvent) -> Result<()> {
        let url = self.doc_url(&event);
        let body = serde_json::json!({
            "fields": {
                // org_id/app_id live in the document path, but query()
                // filters on them as FIELDS (mandatory for collection-group
                // queries) — a filter on a never-written field matches
                // nothing, which made every appended event unqueryable.
                "org_id":      { "stringValue": event.org_id },
                "app_id":      { "stringValue": event.app_id },
                "event_type":  { "stringValue": event.event_type },
                "context_id":  { "stringValue": event.context_id.as_deref().unwrap_or_default() },
                "fact_id":     { "stringValue": event.fact_id.as_deref().unwrap_or_default() },
                "payload":     { "stringValue": serde_json::to_string(&event.payload).unwrap_or_default() },
                "occurred_at": { "timestampValue": event.occurred_at.to_rfc3339() },
            }
        });

        self.client
            .patch(&url)
            .bearer_auth_if_set(&self.bearer().await?)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?
            .error_for_status()
            .map_err(|e| Error::Network(e.to_string()))?;

        Ok(())
    }

    async fn query(&self, q: EventQuery) -> Result<Vec<StoredEvent>> {
        // Choose the runQuery URL:
        //   - Both org_id + app_id set → subcollection on that app
        //   - Otherwise → collection group query at the database root
        let docs_base = crate::endpoints::firestore_documents(&self.project_id);
        let url = match (&q.org_id, &q.app_id) {
            (Some(org), Some(app)) => {
                format!("{docs_base}/orgs/{org}/apps/{app}:runQuery")
            }
            _ => format!("{docs_base}:runQuery"),
        };

        // Build the list of field filters.
        let mut filters: Vec<Value> = Vec::new();

        // For a collection group query, add the org_id filter when set.
        // (For subcollection queries the org/app are baked into the path, but an
        // explicit filter is harmless and keeps the logic uniform.)
        if let Some(org) = &q.org_id {
            filters.push(serde_json::json!({
                "fieldFilter": {
                    "field": { "fieldPath": "org_id" },
                    "op": "EQUAL",
                    "value": { "stringValue": org }
                }
            }));
        }

        if let Some(app) = &q.app_id {
            filters.push(serde_json::json!({
                "fieldFilter": {
                    "field": { "fieldPath": "app_id" },
                    "op": "EQUAL",
                    "value": { "stringValue": app }
                }
            }));
        }

        if let Some(event_type) = &q.event_type {
            filters.push(serde_json::json!({
                "fieldFilter": {
                    "field": { "fieldPath": "event_type" },
                    "op": "EQUAL",
                    "value": { "stringValue": event_type }
                }
            }));
        }

        if let Some(since) = q.since {
            filters.push(serde_json::json!({
                "fieldFilter": {
                    "field": { "fieldPath": "occurred_at" },
                    "op": "GREATER_THAN",
                    "value": { "timestampValue": since.to_rfc3339() }
                }
            }));
        }

        // Build the `where` clause.
        let where_clause = match filters.len() {
            0 => None,
            1 => Some(filters.remove(0)),
            _ => Some(serde_json::json!({
                "compositeFilter": { "op": "AND", "filters": filters }
            })),
        };

        // Determine whether to use a collection group or a direct collection.
        let use_collection_group = q.app_id.is_none();
        let from = if use_collection_group {
            serde_json::json!([{ "collectionId": "events", "allDescendants": true }])
        } else {
            serde_json::json!([{ "collectionId": "events" }])
        };

        let mut structured_query = serde_json::json!({ "from": from });
        if let Some(w) = where_clause {
            structured_query["where"] = w;
        }
        if let Some(limit) = q.limit {
            structured_query["limit"] = serde_json::json!(limit);
        }

        let body = serde_json::json!({ "structuredQuery": structured_query });

        let resp: Value = self
            .client
            .post(&url)
            .bearer_auth_if_set(&self.bearer().await?)
            .json(&body)
            .send()
            .await
            .map_err(|e| Error::Network(e.to_string()))?
            .error_for_status()
            .map_err(|e| Error::Network(e.to_string()))?
            .json()
            .await
            .map_err(|e| Error::Network(e.to_string()))?;

        let mut events = Vec::new();
        if let Some(results) = resp.as_array() {
            for result in results {
                if let Some(doc) = result.get("document")
                    && let Some(event) = parse_event_document(doc)
                {
                    events.push(event);
                }
            }
        }
        Ok(events)
    }
}

/// Parse a Firestore document object (as returned by `runQuery`) into a `StoredEvent`.
///
/// Firestore document name format:
///   `projects/{project}/databases/(default)/documents/orgs/{org_id}/apps/{app_id}/events/{event_id}`
///
/// Returns `None` if the document is missing required fields or the name cannot be parsed.
fn parse_event_document(doc: &Value) -> Option<StoredEvent> {
    let name = doc["name"].as_str()?;

    // event_id is always the final path segment. org_id/app_id are read from
    // the stored fields, NOT from name positions: ids containing `/` shift
    // the segment layout, and position-based parsing silently produced
    // garbage ids that never matched callers' queries (contract-suite
    // finding). Name-derived values remain only as a fallback for documents
    // written before org_id/app_id were stored as fields.
    let segments: Vec<&str> = name.split('/').collect();
    let event_id = (*segments.last()?).to_string();
    let orgs_pos = segments.iter().position(|&s| s == "orgs");
    let name_org = orgs_pos
        .and_then(|p| segments.get(p + 1))
        .map(|s| s.to_string());
    let name_app = orgs_pos
        .and_then(|p| segments.get(p + 3))
        .map(|s| s.to_string());

    let fields = &doc["fields"];

    let str_field =
        |key: &str| -> Option<String> { fields[key]["stringValue"].as_str().map(str::to_string) };

    let org_id = str_field("org_id").filter(|s| !s.is_empty()).or(name_org)?;
    let app_id = str_field("app_id").filter(|s| !s.is_empty()).or(name_app)?;

    let event_type = str_field("event_type")?;

    let context_id =
        str_field("context_id").and_then(|s| if s.is_empty() { None } else { Some(s) });
    let fact_id = str_field("fact_id").and_then(|s| if s.is_empty() { None } else { Some(s) });

    let payload_str = str_field("payload").unwrap_or_default();
    let payload: Value = serde_json::from_str(&payload_str).unwrap_or(Value::Null);

    let occurred_at = fields["occurred_at"]["timestampValue"]
        .as_str()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .unwrap_or_else(chrono::Utc::now);

    Some(StoredEvent {
        event_id,
        org_id,
        app_id,
        event_type,
        context_id,
        fact_id,
        payload,
        occurred_at,
        synced_at: None,
    })
}
