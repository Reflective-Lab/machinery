# Shared Workflow Playbooks

The canonical process playbooks for this workspace live in
`.claude/skills/<name>/SKILL.md`. They are plain markdown and are the **single
source of truth for how work flows** — session open/close, branching, PRs,
WIP handoffs, reviews, and tickets. Policy (what must be true) lives in
`AGENTS.md`; playbooks (how to do it) live here.

## How each tool consumes them

- **Claude Code** discovers them automatically as project skills and invokes
  them natively (`/focus`, `/pr`, …).
- **Codex** finds the same files via the `.codex/skills` symlink
  (→ `.claude/skills`). If skill discovery is unavailable, read the relevant
  `SKILL.md` before performing the matching action.
- **Cursor** reads root and nested `AGENTS.md` automatically (Settings → Rules).
  Do **not** duplicate policy into `.cursor/rules/` — if a future Cursor version
  stops auto-reading `AGENTS.md`, add a single `.cursor/rules/agents.mdc` with
  `alwaysApply: true` and body `Read and follow AGENTS.md` only. Playbooks are
  discovered via the `.cursor/skills` symlink (→ `.claude/skills`), same files
  Claude uses. Before branching, opening a PR, saving WIP, or closing a session,
  read the matching playbook and follow it.

**Precedence:** these playbooks override any user-global or tool-built-in
skill of the same or similar name (e.g. a generic `ship`, `merge`, or
`checkpoint` skill). If a playbook conflicts with `AGENTS.md`, `AGENTS.md`
wins — file a B-tier finding to reconcile.

## Skill mapping (Claude ↔ Cursor ↔ Codex)

Repo playbooks are **1:1** across Claude and Cursor — same directory, same
`SKILL.md`, invoked by name. Codex user-global skills (`~/.codex/skills/`) and
Cursor built-in skills (`~/.cursor/skills-cursor/`) are **not** process
playbooks; use the repo playbook when one exists.

| Repo playbook | Claude | Cursor | Codex user-global alias | Cursor built-in alias |
|---|---|---|---|---|
| `focus` | `/focus` | read `.claude/skills/focus/SKILL.md` | `sync` (general repos only) | — |
| `branch` | `/branch` | read playbook | — | — |
| `next` | `/next` | read playbook | — | — |
| `check` | `/check` | read playbook | `quality`, `status` | — |
| `test` | `/test` | read playbook | — | — |
| `experiment` | `/experiment` | read playbook | — | — |
| `wip` | `/wip` | read playbook | — | — |
| `pr` | `/pr` | read playbook | `ship` (partial — repo playbook wins) | `split-to-prs` (split only) |
| `review` | `/review` | read playbook | — | `review-bugbot`, `review-security` (subagents) |
| `merge-cleanup` | `/merge-cleanup` | read playbook | — | `babysit` (CI/comment loop only) |
| `ticket` | `/ticket` | read playbook | `feedback` (partial) | — |
| `done` | `/done` | read playbook | `ship` (partial) | — |

Codex-only user-global skills with **no repo playbook** (`audit`, `backlog`,
`deploy`, `dev`, `jj`, `parallel`) stay available for cross-repo or
infrastructure work; they do not override Reflective process inside this
workspace.

## Software Factory Skills

The Build-Depot repo carries repo-local factory skills for the quality,
security, and reliable-delivery workflows:

- `build-depot/.claude/skills/build-depot-quality/SKILL.md`
- `build-depot/.claude/skills/build-depot-security/SKILL.md`
- `build-depot/.claude/skills/build-depot-delivery/SKILL.md`

Use those when the work is specifically about the Build-Depot software-factory
control plane. Use the root playbooks below for workspace process.

## Catalog

| Playbook | When | What it does |
|---|---|---|
| `focus` | Start of every session | Current issue, epic, deadline, open deliverables — one Linear query |
| `branch` | Before touching code | Cut/switch to short-lived issue branch `e{N}/{issue-id}-{slug}` from fresh `main` |
| `next` | Choosing work | Remaining tasks for the current issue/milestone from Linear |
| `check` | Before claiming done | Lint + type check + test — am I clean? |
| `test` | Adding coverage | Rust test taxonomy: unit, negative, integration, property, compile-fail, soak |
| `experiment` | Uncertain approach | Formulate hypothesis, run experiment, record outcome |
| `wip` | Pausing / switching device or agent | Save and push work-in-progress on the issue branch |
| `pr` | Work complete | Push issue branch, open PR to `main` with Linear issue URL in body |
| `review` | Reviewing a PR | Security, correctness, style, ops — findings first |
| `merge-cleanup` | After a PR merges | Delete issue branch locally and remotely, prune stale merged branches |
| `ticket` | Capturing work | Create an issue detailed enough for an agent to execute |
| `done` | End of session | Progress, changelog, observations — session closeout per `AGENTS.md` |

## Conventions the playbooks assume

- Project tracking is **Linear** (team RFL); `MILESTONES.md` / `EPIC.md` files
  are archived history.
- `main` is always green; issue branches are short-lived
  (`e{N}/{issue-id}-{slug}`).
- **One worktree per concurrent agent** — an operating habit, not config. The
  main checkout belongs to one agent (usually Claude); a second agent (e.g.
  Cursor) works in its own worktree on its own issue branch. Two agents never
  share a checkout.
- Docs-only changes go directly to `main`; anything else goes through a PR.

## Changing a playbook

Playbooks are versioned files: change them via PR like code. Do not fork a
playbook into a tool-specific location (`~/.codex/skills`, `~/.cursor`,
user-global `~/.claude/skills`) — that is how drift starts. Tool-specific
copies may only be symlinks or pointers to this directory.
