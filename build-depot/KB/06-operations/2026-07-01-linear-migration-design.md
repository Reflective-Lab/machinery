---
name: linear-migration-design
description: Design for migrating MILESTONES.md and EPIC.md files to a single Linear team — epic-first structure, module labels, Git branch convention
metadata:
  type: project
  date: 2026-07-01
  status: approved — revised 2026-07-02, see Revision addendum at the end
---

# Linear Migration Design

Replace ~43 `MILESTONES.md` and 3 `EPIC.md` files spread across the monorepo with a
single Linear team. Agents and humans read Linear for current milestone state.

## Context

The repo has:
- 3 EPIC files: `./EPICS.md` (coord summary), `./bedrock-platform/EPIC.md` (canonical detail), `./mobile-apps/EPIC.md` (mobile-specific)
- 43 MILESTONES.md files across: atelier-showcase, beacon-sites, bedrock-platform, commerce-rails, forge-templates, KB, lattice-mesh, marquee-apps, mobile-apps, mosaic-extensions, runtime-runway, studio-apps

## Linear Structure

### Team
One team: **Reflective**

### Projects (22 total)

The epic is the primary organizational unit. Every issue belongs to exactly one project,
forcing the discipline of asking "which outcome does this advance?" before creating work.

Platform epics from `bedrock-platform/EPIC.md` and root `EPICS.md`:

| Project | Source |
|---|---|
| E1 — Converge: Publishable Platform | E1 |
| E2 — Organism: Reasons Before It Acts | E2 |
| E3 — Lattice: Execution Mesh | E3 |
| E4 — Marquee Apps: Helm Consumers | E4 |
| E5 — Helm: Trust Transfer Surface | E5 |
| E6 — Commerce Rails | E6 |
| E7 — Axiom: JTBD to Governed Contracts | E7 |
| E8 — Long-running HITL Convergence | E8 |
| E9 — Shared Fuzzy Substrate | E9 |
| E10 — Commercial Spikes | E10 |
| E11a — AI Director UX | root EPICS.md (2026-06-27) |
| E11b — Helm Coordination | bedrock-platform/EPIC.md |

Mobile epics from `mobile-apps/EPIC.md` (separate namespace, Mobile- prefix):

| Project | Mobile Epic |
|---|---|
| Mobile-Foundation | Epic 1 |
| Mobile-Quorum-Capture | Epic 2 |
| Mobile-Consent-Core | Epic 3 |
| Mobile-Domain-Reuse | Epic 4 |
| Mobile-Compute-Placement | Epic 5 |
| Mobile-Collaborative-UX | Epic 6 |
| Mobile-Android-Parity | Epic 7 |
| Mobile-Portfolio-Pattern | Epic 8 |
| Mobile-Studio-Local-First | Epic 9 |
| Mobile-Release-Privacy-Ops | Epic 10 |

### Labels — two tiers

Labels are metadata, not organization. The project (epic) is the org unit.

**`module:*` — which crates/repos are touched** (stack freely; informational):

`module:converge`, `module:organism`, `module:axiom`, `module:helms`,
`module:helm-coordination`, `module:commerce-rails`, `module:runtime-runway`,
`module:mosaic-extensions`, `module:arbiter-policy`, `module:marquee-apps`,
`module:studio-apps`, `module:mobile-apps`, `module:lattice`, `module:beacon-sites`,
`module:forge-templates`, `module:arena-tests`, `module:atelier-showcase`

**`type:*` — nature of the work**:

`type:spike`, `type:platform`, `type:infra`, `type:app`, `type:docs`

Extended lazily as issues are created. No need to pre-create every combination.

## Issue Mapping

### Named milestone → Linear issue

Each named milestone section (e.g., `### organism — v2.0 First-Class Formation Engine`)
becomes one issue. Milestones that span multiple modules get stacked `module:*` labels
and belong to one primary epic project.

| Milestone field | Linear field |
|---|---|
| Section heading | Issue title |
| `**Deadline:**` value | Due date |
| `**Epic:**` (first epic listed) | Project |
| All modules touched | `module:*` labels (one per module) |
| Narrative body | Issue description |

Multi-epic milestones: pick the single epic that most directly advances the outcome.
Secondary epics are recorded in the issue description — not as project assignments.

### Deliverables → sub-issues

Each `- [ ]` bullet inside a milestone becomes a sub-issue linked to the parent.
Checked items (`- [x]`) are skipped (open work only).

