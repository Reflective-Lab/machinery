# Software Factory Learning Loops: What Developers Need to Know

**Canonical doctrine:** `build-depot/docs/operations/learning-feedback-loops.md` (build-depot repo), with the graph schema in `docs/operations/signal-capture.md`. This note is the developer-facing digest. If they disagree, build-depot governs. Companion to [[software-factory-daily-pulse]].

## The one-paragraph version

The factory learns when signals close, not when they accumulate. Every failure, review finding, incident or flake must end in one of three sinks: a **mechanism** (doctor check, arena dimension, lint, trybuild tripwire, typed contract), a **baseline ratchet** (coverage, footprint, lint count, soak envelope, moved only by signed commits) or a **standard plus skill update** (RP-* recurring property, KB standard, playbook). Omnigraph is the memory that routes signals to sinks and later proves the sink closed. A signal that ends in a dashboard is telemetry, not learning.

## The one habit that matters most

Every bug fix answers two questions in the PR: **which gate caught this, and which cheaper gate should catch the next one?** That is the escaped-defect autopsy, the highest-density learning signal in the factory and the first loop that will be automated. Review caught what a test should have? Improve the detector, not the reviewer. A test caught what a type should have made unrepresentable? Move it left. Detection always moves toward the cheapest layer: reviewer → arena/doctor → clippy/trybuild → type system.

## What app developers ask before marking work ready

- Does the repo expose a meaningful `just ci`, and did I run it?
- If Actions or the factory is dark, can someone reconstruct my evidence from the PR body? (Local gates plus pasted evidence are the merge bar in degraded mode.)
- Is the Linear issue linked with the repo module label?
- Did I touch a dependency, deploy path, secret slot, external provider or contract surface the factory should know about, and does it trigger the heavier path?
- Does this app emit runtime incidents, or is Sentry explicitly not applicable?
- Am I duplicating platform, Runtime-Runway or Commerce-Rails logic that belongs in a shared layer?
- Factory doctrine points back to Build-Depot; app-local docs stay app-local.

## What platform and Bedrock developers ask

- Did I change a public API, schema, protocol, crate boundary or generated contract? Then full workspace confidence, not a focused gate.
- Did I touch workspace dependencies, registry attribution, publish paths, release mechanics, CI or the Justfile? Same answer.
- Are trybuild, arena, publish-dry or relevant clippy required here? (Whole-workspace `clippy -D warnings` is transition debt until the burn-down lands; do not claim it green.)
- Structural Cargo facts stay (`registry = "reflective-labs"` attributions, non-secret `.cargo/config.toml`); credentials never enter the repo.
- Is this failure recurring? Then it does not end with the fix: propose the doctor check, standard or template, and let the owner sign the promotion.
- Does my change move detection left, or only add review burden?

## Rules that protect the loop

- **Provenance or it did not happen**: facts carry commit, timestamp, gate version and evidence artifact. Append-only evidence, revisable judgment.
- **The factory proposes, owners sign**: new mechanisms, ratchet movements, deleted signals and cross-repo standard changes all need a human at the promotion point.
- **Signal hygiene**: a gate that is routinely excepted trains everyone to ignore red; fix it or delete it. A standing red is acceptable only as a tracked transition with an owner and phases.
- **Verification is scheduled, not hoped for**: every corrective action names the signal it expects to change and a revisit window.
- **Scope**: Build-Depot learns about how we build. Whether we build the right things is [[chart-room]]'s question.

## Why this shape

This is Converge's thesis applied to ourselves: every promotion is a governed commitment with evidence and a decision receipt, and the factory is customer zero for the commitment machine the platform sells. The loop already ran manually and fast (stale-checkout false green to fetch-backed doctor signal in hours, 2026-07-09); the doctrine exists to keep that speed as the fleet and the automation grow.
