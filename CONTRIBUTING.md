# Contributing to Machinery v1.0

Machinery is a unified platform for Reflective applications. Contributions should respect the four-boundary architecture and coordinated release model.

## Code of Conduct

This project adheres to the Reflective Labs [Code of Conduct](https://github.com/Reflective-Lab/reflective/blob/main/CODE_OF_CONDUCT.md).

## Getting Started

### Prerequisites

- **Rust:** 1.96.0+ (for runtime-runway, commerce-rails)
- **Bun:** 1.3.14+ (for build-depot)
- **Git:** Latest version
- **Just:** 1.x (command runner)

### Setup

```bash
git clone git@github.com:Reflective-Lab/machinery.git
cd machinery
just test  # Verify all 459 tests pass
```

### Branch Naming

Use the Linear issue branch format:

```bash
git checkout -b e{epic}/lin-{RFL-XXXX}-{slug}
```

Example: `e12/lin-RFL-194-storage-contract-refactor`

## Development Workflow

### Before Starting

1. Check which project needs changes (build-depot, runtime-runway, commerce-rails, chart-room)
2. Read the project's README and authority guidelines
3. Verify local setup: `just check && just test`

### Making Changes

#### Single-Project Changes

```bash
cd runtime-runway
cargo check --workspace --all-targets
cargo test --workspace --all-targets
```

#### Multi-Project Changes

Always test across all projects:

```bash
just check    # Type-check all 4 projects
just test     # Run all 459 tests
just ci       # Full CI pipeline
```

#### Code Style

- **Rust:** Follow `rustfmt` (enforced by CI). Run: `cargo fmt --all`
- **TypeScript:** Follow `prettier` in build-depot. Run: `bun run format`
- **Documentation:** Use GitHub Flavored Markdown

### Testing Requirements

| Project | Tests | Command |
|---------|-------|---------|
| build-depot | 25 | `cd build-depot && just test` |
| runtime-runway | 405 | `cd runtime-runway && just test` |
| commerce-rails | 29 | `cd commerce-rails && just test` |
| **Total** | **459** | `just test` |

**No PR merges without all tests passing.**

### Commit Messages

Use conventional commit format:

```
type(scope): short description

Longer explanation if needed. Reference Linear issue:
RFL-123: storage contract update

Breaking changes described explicitly.
```

Types: `feat`, `fix`, `docs`, `refactor`, `test`, `perf`, `chore`
Scopes: `runtime-runway`, `commerce-rails`, `build-depot`, `chart-room`, `machinery`

## Submitting Changes

### Pull Request Process

1. **Create PR:**
   ```bash
   git push -u origin e{epic}/lin-{RFL-XXXX}-{slug}
   gh pr create --title "Description" --body "Details"
   ```

2. **Describe Changes:**
   - Summary of what changed and why
   - Which projects are affected
   - Test results (all 459 pass? or specific subset?)
   - Breaking changes (if any)

3. **Get Review:**
   - Assign to project authority (see AGENTS.md)
   - Address feedback, push new commits
   - Squash only if requested

4. **Merge:**
   - Authority approves
   - All CI checks pass (local via `just ci`)
   - PR is merged to main

### Release Process

Only machinery maintainers can release new versions.

```bash
# On main branch, after merged PRs
git tag -a v1.1 -m "machinery v1.1: Feature summary"
git push origin v1.1
gh release create v1.1 --notes "Release notes"
```

Versioning: `machinery v{major}.{minor}.{patch}`

## Authority & Questions

Each project has an authority responsible for code review and standards:

- **Build-Depot:** Factory Authority (standards, CI/CD)
- **Runtime-Runway:** Deployment Authority (runtime, cloud ops)
- **Commerce-Rails:** Billing Authority (commerce logic)
- **Chart-Room:** Governance (decisions, frameworks)

See `AGENTS.md` for escalation paths and roles.

## Documentation

Update these when relevant:

- **Project README:** High-level overview and setup
- **Code comments:** WHY, not WHAT (well-named code is self-documenting)
- **Architectural decisions:** `build-depot/docs/operations/` for factory-wide, per-project `docs/` for specifics
- **API changes:** Document in `docs/` or code comments with breaking-change notice

## License

By contributing to Machinery, you agree that your contributions will be licensed under the MIT License (see LICENSE).

## Questions?

- **Setup issues:** See individual project README files
- **Standards questions:** Check `build-depot/docs/operations/`
- **Architecture questions:** Read `BOUNDARY_REGISTRY.md` and `AGENTS.md`
- **Process questions:** Check this file or ask a maintainer

---

**Thank you for contributing to Machinery!** 🚀
