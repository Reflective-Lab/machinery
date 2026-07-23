---
type: architecture-module
source-path: studio-apps/wolfgang-chat/apps/web/
last-scanned: 2026-06-07
tags: [architecture, studio-apps, wolfgang-chat]
---

# wolfgang-chat — Web

<!-- @generated:start -->

Part of [[Architecture - Overview|wolfgang-chat]]. SvelteKit static frontend, deployed via [[../../beacon-sites/Architecture - Overview|beacon-sites]] at `apps.reflective.se/wolfgang-chat/**`. Talks gRPC-Web to the [[Architecture - Backend|backend]] Cloud Run service.

## Shape

- **Package:** `wolfgang-web` (private workspace package; not published)
- **Framework:** SvelteKit v2.15.0, Svelte v5.0.0
- **Tooling:** Vite 6, TailwindCSS v4, TypeScript 5.7
- **Adapter:** `@sveltejs/adapter-static` v3 — fully static site
- **Base path:** `/wolfgang-chat` (`svelte.config.js`) — mounts under the [[../../beacon-sites/Architecture - Overview|apps.reflective.se]] portal

## Routes

Under `apps/web/src/routes/`:

- `+layout.svelte` + `+layout.ts` — root layout
- `+page.svelte` — home
- `/chat/` — chat interface (the product surface)
- `/login/` — Firebase Auth login
- `/settings/` — settings
- `/upgrade/` — upgrade/billing CTA
- `/download/` — desktop app download page

## Auth + RPC stack

- **Firebase Auth:** `firebase` v12.11.0 (client-side) — issues tokens that the [[Architecture - Backend|backend]] validates via `runway-auth`.
- **gRPC-Web:** `@bufbuild/protobuf` v2 + `@connectrpc/connect` v2 + `@connectrpc/connect-web` v2 — connect-RPC client over HTTP. Talks to the backend's `DocumentService` and `SearchService` (`proto/` definitions).
- **Local UI components:** `@wolfgang/ui` (workspace alias `@wolfgang/ui/*` → `../../packages/ui/*`)
- **Helm flow integration:** `@reflective/helm-flow` (local path) — workflow-UI primitives shared with [[../../bedrock-platform/Architecture - Helms|Helms]].

## Shared UI library: `packages/ui/`

`@wolfgang/ui` — private workspace package. Exports: `./components/*`, `./stores/*`, `./styles/*`, `./assets/*`, `./types/*`, `./themes`.

4 components (`apps/web` is the primary consumer; the desktop app uses its own SvelteKit instance but could share):

- `ChatInput.svelte` — message input
- `ChatMessage.svelte` — single message render
- `ChatShell.svelte` — chat container with message list
- `Sidebar.svelte` — left rail with conversations / knowledgebases

Subdirectories also include `stores/` (Svelte stores), `themes/` (with `index.ts` export), `styles/`, `assets/`, `types/`, `utils/`.

## Boundary

Owns: web frontend product surface, gRPC-Web client, Firebase Auth client flow, shared Svelte UI components.
Does NOT own: chat semantics or LLM calls (→ [[Architecture - Backend|Backend]] for cloud path, [[Architecture - Desktop|Desktop]] for local), persona definitions (→ [[Architecture - Core|Core]]), proto-service definitions (→ [[Architecture - Backend|Backend]]).

## Cross-references

- [[Architecture - Overview|wolfgang-chat overview]]
- [[Architecture - Backend|Backend]] — the gRPC-Web target
- [[Architecture - Desktop|Desktop]] — the parallel compute path
- [[../../beacon-sites/Architecture - Overview|beacon-sites]] — deployment host

<!-- @generated:end -->
