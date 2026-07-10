#!/usr/bin/env bash
set -euo pipefail
# Upload platform binaries for a marquee app to the releases bucket and invalidate CDN cache.
#
# Usage:
#   APP=folio VERSION=v1.2.0 PROJECT_ID=my-project bash ops/infra/scripts/publish-release.sh
#   APP=scout VERSION=v2.0.0 PROJECT_ID=my-project ENV=staging bash ...
#
# Expected artifacts in dist/ (produced by each app's CI pipeline):
#   dist/darwin-aarch64/
#   dist/darwin-x86_64/
#   dist/windows-x64/
#   dist/linux-x64/
#   dist/linux-aarch64/
#
# Resulting bucket paths:
#   gs://{bucket}/{app}/{version}/{platform}-{arch}/{filename}

APP="${APP:-}"
VERSION="${VERSION:-}"
PROJECT_ID="${PROJECT_ID:-}"
ENV="${ENV:-prod}"
BUCKET="reflective-${ENV}-releases"
DIST_DIR="${DIST_DIR:-dist}"

[[ -z "$APP" ]]        && { echo "Set APP (e.g. folio, scout, quorum, wolfgang, vouch)"; exit 1; }
[[ -z "$VERSION" ]]    && { echo "Set VERSION (e.g. v1.2.0)"; exit 1; }
[[ -z "$PROJECT_ID" ]] && { echo "Set PROJECT_ID"; exit 1; }

command -v gcloud >/dev/null 2>&1 || { echo "gcloud CLI required"; exit 1; }

DEST="gs://${BUCKET}/${APP}/${VERSION}"
echo "Publishing ${APP} ${VERSION} → ${DEST}/"

PLATFORMS=(
  "darwin-aarch64"
  "darwin-x86_64"
  "windows-x64"
  "linux-x64"
  "linux-aarch64"
)

# Detect sha256 tool (Linux uses sha256sum; macOS uses shasum -a 256)
if command -v sha256sum >/dev/null 2>&1; then
  sha256_of() { sha256sum "$1" | awk '{ print $1 }'; }
elif command -v shasum >/dev/null 2>&1; then
  sha256_of() { shasum -a 256 "$1" | awk '{ print $1 }'; }
else
  echo "ERROR: neither sha256sum nor shasum found"; exit 1
fi

CDN_BASE="https://cdn.reflective.run/${BUCKET}/${APP}/${VERSION}"
PUBLISHED_AT="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

UPLOADED=0
FILES_JSON=""

for PLATFORM in "${PLATFORMS[@]}"; do
  SRC="${DIST_DIR}/${PLATFORM}"
  [[ -d "$SRC" ]] || { echo "  Skipping $PLATFORM (no dist/${PLATFORM}/ directory)"; continue; }

  # Determine binary filename inside the platform directory
  if [[ "$PLATFORM" == "windows-x64" ]]; then
    BINARY="${APP}-${PLATFORM}.exe"
  else
    BINARY="${APP}-${PLATFORM}"
  fi

  BINARY_PATH="${SRC}/${BINARY}"
  if [[ -f "$BINARY_PATH" ]]; then
    SHA="$(sha256_of "${BINARY_PATH}")"
  else
    SHA="unknown"
  fi

  echo "  Uploading $PLATFORM ..."
  gcloud storage cp --recursive "${SRC}/" \
    "${DEST}/${PLATFORM}/" \
    --project="$PROJECT_ID"
  UPLOADED=$((UPLOADED + 1))

  ENTRY="      \"${PLATFORM}\": { \"url\": \"${CDN_BASE}/${PLATFORM}/${BINARY}\", \"sha256\": \"${SHA}\" }"
  if [[ -n "$FILES_JSON" ]]; then
    FILES_JSON="${FILES_JSON},"$'\n'"${ENTRY}"
  else
    FILES_JSON="${ENTRY}"
  fi
done

[[ $UPLOADED -eq 0 ]] && { echo "No platform directories found in ${DIST_DIR}/"; exit 1; }

# latest.json per app — clients poll this to detect available updates
LATEST_JSON=$(cat <<JSON
{
  "version": "${VERSION}",
  "published_at": "${PUBLISHED_AT}",
  "files": {
${FILES_JSON}
  }
}
JSON
)

echo "$LATEST_JSON" | gcloud storage cp - \
  "gs://${BUCKET}/${APP}/latest.json" \
  --content-type="application/json" \
  --project="$PROJECT_ID"

# Invalidate CDN cache for this app's new version + its latest pointer
LB_NAME="reflective-${ENV}-releases"
echo "Invalidating CDN cache ..."
gcloud compute url-maps invalidate-cdn-cache "$LB_NAME" \
  --path="/${APP}/${VERSION}/*" \
  --project="$PROJECT_ID" \
  --async 2>/dev/null || echo "  (CDN invalidation skipped — Load Balancer not yet provisioned)"

gcloud compute url-maps invalidate-cdn-cache "$LB_NAME" \
  --path="/${APP}/latest.json" \
  --project="$PROJECT_ID" \
  --async 2>/dev/null || true

echo ""
echo "Done. Published ${UPLOADED} platform(s)."
echo "  Bucket:  ${DEST}/"
echo "  Latest:  gs://${BUCKET}/${APP}/latest.json"
