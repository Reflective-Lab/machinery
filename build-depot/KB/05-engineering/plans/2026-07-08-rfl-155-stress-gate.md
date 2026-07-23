# RFL-155 — Stress gate implementation plan (app-dev blocker)

> Architect analysis 2026-07-08 over consolidation/main 4be1e855 + atelier main. Full path:line analysis in the SDD transcript a18a6aaf / RFL-155 comments.

## Scope honesty
- Arena harness contract real+stable (Dimension trait, RunContext, Verdict, min-aggregation); 1 real dim (layering), 7 stubs that are roadmaps-in-docs.
- crm-helm: real gRPC services EXIST and compile (PartiesGrpc etc. over KernelStore) but HelmModule wrappers discard the store and mount /status stubs — graduation = wiring, not writing. Hub/lease allocated-unused by design (self-documented 0/0). NOT yet in bedrock-consolidated (arrives at next atelier sync — sync must carry Cargo.toml+build.rs or the members glob breaks metadata).
- Multiuser session hardcoded to 4 roles + fixed cycles (session.rs:98-103) — N×M soak needs constructor parameterization (keep 4-role default; existing 10 tests stay green).
- Soak convention exists (#[ignore] + SOAK_ITERS) — reuse, don't reinvent. Platform-level soak absent.
- Driver runs all dims synchronously — shell-out dims MUST self-gate behind ARENA_HEAVY=1 (default fast path Skip-with-reason) to keep `arena report` sub-5s.

## Dimension verdicts: 4 LAND / 3 DEFER
LAND: snapshot-portability (regex fixture scan; very low cost, real anchor); hermeticity REDUCED to static/config detector (mock-by-default assertion; syscall sandbox explicitly deferred — hold the line); crate-footprint (cargo package --list + dep budget vs checked-in baseline JSON; ARENA_HEAVY-gated); determinism (3-headless-scenario JSONL rerun-diff; ARENA_HEAVY-gated; nextest flake-harness deferred).
DEFER with honest Skip evidence strings: coverage (trend-not-gate by own docs, no baseline, expensive), performance (no thresholds, pre-emptive, no incident anchor), semver (precondition = first 4.0.0 publish becomes the baseline).

## Waves
W0 (RFL-181, unblocks hermeticity verdict): arbiter CARGO_MANIFEST_DIR fix ∥ mock-by-default (sec-edgar, counterparty-kyc; live behind ARENA_LIVE_NET=1) ∥ fast-path flags for debate-loop + solver-policy-allocation (<30s).
W1 (∥, dim-layering template incl. negative synthetics + property tests): the 4 dims + defer-honesty pass on the 3 stubs.
W2 (sequential, needs crm-helm sync first — T2.4 membership gate): mount real gRPC into routers (7 modules, parties first) → hub consumers>0 (module writes publish EventEnvelopes) → lease consumers>0 (multiuser ownership on a mutating route) . Scope bar: in-memory substrate only; runway root stays app-side.
W3: parameterize multiuser session THEN 3 soak suites (collab convergence N×M via SOAK_PARTICIPANTS/SOAK_ROUNDS; realtime stem sustained load via SOAK_ITERS on ServerOffloadUnderLoad; operator concurrency extension) — #[ignore], quick defaults <60s in CI, nightly big, SOAK RESULT: lines, JSONL for determinism to diff.

## Done criteria (tied to migration-verdict rows)
(1) arena report: 4 real verdicts + 3 precondition-Skips, aggregate PASS. (2) 3 soak suites run w/ quick defaults + nightly results recorded. (3) crm-helm assembly reports hub>0 AND lease>0 with real gRPC responses. (4) live-network mock-by-default + hermeticity PASS. (5) crate-footprint real verdict vs baseline. (6) RFL-181 closed. Rows 2/6 already closed — must not regress (tonic edge watch in crm-helm mounting: it's L4/scenario so allowed; must not leak into foundation/helm).

## Risks
crm-helm sync integrity; tonic-in-axum edge (L4 ok, L3 forbidden); driver latency (self-gate); macOS sandbox scope-creep (deferred, hold); multiuser param regressions (default-preserving); determinism may surface real nondeterminism (uuid/wall-clock in JSONL) — treat as discovery.
