# Marquee App Seam — Factory Contract

**Owner:** Build-Depot / software factory  
**Spec:** [bedrock marquee-app-seam.md](../../../framework/bedrock/kb/06-consumption/marquee-app-seam.md) (Bedrock)  
**Machinery:** [marquee-app-machinery-seam.md](../../../framework/bedrock/kb/06-consumption/marquee-app-machinery-seam.md) (Runway + Commerce)  
**Reference app:** `applications/marquee-apps/quorum-sense`  
**Cohort:** B (App family rollout) in `factory-cohorts.json`

---

## Purpose

Bedrock defines the **platform seam** (`bedrock-application` + `{app}-platform`).
Build-Depot defines what every marquee app repo must carry so the seam is
**mechanically enforced** in CI — not reviewed by hand each PR.

This contract extends Cohort B `adoption_target` with seam-specific evidence.

---

## Required artifacts (every marquee app repo)

| Artifact | Drift check |
|----------|-------------|
| `AGENTS.md` | `agents-doctor` (root / nested pointers) |
| `CAPABILITIES.md` | manual / adoption-doctor |
| `BOUNDARY_MAINTENANCE.md` | links bedrock + machinery seam docs |
| `UPSTREAM_REQUIREMENTS.md` | ledger non-empty or explicit "none yet" |
| `boundary-manifest.toml` | `boundary-doctor` in `just ci` |
| `scripts/boundary-doctor.sh` | runs clean on default branch |
| `crates/{app}-platform/` | sole Bedrock bridge crate |
| `runway.app.json` | RR D1 manifest verifier (when enforced) |
| `{app}-server` only | Runway + Commerce composition root |
| `Justfile` with `just ci` | CI workflow calls it |
| `BEDROCK_PIN.md` | present when consuming tagged Bedrock |

---

## Required CI behaviors

1. **`just ci` green** on default branch.
2. **`boundary-doctor`** in CI — fails on direct Mosaic import outside platform crate.
3. **No unmarked shims** — `SHIM(QF-*, expires:*)` or UR comment on temporary bridges.
4. **Bedrock pin lockstep** — one `bedrock-platform` tag across workspace deps when git-pinned.
5. **Fleet pin visibility** — if app path-deps `runtime-runway`, document Bedrock tag
   alignment; runway helm crates must match app Bedrock tag (see machinery seam).
6. **Commerce seam** — single `CommerceRails` at `{app}-server`; `is_entitled` gate;
   no provider IDs in tier 0–2; `deploy_contracts` in `runway.app.json`.
7. **Runway seam** — `RunwayAppHost::builder` + `runway.app.json`; no parallel HTTP
   stack in `{app}-app`.

---

## Adoption-doctor signals (target)

When scanning Cohort B `marquee-apps` members:

| Signal | Pass |
|--------|------|
| `boundary-manifest.toml` exists | yes |
| `scripts/boundary-doctor.sh` exists | yes |
| `just ci` invokes boundary-doctor | yes |
| `{app}-platform` crate exists | yes |
| `UPSTREAM_REQUIREMENTS.md` exists | yes |
| Direct `converge-provider-adapters` in `{app}-app` | **no** |

---

## Machinery release alignment

Factory policies moved from Bedrock into `machinery/` and `build-depot/` should
include this contract in the **Machinery release** checklist:

- Cohort B adoption-doctor checks seam artifacts.
- Root `just *-doctor` recipes remain workspace-level; per-app `boundary-doctor`
  stays in the app repo (app-specific crate names).
- Bedrock tag bumps trigger app `BEDROCK_PIN.md` execution — tracked in Linear,
  not silent manifest drift.

---

## When Bedrock ships 4.2+

1. Bedrock owner updates pin handoff + CHANGELOG.
2. Each marquee app runs pin checklist (shrink platform bridges for shipped URs).
3. Build-Depot records cohort health delta in factory scorecard / graph export.

---

## Related

- [Software factory quality system](software-factory-quality-system.md)
- [Quality gates](quality-gates.md)
- Bedrock [canon/ARCHITECTURE.md](../../../framework/bedrock/canon/ARCHITECTURE.md#marquee-boundaries)
