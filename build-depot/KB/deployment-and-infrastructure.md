# Deployment and Infrastructure

## Firebase Hosting

**Project**: `converge-369ad`

| Site | Firebase Target | Status |
|------|----------------|--------|
| converge.zone | default | Deployed (GitHub Actions on v* tags) |
| organism.zone | organism | Deployed (GitHub Actions on v* tags) |
| reflective.se | reflective | Deployed (GitHub Actions on v* tags) |
| axioms.zone | axioms | Firebase target configured; GitHub Actions on v* tags |
| helms.zone | helms | Firebase target configured; GitHub Actions on v* tags |
| apps.reflective.se | marquee-apps | GitHub Actions on main changes; embeds Wolfgang web at `/wolfgang-chat/**` |
| Wolfgang web | marquee-apps subpath | Built from `studio-apps/wolfgang-chat/apps/web` into apps.reflective.se |

### Firebase Services Used
- **Hosting**: Static site serving with SPA rewrites
- **Cloud Run rewrite**: no active standalone Converge runtime rewrite; the
  former converge.zone `/api/**` -> `converge-runtime` route was retired on
  2026-06-02
- **Cloud Functions**: apps.reflective.se function `submitPartnerApplication` in codebase `marquee-apps`
- **Auth**: Firebase Auth + WebAuthn passkeys (converge.zone)

## CI/CD (GitHub Actions)

### Configured workflows
- **converge.zone**: `deploy.yml` (build + deploy on v* tags), `lighthouse.yml` (performance audits)
- **organism.zone**: `deploy.yml` (build + deploy on v* tags)
- **reflective.se**: `deploy.yml` (build + deploy on v* tags)
- **axioms.zone**: `deploy.yml` (build + deploy on v* tags)
- **helms.zone**: `deploy.yml` (build + deploy on v* tags)
- **apps.reflective.se**: `deploy.yml` (build + deploy on main changes; also builds Wolfgang web)
- **Wolfgang web**: `studio-apps/wolfgang-chat/.github/workflows/deploy-web-v2.yml` builds the web app and deploys it into the apps.reflective.se portal

### Deploy pattern
Most standalone `beacon-sites/` sites follow this pattern:
1. Push a `v*` tag
2. GitHub Actions triggers build
3. Deploys to Firebase Hosting

apps.reflective.se is different: it deploys on `main` changes and copies the
Wolfgang web build into the portal before deployment.

## Security Headers (from firebase.json)

converge.zone sets:
```json
"X-Frame-Options": "DENY",
"X-Content-Type-Options": "nosniff",
"X-XSS-Protection": "1; mode=block",
"Referrer-Policy": "strict-origin-when-cross-origin"
```
Cache control: 1 hour for HTML, 1 year for hashed assets.

## apps.reflective.se Secrets

Secrets for the `marquee-apps` Firebase Functions are stored in GCP Secret Manager under project `converge-369ad`.

### APPLICATION_NOTIFY_WEBHOOK_URL

Make.com webhook that receives a notification each time a partner application is submitted with ≥ 74% readiness. The Cloud Function resolves the secret at runtime via `defineSecret`; no redeploy is needed when the URL changes.

**Rotate the webhook URL** (no redeploy required — takes effect on next cold start):

```bash
echo -n "https://hook.eu1.make.com/NEW_WEBHOOK_ID" | \
  gcloud secrets versions add APPLICATION_NOTIFY_WEBHOOK_URL \
    --project=converge-369ad \
    --data-file=-
```

**First-time setup** (only needed once):

```bash
gcloud secrets create APPLICATION_NOTIFY_WEBHOOK_URL \
  --project=converge-369ad \
  --replication-policy=automatic

echo -n "https://hook.eu1.make.com/YOUR_WEBHOOK_ID" | \
  gcloud secrets versions add APPLICATION_NOTIFY_WEBHOOK_URL \
    --project=converge-369ad \
    --data-file=-

# Deploy once so Firebase registers the secretAccessor IAM binding
firebase deploy --only functions --project converge-369ad
```

**Inspect versions:**

```bash
gcloud secrets versions list APPLICATION_NOTIFY_WEBHOOK_URL --project=converge-369ad
```

**Local development:** create `functions/.env` (gitignored) with:
```
APPLICATION_NOTIFY_WEBHOOK_URL=https://hook.eu1.make.com/your-dev-webhook
```
The emulator reads `.env` files; Secret Manager is not used locally.

---

## Google Cloud / Terraform

> **TODO**: Audit GCP project settings, IAM, Terraform state if any exists.
> Known project-local infra docs exist in `studio-apps/wolfgang-chat/infra`
> and `studio-apps/wolfgang-chat/deploy`.

## DNS

> **TODO**: Document DNS configuration for all domains.
> - converge.zone
> - organism.zone
> - reflective.se
> - apps.reflective.se
> - wolfgang.bot / Wolfgang web routing
> - axioms.zone
> - helms.zone
