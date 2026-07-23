# Boundary Registry — Reflective workspace

**Purpose:** This file is the first document any AI session, architect, or new engineer reads when entering this workspace. It is the workspace-level anchor for the four-layer platform boundary. Without it, the next session re-litigates settled architectural decisions from scratch.

**Last updated:** 2026-07-10 (machinery v1.0 consolidation)
**Maintained by:** the three platform architects jointly. Edits require panel review (any architect can propose; all three sign).

## Machinery v1.0 Consolidation (2026-07-10)

As of July 10, 2026, the machinery subsystem was consolidated into a single unified repository with four coordinated sub-projects:

- **Unified repository:** `Reflective-Lab/machinery` (v1.0 release)
- **Old repos archived:** `build-depot`, `runtime-runway`, `commerce-rails`, `chart-room` (read-only)
- **Versioning:** All 4 projects tagged together (machinery v1.0, v1.1, etc.)
- **Authority:** Build-Depot remains the factory authority for standards and quality gates

See `machinery/README.md`, `machinery/AGENTS.md`, and `machinery/CONTRIBUTING.md` for operational details.

---

## The four-layer authority matrix

| Authority | Owner | Crisp test |
|---|---|---|
| Identity, org, auth, secrets, telemetry, runtime, deploy, storage, app shell, session ownership | **Runtime-Runway** | "Who can act and where the code runs." |
| Subscriptions, plans, entitlements, payments, revenue share, ledger, webhook receipts | **Commerce-Rails** | "Who pays, what is granted, what must be reconciled." |
| Trust-transfer surfaces, operator workbench, HITL approvals, truth catalog binding, governed-job ledger shape | **Helms** | "What the operator sees/approves and what the audit shape is." |
| Domain semantics, product flows, app-specific subject refs, process receipts | **Marquee app** | "A question only this product asks." |

Originating claim: `[CR-ARCH]` Round 1 of `REVIEW_quorum-sense_2026-06-15.md`. Confirmed by all three architects in Round 3.

## Canonical boundary documents (one per platform repo)

| Repo | Boundary doc | Owner |
|---|---|---|
| machinery (unified v1.0) | `machinery/runtime-runway/kb/Architecture/App Execution Container.md` | `[RR-ARCH]` |
| machinery (unified v1.0) | `machinery/commerce-rails/kb/Architecture/Operating Authority Boundary.md` | `[CR-ARCH]` |
| framework/bedrock/foundation/helm | `framework/bedrock/foundation/helm/kb/Architecture/Operating Authority Boundary.md` | `[HELMS-ARCH]` |

**Note:** As of machinery v1.0, Runtime-Runway and Commerce-Rails are sub-projects within the unified `Reflective-Lab/machinery` repository, but maintain independent boundary documentation within their respective directories (`machinery/runtime-runway/kb/` and `machinery/commerce-rails/kb/`).

Each boundary doc cross-links to the other two by absolute path. If you're an AI session looking for layer authority on a specific concern, read the owning repo's doc first.

## Active reviews and handoffs

| File | Status | Purpose |
|---|---|---|
| `/Users/kpernyer/dev/reflective/REVIEW_quorum-sense_2026-06-15.md` | FROZEN (signed by `[CR-ARCH]`, `[RR-ARCH]`, and `[HELMS-ARCH]` on 2026-06-15) | Three-architect panel review. Historical record only — implementors do not read this. |
| `/Users/kpernyer/dev/reflective/HANDOFF_quorum-sense_2026-06-15.md` | **ACTIVE** — signed by all three architects on 2026-06-15 | Implementor-facing brief. **Single source of truth for the quorum-sense engineer.** Section 1 (immediate stopgaps), Section 2 (platform-blocked rows), Section 5 (commitment ledgers) are the action surface. |

## Per-repo quality backlogs

| Repo | Backlog | Tracks |
|---|---|---|
| machinery (unified v1.0) | `machinery/QUALITY_BACKLOG.md` (coordination); see sub-project backlogs | Machinery-wide coordination |
| machinery/runtime-runway | `machinery/runtime-runway/QUALITY_BACKLOG.md` | RR D-IDs (D1–D6) from the 2026-06-15 review |
| machinery/commerce-rails | `machinery/commerce-rails/QUALITY_BACKLOG.md` | CR QF-IDs (QF-CR-02 through QF-CR-11) |
| framework/bedrock/foundation/helm | `framework/bedrock/foundation/helm/QUALITY_BACKLOG.md` | HELMS H-IDs (`H-2026-06-15-01` through `H-2026-06-15-05`) |
| applications/marquee-apps/quorum-sense | `applications/marquee-apps/quorum-sense/QUALITY_BACKLOG.md` (if absent: see HANDOFF Section 1) | App-side actions QS-* |

## Software factory quality boundary

