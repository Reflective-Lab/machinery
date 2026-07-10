#!/usr/bin/env bash
# RR D4 drift check — RP-NO-MATH-BASE-LATEST.
#
# Scans sibling Dockerfiles for kenneth-backend-math-base references pinned to a
# moving (`:latest`) or opaque (`@sha256`) tag instead of the human-readable
# semantic tag (v<rust>-ortools-<ortools>-highs-<highs>). A moving tag makes
# "which solver version is in prod?" unanswerable and silently changes builds;
# a bare digest is unreadable and hides the version triple the tag encodes.
#
# Exits non-zero (and lists the offenders) when any violation is found, so it
# can back the standard's drift check. Mirrors the role of the root repo's
# `just project-doctor`, scoped to the math-base rule.
#
# Usage:   ops/scripts/math-base-audit.sh
# Env:     AUDIT_ROOT  (default: the reflective workspace, RR's parent dir)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RR_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
AUDIT_ROOT="${AUDIT_ROOT:-$(cd "$RR_ROOT/.." && pwd)}"

# Moving (`:latest`) or opaque (`@sha256`) pin on the math-base image.
PATTERN='kenneth-backend-math-base(:latest|@sha256)'

echo "math-base-audit: scanning Dockerfiles under $AUDIT_ROOT"
# Prefer ripgrep (fast, but must override VCS-ignore so nested sibling repos are
# scanned). Fall back to grep with dir excludes where rg is unavailable.
if command -v rg >/dev/null 2>&1; then
  hits="$(rg -n --no-ignore-vcs --hidden \
    -g 'Dockerfile*' -g '!**/target/**' -g '!**/node_modules/**' -g '!**/.git/**' \
    "$PATTERN" "$AUDIT_ROOT" 2>/dev/null || true)"
else
  hits="$(grep -rEn --include='Dockerfile*' \
    --exclude-dir=target --exclude-dir=node_modules --exclude-dir=.git \
    "$PATTERN" "$AUDIT_ROOT" 2>/dev/null || true)"
fi

if [[ -n "$hits" ]]; then
  echo "VIOLATION — math-base must be pinned to a semantic tag, not :latest/@sha256:" >&2
  echo "$hits" >&2
  echo "" >&2
  echo "Fix: pull v<rust>-ortools-<ortools>-highs-<highs> (see: ops/scripts/build-math-base.sh PRINT_TAG=1)." >&2
  exit 1
fi

echo "math-base-audit: OK — no :latest/@sha256 math-base pins found."
