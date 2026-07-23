# Masterplan — Reflective Labs

**Last updated:** 2026-07-02
**Strategic goal:** Make intent computable. Make convergence trustworthy. Earn autonomy.
**Operating model:** Single developer, multiple agents. Tracking lives in Linear (see `AGENTS.md` → Project Tracking). `main` is always green; work happens on short-lived issue branches (`e{N}/lin-XX-slug`), one worktree per concurrent agent, no branches left behind. Releases are named cities (`KB/release-naming.md`) — outward promises that slice across epics. The train moves together — no component gets ahead of where it's actually used.
**Quality system:** `QUALITY_BACKLOG.md`, `KB/05-engineering/standards/`,
`KB/06-operations/factory-scorecard.md`, and `machinery/build-depot/` form the
quality/security/reliable-delivery loop. Build-Depot is the graph-backed
software-factory control plane and canonical source for factory doctrine and
quality-gate semantics.

This is a roadmap document. Current reality lives in the canonical KB domains,
especially `KB/04-architecture/` and `KB/07-knowledge/`. Future-facing material
belongs here or under `KB/08-roadmap/`, not in architecture docs.

---

## The Bet

Organizations should be able to express what must hold, then rely on a governed system to compile, evaluate, execute, and converge that intent — without making humans orchestrate every step.

The core pipeline:

```
Truth Document (.truths)
  → Axiom: validate & compile → IntentPacket
  → Organism: formation selection & adversarial planning
  → Converge: fixed-point execution through Suggestors
  → governed facts & projections
  → ExperienceStore events
  → better planning priors (feeds back to Organism)
```

**What we are NOT building:** SaaS feature parity, generic AI agents, a UX-first product, model wrappers, or free-running automation.

---

## Single-Developer Rule: The Train

One developer. Work advances along one track. The system moves as a unit.

**Rules:**
1. Do not push a component further than where it is actually used in a running app.
2. Before adding features to Organism, check that Helms (or an app) actually calls Organism.
3. Before adding Axiom features, check that something downstream actually consumes IntentPacket.
4. No feature-branch archaeology. Work on `main` (or `next`). Merge or drop.
5. New capability only earns its keep when it is wired through to a surface a human can touch.

The measure of progress is not "what was added to the codebase" — it is "what can a human do today that they couldn't do yesterday, end-to-end."

---

## Gap Analysis — What Works End-to-End vs. What Is Isolated

### Actually working or code-backed today

| Path | Where it runs | Evidence |
|------|--------------|---------|
| Helm executable truths | `framework/bedrock/foundation/helm/crates/workbench-backend` | `qualify-inbound-lead`, `submit-expense-report`, `activate-subscription`, `refill-prepaid-ai-credits` are supported in code |
| Axiom truth/JTBD tooling | `framework/bedrock/foundation/axiom` | `axiom-truth` crate version `0.15.0`, `cz` binary |
| Organism planning/runtime contracts | `framework/bedrock/foundation/organism` | `1.9.1` workspace with intent, planning, adversarial, simulation, learning, runtime, catalog, notes, and dynamics crates |
| Converge provider/platform API train | `framework/bedrock/foundation/converge` | `3.9.1` workspace with 11 crates |
| Public web/portal deployment config | `sites/` | Firebase targets and workflows for core sites plus apps.reflective.se |
| Wolfgang product scaffold | `applications/studio-apps/wolfgang-chat` | desktop, web, backend, personas, KB ingestion, auth, and deployment scaffolding |
| Software factory control plane | `machinery/build-depot/` | Bun/TypeScript Trigger.dev worker plus Omnigraph schema for repository, finding, incident, RP, and standard facts |

### Built but not connected end-to-end

