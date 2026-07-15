---
tags: [contract, runway, marquee-apps, seam]
source: mixed
verified-against: quorum-sense 2026-07-14
---
# Marquee App Seam — Runtime Runway

**Design status:** Done — see [App Execution Container](../Architecture/App%20Execution%20Container.md) (authoritative).

This page is the **bootstrap checklist** for App #2. Do not duplicate the
architecture doc; copy Quorum's shape.

**Umbrella:** [Bedrock marquee-app-machinery-seam.md](../../../../framework/bedrock/kb/06-consumption/marquee-app-machinery-seam.md)  
**Workspace:** [`BOUNDARY_REGISTRY.md`](../../../../BOUNDARY_REGISTRY.md)  
**Reference app:** `applications/marquee-apps/quorum-sense`

---

## What Runtime Runway owns (crisp test)

*"Who can act and where the code runs."*

Auth, middleware, telemetry, secrets, storage kit, SPA serving, deployment
packaging, `AppExecutionPacket` / `runway.app.json`, session ownership (when
shipped). **Not** domain sensemaking, commercial state, or Bedrock formation logic.

---

## Consumer crates (app workspace)

Path-dep from app `Cargo.toml` into `machinery/runtime-runway/crates/`:

| Crate | Typical use |
|-------|-------------|
| `runway-app-host` | `RunwayAppHost::builder`, `AppExecutionPacket`, `HelmModule` re-exports |
| `runway-auth` | Firebase middleware, auth context |
| `runway-middleware` | trace, CORS, request-id, health |
| `runway-telemetry` | OTel + Sentry bootstrap |
| `runway-secrets` | GCP Secret Manager |
| `runway-storage` | `StorageKit` — events, documents, vectors |
| `runway-accounts` | accounts routes + Stripe webhook mount (with Commerce) |
| `runway-ambient` | optional ambient integration |

**Import rule:** only `{app}-server` (tier 4). Never `{app}-app` for host concerns.

---

## `runway.app.json` (minimum fields)

```json
{
  "app_id": "your-app",
  "route_prefix": "/your-app",
  "auth_app": "your-app",
  "domain_routes": [{ "method": "POST", "path": "/...", "owner": "app-domain" }],
  "mounted_modules": [{ "module_id": "helm.operator-control", "mount_kind": "planned" }],
  "boundaries": [{
    "layer": "app",
    "owns": ["domain semantics"],
    "consumes": ["Runtime Runway host", "commerce-rails entitlement contract", "Helm operator-control"],
    "status": "active"
  }],
  "deploy_contracts": [{ "key": "commerce-rails-deploy", "version": "0.2.0" }]
}
```

- `mount_kind: "planned"` until live Helm state is wired — never fake `"mounted"`.
- `domain_routes` must match actual handlers (RR D1 manifest verifier).

---

## Host wiring (pattern)

```rust
let packet = AppExecutionPacket::from_json_str(include_str!("../../../runway.app.json"))?;
let host = RunwayAppHost::builder(packet)
    .with_storage(storage)
    .mount(helm_operator_module)
    .mount(app_domain_module)
    .build()
    .await?;
host.serve().await?;
```

Domain HTTP: `domain_host::wire_domain_routes` or `HelmModule` impl — product
routes only; generic host stays in Runway.

---

## Bedrock alignment (fleet pin)

`runway-app-host` depends on Bedrock `helm-module-contracts` and
`helm-event-substrate`. Those **must** use the **same `bedrock-platform` tag** as
the marquee app's Bedrock deps, or `HelmModule` types split across two git
checkouts.

When bumping Bedrock in the app workspace, bump helm deps in
`machinery/runtime-runway` in the **same train**.

---

## Anti-patterns (Marquee App Contract)

- App-owned Axum server crate duplicating Runway middleware stack
- Auth / secrets / telemetry bootstrap in `{app}-app`
- Test-only routes in production binary
- GraphQL-per-app without portfolio contract
- Importing `runway-app-host` outside `{app}-server`

---

## New app checklist

- [ ] Path deps to required `runway-*` crates in workspace `Cargo.toml`
- [ ] `runway.app.json` at app repo root (or path cited by packet loader)
- [ ] `{app}-server` composes `RunwayAppHost` — no parallel host
- [ ] `boundary-manifest.toml` forbids runway crates in tier 0–3
- [ ] `boundaries[]` in packet cites this contract + Commerce contract
- [ ] Bedrock helm tag aligned with app + runway checkout
- [ ] `--max-instances=1` until RR D5 session ownership ships

---

## Related

- [App Execution Container](../Architecture/App%20Execution%20Container.md)
- [Commerce Marquee App Seam](../../commerce-rails/kb/Contracts/Marquee%20App%20Seam.md)
- Quorum: `runway.app.json`, `crates/quorum-server/src/domain_host.rs`
