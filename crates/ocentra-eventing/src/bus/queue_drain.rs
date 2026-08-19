use crate::{EventType, EventingError};

use super::{publisher::RootEventPublisher, DispatchMode, EventBus, QueueDrainReport};

mod runner;

impl RootEventPublisher {
    /// Drains queued work as independent root dispatch under explicit root
    /// publication authority.
    pub async fn drain_queued(
        &self,
        dispatch_mode: DispatchMode,
    ) -> Result<QueueDrainReport, EventingError> {
        self.bus.ensure_active()?;
        runner::drain_queued_matching_unchecked(&self.bus, dispatch_mode, None).await
    }
}

impl EventBus {
    pub(super) async fn drain_queued_unchecked(
        &self,
        dispatch_mode: DispatchMode,
    ) -> Result<QueueDrainReport, EventingError> {
        runner::drain_queued_matching_unchecked(self, dispatch_mode, None).await
    }

    pub(super) async fn drain_queued_for_event_unchecked(
        &self,
        dispatch_mode: DispatchMode,
        event_type: &EventType,
    ) -> Result<QueueDrainReport, EventingError> {
        runner::drain_queued_matching_unchecked(self, dispatch_mode, Some(event_type)).await
    }
}
