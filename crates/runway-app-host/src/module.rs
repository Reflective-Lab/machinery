// RP-LAYERING (RFL-128): `HelmModule` and `ModuleState` now live in the
// neutral `helm-module-contracts` crate so that helms Foundation crates and
// this Runtime substrate can share the interface without either depending on
// the other.  runway-app-host re-exports both so callers importing from here
// need no source change.
pub use helm_module_contracts::{HelmModule, ModuleState};

/// Placeholder type alias for a Tonic-served service.
///
/// In Phase 1 we don't wire gRPC bring-up; modules return an empty `Vec`.
/// `tonic::transport::server::Routes` is the simplest concrete type that
/// compiles cleanly against tonic 0.11 and can be collected into a `Vec`.
pub type TonicService = tonic::service::Routes;
