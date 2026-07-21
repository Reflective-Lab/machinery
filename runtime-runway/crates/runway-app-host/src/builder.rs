use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use anyhow::Result;
use axum::Router;
use axum::handler::Handler;
use axum::http::Method;
use axum::routing::{MethodRouter, delete, get, patch, post, put};
use tokio::net::TcpListener;
use tower_http::services::{ServeDir, ServeFile};

use crate::approvals::{self, ApprovalsState};
use crate::config::HostConfig;
use crate::health;
use crate::module::{HelmModule, ModuleState};
use crate::sse;
use crate::{AppExecutionPacket, MountKind, RouteOwner, RunwayAppHost};
use helm_event_substrate::EventHub;

use runway_middleware;
use runway_storage::StorageKit;

/// A route registered with the host as data, so the D1 verifier can reconcile
/// the served surface against the manifest without introspecting the opaque
/// `axum::Router`. `handler_id` is the handler's `std::any::type_name`, used by
/// check 3 to reject `_test` handlers reaching production.
struct RegisteredRoute {
    method: Method,
    path: String,
    owner: RouteOwner,
    handler_id: String,
}

/// Static single-page-app serving config. The host nests a `ServeDir` at
/// `route_prefix` and falls back to `fallback_index` for unknown paths so
/// client-side deep links resolve. Replaces every app's hand-rolled
/// `tower_http::services::{ServeDir, ServeFile}` block.
#[derive(Debug, Clone)]
pub struct SpaConfig {
    /// Directory of built static assets (e.g. the SPA `dist/`).
    pub dist_dir: PathBuf,
    /// Route prefix the SPA is served under (e.g. `/quorum-sense`).
    pub route_prefix: String,
    /// File served for any path not found under `dist_dir` (the SPA shell).
    pub fallback_index: PathBuf,
}

impl SpaConfig {
    /// Build from the single env contract `RUNWAY_SPA_DIST` (the dist dir).
    /// Returns `None` when the env var is unset, so callers can make the SPA
    /// optional: `if let Some(spa) = SpaConfig::from_env("/app") { b.with_spa(spa) }`.
    /// `fallback_index` defaults to `<dist>/index.html`.
    pub fn from_env(route_prefix: impl Into<String>) -> Option<Self> {
        let dist_dir = PathBuf::from(std::env::var("RUNWAY_SPA_DIST").ok()?);
        let fallback_index = dist_dir.join("index.html");
        Some(Self {
            dist_dir,
            route_prefix: route_prefix.into(),
            fallback_index,
        })
    }
}

type DomainRouterAssembler<S> = Box<dyn FnOnce(HashMap<String, MethodRouter<S>>) -> Router + Send>;

/// Extension seam for the background jobs runtime (RP-LAYERING). The host
/// declares only this interface; the internal `runway-ambient` substrate
/// implements it on its side of the layering boundary, so this publishable
/// crate never depends on the unpublishable substrate.
pub trait JobsRuntime: Send {
    /// Mount the jobs surface. Called once from `build()` when the packet
    /// declares jobs.
    ///
    /// `registered_job_keys` are the packet-declared keys — the full set the
    /// enqueue surface will accept. Implementations must fail (`Err`) rather
    /// than mount a runtime that cannot serve that set: an accepted-but-never-
    /// claimed job stalls silently, which is strictly worse than refusing to
    /// start.
    fn mount(
        self: Box<Self>,
        storage: &StorageKit,
        app_id: String,
        registered_job_keys: Vec<crate::JobKey>,
    ) -> anyhow::Result<MountedJobs>;
}

/// What a [`JobsRuntime`] contributes to the host: the routes to merge and an
/// optional background worker the host keeps alive for the process lifetime.
pub struct MountedJobs {
    pub router: Router,
    pub worker: Option<tokio::task::JoinHandle<()>>,
}

pub struct RunwayAppHostBuilder<S = ()> {
    packet: Arc<AppExecutionPacket>,
    storage: Option<StorageKit>,
    modules: Vec<Arc<dyn HelmModule>>,
    config: Option<HostConfig>,
    spa: Option<SpaConfig>,
    middleware_cfg: Option<runway_middleware::MiddlewareConfig>,
    routes: Vec<RegisteredRoute>,
    route_methods: Vec<(String, MethodRouter<S>)>,
    domain_router_assembler: Option<DomainRouterAssembler<S>>,
    jobs_runtime: Option<Box<dyn JobsRuntime>>,
    _state: PhantomData<fn(&S)>,
}

