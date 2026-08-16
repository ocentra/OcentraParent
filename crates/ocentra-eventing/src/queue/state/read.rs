use crate::EventType;

use super::{EventQueue, QueuedEnvelope};
use crate::ExpectValue;

impl EventQueue {
    pub(crate) fn queued_count(&self, event_type: Option<&EventType>) -> usize {
        let state = self.state.lock().expect_value("event queue lock");
        state
            .queued
            .iter()
            .filter(|queued| {
                event_type.is_none_or(|event_type| queued.matches_event_type(event_type))
            })
            .count()
    }

    pub(crate) fn take_next_queued(
        &self,
        event_type: Option<&EventType>,
    ) -> Option<QueuedEnvelope> {
        let mut state = self.state.lock().expect_value("event queue lock");
        let position = state.queued.iter().position(|queued| {
            event_type.is_none_or(|event_type| queued.matches_event_type(event_type))
        })?;
        let queued = state
            .queued
            .remove(position)
            .expect_value("queued position was selected from queue");
        state.queued_event_ids.remove(&queued.stored.event_id);
        if self.policy.idempotency_registry_enabled() {
            state.queued_keys.remove(&queued.stored.idempotency_key);
        }
        Some(queued)
    }

    pub(crate) fn take_all_queued(&self) -> Vec<QueuedEnvelope> {
        let mut state = self.state.lock().expect_value("event queue lock");
        let queued = state.queued.drain(..).collect();
        state.queued_event_ids.clear();
        state.queued_keys.clear();
        queued
    }

    pub(crate) fn requeue(&self, queued: QueuedEnvelope) {
        let mut state = self.state.lock().expect_value("event queue lock");
        state
            .queued_event_ids
            .insert(queued.stored.event_id.clone());
        if self.policy.idempotency_registry_enabled() {
            state
                .queued_keys
                .insert(queued.stored.idempotency_key.clone());
        }
        state.queued.push_back(queued);
    }
}
