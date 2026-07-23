> **Relocated 2026-07-08** from `bedrock-platform/STRATEGY.md` during the
> Plan C doc charter (RFL-179): the consolidated Bedrock repo keeps only
> entry-point markdown; workspace-level strategy lives here in the root KB.
> References to `EPIC.md`/`MILESTONES.md` are federation-era (superseded by
> Linear).

# Strategy

This document is the workspace-level strategic center. `EPIC.md` translates this
strategy into cross-project outcomes; product repos translate those outcomes
into their own milestones.

## Strategic Goal

Make organizational intent computable and safely convergent, with autonomy
earned through experience and governed by policy.

Short form:

> Make intent computable. Make convergence trustworthy. Earn autonomy.

## Strategic Thesis

Organizations should be able to express what must hold, then rely on a governed
system to compile, evaluate, execute, and converge that intent under explicit
constraints.

This is not a better workflow tool. It is a system for turning ambiguous
business intent into stable, validated, executable reality without making
humans orchestrate every step.

The core path is:

```text
Truth (a .truths Truth Document)
  -> Axiom validation and compilation
  -> IntentPacket
  -> Organism formation planning
  -> Converge fixed-point execution
  -> governed facts and projections
  -> ExperienceStore events
  -> better planning priors
```

## What We Are Not

- Not replacing SaaS as a feature-for-feature tool category.
- Not building a generic intent interface.
- Not automating arbitrary workflows.
- Not making free-running AI agents for business.
- Not competing on model access or prompt quality.
- Not treating UX as the product.

Those framings pull the work back toward tools, dashboards, assistants,
configuration, and crowded categories.

## What We Are Building

### Intent Specification

Intent must become structured without becoming painful to express. The
strategic unit is the **Truth** — a `.truths` Truth Document carrying outcome,
goal, authority, constraints, evidence, exceptions, and success criteria. In
commercial framing we call it a *Business Truth*; the underlying type is the
same in every domain.

### Intent Compilation

Axiom turns intent into something checkable before execution: validation,
simulation, policy analysis, invariant generation, and compile-time feedback.

### Convergence Execution

Converge does not run imperative workflows. It runs Suggestors over shared
context until a fixed point is reached or a typed budget stops the run.
Agents propose; the engine decides; facts are promoted through gates.

### Formation Planning

Organism decides which team shape should attempt the intent. Planners,
adversarial reviewers, simulators, policy agents, optimizers, analytics, and
domain packs all enter through the same Suggestor contract.

### Governance

Authority is never inherited from reasoning. The architecture recomputes
authority at the commit boundary through policy, promotion gates, HITL
approval, reversibility, expiry, and audit. Gate types are in place; end-to-end
Cedar wiring through the commit path is in progress.

### Experience-Informed Learning

ExperienceStore is an append-only ledger, not a memory blob. It records what
was proposed, validated, promoted, blocked, approved, exhausted, and resolved.
Those events calibrate future planning priors. They do not grant authority.

### Operator Surfaces

Helm surfaces intent, evidence, blockers, approvals, and projections. The
surface is an entrypoint and observability layer, not the strategic core.

## The Trust Strategy

The system should not claim that it takes over. It should earn autonomy.

Progression:

1. Human decides. The system observes and records the episode.
2. System suggests. Humans approve or reject proposed actions.
3. System proposes policy from repeated patterns.
4. Human delegates bounded authority.
5. System acts autonomously inside approved policy. Helm remains for
   visibility, override, exceptions, and new boundaries — at every stage, not
   just this one.

Stages 1–2 are the operational mode today. Stages 3–5 are the trust
progression we are wiring; each step adds explicit policy, governance, and
gating before autonomy expands.

Autonomy is not granted upfront. It is compiled from experience, governed by
policy, and expanded through human approval.

## Strategic Moat

The moat is not model access, UI polish, or a workflow library.

The moat is the decoder for trust:

- intent structure that captures outcome, constraints, authority, forbidden
  actions, reversibility, expiry, and evidence
- Axiom's pre-execution validation and compilation path
- Converge's fixed-point semantics, promotion gates, and immutable facts
- Organism's formation planning, adversarial review, simulation, and priors
- Cedar/HITL policy boundaries that make autonomy gradual and auditable
- ExperienceStore as an append-only foundation for experience-informed
  planning priors; the path to materialize the advantage runs through
  confidence-weighted recall and calibrated proposal shaping (not yet wired
  end-to-end)

## Wedge Strategy

Do not start with all organizational intent.

Start with one class of intent where convergence is valuable, bounded, and
provable.

