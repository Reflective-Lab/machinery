# Quorum-sense — ambient handler adoption

Wire quorum ambient jobs against Runtime Runway's `runway-ambient` substrate.
The handler lives in **quorum-sense** (`crates/quorum-server/src/ambient_handler.rs`);
this repo provides the seam.

As of v3.6.0 the seam is fully typed:

- Job keys are [`JobKey`] (validated kebab-case newtype in `runway-app-host`),
  not bare strings. Serde round-trips them as plain strings — the wire format
  and `runway.app.json` are unchanged.
- The app injects the runtime via `with_jobs_runtime(Box<dyn JobsRuntime>)`;
  `runway-app-host` has no dependency on `runway-ambient`.
- `AmbientJobs::mount` **refuses to start** if the packet-registered job keys
  and `handler.job_keys()` drift in either direction. A registered key with no
  handler arm would accept enqueues that are never claimed; a handler key that
  is not registered could never be enqueued. Both are startup errors, not
  runtime surprises.

## Job keys (register in `runway.app.json`)

| `job_key` | Phase | Purpose |
|-----------|-------|---------|
| `sensemap-refresh` | preparatory | SenseMap maintenance between bursts |
| `mnemos-recall` | preparatory | Mnemos cross-session recall prep |
| `drift-scan` | preparatory | Anticipatory drift scan (Fathom/SenseMap spine) |

```json
"jobs": [
  { "key": "sensemap-refresh", "display_name": "SenseMap refresh", "source": "app-domain" },
  { "key": "mnemos-recall", "display_name": "Mnemos cross-session recall", "source": "app-domain" },
  { "key": "drift-scan", "display_name": "Anticipatory drift scan", "source": "app-domain" }
]
```

Every key in the `jobs` array must have a matching arm in the app handler —
the manifest is a promise the handler must keep, enforced at startup.

## `quorum-server` wiring

Add to `Cargo.toml`:

```toml
runway-ambient = { path = "../../runtime-runway/crates/runway-ambient" }
```

Implement the handler (keys are `JobKey`, parsed once at construction):

```rust
use async_trait::async_trait;
use runway_ambient::{AmbientError, AmbientJobHandler, AmbientJobRecord};
use runway_app_host::JobKey;
use serde_json::Value;

pub struct QuorumAmbientHandler {
    keys: Vec<JobKey>,
    // ... services ...
}

#[async_trait]
impl AmbientJobHandler for QuorumAmbientHandler {
    fn job_keys(&self) -> &[JobKey] {
        &self.keys
    }

    async fn execute(&self, job: &AmbientJobRecord) -> Result<Value, AmbientError> {
        // Parse job.job_key into an app-level enum at this boundary;
        // return AmbientError::InvalidPayload for malformed payloads
        // (deterministic failures are never retried).
        todo!()
    }
}
```

In `main.rs` (after `StorageKit` is ready):

```rust
use std::sync::Arc;
use runway_ambient::AmbientJobs;

let handler = Arc::new(QuorumAmbientHandler::new(/* services */));

RunwayAppHost::builder(packet)
    .with_storage(storage)
    .with_jobs_runtime(Box::new(AmbientJobs::with_handler(handler)))
    // ... existing mounts ...
    .build()
    .await?
    .serve()
    .await?;
```

`build()` fails fast if registered keys and handler keys don't match exactly.

Environment:

```bash
RUNWAY_AMBIENT_ENABLED=true
# optional wake fan-out:
# RUNWAY_AMBIENT_PUBSUB_TOPIC=projects/${PROJECT}/topics/${ENV}.quorum-sense.ingestion
```

## Enqueue from Quorum (preparatory)

HTTP (operator or internal client):

```http
POST /v1/ambient/jobs
Authorization: Bearer <firebase-token>
Content-Type: application/json

{
  "job_key": "sensemap-refresh",
  "phase": "preparatory",
  "payload": { "scope": { "inquiry_id": "inq-abc" } }
}
```

A malformed `job_key` returns `400 {"error": "invalid_job_key"}`; a
well-formed but unregistered key returns `400 {"error": "unregistered_job_key"}`.

In-process (after a burst schedules ambient work):

```rust
use runway_ambient::{AmbientJobQueue, AmbientJobRequest};

let job_id = queue.enqueue(AmbientJobRequest::preparatory(
    "sensemap-refresh".parse()?,   // JobKey — parse once, at the boundary
    org_id.clone(),
    "quorum-sense",
    json!({ "scope": { "inquiry_id": inquiry_id } }),
)).await?;
```

Commit-bound work (requires recorded canonical core):

```rust
queue.enqueue(AmbientJobRequest::commit_bound(
    "drift-scan".parse()?,
    org_id,
    "quorum-sense",
    commit_ref,  // canonical core id / truth package ref
    json!({ "signals": [] }),
)).await?;
```

## Error contract

`AmbientError` is fully structured — there is no catch-all string variant.
Retryability lives on the type: `AmbientError::is_retryable()` returns `false`
for deterministic failures (`InvalidPayload`, `UnhandledJobKey`,
`MissingCommitRef`), which go straight to dead-letter. App-side failures wrap
via `AmbientError::execution(err)` with the source chain preserved.

## Verify

```bash
# runtime-runway
just test-ambient

# quorum-sense (after wiring)
curl -H "Authorization: Bearer dev" -H "Content-Type: application/json" \
  -d '{"job_key":"sensemap-refresh","phase":"preparatory","payload":{}}' \
  http://localhost:8080/quorum-sense/v1/ambient/jobs
```