| Gap | What exists | What's missing | Blocking what |
|-----|-------------|----------------|--------------|
| **Axiom → Organism → Converge in a product path** | Axiom, Organism, and Converge contracts exist | A current Helm or app path does not yet run the full sequence from human job to formation to governed promotion | E2, E5, E7 |
| **Helm full truth catalog** | 23 registered truth definitions | Only four are currently executable through `workbench-backend` | Product proof depth |
| **Runtime Runway alignment** | Runtime Runway workspace exists at `3.4.1` | It still pins Converge `v3.4.0` release tags while local Converge is `3.9.1` | Runtime/app consumption of latest platform train |
| **Commerce Rails consumption** | Contracts and Stripe adapter boundary exist | Apps do not yet consume the full commercial authority loop end-to-end | Paid app launch and partner marketplace |
| **ExperienceStore → Organism priors** | ExperienceStore logs events; spec defined | ExperienceStore events do NOT feed back into Organism planning priors | Learning loop (E2, E5) |
| **Long-running HITL resume** | Architecture direction exists | Ledger-backed resume, authority epochs, and sharded context proofs are not complete | E8 |
| **Lattice / distributed mesh** | Lattice folder exists | Full distributed execution mesh is not product-critical yet | E3 |

### The honest picture

Helm has concrete executable truth paths, but the deep pipeline is
not fully wired. A human can use Helm/application paths, but the current
product proof does not yet flow through Axiom → Organism → Converge with full
lineage and operator trust events.

**The most important gap:** the full planning loop — intent → adversarial →
simulation → commit — is not the default path for a real user-facing product
job.

**The second most important gap:** ExperienceStore does not feed back. The learning loop exists in spec; the data is not flowing.

---

## The Shortest Path to End-to-End (The Train's Next Station)

The target: a human opens a Helm/product surface, asks for a consequential
job, and the full pipeline runs — Axiom compiles the job, Organism selects and
argues over the formation, Converge executes and promotes facts, and the result
is visible with evidence and approval state.

Steps in sequence (each must work before the next):

```
1. Select one current Helm truth as the proof path.
2. Compile its job contract through Axiom rather than hand-shaped local inputs.
3. Route formation selection through Organism instead of local static binding.
4. Execute through Converge and project the result back into Helm or the thin app.
5. Record operator and runtime events so the learning loop has real input.
6. Add Commerce Rails only when the proof touches paid entitlement or billing.
```

Do not advance Organism Stage 4 features beyond step 2 until Helms actually calls Organism (step 3). Do not build ExperienceStore recall until the pipeline runs (step 3 produces the events).

---

## Epics — Big Movements

Epics are the major bets. Linear Projects are the live source of epic state;
this table is a roadmap snapshot from 2026-07-02.

| Epic | Statement | Status |
|------|-----------|--------|
| **E1** | Converge is a publishable platform | In flight; local crate train is `3.9.1` |
| **E2** | Organism reasons before it acts | In flight; `1.9.1`, not fully product-wired |
| **E3** | Reflective Lattice is the execution mesh | Not started as product-critical path |
| **E4** | Concrete apps live outside platform core and consume shared surfaces | In flight; app workspaces exist, standard boundary still being proven |
| **E5** | Helms is the trust transfer surface | In flight; four executable truth paths, deep loop incomplete |
| **E6** | Commerce Rails makes the platform commercially usable | In flight; `machinery/commerce-rails/` is canonical home, partner loop incomplete |
| **E7** | Axiom translates human jobs into governed runtime contracts | In flight; `axiom-truth` is `0.15.0`, full marquee proof missing |
| **E8** | Long-running HITL convergence crosses process boundaries | Planned/in flight at architecture level |
| **E9** | Shared fuzzy substrate across regulated apps | Planned/in flight through app pressure |
| **E10** | Commercial spikes turn platform capability into packaged outcomes | In flight |

---

## Current State — Quick Board

### Platform

| System | Version | State | Deadline |
|--------|---------|-------|----------|
| **Converge** | v3.9.1 | Local provider/platform API train | None in this doc |
| **Organism** | v1.9.1 | Local crate train; not fully product-wired | None in this doc |
| **Axiom** | v0.15.0 | Truth/JTBD tooling and `cz` CLI | None in this doc |
| **Runtime Runway** | v3.4.1 | Runtime ops workspace; still pins Converge `v3.4.0` release tags | Align when needed by app path |
| **Commerce Rails** | v0.1.0 | Contracts and Stripe adapter boundary | Partner loop not complete |
| **Helm / Helms** | v0.2.0 | 35-crate trust-transfer workspace; four executable truths | Expand proof path |

