# Platform Vision & Operating Model

This is the highest-level business and operating context for Reflective Labs.
Read this first when you need to understand what the platform is, why it
exists, and how the pieces create value together.

## Current Platform Thesis

Reflective keeps a consequential decision's truth, authority, and dissent
intact as it becomes the operating reality of every role it touches. Group
judgment becomes governed commitment, and commitment becomes shared
understanding — without losing what was actually decided. We call the
category **governed decision translation**.

The platform sits beyond the familiar enterprise progression from **systems
of record** to **systems of action**. Records remember what happened; action
systems move work. Reflective is a **system of outcome commitments**: it binds
records and actions to the committed outcome, authority, assumptions,
projections, drift signals, and reopen path they are meant to serve.

Beneath that market-facing category, the platform is a **governed
commitment substrate**: the same primitive governs a commitment whether it
is *decided in a room*, *negotiated between sovereign parties*, or *set once
to govern continuous autonomous action*. The convened decision burst
(`quorum-sense`) is one shape of commitment, not the template for all of
them — see the paradigm §6 fit test (six universal requirements + three
shapes).

The full position paper is [[reflective-paradigm|The Reflective Paradigm]].
It is the single source of truth for this claim and is canonical for
**platform doctrine and positioning**; code-backed boundaries remain
governed by [[../04-architecture/current-system-map|current-system-map]].

The stack is organized around authority:

- Helm/application surfaces own what operators see and do.
- Axiom translates jobs and truths into runtime contracts.
- Organism selects formations and planning strategy.
- Converge owns governance, admission, promotion, and facts.
- Mosaic provides reusable specialist capabilities.
- Runtime Runway operates deployment, auth, storage, secrets, telemetry, and runtime paths.
- Commerce Rails owns commercial state, entitlements, obligations, and provider reconciliation.

## Value Proposition

The business value is not generic automation. It is governed work:

- consequential decisions can be traced back to intent, evidence, and authority
- data and workflow become meaningful because they are bound to the outcome
  commitment they serve
- humans keep approval and redirect control at the right points
- product teams reuse platform foundations without duplicating runtime, billing, or specialist capabilities
- commercial applications can launch on shared rails instead of rebuilding billing, entitlement, and runtime plumbing

## Ecosystem Map

| Layer | Current home | Role |
|-------|--------------|------|
| Bedrock | `bedrock-platform/` | Core system base: Helm, Axiom, Organism, Converge |
| Mosaic | `mosaic-extensions/` | Reusable policy, model, port, solver, adapter, memory, analytics, and SMT specialists |
| Atelier Showcase | `atelier-showcase/` | Live/local scenario gallery and tutorial spine for stack composition |
| Arena Tests | `arena-tests/` | Cross-extension integration and contract-shape pressure tests |
| Runtime | `runtime-runway/` | Operational runtime, app host, auth, storage, secrets, telemetry, LLM/GPU paths |
| Commerce | `commerce-rails/` | Billing, entitlements, marketplace, payouts, reconciliation |
| Marquee apps | `marquee-apps/` | Thin JTBD apps and commercial proof surfaces built on Bedrock and Mosaic |
| Studio apps | `studio-apps/` | Creative, research, notes, writing, and presentation apps moved out of marquee |
| Other products | `beacon-sites/`, `mobile-apps/` | Public sites, app portal, and native clients |
| Knowledge | `KB/` | Canonical operating context and decision memory |

## Stakeholders

- Founders and leadership: platform direction, operating model, and strategic bets.
- Product and design: user experiences built on governed work.
- Engineers: domain boundaries, APIs, runtime contracts, and reusable patterns.
- Business development and partners: commerce flows, marketplaces, and customer packaging.
- Operators and support: deployment, incidents, secrets, and recovery.

## Canonical Links

- [[reflective-paradigm|The Reflective Paradigm]] — canonical position paper
- [[../category-one-pager|The One-Page Story]] — single-page category claim
- [[../stack-narrative|The Reflective Labs Story]]
- [[../business-pitch|Business Pitch]]
- [[../investor-pitch|Investor Pitch]]
- [[../glossary|Glossary]]
- [[../04-architecture/current-system-map|Current System Map]]
