---
type: architecture-module
source-path: studio-apps/wolfgang-chat/backend/
last-scanned: 2026-06-07
tags: [architecture, studio-apps, wolfgang-chat, drift]
---

# wolfgang-chat — Backend

<!-- @generated:start -->

Part of [[Architecture - Overview|wolfgang-chat]]. Rust HTTP + gRPC service deployed to Google Cloud Run (europe-west1). Serves the [[Architecture - Web|web]] product path; the [[Architecture - Desktop|desktop]] app does not call it.

## Shape

- **Package:** `wolfgang-backend` v0.1.0 (binary)
- **HTTP framework:** `axum` v0.8 + `tower` v0.5
- **gRPC framework (optional `grpc` feature):** `tonic` v0.11 + `tonic-web` v0.11 + `prost` v0.12
- **Database:** `rusqlite` v0.37 (bundled SQLite) for relational state
- **Vector store:** `hnsw_rs` v0.3 (with note in code: qdrant unavailable; HNSW is the local fallback)
- **Embeddings:** `ort` v2.0-rc.11 (ONNX Runtime, `load-dynamic` feature) running `all-MiniLM-L6-v2`
- **Tokenization:** `tokenizers` v0.20 (onig feature)
- **Document processing:** `pdf-extract` v0.7, `scraper` v0.20
- **Billing:** `async-stripe` v1.0.0-rc.3 — Wolfgang issues subscription / topup checkouts + handles Stripe webhooks directly (see Drift below)

## HTTP route surface

From `backend/src/main.rs`:

**Chat:**
- `POST /v1/chat/stream` → `chat_stream`

**Billing** (under `/v1/billing/`):
- `GET summary` → `billing_summary`
- `POST checkout/subscription` → `create_subscription_checkout`
- `POST checkout/topup` → `create_topup_checkout`
- `POST portal` → `create_portal_session`
- `POST webhooks/stripe` → `stripe_webhook`

**Brand customization** (under `/v1/brand/`):
- `POST create`
- `GET /` → `load_brand`, `DELETE /` → `clear_brand`
- `GET /avatar`, `GET /background` → serve uploaded assets

**Knowledge** (under `/v1/knowledge/`):
- `POST upload`
- `GET documents`
- `DELETE documents/{doc_hash}`

**Orgs** (under `/v1/orgs/`):
- `POST /` → create_org, `GET /` → list_orgs
- `GET /{org_id}`
- `POST /{org_id}/members`, `GET /{org_id}/members`

**Realtime:** WebSocket layer (see `backend/src/realtime.rs`).

## gRPC services

Exposed via `tonic` when the `grpc` feature is enabled. Definitions in [[Architecture - Overview|wolfgang-chat]]'s `proto/`:

- `DocumentService` (from `proto/document.proto`) — upload, list, delete, URL-ingest documents; stages: `PARSING`, `CHUNKING`, `EMBEDDING`, `INDEXED`, `ERROR`, `FETCHING`.
- `SearchService` (from `proto/search.proto`) — vector + BM25 keyword fallback search; conversational search with conversation history + LLM-generated answers.

gRPC-Web bridge (`tonic-web`) lets [[Architecture - Web|the SvelteKit frontend]] call these services from the browser via Connect-RPC.

## Subdirectories

- `handlers/` — top-level request handlers: conversation, search, upload, `url_ingest`
- `http/` — per-route module implementations: billing, brand, chat, knowledge, org
- `storage/` — `event_db`, metadata, persistence, seed, vector_db management
- `grpc/` — tonic service implementations (`document_service`, `search_service`)
- `embedding/` — Xenova-style embedding backend
- Top-level files: `auth.rs`, `llm.rs`, `main.rs`, `proto.rs`, `realtime.rs`, `store.rs`, `upload_validation.rs`

## ⚠ Drift: depends on retired `converge-runtime`

The backend's `Cargo.toml` declares:

```toml
converge-runtime = { workspace = true, features = ["firebase"] }
runway-auth     = { workspace = true }
runway-middleware = { workspace = true }
runway-telemetry = { workspace = true }
```

`converge-runtime` was retired 2026-06-02 (see [[../../decisions/2026-06-02-converge-runtime-retirement|retirement ADR]]); it is now a compatibility-only shell. Wolfgang's backend appears to pull the crate specifically for its `firebase` feature on HTTP auth. The retirement-ADR's Old→New table says HTTP auth lives in [[../../runtime-runway/Architecture - Crates|runway-auth]] now.

**Migration path:**

1. Verify what `converge_runtime::http_auth` (referenced in [[../../../wolfgang-business/02-architecture/system-overview|wolfgang-business/system-overview.md]] line 45) actually provides vs. what `runway-auth` provides.
2. If `runway-auth` covers it: drop the `converge-runtime` dependency, switch the import in `backend/src/auth.rs` to `runway-auth`'s Tower layer.
3. If `runway-auth` does NOT cover the `firebase` feature surface that Wolfgang needs: this is a real gap in `runway-auth` and should be filed against [[../../runtime-runway/Architecture - Crates|runtime-runway]] rather than worked around in Wolfgang.

Until migrated, Wolfgang's backend is the largest remaining consumer keeping the compat shell alive.

## ⚠ Drift: bypasses commerce-rails

Wolfgang's backend uses `async-stripe` directly for subscription / topup checkout + webhook handling. Per [[../../commerce-rails/Architecture - Overview|commerce-rails]]:

> *"Owns: commercial state, billing, entitlement, marketplace, payout, reconciliation; commercial-authority contracts and provider adapters."*

Wolfgang re-implements its own Stripe layer instead of consuming `commerce-rails-stripe`'s `WebhookReceipt`/`CommercialCommand` envelope and `WebhookVerification` / `ReplayProtection` gates. This is the "thin JTBD app should not redefine commercial authority" anti-pattern flagged in [[../../applet-runtime-boundaries|applet-runtime-boundaries]].

This is documented drift to be addressed as commerce-rails matures past v0.1.2 — not a "fix today" item. Worth recording as a known boundary violation.

## Boundary (target, not current)

Owns: chat semantics for the web path, document/search proto-service implementations, HTTP/gRPC route handlers.
SHOULD NOT own (currently does — see drift above): direct Stripe integration, direct dependency on retired `converge-runtime`.
Does not own: persona definitions (→ [[Architecture - Core|Core]]), vector format definitions (→ [[Architecture - Core|Core]]), proto schemas as authority — but DOES own the `.proto` files in this repo's `proto/` since they define this service's surface.

## Cross-references

- [[Architecture - Overview|wolfgang-chat overview]]
- [[Architecture - Web|Web]] — the gRPC-Web caller
- [[Architecture - Core|Core]] — shared `wolfgang-core` library
- [[../../decisions/2026-06-02-converge-runtime-retirement|2026-06-02 retirement ADR]] — migration target
- [[../../commerce-rails/Architecture - Overview|commerce-rails]] — target owner for billing semantics
- [[../../runtime-runway/Architecture - Crates|runtime-runway/runway-auth]] — target HTTP auth owner

<!-- @generated:end -->
