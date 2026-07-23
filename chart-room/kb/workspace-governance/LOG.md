---
tags: [log]
source: llm
---
# Knowledge Base Mutation Log

Tracks all changes to the workspace kb/. Most recent first.

---

## 2026-04-15

- **Updated** `Standards/Conventions.md` — added the canonical Helm/Axiom/Organism/Converge/Providers layer model and cross-project dependency rules
- **Updated** `Home.md` — standards link now calls out the shared layer model

## 2026-04-13

- **Updated** `Observations.md` — captured the need for a separate workspace-governance git repo above nested product repos
- **Created** `Justfile` (workspace) — added `just cheatsheet` terminal helper for the canonical workflow vocabulary
- **Updated** `Workflow/Cheat Sheet.md` — documented `just cheatsheet` at workspace root
- **Updated** `Workflow/Cheat Sheet.md` — switched to canonical workflow names by hat, documented Codex use of the same terms, added legacy alias mapping
- **Updated** `CLAUDE.md` (workspace) — weekly workspace review now refers to `/audit`
- **Updated** `Observations.md` — weekly review cadence now refers to `/audit`
- **Updated** `Standards/Conventions.md` — canonical workflow names standardized across Claude, Codex, and Gemini
- **Consolidated** skills 21 → 13 across all 8 projects: removed jj, parallel, backlog, roadmap, status, feedback, ship, merge; renamed checkpoint→done, quality→check; consolidated 4 audit skills→1
- **Created** `Workflow/Cheat Sheet.md` — daily habit by hat (developer, PO, VP eng, devops)
- **Created** `Observations.md` — session insights pending weekly graduation
- **Updated** all 8 checkpoint skills — step 5 asks "Anything surprising?" and appends to Observations.md
- **Updated** `workspace-review` skill — step 4 reviews observations and proposes graduation
- **Updated** `Home.md` — added Observations.md link
- **Added** `source:` provenance tags to all existing kb files (all `llm`)
- **Updated** `Home.md` — added quick access links to INDEX.md and LOG.md
- **Updated** `Standards/Project Scaffold.md` — added kb conventions (provenance, auto-enrichment)
- **Updated** `CLAUDE.md` (workspace) — added Knowledge Base Conventions section
- **Created** `INDEX.md` — entity catalog across all projects, crates, services, infra, domain concepts
- **Created** `LOG.md` — this file, tracks all kb mutations
- **Updated** `Audits/Security.md` — converge vulns 21→1 after dep bumps
- **Updated** `Audits/Architecture Drift.md` — unsafe audit corrected (false positives), edition/version fixes recorded
- **Updated** `Audits/Compliance.md` — all .gitignore gaps fixed, all projects pushed
- **Updated** `Standards/Conventions.md` — added cloud/IAC rules, Justfile standards, CI/CD safety, skills/process consistency
- **Updated** `History/Audit Log.md` — full baseline audit recorded
- **Created** `Audits/Security.md` — baseline security audit
- **Created** `Audits/Compliance.md` — baseline compliance audit
- **Created** `Audits/Architecture Drift.md` — baseline drift audit
- **Created** `Standards/Project Scaffold.md` — required files/dirs for every project
- **Created** `Standards/Conventions.md` — cross-project rules
- **Created** `History/Audit Log.md` — timestamped audit record
- **Created** `Home.md` — workspace kb entry point
