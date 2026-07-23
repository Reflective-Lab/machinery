//! Pure domain types. No platform dependencies.
//!
//! Anything in this crate must compile without Converge / Organism / Axiom / Ferrox.
//! That guarantee is what lets `-kernel` stay testable in isolation.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormationMode {
    Routine,
    Deliberated,
    Huddle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FormationContextKey {
    Seeds,
    Signals,
    Proposals,
    Constraints,
    Strategies,
    Evaluations,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityId(pub Uuid);

impl EntityId {
    #[must_use]
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self::new()
    }
}
