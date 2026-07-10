> **Archived 2026-07-02** — active milestone tracking moved to Linear (Reflective team).
> This file is kept for historical context only. Do not add new items here.

# Milestones

Strategic milestones for getting Reflective apps online. Each milestone is a shippable state.

---

## Immediate priority — Canonical app execution container

Stop the drift toward app-owned backend servers. Runtime Runway should provide the
standard execution container, Helm should mount operator-control/job modules
into it, and marquee apps should instantiate it with typed app packets.

- [x] Document the Runtime Runway/Helm/app execution-container boundary
- [x] Define the first `AppExecutionPacket` shape using Catalyst
- [x] Split Helm `application-server` responsibilities into Runtime Runway host concerns
      and Helm module concerns
- [x] Extract reusable Runtime Runway host construction into `crates/runway-app-host`
- [x] Mount Helm operator-control and governed-job routes into the Runtime Runway host
      (mounted in quorum-sense 2026-06; modules currently default-shell — wire live state per HELMS H-2026-06-15-01)
- [x] Prove the Runtime Runway container through quorum-sense (overtook Catalyst as the
      reference app; see `REVIEW_quorum-sense_2026-06-15.md`)

---

## RR D5 — Session Ownership Lease ✅ DONE 2026-07

Single-process admission-time lease gate that prevents two Cloud Run instances from
simultaneously believing they own the same session. Hard gate for `--max-instances > 1`
on marquee apps. Cited as CRITICAL in the Helms scale analysis (E11 / E8).

- [x] `LeaseStore` trait in `runway-storage` — `try_acquire`, `renew`, `release`, `current`
- [x] `RedbLeaseStore` — local backend (single `WriteTransaction` read-modify-write)
- [x] `FirestoreLeaseStore` — remote backend (`_runway_leases/{scope_key}` via Firestore REST)
- [x] `StorageKit.leases: Arc<dyn LeaseStore>` — both builders wired
- [x] `SessionOwnershipLayer` / `SessionOwnershipMiddleware` — Tower middleware in `runway-app-host`
- [x] `LeaseGuard` — background renewal task; fire-and-forget release on handler return
- [x] `AppExecutionPacket.ownership_exempt_routes` — manifest field for D1 cross-check
- [x] 4 integration tests passing (including Firestore emulator contention variant)
- [x] 12-case cross-backend contract suite in `runway-storage-contract`

**Limitation (v1):** Admission-time correctness only. No write-side fencing — a paused process that
wakes after a TTL steal can still write through `DocumentStore`. Tracked as `RP-NO-LEASE-WITHOUT-FENCING-V1`.
Follow-up D5.1 will inject `SessionLeaseLost: watch::Receiver<()>` into request extensions for
opt-in graceful abort.

---

## M1 — Shared infrastructure compiles ✅ DONE 2026-05-11

All five `runway-*` crates build and pass `just lint`:

- [x] `runway-storage` — StorageKit with local (redb) and remote (Firestore/GCS/Vertex AI) backends
- [x] `runway-auth` — Firebase Auth Tower middleware
- [x] `runway-middleware` — Axum middleware stack (request-id, trace, CORS, gzip, graceful shutdown)
- [x] `runway-secrets` — GCP Secret Manager client (SecretString, zeroized)
- [x] `runway-telemetry` — OTel OTLP/HTTP → Cloud Trace, Sentry, JSON logs

---

## M2 — GCP project ready for production traffic

**Epic:** E3

Infrastructure provisioned via Terraform, security rules live, billing connected.

**Terraform (ops/infra/terraform/)**
- [ ] Audit all 9 modules: apis, firestore, spanner, storage, pubsub, bigquery, vertex-vector, memorystore, releases
- [ ] Add IAM service accounts + least-privilege bindings to each module
- [ ] `just tf-init` / `just tf-plan` / `just tf-apply` targets in justfile
- [ ] `terraform.tfvars` for prod + staging environments