impl RunwayAppHostBuilder<()> {
    pub fn new(packet: AppExecutionPacket) -> Self {
        Self {
            packet: Arc::new(packet),
            storage: None,
            modules: Vec::new(),
            config: None,
            spa: None,
            middleware_cfg: None,
            routes: Vec::new(),
            route_methods: Vec::new(),
            domain_router_assembler: None,
            jobs_runtime: None,
            _state: PhantomData,
        }
    }

    /// Switch to a stateful app-domain router surface (`State<S>` handlers).
    /// Call before any `route_*` registrations that use `State<S>`.
    pub fn with_domain_state<DS: Clone + Send + Sync + 'static>(self) -> RunwayAppHostBuilder<DS> {
        RunwayAppHostBuilder {
            packet: self.packet,
            storage: self.storage,
            modules: self.modules,
            config: self.config,
            spa: self.spa,
            middleware_cfg: self.middleware_cfg,
            routes: self.routes,
            route_methods: Vec::new(),
            domain_router_assembler: None,
            jobs_runtime: self.jobs_runtime,
            _state: PhantomData,
        }
    }
}

impl<S: Clone + Send + Sync + 'static> RunwayAppHostBuilder<S> {
    /// Register the jobs runtime mounted at `build()` when the packet
    /// declares jobs (e.g. `runway_ambient::AmbientJobs`, which mounts
    /// `POST/GET /v1/ambient/jobs` and spawns a background worker when
    /// `RUNWAY_AMBIENT_ENABLED=true`).
    pub fn with_jobs_runtime(mut self, runtime: Box<dyn JobsRuntime>) -> Self {
        self.jobs_runtime = Some(runtime);
        self
    }

    /// Apply the `runway-middleware` stack (CORS, tracing, compression, error
    /// formatting) to the fully-assembled router at serve time.
    ///
    /// The host's own `/health` route is already present before layers are
    /// applied, so this calls `runway_middleware::apply_layers()` rather than
    /// `stack()` to avoid a duplicate-route panic.
    pub fn with_middleware(mut self, cfg: runway_middleware::MiddlewareConfig) -> Self {
        self.middleware_cfg = Some(cfg);
        self
    }

    pub fn with_storage(mut self, storage: StorageKit) -> Self {
        self.storage = Some(storage);
        self
    }

    pub fn with_config(mut self, config: HostConfig) -> Self {
        self.config = Some(config);
        self
    }

    pub fn mount(mut self, module: Arc<dyn HelmModule>) -> Self {
        self.modules.push(module);
        self
    }

    /// Serve a static SPA bundle. Nested at the outermost router level so the
    /// app's API/domain routes (under `route_prefix`) take precedence; the SPA
    /// only handles paths that don't match a registered route.
    pub fn with_spa(mut self, config: SpaConfig) -> Self {
        self.spa = Some(config);
        self
    }

    /// Assemble registered app-domain routes into a layered `Router` (state,
    /// auth, D5 ownership groups, etc.). When set, the host passes the
    /// path→`MethodRouter` map built from `route_*` registrations instead of
    /// mounting routes flat. D1 verification still uses the `route_*` registry.
    pub fn assemble_domain_router<F>(mut self, f: F) -> Self
    where
        F: FnOnce(HashMap<String, MethodRouter<S>>) -> Router + Send + 'static,
    {
        self.domain_router_assembler = Some(Box::new(f));
        self
    }

    /// Register an app-domain route as data. Unlike mounting an opaque
    /// `axum::Router`, this records `(method, path, owner, handler_id)` so the
    /// D1 verifier can reconcile the served surface against the manifest's
    /// `domain_routes` (check 1) and reject `_test` handlers (check 3). The
    /// router is built from these registrations, so the manifest and the live
    /// surface cannot silently diverge.
    ///
    /// `owner` is almost always `RouteOwner::AppDomain`; the verifier only
    /// reconciles `AppDomain` registrations against the manifest.
    pub fn route_get<H, T>(self, path: impl Into<String>, owner: RouteOwner, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let id = std::any::type_name::<H>();
        self.register(Method::GET, path.into(), owner, id, get(handler))
    }

    pub fn route_post<H, T>(self, path: impl Into<String>, owner: RouteOwner, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let id = std::any::type_name::<H>();
        self.register(Method::POST, path.into(), owner, id, post(handler))
    }

    pub fn route_put<H, T>(self, path: impl Into<String>, owner: RouteOwner, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let id = std::any::type_name::<H>();
        self.register(Method::PUT, path.into(), owner, id, put(handler))
    }

    pub fn route_patch<H, T>(self, path: impl Into<String>, owner: RouteOwner, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let id = std::any::type_name::<H>();
        self.register(Method::PATCH, path.into(), owner, id, patch(handler))
    }

    pub fn route_delete<H, T>(self, path: impl Into<String>, owner: RouteOwner, handler: H) -> Self
    where
        H: Handler<T, S>,
        T: 'static,
    {
        let id = std::any::type_name::<H>();
        self.register(Method::DELETE, path.into(), owner, id, delete(handler))
    }

    fn register(
        mut self,
        method: Method,
        path: String,
        owner: RouteOwner,
        handler_id: &str,
        method_router: MethodRouter<S>,
    ) -> Self {
        self.routes.push(RegisteredRoute {
            method,
            path: path.clone(),
            owner,
            handler_id: handler_id.to_string(),
        });
        self.route_methods.push((path, method_router));
        self
    }

    pub async fn build(self) -> Result<BuiltHost> {
        let storage = self
            .storage
            .ok_or_else(|| anyhow::anyhow!("with_storage(...) must be called before build()"))?;
        let config = match self.config {
            Some(c) => c,
            None => HostConfig::from_env(&self.packet),
        };

        let org_id = std::env::var("RUNWAY_ORG_ID").unwrap_or_else(|_| self.packet.app_id.clone());
        let hub =
            EventHub::with_event_log(storage.events.clone(), org_id, self.packet.app_id.clone())
                .await;

        for module in &self.modules {
            module
                .init()
                .await
                .map_err(|e| anyhow::anyhow!("module '{}' init failed: {e}", module.module_id()))?;
        }

        let mut router = Router::new()
            .merge(health::router())
            .merge(sse::router(hub.handle()))
            .merge(approvals::router(ApprovalsState {
                packet: self.packet.clone(),
                realtime: hub.handle(),
            }));

        for module in &self.modules {
            router = router.merge(module.clone().router());
        }

        // Build the app-domain router from the data registrations. Method
        // routers for the same path are merged so `GET /x` and `POST /x` coexist
        // (axum panics on a duplicate `route(path, ...)`).
        let mut by_path: HashMap<String, MethodRouter<S>> = HashMap::new();
        for (path, method_router) in self.route_methods {
            match by_path.remove(&path) {
                Some(existing) => {
                    by_path.insert(path, existing.merge(method_router));
                }
                None => {
                    by_path.insert(path, method_router);
                }
            }
        }
        if let Some(assemble) = self.domain_router_assembler {
            router = router.merge(assemble(by_path));
        } else {
            anyhow::ensure!(
                by_path.is_empty(),
                "app-domain routes were registered via route_*; call \
                 assemble_domain_router(...) before build()"
            );
        }

        let job_keys: Vec<crate::JobKey> = self.packet.jobs.iter().map(|j| j.key.clone()).collect();
        let jobs_worker = if job_keys.is_empty() {
            None
        } else {
            let runtime = self.jobs_runtime.ok_or_else(|| {
                anyhow::anyhow!(
                    "packet declares jobs but no jobs runtime is registered; call \
                     with_jobs_runtime(...) before build()"
                )
            })?;
            let mounted = runtime.mount(&storage, self.packet.app_id.clone(), job_keys)?;
            router = router.merge(mounted.router);
            mounted.worker
        };

        let prefix = self.packet.route_prefix.trim_end_matches('/');
        let mut router = if prefix.is_empty() {
            router
        } else {
            Router::new().nest(prefix, router)
        };

        if let Some(spa) = self.spa {
            // `fallback` (not `not_found_service`) so unknown deep links return
            // the SPA shell with 200 — client-side routing then resolves them.
            // `not_found_service` would preserve the 404 status (it's for custom
            // error pages), which breaks history-API deep links.
            let spa_service =
                ServeDir::new(&spa.dist_dir).fallback(ServeFile::new(&spa.fallback_index));
            let spa_prefix = spa.route_prefix.trim_end_matches('/');
            router = if spa_prefix.is_empty() {
                router.fallback_service(spa_service)
            } else {
                router.nest_service(spa_prefix, spa_service)
            };
        }

        Ok(BuiltHost {
            packet: self.packet,
            router,
            config,
            middleware_cfg: self.middleware_cfg,
            modules: self.modules,
            routes: self.routes,
            _hub: hub,
            _jobs_worker: jobs_worker,
        })
    }
}

