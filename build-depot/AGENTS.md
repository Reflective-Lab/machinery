# Codex Instructions

Build-Depot is the Reflective software-factory graph and Trigger.dev worker. It
normalizes quality signals from GitHub, Linear, and Sentry into the Omnigraph
schema in `build-depot.pg`.

## Tooling

- Use **Bun** for all project-owned JavaScript/TypeScript workflows.
- Keep `Justfile` as the operator-facing command surface.
- Do not introduce `package-lock.json`; `bun.lock` is the dependency lockfile.
- Do not add project-owned Python or shell scripts. Use TypeScript under
  `scripts/` for automation.
- Terraform (`terraform/*.tf`) and Omnigraph schema/query files (`*.pg`,
  `queries/*.gq`) are domain files and should stay in their native formats.

## Common Commands

- `just check` or `bun run check` — strict TypeScript typecheck.
- `just test` or `bun run test` — Bun unit tests.
- `just ci` or `bun run ci` — local gate: typecheck plus tests.
- `just doctor` or `bun run doctor` — read-only factory setup drift checks.
- `just security-audit` — dependency audit plus tracked-file secret scan.
- `just delivery-preflight` — doctor checks plus local CI before deploy.
- `just seed` or `bun run seed` — regenerate `seed/seed.jsonl` from the
  factory ledger inputs.
- `just scorecard` or `bun run scorecard` — emit the local machine-readable
  factory scorecard while Omnigraph ingest is deferred.
- `just dev` — start the Trigger.dev local worker.
- `just deploy` — deploy Trigger.dev tasks.
- `just setup` — initialize Omnigraph and load seed data. Requires
  `omnigraph` and `omnigraph-server` in `PATH`.

## TypeScript Rules

- Keep `tsconfig.json` strict. Do not relax compiler options to land a change.
- Validate external payloads with `zod` before using them.
- Prefer Trigger `schemaTask` when a task accepts one known payload shape.
- For multi-source webhook tasks, normalize unknown input into typed internal
  records and skip unsupported shapes explicitly.
- Optional graph fields should be omitted when unknown, not written as
  explicit `undefined`.

## Graph And Factory Conventions

- Build-Depot is the canonical source for Reflective software-factory doctrine,
  quality-gate semantics, graph semantics, and scorecard interpretation. Keep
  `docs/operations/software-factory-quality-system.md`,
  `docs/operations/signal-capture.md`,
  `docs/operations/quality-gates.md`, and
  `docs/architecture/software-factory-build-depot.md` aligned.
- Keep graph record shapes aligned with `build-depot.pg`.
- `scripts/seed.ts` and `trigger.dev/debt-tracker.ts` should emit compatible
  NDJSON-shaped records: `{ "type": "...", "data": { ... } }`.
- Aggregate high-volume sources before graph emission. Sentry event floods,
  flaky retry storms, and advisory bursts should become stable
  `FactorySignal` records with counts and time windows, not one graph node per
  raw event.
- When adding a new node type or graph-facing field, update the schema, relevant
  queries, seed generation, and README together.
- `seed/seed.jsonl` is generated and ignored; update the generator, not the
  generated file.

## Dependencies

- This project uses Trigger.dev for worker execution and Anthropic for PR gate
  analysis.
- Reflective Rust artifacts are hosted on a private Cargo registry (Kellnr on
  the build server, per docs/operations/build-machine-stack.md); do not assume
  crates.io is the only Rust artifact path when editing related docs or infra.

## Repo Skills

- Repo-local playbooks live under `.claude/skills/`.
- `.codex/skills` and `.cursor/skills` should point at `.claude/skills` so
  Codex, Claude, and Cursor use the same project workflows.
- Read the matching `SKILL.md` before running quality, security, or delivery
  workflows for this repo.
