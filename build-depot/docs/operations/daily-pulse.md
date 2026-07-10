# Daily Pulse

Build-Depot is the software-factory pulse for Reflective engineering. Repos own
local truth. Build-Depot owns judgment and coordination.

This document is the canonical daily-pulse doctrine. Repo-local docs and KB
pages should point here instead of restating the contract.

## One Truth

Keep three states separate:

- **Current mechanism:** what actually verifies, ships, or records evidence
  today.
- **Target mechanism:** what Build-Depot should own once the worker, secret,
  workflow, or graph mechanism exists.
- **Transition debt:** tracked work needed to make current state match target
  state.

Current mechanism:

- Developers run repo-local Just gates, usually `just ci`.
- GitHub Actions workflows are thin runners around repo-local recipes when
  Actions is available.
- The adoption doctor is an operator-run pull/scan.
- Scorecards are local exports while Omnigraph runtime is deferred.
- Trigger workers, scheduled fleet automation, GitHub check-run integration, and
  graph-backed factory dashboards are target mechanisms unless the specific
  worker is already deployed and verified.

Target mechanism:

- Build-Depot receives GitHub, Linear, Sentry, release, deploy, and scheduled
  fleet signals.
- Build-Depot records repository, finding, incident, check, deployment, and
  scorecard facts in Omnigraph.
- Build-Depot emits PR/check feedback and scheduled drift reports without
  requiring app developers to understand factory internals.

For Bedrock crate publishing, the current mechanism is still transition
bound: the in-repo publish path remains the ship path until the depot-side
publisher, secret values, and smoke test exist. The target mechanism is
Build-Depot-owned registry orchestration (Kellnr, August 2026). Linear carries that transition work.

## Degraded Mode

The daily pulse must not depend on Build-Depot or GitHub Actions being online.

When Actions, Trigger.dev, Omnigraph, or another factory surface is dark, the
merge bar is:

- the repo-local gate that would normally back CI, usually `just ci`
- any release- or surface-specific local gate named by that repo, such as
  `just publish-dry`, `just arena`, or `just delivery-preflight`
- pasted or attached evidence in the PR body or merge note showing the command,
  timestamp, checkout, and result
- the linked Linear issue or accepted-risk reference when a required factory
  signal is unavailable

Build-Depot judgment is additive for the daily pulse. A dark factory must not
block normal development when local gates and evidence are present. Once factory
surfaces recover, Build-Depot should harvest the evidence and turn any drift
into tracked work.

## Developer Responsibility

Application and Bedrock developers own local truth:

- write the code and tests
- keep `just ci` meaningful and green
- commit and open PRs
- link or label the Linear issue with the repo module label
- keep repo-local operating notes accurate and point factory doctrine back to
  Build-Depot
- update tests and docs when behavior changes
- declare runtime/Sentry applicability
- avoid committing private factory credentials or credential wiring

Bedrock developers also preserve structural Cargo facts, including
`registry = "reflective-labs"` manifest attribution and the non-secret
`.cargo/config.toml` registry index configuration Cargo needs for workspace
metadata.

Developers do not own fleet scorecards, cross-repo adoption verdicts, registry
credential storage, Sentry aggregation semantics, security scan policy, or
release orchestration mechanics once the depot-side workers exist.

## Operating Rhythm

Every PR, current mechanism:

- Run the repo-local gate, usually `just ci`.
- Let CI call the same Just recipe when Actions is available.
- Include local evidence in the PR when Actions or factory checks are dark.
- Keep missing required signals tied to Linear work or accepted risk.

Every PR, target mechanism:

- Build-Depot factory checks inspect labels, gates, adoption signals, linked
  work, obvious engineering drift, and quality signals.
- GitHub check-run feedback mirrors repo-local evidence and factory judgment.

Merge to trunk, current mechanism:

- The repo merge records the local gate result and linked work.
- Operators may run Build-Depot scans or scorecards after merge.

Merge to trunk, target mechanism:

- Build-Depot records repository, finding, check, release, incident, and
  delivery evidence as graph-compatible facts.
- GitHub, Linear, and Sentry events become normalized factory signals.

Scheduled pulse, current mechanism:

