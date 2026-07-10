use std::sync::Arc;

use runway_storage::StorageKit;

use crate::AppExecutionPacket;
use helm_event_substrate::EventHubHandle;

#[derive(Clone)]
pub struct HostContext {
    pub packet: Arc<AppExecutionPacket>,
    pub storage: StorageKit,
    pub realtime: EventHubHandle,
}
