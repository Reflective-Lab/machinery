# Retirement ADR Template

Copy this file to `KB/04-architecture/decisions/YYYY-MM-DD-<slug>-retirement.md` and fill it in. Every retirement / deprecation / replacement of a named subsystem, crate, binary, deployment target, or canonical doc lands as a retirement ADR. The point is **claim hygiene**: a retirement isn't done until every README, AGENTS.md, KB page, and registry entry that referenced the old thing has been swept.

The status progression for retirement ADRs:

- `proposed` — written, sweep not yet done
- `swept` — all citing files updated; old claim no longer appears anywhere it can mislead
- `archived` — old code path also deleted (optional — some retirements keep the old code as compat shells)

A retirement ADR can sit in `swept` indefinitely if the old code is intentionally preserved (compat shell, frozen-but-loadable, archival).

---

# Retirement ADR: <name of retired thing>

- Date: YYYY-MM-DD
- Status: proposed | swept | archived
- Decision type: retirement / deprecation / replacement
- Replaces: <link to prior ADR if any>
- Related: <links>

## What's being retired

Plain-language statement of the subsystem / crate / binary / doc / deployment target that is no longer canonical. One paragraph. If the code/file still exists in some form (compat shell, frozen artifact), say so explicitly.

## Why

Two-to-four sentences. The forcing function (release, audit, deprecation upstream, code-vs-doc divergence, etc.). Link to the LOG.md entry that recorded the underlying change if there is one.

## Old → New (the migration table)

| What it was | Where it lived | What it is now | Where it lives now |
|---|---|---|---|
| `<old name>` | `<path>` | `<new name>` (or: removed; or: compat-only shell) | `<new path or N/A>` |

Add a row for every artifact name that moved. Include public crate names, binary names, deployment service names, Firebase target names, proto package names, KB page titles — anything that someone might still search for.

## Claim sweep checklist

The retirement is **not done** until every box is ticked or explicitly marked N/A. Tick boxes by replacing `[ ]` with `[x]` and adding a SHA or commit-message reference where you fixed each claim.

### Registry

- [ ] `KB/04-architecture/current-system-map.md` — boundary anchor row updated, blockquote rewritten if responsibility moved.
- [ ] `KB/04-architecture/current-system-map.md` — review-notes section mentions the change if behaviour or pinning shifted.

### Per-project READMEs and AGENTS.md

For each repo that mentioned the retired thing — list them and tick:

- [ ] `<repo>/README.md` — old name removed or marked as "compat shell, see [[…]]".
- [ ] `<repo>/AGENTS.md` — same.
- [ ] `<repo>/CLAUDE.md` — same (if it carries doc claims, not just session config).

### KB pages

- [ ] `KB/04-architecture/<project>/Architecture - Overview.md` (the scan-generated note) — generated block re-runs cleanly via `/obsidian-architect`; any user-block claims about the retired thing updated.
- [ ] `KB/00-index.md` — links to retired thing removed or repointed.
- [ ] `KB/<other domain>/<page>.md` — sweep any non-architecture KB pages that named the retired thing. Use `grep -ri "<old name>" KB/` to find them.

### Deployment / infra

- [ ] Firebase Hosting rewrites or Cloud Run service names — removed or marked deprecated.
- [ ] Terraform / CI workflow references — removed.
- [ ] Pin tables, floor-version tables (e.g. `forge-templates/converge-engagement/README.md`) — updated.

### Code-side markers (optional but recommended)

- [ ] `// @deprecated` / `#[deprecated]` on the old binary/library, pointing at the new owner.
- [ ] Top-of-file comment on the old `main.rs` saying "compat shell only — production runtime lives at <path>."

## Consequences

- Listed concretely: what changes for callers, deployers, on-call.
- Any retained compat surface and its sunset condition (e.g. "compat shell removed when the last consumer migrates").
- What an `arena-tests` or smoke check should now assert (and what it should stop asserting).

## Follow-Ups

- Future work that fell out of the retirement but isn't blocking the sweep.
- The drift-check script (Move 2) should learn the old-name → new-name pair so future false claims get flagged.

## Sweep evidence

When the status flips to `swept`, add the commit SHA(s) that completed the sweep here and update `Date:` if the sweep landed later than the decision.

- Sweep landed: YYYY-MM-DD
- Commits: `<sha1>`, `<sha2>`, …
