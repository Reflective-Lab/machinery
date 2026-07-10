---
tags: [standards, marquee-apps, definition-of-done, milestones]
source: claude
date: 2026-06-07
status: per-app-pilot
---

# Milestone Definition of Done (per marquee-app pilot)

`marquee-apps/quorum-sense/` is piloting a per-app Definition of Done
convention for `MILESTONES.md` items: every `[ ]`/`[x]` carries structured
sub-bullets that `just milestone-done <id>` walks as eight numbered
accumulating checks.

**Decision doc (authoritative spec):**
`~/dev/reflective/marquee-apps/quorum-sense/kb/Decisions/2026-06-07-definition-of-done.md`

**Implementation:**
`~/dev/reflective/marquee-apps/quorum-sense/Justfile`
(`milestone-done`, `milestone-done-all`, `_dod-parse`)

**Status:** per-app pilot. If the convention survives two `~/dev/reflective`
cycles without revision (next review 2026-06-21), promote this file from a
pointer to a full workspace standard and clone the recipe into the other 9
marquee-apps. See *Future tightening* in the decision doc.
