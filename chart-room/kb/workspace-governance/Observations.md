---
tags: [observations]
source: mixed
---
# Observations

Insights captured during sessions. Reviewed weekly during `/audit`. Graduated into permanent rules, skills, or kb pages — then removed from here.

---

## 2026-04-13

- The collaboration layer at `~/dev/reflective/bedrock-platform` benefits from its own git repo, separate from nested product repos. Workflow docs, KB, epics, milestones, and shared agent helpers change on a different cadence than product code.
- surrealdb 1→3, wasmtime 30→43, async-nats 0.40→0.47 all compiled without API changes despite major version bumps. Can be more aggressive with dep upgrades in converge.
- Drift audit grep for "unsafe" produced false positives from string literals. Audit skills need word-boundary matching or manual filtering for code-pattern searches.
- organism had 2.7GB of build artifacts in git history from legacy `target/` dirs committed before .gitignore was broad enough. New projects should start with comprehensive .gitignore from day one.
- Checkpoint skills had silently diverged across 6 projects. Need periodic skill checksum comparison to catch drift early.
- epic-brand was still on edition 2021 — easy to miss when projects are bootstrapped from older templates.
