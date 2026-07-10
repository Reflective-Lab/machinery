#!/usr/bin/env bash
# RR D4 — Cloud Run deploy template.
#
# A reusable wrapper that deploys a Runtime Runway app to Cloud Run with its
# deploy_contracts materialized from Commerce-Rails recipes. Apps replace their
# hand-rolled provision script with a thin caller that exports the variables
# below and execs this template — so provider env-var NAMES (STRIPE_*, CR_*)
# live only in the CR recipe + this template, never in the app repo.
#
# The app's own (non-provider) env/secrets go in BASE_ENV_VARS / BASE_SECRETS;
# the contract-owned ones are appended by materialize-deploy-contracts.sh.
#
# Required env:
#   SERVICE_NAME      Cloud Run service name
#   IMAGE             fully-qualified image ref (REPO/name:TAG)
# Common env (with defaults):
#   PROJECT_ID (reflective-labs)  REGION (europe-west1)
#   APP_DIR (.)         dir containing runway.app.json
#   ROUTE_PREFIX        defaults to runway.app.json route_prefix
#   SERVICE_ACCOUNT, MEMORY (1Gi), CPU (1), MIN_INSTANCES (0),
#   MAX_INSTANCES (1), PORT (8080)
#   BASE_ENV_VARS       app's own env vars (comma-separated NAME=value)
#   BASE_SECRETS        app's own secrets (comma-separated NAME=id:version)
#   DRY_RUN=1           print the gcloud command instead of running it
#
# Plus any config-source values the recipes require (e.g. STRIPE_PRICE_*),
# exported by CI/operator at deploy time — never committed to the app repo.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

die() { echo "cloud-run-deploy: $*" >&2; exit 1; }

: "${SERVICE_NAME:?set SERVICE_NAME}"
: "${IMAGE:?set IMAGE (REPO/name:TAG)}"

PROJECT_ID="${PROJECT_ID:-reflective-labs}"
REGION="${REGION:-europe-west1}"
APP_DIR="${APP_DIR:-.}"
MEMORY="${MEMORY:-1Gi}"
CPU="${CPU:-1}"
MIN_INSTANCES="${MIN_INSTANCES:-0}"
MAX_INSTANCES="${MAX_INSTANCES:-1}"
PORT="${PORT:-8080}"
BASE_ENV_VARS="${BASE_ENV_VARS:-}"
BASE_SECRETS="${BASE_SECRETS:-}"

MANIFEST="${APP_DIR%/}/runway.app.json"
[[ -f "$MANIFEST" ]] || die "no runway.app.json at '$MANIFEST'"
if [[ -z "${ROUTE_PREFIX:-}" ]]; then
  ROUTE_PREFIX="$(jq -r '.route_prefix // empty' "$MANIFEST")"
fi

# Materialize CR deploy_contracts -> RUNWAY_DEPLOY_ENV_VARS / RUNWAY_DEPLOY_SECRETS.
eval "$("$SCRIPT_DIR/materialize-deploy-contracts.sh" "$APP_DIR")"

join_nonempty() {
  local out=""
  for part in "$@"; do
    [[ -n "$part" ]] || continue
    if [[ -z "$out" ]]; then out="$part"; else out="$out,$part"; fi
  done
  echo "$out"
}

ENV_VARS="$(join_nonempty "$BASE_ENV_VARS" "${RUNWAY_DEPLOY_ENV_VARS:-}")"
SECRETS="$(join_nonempty "$BASE_SECRETS" "${RUNWAY_DEPLOY_SECRETS:-}")"

gcloud_args=(
  run deploy "$SERVICE_NAME"
  --project="$PROJECT_ID"
  --region="$REGION"
  --image="$IMAGE"
  --platform=managed
  --memory="$MEMORY"
  --cpu="$CPU"
  --min-instances="$MIN_INSTANCES"
  --max-instances="$MAX_INSTANCES"
  --port="$PORT"
)
[[ -n "${SERVICE_ACCOUNT:-}" ]] && gcloud_args+=(--service-account="$SERVICE_ACCOUNT")
[[ -n "$ENV_VARS" ]] && gcloud_args+=(--set-env-vars="$ENV_VARS")
[[ -n "$SECRETS" ]] && gcloud_args+=(--set-secrets="$SECRETS")
[[ "${ALLOW_UNAUTHENTICATED:-1}" == "1" ]] && gcloud_args+=(--allow-unauthenticated)

if [[ "${DRY_RUN:-0}" == "1" ]]; then
  printf 'gcloud'
  printf ' %q' "${gcloud_args[@]}"
  printf '\n'
  exit 0
fi

command -v gcloud >/dev/null 2>&1 || die "gcloud CLI is required (or run with DRY_RUN=1)"
echo "==> Deploying ${SERVICE_NAME} (${IMAGE})"
gcloud "${gcloud_args[@]}"
