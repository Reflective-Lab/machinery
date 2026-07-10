# RP-NO-MATH-BASE-LATEST

**Status:** Active. Promoted 2026-06-15 (D4 — math-base semantic tagging).
**Originating finding:** `QUALITY_BACKLOG.md` → D4 (deploy template + math-base tag).
**Source review:** `REVIEW_quorum-sense_2026-06-15.md` Round 2 D4; recipe contract QF-CR-04.

## What

`kenneth-backend-math-base` (the OR-Tools + HIGHS + Rust builder/runtime base for
math-capable marquee apps) is published and consumed under a **semantic tag**
that encodes the versions baked into the image:

```
v<RUST_VERSION>-ortools-<ORTOOLS_TAG>-highs-<HIGHS_TAG>
e.g. v1.96-ortools-9.14-highs-1.14.0
```

The tag is derived mechanically from the `ARG` defaults in
`docker/Dockerfile.math-base` by `ops/scripts/build-math-base.sh`. Publishing or
pulling the image as `:latest` — or as a bare `@sha256` digest — is **banned**.

## Why

`:latest` is a moving tag: two builds of the same app, weeks apart, silently get
different OR-Tools / HIGHS binaries. "Which solver version is in production?"
becomes unanswerable, and a base-image rebuild can break a downstream app with
no diff in that app's repo. A bare `@sha256` digest is reproducible but
unreadable — it hides the exact version triple the semantic tag exists to make
legible at a glance in a `FROM` line and in `git blame`. The semantic tag gives
both: reproducibility (the tag changes iff the versions change) and legibility.

This pairs with the D4 deploy template (`ops/templates/`), which materializes an
app's `runway.app.json:deploy_contracts` into Cloud Run env/secret flags from
Commerce-Rails recipes — same principle: version-pinned, declared contracts
instead of drifting, copy-pasted configuration.

## How to check (drift)

```
just math-base-audit          # ops/scripts/math-base-audit.sh
```

Scans sibling Dockerfiles (the reflective workspace by default; override with
`AUDIT_ROOT`) for `kenneth-backend-math-base:latest` or
`kenneth-backend-math-base@sha256` and exits non-zero on any hit. The canonical
workspace gate is the root repo's `just project-doctor`; this RR-scoped recipe
is the runnable implementation it should call.

Build side: `build-math-base.sh` refuses to compose a `:latest`-style tag and
`cloudbuild.math-base.yaml` carries a non-valid sentinel default so an
unparameterized `gcloud builds submit` fails rather than shipping `:latest`.

## Status of enforcement

| Surface | Mechanism | State |
|---|---|---|
| Publish | `build-math-base.sh` derives semantic tag, refuses `:latest` | **Enforced** |
| Cloud Build default | sentinel `_TAG` (not a valid moving tag) | **Enforced** |
| Downstream pulls | `just math-base-audit` drift check | **Enforced (advisory exit code)** |

### Known pending migration (tracked, not a gap)

- `marquee-apps/quorum-sense/deploy/backend/Dockerfile.cloudrun` (lines 54, 125)
  still pins `kenneth-backend-math-base:latest`. The audit reports it today; it
  clears once the semantic tag is pushed and Quorum's `FROM` lines adopt it.
  Tracked downstream in the Quorum handoff alongside D6 adoption.

## Links

- Build script: `ops/scripts/build-math-base.sh` · Audit: `ops/scripts/math-base-audit.sh`
- Image: `docker/Dockerfile.math-base`, `docker/cloudbuild.math-base.yaml`
- Deploy template (D4 sibling): `ops/templates/cloud-run-deploy.sh`,
  `ops/templates/materialize-deploy-contracts.sh`
- Recipe contract: `commerce-rails/kb/Contracts/Deploy Recipes/` (QF-CR-04)
