#!/usr/bin/env bash
# RR D4 — build & push kenneth-backend-math-base with a SEMANTIC tag.
#
# Derives the tag from the pinned ARG versions in docker/Dockerfile.math-base
# so the registry tag always reflects the actual OR-Tools / HIGHS / Rust
# versions baked into the image:
#
#   v<RUST>-ortools-<ORTOOLS>-highs-<HIGHS>
#   e.g. v1.96-ortools-9.14-highs-1.14.0
#
# `:latest` is BANNED for this image (RP-NO-MATH-BASE-LATest): a moving tag
# makes "which solver version is in prod?" unanswerable and silently changes
# downstream builds. Downstream Dockerfiles pull the semantic tag.
#
# Usage:
#   ops/scripts/build-math-base.sh            # build+push the semantic tag
#   DRY_RUN=1 ops/scripts/build-math-base.sh  # print the tag + command only
#   PRINT_TAG=1 ops/scripts/build-math-base.sh # print only the computed tag
#
# Env overrides: PROJECT_ID (reflective-labs), REPO, RUST_VERSION,
# ORTOOLS_TAG, HIGHS_TAG (default: parsed from the Dockerfile).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RR_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
DOCKERFILE="$RR_ROOT/docker/Dockerfile.math-base"

die() { echo "build-math-base: $*" >&2; exit 1; }
[[ -f "$DOCKERFILE" ]] || die "missing $DOCKERFILE"

arg_default() { awk -v k="$1" '$1=="ARG" && $2 ~ "^"k"=" {sub("^"k"=","",$2); print $2; exit}' "$DOCKERFILE"; }

RUST_VERSION="${RUST_VERSION:-$(arg_default RUST_VERSION)}"
ORTOOLS_TAG="${ORTOOLS_TAG:-$(arg_default ORTOOLS_TAG)}"
HIGHS_TAG="${HIGHS_TAG:-$(arg_default HIGHS_TAG)}"
[[ -n "$RUST_VERSION" && -n "$ORTOOLS_TAG" && -n "$HIGHS_TAG" ]] \
  || die "could not parse RUST_VERSION/ORTOOLS_TAG/HIGHS_TAG from Dockerfile"

# Normalize: strip a leading 'v' from solver tags so the composite tag has one
# canonical 'v' prefix and no doubled v's.
strip_v() { echo "${1#v}"; }
TAG="v$(strip_v "$RUST_VERSION")-ortools-$(strip_v "$ORTOOLS_TAG")-highs-$(strip_v "$HIGHS_TAG")"

[[ "$TAG" != *latest* ]] || die "refusing to build a ':latest'-style tag"

if [[ "${PRINT_TAG:-0}" == "1" ]]; then
  echo "$TAG"
  exit 0
fi

PROJECT_ID="${PROJECT_ID:-reflective-labs}"
REPO="${REPO:-europe-west1-docker.pkg.dev/reflective-labs/apps}"
IMAGE="${REPO}/kenneth-backend-math-base:${TAG}"

cmd=(gcloud builds submit
  --project="$PROJECT_ID"
  --config="$RR_ROOT/docker/cloudbuild.math-base.yaml"
  --substitutions="_TAG=${TAG},_REPO=${REPO}"
  "$RR_ROOT/docker")

echo "==> math-base semantic tag: ${TAG}"
echo "==> image: ${IMAGE}"
if [[ "${DRY_RUN:-0}" == "1" ]]; then
  printf '%q ' "${cmd[@]}"; printf '\n'
  exit 0
fi
command -v gcloud >/dev/null 2>&1 || die "gcloud CLI required (or run DRY_RUN=1)"
"${cmd[@]}"
echo "Pushed ${IMAGE}"
echo "Downstream Dockerfiles must pull this semantic tag — never :latest."
