---
name: linear-consolidation-grouped
description: Bulk-decision summary of the 148-finding Linear consolidation report — grouped for yes/no decisions rather than item-by-item review
metadata:
  type: operations
  date: 2026-07-01
  status: decided 2026-07-02 — all groups resolved, see inline decisions
  source: 2026-07-01-linear-consolidation-report.md
---

# Linear Consolidation — Grouped Decision Summary

> Derived from the 148-finding consolidation report (2026-07-01).  
> Each group has a one-line **Bulk action**. Respond YES / NO / MODIFY to the whole group.  
> Individual items that need separate decisions are called out explicitly.

**Totals: 7 orphaned files · 136 no-epic milestones · 5 multi-epic milestones**

---

## Quick-Reference Table

| # | Group | Type | Items | Suggested Bulk Action |
|---|---|---|---|---|
| 1A | KB (outcome-workbench orphan) | Orphaned | 1 | Archive, skip import |
| 1B | bedrock-platform/axiom | Orphaned | 1 | Archive, skip import |
| 1C | marquee-apps/warden-compliance | Orphaned | 1 | Archive, skip import |
| 1D | mosaic-extensions (4 files) | Orphaned | 4 | Archive all, skip import |
| 2A | KB/monterro | No-epic | 2 | Skip import (CRM, not platform) |
| 2B | KB/outcome-workbench/kb/Planning | No-epic | 6 | Skip import (likely duplicates bedrock-platform/helms) |
| 2C | atelier-showcase | No-epic | 3 | Assign all → E7 |
| 2D | beacon-sites (4 sites) | No-epic | 4 | Assign per module: axioms→E7, converge→E1, helms→E5, organism→E2 |
| 2E | bedrock-platform root | No-epic | 1 | Assign → E3 |
| 2F | bedrock-platform/helms | No-epic | 7 | Assign all → E5 |
| 2G | bedrock-platform/organism | No-epic | 11 | Assign all → E2 |
| 2H | commerce-rails | No-epic | 2 | Assign all → E6 |
| 2I | forge-templates | No-epic | 2 | Assign all → E1 |
| 2J | lattice-mesh | No-epic | 2 | Assign all → E3 |
| 2K | marquee-apps root (rollup) | No-epic | 9 | Skip import (covered by sub-directory files) |
| 2L | marquee-apps/atlas-integration | No-epic | 5 | Assign all → E10 |
| 2M | marquee-apps/catalyst-biz | No-epic | 5 | Assign all → E10 |
| 2N | marquee-apps/plumb-execution | No-epic | 4 | Assign all → E4 |
| 2O | marquee-apps/quorum-sense | No-epic | 2 | Assign all → E10 |
| 2P | marquee-apps/scout-sourcing | No-epic | 9 | Assign all → E4 |
| 2Q | marquee-apps/tally-escrow | No-epic | 4 | Assign all → E4 |
| 2R | marquee-apps/vouch-lending | No-epic | 3 | Assign all → E4 |
| 2S | mobile-apps | No-epic | 8 | Assign each to matching Mobile-* epic by content |
| 2T | mosaic-extensions (5 with open work) | No-epic | 5 | Assign 4 → E9, 1 (mnemos) → E1 |
| 2U | runtime-runway | No-epic | 2 | Assign all → E3 |
| 2V | studio-apps/folio-editor | No-epic | 21 | Assign all → E10 (Spike 2 newspaper track) |
| 2W | studio-apps/inkling-notes | No-epic | 1 | Assign → E4 |
| 2X | studio-apps/moosemen-writer | No-epic | 4 | Assign all → E4 |
| 2Y | studio-apps/wolfgang-chat | No-epic | 12 | Assign all → E4 (Wolfgang is highest-priority marquee app) |
| 2Z | studio-apps/wykkid-preso | No-epic | 1 | Assign → E4 |
| 3A | bedrock-platform commercial spikes (×3) | Multi-epic | 3 | Assign all → E10 primary |
| 3B | bedrock-platform/converge v3.5 & v3.7 | Multi-epic | 2 | Assign all → E1 primary |

---

## Part 1: Orphaned Files (7 files)

These files have no open deliverables. "Archive all" in a group means: add the deprecation header
(`> **Archived 2026-07-01** — active tracking moved to Linear.`) to each file and skip import.

---