### Status

- No checked items → `Todo`
- Some checked items → `In Progress`

### Scope

Open/in-progress milestones only. Completed milestones are not imported.

### Markdown format variants

MILESTONES.md files use two heading styles:
- **Named sections** (`### converge — v3.7 Audit Trace Gradients`) — common in bedrock-platform
- **Numbered sections** (`## M1 — Shared infrastructure compiles`) — common in runtime-runway, mobile-apps

Both are treated the same: section heading → issue title, `- [ ]` bullets → sub-issues.
Sections with no open `- [ ]` bullets are skipped entirely.

## Git Branch and Worktree Convention

Linear issue IDs appear in branch names. This makes the epic/issue visible in every PR
and enables Linear's native GitHub integration to auto-link branches and PRs to issues.

### Branch naming

```
e{N}/{linear-id}-{slug}

# examples
e2/lin-42-organism-v2-formation-engine
e5/lin-67-helm-trust-surface-sse
e10/lin-83-quorum-spike1-billing
```

- Prefix `e{N}` = epic number — immediately clear which outcome the branch advances
- `{linear-id}` = Linear issue ID — auto-links PRs via GitHub integration
- `{slug}` = short kebab-case description

For docs-only branches (push directly to main per push policy), the convention is
optional — use it only when the docs change is scoped to a specific issue.

### Worktrees

One worktree per active issue:

```
worktrees/e{N}-lin{ID}/

# example
worktrees/e2-lin42/
```

Worktrees are short-lived — created when starting an issue, removed on merge.

### PR body

Every PR body includes the Linear issue URL in the summary section so GitHub's
integration closes the issue on merge:

```
Fixes: https://linear.app/reflective/issue/LIN-42
```

## Migration Script

**Location:** `tools/linear-import/import.py`
**Runtime:** Python 3, stdlib only (`http.client`, `json`, `re`, `pathlib`)
**API key:** `LINEAR_API_KEY` env var

### Phases

```
Phase 1 — Setup
  Create 22 projects
  Create seed label sets (module:* and type:*)

Phase 2 — Discover
  find . -name "MILESTONES.md" (43 files)

Phase 3 — Parse
  Extract: milestone sections, deadlines, epic refs, open deliverables
  Derive: module labels from directory path and heading prefix,
          status from checkbox ratio

Phase 4 — Import
  For each open milestone → create issue (project, labels, due date, description)
  For each open deliverable → create sub-issue linked to parent

Phase 5 — Report
  Print: N projects, N labels, N issues, N sub-issues created
  Print: skipped milestones (no epic ref, unrecognized format)
```

**Idempotency:** check-before-create on all resources. Safe to re-run.

## Phase 0: Consolidation Audit (pre-import)

Before the import script runs, a read-only audit pass over all 43 files produces a
consolidation report for human review and action. The import runs only on the
cleaned-up files.

### What the audit flags

1. **Stale open items** — `- [ ]` deliverables that appear already done based on:
   - Narrative text in the same file ("shipped", "done", "completed")
   - Matching entries in CHANGELOG or `Recently Completed` sections
   - Git commit messages that directly reference the deliverable

2. **Orphaned files** — MILESTONES.md files with:
   - No open deliverables (everything is `- [x]`)
   - No epic reference anywhere in the file
   - Content that duplicates a higher-level rollup file

3. **Epic assignment gaps** — milestone sections with no `**Epic:**` field; these
   cannot be assigned to a Linear project without one

4. **Single-epic discipline violations** — milestones referencing 3+ epics with no
   clear primary; flag for human assignment decision

5. **E11 conflict** — reconcile `EPICS.md` (AI Director UX) vs `bedrock-platform/EPIC.md`
   (Helm Coordination) for E11; surface the diff for human decision

### Output

`KB/06-operations/2026-07-01-linear-consolidation-report.md` — a structured report
with one entry per flagged item:

```
File:       bedrock-platform/MILESTONES.md
Milestone:  converge — v3.7 Audit Trace Gradients
Flag:       stale-open — CHANGELOG says v3.7 shipped 2026-06-30
Suggested:  mark all deliverables done; skip import
```

Human reviews the report, makes decisions (mark done / archive file / pick primary epic /
add epic ref), then runs the import.

## Agent Integration

### AGENTS.md addition

