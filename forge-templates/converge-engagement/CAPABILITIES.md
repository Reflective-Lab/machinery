# Capabilities — {{project}}

What this engagement owns, and what it consumes from the platform.

## What {{project}} owns

- Domain types specific to this engagement
- Business invariants expressed as `TruthSpec`s in `{{project}}-truths`
- Suggestors specialized for this JTBD
- Formation compositions that string suggestors into a workflow
- IntentPacket builders carrying engagement-specific authority and forbidden actions

## What {{project}} consumes (does not reimplement)

| Concern | Crate |
|---|---|
| Convergence engine, promotion gate, integrity proof | `converge-kernel` |
| Suggestor trait, ProposedFact, Engine | `converge-kernel` |
| Pack registration, Formation contracts | `converge-pack` |
| Cedar policy evaluation | `converge-policy` |
| LLM / search providers | `converge-provider` |
| Knowledge corpus retrieval | `converge-knowledge` |
| Constraint solving (CP-SAT / MIP / LP) | `ferrox-solver` |
| Intent admission, decomposition, huddle, adversarial review, simulation | `organism-runtime`, `organism-planning`, `organism-intent` |
| Truth validation + Rust codegen from `.truths` files | `axiom-truth` |
| Application-layer kernel + workbench backend | `application-kernel`, `workbench-backend` (Helms) |

If a capability above appears as bespoke code in this repo, it's a reinvention bug. Move the capability into `~/dev/reflective/bedrock-platform/<platform>` and consume it.
