#!/usr/bin/env bash
# shim-doctor — RP-SHIM-FIRST-CLASS drift check.
#
# Policy (AGENTS.md > Shims ... are first-class debt, 2026-07-03): no
# workaround lands silently. Every shim carries an inline marker
#   SHIM(QF-YYYY-MM-DD-NN, expires: YYYY-MM-DD): reason
# backed by a ledger finding; expiries are promises.
#
# Checks (scope v1: root-repo tracked files; fleet rollout is the
# standard's residual):
#   1. Every SHIM( marker parses, its QF id exists in QUALITY_BACKLOG.md,
#      and its expiry date is today or later.
#   2. Smell list — patterns that are shims by construction must have a
#      SHIM( marker within 5 lines: allow-shlib-undefined, bare #[ignore],
#      commented-out #[test].
#
# Exits 1 on any failure; cron/CI-safe.

set -uo pipefail
cd "$(dirname "$0")/../.."

fail=0
report() { echo "✗ $1"; fail=1; }
today=$(date +%Y-%m-%d)

files=$(git ls-files | grep -vE '^(KB/05-engineering/standards/first-class-shims\.md|scripts/factory/shim-doctor\.sh|AGENTS\.md|QUALITY_BACKLOG\.md|SKILLS\.md)$')

# 1 — validate every SHIM( marker
while IFS=: read -r f ln line; do
  [ -z "$f" ] && continue
  if [[ "$line" =~ SHIM\((QF-[0-9]{4}-[0-9]{2}-[0-9]{2}-[0-9]{2}),\ *expires:\ *([0-9]{4}-[0-9]{2}-[0-9]{2})\) ]]; then
    qid="${BASH_REMATCH[1]}"; exp="${BASH_REMATCH[2]}"
    grep -q "#### $qid\|### $qid" QUALITY_BACKLOG.md \
      || report "check 1: $f:$ln — $qid not found in QUALITY_BACKLOG.md"
    [[ "$exp" < "$today" ]] \
      && report "check 1: $f:$ln — SHIM expired $exp ($qid): remove it or re-justify in the finding"
  else
    report "check 1: $f:$ln — malformed SHIM marker (need SHIM(QF-..., expires: YYYY-MM-DD))"
  fi
done < <(echo "$files" | xargs grep -n "SHIM(QF-[0-9]" 2>/dev/null)

# 2 — smell list requires a nearby SHIM marker
smells='Wl,--allow-shlib-undefined|^\s*//\s*#\[test\]|#\[ignore\]\s*$'
while IFS=: read -r f ln line; do
  [ -z "$f" ] && continue
  start=$((ln > 5 ? ln - 5 : 1))
  sed -n "${start},$((ln + 5))p" "$f" | grep -q "SHIM(" \
    || report "check 2: $f:$ln — unmarked shim smell: $(echo "$line" | head -c 80)"
done < <(echo "$files" | xargs grep -nE "$smells" 2>/dev/null)

if [ "$fail" -eq 0 ]; then
  echo "✓ shim-doctor: all shims are first-class (markers valid, none expired, no unmarked smells)"
fi
exit "$fail"
