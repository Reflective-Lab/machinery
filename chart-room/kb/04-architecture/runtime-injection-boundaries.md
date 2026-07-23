# Runtime and Injection Boundary Diagrams

Last audited: 2026-06-01.

This page draws the current shape around four recurring participants:

- **Application** means a product workspace under `marquee-apps/*`.
- **Helm / Helms** means the operator and trust-transfer surface in
  `bedrock-platform/helms`.
- **Commerce Rails** means Reflective commercial authority in `commerce-rails`.
- **Runtime Runway** means the operational runtime substrate in
  `runtime-runway`.

The diagrams distinguish code ownership from runtime injection. A dependency may
be present in a process without owning the meaning of the domain event that moves
through it.

## Code Homes

```mermaid
flowchart TB
    subgraph Apps["Applications: marquee-apps/*"]
        AppDomain["App domain routes\nsubject refs, UX state, projections"]
        AppPacket["runway.app.json\nAppExecutionPacket"]
    end

    subgraph Helm["Helm / Helms: bedrock-platform/helms"]
        HelmModules["HelmModule crates\noperator-control, governed-jobs, truth execution"]
        HelmUX["Operator surfaces\napprovals, receipts, visibility"]
    end

    subgraph Commerce["Commerce Rails: commerce-rails"]
        CommerceContracts["Commercial contracts\nSubscription, EntitlementGrant, LedgerEntry"]
        CommerceAdapters["Provider adapters\nStripe starts here"]
    end

    subgraph Runway["Runtime Runway: runtime-runway"]
        Host["runway-app-host\nHTTP container, route prefix, health"]
        RuntimeServices["auth, storage, telemetry,\nsecrets, deployment runtime"]
        Realtime["EventHub / SSE\nruntime-owned transport"]
    end

    AppPacket --> Host
    AppDomain --> Host
    HelmModules --> Host
    HelmUX --> HelmModules
    CommerceContracts --> AppDomain
    CommerceAdapters --> CommerceContracts
    RuntimeServices --> Host
    Realtime --> Host
```

Rules:

- Apps own product-domain meaning and app-specific projections.
- Helm owns operator trust-transfer modules and receipt-oriented views.
- Commerce Rails owns commercial state and commercial consequences.
- Runtime Runway owns the container and operational services. It does not own
  app, Helm, or commercial semantics.

## Runtime Host Composition

Two host shapes are live in code today.

```mermaid
flowchart LR
    subgraph Classic["Classic app host path"]
        Packet1["AppExecutionPacket\nfrom runway.app.json"]
        FromEnv["RunwayAppHost::from_env(packet)"]
        AppRoutes["public_routes + protected_routes"]
        Serve1["host.serve(public, protected)"]
        Axum1["Axum router\n/status + app routes + auth/middleware"]

        Packet1 --> FromEnv
        FromEnv --> Serve1
        AppRoutes --> Serve1
        Serve1 --> Axum1
    end

    subgraph Builder["Mounted-module host path"]
        Packet2["AppExecutionPacket\nfrom runway.app.json"]
        Storage["StorageKit\nlocal or remote"]
        BuilderStart["RunwayAppHost::builder(packet)"]
        Mounts[".mount(HelmModule)\n.mount(AppDomainModule)"]
        Build[".with_storage(storage).build()"]
        Serve2["built_host.serve()"]
        Axum2["Axum router\n/healthz + /sse/stream + approvals + module routers"]

        Packet2 --> BuilderStart
        Storage --> Build
        BuilderStart --> Mounts --> Build --> Serve2 --> Axum2
    end
```

Current app examples:

- Atlas and Quorum use the mounted-module path and wrap their app-domain routes
  as `HelmModule` implementations.
- Catalyst, Tally, Plumb, Fathom, Scout, Triage, Vouch, and Warden still use the
  classic app host path or packet-first declarations.

## Injected Handles

The implemented `HostContext` is intentionally small today:

```text
HostContext {
  packet: Arc<AppExecutionPacket>,
  storage: StorageKit,
  realtime: EventHubHandle,
}
```

```mermaid
flowchart TB
    Host["RunwayAppHost builder\nowns EventHub and storage handle"]
    Ctx["HostContext"]
    Packet["packet: AppExecutionPacket\napp id, route prefix, declared boundaries"]
    Storage["storage: StorageKit\nlocal or remote event/document stores"]
    Realtime["realtime: EventHubHandle\nbounded event broadcast + SSE stream"]

    HelmJob["helm.governed-jobs"]
    HelmOps["helm.operator-control"]
    AppModule["app domain module\nAtlasDomainModule, QuorumDomainModule"]

    Host --> Ctx
    Ctx --> Packet
    Ctx --> Storage
    Ctx --> Realtime

    Ctx -->|init(ctx)| HelmJob
    Ctx -->|init(ctx)| HelmOps
    Ctx -->|init(ctx)| AppModule

    HelmJob -->|publishes typed envelopes| Realtime
    HelmOps -->|publishes typed envelopes| Realtime
    AppModule -->|persists domain state| Storage
```