### 1A · KB — 1 file

| File |
|---|
| `KB/outcome-workbench/MILESTONES.md` |

"Archive all" means: add deprecation header to `KB/outcome-workbench/MILESTONES.md`; skip import.
Note: the inner `KB/outcome-workbench/kb/Planning/MILESTONES.md` has open items — that is group 2B, not this one.

**Bulk action:** Archive the root file, skip import.
- [x] Decision: YES — archived (banner applied), skipped

---

### 1B · bedrock-platform — 1 file

| File |
|---|
| `bedrock-platform/axiom/MILESTONES.md` |

"Archive all" means: add deprecation header to `bedrock-platform/axiom/MILESTONES.md`; skip import.
Axiom milestone tracking likely lives in the root `bedrock-platform/MILESTONES.md` sections instead.

**Bulk action:** Archive, skip import.
- [x] Decision: YES — archived (banner applied), skipped

---

### 1C · marquee-apps — 1 file

| File |
|---|
| `marquee-apps/warden-compliance/MILESTONES.md` |

"Archive all" means: add deprecation header; skip import.
Note: the root `marquee-apps/MILESTONES.md` still contains a `warden-compliance — M0` entry (group 2K). If that root file is also skipped, no warden-compliance work enters Linear at all — confirm that is intentional.

**Bulk action:** Archive, skip import; verify warden-compliance has no open work before proceeding.
- [x] Decision: YES — archived; no open warden-compliance work enters Linear (root rollup also archived, group 2K)

---

### 1D · mosaic-extensions — 4 files

| File |
|---|
| `mosaic-extensions/embassy-ports/kb/Planning/MILESTONES.md` |
| `mosaic-extensions/kb/Planning/MILESTONES.md` |
| `mosaic-extensions/prism-analytics/kb/Planning/MILESTONES.md` |
| `mosaic-extensions/soter-smt/kb/Planning/MILESTONES.md` |

"Archive all" means: add deprecation header to all four files; skip import.
These four extensions (embassy-ports, prism-analytics, soter-smt, plus the mosaic root) have no open
deliverables — they are complete or paused.

**Bulk action:** Archive all 4, skip import.
- [x] Decision: YES — all 4 archived, skipped

---

## Part 2: No-Epic Milestones (136 items across 30 files)

Each group lists the milestone titles, states the reasoning for the epic choice, and gives a bulk action.
After a YES decision, the import script adds the `**Epic:**` field and uses the `module:*` + `type:*` labels
shown.

---

### 2A · KB/monterro — 2 items

**File:** `KB/monterro/MILESTONES.md`

| Milestone title |
|---|
| Current: Kickoff — "End-to-end demo" |
| Next: Portfolio Intelligence — "Monterro manager questions" |

These are client-facing consulting/sales milestones for the Monterro PE engagement — not platform
development work. There is no corresponding platform epic for deal pipeline management. Importing them
into Linear alongside platform milestones would pollute the engineering backlog.

**Bulk action:** Skip import — track in KB/CRM only, not in Linear.
- [x] Decision: YES — skipped; stays in KB/CRM (file archived)

---

### 2B · KB/outcome-workbench/kb/Planning — 6 items

**File:** `KB/outcome-workbench/kb/Planning/MILESTONES.md`

| Milestone title |
|---|
| Deliverables |
| Stretch: Mobile Daily Priorities App |
| Notes application (full implementation) |
| Expenses & Receipts (OCR integration) |
| Deliverables |
| Deliverables |

⚠️ **Likely duplicate.** These titles match `bedrock-platform/helms/kb/Planning/MILESTONES.md` exactly.
The KB/outcome-workbench path appears to be an older or shadow copy of the helms planning file.
Importing both would create duplicate issues.

**Bulk action:** Skip import — verify against bedrock-platform/helms; if content is identical, archive this file.
- [x] Decision: YES — confirmed duplicate of bedrock-platform/helms; archived, skipped

---

### 2C · atelier-showcase — 3 items

**File:** `atelier-showcase/kb/Planning/MILESTONES.md`

| Milestone title |
|---|
| Acceptance criteria for any scenario added under v1.1.0 |
| Proposed scenarios |
| Definition of done for v1.1.0 |

Atelier-showcase is the Axiom scenario test harness. The milestones are about defining and gating
Axiom scenarios — natural fit for E7. Labels: `module:atelier-showcase`, `type:platform`.

