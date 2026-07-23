# Converge Engagement Template

Workspace skeleton for a new Reflective Labs engagement built on the Converge / Organism / Axiom / Ferrox / Helms stack.

Patterns lifted from `engagements/newspaper` — the reference implementation.

## Floor versions

| Layer | Crate / Repo | Floor |
|---|---|---|
| Converge | `converge-*` | 3.8.1 |
| Organism | `organism-*` | 1.5.0 |
| Axiom | `axiom-truth` | 0.7.0 |
| Arbiter (policy) | `converge-arbiter-policy` | 1.0.0 |
| Atelier (showcase + domain) | `converge-atelier-domain` | 1.0.0 |
| Embassy (ports) | `converge-embassy-*` | 1.0.0 |
| Ferrox (solvers) | `converge-ferrox-solver` | 0.4.1 |
| Manifold (adapters) | `converge-manifold-adapters` | 1.0.0 |
| Mnemos (knowledge) | `converge-mnemos-knowledge` | 1.0.0 |
| Prism (analytics) | `converge-prism-analytics` | 1.0.0 |
| Helms (optional) | `application-kernel`, `workbench-backend` | 0.1.1 |

Extensions follow the canonical naming `converge-<brand>-<topic>` and each lives in its own repo under `~/dev/reflective/mosaic-extensions/`. The workspace `Cargo.toml` keeps short aliases (`converge-knowledge`, `converge-analytics`, `converge-policy`, `converge-domain`, `ferrox-solver`) via the `package = "..."` rename trick so existing source `use converge_X::*` keeps working.

While platform head is unreleased, a `[patch.crates-io]` block at the bottom redirects every crate to `~/dev/reflective/bedrock-platform/{converge,organism,axiom}` and `~/dev/reflective/mosaic-extensions/*` so the resolver always picks up local head. Once the floor versions ship to crates.io, delete that block.

### Platform integration is deferred

Out of the box this template builds a green skeleton with no platform deps wired into `{{project}}-platform`. Helms is still broken (`helms/truth-catalog` and the helms workspace have stale extension paths from before the rename), so `application-kernel` / `workbench-backend` stay disabled until Helms catches up.

`axiom-truth` HEAD now depends on `converge-provider` + `converge-manifold-adapters` cleanly, so it can be re-enabled in `{{project}}-platform/Cargo.toml`. Re-enable per crate as you wire integration.

## Layout

```
{{project}}/
├── Cargo.toml                    # workspace root, platform pins, [patch.crates-io]
├── Justfile                      # check / fmt / clippy / test
├── README.md
├── AGENTS.md                     # canonical agent entrypoint
├── CAPABILITIES.md               # what this engagement owns vs platform
├── MILESTONES.md
├── CHANGELOG.md
├── crates/
│   ├── {{project}}-domain/       # Pure types — no platform deps
│   ├── {{project}}-kernel/       # Event-sourced state, write_with_events
│   ├── {{project}}-truths/       # TRUTHS &[TruthSpec] — business invariants
│   ├── {{project}}-app/          # Wires kernel + suggestors + formations
│   ├── {{project}}-platform/     # Feature-gated platform stack
│   └── {{project}}-server/       # Axum HTTP entry
└── kb/
    ├── Home.md                   # vault index
    ├── LOG.md                    # append-only mutation log
    ├── INDEX.md                  # entity catalog
    └── ...
```

## Patterns to follow

### 1. Crate split (from newspaper)

- **`-domain`** — pure types, zero platform deps. Importable by anything.
- **`-kernel`** — owns state. `write_with_events()` snapshot/drain; events go out *after* commit. No business logic in the kernel — only state transitions.
- **`-truths`** — `TRUTHS: &[TruthSpec]` array of business invariants. Reference constants for the dashboard, **and** the input to Axiom codegen when invariants graduate to compiled gates.
- **`-app`** — wires Suggestors and Formations. Defines `default_for_*()` factory functions on each Suggestor.
- **`-platform`** — feature-gated bridge to the platform stack. The `platform-local` cascade flips on Organism/Converge/Axiom/Ferrox in one go.
- **`-server`** — thin Axum wrapper.

### 2. Suggestor pattern

Suggestors define `reads`, `writes`, `formation_mode`, and a `default_for_*()` constructor. They expose a domain method (`enrich`, `cluster`, `score`, …) that takes domain types in and returns domain types out — **no Converge types in the public API**. Promotion to Converge `ProposedFact` happens at the Formation boundary, in `-app`.

### 3. Formation pattern

A Formation declares `steps: Vec<FormationStep>`, `proposed_facts`, `required_gates`, and `honest_stop_states`. Each step has explicit `reads` and `writes` over `FormationContextKey` — Seeds → Signals → Proposals → Strategies. `FormationMode` is `Routine`, `Deliberated`, or `Huddle` depending on whether human review is required.

### 4. IntentPacket builders

For every named workflow, the platform crate exposes a `..._to_organism_intent(run) -> IntentPacket` function. The packet carries `context`, `authority`, and **`forbidden`** actions (with reasons). This is the only thing handed to organism-runtime.

### 5. Truths

Truths are compiled constants today. When a truth graduates to a gate, run `cz codegen` (Axiom) to emit Rust invariants and register them as Suggestors. **Never hand-roll Gherkin parsing** — delegate to Axiom.

### 6. Ferrox

Where the JTBD has a constraint problem (scheduling, allocation, routing, layout, capital), add a `*OptimalSuggestor` with `solver_backend: SolverBackend::FerroxCpSat` or `FerroxMip`. Pair it with a fast greedy `Suggestor` so the Formation always has a fallback at lower confidence.

### 7. kb/ conventions

Every `.md` file in `kb/` has YAML frontmatter with `source: human | llm | mixed`. `LOG.md` is append-only. Auto-enrich: when you read a stale page, fix it in place and log the change.

## Anti-patterns (reinvention flags)

- ❌ Custom `Engine` / `Suggestor` / `Formation` / `Pack` traits — re-export from the platform.
- ❌ Custom Gherkin/BDD parsing — use Axiom (`cz codegen`).
- ❌ Custom MIP/LP/CP-SAT/scheduling — use Ferrox suggestors.
- ❌ Custom Cedar policy engine — use `converge-policy` or `prio-policies`.
- ❌ Custom identity, conversations, ledger, tasks, opportunities, workflow — use `prio-*` from Helms.
- ❌ Hand-written convergence loop or promotion gate — use `converge_kernel::Engine`.
- ❌ `register_*_suggestor(engine)` wrappers that just re-register a platform Suggestor — call `engine.register_suggestor(...)` directly. Wrappers earn their keep only when they bundle multiple suggestors or attach engagement-specific dependencies.

## Quickstart

```bash
cp -R ~/dev/reflective/templates/converge-engagement ~/dev/reflective/engagements/<name>
cd ~/dev/reflective/engagements/<name>
# Replace {{project}} placeholders
sed -i '' "s/{{project}}/<name>/g" Cargo.toml crates/*/Cargo.toml crates/*/src/lib.rs
just check
```