pub struct BuiltHost {
    packet: Arc<AppExecutionPacket>,
    router: Router,
    config: HostConfig,
    middleware_cfg: Option<runway_middleware::MiddlewareConfig>,
    modules: Vec<Arc<dyn HelmModule>>,
    routes: Vec<RegisteredRoute>,
    _hub: EventHub,
    _jobs_worker: Option<tokio::task::JoinHandle<()>>,
}

/// Tokens that mark a handler as test-only. A registered `handler_id`
/// (`std::any::type_name`) containing any of these must never reach production.
const TEST_HANDLER_TOKENS: [&str; 3] = ["for_test", "_test", "test_only"];

/// Env var naming the Commerce-Rails deploy-recipe directory. Matches
/// `ops/templates/materialize-deploy-contracts.sh` (`CR_RECIPES_DIR`).
const CR_RECIPES_DIR_ENV: &str = "CR_RECIPES_DIR";

/// Recipe filename for a deploy contract, per QF-CR-04 / D4 materialization.
fn deploy_contract_recipe_path(recipes_dir: &Path, key: &str, version: &str) -> PathBuf {
    recipes_dir.join(format!("{key}@{version}.yaml"))
}

fn verify_deploy_contracts(packet: &AppExecutionPacket, recipes_dir: Option<&Path>) -> Result<()> {
    for contract in &packet.deploy_contracts {
        if contract.key.is_empty() {
            anyhow::bail!(
                "deploy_contracts entry has an empty `key` — each contract must \
                 declare a non-empty recipe key"
            );
        }
        if contract.version.is_empty() {
            anyhow::bail!(
                "deploy_contracts entry `{}` has an empty `version` — each contract \
                 must declare a non-empty recipe version",
                contract.key
            );
        }
    }
    if let Some(recipes_path) = recipes_dir {
        for contract in &packet.deploy_contracts {
            let recipe =
                deploy_contract_recipe_path(recipes_path, &contract.key, &contract.version);
            if !recipe.is_file() {
                anyhow::bail!(
                    "deploy_contract `{}@{}` has no matching recipe at `{}` — set \
                     `CR_RECIPES_DIR` to the Commerce-Rails deploy-recipe directory \
                     and ensure the recipe is published",
                    contract.key,
                    contract.version,
                    recipe.display()
                );
            }
        }
    }
    Ok(())
}