`machinery/build-depot/` observes and normalizes quality, security, delivery, and incident
signals across repos. It owns factory graph facts and scorecard inputs; it does
not own product behavior, runtime authority, commercial authority, or Helm
operator-control authority. When Build-Depot finds drift in another layer, it
records and routes the finding to Linear, `QUALITY_BACKLOG.md`, or the owning
repo's backlog.

Canonical docs:

- `machinery/build-depot/docs/architecture/software-factory-build-depot.md`
- `machinery/build-depot/docs/operations/software-factory-quality-system.md`
- `machinery/build-depot/docs/operations/quality-gates.md`

## Consumer contracts (apps consume by contract; never by adapter reach-around)

| Layer | Contract doc | Seam checklist |
|---|---|---|
| Commerce-Rails (machinery v1.0) | `machinery/commerce-rails/kb/Contracts/Apps Consuming Commerce Rails.md` | `machinery/commerce-rails/kb/Contracts/Marquee App Seam.md` |
| Runtime-Runway (machinery v1.0) | `machinery/runtime-runway/kb/Architecture/App Execution Container.md` | `machinery/runtime-runway/kb/Contracts/Marquee App Seam.md` |
| Bedrock (marquee facade) | `framework/bedrock/kb/06-consumption/marquee-app-seam.md` | `{app}-platform` + `boundary-doctor` |
| Machinery (umbrella) | `framework/bedrock/kb/06-consumption/marquee-app-machinery-seam.md` | Runway + Commerce together at `{app}-server` |
| Helms | _pending_ — once H-2026-06-15-05 lands the live-readiness contract, link here | Live `JobReadinessPacket` shape, operator-control mount semantics, `mount_kind: "mounted"` requirements. |

Apps cite these contract docs in their own `runway.app.json:boundaries[]` field — `consumes: ["Runtime Runway host", "commerce-rails entitlement contract", "Helm operator-control"]`.

## Hard rules — the durable Marquee App Contract

These supersede any earlier doc. Promoted as standards into each repo's `kb/05-engineering/standards/` per the architect responsible.

1. **No app-local platform clones.** Apps do not hand-roll Axum, Cloud Run scripts, Dockerfiles, SPA serving, frontend shell, auth bootstrap, or session ownership. Use `RunwayAppHost::builder(...)` and what RR provides; if RR has not yet shipped a primitive, freeze that piece of app work until it ships.
2. **No commercial state outside commerce-rails.** Helms, runtime-runway, organism, converge, axiom, mosaic, and marquee apps do not own subscriptions, entitlements, payments, or plan→app mappings. Commerce-Rails is the source of truth, consumed by contract.
3. **No provider IDs (Stripe `cus_*`, `sub_*`, `price_*`) in app or domain code.** Use CR-internal `CustomerId`, `Plan` enum, and `entitlement_projection`. Provider refs are `ProviderObjectRef` only, owned by CR.
4. **No test-only HTTP handlers in production binaries.** `#[doc(hidden)]` and `TEST-ONLY` comments are documentation, not enforcement. RR's strict-always manifest verifier (D1) fails any build that exposes a route whose handler symbol contains `for_test` / `_test` / `test_only`.
5. **No Helm shells presented as live.** If `helm.operator-control` or `helm.governed-jobs` modules are mounted with default state, `runway.app.json` must declare `mount_kind: "planned"`. `"mounted"` means live state is wired.
6. **No multi-writer scale until RR session-ownership + CR persistent entitlement coherence ship.** Marquee apps run at `--max-instances=1` until D5 and CR-03/CR-08 are in.
7. **No feature flag, `--strict-mode` switch, or env toggle to soften a strict platform check.** Per `machinery/runtime-runway/CLAUDE.md`. Fix the cause, not the check.
8. **No caching of `is_entitled(uid, app_id)` beyond the JWT validity window.** Push refresh (CR-06) + `refresh-on-403` retry IS the contract. Caching past JWT expiry creates the "user paid, app says deny" failure mode.

## Review cadence

- Every new marquee-app onboarding triggers a panel review against this contract. The hard rules become the gate.
- The frozen review for that app produces a dated `REVIEW_<app>_<YYYY-MM-DD>.md` plus a `HANDOFF_<app>_<YYYY-MM-DD>.md`. Both are linked from this registry.
- Any architect can call a contract revision; revision requires all three signatures.

## For the next AI session

If you are reading this for the first time:
1. Read this file in full (you're doing that now).
2. Read the boundary doc for your concern's owning layer.
3. Read the active handoff for the app you're working on.
4. Only then read code.

If you're tempted to introduce something the Marquee App Contract forbids, stop and either (a) prove the concern doesn't apply, or (b) propose a contract revision through the panel. Do not silently route around the contract.

---

_This registry is workspace-level. Per-repo CLAUDE.md / AGENTS.md files quote from it but do not modify it._
