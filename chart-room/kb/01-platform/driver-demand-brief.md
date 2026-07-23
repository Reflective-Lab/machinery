# Reflective — Driver App Demand Brief

*Status: standing task, v1, 2026-06-18. For a Cloud Agent running in the
`marquee-apps` repo, scoped to the driver apps — `quorum-sense`,
`atlas-integration`, `vouch-lending`. **Drivers define the category and pull the
platform.** This task asks each driver to translate the New Normal + its
commitment shape into concrete demands on the substrate, so the platform
components (`runtime-runway`, `bedrock-platform`, `mosaic-extensions`,
`commerce-rails`, `lattice-mesh`) can review readiness against **real app pull**
instead of abstract doctrine.*

*Canonical source for the positioning is the reflective KB. A dated snapshot of
the relevant docs sits beside this task in `docs/strategy/` (read-only copies —
edit upstream in the KB). This brief is copied in as
`docs/strategy/00-DRIVER-DEMAND-TASK.md`.*

## Why drivers go first

Reflective's own driver/applier axis: a *driver* defines and advances the
category and carries the narrative and commercial leverage; everything else
*applies* the substrate. A demand-driven review beats infra self-assessment —
the platform should react to what real apps need, not to imagined requirements.
The output of this phase (**Platform Demand Sheets**) is the primary input to the
component review (`00-REVIEW-TASK.md` in each component repo). This is phase 1 of
two:

1. **Drivers (here)** → produce demand sheets.
2. **Components** → review readiness against the de-duplicated demand rollup.

## What changed (60-second version)

The authoritative text is in the snapshots; brief recap:

- **The operating default inverts** — continuous action becomes machine-driven
  and ambient; human judgment becomes a sparse, authoritative governing core.
- **Systems of record → action → outcome commitments** — the business object is
  the governed outcome commitment.
- **Category vs. substrate** — *governed decision translation* is the market
  category; beneath it is a **governed commitment substrate** with six universal
  requirements: canonical commitment/mandate, explicit authority, faithful
  projections, receipts/provenance, drift detection, reopen/contest/revoke.
- **Three commitment shapes** — convened burst (`quorum-sense`), multi-sovereign
  (`tally-escrow`), standing governance of autonomy (`triage-keeper`); plus the
  partial truth-preserving projection (`folio-editor`).
- **One loop, pluggable Decide** — Decide → Translate → Operate → Detect drift →
  Reopen; only Decide changes by shape.

Read the snapshots for the authoritative text: `the-new-normal.md`,
`reflective-paradigm.md` (§5 loop, §6 fit test + shapes), `stack-one-pager.md`
(layer ownership + three modes), `quorum-sense-substrate-scoping.md`,
`domain-fit-scan.md`.

## Your task — one Platform Demand Sheet per driver app

Do this for **each** of `quorum-sense`, `atlas-integration`, `vouch-lending`.
(Quorum is the convened-burst reference; Atlas is the M&A-integration domain;
Vouch is the lending/underwriting domain. All three are convened-burst today.)

1. **Shape statement.** Name the app's commitment shape today. State whether the
   New Normal pushes it toward a **standing mandate** (the decision becomes a
   continuously-operated mandate) or toward **multi-sovereign edges** (parties
   across a trust boundary). Explicitly flag where multi-sovereign or
   standing-autonomy demands *would* change the asks, even if out of current
   scope.
2. **What it must become.** Under the inversion: what runs continuously/ambiently,
   where the human core stays sparse and authoritative, and what canonical
   commitment the app governs.
3. **Substrate demand sheet.** Concrete asks per platform layer. Each ask carries:
   capability, *why* (tie it to one of the six requirements or a shape's
   machinery), **severity** (blocker / soon / strategic), and the **owning
   component** if known. Layers:
   - **Auth & identity** (`runtime-runway`) — incl. any cross-org identity needs.
   - **Runtime & execution** (`runtime-runway`) — session lifetime, realtime vs
     async, long-horizon operate, kill/reopen, degraded-connectivity behaviour.
   - **Storage & provenance** (`runtime-runway` + Converge/Axiom) — receipts,
     audit, selective disclosure.
   - **Telemetry & drift** (`runtime-runway` + Mosaic/Fathom) — runtime telemetry
     compared against the commitment/mandate.
   - **Governance & commitment** (`bedrock-platform`: Converge/Axiom/Organism/
     Helms) — authority model, ratification, reopen path.
   - **Commerce & entitlement** (`commerce-rails`).
   - **Specialist capabilities** (`mosaic-extensions`).
   - **`lattice-mesh`** where relevant.
4. **Gaps the app feels today.** Where the current platform blocks the app — cite
   files (`runway.app.json`, `CAPABILITIES.md`, `kb/`, milestones).
5. **Findings for the ledger.** bucket (A/B/C/D), effort (S/M/L), `Codex-safe
   now?`, evidence (file/line/commit), per the root `AGENTS.md` conventions.

## Output

- One sheet per app → `docs/strategy/DEMAND-<app>-<YYYY-MM-DD>.md`.
- A **de-duplicated rollup** → `docs/strategy/DEMAND-rollup-<YYYY-MM-DD>.md` that
  merges asks across the three apps and tags each with its owning component. This
  rollup is what gets relayed into the component repos for phase 2.

Do not edit the snapshot docs — they are read-only copies of the canonical KB.
