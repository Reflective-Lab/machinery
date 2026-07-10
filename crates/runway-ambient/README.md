# runway-ambient

Runtime Runway substrate for **ambient operate** — scheduled or triggered work
that continues between human bursts without an HTTP request driving each step.

## Contract

### Who enqueues

| Caller | Mechanism | When |
|--------|-----------|------|
| App / Helm HTTP handler | `POST /v1/ambient/jobs` | Operator or burst completion triggers preparatory or commit-bound work |
| Converge / Organism (future) | Same envelope via `AmbientJobQueue::enqueue` | Promotion or formation completion triggers ambient continuation |
| Pub/Sub fan-out | `PubSubNotify::notify_enqueued` after durable enqueue | Wake remote workers without polling |

Runtime Runway **owns delivery** (persist, claim, retry, dead-letter, provenance
events). Callers own **job semantics** via `job_key` and `payload`.

### Phases

- **`preparatory`** — research, evidence gathering, candidate drafting before a
  scheduled burst. May enqueue without `commit_ref`.
- **`commit-bound`** — work that depends on a recorded canonical commitment.
  **Requires** `commit_ref`; enqueue fails closed if absent.

### Provenance

Every state transition appends an `EventLog` record:

- `ambient.job.enqueued`
- `ambient.job.started`
- `ambient.job.completed`
- `ambient.job.failed`

Downstream Converge reopen and drift layers consume these events; RR does not
interpret commitment semantics.

### Worker

Apps implement [`AmbientJobHandler`] and pass it to
[`AmbientWorker::spawn`]. The worker polls [`AmbientJobQueue::claim_next`] for
registered `job_key` values declared in the app packet (`JobRegistration`).

### Environment

| Variable | Purpose |
|----------|---------|
| `RUNWAY_AMBIENT_ENABLED` | `true` to spawn worker (default off) |
| `RUNWAY_AMBIENT_POLL_MS` | Poll interval when queue empty (default 1000) |
| `RUNWAY_AMBIENT_MAX_ATTEMPTS` | Retries before dead-letter (default 5) |
| `PUBSUB_EMULATOR_HOST` | Emulator base (`host:port`) for local notify tests |
| `RUNWAY_AMBIENT_PUBSUB_TOPIC` | Full topic id `projects/{p}/topics/{name}` for notify |

### Quorum-sense reference handler

See `src/quorum.rs` (`QuorumAmbientHandler`) and `docs/adoption/quorum-ambient-handler.md`
for the first app wiring — job keys `sensemap-refresh`, `mnemos-recall`, `drift-scan`.

Terraform provisions per-app ingestion topics at
`{env}.{app}.ingestion` (`ops/infra/terraform/modules/pubsub/main.tf`).