**Bulk action:** Assign all 3 → **E7** (Axiom: JTBD to Governed Contracts).
- [x] Decision: YES — all 3 → E7

---

### 2D · beacon-sites — 4 items (4 files)

Each file maps to a different platform component, so four different epics apply.

| File | Milestone title | Suggested epic |
|---|---|---|
| `beacon-sites/www.axioms.zone/MILESTONES.md` | v0.2 — Content & Polish | **E7** (Axiom) |
| `beacon-sites/www.converge.zone/MILESTONES.md` | v0.3 — Series Complete, Player & Depth | **E1** (Converge) |
| `beacon-sites/www.helms.zone/MILESTONES.md` | v0.2 — Content & Polish | **E5** (Helm) |
| `beacon-sites/www.organism.zone/MILESTONES.md` | v0.2 — Content & Polish | **E2** (Organism) |

Labels for all: `module:beacon-sites`, `type:app`.

**Bulk action:** Assign per-module (4 different epics as shown above); add shared labels to each.
- [x] Decision: YES — per-module epics as shown

---

### 2E · bedrock-platform root — 1 item

**File:** `bedrock-platform/MILESTONES.md`

| Milestone title |
|---|
| reflective/runtime-runway — v3.4 Distribution & Infra |

This milestone lives in the bedrock-platform root rollup but describes runtime-runway distribution
infrastructure — the deployment layer, which is E3 (Lattice: Execution Mesh).
Labels: `module:runtime-runway`, `type:infra`.

**Bulk action:** Assign → **E3** (Lattice: Execution Mesh).
- [x] Decision: YES — → E3

---

### 2F · bedrock-platform/helms — 7 items

**File:** `bedrock-platform/helms/kb/Planning/MILESTONES.md`

| Milestone title |
|---|
| Deliverables |
| Stretch: Mobile Daily Priorities App |
| Notes application (full implementation) |
| Expenses & Receipts (OCR integration) |
| Deliverables |
| Deliverables |
| Deliverables |

Note: "Deliverables" appears four times — these are section containers for structured sub-tasks, not
a naming problem. They will produce four separate sub-issue trees under E5.
Labels: `module:helms`, `type:platform`.

**Bulk action:** Assign all 7 → **E5** (Helm: Trust Transfer Surface).
- [x] Decision: MODIFIED — → E5, but H3 subsections demoted to H4 (2026-07-02) so each H2 stage imports as ONE issue with its checklist, not 7 fragments. Also fixed malformed '**Epic: E5 (...)**' line.

---

### 2G · bedrock-platform/organism — 11 items

**File:** `bedrock-platform/organism/MILESTONES.md`

| Milestone title |
|---|
| Phase A — parallel-safe (ships tonight in 1.5.0 + Converge 3.8.0) |
| Phase B — blocked on Converge Authority Slice (ships in fast-follow 1.5.x) |
| Sync release with Converge 3.8.0 |
| Per-role suggestor scoring |
| Public API contract |
| Axiom proof |
| Helm and application proof |
| Learning loop |
| Extension composition proof |
| App |
| Runtime maturity |

All are Organism delivery milestones. Labels: `module:organism`, `type:platform`.

**Bulk action:** Assign all 11 → **E2** (Organism: Reasons Before It Acts).
- [x] Decision: YES — all → E2

---

### 2H · commerce-rails — 2 items

**File:** `commerce-rails/MILESTONES.md`

| Milestone title |
|---|
| M2a — Entitlement-Gate Proof |
| M2b — Partner Piggy-Back Loop |

Direct match — these are the commerce/billing infrastructure milestones. Labels: `module:commerce-rails`, `type:platform`.

**Bulk action:** Assign both → **E6** (Commerce Rails).
- [x] Decision: YES — both → E6

---

### 2I · forge-templates — 2 items

**File (1):** `forge-templates/converge-engagement/MILESTONES.md` — M0 — Skeleton  
**File (2):** `forge-templates/converge-extension/kb/Planning/MILESTONES.md` — Current: v0.1 — _release theme_

Both are Converge ecosystem templates — scaffolding for new Converge engagements and extensions.
Labels: `module:converge`, `type:platform`.

**Bulk action:** Assign both → **E1** (Converge: Publishable Platform).
- [x] Decision: YES — both → E1

