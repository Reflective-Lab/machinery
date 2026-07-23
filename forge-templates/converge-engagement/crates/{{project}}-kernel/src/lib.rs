//! Event-sourced kernel — owns state transitions only.
//!
//! Pattern: `write_with_events` takes a closure that mutates a snapshot,
//! drains pending events on success, and emits them after commit.
//! Projectors and suggestors observe events; they don't mutate state.

use std::sync::RwLock;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Default, Clone)]
pub struct Kernel {
    pending_events: Vec<DomainEvent>,
    // Add domain state here.
}

impl Kernel {
    pub fn drain_events(&mut self) -> Vec<DomainEvent> {
        std::mem::take(&mut self.pending_events)
    }

    fn record(&mut self, event: DomainEvent) {
        self.pending_events.push(event);
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DomainEvent {
    // EntityCreated { id: Uuid, ... },
}

#[derive(Debug, Error)]
pub enum KernelError {
    #[error("invariant violated: {0}")]
    Invariant(String),
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("kernel lock poisoned")]
    LockPoisoned,
    #[error(transparent)]
    Kernel(#[from] KernelError),
}

#[derive(Debug)]
pub struct StoreWriteResult<R> {
    pub value: R,
    pub events: Vec<DomainEvent>,
}

#[derive(Debug, Default)]
pub struct InMemoryStore {
    kernel: RwLock<Kernel>,
}

impl InMemoryStore {
    /// Snapshot-and-drain pattern. The closure mutates a clone; if it succeeds,
    /// the clone replaces the kernel and pending events are emitted.
    pub fn write_with_events<R>(
        &self,
        f: impl FnOnce(&mut Kernel) -> Result<R, KernelError>,
    ) -> Result<StoreWriteResult<R>, StoreError> {
        let mut kernel = self.kernel.write().map_err(|_| StoreError::LockPoisoned)?;
        let mut snapshot = kernel.clone();
        let value = f(&mut snapshot)?;
        let events = snapshot.drain_events();
        *kernel = snapshot;
        Ok(StoreWriteResult { value, events })
    }

    pub fn read<R>(&self, f: impl FnOnce(&Kernel) -> R) -> Result<R, StoreError> {
        let kernel = self.kernel.read().map_err(|_| StoreError::LockPoisoned)?;
        Ok(f(&kernel))
    }
}
