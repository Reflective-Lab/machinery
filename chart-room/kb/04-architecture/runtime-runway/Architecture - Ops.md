---
type: architecture-module
source-path: runtime-runway/ops/
last-scanned: 2026-06-07
tags: [architecture, runtime-runway, ops, infrastructure]
---

# runtime-runway — Ops

<!-- @generated:start -->

Part of [[Architecture - Overview|runtime-runway]]. Operational assets at `runtime-runway/ops/`: 41 source files (Shell, JavaScript, Python, HCL, JSON, YAML) at commit `012b81b`.

## Layout

| Path | Contents |
|---|---|
| `ops/deploy/gpu/cloudrun/` | Cloud Run GPU deploy configuration |
| `ops/deploy/gpu/modal/` | Modal GPU deploy configuration |
| `ops/deploy/gpu/runpod/` | RunPod GPU deploy configuration |
| `ops/infra/terraform/` | GCP Terraform modules: `environments/`, `modules/`, `main.tf`, `variables.tf`, `versions.tf`, `terraform.tfvars.example`, `outputs.tf` |
| `ops/infra/firebase/` | Firestore + Storage security: `firestore.rules`, `storage.rules`, `firestore.indexes.json`, `firebase.json`, `.firebaserc.example`, `apps/` |
| `ops/scripts/` | Deployment + ops scripts: `deploy-api-server.sh`, `deploy-cloud-run.sh`, `dev-up.sh`, `dev-down.sh`, `gcp-setup.sh`, `register-app.sh`, `smoke-test.sh`; plus subdirs `ollama/`, `workflow/` |

## What this gives the platform

Three deployable surfaces with explicit IaC:

1. **GCP Cloud Run** — the reference HTTP service target. `deploy-cloud-run.sh` + `deploy-api-server.sh` invoke against the Terraform-managed project.
2. **Firebase** — Firestore data plane + Storage. Security rules and composite indexes are owned here, not by application code.
3. **GPU inference targets (three choices)** — Cloud Run GPU, Modal, or RunPod. The crate `runtime-runway/crates/llm` is the runtime that runs on any of these.

Local development uses `dev-up.sh` / `dev-down.sh` against `ollama/` (local LLM) for the GPU path.

## CI

5 workflows at `runtime-runway/.github/workflows/`:

- `ci.yml` — main lint/test/build pipeline
- `contract.yml` — `runway-storage-contract` test suite (parity across local vs. remote backends — see [[Architecture - Crates|Crates]])
- `contract-staging.yml` — same suite against staging
- `release.yml` — tag → publish
- `security.yml` — SAST + dependency checks

Per commit `1dd69a0` (2026-05-29), `gitleaks-action` was replaced with a direct binary call to skip the GitHub org license gate. Per commit `698ddde` (2026-05-30), CI sibling-repo checkout was fixed: drops the converge checkout step and pulls `commerce-rails` into place — implies the CI runs against multiple sibling polyrepos by checking each one into a known path.

## Cross-references

- [[Architecture - Overview|runtime-runway overview]]
- [[Architecture - Crates|runtime-runway crates]]
- [[../../deployment-and-infrastructure|Deployment and Infrastructure]] (KB-wide)
- [[../../security-review|Security Review]] (KB-wide)

<!-- @generated:end -->
