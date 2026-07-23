---
type: architecture-module
source-path: studio-apps/wolfgang-chat/infra/
last-scanned: 2026-06-07
tags: [architecture, studio-apps, wolfgang-chat, infrastructure]
---

# wolfgang-chat — Infrastructure

<!-- @generated:start -->

Part of [[Architecture - Overview|wolfgang-chat]]. Terraform-managed GCP infrastructure for the [[Architecture - Backend|backend]] Cloud Run service + web frontend hosting. Wolfgang owns its own infra stack; this is **not** delegated to [[../../runtime-runway/Architecture - Ops|runtime-runway/ops]] — the two are parallel infra surfaces with different ownership.

## Shape

- **Primary configuration:** `infra/environments/prod/wolfgang-bot/main.tf`
- **6 reusable modules** at `infra/modules/`:

| Module | Owns |
|---|---|
| `vpc` | VPC + Serverless VPC Connector |
| `artifact-registry` | Docker image registry (europe-west1) |
| `cloud-run-service` | Cloud Run service deployment |
| `firebase-auth` | Firebase Authentication |
| `load-balancer` | Global load balancer + Cloud Armor |
| `bigquery` | BigQuery dataset + logging sink from Cloud Run |

## GCP resources declared

From `infra/environments/prod/wolfgang-bot/main.tf`:

- `google_project_service` — API enablement
- `google_project_iam_member` — backend Firestore access
- `google_compute_global_address` — load balancer IP
- `google_compute_region_network_endpoint_group` — Cloud Run NEG
- `google_compute_backend_service` — backend config
- `google_bigquery_dataset` — logs dataset
- `google_bigquery_table` — feedback table
- `google_logging_project_sink` — Cloud Run logs → BigQuery

## Outputs

- `registry_url` (Artifact Registry)
- Backend service URL
- Load balancer IP
- BigQuery dataset ID

## Deploy recipes (`Justfile`)

The root `Justfile` exposes parallel recipes for desktop / web / backend / infra:

**Infra:**
- `just gcp-setup` — initial GCP project setup
- `just infra-bucket` — Terraform state bucket bootstrap
- `just infra-init`, `just infra-plan`, `just infra-apply`, `just infra-output`

**Cloud build + deploy:**
- `just cloud-build`, `just cloud-build-prod`
- `just deploy-backend`, `just deploy-backend-prod`
- `just deploy-web`

**Base image lifecycle:**
- `just base-build`, `just base-push`, `just base-upgrade`
- `just docker-build`, `just docker-push`, `just docker-auth`

**Local dev / CI:**
- `just desktop-dev`, `just desktop-build`, `just desktop-release`, `just desktop-dmg`
- `just web-dev`, `just web-build`
- `just backend-dev`, `just backend-build`, `just backend-test`
- `just check`, `just lint`, `just test`, `just smoke`
- `just security-audit`, `just pentest`, `just pentest-staging`

## ⚠ Boundary observation: parallel infra to runtime-runway

Wolfgang owns its full GCP infra stack (VPC, load balancer, Cloud Run service, Firebase Auth project, BigQuery). [[../../runtime-runway/Architecture - Ops|runtime-runway/ops]] also owns a GCP infra stack (Terraform modules for GCP, Firestore rules, Cloud Run GPU deploy).

These are two **separate** infra topologies — Wolfgang is not currently deployed via runtime-runway's recipes. That's a reasonable choice for product-specific deployment surfaces, but worth being explicit about:

- **runtime-runway/ops** owns deployment for `runtime-runway/crates/api-server` (the reference Cloud Run binary embedded apps can use).
- **wolfgang-chat/infra** owns deployment for Wolfgang's product backend specifically.
- These should not be unified unless Wolfgang's backend is replaced with the api-server reference pattern — at which point the wolfgang-chat/infra would shrink to product-specific config (Firebase Auth project, custom domain, BigQuery feedback table) layered on top of runway's deployment substrate.

## Cross-references

- [[Architecture - Overview|wolfgang-chat overview]]
- [[Architecture - Backend|Backend]] — what gets deployed
- [[../../runtime-runway/Architecture - Ops|runtime-runway/ops]] — parallel infra owner
- [[../../beacon-sites/Architecture - Overview|beacon-sites]] — owns web frontend hosting (apps.reflective.se), Wolfgang web embeds there

<!-- @generated:end -->
