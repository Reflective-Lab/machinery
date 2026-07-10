# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 1.0.x | ✅ Current |
| < 1.0 | ❌ No |

## Reporting a Vulnerability

**Do not open public issues for security vulnerabilities.**

Please report security vulnerabilities to security@reflective-labs.io with:

1. Description of the vulnerability
2. Steps to reproduce
3. Potential impact
4. Suggested fix (if available)

We will acknowledge your report within 48 hours and provide a timeline for a fix.

## Security Practices

### Dependency Management

- **Runtime-Runway** and **Commerce-Rails** (Rust):
  - `cargo audit` runs in CI/CD
  - Ignored vulnerabilities tracked in `.audit-ignores`
  - Monthly dependency updates

- **Build-Depot** (Node.js/Bun):
  - `bun audit` for dependency scanning
  - Locked dependency versions in `bun.lock`
  - Security audit: `just security-audit`

### Code Security

#### Rust Projects
- `unsafe` code forbidden (clippy lint: `unsafe_code = "forbid"`)
- Network requests require explicit allow in tests (RP-HERMETIC-UNIT)
- Secrets managed via GCP Secret Manager (runtime-runway)
- Request signing enforced (commerce-rails/Stripe webhooks)

#### Node/TypeScript Projects
- TypeScript strict mode enforced
- Input validation with zod (build-depot)
- Webhook signature verification

### Deployment Security

- Secrets stored in environment variables (never in code)
- Cloud Run deployment with IAM roles (runtime-runway)
- Firebase Auth for user authentication
- Stripe webhook signature verification

### Secret Scanning

Before committing:

```bash
just security-audit  # Scans for committed secrets
git commit           # Pre-commit hook runs secret scan
```

Scanned secrets:
- AWS keys
- GitHub tokens
- Stripe API keys
- Firebase credentials
- GCP service account keys

### Cryptography

- HMAC-SHA256 for webhook signatures
- Firebase JWT verification (offline JWKS)
- blake3 for hashing
- secrecy crate for sensitive data (zeroized on drop)

## Build & Release Security

1. **CI Checks:** All tests pass before merge
2. **Code Review:** Authority approval required
3. **Signed Commits:** Recommended (not enforced)
4. **Release Tags:** Annotated with GPG signature (when available)

## Known Limitations

- **bedrock formatting drift:** Minor formatting issues in transitive dependencies (tracked, non-security impact)
- **GitHub Actions not configured:** CI runs locally via Justfile
- **No automated scanning:** Manual security audits via `just security-audit`

## Security Headers & Best Practices

### Runtime-Runway (Axum HTTP Server)
- CORS policy: Configurable per deployment
- Request tracing: OpenTelemetry instrumentation
- Error handling: No sensitive data in error responses

### Commerce-Rails (Stripe Integration)
- Webhook verification required on all Stripe events
- Idempotency keys for payment operations
- Audit trail for all commercial commands

### Build-Depot (Trigger.dev Integration)
- Environment variables for secrets
- GitHub OAuth (never stored)
- Webhook signature validation

## Compliance

- **License:** MIT
- **Data Handling:** Follows Reflective Labs privacy policy
- **Audit Trail:** Commerce operations are fully auditable (commerce-rails)
- **Encryption in Transit:** TLS 1.2+ for all external communication

## Security Contacts

- **Primary:** security@reflective-labs.io
- **Emergency:** (contact Reflective Labs leadership)

---

For additional context, see:
- `build-depot/docs/operations/security.md`
- `.audit-ignores` in individual projects
- `CONTRIBUTING.md` for development practices
