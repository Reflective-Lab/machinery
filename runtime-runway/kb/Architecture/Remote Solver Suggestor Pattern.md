---
source: llm
type: architecture-note
date: 2026-06-14
relates_to:
  - ../../docker/Dockerfile.math-base
  - ../../crates/api-server/
spec: marquee-apps/quorum-sense/docs/superpowers/specs/2026-06-14-converge-grpc-suggestor-pattern-design.md
---

# Remote Solver Suggestor Pattern

Pointer note. Authoritative spec lives in quorum-sense (first consumer):
`marquee-apps/quorum-sense/docs/superpowers/specs/2026-06-14-converge-grpc-suggestor-pattern-design.md`.

## Runtime Runway's role

This repo owns the *deprecated* path the pattern replaces. Specifically:

- `docker/Dockerfile.math-base` (`kenneth-backend-math-base` image, OR-Tools
  v9.14 + HiGHS v1.14.0) is **deprecated for marquee-app use** by this pattern.
  Its only legitimate consumer going forward is Wolfgang's own backend image
  (out of scope — separate migration).
- Marquee-apps must not consume `kenneth-backend-math-base` after M3 lands. The
  invariant lives in §3.5 of the spec ("each extension owns its native build")
  and §2 invariant 2 ("startup tolerance: no marquee-app goes down because a
  solver service is down").
- The reliable-image-build problem documented in the recent `fix(docker)` run
  on math-base (5 commits: `885851a`, `430437d`, `41e13df`, `57a93f6`,
  `cd972c6`) is the immediate motivation for the extraction.

## What this repo gains

Nothing — the pattern is platform infrastructure that ships from
mosaic-extensions (ferrox-solvers, soter-smt) and is consumed by marquee-apps.
Runtime Runway tracks the contract here so:

1. A future contributor sees the deprecation arc on math-base before proposing
   another `fix(docker)`.
2. The shared GCP project (`reflective-labs`) and Artifact Registry path
   (`europe-west1-docker.pkg.dev/reflective-labs/converge/<service>:<tag>`)
   are findable from the platform-runtime repo.

## Migration status (2026-06-14)

- **Short-term unblock (Plan 5 T5):** ✅ shipped. quorum-sense Cloud Run deploy
  is green using `scheduling::greedy` (pure Rust) and skip-registration for
  reachability assurance. math-base remains in the build chain but is dead
  weight for quorum-sense's call graph.
- **M1 / M2 / M3 of the spec:** not started. Tracked in writing-plans output.
