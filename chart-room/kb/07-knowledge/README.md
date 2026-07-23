# Knowledge Map & Decision Register

This domain is the memory anchor. It explains where knowledge belongs, which
decisions are active, and how older notes should be interpreted.

## Canonical Knowledge Domains

| Domain | Folder | Purpose |
|--------|--------|---------|
| Platform Vision & Operating Model | `01-platform/` | What the platform is and why it exists |
| Product & User Experience Architecture | `02-product/` | How users experience products and capabilities |
| Business & Commerce Architecture | `03-commerce/` | How money, entitlement, partner obligations, and growth work |
| System Architecture & Technical Platform | `04-architecture/` | Services, domains, APIs, data ownership, infrastructure |
| Engineering Principles & Development Guide | `05-engineering/` | How to build here |
| Operations & Runtime Handbook | `06-operations/` | Deployment, monitoring, environments, incidents |
| Knowledge Map & Decision Register | `07-knowledge/` | ADRs, glossary, ownership, superseded ideas |
| Roadmap & Strategic Direction | `08-roadmap/` | Future-facing plans and active bets |

## Documentation Rule

Every README, KB note, architecture document, configuration guide, runbook, test
description, ADR, or design note should trace back to one or more of these
domains. When conflicts arise, code and runtime behavior override
documentation, and these canonical domain docs override local narrative docs.

Only `08-roadmap/` should describe future architecture as future architecture.
All other domains describe current reality unless a section is explicitly
marked historical, superseded, or TODO.

## Active Decision Register

| Decision | Status | Current source |
|----------|--------|----------------|
| Root repo is a coordination and KB layer, not the implementation monorepo | Active | `README.md`, `.gitignore` |
| Eight canonical knowledge domains organize the KB | Active | this file, `KB/00-index.md` |
| Code is the source of truth over docs | Active | repository sync objective, this file |
| Product repos own UX and application consequences | Active | [[../04-architecture/current-system-map]] |
| Converge owns promotion/facts; Organism owns formation; Axiom owns translation | Active | [[../04-architecture/current-system-map]], [[../glossary]] |
| Commerce Rails owns commercial truth; Stripe is an adapter | Active | [[../03-commerce/README]] |
| `bedrock-platform/helms` is Helm / Outcome Workbench, not a separate product brand | Active | [[../04-architecture/current-system-map]] |
| Atelier Showcase and Arena Tests are root-level peer repos, not children of `stack/` | Active | `README.md`, [[../04-architecture/current-system-map]] |
| Helm naming migration is staged, not big-bang | Active | [[../outcome-workbench/kb/Architecture/Naming Migration Map]] |
| Helm handoff notes are historical unless revalidated | Superseded in part | [[../outcome-workbench/kb/Operations/Coordinator Handoff]], [[../outcome-workbench/kb/Architecture/Truths Layer]] |
| Project-local milestone logs must be checked against canonical roadmap, Linear, and code | Active | [[../08-roadmap/README]], `KB/08-roadmap/MASTERPLAN.md`, Linear |
| Old Converge L0-L4 spec set is historical where it conflicts with `converge` 3.9.1 crates | Superseded in part | [[../converge-business/specs/README]] |
| Motes Path A setup template is historical | Historical | [[../motes-path-a]] |

## Reconciliation Reports

- [[stack-reconciliation-2026-05-31]] - code-backed reconciliation across
  Bedrock, Mosaic, Atelier Showcase, and Arena Tests

## Ownership Map

- Platform doctrine and business story: `01-platform`, `KB/stack-narrative.md`
- Product surfaces: `02-product`, product-specific KBs
- Commerce and marketplace: `03-commerce`, Commerce Rails docs/code
- Technical architecture: `04-architecture`, code manifests, architecture notes
- Engineering practice: `05-engineering`, project `AGENTS.md`/`CLAUDE.md`/`CODEX.md`
- Operations: `06-operations`, deployment docs, runbooks, security docs
- Strategy and future plans: `08-roadmap`, `KB/08-roadmap/MASTERPLAN.md`, Linear

## Glossary

- [[../glossary|Canonical Glossary]]
- [[../business-tech-map|Business / Tech Map]]
- [[../workspace-governance/INDEX|Entity Index]]
