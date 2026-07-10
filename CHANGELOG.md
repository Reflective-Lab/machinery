# Changelog

All notable changes to Machinery are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed (RFL-195 — Bedrock 4.0.0 migration wave)
- runtime-runway: converge deps repointed from `Reflective-Lab/converge` v3.4.0 to `bedrock-platform` git tag v4.0.0 (registry flip pending RFL-194); 10 dead converge declarations deleted; helm contracts switched from framework/bedrock path deps to the same git tag
- runtime-runway: ported to converge 4.0 API (`ContextView`→`Context` trait, `ContextState`, typed `ProposedFact` construction, `select_chat_backend` via new `manifold-adapters` dep)
- runtime-runway: CI sibling-checkout machinery removed; workflows authenticate via ssh-agent (`SHIPYARD_SSH_KEY`) so CI proves genuine git-tag resolution; `.cargo/config.toml` (git-fetch-with-cli) now checked in
- commerce-rails: reqwest 0.12→0.13 (coordinated wave with runway and quorum-sense); runway-storage path-dep pin corrected to 3.6.0
- Version alignment with bedrock 4.0.0: sha2 0.11, strum 0.28, reqwest 0.13 (`form`/`query` features now explicit)

## [1.1.0] - 2026-07-10

### Security
- Removed absolute user paths (`/Users/kpernyer/...`) from tracked docs across all 4 sub-projects, replacing with `(reflective-root)` placeholder
- Verified no secrets, API keys, or credentials are committed anywhere in the repo
- Confirmed build reproduces cleanly from a fresh clone (Rust check + Bun typecheck + all 459 tests)

### Changed
- Archived repositories (build-depot, runtime-runway, commerce-rails, chart-room) now private on GitHub
- Bedrock v4.0.0 is now explicit prerequisite in CONTRIBUTING.md
- Clarified CI/CD ownership (per-project workflows remain in place)

## Linear Issues
- [RFL-212](https://linear.app/reflective-labs/issue/RFL-212/machinery-v10-consolidate-4-separate-repos-into-unified-platform) — machinery v1.0 consolidation (DONE)

## [1.0.0] - 2026-07-10

### Added

#### Unified Repository
- Consolidated 4 separate repositories into single `machinery` git repo
- Build-Depot, Runtime-Runway, Commerce-Rails, Chart-Room as logical sub-projects
- Root `Justfile` (uppercase) for coordinated commands
- Unified versioning: all 4 projects tagged together as `machinery v1.0`

#### Factory Authority (Build-Depot)
- Factory automation via Trigger.dev integration
- Omnigraph coordination for project health and standards
- Quality gates and delivery pipeline checks
- Scorecard generation for factory metrics

#### Runtime Operations (Runtime-Runway)
- Cloud Run deployment orchestration
- LLM inference engine with Burn + llama.cpp support
- Firebase Auth integration for user identity
- Firestore storage contracts (local + remote)
- Converge SDK runtime integration (v3.4.0)
- **Tests:** 405 comprehensive tests (converge-llm: 310, runway-auth: 39, storage: 14, etc.)

#### Commerce Platform (Commerce-Rails)
- Commercial authority for billing and entitlements
- Stripe Connect integration with webhook processing
- Revenue-share agreements and payout orchestration
- Provider reconciliation and audit trail
- Idempotent command processing with replay gates
- **Tests:** 29 tests (contracts: 12, stripe: 3, entitlements: 14)

#### Governance (Chart-Room)
- Decision evaluation frameworks (10+ lenses)
- Strategic personas for cross-functional thinking
- Schema definitions and fixture data
- Governance contracts for machinery operations

#### Professional Polish
- README with version/test/platform badges
- AGENTS.md documenting roles and workflows
- CONTRIBUTING.md with development guidelines
- SECURITY.md with vulnerability reporting and practices
- LICENSE (MIT)
- CHANGELOG (this file)

### Fixed

- Bedrock path references in Cargo.toml (machinery → framework/bedrock structure)
- Version constraints for helm contracts (0.1.0 → 4.0.0 to match bedrock)
- Build-Depot dependency installation (Bun packages)

### Infrastructure

- Created `Reflective-Lab/machinery` GitHub repository
- Archived old repos (build-depot, runtime-runway, commerce-rails, chart-room)
- Released machinery v1.0 with notes
- Local CI via Justfile (no GitHub Actions yet)

---

## Release Notes by Project

### build-depot v1.0
- Factory automation authority
- 25 passing tests
- Omnigraph integration

### runtime-runway v1.0
- Converge SDK runtime
- 405 passing tests
- Cloud Run deployment ready

### commerce-rails v1.0
- Billing & commerce authority
- 29 passing tests
- Stripe Connect integration

### chart-room v1.0
- Governance & decision frameworks
- 10+ evaluation lenses
- Strategic personas

---

## Total Stats

| Metric | Value |
|--------|-------|
| Total Tests | 459 |
| Commits | 11 |
| Sub-projects | 4 |
| Git Size | 97 MB |
| License | MIT |

---

## Migration Guide (from old repos)

If you were using the old separate repositories, here's how to migrate:

```bash
# Clone the unified machinery repo
git clone git@github.com:Reflective-Lab/machinery.git
cd machinery

# Build-Depot work
cd build-depot && just ci

# Runtime-Runway work
cd ../runtime-runway && just test

# Commerce-Rails work
cd ../commerce-rails && just test

# Run all tests at once from root
cd .. && just test
```

The old repositories (build-depot, runtime-runway, commerce-rails, chart-room) are archived and read-only.

---

## Future Roadmap

- [ ] GitHub Actions CI/CD setup
- [ ] Automated security scanning
- [ ] Performance benchmarking suite
- [ ] API documentation generation
- [ ] Release automation
- [ ] Dependency update automation (Dependabot)
- [ ] Code coverage tracking
- [ ] Integration test suite

---

## See Also

- [GitHub Releases](https://github.com/Reflective-Lab/machinery/releases)
- [Contributing Guide](CONTRIBUTING.md)
- [Security Policy](SECURITY.md)
- [Agents & Roles](AGENTS.md)