---

### 2J · lattice-mesh — 2 items

**File:** `lattice-mesh/kb/Planning/MILESTONES.md`

| Milestone title |
|---|
| Planned: v0.1 — Mesh Foundation |
| Backlog |

Direct match to E3. Labels: `module:lattice`, `type:platform`.

**Bulk action:** Assign both → **E3** (Lattice: Execution Mesh).
- [x] Decision: YES — both → E3

---

### 2K · marquee-apps root (rollup) — 9 items

**File:** `marquee-apps/MILESTONES.md`

| Milestone title |
|---|
| Immediate commercial priority — Spike 1 Quorum Full Vertical |
| Immediate portfolio priority — Runtime Runway execution container |
| catalyst — Runtime Runway Proof "Full stack app" |
| wolfgang — Stage 1 "People can use it" |
| wolfgang — Stage 1.5 "Converge powers the reasoning" (next) |
| moosemen-writer — Stage 0 "Project Scaffold" |
| quorum-sense — M0 "Skeleton" |
| warden-compliance — M0 "Compliance Control Plane" |
| Scenarios |

⚠️ This is a rollup/index file. Each app listed here has its own sub-directory MILESTONES.md
(groups 2L–2R below) that covers the same work in more detail. Importing both creates duplicate issues.
The "Scenarios" entry has no counterpart and may warrant a standalone issue, but it is too vague to
assign without inspection.

**Bulk action:** Skip import — covered by sub-directory files; inspect "Scenarios" entry manually before deciding.
- [x] Decision: YES — skipped (archived); 'Scenarios' entry inspected — covered by per-app files, no standalone issue needed

---

### 2L · marquee-apps/atlas-integration — 5 items

**File:** `marquee-apps/atlas-integration/MILESTONES.md`

| Milestone title |
|---|
| Current Spike 1 Priority — REAL LIVE Evidence Pairing |
| M1 — Evidence Graph |
| M2 — Governed Extraction Candidate |
| M3 — Adapter PR Draft |
| M4 — Fuzzy Integration Reasoning |

Atlas integration is framed as a commercial spike ("Spike 1 Priority", "REAL LIVE").
Labels: `module:marquee-apps`, `type:spike`.

**Bulk action:** Assign all 5 → **E10** (Commercial Spikes).
- [x] Decision: YES — all 5 → E10

---

### 2M · marquee-apps/catalyst-biz — 5 items

**File:** `marquee-apps/catalyst-biz/MILESTONES.md`

| Milestone title |
|---|
| M2 Scope — Approval Gates & HITL Forms |
| M2.5 — Full Stack Proof |
| Later: M3 — Demo & Recording |
| Later: M4 — Polish & Distribution |
| Later: M5 — Fuzzy JTBD Scoring |

Catalyst is one of the three named commercial spike apps (Spike 1 in the multi-epic milestones).
Labels: `module:marquee-apps`, `type:spike`.

**Bulk action:** Assign all 5 → **E10** (Commercial Spikes).
- [x] Decision: YES — all 5 → E10

---

### 2N · marquee-apps/plumb-execution — 4 items

**File:** `marquee-apps/plumb-execution/MILESTONES.md`

| Milestone title |
|---|
| M0 — Skeleton |
| M1 — Closed Loop |
| M2 — Fuzzy Drift Detection |
| M3 — Behavior Shift Layer |

Plumb-execution is an execution-monitoring app consuming the Helm/Converge stack — fits E4.
Labels: `module:marquee-apps`, `type:app`.

**Bulk action:** Assign all 4 → **E4** (Marquee Apps: Helm Consumers).
- [x] Decision: YES — all 4 → E4

---

### 2O · marquee-apps/quorum-sense — 2 items

**File:** `marquee-apps/quorum-sense/MILESTONES.md`

| Milestone title |
|---|
| Priority — Atlas Cross-App Spike (REAL LIVE) |
| M4 — Product Proof + Live Run Surface |

Explicitly framed as a spike ("Atlas Cross-App Spike", "REAL LIVE").
Labels: `module:marquee-apps`, `type:spike`.

**Bulk action:** Assign both → **E10** (Commercial Spikes).
- [x] Decision: MODIFIED (2026-07-02) — Atlas Cross-App Spike → E10; M4 Product Proof → E4 (market-vehicle work, not spike)

