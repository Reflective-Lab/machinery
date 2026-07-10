#!/usr/bin/env bash
# Deploy quorum-sense-backend to Cloud Run and refresh the apps portal.
#
# quorum-sense owns its own build and deploy pipeline:
#   cloudbuild.yaml              — Cloud Build image spec
#   deploy/backend/Dockerfile.cloudrun  — multi-stage Rust + SPA image
#   deploy/cloud-run-provision.sh       — thin caller over RR D4 template
#
# This script is a Runtime Runway–side entry point that:
#   1. Delegates the build + Cloud Run deploy to quorum-sense's own provision
#      script (which already wires the RR D4 template correctly).
#   2. Registers quorum-sense in the apps portal registry (apps.json).
#   3. Redeploys Firebase Hosting so the new registry entry goes live.
#
# Environment overrides (all optional):
#   QUORUM_ROOT    — path to quorum-sense checkout  (default: ../marquee-apps/quorum-sense)
#   TAG            — image tag to deploy             (default: latest)
#   BUILD          — set to 1 to cloud-build before deploy
#   DRY_RUN        — set to 1 to print gcloud cmd without running it
#   GCP_PROJECT    — GCP project id                  (default: reflective-labs)
#   GCP_REGION     — Cloud Run region                (default: europe-west1)
#   PROJECT_ID     — Firebase project for hosting    (default: wolfgang-kb-prod)
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
QUORUM_ROOT="${QUORUM_ROOT:-$(cd "$ROOT_DIR/../marquee-apps/quorum-sense" && pwd)}"
PROJECT_ID="${PROJECT_ID:-wolfgang-kb-prod}"

command -v gcloud >/dev/null 2>&1 || { echo "gcloud CLI is required" >&2; exit 1; }
[[ -d "$QUORUM_ROOT" ]] || { echo "quorum-sense not found at $QUORUM_ROOT" >&2; exit 1; }

PROVISION_SCRIPT="$QUORUM_ROOT/deploy/cloud-run-provision.sh"
[[ -x "$PROVISION_SCRIPT" ]] || { echo "provision script not found/executable at $PROVISION_SCRIPT" >&2; exit 1; }

# Point the provision script at this checkout's RR D4 template so it uses the
# local version rather than the sibling-repo default.
export RR_DEPLOY_TEMPLATE="$ROOT_DIR/ops/templates/cloud-run-deploy.sh"

GIT_SHA="$(git -C "$ROOT_DIR" rev-parse --short HEAD)"
QUORUM_VERSION="$(grep '^version' "$QUORUM_ROOT/Cargo.toml" | head -1 | sed 's/.*= "\(.*\)"/\1/')"
ROUTE_PREFIX="${ROUTE_PREFIX:-/quorum-sense}"

echo "Project:        $PROJECT_ID"
echo "Quorum root:    $QUORUM_ROOT"
echo "Version:        $QUORUM_VERSION"
echo "RR SHA:         $GIT_SHA"
echo "Route prefix:   $ROUTE_PREFIX"
echo ""

echo "==> Delegating build + Cloud Run deploy to quorum-sense provision script..."
"$PROVISION_SCRIPT"

echo ""
echo "Registering in apps portal..."
bash "$ROOT_DIR/ops/scripts/register-app.sh" \
    --key        "quorum" \
    --name       "Quorum" \
    --description "Sense-making and decision intelligence for leadership teams" \
    --path       "$ROUTE_PREFIX" \
    --status-path "${ROUTE_PREFIX}/healthz" \
    --version    "$QUORUM_VERSION" \
    --sha        "$GIT_SHA"

echo "Deploying apps portal..."
(cd "$ROOT_DIR/ops/infra/firebase/apps" && firebase deploy --only hosting:apps-reflective-se --project "$PROJECT_ID" --non-interactive)
