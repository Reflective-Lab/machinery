# App Execution Container

> **Boundary cross-links** (per panel review 2026-06-15):
> - Workspace anchor (read first): `/Users/kpernyer/dev/reflective/BOUNDARY_REGISTRY.md`
> - Active implementor handoff: `/Users/kpernyer/dev/reflective/HANDOFF_quorum-sense_2026-06-15.md` (signed by all three architects 2026-06-15)
> - Commerce-Rails authority: `/Users/kpernyer/dev/reflective/commerce-rails/kb/Architecture/Operating Authority Boundary.md`
> - Helms authority: `/Users/kpernyer/dev/reflective/bedrock-platform/helms/kb/Architecture/Operating Authority Boundary.md`
> - Frozen panel review (historical): `/Users/kpernyer/dev/reflective/REVIEW_quorum-sense_2026-06-15.md`

Runtime Runway owns the standard server execution container for Reflective apps.

This is a hard architectural direction: marquee apps should not each invent
their own HTTP, gRPC, GraphQL, auth, telemetry, secrets, realtime, or deployment
host. Apps instantiate the Runtime Runway container with an app packet. Helm mounts
operator-control and governed-job modules into that container. Axiom, Organism,
Converge, and Mosaic keep their own lower-layer authority.

## Decision

The deployable unit for an app backend should be:

```text
Runtime Runway execution container
  -> Runtime Runway auth, middleware, telemetry, secrets, storage, and deployment
  -> Helm operator-control and governed-job routes
  -> app packet: app id, truths, projections, subject refs, fixtures, copy
  -> optional domain routes when the app has real product-specific HTTP
```

The app should not own the generic server. It owns domain meaning.

## Why This Exists

The current `application-server` shape in Helm is useful, but it mixes two
responsibilities:

- a generic execution host that any app needs;
- Helm-specific operator-control, jobs, approvals, and projections.

That mix is a bad smell. It encourages Catalyst, Tally, Quorum, Fathom, Atlas,
and later apps to either call a Helm server as if it were the platform, or to
copy the same server concerns into each app. The right split is to move the
host responsibility to Runtime Runway and keep Helm as a mounted operator module.

## Ownership

| Layer | Owns | Must not own |
|---|---|---|
| Runtime Runway | process lifecycle, ports, Cloud Run packaging, health, auth, middleware, CORS, secrets, telemetry, storage, append-only event log, public transport defaults | domain truth semantics, operator-control authority, convergence rules, specialist cores |
| Helm | operator-control read models, governed job surface, HITL approvals, readiness packets, receipt views, workbench/client contracts | deployment substrate, app-specific business authority, lower-layer specialist implementations |
| App | product UX, domain truths, fixtures, app subject refs, projections, copy, product-specific routes | reusable server host, generic realtime parsing, auth/secrets/telemetry stacks, generic HTTP/gRPC/GraphQL frameworks |
| Axiom | truth validation, intent artifacts, compiled invariants, calibration doctrine | hosting, operator control, deployment |
| Organism/Converge/Mosaic | formations, fixed-point execution, promotion, receipts, specialists | app deployment topology |

## Packet Shape

The current implementation is the canonical reference shape — verified by quorum-sense in panel review 2026-06-15 (`REVIEW_quorum-sense_2026-06-15.md`). The `runway-app-host` crate exposes the packet and host bootstrap as a typed builder:

```rust
let packet = AppExecutionPacket::from_json_str(include_str!("../../../runway.app.json"))?;

RunwayAppHost::builder(packet)
    .with_storage(storage)
    .mount(Arc::new(operator_control_module))   // Helm modules
    .mount(Arc::new(governed_jobs_module))
    .mount(app_domain_module)                    // app's HelmModule impl
    .build()
    .await?
    .serve()
    .await?
```

The app packet (`runway.app.json`) carries:

- `app_id`, display metadata, version, route_prefix, auth_app;
- truth/job registrations;
- operator-control packet registrations;
- subject-ref codecs;
- fixture/demo seeds;
- `domain_routes: [{method, path, owner}]` — declarative manifest of every route, enforced by D1 verifier;
- `mounted_modules: [{module_id, mount_kind: "planned"|"mounted", routes}]` — `"mounted"` means live state wired, `"planned"` means default-shell;
- `boundaries: [{layer, owns, consumes, status}]` — the per-layer ownership claim;
- `deploy_contracts: [{key, version}]` — declarations of CR / commerce / other-layer recipes the deploy template must materialize. The app never writes provider-specific env-var names (e.g. `STRIPE_PRICE_*`). Materialized by the RR deploy template against CR's `commerce-rails-deploy` recipes.

It must not carry auth implementation, middleware implementation, telemetry bootstrap, event-log implementation, or container/deployment policy.

## EntitlementProjection schema (canonical, panel-locked 2026-06-15)

Locked field set for `entitlement_projection(uid, app_id) -> EntitlementProjection`:

```rust
struct EntitlementProjection {
    entitled: bool,
    checkout_url: Option<String>,
    portal_url: Option<String>,
    signup_url: Option<String>,
    next_renewal: Option<DateTime<Utc>>,
    plan_label: Option<String>,
}
```

Adding new optional fields is non-breaking. Renaming or removing a field requires a new dated panel review. The `runway-app-shell` widget (RR scope, D3b) and CR's projection emitter (CR-05) build against this schema without back-channel checks.

Hot-path callers use `CommerceRails::is_entitled(uid, app_id) -> bool` — fast, cacheable up to JWT validity, NEVER beyond. Projection is called once at shell init and after entitlement transitions. Push refresh (CR-06) + refresh-on-403 retry handles failure-mode coherence.

## Protocol Defaults

HTTP plus SSE is the default public app surface because browsers and desktop
webviews consume it directly.

gRPC is for internal typed service-to-service paths or lower-level runtime
streams where the client is controlled.

GraphQL is not a default app backend style. It can become a read/query facade
later if portfolio-wide projection browsing needs it, but it should not be
introduced independently by each app.

## Migration Path

1. Keep Helm `application-server` working as the reference host while the
   contract is extracted.
2. Define a typed app packet around the Catalyst proof first.
3. Extract Runtime Runway host construction from `crates/api-server` into
   `crates/runway-app-host`.
4. Mount Helm operator-control/job routes into that Runtime Runway host.
5. Move Catalyst from "calls Helm application-server" to "runs in Runtime Runway
   container with Helm module mounted".
6. Repeat with Tally, Quorum, Fathom, Warden, Plumb, Atlas, and the rest of the
   marquee apps.
7. Retire or shrink any app-local server crates to product-specific route
   adapters.

## Guardrails

- No new app-owned server framework unless it is explicitly temporary and has a
  deletion path.
- No app-local auth, secrets, telemetry, CORS, health, or deployment bootstrap.
- No app-local realtime parser when the Helm/Runtime Runway envelope exists.
- No GraphQL-per-app experiment until there is a portfolio-level query contract.
- New generic routes belong in Helm if they are operator-control semantics, and
  in Runtime Runway if they are host/runtime/deployment semantics.

## Immediate Priority

For the rest of the current workday, this is higher priority than adding more
app probes:

1. make the contract explicit in Runtime Runway, Helm, and marquee-apps docs;
2. identify the minimal `AppExecutionPacket` for Catalyst;
3. decide which pieces of Helm `application-server` are host concerns versus
   Helm module concerns;
4. implement the first Runtime Runway-hosted Catalyst path before adding another
   app-owned backend pattern.

Current slice:

- `runway-app-host` defines `AppExecutionPacket` and route ownership metadata.
- Catalyst uses the Runtime Runway host for telemetry, storage, auth, route prefixing,
  status, health, and middleware.
- Catalyst still marks Helm job/operator routes as planned module mounts until
  Helm exposes them as a mountable router.