The right wedge has:

- clear success criteria
- explicit constraints and forbidden actions
- meaningful authority boundaries
- repeatable evidence patterns
- measurable convergence or honest budget exhaustion
- a human approval point for risky or irreversible outcomes
- enough repetition for ExperienceStore priors to matter

Strong candidate classes include governed revenue qualification, vendor
selection, onboarding, compliance checks, incident handling, and
receipt/expense approval. Each should reduce human orchestration while making
the resulting facts more auditable.

## Commercial Spike Portfolio

The platform work now needs to be pulled by real app packages, not only by
bottom-up contract cleanup. These spikes are additive to the epics below; they
do not cancel the platform milestones. They decide which real product pressure
gets to force Axiom, Helm, Runtime Runway, Organism, Movement, Converge, and Mosaic to
clarify their boundaries next.

### Spike 1 - Acquisition Readiness Room

Current focus. Package `atlas-integration`, `warden-compliance`, and
`quorum-sense` as a demonstrable M&A readiness room for companies growing
through acquisitions.

The business job is:

- front-load uncertainty before due diligence and post-close integration
- map what assets are available to consume: legal artifacts, codebases,
  product portfolio, systems, vendors, and people networks
- govern evidence and compliance gates before teams treat an asset as usable
- show operator control, receipts, blockers, and unresolved questions in one
  visible app surface

`atlas-integration` should be the visible map. `warden-compliance` should be
the governed DD gate. `quorum-sense` should be the uncertainty intake. Scout,
Tally, and Fathom are supporting modules, not the first demo shell.

### Spike 2 - Newspaper / Local Information Track

Second commercial spike. Use the newspaper/editorial track to prove a near-term
business package around claims, sources, local knowledge, editorial workflow,
community trust, and publication boundaries.

The likely app center is `folio-editor`, with `fathom-narrative` for temporal
evidence windows and `inkling-notes` for local-first knowledge capture. The
business question is whether a governed editorial operating system can become a
real customer conversation soon.

### Spike 3 - Branded Wolfgang Expert Rooms

Third commercial spike. Package `wolfgang-chat` as an easy-to-deploy,
recurring-revenue product for small companies: their colors, logo,
knowledgebase, experts, panels, and research groups.

The business job is not generic chat. It is a branded expert room where a
company can put its knowledge and people into a governed research surface with
clear source boundaries, expert disagreement, and reusable commercial rails.

## Reflective Labs Business Layer

Reflective Labs is not only proving the stack with its own apps. It is offering
builders and SMBs a way to launch governed software when they know their domain
better than anyone but do not have the technology foundation to run it.

That requires one Reflective-owned business layer above the reusable stack:
Commerce Rails. Commerce Rails owns billing, entitlements, partner piggy-back
commerce, marketplace terms, revenue share, payouts, refunds, disputes, and
commercial audit. It consumes Converge, Organism, Axiom, Helms, Mosaic, and
Runtime Runway, but it does not live inside them.

The strategic distinction is strict:

- the platform owns reusable machinery,
- marquee and customer apps own domain outcomes,
- Reflective Commerce Rails owns Reflective Labs' commercial obligations.

## Success Metrics

Prefer metrics that measure convergence quality and safe autonomy:

- percentage of admitted intents that compile successfully
- percentage of compiled intents that reach stable convergence
- correctness of promoted facts against acceptance criteria
- rate of honest blocked states versus silent failure
- budget exhaustion rate and reason distribution
- HITL approval, rejection, and override rates
- number of repeated patterns graduated into proposed policy
- amount of bounded authority safely delegated
- replay and audit completeness for consequential runs

Avoid optimizing for clicks, dashboards, workflow count, prompts sent, or raw
agent activity.

## Operating Implications

- `STRATEGY.md` defines the goal.
- `EPIC.md` defines cross-project outcomes that advance the goal.
- `MILESTONES.md` defines the current work rollup.
- `kb/Doctrine/Intent-Driven Systems.md` carries the deeper category doctrine.
- Product repos own their implementation milestones and local strategy details.

When prioritizing, prefer work in this order:

1. Make intent more computable.
2. Make convergence more reliable and inspectable.
3. Make authority and policy boundaries more explicit.
4. Make learning improve planning priors.
5. Make Helm surfaces thinner, clearer, and more action-focused.

## External Positioning

Use simple language when needed:

> We build systems where companies express what must hold, and the system makes
> it converge.

Or, shortest:

> Make intent computable. Make convergence trustworthy. Earn autonomy.