/// D1 strict-always manifest verification (Phases 1 + 2 + D6 deploy contracts).
///
/// Runs inside `BuiltHost::serve()` before binding and rejects manifest/router
/// divergence in every environment (no flag, per `CLAUDE.md`):
///
/// - **Check 1 (route ↔ manifest):** the set of `domain_routes` the manifest
///   declares with `owner: AppDomain` must equal the set of `AppDomain` routes
///   registered via `route_*`. A declared-but-unregistered route, or a
///   registered-but-undeclared route, fails.
/// - **Check 2 (module state):** every module the manifest marks `Mounted` must
///   be mounted and report `ModuleState::Live` (the planned-vs-mounted lie D2
///   surfaced).
/// - **Check 3 (`_test` handlers):** no registered route resolves to a
///   `handler_id` containing a test-only token (HELMS F4's
///   `apply_proposals_for_test` HTTP exposure).
/// - **Check 4 (deploy contracts):** every `deploy_contracts` entry has a
///   non-empty `key` and `version`. When `CR_RECIPES_DIR` is set, each
///   `{key, version}` must resolve to `<key>@<version>.yaml` in that directory
///   (same convention as `materialize-deploy-contracts.sh`).
///
/// Module routes are governed by check 2 + the manifest's `mounted_modules`;
/// check 1/3 cover `AppDomain` routes, which is what the original D1 scope
/// specifies. See `docs/superpowers/specs/2026-06-15-d1-manifest-verifier-redesign.md`.
fn verify_manifest(
    packet: &AppExecutionPacket,
    modules: &[Arc<dyn HelmModule>],
    routes: &[RegisteredRoute],
) -> Result<()> {
    // --- Check 1: AppDomain routes ↔ manifest.domain_routes ---
    let manifest_domain: HashSet<(String, String)> = packet
        .domain_routes
        .iter()
        .filter(|r| r.owner == RouteOwner::AppDomain)
        .map(|r| (r.method.to_ascii_uppercase(), r.path.clone()))
        .collect();
    let registered_domain: HashSet<(String, String)> = routes
        .iter()
        .filter(|r| r.owner == RouteOwner::AppDomain)
        .map(|r| (r.method.as_str().to_string(), r.path.clone()))
        .collect();

    for (method, path) in &manifest_domain {
        if !registered_domain.contains(&(method.clone(), path.clone())) {
            anyhow::bail!(
                "manifest declares AppDomain route `{method} {path}` but no matching route is \
                 registered (use `RunwayAppHostBuilder::route_*`). Manifest claims a surface the \
                 host does not serve."
            );
        }
    }
    for (method, path) in &registered_domain {
        if !manifest_domain.contains(&(method.clone(), path.clone())) {
            anyhow::bail!(
                "registered AppDomain route `{method} {path}` is not declared in the manifest's \
                 domain_routes. Every served domain route must be declared."
            );
        }
    }

    // --- Check 3: no test-only handlers on registered routes ---
    for r in routes {
        let id = r.handler_id.to_ascii_lowercase();
        if TEST_HANDLER_TOKENS.iter().any(|tok| id.contains(tok)) {
            anyhow::bail!(
                "route `{} {}` resolves to a test-only handler `{}` — `_test`/`for_test`/`test_only` \
                 handlers must not be served (HELMS F4).",
                r.method.as_str(),
                r.path,
                r.handler_id
            );
        }
    }

    // --- Check 2: mounted modules must be live ---
    let live: HashMap<&str, ModuleState> = modules
        .iter()
        .map(|m| (m.module_id(), m.module_state()))
        .collect();

    for declared in &packet.mounted_modules {
        if declared.mount_kind != MountKind::Mounted {
            continue;
        }
        match live.get(declared.module_id.as_str()) {
            None => anyhow::bail!(
                "manifest marks module '{}' as mounted, but no module with that id is \
                 mounted in the host (manifest claims a route surface that does not exist)",
                declared.module_id
            ),
            Some(ModuleState::Shell) => anyhow::bail!(
                "module '{}' is declared mount_kind=mounted in the manifest but reports \
                 module_state=Shell — the planned-vs-mounted lie (D2). Either wire the module \
                 to live state or set mount_kind=planned.",
                declared.module_id
            ),
            Some(ModuleState::Live) => {}
        }
    }

    let recipes_dir = std::env::var(CR_RECIPES_DIR_ENV).ok().map(PathBuf::from);
    verify_deploy_contracts(packet, recipes_dir.as_deref())?;

    Ok(())
}

