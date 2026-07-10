# Machinery v1.0

Unified operational platform for Reflective applications. One repository, four strong boundaries, coordinated authority.

```
machinery/
├── build-depot/          ⭐ Factory Authority (Node.js/Bun)
├── runtime-runway/       Runtime orchestration (Rust, 405+ tests)
├── commerce-rails/       Commerce operations (Rust, 14+ tests)
└── chart-room/           Governance & evaluation (Documentation)
```

## Quick Start

### Check the machinery
```bash
just check              # Type-check all projects
just test               # Run all tests
just ci                 # Full CI pipeline
```

### Factory operations (Build-Depot authority)
```bash
just doctor                   # Factory drift checks
just factory-adoption         # Machinery adoption metrics
just scorecard                # Factory scorecard
just security-audit           # Security audits across all projects
```

### Individual projects
```bash
cd build-depot && just ci     # Full build-depot pipeline
cd runtime-runway && just test # 405 tests
cd commerce-rails && just test # 14 tests
cd chart-room && just --list   # Docs/evaluation commands
```

## Project Overview

### 🔑 Build-Depot (Authority)
**Source of truth for machinery standards and configuration.**

- Factory automation and Omnigraph coordination
- Trigger.dev integration
- Project health checks and standards
- Configuration authority (registry, CI/CD parity, quality gates)
- See: `build-depot/docs/operations/`

### 🚀 Runtime-Runway
**Distribution, deployment, and infrastructure.**

- Converge SDK runtime
- Cloud Run deployment (GCP)
- Firebase auth integration
- LLM inference & GPU paths
- 405 comprehensive tests (converge-llm: 310, runway-auth: 39, storage: 13, etc.)
- See: `runtime-runway/README.md`

### 💳 Commerce-Rails
**Commercial authority and billing.**

- Customer accounts and subscriptions
- Revenue-share and payout operations
- Stripe Connect integration
- Webhook processing and reconciliation
- 14 tests
- See: `commerce-rails/README.md`

### 📋 Chart-Room
**Governance, decisions, and evaluation.**

- Decision criteria and evaluation frameworks
- Strategic personas (10+ lenses: founder, QA, SRE, security, etc.)
- Fixture data and schemas
- See: `chart-room/README.md`

## Architecture

### Single Repository, Four Boundaries

**machinery** is ONE git repository with unified versioning:
- All 4 projects share a version: `machinery v1.0`
- Commit history from each project is preserved (via subtree merge)
- Coordinated releases: "machinery v1.0" tags the whole platform

Each project maintains strong boundaries:
- Own `Justfile` (isolated build recipes)
- Own `Cargo.toml` / `package.json` (independent dependencies)
- Own tests and documentation
- Own `.cargo/config.toml` (registry configuration)

**Authority flows through Build-Depot:**
- `build-depot/docs/operations/machinery-project-standards.md` — defines standards
- Each project's README points to build-depot for setup guidance
- Factory checks (`just doctor`) verify machinery-wide health

## Configuration

### Registry Configuration (Rust Projects)

Both Rust projects require the `reflective-labs` Shipyard registry:

```toml
# In commerce-rails/.cargo/config.toml and runtime-runway/.cargo/config.toml
[registries.reflective-labs]
index = "ssh://git@ssh.shipyard.rs/reflective-labs/crate-index.git"

[net]
git-fetch-with-cli = true
```

See `build-depot/docs/operations/machinery-project-standards.md` for details.

## Testing

| Project | Tests | Framework |
|---------|-------|-----------|
| build-depot | 25 | Bun test |
| runtime-runway | 405 | Cargo test |
| commerce-rails | 14 | Cargo test |
| chart-room | — | (Docs only) |
| **Total** | **444** | — |

Run all: `just test`

## CI/CD

### Local CI (before push)
```bash
just ci
```

### Machinery CI Pipeline (from root)
1. **build-depot**: Full pipeline (fmt-check → check → lint → test)
2. **commerce-rails**: Check + test (fmt-check blocked on bedrock drift)
3. **runtime-runway**: Check + test (fmt-check blocked on bedrock drift)

### Known Issue: bedrock Formatting
Both Rust projects fail `cargo fmt --all -- --check` due to formatting issues in bedrock's transitive dependencies (not machinery code). **Workaround:** Run individual stages (`just check`, `just test`) instead of full `just ci`.

## Versioning

**machinery v1.0** represents the unified platform:
- All 4 projects versioned together
- Breaking changes → machinery v1.1
- Release notes document changes across all 4 projects

## See Also

- **Root standards:** `build-depot/docs/operations/machinery-project-standards.md`
- **Quality system:** `build-depot/docs/operations/software-factory-quality-system.md`
- **Boundary registry:** `/reflective/BOUNDARY_REGISTRY.md` (canonical authority claims)
- **KB:** `/reflective/KB/` (Obsidian vault for deep dives)
