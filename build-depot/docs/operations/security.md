# Security Setup

Build-Depot handles webhook payloads, API tokens, and factory state. The
security posture is built around validation at boundaries, explicit secret
slots, and scheduled dependency/secret checks.

## Local Commands

```bash
just secrets-scan
just security-audit
just security-doctor
```

`just security-audit` runs dependency audit plus the tracked-file secret scan.
It is intentionally separate from `just ci`: advisory verdicts can change when
the outside world changes, even if the commit did not.

## Secret Handling

- Do not commit `.env` files.
- Store production and Trigger secrets in the configured secret manager or
  GitHub Actions secrets.
- Keep secret names documented; never document secret values.
- Webhook secrets are shared only through secret manager-backed configuration.
- Local one-off credentials should come from the operator's environment or
  keychain, not from tracked files.

Required secret names are expected to include:

- `ANTHROPIC_API_KEY`
- `GITHUB_TOKEN`
- `OMNIGRAPH_INGEST_URL`
- `OMNIGRAPH_INGEST_TOKEN`
- `TRIGGER_ACCESS_TOKEN` or the Trigger.dev credential expected by deploy
- provider credentials required by Terraform

Distribution secrets are required only when Build-Depot is publishing private
Rust crates on behalf of a repo such as `bedrock-consolidated`. Terraform may
declare the secret slots before the publish worker is enabled, but the values are
only required once the depot-side publisher is the active ship path:

- `SHIPYARD_SSH_KEY`
- `SHIPYARD_TOKEN`

Those names may be documented, but values stay in the configured secret manager
or CI/Trigger secret store. Public source repos should not carry Shipyard
credential wiring.

## Boundary Validation

- GitHub, Linear, and Sentry payloads must be validated before normalization.
- Unsupported payloads should be skipped explicitly with a reason.
- Missing required identifiers should not produce partial graph records.
- Webhook signature verification belongs at ingress before task routing.

## Scheduled Security

`.github/workflows/security.yml` runs `just security-audit` on a schedule and
by manual dispatch. It is allowed to go red for world-red dependency advisories.
Those failures should create or update Linear work rather than being hidden in
the CI gate.

## Workspace Security Signals

Build-Depot does not delete or replace existing fleet security signals. It
should observe and normalize them:

- dependency audits from project-local `security-audit` recipes
- `cargo audit`, `bun audit`, and equivalent package-manager advisory results
- secret scans, including repo-local scanners where present
- `.github/workflows/hermetic-audit.yml` no-network test audit
- Sentry incidents linked to repository and finding facts
- Linear security findings and accepted risks

Known historical gaps from the workspace CI/CD inventory remain real work until
closed in Linear or the quality ledger: missing local `security-audit` recipes,
missing `deny.toml`, missing Dependabot coverage, and CI-only secret scanning
that operators cannot reproduce locally.

## Security Review Lens

Review security-sensitive changes for:

- secret exposure or logging
- webhook signature verification
- token scopes
- dependency advisories
- unsafe shell expansion in operator commands
- graph ingest authorization
- malformed payload handling
- failure modes that leak payloads or credentials
