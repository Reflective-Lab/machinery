# M5 Deploy Deliverables Report

## Task A — Firebase Hosting GitHub Actions workflow

**Created:** `.github/workflows/deploy-hosting.yml`

- Triggers on `push` to `main` when `ops/infra/firebase/apps/public/**` changes, or manually via `workflow_dispatch`
- Job: `deploy-hosting` on `ubuntu-latest`
- Auth: `google-github-actions/auth@v2` with workload identity (matches the pattern in `contract-staging.yml`)
- Installs Firebase CLI via `npm install -g firebase-tools`
- Deploys: `firebase deploy --only hosting:apps-reflective-se --project wolfgang-kb-prod --non-interactive`
- Working directory set to `ops/infra/firebase/apps` (where `firebase.json` lives)
- No untrusted input interpolated into `run:` steps (security-clean)

**Secrets required in GitHub repo settings:**

| Secret | Value |
|--------|-------|
| `GCP_WORKLOAD_IDENTITY_PROVIDER` | Workload identity provider resource name (same value used in `contract-staging.yml`) |
| `GCP_HOSTING_SERVICE_ACCOUNT` | Service account email with `roles/firebasehosting.admin` on `wolfgang-kb-prod` |

The service account and workload identity binding need to be provisioned in Terraform/ops before this workflow can succeed.

---

## Task B — Quorum-sense Cloud Run deploy script

**Created:** `ops/scripts/deploy-quorum-server.sh`

**Added justfile target:** `deploy-quorum`

quorum-sense has its own `cloudbuild.yaml` and `deploy/backend/Dockerfile.cloudrun` (self-contained multi-stage Rust + SPA build that clones all sibling repos inside Cloud Build). It also has `deploy/cloud-run-provision.sh` which is already a thin caller over the RR D4 template (`ops/templates/cloud-run-deploy.sh`).

The runtime-runway script is a **thin wrapper** that:
1. Points `RR_DEPLOY_TEMPLATE` at this checkout's template (so local edits are used)
2. Delegates the full build + Cloud Run deploy to quorum-sense's own provision script
3. Adds `register-app.sh` registry update (quorum's provision script did not do this)
4. Redeploys Firebase Hosting (also missing from quorum's provision script)

This follows the catalyst/api-server pattern exactly for the portal-side steps, while respecting that quorum-sense owns its build and deploy contract configuration.

**NOT blocked** — quorum-sense has both a Dockerfile and cloudbuild.yaml, and its own provision script that is already wired to the RR D4 template.

---

## Files changed

- `.github/workflows/deploy-hosting.yml` — new
- `ops/scripts/deploy-quorum-server.sh` — new (chmod +x)
- `justfile` — added `deploy-quorum` target
