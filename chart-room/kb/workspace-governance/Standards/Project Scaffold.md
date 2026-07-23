---
tags: [standard]
source: llm
---
# Project Scaffold Standard

Every active project under `~/dev/reflective/bedrock-platform/` must have:

## Required Files

| File | Purpose |
|------|---------|
| `CLAUDE.md` | Claude agent entrypoint with Session Scope block |
| `AGENTS.md` | Canonical project docs for all agents |
| `MILESTONES.md` | What ships and when, references `~/dev/reflective/bedrock-platform/EPIC.md` |
| `CHANGELOG.md` | Keep a Changelog format |
| `.gitignore` | Must cover: `target/`, `node_modules/`, `build/`, `dist/`, `.env`, `.DS_Store` |

## Required Directories

| Directory | Purpose |
|-----------|---------|
| `kb/` | Obsidian vault with `Home.md` as entry |
| `.claude/skills/` | 13 standard skills (audit, check, deploy, dev, done, fix, focus, next, pr, review, sync, ticket, wip) |
| `.claude/settings.local.json` | Permissions config |

## Knowledge Base Conventions

- Every `kb/` file must have YAML frontmatter with `source: human | llm | mixed`
- `INDEX.md` in kb/ catalogs key entities (optional for small projects, required for 10+ kb files)
- When reading a kb/ page and noticing stale info, update it in place (auto-enrichment)

## CLAUDE.md Must Reference

- `MILESTONES.md` — session scoping
- `CHANGELOG.md` — change tracking
- `~/dev/reflective/bedrock-platform/EPIC.md` — strategic context

## Checkpoint Skill Must

- Read `MILESTONES.md` (root, not kb/)
- Update `CHANGELOG.md` under `[Unreleased]`
- Reference `~/dev/reflective/bedrock-platform/EPIC.md` for epic signals

## GitHub

- converge, organism → `Reflective-Lab` org (public)
- All others → `kpernyer` (private)
- Remote must be configured and pushed
