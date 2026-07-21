# Software Factory Daily Pulse

**Canonical doctrine:** `build-depot/docs/operations/daily-pulse.md` (build-depot repo). This note records the contract in the vault and adds the reviewer's perspective; it does not restate the doctrine. If the two ever disagree, the build-depot document governs.

## The contract in one paragraph

Build-Depot is the factory clock. Repos own local truth: meaningful `just ci`, accurate repo-local docs, module labels, tests updated with behavior, no factory credentials in the tree. Build-Depot owns judgment and coordination: one adoption contract, one doctor, one scorecard, one secret-slot inventory, gate semantics, drift detection and the conversion of drift into Linear work. The daily developer pulse (code, local gate, commit, PR, label) never requires factory knowledge and never depends on the factory being online.

## Structural rules the doctrine encodes

- **One Truth labeling.** Every capability is stated as current mechanism, target mechanism or transition debt. Present-tense "Build-Depot owns X" is banned until the worker, secrets and smoke test exist. Transition debt lives in Linear, not in optimistic prose.
- **Degraded mode.** When Actions, Trigger or any factory surface is dark, the merge bar is the repo-local gate plus pasted evidence plus a linked issue. Factory judgment is additive, never a hard dependency.
- **Mechanical rebuild triggers.** Heavy paths (full workspace, ARENA_HEAVY, publish dry-runs, audits) fire from a changed-surface classifier, not reviewer judgment. Until per-repo classifiers exist, the doctrine lists the trigger paths explicitly. Same rule as [[ci-parity]]: one definition, consumed by both repo CI and the factory.
- **The Chart-Room boundary.** Build-Depot watches how we build (gates, adoption, dependencies, engineering drift). Chart-Room watches whether we build the right things (strategy alignment, roadmap drift, commitment drift). Mixed signals: depot records the engineering fact and links to Chart-Room for the strategic judgment.

## Reviewer's perspective (Claude, 2026-07-09)

This doctrine was born under fire, which is why I trust it more than a whiteboard version:

- **The wording rule is a process, not a one-time fix.** The Shipyard split review caught the doctrine's own failure mode twice in one day: docs claiming "Build-Depot owns publishing" before any publisher existed, and an ADOPTED verdict measured against a checkout 21 commits stale. Both were fixed by mechanism, not by wording alone: strict signals with no exception path, and a `checkout-current` signal that fetches before it judges and blocks when it cannot fetch. That is the standard to hold: a doctrine claim without an enforcing mechanism is transition debt and gets a Linear issue, same as [[first-class-shims]].
- **Degraded mode is proven, not theoretical.** On 2026-07-09 GitHub Actions was down org-wide (billing) and four PRs merged on locally-run gates with evidence in the PR bodies, including two reviewed contract extractions. The doctrine legitimizes what already worked under outage; keep it that way.
- **The classifier is the largest open gap.** Mechanical rebuild triggers are prose today. Until each repo exposes a changed-surface recipe that CI and depot both consult, heavy-path selection quietly reverts to judgment calls. This should be the next depot mechanism after the Shipyard publisher (RFL-194 phases 2 and 3 come first).
- **Watch the standing red.** The strict signals mean the fleet scan carries blocking failures for the whole Shipyard transition window, by design. If anything downstream ever consumes the doctor's exit code, separate tracked transition debt from untracked failures in the exit semantics rather than softening the signals.
- **Bedrock structural facts are settled.** The `registry = "reflective-labs"` manifest attributions and the non-secret `.cargo/config.toml` index definition stay in Bedrock permanently; Cargo needs the registry name at workspace-metadata load. Tested and confirmed 2026-07-09; do not relitigate.

Related: [[software-factory-learning-loops]], [[repo-layering]], [[typed-cross-layer-semantics]], [[branch-hygiene]], [[doctor-recipe-pattern]].
