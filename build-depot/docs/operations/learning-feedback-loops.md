# Learning And Feedback Loops

Build-Depot should become the memory and feedback loop for how Reflective builds
software. Omnigraph is not only a dashboard store; it is the software factory's
learning substrate. Evidence goes in, patterns are detected, decisions are made,
actions happen, and the result feeds back into the graph.

The goal is fewer surprises, less tribal memory, and a factory that learns from
every PR, failure, incident, and release without becoming a blocker or a black
box.

Telemetry is not enough. Learning has happened only when a signal closes into a
mechanism, a ratchet, or a standard that changes future behavior.

**One Truth applies to this document.** Almost everything here is target
mechanism: Omnigraph runtime is deferred and no factory workers are deployed.
The current mechanism is the loop run manually: operators and agents harvest
review findings into Linear, run the adoption doctor and scorecards by hand,
and land baseline ratchets and doctor checks through normal PRs. That manual
loop already works (the 2026-07-09 stale-checkout false green closed into the
fetch-backed `checkout-current` signal within hours) and is the behavior this
document automates. Per the [Daily Pulse](daily-pulse.md) wording rule, do not
describe a loop stage as owned by Build-Depot until its worker, schema, and
verification exist; the transition work lives in Linear.

## Learning Loop

1. **Observe.** Capture signals from GitHub, Linear, Sentry, CI, local Just
   gates, releases, deploys, audits, adoption scans, and repo metadata.
2. **Normalize.** Convert noisy source events into stable graph facts such as
   `Repository`, `Finding`, `Incident`, `FactorySignal`, `CheckRun`,
   `Deployment`, `Risk`, `ADR`, ownership, and provenance edges.
3. **Interpret.** Classify whether the signal is a one-off, recurring drift,
   accepted risk, blocked work, missing ownership, fragile gate, stale doctrine,
   release risk, or platform/app boundary mismatch.
4. **Act.** Turn conclusions into Linear issues, PR comments or checks,
   scorecard movement, updated gates, runbook updates, standards, adoption-doctor
   failures, or release/deploy blockers when the blocker is warranted.
5. **Verify.** Check whether the action changed the signal: improved,
   unchanged, worse, accepted, or obsolete. Verification is scheduled, not
   hoped for: every Act step records which signal it expects to change and a
   revisit window, and the scheduled pulse (or the operator, in the current
   mechanism) re-checks at that window. An action with no expected signal
   change is not part of the loop.
6. **Promote.** Repeated lessons become doctrine, Just recipes, doctor checks,
   scorecard dimensions, repo conventions, templates, or shared platform
   capabilities.

## Closure Sinks

Every learning loop must close into one of three sinks:

- **A mechanism:** a doctor signal, Arena dimension, lint, trybuild tripwire,
  type-level contract, typed parser, or other executable detector. Example:
  stale-checkout false green became `checkout-current` with Git evidence and a
  regression test.
- **A baseline ratchet:** coverage, crate footprint, lint warning count, soak
  parameters, performance envelopes, or similar measured baselines. Ratchets move
  in the good direction through signed commits; regressions create tracked work.
- **A standard plus skill update:** recurring properties, KB standards, repo
  instructions, and `.claude/skills` playbooks that teach agents and humans the
  new operating rule.

The graph's job is to remember enough to route every signal to the right sink
and later prove that the sink closed. A signal that ends only in a dashboard is
telemetry, not learning.

## Single-Loop And Double-Loop Learning

Single-loop learning fixes the immediate defect. Double-loop learning fixes the
system that let the defect through.

Build-Depot should optimize for double-loop closure:

- If review catches a bug tests should have caught, improve the detector rather
  than asking reviewers to remember harder.
- If a Sentry incident escapes production gates, record which gate should have
  caught it and file the gate-improvement work.
- If a signal is repeatedly excepted, fix or delete the signal rather than
  teaching everyone to ignore red.
- If debt expires, escalate the expiry breach automatically instead of letting
  old accepted risk become background noise.

This is Converge's thesis applied to the factory itself: every promotion from
finding to mechanism, or from baseline to ratchet, is a governed commitment with
evidence and a decision receipt. Build-Depot is customer zero for the commitment
machine it helps the platform sell.

## Signal Requirements

Build-Depot learning depends on signals that are:

- **Structured:** repo, command, status, timestamp, owner, source, and affected
  surface are fields rather than prose-only logs.
- **Provenanced:** every fact records where it came from and when it was
  observed.
- **Linked:** PRs link to Linear, incidents link to repos, deployments link to
  commits, and findings link to the evidence that supports them.
- **Stable:** durable IDs identify the thing being tracked; changing text
  snippets are not identity.
- **Aggregated:** Sentry floods, flaky retry storms, and advisory waves become
  counted signals with time windows, not thousands of raw graph nodes.
- **Actionable:** recurring problems point to a next action, owner, issue, or
  accepted-risk reference.
- **Scoped:** Build-Depot watches how we build; Chart-Room watches whether we
  are building the right things.
- **Degradable:** local gates remain sufficient truth when Actions or factory
  workers are dark.
- **Safe:** graph facts do not store secrets, raw tokens, sensitive customer
  payloads, or unbounded logs.
- **Feedback-capable:** each conclusion can be tested later against subsequent
  signals.

Evidence rules:

- **Provenance or it did not happen:** graph facts carry commit, timestamp, gate
  version, and the evidence artifact or stable artifact reference.
- **Append-only evidence, mutable judgment:** observations do not change; later
  verdicts may be revised when doctrine improves.
- **Boundary validation first:** source payloads are normalized and validated at
  ingest. Garbage signals produce confident garbage conclusions.

## Graph Learning Requirements

Omnigraph should support these learning questions directly:

- **Gate efficacy:** every defect record should carry which gate caught it. For
  escaped defects, it should also carry which gate should have caught it.
- **Escaped-defect autopsy:** Sentry incidents and review-caught bugs should
  produce a gate-improvement finding when a cheaper detector should exist.
- **Detection movement:** repeated review findings should show whether they have
  moved left into Arena, clippy, trybuild, typed contracts, or the type system.
- **Signal hygiene:** each signal should expose exception rate and standing-red
  duration. A tracked transition with owner and phases is tolerable; untracked
  standing red is drift.
- **Debt dynamics:** debt creation rate, burn rate, half-life, and expiry
  breaches should be queryable by repo, owner, and bucket.
- **Loop latency:** time from signal to tracked work, and from tracked work to
  landed mechanism, should be measured as factory drift when it regresses.

The first loop to automate should be escaped-defect autopsy:

1. Capture a Sentry incident, review-caught bug, or escaped release defect.
2. Ask which existing gate caught it, or which gate should have caught it.
3. File the gate-improvement issue with repo, owner, evidence, and proposed sink.
4. Track whether the fix landed as a mechanism, ratchet, or standard.

That loop has high information density and reuses existing `Incident`,
`Finding`, and repository facts while driving the rest of the learning model:
gate efficacy, left-shifting, signal hygiene, and loop latency.

## App Developer Prompts

Application developers should ask these questions before marking work ready:

- Does this repo expose a meaningful `just ci`, and did I run it when needed?
- If GitHub Actions or factory workers are dark, can someone understand the
  local evidence from the PR body?
- Did I label or link the Linear issue with the repo module label?
- Did I touch a dependency, deploy path, external provider, secret, runtime
  incident surface, or app contract that Build-Depot should know about?
- Did I touch a contract surface that should trigger the heavier changed-surface
  path?
- Does this app emit runtime incidents, or is Sentry explicitly not applicable?
- Am I duplicating platform, Runtime-Runway, or Commerce-Rails logic that
  belongs in a shared layer?
- Did I keep app-local docs app-local and point factory doctrine back to
  Build-Depot?
- If this PR fixes a bug, which gate caught it, and which cheaper gate should
  catch the next one?

## Platform And Bedrock Prompts

Platform and Bedrock developers should ask:

- Did I change a public API, schema, protocol, crate boundary, graph shape, or
  generated contract?
- Did I touch Cargo workspace dependencies, registry attribution, publish paths,
  release mechanics, CI, Justfile, or security/audit configuration?
- Does this change require full workspace confidence or only a focused gate?
- Are trybuild, Arena, publish-dry, relevant clippy, or a changed-surface
  classifier required for this change?
- Is whole-workspace clippy still transition debt for this repo, and have I
  avoided claiming it as a green complete-rebuild bar before the burn-down lands?
- Is new shared behavior documented where apps can consume it?
- Did I preserve structural Cargo facts while keeping credentials out of the
  repo?
- Is this a recurring issue that should become a doctor check, standard, or
  reusable template?
- Does this change move detection left, or only add more review burden?
- Does it create or change a baseline ratchet, and is the ratchet movement
  signed in the commit?

## Human Promotion Points

The factory proposes; owners sign.

Human or designated owner approval is required when:

- a repeated finding class becomes a new mechanism
- a baseline ratchets upward or a threshold tightens
- an accepted risk becomes a standing exception
- a signal is deleted as noisy or low-value
- a standard or skill update changes agent behavior across repos

This keeps Build-Depot from ossifying around early signals. Agents can execute
the loop, draft the mechanism, and gather evidence; humans judge whether the
promotion reflects the system we want.

## Promotion Rules

The factory gets better when every repeated failure leaves behind a sharper
system:

- a flaky test becomes a tracked flake signal
- a repeated manual check becomes a Just recipe
- a repeated review comment becomes a doctor check
- a repeated incident pattern becomes a recurring property or standard
- a repeated app mistake becomes a template or platform capability
- repeated release confusion becomes delivery doctrine
- a repeated unclear ownership question becomes a repo label, owner edge, or
  Chart-Room handoff
- a baseline regression becomes a ratchet-protection issue
- a slow loop becomes a loop-latency finding

A signal that never feeds a query, scorecard, gate, issue, standard, or decision
should be removed or reshaped. More data is not learning. Better feedback loops
are learning.

## Feedback Back Into Planning

Learning outputs should return to the systems where work is planned:

- Linear carries active implementation work, accepted risks, and revisit dates.
- Scorecards show whether the factory is getting easier or harder to operate.
- PR checks and comments return focused, repo-scoped feedback.
- Docs and runbooks record durable decisions.
- Templates and shared platform capabilities prevent repeated local fixes.
- Chart-Room receives strategy, roadmap, and commitment drift rather than
  engineering gate drift.

Build-Depot should make the next similar change easier, safer, or more obvious.
If a captured signal does not help with that, it is not yet part of the learning
loop.
