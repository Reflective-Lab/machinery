# Tech Stack

Cross-project audit of frameworks, languages, and tooling. Last narrative pass:
2026-05-31.

## Sites Overview

| Site | Framework | Language | Package Mgr | CSS | Adapter |
|------|-----------|----------|-------------|-----|---------|
| converge.zone | SvelteKit 2 | TypeScript 6 | Bun | CSS custom properties + tokens | adapter-auto |
| organism.zone | SvelteKit 2 | TypeScript 6 | Bun | CSS custom properties | adapter-static |
| reflective.se | Svelte 5 (SPA, no Kit) | TypeScript 6 | Bun | CSS custom properties | N/A (SPA) |
| apps.reflective.se | Svelte 5 (SPA, no Kit) | TypeScript 6 | Bun | CSS custom properties | N/A (SPA) |
| Wolfgang web | SvelteKit 2 | TypeScript 5.7 | Bun | **Tailwind CSS 4** | adapter-auto/static |
| axioms.zone | SvelteKit 2 | TypeScript 6 | Bun | CSS custom properties | adapter-auto/static |
| helms.zone | SvelteKit 2 | TypeScript 6 | Bun | CSS custom properties | adapter-static |

All maintained web properties are Svelte-based. No React app is present in the maintained site surfaces.

## Inconsistencies to Resolve

### CSS approach
- **wolfgang.bot** uses Tailwind 4. All others use hand-rolled CSS custom properties.
- Recommend: pick one and consolidate.

### Package manager
- Current web workflows use Bun. `beacon-sites/www.converge.zone` still has a `package-lock.json`, so treat npm metadata there as legacy unless a project-local README says otherwise.

### TypeScript version
- Wolfgang web remains on TypeScript 5.7 while the `beacon-sites/` web properties are on TypeScript 6.
- Helm desktop packages are still on TypeScript 5.7 and Vite 6.

## Backend / Platform

| Concern | Stack |
|---------|-------|
| Hosting | Firebase Hosting (project: converge-369ad) |
| Functions | Firebase Functions for apps.reflective.se (`marquee-apps`, Node 22); converge.zone no longer routes `/api/**` to standalone `converge-runtime` |
| Auth | Firebase Auth + WebAuthn passkeys (converge.zone) |
| Database | Firestore where project-local `firebase.json` includes rules/indexes (`converge.zone`, `reflective.se`, `apps.reflective.se`) |
| VCS | Git + Jujutsu (jj) where available |
| Task runner | just (Justfile) where available |
| CI/CD | GitHub Actions: most `beacon-sites/` sites deploy on `v*` tags; apps.reflective.se deploys on `main` changes; Wolfgang web is built into apps.reflective.se |
| Monitoring | Lighthouse CI (converge.zone only) |

## Rust Platform Projects
- Edition 2024, rust-version 1.94
- Clippy pedantic
- `unsafe` forbidden
- Workspace deps with `workspace = true`

| Area | Current home |
|------|--------------|
| Bedrock platform | `bedrock-platform/` (`2`/Helm, `axiom`, `organism`, `converge`) |
| Mosaic extensions | `mosaic-extensions/` |
| Atelier Showcase | `atelier-showcase/` |
| Arena Tests | `arena-tests/` |
| Runtime Runway | `runtime-runway/` |
| Commerce Rails | `commerce-rails/` |
| Marquee apps | `marquee-apps/` |
