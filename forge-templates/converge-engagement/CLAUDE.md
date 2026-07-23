# Claude Code Entrypoint

Read and follow `AGENTS.md` — it is the canonical project documentation.

## Session Scope

- **Milestones:** Read `MILESTONES.md` at the start of every session.
- **Changelog:** Update `CHANGELOG.md` for notable changes.
- **Strategic context:** `~/dev/reflective/bedrock-platform/EPIC.md`

## Claude-Specific Notes

- Prefer Edit over Write for existing files. Prefer Grep/Glob over Bash for search.
- Knowledge belongs in `kb/`.
- Run `just check` before considering work done.
- Never push to main without confirmation.

## Floor versions

This engagement requires:

- Converge ≥ 3.8.1
- Organism ≥ 1.5.0
- Axiom (`axiom-truth`) ≥ 0.7.0
- Extensions: `converge-arbiter-policy`, `converge-atelier-domain`, `converge-embassy-*`, `converge-ferrox-solver` ≥ 0.4.1, `converge-manifold-adapters`, `converge-mnemos-knowledge`, `converge-prism-analytics`, all ≥ 1.0.0

While these are unreleased, `[patch.crates-io]` in the workspace Cargo.toml redirects to `~/dev/reflective/bedrock-platform/{converge,organism,axiom}` and `~/dev/reflective/mosaic-extensions/*` for local head.

The template ships with platform deps in `{{project}}-platform` commented out. Helms (`application-kernel`, `workbench-backend`) is still on stale paths after the extension rename — re-enable when Helms catches up. See `README.md` § "Platform integration is deferred".
