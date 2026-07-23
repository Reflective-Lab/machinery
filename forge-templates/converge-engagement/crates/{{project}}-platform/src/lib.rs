//! Platform bridge. Build IntentPackets, register Suggestors, run Formations.
//!
//! When `platform-local` is off, this crate compiles to a stub.
//! When on, it wires every layer of the stack.

#[cfg(feature = "platform-local")]
pub fn local_stack_enabled() -> bool {
    true
}

#[cfg(not(feature = "platform-local"))]
pub fn local_stack_enabled() -> bool {
    false
}

// Example IntentPacket builder — uncomment when adopting Organism.
//
// #[cfg(feature = "platform-local")]
// pub fn run_to_organism_intent(run: &SomeRun) -> organism_pack::IntentPacket {
//     organism_pack::IntentPacket::new(
//         format!("Process {}", run.id),
//         run.expires_at,
//     )
//     .with_context(serde_json::json!({ "run_id": run.id }))
//     .with_authority(vec!["converge-promotion-required".into()])
// }
