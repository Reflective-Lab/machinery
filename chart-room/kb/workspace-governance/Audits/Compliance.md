---
tags: [audit]
source: llm
---
# Git & Code Compliance Tracker

## What We Check
- .gitignore coverage (target/, node_modules/, build/, dist/, .svelte-kit/, .env, .DS_Store)
- Tracked artifacts that shouldn't be (build output, binaries, .obsidian/)
- Repo size (.git/) — flag anything over 50MB
- GitHub remote configured and pushed
- Scaffold compliance (CLAUDE.md, AGENTS.md, MILESTONES.md, CHANGELOG.md, kb/, skills)

## Audit History

### 2026-04-13 — Full Baseline

| Project | .git size | .gitignore | Tracked artifacts | Remote | Unpushed | Scaffold | Status |
|---------|-----------|-----------|-------------------|--------|----------|----------|--------|
| converge | 18 MB | **Gaps** (node_modules, build, dist, .svelte-kit) | **.obsidian/ tracked** | Reflective-Lab/converge | 0 | Pass | FAIL |
| wolfgang | 31 MB | Good | Clean | kpernyer/wolfgang | 0 | Pass | Pass |
| organism | 720 KB | **Gaps** (node_modules, build, dist, .svelte-kit) | Clean | Reflective-Lab/organism | 0 | Pass | FAIL |
| saas-killer | 22 MB | **Gaps** (build, dist) | Clean | kpernyer/crm.prio.ai | **2 unpushed** | Pass | FAIL |
| hackathon | 1.5 MB | **Gaps** (build, dist, .svelte-kit) | Clean | Reflective-Lab/hackathon | **3 unpushed** | Pass | FAIL |
| epic-brand | 5 MB | Good | Clean | kpernyer/epic-brand | 0 | Pass | Pass |
| wykkid-preso | 2.6 MB | **Gap** (dist) | Clean | kpernyer/wykkid-preso | 0 | Pass | FAIL |
| moosemen-writer | 316 KB | **No .gitignore** | Clean | kpernyer/moosemen-writer | 0 | Pass | FAIL |

**Action items:**
- moosemen-writer: create .gitignore
- converge: add node_modules/, build/, dist/, .svelte-kit/, .obsidian/ to .gitignore; untrack .obsidian/
- organism: add node_modules/, build/, dist/, .svelte-kit/ to .gitignore
- saas-killer: add build/, dist/; push 2 commits
- hackathon: add build/, dist/, .svelte-kit/; push 3 commits
- wykkid-preso: add dist/ to .gitignore