impl BuiltHost {
    /// Apply an arbitrary transformation to the assembled router before the
    /// server is bound.  Use this to nest additional services (e.g. a static
    /// SPA directory) that cannot be expressed as `HelmModule` implementations.
    ///
    /// The closure receives the fully-assembled router (with route prefix and all
    /// mounted modules already applied) and must return a new `Router`.  Nesting
    /// services here is safe: the domain/API routes registered by modules take
    /// precedence over any fallback / nested service added by the closure.
    pub fn modify_router(mut self, f: impl FnOnce(Router) -> Router) -> Self {
        self.router = f(self.router);
        self
    }

    /// Consume the host and return the assembled Axum router (e.g. for tests or
    /// extra middleware before `axum::serve`).
    pub fn into_router(self) -> Router {
        self.router
    }

    pub async fn serve(self) -> Result<()> {
        verify_manifest(&self.packet, &self.modules, &self.routes)?;

        let addr = format!("0.0.0.0:{}", self.config.port);
        let listener = TcpListener::bind(&addr).await?;
        tracing::info!("runway-app-host listening on http://{addr}");
        let router = if let Some(ref cfg) = self.middleware_cfg {
            runway_middleware::apply_layers(self.router, cfg)
        } else {
            self.router
        };
        axum::serve(listener, router).await?;
        Ok(())
    }
}

impl RunwayAppHost {
    pub fn builder(packet: AppExecutionPacket) -> RunwayAppHostBuilder {
        RunwayAppHostBuilder::new(packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::module::ModuleState;
    use crate::{MountedModule, RunwayAppHost};
    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode};
    use std::fs;
    use tower::ServiceExt;

    struct TestModule {
        id: &'static str,
        state: ModuleState,
    }

    #[async_trait::async_trait]
    impl HelmModule for TestModule {
        fn module_id(&self) -> &'static str {
            self.id
        }
        fn module_state(&self) -> ModuleState {
            self.state
        }
    }

    fn module(id: &'static str, state: ModuleState) -> Arc<dyn HelmModule> {
        Arc::new(TestModule { id, state })
    }

    fn domain_route(method: Method, path: &str, handler_id: &str) -> RegisteredRoute {
        RegisteredRoute {
            method,
            path: path.to_string(),
            owner: RouteOwner::AppDomain,
            handler_id: handler_id.to_string(),
        }
    }

