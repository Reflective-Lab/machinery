# The Reflective Labs Story

## The Problem

SaaS is a UI wrapped around a database with hardcoded workflows. When the workflow changes, you buy another SaaS tool. When tools multiply, you hire people to move data between them. The result: organizations spend more on coordination than on outcomes.

AI agents make this worse before they make it better — agents that can act but can't be governed are just faster ways to create chaos.

## The Thesis

**The SaaS era ends when software can declare what should be true, rather than prescribing how to get there.**

Three layers of SaaS are unbundling at different speeds:
- **UI** dies first — agents don't need dashboards
- **Workflow engines** get contested — declarative truths replace hardcoded pipelines
- **Systems of record** persist — someone still owns the data

See [[saas-killer-business/Saas killer/00-index|SaaS Killer Research Vault]] for the full thesis and counter-arguments.

## The Stack

The stack now has platform layers, specialist extensions, operating runway, and
business rails. Each layer answers one question and owns one kind of authority.

```
Applications / Helm  ->  "What does the operator see and do?"
Axiom                ->  "What should be true?"
Organism             ->  "Which formation should try?"
Converge             ->  "What is admitted, promoted, and recorded?"
Mosaic               ->  "Which specialist capabilities are reusable?"
Runtime Runway       ->  "Where does it run and how is it operated?"
Commerce Rails      ->  "Who pays, what is owed, and what is granted?"
```

### Mosaic — Specialist Bench

Reusable Converge-adjacent capabilities live in
`mosaic-extensions/`: Arbiter policy, Crucible models, Embassy
source-specific ports, Ferrox solvers, Manifold providers/adapters, Mnemos
memory, Prism analytics, and Soter SMT evidence. Mosaic specialists do work and
propose evidence. They do not own product or commercial consequence.

### Converge — Governance

The deterministic execution engine. Nine axioms guarantee that every action is authorized, every result is traceable, and every state change passes through a commit boundary. Converge doesn't think. It enforces.

- [[converge-business/README|Overview]]
- [[converge-business/specs|Specifications]]

### Organism — Intelligence

Translates human intent into governed plans through huddles, adversarial agents, and simulation swarms. Organism proposes; Converge decides.

- [[organism-business/docs/runtime/ORGANISM_RUNTIME|Runtime Architecture]]
- [[organism-business/docs/planning/PLANNING_LAYER|Planning Layer]]
- [[organism-business/docs/intent/INTENT_SYSTEM|Intent System]]

### Axiom — Truth

Where business outcomes are defined. Truths, projections, validation, app state. Axiom initiates `Engine.run()`. Product repos own this layer.

### Helm — Control Surface

What the operator sees and touches. Desktop apps, web UIs, CLIs. Product repos own this layer too.

### Runtime Runway — Operations

Runtime Runway packages, deploys, and operates the stack. It owns the
`converge` binary, local and remote storage, Firebase auth middleware, secrets,
telemetry, containers, Cloud Run, and GPU deployment paths.

### Commerce Rails

Commerce Rails owns Reflective billing, entitlements, app listings, partner
revenue share, payout obligations, refunds, disputes, ledger-sensitive audit,
and payment-provider reconciliation. Stripe and future providers are adapters;
Commerce Rails owns the commercial truth.

## The Products

Each product owns its own Helm (UI) and Axiom (truth definitions), built on shared Organism and Converge layers.

**Wolfgang** — Research companion. Expert panels, team research, contrarian persona. The flagship that drives revenue.
- [[wolfgang-business/01-product/what-wolfgang-is|What Wolfgang Is]]
- [[wolfgang-business/03-business/positioning-and-go-to-market|GTM]]

**Helm and thin JTBD apps** — The SaaS killer made concrete. Helm currently has 35 Rust crates, 21 registered capability modules, a 23-definition truth catalog, and four executable truth paths. Thin apps such as Catalyst, Shoal, and Keystone keep domain expression in app repos while pulling reusable behavior from Bedrock and Mosaic.
- [[outcome-workbench/kb/Architecture/Application Layer Restructure|Helm Architecture]]
- [[outcome-workbench/kb/Architecture/Module Map|Helm Module Map]]

**Monterro** — Portfolio intelligence for a Nordic PE firm. Convergent due diligence across 8 companies. The proof that Converge + Organism work together.
- [[monterro/flows/convergent-dd|Convergent DD Flow]]
- [[monterro/kb/Concepts/Portfolio Intelligence|Portfolio Intelligence]]

## Proof Points

| Proof | What it demonstrates |
|-------|---------------------|
| Monterro DD traces | 194 facts from 1 seed, 0 contradictions, adaptive gap-chasing |
| Receipt OCR pipeline | Receipt-backed expense truth path projects standard approvals and manual-review cases |
| Helm truth catalog | 23 definitions; 4 currently executable through `workbench-backend` |
| Expert panels (Wolfgang) | Multi-voice reasoning with structured debate |
| Mosaic extensions | Suggestors for policy, models, ports, solvers, providers, memory, analytics, and SMT |
| Runtime Runway | Same code can run locally or remotely with deployment, auth, storage, secrets, and telemetry |
| Commerce Rails | Commercial authority is separated from platform mechanics and provider adapters |

## The Moat

1. **Axioms** — Mathematical guarantees agents can't bypass. Correctness baked in, not bolted on.
2. **Packs** — Domain truth libraries that compound. Every deployment makes the next one cheaper.
3. **Organizational learning** — Planning models improve with use.
4. **Adversarial architecture** — Institutionalized disagreement eliminates hallucinated certainty.
