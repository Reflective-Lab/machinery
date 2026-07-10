# runway-app-shell

Shared marquee app shell for Reflective desktop and web clients.

This crate packages the **Rust-side contract** for the shared shell: typed
entitlement projection, Firebase auth bootstrap payloads, and Axum routes that
the Svelte/TS widget layer (`MarqueeTopbar`, `DesktopShell`, entitlement pane)
will consume. The frontend components ship in a follow-up; this scaffold
defines the integration boundary apps must wire against.

## Integration contract

### 1. Mount shell routes on the app host

Apps merge `runway_app_shell::routes(state)` into their `RunwayAppHost` router
behind `runway_auth::AuthLayer` for protected endpoints:

```rust
use runway_app_shell::{ShellConfig, ShellState, routes};

let state = ShellState::new(ShellConfig {
    app_id: packet.app_id.clone(),
    display_name: packet.display_name.clone(),
    route_prefix: packet.route_prefix.clone(),
    auth_domain: std::env::var("FIREBASE_AUTH_DOMAIN")?,
    firebase_api_key: std::env::var("FIREBASE_API_KEY")?,
});

let shell = routes(state);
// host_router.merge(shell)
```

### 2. Entitlement projection (panel-locked schema)

`EntitlementProjection` matches the canonical schema in
`kb/Architecture/App Execution Container.md`. Commerce Rails emits the
authoritative projection; the shell widget renders against this type only.

Hot-path entitlement checks use `CommerceRails::is_entitled(uid, app_id)`.
The projection is fetched once at shell init and refreshed after entitlement
transitions (push refresh + refresh-on-403 retry).

### 3. Auth bootstrap

`GET /v1/shell/auth-bootstrap` returns the Firebase web client config the SPA
needs before sign-in. Values come from `ShellState` — apps must populate
`firebase_api_key` and `auth_domain` from Secret Manager / env at startup.

### 4. Widget endpoints

| Route | Auth | Purpose |
|-------|------|---------|
| `GET /v1/shell/auth-bootstrap` | Public | Firebase client config for SPA init |
| `GET /v1/shell/entitlement` | Protected | `EntitlementProjection` for the signed-in user |

The entitlement handler is a stub returning `entitled: false` until Commerce
Rails wiring lands in the host. Apps replace it by supplying a custom
`EntitlementResolver` in a follow-up PR.

## Boundary

- **Runtime Runway owns** shell transport types, route shapes, and the locked
  `EntitlementProjection` schema.
- **Commerce Rails owns** entitlement truth, Stripe checkout/portal URLs, and
  projection emission.
- **Apps own** product-specific routes and domain meaning only — not a bespoke
  topbar, Firebase bootstrap, or entitlement widget.
