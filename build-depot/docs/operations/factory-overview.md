# The Reflective Labs Software Factory — One View

**Build-Depot is the single control plane.** It decides the rules, owns the tools,
and holds the ledger. Every software project in machinery collaborates with it but
does not redefine it: *projects own their local truth (code, tests, config);
Build-Depot owns judgment (standards, gates, doctrine).*

This document is the one authoritative view of the factory. It is generated from,
and kept consistent with, the machine-readable sources listed at the bottom.

_Last consolidated: 2026-07-10._

## Where the factory lives

Everything is inside **`machinery/build-depot`** — one git repo, self-contained:

| Asset | Location | Owner |
|---|---|---|
| Doctrine | `docs/operations/software-factory-quality-system.md` | Build-Depot |
| Gate semantics | `docs/operations/quality-gates.md` | Build-Depot |
| This overview | `docs/operations/factory-overview.md` | Build-Depot |
| Recurring properties (source) | `KB/05-engineering/standards/recurring-properties.json` | Build-Depot |
| Standards (16 `.md`) | `KB/05-engineering/standards/*.md` | Build-Depot |
| Quality ledger | `QUALITY_BACKLOG.md` | Build-Depot |
| Approved libraries | `docs/operations/approved-libraries.md` | Build-Depot |
| Adoption cohorts | `factory-cohorts.json` | Build-Depot |
| Doctor tooling | `scripts/factory/*.sh`, `scripts/rp-table*.py`, `scripts/*.ts` | Build-Depot |

As of the 2026-07-10 consolidation, machinery no longer depends on the outer
`reflective` repo for any factory function: the RP-table generator/checker and the
14 standards documents were ported in, and Build-Depot can regenerate and verify
its own RP table (`just rp-table-sync`, `just rp-table-check`).

## The factory at a glance

- **19 recurring properties** — 16 Enforced, 3 Aspired.
- **16 standards** documented in `KB/05-engineering/standards/`.
- **22 findings** in the ledger (17 open: 1×A, 10×B, 9×C, 2×D).
- **1 control plane + 3 governed software projects + governance framework.**

## Governed projects — every one plugs into the same gates

The contract is **RP-CI-PARITY**: a project's local `just ci` gives the same verdict
as CI on the same commit. Build-Depot defines what the gates mean; each project
implements the canonical recipe surface.

| Project | Type | `just ci` (canonical) | `security-audit` | Release/delivery gate | Status |
|---|---|---|---|---|---|
| **build-depot** | TS / Bun | `check → test` | `bun security:audit` + secrets | `delivery-preflight` → deploy | ✅ authority |
| **runtime-runway** | Rust (13 crates) | `fmt-check → check → lint → test` | `cargo audit` + gitleaks + `deny.toml` | GitHub release on `v*` tags (sign+notarize+GCS) | ✅ Tier1 |
| **commerce-rails** | Rust (2 crates) | `fmt-check → check → lint → test` | `cargo audit` + gitleaks + `deny.toml` | `delivery-preflight`; registry publish | ✅ Tier1 |
| **strategy-validator** | Rust (bin) | `fmt-check → check → lint → test` | `cargo deny` (advisories+bans) | `delivery-preflight` | ✅ Tier2 (onboarded 2026-07-10) |
| chart-room | governance | — (docs) | — | phase gates | ingested |

Before 2026-07-10, `strategy-validator` had no gate at all. Onboarding it surfaced
and fixed real debt (unformatted code, clippy issues, an unmaintained-dep advisory,
`git2` needing a security bump) and aligned its lint policy to the fleet baseline.

## What Build-Depot enforces (the doctor family)

Run from the repo via `just`:

| Gate | Enforces |
|---|---|
| `quality-doctor` | policy freshness, snapshot consistency, RP-table ↔ JSON sync |
| `project-doctor` | release-train integrity, crate layering, crate-size, fixture portability, rustc pinning |
| `agents-doctor` | AGENTS.md presence + tool-pointer discipline across nested repos |
| `shim-doctor` | every shim carries a `SHIM(QF-…, expires:…)` marker backed by a live finding |
| `fleet-status` | per-repo main-branch CI verdict (andon board) |
| `factory-adoption-doctor` | each repo against the 13-signal adoption contract |
| `rp-table-check` | the generated RP table matches `recurring-properties.json` |
| `scorecard` | numeric factory health (findings, RPs, repos, incidents, signals) |

## Supply-chain: detect before Dependabot

`cargo deny check advisories` reads the RustSec DB and fails at `just ci` /
`just security-audit` time — a pre-merge gate that catches vulnerable, unmaintained,
and yanked crates before Dependabot files a PR. Every Rust project carries a
`deny.toml`; the blessed-library policy lives in `approved-libraries.md`
(RP-DEP-CATALOG). Accepted advisories are tracked, justified `ignore` entries, not
silent skips.

## Onboarding a new project (the whole contract)

1. Add a `Justfile` with the canonical surface: `ci` = `fmt-check → check → lint → test`, plus `security-audit`, `delivery-preflight`.
2. Add `deny.toml` (Rust) or the equivalent audit gate; align lint config to the fleet baseline.
3. Wire it into `machinery/Justfile` (`check`/`test`/`ci`/`security-audit`) and add it to `factory-cohorts.json`.
4. Point the repo's `AGENTS.md` at Build-Depot; do not redefine doctrine locally.
5. Open a `QF-*` finding recording the onboarding.

## Machine-readable sources

- `KB/05-engineering/standards/recurring-properties.json` — the 19 RPs (drives the ledger table via `just rp-table-sync`).
- `factory-cohorts.json` — governed repos, tiers, cohorts.
- `just scorecard` — live numeric health as JSON.
- `QUALITY_BACKLOG.md` — the append-only findings ledger.