**Firebase (ops/infra/firebase/)**
- [ ] Deploy `firestore.rules` and `storage.rules` via `firebase deploy --only firestore:rules,storage`
- [ ] Deploy `firestore.indexes.json` via `firebase deploy --only firestore:indexes`
- [ ] Firebase Auth: enable custom claims flow (set by backend on org creation)

**Releases CDN**
- [x] `reflective.se/downloads` static page — detects OS/arch, fetches `latest.json`, per-app download cards
- [x] `latest.json` per app: `{ version, files: { "darwin-aarch64": { url, sha256 }, ... } }`

**Secrets + billing**
- [ ] Populate Secret Manager: `prod-platform-firebase-api-key`, `prod-platform-stripe-webhook-secret`
- [ ] Stripe billing webhook handler (shared Cloud Run) deployed

---

## M3 — Reference app wired (quorum-sense)

> Reference app reassigned 2026-06-15 per panel review. Quorum-sense reached deployed-on-Cloud-Run first and used all five runway crates end-to-end. Wolfgang and Inkling remain as future deployments under M5; they are not the reference shape.



One app uses all five runway crates end-to-end in its Cloud Run backend.

- [x] `runway-telemetry::init()` called at startup; traces flowing to Cloud Trace
- [x] `runway-secrets::Secrets::load_all()` at startup; fails fast on missing secrets
- [x] `runway-storage::StorageKit::remote()` initialized with `RemoteConfig::from_env()`
- [x] `runway-auth::AuthLayer` on all protected Axum routes; `AuthContext` available in handlers
- [x] `runway-middleware::stack()` wrapping the router (via `with_middleware()` on RunwayAppHostBuilder)
- [x] Firestore `EventLog::query()` working (remote): events queryable by org+app+type
- [x] Left column component wired in Svelte frontend: user avatar, subscription badge, app switcher

---

## M4 — Tauri offline-first working ✅ DONE 2026-07

Tauri app runs fully with `StorageKit::local()` and syncs when online.

- [x] `StorageKit::local("~/.quorum-sense")` initialized in Tauri Rust backend
- [x] `local/sync.rs` sync engine complete:
  - Push: `EventLog::query(unsynced_only: true)` → remote `append()` → `mark_synced()`
  - Pull: remote `DocumentStore::query(updated_after: checkpoint)` → local `put()`
  - Checkpoint stored in local object store at `sync/checkpoint.json`
  - Conflict rule: remote wins on `status` fields, local wins on `body`/`content`
- [x] Re-embedding on sync: replace zero-padded local fastembed vectors with Vertex AI 768-dim vectors
- [x] Tauri `onMount` hook triggers sync; spinner overlay while syncing

**Note:** Collection paths are leaf-name only today (`inquiries`, `sensemap`, etc.). Fully-qualified Firestore paths (`orgs/{org_id}/apps/{app_id}/...`) require runtime org/app context — tracked as follow-up once desktop auth flow is wired.

---

## M5 — All marquee apps online

Folio, Wolfgang, Inkling, Scout, Quorum, Vouch — each fully deployed.

**Per app:**
- [x] Firebase Hosting deploy automated via GitHub Actions on push to main (`deploy-hosting.yml`)
- [x] Cloud Run deploy via `just deploy-{app}` (quorum wired; remaining apps tracked in M6)
- [x] Downloadable Tauri binary pipeline wired (`release.yml`); cert procurement tracked in M6

**Shared release pipeline (`.github/workflows/release.yml`):**
- [x] Triggered on `v*` tag push
- [x] Matrix build: `macos-14` (aarch64), `macos-13` (x86_64), `windows-2022`, `ubuntu-22.04`
- [x] Code signing: Apple notarytool, Windows EV cert (signtool), Linux GPG detached sig
- [x] ClamAV scan on built binary
- [x] Upload to `gs://reflective-prod-releases/{app}/{version}/{platform}-{arch}/`
- [x] Update `gs://reflective-prod-releases/{app}/latest.json`
- [x] CDN cache invalidation