Important constraint: auth, secrets, and telemetry are Runtime Runway concerns,
but they are not all `HostContext` fields in the current code. Some app-domain
modules still read auth configuration from environment and apply `AuthLayer`
inside their router. That is an implementation transition, not a semantic
license for product code to own runtime identity.

## Consequence Lanes

```mermaid
flowchart TB
    User["Human or agent actor"]
    App["Application\nmarquee-apps/*"]
    Helm["Helm modules\noperator-control, governed-jobs"]
    Commerce["Commerce Rails\ncommercial command envelope"]
    Runway["Runtime Runway\nhost, auth, storage, telemetry, deployment"]
    Converge["Converge\nadmission, promotion, receipts"]

    User -->|domain action| App
    User -->|approval, redirect, operator action| Helm
    User -->|commercial action request| Commerce

    App -->|proposed facts, domain events| Converge
    Helm -->|operator receipts, gates, job events| Converge
    Commerce -->|commercial commands, receipts, entitlements| Converge

    Runway -->|authenticates, routes, stores, observes| App
    Runway -->|mounts and transports| Helm
    Runway -->|webhook ingress, secrets, observability| Commerce

    Converge -->|accepted facts and stop reasons| App
    Converge -->|receipt trail| Helm
    Converge -->|accepted commercial facts when applicable| Commerce
```

Read the arrows by authority:

- Runtime Runway can route, authenticate, persist, and observe. It cannot decide
  what a commercial event means.
- Commerce Rails can accept commercial state. It cannot decide canonical login,
  deployment, or app runtime topology.
- Helm can ask for, show, and record operator trust transfer. It cannot promote
  a fact by itself.
- The Application can own product workflow and presentation. It should not
  smuggle runtime, operator, or commercial authority into local strings.

## Commercial Ingress Split

```mermaid
sequenceDiagram
    participant Provider as Stripe or commerce provider
    participant Runway as Runtime Runway
    participant Rails as Commerce Rails
    participant Helm as Helm
    participant App as Application
    participant Conv as Converge

    Provider->>Runway: Webhook HTTP request
    Runway->>Runway: Secret access, route, telemetry
    Runway->>Rails: Provider event + runtime envelope
    Rails->>Rails: Verify provider semantics, idempotency, replay, policy
    Rails->>Conv: Commercial proposal or receipt-bearing event
    Conv-->>Rails: Accepted fact, refusal, or stop reason
    Rails-->>App: Entitlement or commercial projection
    Rails-->>Helm: Operator-visible receipt or exception
```

The provider transport crosses Runtime Runway. The commercial meaning belongs to
Commerce Rails. App access to the resulting capability should use typed
entitlement or subscription state, not a provider-specific string.

## Application Plus Helm Example

```mermaid
sequenceDiagram
    participant Client as Desktop or web client
    participant Host as Runtime Runway host
    participant App as AppDomainModule
    participant HelmJob as helm.governed-jobs
    participant EventHub as EventHub and SSE
    participant Storage as StorageKit
    participant Conv as Converge

    Client->>Host: Request under route_prefix
    Host->>App: Authenticated app-domain route
    App->>Storage: Read/write app projection
    App->>Conv: Admit proposal or signal
    Conv-->>App: Accepted event or honest refusal
    App-->>Client: Domain response

    Client->>Host: Start governed job
    Host->>HelmJob: Helm module route
    HelmJob->>EventHub: job.started / gate.paused / job.completed
    EventHub-->>Client: SSE event stream
    HelmJob->>Conv: Receipt or governed fact path
```

The app and Helm module can share a process, storage handle, and realtime hub.
That does not merge their authority. The route owner in `runway.app.json` should
still say whether a route is `app-domain` or `helm-module`.

## Boundary Checklist

Before adding a new cross-layer dependency, answer these questions:

1. Is this operational state? Put it behind Runtime Runway.
2. Is this commercial state or commercial consequence? Put it behind Commerce
   Rails.
3. Is this operator trust transfer, approval, redirect, or receipt visibility?
   Put it in Helm.
4. Is this product-specific workflow, projection, or subject model? Keep it in
   the Application.
5. Is this admission, promotion, fact integrity, or governed convergence? Route
   it through Converge.
6. Is a string carrying semantics that should be a closed set, bounded number,
   typed actor, typed source, typed route owner, typed entitlement, or typed
   event? Stop and add the type before wiring the boundary.
