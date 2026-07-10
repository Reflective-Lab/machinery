// Sync engine: push local ExperienceEvents to remote, pull remote document changes.
//
// Protocol:
//   1. Push: query local event_log WHERE synced_at IS NULL → append to remote EventLog → mark_synced
//   2. Pull: query remote documents WHERE updated_at > last_checkpoint → merge into local DocumentStore
//   3. Conflict: remote wins on `status` fields; local wins on `body`/`content`
//   4. Re-embed: replace zero-padded local fastembed vectors with remote provider embeddings
//   5. Update checkpoint in local objects store at "sync/checkpoint.json"

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    StorageKit,
    traits::{Error, document::Document, embedding::EMBEDDING_DIMS, event::EventQuery},
};

const CHECKPOINT_KEY: &str = "sync/checkpoint.json";
const LOCAL_EMBED_DIMS: usize = 384;

/// Fields where the local copy wins during pull merge.
const LOCAL_WINS_FIELDS: &[&str] = &["body", "content"];

#[derive(Debug, Serialize, Deserialize)]
struct Checkpoint {
    last_sync: DateTime<Utc>,
}

/// Offline-to-cloud sync engine for Tauri desktop apps.
///
/// One `SyncEngine` per app session. Call [`SyncEngine::sync`] on connect and
/// periodically (e.g. every 30 s) to keep local and remote storage in agreement.
pub struct SyncEngine {
    local: StorageKit,
    remote: StorageKit,
    /// Firestore collections to pull on every sync cycle.
    collections: Vec<String>,
    /// Vector namespaces to scan for zero-padded offline embeddings to re-embed.
    vector_namespaces: Vec<String>,
}

impl SyncEngine {
    pub fn new(local: StorageKit, remote: StorageKit, collections: Vec<String>) -> Self {
        Self {
            local,
            remote,
            collections,
            vector_namespaces: Vec::new(),
        }
    }

    /// Configure vector namespaces whose zero-padded local embeddings should be
    /// upgraded via the remote [`EmbeddingProvider`] on each sync cycle.
    pub fn with_vector_namespaces(mut self, namespaces: Vec<String>) -> Self {
        self.vector_namespaces = namespaces;
        self
    }

    /// Run one full sync cycle.
    ///
    /// Returns `(events_pushed, docs_pulled)`.
    ///
    /// # Errors
    ///
    /// Push errors per-event are logged and skipped — only successfully pushed
    /// events are marked synced. Pull errors terminate the cycle and the
    /// checkpoint is **not** advanced, so the next cycle will retry.
    pub async fn sync(&self) -> Result<(usize, usize), Error> {
        let events_pushed = self.push_events().await?;
        let docs_pulled = self.pull_documents().await?;
        let vectors_reembedded = self.reembed_vectors().await?;
        tracing::info!(
            events_pushed,
            docs_pulled,
            vectors_reembedded,
            "sync cycle complete"
        );
        Ok((events_pushed, docs_pulled))
    }

    // ── Push phase ────────────────────────────────────────────────────────────

    async fn push_events(&self) -> Result<usize, Error> {
        let syncable = self
            .local
            .syncable_events
            .as_ref()
            .ok_or_else(|| Error::Other("local StorageKit missing syncable_events".into()))?;

        let unsynced = syncable.query_unsynced(EventQuery::default()).await?;

        if unsynced.is_empty() {
            return Ok(0);
        }

        let mut synced_ids: Vec<String> = Vec::with_capacity(unsynced.len());

        for event in unsynced {
            let id = event.event_id.clone();
            match self.remote.events.append(event).await {
                Ok(()) => {
                    synced_ids.push(id);
                }
                Err(err) => {
                    tracing::warn!(event_id = %id, error = %err, "skipping event — remote append failed");
                }
            }
        }

        let pushed = synced_ids.len();
        if !synced_ids.is_empty() {
            syncable.mark_synced(&synced_ids).await?;
        }

        Ok(pushed)
    }