**Subscription enforcement:**
- [x] Stripe webhook sets `apps` custom claim on Firebase user (via Admin SDK in shared Cloud Run)
- [x] `runway-auth::AuthLayer::requiring_app("folio")` returns 403 if not in claim

---

## M6 — Remaining app backends + distribution signing

**Epic:** E3

Unblocks full M5 per-app coverage. Three independent work streams.

**App backends (one Cloud Run Rust server per app):**
- [ ] Folio backend — `crates/folio-server` using all five runway crates, `just deploy-folio`
- [ ] Inkling backend — `crates/inkling-server` using all five runway crates, `just deploy-inkling`
- [ ] Vouch backend — `crates/vouch-server` using all five runway crates, `just deploy-vouch`
- [ ] Wolfgang backend — `crates/wolfgang-server` using all five runway crates, `just deploy-wolfgang`

**Code-signing credentials:**
- [ ] Apple Developer Program membership active; Distribution cert + provisioning profile in GitHub secrets
- [ ] Windows EV code-signing cert purchased and loaded into GitHub secrets (`WINDOWS_CERTIFICATE`, `WINDOWS_CERTIFICATE_PASSWORD`)
- [ ] GPG signing key generated and public key published to keys.openpgp.org

**CI wiring:**
- [ ] `GCP_HOSTING_SERVICE_ACCOUNT` secret set in GitHub (`roles/firebasehosting.admin` on `wolfgang-kb-prod`)
- [ ] Release pipeline smoke-tested end-to-end with a `v0.0.1-test` tag on a non-production app

---

## Current sprint — parallel workstreams (2026-05-11)

Four agents running in parallel, each adding one piece to the stack:

| Workstream | Target | Status |
|------------|--------|--------|
| A — Sync engine | `runway-storage/src/local/sync.rs` | In progress |
| B — Release CI/CD | `.github/workflows/release.yml` | In progress |
| C — Terraform audit | `ops/infra/terraform/` modules + justfile targets | In progress |
| D — Remote EventLog query | `runway-storage/src/remote/event.rs::query()` | In progress |

---

## Boundary debt — relocate after canonical execution container lands

Surfaced 2026-05-28 during the Runtime Runway/Helm app-host boundary work. The layer
model says Runtime Runway owns ops substrate and Commerce Rails owns commercial
authority — but several crates currently sit on the
wrong side of that line.

- [x] **`runway-accounts/` → `commerce-rails/`** — fixed 2026-05-28.
      Stripe provider config, API calls, webhook signature mechanics, receipt
      construction, and webhook event mapping now live in
      `commerce-rails/crates/commerce-rails-stripe/`. Runtime Runway keeps the
      intended HTTPS route and identity/org mirror plumbing, and calls the
      Commerce Rails-owned adapter instead of carrying Stripe business logic inside
      `runway-accounts`.

---

## Architecture decisions (locked)

| Decision | Choice | Rationale |
|----------|--------|-----------|
| GCP all-in | Google Cloud + Firebase | Managed services, no DB ops |
| Local storage | redb (pure Rust, ACID) | No system lib conflicts with burn/rusqlite |
| Remote storage | Firestore + GCS + Vertex AI | Fully managed, no ops |
| Embeddings | Vertex AI text-multilingual-embedding-002 | 768-dim, multilingual (Swedish Folio pilot) |
| Auth | Firebase Auth + custom claims | One identity, many app entitlements |
| Vector dims | 768 everywhere | Index compatibility local↔remote |
| Offline vectors | fastembed 384-dim zero-padded → 768 | Re-embedded to exact 768-dim on sync |
| Multi-tenancy | `orgs/{orgId}/apps/{appId}/...` Firestore path | Enforced by security rules + auth claims |
| Messaging | Pub/Sub only (no NATS) | Same capability, fully managed |
| Consensus/Raft | `lattice` crate, not Runtime Runway | Runtime Runway wraps services; Lattice holds algorithms |
| Stripe billing | `org_id` = Stripe customer | One org = one subscription, multiple app entitlements |
