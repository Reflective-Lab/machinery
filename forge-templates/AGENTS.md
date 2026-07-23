# Forge Templates

Project templates for the Reflective fleet (`converge-engagement`,
`converge-extension`). Not a git repository — treat as reference material;
changes here are unversioned.

This directory is part of the Reflective workspace. Operating policy:
`~/dev/reflective/AGENTS.md`. Shared workflow playbooks:
`~/dev/reflective/SKILLS.md`.

## Software Factory

Software-factory policy and quality-gate semantics are owned by
`build-depot/`. This repo emits local evidence through its Just recipes, CI,
template checks, and docs; Build-Depot normalizes that evidence into the
workspace factory graph.

- Build-Depot cohort: `A`
- Linear label: `module:forge-templates`
- Sentry: not applicable. Forge Templates does not emit runtime incidents.

## Rules

- Templates must compile against the current platform floor before being
  updated in place.
- Never commit secrets, .env files, or credentials into a template.
