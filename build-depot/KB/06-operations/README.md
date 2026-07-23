# Operations & Runtime Handbook

This domain is the living operational truth for local development, deployment,
runtime behavior, secrets, monitoring, and incident response.

## Environment Model

| Environment | Current evidence |
|-------------|------------------|
| Local | Project-local `Justfile`, `package.json`, Cargo workspaces, `.envrc.example` files |
| Web production | Firebase Hosting targets under project `converge-369ad` for `beacon-sites/` properties and apps.reflective.se |
| Wolfgang production | GCP/Firebase project `wolfgang-kb-prod` documented in `studio-apps/wolfgang-chat/deploy` and `infra` |
| Runtime services | Runtime Runway app-host paths, Lattice Mesh distributed work, Commerce Rails ingress, plus retired Converge runtime compatibility paths |

## Deployment Truths

- `converge.zone`, `organism.zone`, `reflective.se`, `axioms.zone`, and `helms.zone` have Firebase deploy workflows under their `beacon-sites/` project directories.
- apps.reflective.se deploys on main changes and embeds Wolfgang web under `/wolfgang-chat/**`.
- Wolfgang has its own backend/frontend deploy docs and Cloud Run/Firebase setup.
- The standalone `converge-runtime` Cloud Run route is retired; current app
  runtime ownership belongs to Runtime Runway and app hosts.
- Some deployment status in older docs may be stale; check project-local Firebase config and workflows first.

## Operations Links

- `../../build-depot/docs/operations/software-factory-quality-system.md` — Software Factory Quality System
- `../../build-depot/docs/operations/quality-gates.md` — Quality Gates
- [[engineering-operations-node|Engineering Operations Node]]
- `../../build-depot/docs/operations/factory-scorecard.md` — Factory Scorecard schema
- [[factory-scorecard|Factory Health Scorecard]]
- [[../deployment-and-infrastructure|Deployment and Infrastructure]]
- [[../security-review|Security Review]]
- [[system-size-metrics|Reflective Labs System Size Metrics]]
- [[local-build-cache|Local Build Cache & Disk Management]]
- [[../workspace-governance/Audits/Security|Workspace Security Audit]]
- [[../workspace-governance/Audits/Compliance|Workspace Compliance Audit]]
- [[../workspace-governance/Workflow/Cheat Sheet|Workflow Cheat Sheet]]

## Runbook Gaps

TODO:

- Confirm live DNS and hosting state for all domains.
- Document rollback per deployed web property.
- Document alerting/dashboards beyond Lighthouse CI and local security tooling.
- Consolidate secret rotation procedures by project.
- Add incident roles and escalation paths once there is a production support loop.
