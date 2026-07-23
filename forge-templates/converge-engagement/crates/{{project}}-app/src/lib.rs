//! Application layer. Wires kernel + suggestors + formations.

use std::sync::Arc;

use {{project}}_kernel::InMemoryStore;
use {{project}}_truths::{TRUTHS, TruthSpec};

pub struct App {
    store: Arc<InMemoryStore>,
}

impl App {
    #[must_use]
    pub fn new() -> Self {
        Self {
            store: Arc::new(InMemoryStore::default()),
        }
    }

    #[must_use]
    pub fn list_truths(&self) -> Vec<TruthSpec> {
        TRUTHS.to_vec()
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
