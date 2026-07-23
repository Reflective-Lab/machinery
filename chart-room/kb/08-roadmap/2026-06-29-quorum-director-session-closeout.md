# Session closeout — Quorum director + dev stack (2026-06-29)

**Scope:** Session 1 coordination (`reflective`) + Quorum product (`marquee-apps/quorum-sense`).
**Outcome:** Mobile M3A.12 live HTTP path unblocked on `main`; dev stack + Plan 2 projection landed; D1 boot regression hotfixed.

## Merge train (quorum-sense → `main`)

| PR | Branch | What |
|----|--------|------|
| [#6](https://github.com/Reflective-Lab/quorum-sense/pull/6) | `feat/director-snapshot-m3a12` | `GET /api/director/snapshot` — Bearer auth, `director-contracts` `{ version, frame }`, v0 fixture (1844) |
| [#5](https://github.com/Reflective-Lab/quorum-sense/pull/5) | `next` | Ambient handler, helm feed, M3 remote-smt slim Docker, `domain_host` D1 wiring, boundary-local |
| [#7](https://github.com/Reflective-Lab/quorum-sense/pull/7) | `feat/plan2-director-live-projection` | Plan 2: `helm-session-host` mount, `resolve_director_snapshot()` live projection |
| [#8](https://github.com/Reflective-Lab/quorum-sense/pull/8) | `hotfix/d1-wire-domain-routes` | Restore `wire_domain_routes` after #7 dropped D1 registration (boot failed at serve) |

**Tip of `main` / `next`:** `371c728` (merge of #8).

## Architecture locked in

- **HTTP contract (stable):** `GET /quorum-sense/api/director/snapshot`, Bearer auth, JSON `{ version, frame }` from `director-contracts`.
- **Plan 2 seam:** swap `resolve_director_snapshot()` / `v0_director_snapshot()` body only — route, handler, manifest unchanged.
- **D1 wiring:** all manifest `domain_routes` on `RunwayAppHostBuilder::route_*` via `domain_host.rs`; director registered there with `SessionHostService` Extension; `QuorumDomainModule` is an init marker only (empty router).
- **Live vs fixture:** no session state → fixture v1844; after session-host push/gate → hub sequence drives live snapshot.

## Smoke evidence (local, post-#8)

```bash
just dev   # main@371c728, LOCAL_DEV=true
curl -sf http://127.0.0.1:5161/quorum-sense/healthz          # → ok
curl -sf -H "Authorization: Bearer dev" \
  http://127.0.0.1:5161/quorum-sense/api/director/snapshot   # → 200, version 1844
curl -s -o /dev/null -w "%{http_code}" \
  http://127.0.0.1:5161/quorum-sense/api/director/snapshot   # → 401 without Bearer
```

Mobile: `quorum_configure_director_api("http://127.0.0.1:5161/quorum-sense", "dev")` should resolve with source **`live`** (not `fixture_fallback`).

## Branch policy going forward

- **Director snapshot:** owned on `main` (#6 + #7 + #8). Do not re-add duplicate director routes on `next`-only stacks.
- **Integration branch:** `next` synced to `main@371c728`; future `next` work rebases onto `main`, not parallel director forks.

## Related coordination docs

- E11: `EPICS.md` → `KB/08-roadmap/2026-06-27-ai-director-ux-epic.md`
- Spine Plan 2: `KB/08-roadmap/2026-06-26-spine-plan-2-helm-session-host.md` (slice 2 status updated there)
- Cloud smoke checklist: `marquee-apps/quorum-sense/kb/Operations/Cloud Dev Runbook.md` (slim remote-smt section, added Session 1)

## Not done (manual / follow-up)

- [ ] Mobile DEBUG: confirm Director screen shows `live` on device/simulator against local `just dev`
- [ ] Cloud deploy smoke (cloud dev area): `BUILD=1` + runbook checklist for slim remote-smt image
- [ ] M4 milestone items in `marquee-apps/quorum-sense/MILESTONES.md` (product proof run, Atlas deployed demo)
- [ ] `quorum://` citation resolver (platform-level; Atlas spike exit)

## Session 1 reflective ledger

No new `QUALITY_BACKLOG.md` entries this session. PR #7 boot gap was caught by smoke and closed in #8 without a separate finding ID.
