# Reliable Delivery Setup

Reliable delivery for Build-Depot means every deploy has the same preflight,
the same operator commands, and a documented rollback path.

## Local Commands

```bash
just delivery-preflight
just deploy
just delivery-doctor
```

`just delivery-preflight` runs the factory doctors and the local CI gate. It is
the command to run before production deployment.

## CI/CD Workflows

- `.github/workflows/ci.yml` runs on push and pull request. It installs Bun and
  Just, then calls `just ci`.
- `.github/workflows/security.yml` runs scheduled and manual security checks via
  `just security-audit`.
- `.github/workflows/delivery.yml` runs delivery preflight on `main` and allows
  manual production deploys through `just deploy`.

Workflows are thin runners. Add or change checks in the Justfile first, then let
CI call the same recipe.

## Deploy Contract

Before deploying:

- `just delivery-preflight` passes.
- Required Trigger.dev and provider credentials exist.
- Webhook URLs and secret slots are configured.
- Omnigraph storage and graph name are known.
- There is a Linear issue or project entry for the change being delivered.

## Private Crate Distribution

The target state is that Build-Depot owns distribution operations for
Reflective private crates. For `bedrock-consolidated`, Cargo manifest
`registry = "reflective-labs"` entries and non-secret registry index
configuration remain structural repo facts because Cargo needs the registry name
while loading workspace metadata. Shipyard credentials, tag reaction, checkout,
topological publish orchestration, and rollback notes belong in Build-Depot.

The target Bedrock flow is:

- `bedrock-consolidated` tags a `v4.*` release after its local `just ci` and
  `just publish-dry` gates are green.
- Build-Depot receives or scans the tag, checks out the tagged revision, and
  performs the topological publish with Build-Depot-held credentials.
- The Bedrock public workspace should remain clone/build/test capable without
  Shipyard credentials.

Current transition sequence:

- If `v4.0.0` must ship before the depot publish worker exists, the in-repo
  Bedrock publish workflow remains the ship path for that release.
- Depot-first shipping blocks the tag until the Trigger.dev publish task exists,
  the Shipyard secret values are loaded into Build-Depot's secret store, and a
  dry-run smoke test has passed against a tagged checkout.
- Only after the depot worker is live should Bedrock strip publish workflows,
  private registry guides, and Shipyard credential wiring. Until then,
  `depot-distribution` should remain a missing adoption signal rather than an
  accepted green.

Deploy:

```bash
just deploy
```

Smoke test:

- trigger a known-safe payload against `debt-tracker`
- confirm normalized records return
- confirm graph ingest succeeds when `OMNIGRAPH_INGEST_URL` is set
- trigger or inspect `pr-gate` on a low-risk PR
- check Trigger.dev task logs for skipped/error counts

Rollback:

- redeploy the previous known-good Trigger.dev version when available
- disable the affected webhook if duplicate or unsafe processing is observed
- keep graph records append-only unless a documented data repair is required
- open or update the Linear incident/finding before continuing feature work

## Delivery Review Lens

Review delivery changes for:

- whether local and CI gates still match
- whether deploy requires undocumented secrets
- whether webhook or task changes are idempotent
- whether failures are visible in logs and graph state
- whether rollback is possible without data loss
