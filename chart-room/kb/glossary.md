# Glossary

Canonical terminology across all Reflective Labs projects.

## Core Layers And Rails

```
Applications / Helm -> Axiom -> Organism -> Converge -> Mosaic -> Runtime Runway
Commerce Rails sits above the platform where Reflective owns commercial authority.
```

| Layer | Role | Question | Examples |
|-------|------|----------|----------|
| **Helm** | Control surface | "What does the operator see and do?" | Desktop UI, web app, CLI |
| **Axiom** | Truth layer | "What should be true?" | App state, truth definitions, projections, validation |
| **Organism** | Formation layer | "Which team shape should try?" | Intent, huddle, debate, formation selection, gap-chasing |
| **Converge** | Governance layer | "Whether it's allowed" | Engine, promotion gates, Cedar policy, budgets, audit |
| **Mosaic** | Specialist bench | "Which reusable capability does the work?" | Arbiter, Crucible, Embassy, Ferrox, Manifold, Mnemos, Prism, Soter |
| **Runtime Runway** | Operations layer | "Where does it run and how is it operated?" | CLI/TUI packaging, auth, storage, secrets, telemetry, GPU workers |
| **Commerce Rails** | Business rail | "Who pays, what is owed, and what is granted?" | Billing, entitlements, partner payouts, refunds, disputes |

**Ownership**: Product repos own Helm, domain Truths, projections, and writeback.
Organism composes formations. Converge owns promotion authority. Mosaic owns
reusable specialist Suggestors and adapters. Runtime Runway owns deployment/runtime
plumbing. Commerce Rails owns Reflective Labs' commercial obligations.

**Flow**: Axiom compiles Truths into runtime intent; Organism selects and binds
a formation; Suggestors propose; Converge decides what becomes Fact; Runtime Runway
operates the runtime; product and business layers decide domain consequence.

## Core Primitives

| Term | Layer | Definition |
|------|-------|------------|
| **Axiom** (rule) | Converge | Non-negotiable convergence rule (9 total). Mathematical guarantees: monotonicity, determinism, idempotency. Not the same as the Axiom layer. |
| **Truth** | Axiom | A governed assertion: Observation → DraftProposal → ValidatedProposal → Fact. Defined in Axiom, executed through Converge. |
| **Fact** | Converge | A promoted truth — passed validation, invariants, and the commit boundary. Immutable once committed. |
| **Pack** | Converge | Bundle of domain agents: Suggestor + Validator + Invariant + CriterionEvaluator. Reusable across products. |
| **Blueprint** | Converge | Multi-pack composition addressing a business outcome (e.g., lead-to-cash). |
| **Intent** | Organism | Structured request: outcome + context + constraints + authority + forbidden + reversibility + expiry. |
| **Intent Codec** | Axiom / Organism | Translates functional, emotional, and relational JTBD into a machine-run job shape without granting authority. Lives at the product-to-runtime handoff. |
| **Huddle** | Organism | Multi-model collaborative planning — LLM reasoning + constraint solvers + domain models argue before proposing. |
| **Adversarial Agent** | Organism | Agent whose job is to break plans. Five types: assumption breaker, constraint checker, causal skeptic, economic skeptic, operational skeptic. |
| **Simulation Swarm** | Organism | Parallel stress-testing of candidate plans before commit. |
| **Cedar Policy** | Converge | Authorization policy language — scopes agent authority at the governance layer. |
| **Suggestor** | Converge/Mosaic | A participant that reads context and proposes facts, plans, evidence, or diagnostics through the Converge contract. |
| **Formation** | Organism | A selected composition of Suggestors and roles intended to converge a specific intent. |
| **Port** | Mosaic/Converge | A typed contract for a capability or source. Source-specific ports live in Embassy; generic swappable adapters live in Manifold. |
| **Commerce Rails** | Business rail | Reflective-owned commercial authority for subscriptions, entitlements, app installs, partner revenue share, payouts, refunds, disputes, and reconciliation. |
| **Runtime Runway** | Runtime Runway | Distribution and runtime operations: packaging, deployment, auth, storage, secrets, telemetry, and GPU worker paths. |
| **System of outcome commitments** | Platform | The Reflective platform posture beyond systems of record and systems of action: records and actions are governed by the committed outcome, authority, assumptions, projections, drift signals, and reopen path they serve. |

## Product Terms

| Term | Product | Definition |
|------|---------|------------|
| **Expert Panel** | Wolfgang | Structured multi-voice reasoning — multiple expert personas debate a question. |
| **Knowledge Base** | Wolfgang | Portable research artifact: manifest + Parquet chunks + expert profiles. |
| **Module** | Helm | Capability unit (e.g., `prio-ledger`, `prio-approvals`). 21 registered modules across 7 suites today. |
| **Suite** | Helm | Grouping of related modules (e.g., Revenue Core = metering + ledger + entitlements + payments). 7 suites total. |
| **Job** | Helm / apps | A business outcome represented as a truth definition. Four jobs are currently executable through `workbench-backend`; other catalog entries may be specs, policies, invariants, or not-yet-wired jobs. |
| **Applet** | Marquee Apps / Helm | A thin product surface around one Intent Codec entry, one primary job, and one governed consequence lane. Not a small SaaS clone. |
| **Convergent DD** | Monterro | Adaptive due diligence — gap-chasing, contradiction detection, governed research. Replaces fixed pipelines. |
| **Fathom Narrative** | Marquee Apps | Temporal-narrative analysis of corporate disclosures using provenance-bearing Suggestors over filing corpora. |

## Often Confused

| Pair | Distinction |
|------|-------------|
| Axiom (layer) vs. Axiom (rule) | The layer is where truths are defined ("what"). The rules are the 9 mathematical guarantees inside Converge. Context makes it clear. |
| Truth vs. Job | A truth is the governed assertion. A job is the execution of a truth to achieve a business outcome. |
| Pack vs. Module | A pack is a Converge primitive (domain agents). A module is a Helm/application capability unit. Modules use packs underneath. |
| Intent vs. JTBD | Intent is Organism's runtime contract (structured packet). JTBD is a product design framework (user research). Same "what the user wants" at different layers. |
| Helm vs. Helms.zone | Helm = the control surface layer. Helms.zone = the marketing site. |
| Embassy vs. Manifold | Embassy owns source-shaped ports where the external system identity matters. Manifold owns interchangeable provider/adapter capabilities. |
| Stripe vs. Commerce Rails | Stripe is a provider adapter. Commerce Rails owns the commercial state and accepted business facts. |
