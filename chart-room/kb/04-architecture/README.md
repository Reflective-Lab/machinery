# System Architecture & Technical Platform

This is the engineering map of the city: domains, services, APIs, boundaries,
data ownership, integrations, and infrastructure.

## Source Of Truth

When documents conflict, prefer:

1. running code
2. public interfaces and manifests
3. domain models
4. implemented architecture and boundaries
5. existing documentation
6. historical plans

## Current Domain Map

| Domain | Owner/home | Data or authority owned |
|--------|------------|-------------------------|
| Governance and facts | `bedrock-platform/converge` | admission, promotion, criteria, facts, protocol, runtime contracts |
| Intent and formation | `bedrock-platform/organism` | intent contracts, planning, adversarial review, simulation, formation selection |
| Truth compilation | `bedrock-platform/axiom` | JTBD/truth translation, verifier reports, calibration |
| Helm and application projection | `bedrock-platform/helms` and product repos | trust-transfer surfaces, app-specific business state, workbench views, user-facing consequence |
| Specialist capabilities | `mosaic-extensions` | reusable policy/model/port/solver/adapter/memory/analytics/SMT capabilities |
| Runtime operations | `runtime-runway` | auth, app host, storage, secrets, telemetry, deployment runtime |
| Commercial authority | `commerce-rails` | billing, entitlement, marketplace, payouts, reconciliation |
| Software factory | `build-depot` | quality/security/delivery signal normalization, repository health, incidents, and scorecard inputs |

## Public Interface Families

- Rust crates and workspaces are the primary internal API for Bedrock, Mosaic, Runtime Runway, Commerce Rails, and app projects.
- Helm still exposes proto packages under `proto/prio/*/v1`; namespace migration is planned but not complete.
- Web projects expose Firebase Hosting surfaces and project-local deploy workflows.
- Wolfgang exposes desktop, web, and backend surfaces from its own repo.

## Canonical Links

- [[current-system-map|Current System Map]]
- `../../build-depot/docs/architecture/software-factory-build-depot.md` — Build-Depot Software Factory Architecture
- [[applet-runtime-boundaries|Applet Runtime Boundaries]]
- [[runtime-injection-boundaries|Runtime and Injection Boundary Diagrams]]
- [[../deployment-and-infrastructure|Deployment and Infrastructure]]
- [[../security-review|Security Review]]
- [[../converge-business/README|Converge]]
- [[../organism-business/README|Organism]]
- [[../outcome-workbench/kb/Architecture/Current Architecture|Helm Current Architecture]]

## Per-Project Architecture

Scan-generated, refresh-safe notes. One overview per repo, one note per core module
(scan + synthesis via `/obsidian-architect`). Sentinel-marked generated blocks; re-running
the command updates only `<!-- @generated -->` regions.

### Platform foundation

- [[bedrock-platform/Architecture - Overview|bedrock-platform]] — Converge, Organism, Axiom, Helms (4 sub-projects, 59 crates total)
- [[mosaic-extensions/Architecture - Overview|mosaic-extensions]] — 8 specialist capability families (embassy, manifold, prism, ferrox, mnemos, arbiter, crucible, soter)

### Runtime + commercial

- [[runtime-runway/Architecture - Overview|runtime-runway]] — distribution, deployment, infrastructure (11 crates + ops/)
- [[commerce-rails/Architecture - Overview|commerce-rails]] — commercial-authority contracts + Stripe adapter (2 crates)
- [[lattice-mesh/Architecture - Overview|lattice-mesh]] — distributed execution mesh (planning stage, no source yet)

### Apps + surfaces

- [[marquee-apps/Architecture - Overview|marquee-apps]] — 10 thin JTBD products (catalyst, scout, fathom, atlas, quorum, tally, plumb, warden, triage, vouch)
- [[studio-apps/Architecture - Overview|studio-apps]] — 5 creative/research apps (Wolfgang, Wykkid, Inkling, Folio, Moosemen)
  - [[studio-apps/wolfgang-chat/Architecture - Overview|wolfgang-chat deep dive]] — 6 notes covering Tauri desktop + SvelteKit web + Cloud Run backend + shared core + Terraform infra
- [[mobile-apps/Architecture - Overview|mobile-apps]] — SwiftUI + Compose + Rust/UniFFI native lab; 1:1 candidates for marquee + studio
- [[beacon-sites/Architecture - Overview|beacon-sites]] — 6 SvelteKit + Firebase web properties under `converge-369ad`

### Validation + templates

- [[atelier-showcase/Architecture - Overview|atelier-showcase]] — 19 numbered tutorials + 23 scenario crates + reusable domain packs
- [[arena-tests/Architecture - Overview|arena-tests]] — cross-extension integration tests (test-only, dependency firewall)
- [[forge-templates/Architecture - Overview|forge-templates]] — workspace skeletons (converge-engagement, converge-extension)

## Decisions

- [[decisions/2026-06-07-retire-engagement-template|2026-06-07 — Retire converge-engagement template]] (retirement ADR — **swept** 2026-06-07)
- [[decisions/2026-06-07-tauri-gtk3-glib-risk|2026-06-07 — Tauri GTK3 / glib risk]]
- [[decisions/2026-06-06-applet-runtime-boundaries|2026-06-06 — Applet Runtime Boundaries]]
- [[decisions/2026-06-02-converge-runtime-retirement|2026-06-02 — converge-runtime retirement]] (retirement ADR — **swept** 2026-06-07)
- [[decisions/2026-05-23-runway-config-injection|2026-05-23 — Runway Config Injection]]

### Templates

- [[decisions/templates/retirement-adr|Retirement ADR Template]] — copy this when retiring / deprecating / replacing a named subsystem, crate, binary, deployment target, or canonical doc. Enforces the claim-sweep checklist.

## Boundary registry

[[current-system-map|current-system-map.md]] is the **canonical boundary registry**. Per-project READMEs and AGENTS.md files quote their boundary blockquote verbatim from `## Project Boundary Anchors` and link back to the anchor. If a README's quote drifts from the registry, update the registry first; then sweep the README per the [[decisions/templates/retirement-adr|retirement-ADR]] checklist (use that protocol for any boundary shift, not just retirements).

Worked examples — 4 READMEs already quote-and-link their anchor: `bedrock-platform/converge`, `runtime-runway`, `commerce-rails`, `mosaic-extensions`.

## Drift check

Run `python3 KB/scripts/drift-check.py` to compare the registry against per-project README Boundary blocks, Cargo.toml workspace versions, and forge-templates floor versions. Exit code: 0 (clean), 1 (warnings), 2 (critical drift or warnings+`--strict`). Source: [[scripts/drift-check|scripts/drift-check.py]] (header docstring has the full contract).

## Reference implementations

- [[reference-engagement|reference-engagement.md]] — `studio-apps/folio-editor` is the live reference for new engagements (replaces the archived `forge-templates/converge-engagement/` template).
- For new Mosaic extensions, use the still-active `forge-templates/converge-extension/` template (it ships CI + release-ritual enforcement + criterion baseline tooling not yet in any individual extension).