    // ── Pull phase ────────────────────────────────────────────────────────────

    async fn pull_documents(&self) -> Result<usize, Error> {
        let checkpoint = self.load_checkpoint().await?;

        let mut docs_pulled: usize = 0;

        for collection in &self.collections {
            let mut q = crate::traits::document::Query::new();
            if let Some(ts) = checkpoint {
                q = q.updated_after(ts);
            }

            let docs = self.remote.documents.query(collection, q).await?;

            for doc in docs {
                let local_doc = self.local.documents.get(collection, &doc.id).await?;
                let merged = merge_document(local_doc.as_ref(), doc);
                self.local.documents.put(collection, merged).await?;
                docs_pulled += 1;
            }
        }

        // Only advance the checkpoint after a fully successful pull.
        self.save_checkpoint(Utc::now()).await?;

        Ok(docs_pulled)
    }

    // ── Re-embedding phase ────────────────────────────────────────────────────

    async fn reembed_vectors(&self) -> Result<usize, Error> {
        if self.vector_namespaces.is_empty() {
            return Ok(0);
        }

        let mut reembedded = 0usize;

        for namespace in &self.vector_namespaces {
            let entries = self.local.vectors.list_for_reembed(namespace).await?;

            for entry in entries {
                if !is_zero_padded(entry.embedding.as_slice()) {
                    continue;
                }
                let Some(text) = entry.text.as_deref() else {
                    tracing::debug!(vector_id = %entry.id, "skipping re-embed — no source text");
                    continue;
                };

                let upgraded = self.remote.embeddings.embed(text).await?;
                self.local
                    .vectors
                    .upsert(
                        namespace,
                        &entry.id,
                        &upgraded,
                        Some(text),
                        std::collections::HashMap::new(),
                    )
                    .await?;
                reembedded += 1;
            }
        }

        Ok(reembedded)
    }

    // ── Checkpoint helpers ────────────────────────────────────────────────────

    async fn load_checkpoint(&self) -> Result<Option<DateTime<Utc>>, Error> {
        match self.local.objects.get_text(CHECKPOINT_KEY).await {
            Ok(text) => {
                let cp: Checkpoint =
                    serde_json::from_str(&text).map_err(|e| Error::Serialisation(e.to_string()))?;
                Ok(Some(cp.last_sync))
            }
            // No checkpoint yet — first sync, pull everything.
            Err(Error::NotFound(_)) => Ok(None),
            Err(err) => Err(err),
        }
    }

    async fn save_checkpoint(&self, ts: DateTime<Utc>) -> Result<(), Error> {
        let cp = Checkpoint { last_sync: ts };
        let text = serde_json::to_string(&cp).map_err(|e| Error::Serialisation(e.to_string()))?;
        self.local.objects.put_text(CHECKPOINT_KEY, &text).await
    }
}

/// Merge a remote document into the local store using M4 conflict rules.
///
/// Remote is the base. Local wins on `body` and `content`. All `status` fields
/// (exact key `status` or suffix `_status`) remain from remote.
fn merge_document(local: Option<&Document>, remote: Document) -> Document {
    let Some(local) = local else {
        return remote;
    };

    let mut merged = remote;
    for field in LOCAL_WINS_FIELDS {
        if let Some(value) = local.data.get(*field) {
            merged.data.insert((*field).to_string(), value.clone());
        }
    }
    merged
}