---

### 2P · marquee-apps/scout-sourcing — 9 items

**File:** `marquee-apps/scout-sourcing/MILESTONES.md`

| Milestone title |
|---|
| Repository Scaffold |
| One Truth |
| Product Architecture |
| Vendor Selection Core |
| Desktop Release |
| Quality Gate |
| Next: Cloud Web App |
| Later: Production Backend |
| Later: Desktop Distribution |

Scout-sourcing is a standalone vendor-selection desktop/cloud app — a Marquee App consumer.
Labels: `module:marquee-apps`, `type:app`.

**Bulk action:** Assign all 9 → **E4** (Marquee Apps: Helm Consumers).
- [x] Decision: YES — all 9 → E4

---

### 2Q · marquee-apps/tally-escrow — 4 items

**File:** `marquee-apps/tally-escrow/MILESTONES.md`

| Milestone title |
|---|
| M1 — Domain Wedge: Web Domain Handoff |
| M2 — Helms Adjudication + Credential Custody |
| M3 — Conditional & Milestone Agreements |
| M4 — Fuzzy Release Predicates |

Tally-escrow is an application consuming the Helms/Converge stack for agreement management.
It could also argue E6 (Commerce Rails) given the financial/escrow domain, but the milestone
content is about Helm adjudication — it is an app consumer. Labels: `module:marquee-apps`, `type:app`.

**Bulk action:** Assign all 4 → **E4** (Marquee Apps: Helm Consumers).  
*(Override to E6 if you consider tally-escrow part of the commerce infrastructure rather than an app.)*
- [x] Decision: YES — all 4 → E4 (E6 override declined; content is Helm adjudication)

---

### 2R · marquee-apps/vouch-lending — 3 items

**File:** `marquee-apps/vouch-lending/MILESTONES.md`

| Milestone title |
|---|
| M1 — Fuzzy Credit Decisioning |
| M2 — Fair-Lending Cohort Analysis |
| M3 — Borrower-Facing Explanation Surface |

Same reasoning as tally-escrow: lending app consuming the Converge/Organism reasoning stack.
Labels: `module:marquee-apps`, `type:app`.

**Bulk action:** Assign all 3 → **E4** (Marquee Apps: Helm Consumers).  
*(Override to E6 if you consider vouch-lending part of commerce infrastructure.)*
- [x] Decision: YES — all 3 → E4

---

### 2S · mobile-apps — 8 items

**File:** `mobile-apps/MILESTONES.md`

Each numbered milestone maps to a Mobile-* epic by content. M-numbers do not directly equal
Mobile-Epic numbers — match by title semantics:

| Milestone title | Suggested Mobile-* epic |
|---|---|
| M2 — Quorum FFI And Canonical Domain Seam | Mobile-Quorum-Capture |
| M4 — Shared Consent, Offline Queue, And Sync Core | Mobile-Consent-Core |
| M5 — Android Parity | Mobile-Android-Parity |
| M6 — Capability-Aware Compute Placement | Mobile-Compute-Placement |
| M7 — Realtime Collaboration UX | Mobile-Collaborative-UX |
| M8 — Portfolio App Pattern | Mobile-Portfolio-Pattern |
| M9 — Release, Privacy, And Operations | Mobile-Release-Privacy-Ops |
| Backlog — Not Yet Scheduled | Mobile-Foundation |

Labels: `module:mobile-apps`, `type:app` for all.

**Bulk action:** Assign each to the corresponding Mobile-* epic as shown above (8 different epics).
- [x] Decision: MODIFIED (2026-07-02) — M* epic refs kept in files, but ALL mobile issues land in the single shared project 'Mobile — Foundation to Release' in Backlog state (projects pruned per design revision)

---

### 2T · mosaic-extensions (open work) — 5 items

Five mosaic-extensions have open milestones (not orphaned). Four are shared substrate libraries → E9.
One (mnemos-knowledge) explicitly names Converge → E1.

