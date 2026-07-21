---
source: llm
---

# Cross-agent instruction files

## What

Every fleet repo has one canonical agent entrypoint: `AGENTS.md`. Tool-specific
entry files (`CLAUDE.md`, `CODEX.md`, `GEMINI.md`) are **pointers, not forks** —
they reference `AGENTS.md`, stay ≤ 25 lines, and may carry only genuinely
tool-specific notes (tool preferences, skill locations), never policy or
project state.

Workflow playbooks (the how-to for branch/PR/WIP/session flows) live
repo-tracked in `.claude/skills/*/SKILL.md`, cataloged in root `SKILLS.md`.
Tool-specific skill copies are symlinks or pointers only (`.codex/skills` and
`.cursor/skills` → `.claude/skills`); user-global skill directories
(`~/.claude/skills`, `~/.codex/skills`, `~/.cursor/skills`) never carry
project process.

## Why

By 2026-07-02, Claude, Codex, and Cursor were executing three different
processes: Codex's user-global skills still read the retired `MILESTONES.md`
flow, `marquee-apps` policy lived in a Claude-only file the other agents never
read, and three `CLAUDE.md` files pointed at a `~/CLAUDE.md` that does not
exist. Forked instruction files drift silently — each fork is read by exactly
one agent, so no session ever sees the disagreement. See `QF-2026-07-02-01`.

## How to check

`just agents-doctor` (`scripts/factory/agents-doctor.sh`) — four checks:
every nested git repo has `AGENTS.md`; tool entry files reference `AGENTS.md`
and stay ≤ 25 lines; no references to `~/CLAUDE.md` / `~/dev/CLAUDE.md`;
`MILESTONES.md`/`EPIC.md` mentions are marked archived (Linear is the source
of truth). Exit 1 on any failure; cron-safe.
