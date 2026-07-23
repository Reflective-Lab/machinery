---
tags: [workflow]
source: mixed
---
# Daily Workflow Cheat Sheet

Use the same workflow vocabulary across Claude, Codex, and Gemini. The workflow name is canonical. `just` remains the deterministic repo automation layer.

Run `just cheatsheet` at the workspace root for a terminal-friendly copy of this page.

For Codex, name the workflow directly in plain text: `focus`, `run focus`, `check`, `done`, `audit`, `fix issue 42`, `review PR 5`. `$focus`-style notation also works when the installed skill name matches.

## Developer Hat (daily)

| Skill | just | What |
|-------|------|------|
| `/dev` | `just dev` | Start coding |
| `/check` | `just lint` | Lint + test. Am I clean? |
| `/fix 42` | — | Fix issue, branch, PR |
| `/pr` | — | Push and create PR |
| `/wip` | — | Save mid-work, push |

## Product Owner Hat (daily)

| Skill | just | What |
|-------|------|------|
| `/focus` | `just focus` | Session opener. Where are we? |
| `/next` | — | Pick from milestone |
| `/ticket` | — | File an issue |
| `/done` | — | End session. Progress, changelog, observations |

## VP Engineering Hat (weekly)

| Skill | What |
|-------|------|
| `/audit` | Full workspace review — security, compliance, drift, observations. One skill, one command |
| `/review 5` | Review a PR |

## DevOps Hat (as needed)

| Skill | just | What |
|-------|------|------|
| `/sync` | `just sync` | Pull, orient, PRs, issues |
| `/deploy` | `just deploy` | Push to prod (asks for confirmation) |

## The Habit

```text
Morning:    /focus → /sync → /next
Work:       /fix, /check, /pr
Evening:    /done
Monday:     /audit
```

## Legacy Aliases

| Old name | Canonical name | Note |
|----------|----------------|------|
| `/checkpoint` | `/done` | Codex may still back this with the `checkpoint` skill internally |
| `/quality` | `/check` | Codex may still back this with the `quality` skill internally |
| `/workspace-review` | `/audit` | Consolidated into one weekly audit workflow |
| `/security-audit` | `/audit` | Consolidated into one weekly audit workflow |
| `/compliance-audit` | `/audit` | Consolidated into one weekly audit workflow |
| `/drift-audit` | `/audit` | Consolidated into one weekly audit workflow |