    async fn list_inquiries() -> &'static str {
        "ok"
    }
    async fn apply_proposals_for_test() -> &'static str {
        "danger"
    }

    #[test]
    fn verify_passes_when_mounted_module_is_live() {
        let packet = AppExecutionPacket::new("a", "A", "d", "/a")
            .with_mounted_module(MountedModule::new("helm.jobs", MountKind::Mounted));
        let mods = vec![module("helm.jobs", ModuleState::Live)];
        assert!(verify_manifest(&packet, &mods, &[]).is_ok());
    }

    #[test]
    fn verify_fails_when_mounted_module_is_shell() {
        let packet = AppExecutionPacket::new("a", "A", "d", "/a")
            .with_mounted_module(MountedModule::new("helm.jobs", MountKind::Mounted));
        // Default module_state() is Shell — the D2 lie.
        let mods = vec![module("helm.jobs", ModuleState::Shell)];
        let err = verify_manifest(&packet, &mods, &[])
            .unwrap_err()
            .to_string();
        assert!(err.contains("helm.jobs"), "{err}");
        assert!(err.contains("Shell"), "{err}");
    }

    #[test]
    fn verify_fails_when_mounted_module_is_absent() {
        let packet = AppExecutionPacket::new("a", "A", "d", "/a")
            .with_mounted_module(MountedModule::new("helm.jobs", MountKind::Mounted));
        let mods: Vec<Arc<dyn HelmModule>> = vec![];
        let err = verify_manifest(&packet, &mods, &[])
            .unwrap_err()
            .to_string();
        assert!(err.contains("helm.jobs"), "{err}");
        assert!(err.contains("not"), "{err}");
    }

    #[test]
    fn verify_ignores_planned_modules() {
        // Planned modules may be shell or absent — only Mounted is enforced.
        let packet = AppExecutionPacket::new("a", "A", "d", "/a")
            .with_mounted_module(MountedModule::new("helm.jobs", MountKind::Planned))
            .with_mounted_module(MountedModule::new("helm.absent", MountKind::Planned));
        let mods = vec![module("helm.jobs", ModuleState::Shell)];
        assert!(verify_manifest(&packet, &mods, &[]).is_ok());
    }

    #[test]
    fn verify_passes_with_no_mounted_modules() {
        let packet = AppExecutionPacket::new("a", "A", "d", "/a");
        let mods: Vec<Arc<dyn HelmModule>> = vec![];
        assert!(verify_manifest(&packet, &mods, &[]).is_ok());
    }

    // --- Phase 2: check 1 (route ↔ manifest) + check 3 (_test handlers) ---

    #[test]
    fn verify_passes_when_domain_routes_match() {
        let packet = AppExecutionPacket::new("a", "A", "d", "/a").with_domain_route(
            crate::RouteRegistration::new("GET", "/inquiries", RouteOwner::AppDomain),
        );
        let routes = vec![domain_route(
            Method::GET,
            "/inquiries",
            "app::list_inquiries",
        )];
        assert!(verify_manifest(&packet, &[], &routes).is_ok());
    }

    #[test]
    fn verify_fails_when_manifest_declares_unregistered_route() {
        let packet = AppExecutionPacket::new("a", "A", "d", "/a").with_domain_route(
            crate::RouteRegistration::new("POST", "/inquiries", RouteOwner::AppDomain),
        );
        let err = verify_manifest(&packet, &[], &[]).unwrap_err().to_string();
        assert!(err.contains("POST /inquiries"), "{err}");
        assert!(err.contains("not"), "{err}");
    }

    #[test]
    fn verify_fails_when_registered_route_is_undeclared() {
        let packet = AppExecutionPacket::new("a", "A", "d", "/a");
        let routes = vec![domain_route(Method::POST, "/inquiries", "app::create")];
        let err = verify_manifest(&packet, &[], &routes)
            .unwrap_err()
            .to_string();
        assert!(err.contains("POST /inquiries"), "{err}");
        assert!(err.contains("not declared"), "{err}");
    }

    #[test]
    fn verify_fails_on_test_only_handler() {
        let packet = AppExecutionPacket::new("a", "A", "d", "/a").with_domain_route(
            crate::RouteRegistration::new("POST", "/proposals", RouteOwner::AppDomain),
        );
        let routes = vec![domain_route(
            Method::POST,
            "/proposals",
            "quorum::apply_proposals_for_test",
        )];
        let err = verify_manifest(&packet, &[], &routes)
            .unwrap_err()
            .to_string();
        assert!(err.contains("test-only"), "{err}");
        assert!(err.contains("apply_proposals_for_test"), "{err}");
    }

    // --- Check 4: deploy_contracts recipe resolution (D6) ---

    #[test]
    fn verify_passes_when_deploy_contract_recipe_exists() {
        let recipes = tempfile::tempdir().unwrap();
        fs::write(
            recipes.path().join("commerce-rails-stripe@0.2.yaml"),
            "recipe_format_version: \"1.0\"\n",
        )
        .unwrap();
        let packet = AppExecutionPacket::new("a", "A", "d", "/a")
            .with_deploy_contract("commerce-rails-stripe", "0.2");
        assert!(verify_deploy_contracts(&packet, Some(recipes.path())).is_ok());
    }

    #[test]
    fn verify_fails_when_deploy_contract_recipe_missing() {
        let recipes = tempfile::tempdir().unwrap();
        let packet = AppExecutionPacket::new("a", "A", "d", "/a")
            .with_deploy_contract("commerce-rails-stripe", "0.2");
        let err = verify_deploy_contracts(&packet, Some(recipes.path()))
            .unwrap_err()
            .to_string();
        assert!(err.contains("commerce-rails-stripe@0.2"), "{err}");
        assert!(err.contains("no matching recipe"), "{err}");
    }

    #[test]
    fn verify_skips_recipe_check_when_cr_recipes_dir_unset() {
        let packet = AppExecutionPacket::new("a", "A", "d", "/a")
            .with_deploy_contract("missing-contract", "9.9");
        assert!(verify_deploy_contracts(&packet, None).is_ok());
    }

    #[test]
    fn verify_fails_on_empty_deploy_contract_key() {
        let packet = AppExecutionPacket::new("a", "A", "d", "/a");
        let mut packet = packet;
        packet.deploy_contracts.push(crate::DeployContract {
            key: String::new(),
            version: "1.0".into(),
        });
        let err = verify_deploy_contracts(&packet, None)
            .unwrap_err()
            .to_string();
        assert!(err.contains("empty `key`"), "{err}");
    }

    #[test]
    fn verify_fails_on_empty_deploy_contract_version() {
        let packet = AppExecutionPacket::new("a", "A", "d", "/a");
        let mut packet = packet;
        packet.deploy_contracts.push(crate::DeployContract {
            key: "stripe".into(),
            version: String::new(),
        });
        let err = verify_deploy_contracts(&packet, None)
            .unwrap_err()
            .to_string();
        assert!(err.contains("empty `version`"), "{err}");
    }

    #[tokio::test]
    async fn serve_rejects_missing_deploy_contract_recipe_when_dir_configured() {
        let recipes = tempfile::tempdir().unwrap();
        let store_dir = tempfile::tempdir().unwrap();
        let storage = StorageKit::local(store_dir.path()).await.unwrap();
        let packet = AppExecutionPacket::new("a", "A", "d", "/a")
            .with_deploy_contract("commerce-rails-stripe", "0.2");
        let built = RunwayAppHost::builder(packet)
            .with_storage(storage)
            .build()
            .await
            .unwrap();
        let recipes_path = recipes.path().to_path_buf();
        let prev = std::env::var(CR_RECIPES_DIR_ENV).ok();
        // SAFETY: single env mutation for one async test; restored before return.
        unsafe { std::env::set_var(CR_RECIPES_DIR_ENV, &recipes_path) };
        let err = built.serve().await.unwrap_err().to_string();
        match prev {
            Some(v) => unsafe { std::env::set_var(CR_RECIPES_DIR_ENV, v) },
            None => unsafe { std::env::remove_var(CR_RECIPES_DIR_ENV) },
        }
        assert!(err.contains("commerce-rails-stripe@0.2"), "{err}");
        assert!(err.contains("no matching recipe"), "{err}");
    }

    fn mount_flat_domain(by_path: HashMap<String, MethodRouter>) -> Router {
        let mut router = Router::new();
        for (path, method_router) in by_path {
            router = router.route(&path, method_router);
        }
        router
    }

    #[tokio::test]
    async fn registered_routes_are_served_and_pass_verification() {
        let store_dir = tempfile::tempdir().unwrap();
        let storage = StorageKit::local(store_dir.path()).await.unwrap();
        let packet = AppExecutionPacket::new("app", "App", "d", "/app").with_domain_route(
            crate::RouteRegistration::new("GET", "/inquiries", RouteOwner::AppDomain),
        );
        let built = RunwayAppHost::builder(packet)
            .with_storage(storage)
            .route_get("/inquiries", RouteOwner::AppDomain, list_inquiries)
            .assemble_domain_router(mount_flat_domain)
            .build()
            .await
            .unwrap();
        let router = built.into_router();
        // route is served under the app prefix
        let resp = router.oneshot(get("/app/inquiries")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(body_string(resp).await, "ok");
    }

    #[tokio::test]
    async fn serve_rejects_test_handler_via_type_name() {
        // End-to-end: the handler's std::any::type_name carries the fn name, so
        // a real `_for_test` handler is caught by check 3 without any explicit
        // handler_id from the caller.
        let store_dir = tempfile::tempdir().unwrap();
        let storage = StorageKit::local(store_dir.path()).await.unwrap();
        let packet = AppExecutionPacket::new("app", "App", "d", "/app").with_domain_route(
            crate::RouteRegistration::new("POST", "/proposals", RouteOwner::AppDomain),
        );
        let built = RunwayAppHost::builder(packet)
            .with_storage(storage)
            .route_post(
                "/proposals",
                RouteOwner::AppDomain,
                apply_proposals_for_test,
            )
            .assemble_domain_router(mount_flat_domain)
            .build()
            .await
            .unwrap();
        let err = built.serve().await.unwrap_err().to_string();
        assert!(err.contains("test-only"), "{err}");
    }

    #[tokio::test]
    async fn serve_rejects_mounted_shell_lie_before_binding() {
        let store_dir = tempfile::tempdir().unwrap();
        let storage = StorageKit::local(store_dir.path()).await.unwrap();
        let packet = AppExecutionPacket::new("a", "A", "d", "/a")
            .with_mounted_module(MountedModule::new("helm.jobs", MountKind::Mounted));
        let built = RunwayAppHost::builder(packet)
            .with_storage(storage)
            .mount(module("helm.jobs", ModuleState::Shell))
            .build()
            .await
            .unwrap();
        // verify runs before bind, so this returns Err without touching a port.
        let err = built.serve().await.unwrap_err().to_string();
        assert!(err.contains("helm.jobs"), "{err}");
    }

    async fn built_with_spa(
        route_prefix: &str,
        spa_prefix: &str,
    ) -> (Router, tempfile::TempDir, tempfile::TempDir) {
        let store_dir = tempfile::tempdir().unwrap();
        let dist = tempfile::tempdir().unwrap();
        fs::write(
            dist.path().join("index.html"),
            "<!doctype html><title>SPA shell</title>",
        )
        .unwrap();
        fs::write(dist.path().join("app.js"), "console.log('hi')").unwrap();

        let storage = StorageKit::local(store_dir.path()).await.unwrap();
        let packet = AppExecutionPacket::new("app", "App", "desc", route_prefix);
        let built = RunwayAppHost::builder(packet)
            .with_storage(storage)
            .with_spa(SpaConfig {
                dist_dir: dist.path().to_path_buf(),
                route_prefix: spa_prefix.to_string(),
                fallback_index: dist.path().join("index.html"),
            })
            .build()
            .await
            .unwrap();
        (built.into_router(), store_dir, dist)
    }

    async fn body_string(resp: axum::response::Response) -> String {
        let bytes = to_bytes(resp.into_body(), 1 << 20).await.unwrap();
        String::from_utf8(bytes.to_vec()).unwrap()
    }

    fn get(uri: &str) -> Request<Body> {
        Request::builder().uri(uri).body(Body::empty()).unwrap()
    }

    #[tokio::test]
    async fn with_spa_serves_assets_and_falls_back_to_index_under_prefix() {
        let (router, _store, _dist) = built_with_spa("/app", "/ui").await;

        let resp = router.clone().oneshot(get("/ui/app.js")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(body_string(resp).await.contains("console.log"));

        // Unknown deep link under the SPA prefix falls back to index.html so
        // client-side routing resolves.
        let resp = router.clone().oneshot(get("/ui/deep/link")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(body_string(resp).await.contains("SPA shell"));

        // App/host routes still resolve — the SPA is nested at a separate prefix.
        let resp = router.oneshot(get("/app/healthz")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(body_string(resp).await, "ok");
    }

    #[tokio::test]
    async fn with_spa_root_prefix_uses_fallback_service_preserving_routes() {
        let (router, _store, _dist) = built_with_spa("/app", "/").await;

        // Registered route wins over the SPA fallback.
        let resp = router.clone().oneshot(get("/app/healthz")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert_eq!(body_string(resp).await, "ok");

        // Everything unmatched falls through to the SPA shell.
        let resp = router.oneshot(get("/totally/unknown")).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
        assert!(body_string(resp).await.contains("SPA shell"));
    }
}
