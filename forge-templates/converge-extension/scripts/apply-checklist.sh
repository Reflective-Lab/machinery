#!/usr/bin/env bash
# Apply the Extension Release Checklist to a target extension repo.
#
# Usage:
#   apply-checklist.sh <target-repo-path> <extension-name>
#
# Behaviour:
#   - Never overwrites a file that already exists in the target.
#   - Substitutes {{extension}} with <extension-name> in copied files only.
#   - Reports a summary of what was added vs skipped.
#
# Idempotent: running twice is a no-op.

set -euo pipefail

if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <target-repo-path> <extension-name>" >&2
    exit 2
fi

target="$1"
ext="$2"
template="$(cd "$(dirname "$0")/.." && pwd)"

if [ ! -d "$target" ]; then
    echo "FAIL: target $target is not a directory" >&2
    exit 1
fi

added=()
skipped=()

copy_if_missing() {
    local rel="$1"
    local dst="$target/$rel"
    local src="$template/$rel"
    if [ -e "$dst" ]; then
        skipped+=("$rel")
        return 0
    fi
    mkdir -p "$(dirname "$dst")"
    # Substitute {{extension}} in copied file (in-place via temp)
    sed "s/{{extension}}/${ext}/g" "$src" > "$dst"
    chmod --reference="$src" "$dst" 2>/dev/null || true
    added+=("$rel")
}

# Files we always want every extension to have.
copy_if_missing "deny.toml"
copy_if_missing ".github/workflows/ci.yml"
copy_if_missing ".github/workflows/coverage.yml"
copy_if_missing ".github/workflows/security.yml"
copy_if_missing ".github/workflows/stability.yml"
copy_if_missing "scripts/extract-criterion-baseline.py"
copy_if_missing "kb/Home.md"
copy_if_missing "kb/INDEX.md"
copy_if_missing "kb/LOG.md"
copy_if_missing "kb/Architecture/Surface.md"
copy_if_missing "kb/Building/Getting Started.md"
copy_if_missing "kb/Building/Release Commands.md"
copy_if_missing "kb/History/CHANGELOG.md"
copy_if_missing "kb/Planning/MILESTONES.md"

# Justfile, README, CLAUDE.md, AGENTS.md: only add when missing. Don't
# clobber repo-specific recipes/docs. The fold-in is a separate, manual
# step the operator decides per repo.
copy_if_missing "Justfile"
copy_if_missing "README.md"
copy_if_missing "CLAUDE.md"
copy_if_missing "AGENTS.md"

echo "── apply-checklist: $ext → $target ──"
echo "added (${#added[@]}):"
for f in "${added[@]}"; do echo "  + $f"; done
echo "skipped (${#skipped[@]}):"
for f in "${skipped[@]}"; do echo "  = $f"; done
