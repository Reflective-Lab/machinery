#!/usr/bin/env bash
# agents-doctor — drift check for cross-agent instruction files.
#
# Guards the standard set on 2026-07-02 (cross-agent alignment):
#   1. Every nested git repo has an AGENTS.md (canonical agent entrypoint).
#   2. Tool-specific entry files (CLAUDE.md, CODEX.md, GEMINI.md) are
#      pointers, not forks: they must reference AGENTS.md and stay small
#      (<= 25 lines; tool-specific notes are fine, policy is not).
#   3. No agent file references the nonexistent ~/CLAUDE.md or
#      ~/dev/CLAUDE.md.
#   4. Any MILESTONES.md/EPIC.md mention in an agent file must mark it
#      archived/historical (Linear is the source of truth since 2026-07-02).
#
# Exits 1 on any failure, so it works from cron and CI.

set -euo pipefail
cd "$(dirname "$0")/../.."

fail=0
report() { echo "✗ $1"; fail=1; }

agent_files() { # all agent entry files in root + nested repos (depth 1)
  ls AGENTS.md CLAUDE.md CODEX.md GEMINI.md 2>/dev/null
  for d in */; do
    case "$d" in KB/) continue;; esac # Obsidian vault — KB/CLAUDE.md is personal context, not a repo entrypoint
    for f in AGENTS.md CLAUDE.md CODEX.md GEMINI.md; do
      [ -f "$d$f" ] && echo "$d$f"
    done
  done
}

# 1 — every nested git repo has AGENTS.md
for d in */; do
  if [ -d "$d.git" ] && [ ! -f "${d}AGENTS.md" ]; then
    report "check 1: $d is a git repo without AGENTS.md"
  fi
done

# 2 — tool entry files are pointers to AGENTS.md, not forks
while IFS= read -r f; do
  case "$f" in *AGENTS.md) continue;; esac
  lines=$(wc -l < "$f" | tr -d ' ')
  if ! grep -q "AGENTS.md" "$f"; then
    report "check 2: $f does not reference AGENTS.md"
  fi
  if [ "$lines" -gt 25 ]; then
    report "check 2: $f is $lines lines — looks like a fork, not a pointer (max 25)"
  fi
done < <(agent_files)

# 3 — no dead ~/CLAUDE.md or ~/dev/CLAUDE.md references
while IFS= read -r f; do
  if grep -nE '~/CLAUDE\.md|~/dev/CLAUDE\.md' "$f" >/dev/null; then
    report "check 3: $f references nonexistent ~/CLAUDE.md or ~/dev/CLAUDE.md"
  fi
done < <(agent_files)

# 4 — MILESTONES/EPIC file mentions must be marked archived/historical.
# Sentences wrap, so judge a ±1-line window around each mention.
while IFS= read -r f; do
  while IFS= read -r line; do
    awk -v n="$line" 'NR>=n-1 && NR<=n+1' "$f" \
      | grep -qiE 'archiv|historical|do not|not rely' \
      || report "check 4: $f:$line treats milestone files as live: $(sed -n "${line}p" "$f")"
  done < <(grep -nE 'MILESTONES\.md|EPIC\.md' "$f" | cut -d: -f1 || true)
done < <(agent_files)

if [ "$fail" -eq 0 ]; then
  echo "✓ agents-doctor: all agent instruction files aligned"
fi
exit "$fail"