/// True when the upper half of a 768-dim vector is all zeros — the fastembed
/// offline zero-pad signature.
fn is_zero_padded(values: &[f32]) -> bool {
    values.len() == EMBEDDING_DIMS && values[LOCAL_EMBED_DIMS..].iter().all(|&v| v == 0.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::traits::event::StoredEvent;
    use chrono::Duration;
    use serde_json::json;
    use std::collections::HashMap;
    use std::sync::Arc;

    async fn build_kit(dir: &std::path::Path) -> StorageKit {
        StorageKit::local(dir).await.expect("local kit")
    }

    fn mk_event(id: &str) -> StoredEvent {
        StoredEvent {
            event_id: id.to_string(),
            org_id: "sync-org".into(),
            app_id: "sync-app".into(),
            event_type: "sync.test".into(),
            context_id: None,
            fact_id: None,
            payload: json!({"n": 1}),
            occurred_at: Utc::now(),
            synced_at: None,
        }
    }

    fn mk_doc(id: &str, data: serde_json::Value, updated_at: DateTime<Utc>) -> Document {
        let map: HashMap<String, serde_json::Value> = match data {
            serde_json::Value::Object(m) => m.into_iter().collect(),
            _ => panic!("document data must be object"),
        };
        Document {
            id: id.to_string(),
            data: map,
            created_at: updated_at - Duration::hours(1),
            updated_at,
        }
    }

    struct DeterministicEmbedder {
        marker: f32,
    }

    #[async_trait::async_trait]
    impl crate::traits::embedding::EmbeddingProvider for DeterministicEmbedder {
        async fn embed(&self, _text: &str) -> Result<crate::traits::embedding::Embedding, Error> {
            let mut values = vec![0.0; EMBEDDING_DIMS];
            values[LOCAL_EMBED_DIMS] = self.marker;
            crate::traits::embedding::Embedding::new(values)
        }
    }

    fn kit_with_embedder(
        base: StorageKit,
        embedder: Arc<dyn crate::traits::embedding::EmbeddingProvider>,
    ) -> StorageKit {
        StorageKit {
            embeddings: embedder,
            ..base
        }
    }

    #[test]
    fn merge_document_remote_status_local_body() {
        let local = mk_doc(
            "doc-1",
            json!({"status": "draft", "body": "local body", "title": "local title"}),
            Utc::now(),
        );
        let remote = mk_doc(
            "doc-1",
            json!({"status": "published", "body": "remote body", "title": "remote title"}),
            Utc::now(),
        );

        let merged = merge_document(Some(&local), remote);
        assert_eq!(merged.get::<String>("status").as_deref(), Some("published"));
        assert_eq!(merged.get::<String>("body").as_deref(), Some("local body"));
        assert_eq!(
            merged.get::<String>("title").as_deref(),
            Some("remote title")
        );
    }

    #[test]
    fn is_zero_padded_detects_fastembed_signature() {
        let mut values = vec![1.0; LOCAL_EMBED_DIMS];
        values.resize(EMBEDDING_DIMS, 0.0);
        assert!(is_zero_padded(&values));

        values[LOCAL_EMBED_DIMS] = 0.1;
        assert!(!is_zero_padded(&values));
    }

    #[tokio::test]
    async fn push_marks_events_synced_on_remote() {
        let tmp = tempfile::tempdir().unwrap();
        let local_dir = tmp.path().join("local");
        let remote_dir = tmp.path().join("remote");

        let local = build_kit(&local_dir).await;
        let remote = build_kit(&remote_dir).await;

        let event = mk_event("evt-push-1");
        local
            .syncable_events
            .as_ref()
            .unwrap()
            .append(event)
            .await
            .unwrap();

        let engine = SyncEngine::new(local.clone(), remote.clone(), vec![]);
        let (pushed, _) = engine.sync().await.unwrap();
        assert_eq!(pushed, 1);

        let remote_events = remote.events.query(EventQuery::default()).await.unwrap();
        assert_eq!(remote_events.len(), 1);
        assert_eq!(remote_events[0].event_id, "evt-push-1");

        let unsynced = local
            .syncable_events
            .as_ref()
            .unwrap()
            .query_unsynced(EventQuery::default())
            .await
            .unwrap();
        assert!(unsynced.is_empty());
    }

    #[tokio::test]
    async fn pull_writes_checkpoint_and_merges_conflicts() {
        let tmp = tempfile::tempdir().unwrap();
        let local_dir = tmp.path().join("local");
        let remote_dir = tmp.path().join("remote");

        let local = build_kit(&local_dir).await;
        let remote = build_kit(&remote_dir).await;

        let ts = Utc::now();
        local
            .documents
            .put(
                "notes",
                mk_doc(
                    "note-1",
                    json!({"status": "draft", "body": "local body", "title": "local"}),
                    ts,
                ),
            )
            .await
            .unwrap();

        remote
            .documents
            .put(
                "notes",
                mk_doc(
                    "note-1",
                    json!({"status": "published", "body": "remote body", "title": "remote"}),
                    ts + Duration::seconds(5),
                ),
            )
            .await
            .unwrap();

        let engine = SyncEngine::new(local.clone(), remote, vec!["notes".into()]);
        let (_, pulled) = engine.sync().await.unwrap();
        assert_eq!(pulled, 1);

        let merged = local
            .documents
            .get("notes", "note-1")
            .await
            .unwrap()
            .unwrap();
        assert_eq!(merged.get::<String>("status").as_deref(), Some("published"));
        assert_eq!(merged.get::<String>("body").as_deref(), Some("local body"));
        assert_eq!(merged.get::<String>("title").as_deref(), Some("remote"));

        let checkpoint = local.objects.get_text(CHECKPOINT_KEY).await.unwrap();
        assert!(checkpoint.contains("last_sync"));
    }

    #[tokio::test]
    async fn pull_incremental_respects_checkpoint() {
        let tmp = tempfile::tempdir().unwrap();
        let local_dir = tmp.path().join("local");
        let remote_dir = tmp.path().join("remote");

        let local = build_kit(&local_dir).await;
        let remote = build_kit(&remote_dir).await;

        let first_ts = Utc::now();
        remote
            .documents
            .put("notes", mk_doc("old", json!({"body": "old"}), first_ts))
            .await
            .unwrap();

        let engine = SyncEngine::new(local.clone(), remote.clone(), vec!["notes".into()]);
        let (_, first_pull) = engine.sync().await.unwrap();
        assert_eq!(first_pull, 1);

        remote
            .documents
            .put("notes", mk_doc("new", json!({"body": "new"}), Utc::now()))
            .await
            .unwrap();

        let (_, second_pull) = engine.sync().await.unwrap();
        assert_eq!(second_pull, 1);

        let all = local.documents.get("notes", "new").await.unwrap();
        assert!(all.is_some());
    }

    #[tokio::test]
    async fn reembed_upgrades_zero_padded_vectors() {
        let tmp = tempfile::tempdir().unwrap();
        let local_dir = tmp.path().join("local");
        let remote_dir = tmp.path().join("remote");

        let local = build_kit(&local_dir).await;
        let remote_base = build_kit(&remote_dir).await;
        let remote = kit_with_embedder(
            remote_base,
            Arc::new(DeterministicEmbedder { marker: 0.42 }),
        );

        let mut values = vec![0.5; LOCAL_EMBED_DIMS];
        values.resize(EMBEDDING_DIMS, 0.0);
        let padded = crate::traits::embedding::Embedding::new(values).unwrap();

        local
            .vectors
            .upsert(
                "chunks",
                "chunk-1",
                &padded,
                Some("hello world"),
                HashMap::new(),
            )
            .await
            .unwrap();

        let engine = SyncEngine::new(local.clone(), remote, vec![])
            .with_vector_namespaces(vec!["chunks".into()]);
        engine.sync().await.unwrap();

        let entries = local.vectors.list_for_reembed("chunks").await.unwrap();
        let entry = entries.iter().find(|e| e.id == "chunk-1").unwrap();
        assert_eq!(entry.embedding.as_slice()[LOCAL_EMBED_DIMS], 0.42);
    }
}