| File | Milestone title | Suggested epic |
|---|---|---|
| `mosaic-extensions/arbiter-policy/kb/Planning/MILESTONES.md` | Next: Cedar Analysis Lane | **E9** |
| `mosaic-extensions/crucible-models/kb/Planning/MILESTONES.md` | v1.0.0 — Release Checklist | **E9** |
| `mosaic-extensions/ferrox-solvers/kb/Planning/MILESTONES.md` | Next: Native Solver Assurance Hardening | **E9** |
| `mosaic-extensions/manifold-adapters/kb/Planning/MILESTONES.md` | Open: pull-driven | **E9** |
| `mosaic-extensions/mnemos-knowledge/kb/Planning/MILESTONES.md` | Current: v1.0.0 — Converge 3.8.1 Knowledge Foundation | **E1** |

Labels: `module:mosaic-extensions`, `type:platform` for all.

**Bulk action:** Assign 4 (arbiter-policy, crucible-models, ferrox-solvers, manifold-adapters) → E9 (Shared Fuzzy Substrate); assign 1 (mnemos-knowledge) → E1 (Converge).
- [x] Decision: YES — 4 → E9, mnemos-knowledge → E1

---

### 2U · runtime-runway — 2 items

**File:** `runtime-runway/MILESTONES.md`

| Milestone title |
|---|
| M2 — GCP project ready for production traffic |
| M6 — Remaining app backends + distribution signing |

Runtime-runway is the cloud execution/deployment layer — the Lattice execution mesh.
Labels: `module:runtime-runway`, `type:infra`.

**Bulk action:** Assign both → **E3** (Lattice: Execution Mesh).
- [x] Decision: YES — both → E3 (**Epic:** fields written 2026-07-02)

---

### 2V · studio-apps/folio-editor — 21 items

**File:** `studio-apps/folio-editor/MILESTONES.md`

| Milestone title |
|---|
| Loop 1 - New Market Explorer to Ingest Seed |
| Loop 2 - Market Newspaper Identity to Five Markdown Files |
| Loop 3 - Live Sources and Identity to Daily Edition |
| Promise Track 1 - Human Room, Shared Truth, and Local Engagement |
| Relationship Track 1 - Individual Newspaper and Adaptive Layout |
| Relationship Track 2 - Adaptive Text, Catch-Up, and Missed Context |
| Relationship Track 3 - Multi-Role Reader, Contributor, and Question Loop |
| Relationship Track 4 - Community Learning and Newspaper Teaching |
| Relationship Track 5 - Get To Know The Reader |
| Value Track 1 - Contribution Value Ledger |
| Advertising Track 1 - Contextual Reverse Auction and Ad Placement |
| Vision Track 1 - Trust Product and Six-Dimension Value Proof |
| Vision Track 2 - Ecosystem Role Contracts and Balance Checks |
| Vision Track 3 - Pilot Economics and Market Validation |
| Vision Track 4 - Defensibility Graphs and Market Memory |
| Vision Track 5 - Demo, Comparables, and Fundraising Assets |
| Vision Track 6 - Publisher Integration and Partnership Surface |
| Cross-Loop Milestone: Demo Readiness |
| Scope |
| Existing Technical Baseline |
| Track milestones |

Folio-editor is the Newspaper / Local Information app — it is identified as **Spike 2** in the
multi-epic bedrock-platform milestones. The Vision and demo-readiness tracks are commercial
validation work. Labels: `module:studio-apps`, `type:spike`.

**Bulk action:** Assign all 21 → **E10** (Commercial Spikes).
- [x] Decision: YES — all → E10

---

### 2W · studio-apps/inkling-notes — 1 item

**File:** `studio-apps/inkling-notes/kb/Planning/MILESTONES.md`

| Milestone title |
|---|
| M1 - Standalone Desktop Repository |

Labels: `module:studio-apps`, `type:app`.

**Bulk action:** Assign → **E4** (Marquee Apps: Helm Consumers).
- [x] Decision: YES — → E4

---

### 2X · studio-apps/moosemen-writer — 4 items

**File:** `studio-apps/moosemen-writer/MILESTONES.md`

| Milestone title |
|---|
| Current: Stage 0 — Project Scaffold |
| Stage 1 — "The writer's notebook" |
| Stage 2 — "Write the damn book" |
| Stage 3 — "Ship it" |

Labels: `module:studio-apps`, `type:app`.

**Bulk action:** Assign all 4 → **E4** (Marquee Apps: Helm Consumers).
- [x] Decision: YES — all 4 → E4

---

### 2Y · studio-apps/wolfgang-chat — 12 items

**File:** `studio-apps/wolfgang-chat/MILESTONES.md`

