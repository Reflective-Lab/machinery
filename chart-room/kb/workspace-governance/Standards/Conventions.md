---
tags: [standard]
source: llm
---
# Cross-Project Conventions

## Version Control
- Use `jj` when available, fall back to git
- Never push to main without confirmation
- Never commit secrets, .env files, or credentials
- .gitignore must cover: `**/target/`, `node_modules/`, `build/`, `dist/`, `.svelte-kit/`, `.env`, `.env.*`, `!.env.example`, `.DS_Store`
- Code must compile and lint clean before pushing to main

## Rust Projects
- Edition 2024, rust-version 1.94 (aligned to converge)
- `unsafe` code is forbidden
- Clippy pedantic with allowed exceptions
- All deps use `workspace = true` in converge

## JS/TS Projects
- Bun for package management
- SvelteKit for web apps
- Tauri for desktop apps

## Build & Task Runner
- Use `just` when a Justfile exists
- Run `just lint` (or equivalent) before considering work done
- Justfiles should provide at minimum: `check`, `lint`, `test`, `dev`
- For JS-only projects without `just`: `bun run check` is the minimum

## Cloud & Infrastructure
- **Terraform** for all infrastructure-as-code — no manual resource creation
- **Firebase** for auth, Firestore, hosting where applicable
- **gcloud CLI** scripts for operational checks and quick fixes (not for provisioning)
- Cloud resources must be defined in terraform, not created ad-hoc
- Firebase config lives in project root (`firebase.json`, `.firebaserc`)

## Skills & Process
- All projects share the same 13 canonical workflow names: `audit`, `check`, `deploy`, `dev`, `done`, `fix`, `focus`, `next`, `pr`, `review`, `sync`, `ticket`, `wip`
- Codex, Claude, and Gemini docs should use the same canonical workflow names
- Codex may implement some names through legacy backing skills such as `checkpoint` for `/done` and `quality` for `/check`
- The done/checkpoint workflow must reference: `MILESTONES.md`, `CHANGELOG.md`, `~/dev/reflective/bedrock-platform/EPIC.md`
- Skills must not drift between projects without reason
- Justfile recipes should align with skill names where possible (e.g., `just dev` ↔ `/dev`)

## Architecture Layers
- Use the same five-layer vocabulary across projects: `Helm → Axiom → Organism → Converge → Providers`
- **Helm** is the control surface: desktop or web UI, operator workflows, local commands
- **Axiom** is the truth layer: app state, truth definitions, projections, validation, the "what"
- **Organism** is the intelligence layer: intent, huddle, debate, suggestors, research, gap-chasing, the "how"
- **Converge** is the governance layer: engine, promotion gates, Cedar policy, budgets, convergence, audit, the "whether"
- **Providers** are the capability layer: OpenRouter, Anthropic, OpenAI, Gemini, Brave, Tavily, MCP-backed services
- Axiom initiates `Engine.run()`, Organism emits `ProposedFact` and `AgentEffect`, and Converge decides what can become fact
- Providers stay behind capability boundaries such as `ChatBackend`, `WebSearchBackend`, `DdLlm`, `DdSearch`, and MCP clients; they must not leak into Helm directly

## Dependencies
- Product repos own Helm and Axiom, and may depend on Organism and/or Converge
- Organism is a client of Converge — uses Converge types directly
- Converge depends on no application layer above it
- Wolfgang depends on Converge — build missing capabilities in Converge first
- saas-killer depends on Converge and Organism
- epic-brand pins to same Converge rev as Wolfgang