```
## Project Tracking

Milestone and epic state lives in Linear (workspace: Reflective Labs).
Read Linear issues for current milestone status — do not rely on
MILESTONES.md or EPIC.md files for open work; those files are archived.

Branch convention: e{N}/{linear-id}-{slug} (e.g. e5/lin-67-helm-trust-surface).
Worktrees: worktrees/e{N}-lin{ID}/. PR bodies must include the Linear issue URL.
```

### Per-project CLAUDE.md updates

Each file currently references `MILESTONES.md` for session scope. Replace with
the Linear project URL filtered to that sub-project's `module:*` label.

### File retirement

Add a deprecation header to each `MILESTONES.md` and `EPIC.md`:

```markdown
> **Archived 2026-07-01** — active tracking moved to Linear.
> This file is kept for historical context only.
```

Files are not deleted — git history and offline access are preserved.

## E11 Note

The two epic sources give E11 different definitions:
- `bedrock-platform/EPIC.md`: Helm Coordination (older)
- Root `EPICS.md`: AI Director UX (2026-06-27, newer)

Both are imported as separate projects (E11a / E11b) until the files are reconciled
in the consolidation audit. Reconciling the files is a Phase 0 deliverable.

> **Resolved 2026-07-02** — see the Revision addendum below: E11 = Helm
> Coordination, E12 = AI Director UX. No E11a/E11b projects exist.

---

## Revision 2026-07-02 (pre-import review)

The full review before running the import changed five things. The importer at
`tools/linear-import/import.py` implements this revision, not the original text
above. Dry-run result: **123 issues across 11 projects, 0 skipped**.

### 1. No sub-issue import

The original design created one sub-issue per open deliverable (~724 sub-issues).
Two problems: title-only global dedup meant identically-named deliverables
("Deliverables", "Infrastructure") would attach to the wrong parent on re-run,
and the total call volume (~1700 requests) exceeded Linear's ~1500 req/h limit.

Instead, deliverables stay as the markdown checklist inside the issue
description. A deliverable is promoted to a real sub-issue **by hand, when work
on it starts**. This keeps the import at ~140 calls and makes every issue
self-describing.

### 2. Projects pruned: 22 → 11, data-driven

Projects are created only for epics that actually have open milestones. E8 and
E12 currently have none, so they get no project until work exists. The ten
`Mobile-*` projects collapse into one project, **Mobile — Foundation to
Release**; all mobile issues land there in **Backlog** state (mobile is not
active work right now).

### 3. E11/E12 resolution

- **E11 = Helm Coordination** — `bedrock-platform/EPIC.md` is canonical.
- **E12 = AI Director UX** — the newer root-EPICS.md entry renumbered.
- Root `EPICS.md` updated 2026-07-02 with both rows and an explanatory note.
- `quorum-sense` M4 reassigned E10 → **E4** (it is market-vehicle work); the
  Atlas cross-app spike stays E10.

### 4. Title uniqueness and labels

- Issue titles are prefixed with their module (`quorum-sense: M4 — ...`) unless
  the heading already starts with it. Duplicate titles **abort the run** before
  any API call — idempotency is title-keyed, so uniqueness is a hard invariant.
- Secondary epics on multi-epic milestones become `epic:E*` labels (the primary
  epic is the project). `release:*` labels (green, per `KB/release-naming.md`)
  are reserved for release slices — created when a release is defined, never at
  import time.
- Module resolution skips `kb/Planning` vault nesting, so
  `mosaic-extensions/kb/Planning/MILESTONES.md` labels as
  `module:mosaic-extensions`, not `module:Planning`.

### 5. Rate-limit-safe, idempotent client

- `linear_client.py` retries 429/5xx with `Retry-After` or exponential backoff.
- All existing issues are prefetched in one paginated query (250/page);
  creation is check-before-create against that map. Re-running the import is
  safe and cheap.

### Branch policy revision (supersedes "one worktree per active issue")

Long-lived epic branches are rejected: with one developer and two agents
(Claude, Cursor), epic branches recreate the chaotic-merge problem at larger
scale. Instead:

- `main` is always green; issue branches are **short-lived**:
  `e{N}/lin-XX-slug`, merged within days.
- **One worktree per concurrent agent**, not per issue — e.g. the main checkout
  for Claude, one extra worktree for Cursor. Two agents never share a checkout.
- Epic traceability is a **Linear query, not a branch**: the `e{N}/` branch
  prefix plus Linear's GitHub integration links every PR to its issue and epic,
  so "everything that advanced E5" is answered in Linear, not in git.