The file covers three stages (Stage 1, 1.5, 2), each with section titles that repeat:

| Milestone title | Stage |
|---|---|
| Product deliverables (desktop) | 1 |
| Web presence (storefront) | 1 |
| Infrastructure | 1 |
| Converge pipeline integration | 1 |
| Organism integration | 1 |
| Converge.zone primitives driven | 1 |
| Product deliverables | 1.5 |
| Infrastructure | 1.5 |
| Converge.zone primitives driven | 1.5 |
| Product deliverables | 2 |
| Infrastructure | 2 |
| Converge.zone primitives driven | 2 |

⭐ Wolfgang Stage 1 is a stated goal in KB/CLAUDE.md — treat this as highest-priority within E4.
Labels: `module:studio-apps`, `type:app`.

**Bulk action:** Assign all 12 → **E4** (Marquee Apps: Helm Consumers); flag Stage 1 items as high priority.
- [x] Decision: MODIFIED (2026-07-02) — → E4 (erroneous E3 stamps corrected); H3 subsections demoted to H4 so each stage imports as one issue with its checklist

---

### 2Z · studio-apps/wykkid-preso — 1 item

**File:** `studio-apps/wykkid-preso/MILESTONES.md`

| Milestone title |
|---|
| Current: Presentation Ready |

Labels: `module:studio-apps`, `type:app`.

**Bulk action:** Assign → **E4** (Marquee Apps: Helm Consumers).
- [x] Decision: YES — → E4

---

## Part 3: Multi-Epic Milestones (5 items)

These must be handled individually — each requires a single primary epic. Secondary epics are preserved
in the issue description, not as project assignments.

---

### 3A · bedrock-platform/MILESTONES.md — Commercial Spikes group

All three are explicitly "commercial spikes" — E10 is the natural primary for all. Secondary epics
capture which modules are in scope.

#### Spike 1 — Quorum Full Vertical (auth + billing + production)
- Current epics: **E10, E4, E5, E6, E7** (5 epics)
- Suggested primary: **E10**
- Secondary in description: E4, E5, E6, E7
- [x] Decision: YES — E10 primary; secondaries as epic:* labels

#### Spike 2 — Newspaper / Local Information Track
- Current epics: **E10, E4, E5, E7** (4 epics)
- Suggested primary: **E10**
- Secondary in description: E4, E5, E7
- [x] Decision: YES — E10 primary; secondaries as epic:* labels

#### Spike 3 — Branded Wolfgang Expert Rooms
- Current epics: **E10, E6, E4, E5** (4 epics)
- Suggested primary: **E10**
- Secondary in description: E6, E4, E5
- [x] Decision: YES — E10 primary; secondaries as epic:* labels

**Bulk action for all three:** Assign all → E10 primary; record secondary epics in issue description.
- [x] Bulk decision: YES — all three as above

---

### 3B · bedrock-platform/converge/kb/Planning/MILESTONES.md

Both are Converge versioned releases that touch downstream modules — E1 is the natural primary.

#### Planned: v3.5 — Capability Contract Realignment
- Current epics: **E1, E2, E3** (3 epics)
- Suggested primary: **E1**
- Secondary in description: E2, E3
- [x] Decision: YES — E1 primary; secondaries as epic:* labels

#### Planned: v3.7 — Curated Facades & Downstream Proof
- Current epics: **E1, E2, E3** (3 epics)
- Suggested primary: **E1**
- Secondary in description: E2, E3
- [x] Decision: YES — E1 primary; secondaries as epic:* labels

**Bulk action for both:** Assign both → E1 primary; record secondary epics in issue description.
- [x] Bulk decision: YES — both as above

---

## After Decisions

Status 2026-07-02:

1. ~~Update the source MILESTONES.md files with `**Epic:**` fields for YES groups~~ — done
2. ~~Add deprecation headers to all ORPHANED files and SKIPPED files~~ — done
3. Run `tools/linear-import/import.py` — **pending, blocked on `LINEAR_API_KEY`**.
   Dry-run verified: 123 issues across 11 projects, 0 skipped.
4. Verify counts in Linear per the design doc's 2026-07-02 revision addendum
   (no sub-issues are created — deliverables stay as checklists in descriptions)

Reference: [linear-migration-design.md](2026-07-01-linear-migration-design.md)
