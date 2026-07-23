# Security Review

## Current State

### Firebase-hosted `beacon-sites/` and portal surfaces
Security headers in project-local `firebase.json` files generally include:
- `X-Frame-Options: DENY`
- `X-Content-Type-Options: nosniff`
- `Referrer-Policy: strict-origin-when-cross-origin`
- `Permissions-Policy: geolocation=(), microphone=(), camera=()`

`converge.zone`, `organism.zone`, `axioms.zone`, and `helms.zone` also set
`X-XSS-Protection: 1; mode=block`. `reflective.se` and apps.reflective.se do
not currently set that legacy header.

### converge.zone (most mature)
- Firebase Auth + WebAuthn passkeys
- Firestore with security rules + indexes
- Cloud Functions (codebase: "api", predeploy: bun build)
- Granular cache control:
  - `/index.html`: `no-cache, no-store, must-revalidate`
  - `/assets/**`: `public, max-age=31536000, immutable`
  - `/decks/**`: `public, max-age=604800` (1 week)
  - `/knowledge/**`: `public, max-age=86400` (1 day)
  - `/signals/feed.xml`: `public, max-age=3600` (1 hour)
  - Images: `public, max-age=31536000, immutable`

### organism.zone, reflective.se, axioms.zone, helms.zone
- Static Firebase-hosted public sites with security headers and SPA rewrites.
- No production auth surface is documented in these site projects.

### apps.reflective.se and Wolfgang web
- apps.reflective.se deploys to Firebase target `marquee-apps`.
- It exposes `POST /api/partner-applications` via Firebase Functions (`submitPartnerApplication`).
- Wolfgang web is built into `/wolfgang-chat/**` from `studio-apps/wolfgang-chat/apps/web`.
- Wolfgang backend security lives in `studio-apps/wolfgang-chat` docs and code, not in the `beacon-sites/` portal.

## Gaps

- [ ] Build-Depot security workflow should publish scheduled audit results into
      Linear or the factory graph once `RFL-162` lands
- [ ] CSP (Content-Security-Policy) — not configured on any site
- [ ] HSTS — not explicitly set
- [ ] Subresource Integrity for Google Fonts
- [ ] Firestore security rules audit
- [ ] Secret management documentation
- [ ] Dependency audit (`bun audit` / `npm audit`)
- [ ] CORS configuration on Cloud Functions
- [ ] Rate limiting on API endpoints

## Recommendations

1. Add CSP headers to all deployed sites
2. Enable HSTS with preload
3. Run dependency audits and set up Dependabot/Renovate
4. Document secret management practices
5. Audit Firestore security rules

## Software Factory Link

Security findings should route through the workspace quality system:
`QUALITY_BACKLOG.md`, Linear, and Build-Depot graph records. Build-Depot's
security runbook lives at `build-depot/docs/operations/security.md`; the
workspace operating model lives at
`build-depot/docs/operations/software-factory-quality-system.md`.
