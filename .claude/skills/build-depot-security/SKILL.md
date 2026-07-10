---
name: build-depot-security
description: "Build-Depot security workflow for dependency audits, secret scans, webhook ingress safety, token handling, and security review. Use when checking or changing secrets, GitHub/Linear/Sentry webhook handling, Omnigraph ingest auth, Terraform secrets, or security CI."
---

# Build-Depot Security

Use this skill for security work in the Build-Depot repo.

## Workflow

1. Read `AGENTS.md`, `docs/operations/security.md`, and the relevant code or
   Terraform file.
2. Run:
   - `just security-doctor` for security setup drift.
   - `just secrets-scan` before committing changes that touch docs, workflows,
     Terraform, or task configuration.
   - `just security-audit` for dependency audit plus secret scan.
3. Keep `just security-audit` outside `just ci`. Dependency advisories can be
   world-red without being code-red for the current commit.
4. For webhook work, verify:
   - signature verification happens before routing
   - unsupported payloads are skipped explicitly
   - missing IDs do not create partial graph records
   - logs do not include tokens or raw secrets
5. For Terraform and CI work, document secret names only. Do not commit values.

## Review Checklist

- `.env` files remain ignored.
- New credentials are stored in secret manager or GitHub Actions secrets.
- Workflow permissions stay minimal.
- Tokens are not echoed in logs or persisted to graph records.
- Graph ingest authentication failures are visible and non-destructive.
- Security workflow calls `just security-audit`.