- Operators run `just factory-adoption-doctor`, `just scorecard`,
  `just security-audit`, or repo-specific scheduled gates as needed.

Scheduled pulse, target mechanism:

- Fleet adoption scans, scorecards, security/dependency scans, Sentry
  aggregation, stale ownership checks, and missing-signal checks run
  automatically and create or update tracked work.

Release or tag:

- Run the relevant release preflight.
- Capture release, publish, deploy, and rollback evidence.
- Rebuild the complete affected surface when confidence must be global.
- Publish/deploy orchestration belongs in Build-Depot target state when the
  corresponding worker and secret values are live.

## Mechanical Rebuild Triggers

Heavy-path selection must be mechanical, not a per-reviewer judgment call. Each
repo should expose a recipe or script that classifies changed surfaces, and CI
and Build-Depot should consult the same classifier.

Until a repo has a dedicated classifier, treat these paths as complete-rebuild
triggers:

- lockfiles: `Cargo.lock`, `bun.lock`, `pnpm-lock.yaml`, package manager locks
- workspace manifests: root `Cargo.toml`, workspace package manifests,
  `package.json`, `tsconfig.json`
- operator surfaces: `Justfile`, `.github/**`, `trigger.dev/**`,
  `terraform/**`
- publish and release surfaces: publish workflows, release workflows,
  `publish-dry` recipes, registry config, versioning files
- shared contracts: protocol/schema files, graph schema, generated clients,
  public Rust API crates, app-facing package contracts
- Bedrock platform surfaces consumed by applications
- security and audit configuration

The long-term mechanism should be one shared changed-surface classifier per
repo, exposed through Just and readable by Build-Depot. That keeps repo CI and
factory judgment aligned under the same CI-parity rule as thin workflows.

## Complete Rebuilds

Complete rebuilds are for global confidence:

- release candidates and release tags
- mechanical trigger hits from the changed-surface classifier
- scheduled verification windows
- explicit operator escalation after a flaky or ambiguous local result

For Bedrock today, a complete gate means format, check, tests, relevant clippy,
trybuild, Arena, and publish dry-runs when release or dependency surfaces are
touched. Full whole-workspace `clippy -D warnings` is transition debt until the
tracked warning burn-down lands; it must not silently become the complete-rebuild
bar while known warnings remain.

For applications, a complete gate means the full app build/test/package path and
integration smoke tests for the Bedrock, Runtime-Runway, or Commerce-Rails
contracts the app depends on.

## Heavy Work

Heavy work runs only when triggered by risk, release, schedule, mechanical
surface classification, or explicit operator command:

- full Bedrock workspace rebuilds
- full Arena dimensions, including `ARENA_HEAVY`
- whole-workspace clippy after the current warning debt is burned down
- publish dry-runs across publishable crates
- dependency audit and deny-style gates
- end-to-end app stacks
- production deploy preflight
- Registry publish dry-runs
- cross-repo adoption scans
- live external-provider tests
- Sentry, backlog, and scorecard harvests
- soak tests on schedule or explicit request

Do not make every small PR pay for every heavy check. The local repo gate should
prove the local change; the shared classifier and Build-Depot determine when
broader confidence is required.

## Build-Depot Boundary

Build-Depot watches how we build:

- gates
- adoption
- dependencies
- stale evidence
- release and delivery mechanics
- security/audit posture
- engineering drift

Chart-Room watches whether we are building the right things:

- strategy alignment
- roadmap drift
- commitment drift
- product and portfolio intent

If a signal mixes both domains, Build-Depot should record the engineering fact
and link to Chart-Room for strategic judgment.

## What Build-Depot Carries

Build-Depot takes the factory burden away from app teams:

- one canonical quality and adoption contract
- one place for gate semantics
- one adoption doctor
- one scorecard
- one secret-slot inventory
- one delivery and preflight doctrine
- GitHub, Linear, and Sentry normalization target state
- engineering drift detection
- release and publish orchestration target state
- evidence capture for compliance and audit
- conversion of engineering drift into Linear work

The developer experience should be simple: each repo exposes clean local
evidence; Build-Depot judges it, records it, and coordinates the factory work
around it.
