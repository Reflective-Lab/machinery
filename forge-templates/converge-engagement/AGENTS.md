# {{project}} — Agent Entrypoint

> See `~/CLAUDE.md` and `~/dev/reflective/engagements/CLAUDE.md` for global conventions.

## What this engagement owns vs what the platform owns

See `CAPABILITIES.md`.

## Crate map

| Crate | Responsibility |
|---|---|
| `{{project}}-domain` | Pure types — no platform deps |
| `{{project}}-kernel` | Event-sourced state, `write_with_events` |
| `{{project}}-truths` | `TRUTHS &[TruthSpec]` — business invariants |
| `{{project}}-app` | Wires kernel + suggestors + formations |
| `{{project}}-platform` | Feature-gated bridge to Converge / Organism / Axiom / Ferrox |
| `{{project}}-server` | Axum HTTP entry |

## Idiomatic patterns to follow

1. **Express JTBD as Suggestors.** Every job-to-be-done is a `Suggestor` (or several) reading from one `FormationContextKey` and writing to another.
2. **Compose Formations.** A workflow = a `Formation` with explicit `steps`, `proposed_facts`, `required_gates`, `honest_stop_states`.
3. **IntentPackets carry forbidden actions.** Every packet declares what *cannot* happen, with a reason.
4. **Truths are constants, then gates.** Start with `TruthSpec` constants. Promote to Axiom-codegen'd gates when an invariant graduates.
5. **Constraint problems → Ferrox.** If the JTBD has scheduling, allocation, routing, layout, or capital math, add a `*OptimalSuggestor` with `SolverBackend::FerroxCpSat` or `FerroxMip`. Pair with a fast greedy fallback.

## Reinvention is a bug

If you find yourself writing:

- a custom `Engine` / `Suggestor` / `Formation` / `Pack` trait
- a Gherkin/BDD parser
- a constraint solver / scheduling algorithm / MIP / LP
- a Cedar policy engine
- a custom auth / conversation / ledger / task / opportunity / workflow primitive
- a hand-rolled convergence loop or promotion gate

…stop. Use the platform crate instead. If the platform doesn't have it, file an issue against `~/dev/reflective/bedrock-platform/converge` (or organism / helms) — the capability lives there, not here.

## Session scope

- **Milestones:** read `MILESTONES.md` at session start
- **Changelog:** update `CHANGELOG.md` for notable changes
- **Knowledge:** start from `kb/Home.md`

## Rules

- Never commit secrets / `.env` / credentials
- Never push to main without confirmation
- Run `just check` before considering work done
