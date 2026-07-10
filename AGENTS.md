# Machinery Agents & Roles

Machinery v1.0 is coordinated through these agent roles, each responsible for a specific domain.

## Factory Authority

**Build-Depot** — Machinery operations, standards, and factory coordination

- **Role:** Coordination authority for all 4 machinery projects
- **Responsibility:** Quality gates, CI/CD standards, factory health checks
- **Interface:** `just doctor`, `just factory-adoption`, `just scorecard`
- **Documentation:** `build-depot/docs/operations/`

### Runbooks
- Project health: Run `just doctor` to check factory drift
- Machinery adoption: Run `just factory-adoption` to verify all projects meet standards
- Quality gates: See `build-depot/docs/operations/quality-gates.md`

---

## Build-Depot (Node.js/Bun)

**Factory automation, Trigger.dev integration, Omnigraph coordination**

- **Tests:** 25 (Node.js, TypeScript)
- **Build:** `cd build-depot && just check`
- **Test:** `cd build-depot && just test`
- **CI:** `cd build-depot && just ci`

### Key Recipes
- `just doctor` — Factory drift checks
- `just security-audit` — Dependency audit + secret scan
- `just scorecard` — Emit factory scorecard as JSON

---

## Runtime-Runway (Rust)

**Cloud deployment, runtime orchestration, LLM inference**

- **Tests:** 405 comprehensive tests
  - converge-llm: 310 tests (adapter lifecycle, inference, embeddings)
  - runway-auth: 39 tests (Firebase middleware)
  - runway-storage: 14 tests (Firestore contracts)
  - runway-app-host: 9 tests
  - converge app: 12 tests
  - runway-ambient: 14 tests
- **Build:** `cd runtime-runway && cargo check`
- **Test:** `cd runtime-runway && cargo test`

### Key Crates
- `runtime-runway/crates/api-server` — Cloud Run entry point
- `runtime-runway/crates/runway-auth` — Firebase auth integration
- `runtime-runway/crates/runway-storage` — Local & remote storage
- `runtime-runway/crates/llm` — LLM inference engine

---

## Commerce-Rails (Rust)

**Payment processing, billing, commerce operations, Stripe integration**

- **Tests:** 29 passing
  - contracts: 12 tests
  - stripe: 3 tests
  - entitlements: 14 tests
- **Build:** `cd commerce-rails && cargo check`
- **Test:** `cd commerce-rails && cargo test`

### Key Crates
- `commerce-rails/crates/commerce-rails-contracts` — Commerce vocabulary
- `commerce-rails/crates/commerce-rails-stripe` — Stripe provider adapter

---

## Chart-Room (Documentation)

**Governance, decision criteria, evaluation frameworks, personas**

- **Content:** Evals, personas, schemas, strategic frameworks
- **Build:** Documentation only (no CI)
- **Role:** Supports decision-making across all 4 projects

---

## Workflow

### Standard Machinery Development

1. **Branch:** Create feature branch from main
   ```bash
   git checkout -b e{N}/lin-{RFL-XX}-{slug}
   ```

2. **Develop:** Work in individual project
   ```bash
   cd runtime-runway
   just check && just test
   ```

3. **Verify:** Run machinery-wide checks
   ```bash
   just ci              # From machinery root
   ```

4. **Push & PR:** Push branch and create PR
   ```bash
   git push -u origin e{N}/lin-{RFL-XX}-{slug}
   gh pr create --title "Feature description"
   ```

5. **Release:** Tag machinery version
   ```bash
   git tag -a v1.1 -m "machinery v1.1: Feature additions"
   git push origin v1.1
   gh release create v1.1 --notes "Release notes"
   ```

### Cross-Project Changes

When changes span multiple projects (e.g., bedrock upgrade):

1. Coordinate in `build-depot` (authority)
2. Update all affected projects
3. Run `just test` to verify all 459 tests pass
4. Create single PR (machinery moves together)

---

## Authority & Escalation

- **Configuration & Standards:** Build-Depot (factory authority)
- **Runtime Operations:** Runtime-Runway (deployment authority)
- **Commerce State:** Commerce-Rails (billing authority)
- **Governance:** Chart-Room (decision frameworks)

### Escalation Path

1. **Question about project health?** → `just doctor`
2. **Question about standards?** → `build-depot/docs/operations/`
3. **Question about deployments?** → `runtime-runway/docs/`
4. **Question about commerce logic?** → `commerce-rails/README.md`
5. **Question about governance?** → `chart-room/`

---

## Resources

- **Repository:** https://github.com/Reflective-Lab/machinery
- **Release:** https://github.com/Reflective-Lab/machinery/releases
- **License:** MIT (see LICENSE)
- **CI/CD:** Local via Justfile (GitHub Actions not configured)
- **Archived Repos:** build-depot, runtime-runway, commerce-rails, chart-room (read-only)

---

## See Also

- `KB/05-engineering/standards/ci-parity.md` — CI/CD standards
- `BOUNDARY_REGISTRY.md` — Authority table
- `build-depot/docs/operations/machinery-project-standards.md` — Technical standards