### Engagements

| Engagement | State | Immediate action |
|-----------|-------|-----------------|
| **Epic Brand** | Proposal sent; rejected | No active follow-up |
| **Fabric** | Pitch prep done | Park until partner follow-up |
| **Kantar** | M3 in progress | Bain return thesis |
| **Monterro** | KB done | Wire Converge + Organism into DD pipeline |
| **Newspaper** | M3 & M4 in progress | Close M4; decide M5 priorities |
| **Whatfix** | M1 done | Run first convergent DD |

### Apps

| App | State | Gap |
|-----|-------|-----|
| **Wolfgang** | Desktop/web/backend scaffold exists | Shared commerce consumption and packaging |
| **apps.reflective.se** | Portal deploy workflow exists | Runtime/product ownership boundaries should stay explicit |
| **Marquee apps** | Multiple app workspaces exist | Package a sellable proof without duplicating platform cores |

---

## Priorities — Next 14 Days

These are in order. Do not move to the next until the current is done or intentionally parked.

1. **Choose one Helm or thin-app proof path** — preferably a current executable truth, then wire the missing Axiom/Organism/Converge path rather than adding new isolated platform features.
2. **Keep Wolfgang commercial boundary clean** — product UX can move, but durable billing/entitlement authority belongs in Commerce Rails.
3. **Align Runtime Runway only when a consumer needs the newer Converge train** — do not churn the runtime without an app path demanding it.
4. **Use apps.reflective.se as packaging pressure** — it should prove app delivery without hiding product ownership boundaries.

---

## 90-Day Horizon (by 2026-08-10)

| Target | Why |
|--------|-----|
| Replacement paid engagement pipeline | Epic Brand proposal was rejected; revenue pipeline needs a new signed path |
| One convergent DD run (Whatfix or Monterro) | First platform-native engagement output |
| Organism wired to Helms — one Truth flows through the full pipeline | E2/E5 train station reached |
| Converge v3.9.1 consumed by a real app path | Stable public API surface |
| Wolfgang Stage 1 actually shipped | Revenue unlock |
| Websites deployed | Credibility unlock (if there's inbound) |

---

## Known Risks

| Risk | Severity | Action |
|------|----------|--------|
| Epic Brand proposal rejected | Medium | Archive learning; no active follow-up unless the buyer reopens |
| Axiom/Organism/Converge proof still too fixture/local-binding heavy | Medium | Pick one Helm or thin-app truth and wire the real path |
| Organism not connected to any real app | **High** | This is the core gap; don't add more Organism features until it's wired |
| Wolfgang overdue — no paying customer | **High** | QA + Stripe first |
| ExperienceStore not looping | Medium | Can't loop until pipeline runs; don't work on it yet |
| Cedar at commit boundary missing | Medium | Don't work on it until pipeline runs |
| Lattice / Ferrox not started | Low now | Don't start — premature |
| Runtime Runway pinned to old Converge release tags | Medium | Align only when a consuming app needs the newer contract |
| Newspaper M5 scope very wide | Medium | Pick 1–2 tracks; skip the rest for now |

---

## Time Allocation Guide

Priority order when choosing where to spend time:

1. **Revenue-gating actions** — Proposals, billing, engagement closing
2. **End-to-end pipeline wiring** — The path that connects Helms → Organism → Converge through a real Truth Document
3. **Converge** — Public contracts must stay stable (E1)
4. **Active engagements** — Whatfix (convergent DD), Monterro, Newspaper (in that order)
5. **Axiom** — Add only what the selected proof path needs
6. **Apps** — Wolfgang first; others are low priority
7. **Websites** — Last; only when there's inbound pressure

---

## Planning Sources

- Linear Projects and issues are the live source for epics, milestones, and
  deliverables.
- Archived `MILESTONES.md` and `EPIC.md` files are historical evidence only.
- Release promises live in dated files under `KB/08-roadmap/` and use
  `release:<city>` labels in Linear.
