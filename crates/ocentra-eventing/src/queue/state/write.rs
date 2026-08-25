use crate::{EventId, IdempotencyKey};

use super::{EventQueue, EventQueueClearReport, QueuedEnvelope};
use crate::ExpectValue;

impl EventQueue {
    pub(crate) fn rollback_queued(&self, event_id: &EventId) -> bool {
        let mut state = self.state.lock().expect_value("event queue lock");
        let Some(position) = state
            .queued
            .iter()
            .position(|queued| &queued.stored.event_id == event_id)
        else {
            return false;
        };
        let queued = state
            .queued
            .remove(position)
            .expect_value("queued rollback position remains valid");
        state.queued_event_ids.remove(&queued.stored.event_id);
        if self.policy.idempotency_registry_enabled() {
            state.queued_keys.remove(&queued.stored.idempotency_key);
        }
        true
    }

    pub(crate) fn rollback_overflow(&self, event_id: &EventId, dropped: QueuedEnvelope) -> bool {
        let mut state = self.state.lock().expect_value("event queue lock");
        let Some(position) = state
            .queued
            .iter()
            .position(|queued| &queued.stored.event_id == event_id)
        else {
            return false;
        };
        let queued = state
            .queued
            .remove(position)
            .expect_value("overflow rollback position remains valid");
        state.queued_event_ids.remove(&queued.stored.event_id);
        if self.policy.idempotency_registry_enabled() {
            state.queued_keys.remove(&queued.stored.idempotency_key);
        }
        state
            .queued_event_ids
            .insert(dropped.stored.event_id.clone());
        if self.policy.idempotency_registry_enabled() {
            state
                .queued_keys
                .insert(dropped.stored.idempotency_key.clone());
        }
        state.queued.push_front(dropped);
        true
    }

    pub(crate) fn mark_completed(&self, event_id: &EventId, key: IdempotencyKey) {
        let mut state = self.state.lock().expect_value("event queue lock");
        state.in_flight_event_ids.remove(event_id);
        state.in_flight_keys.remove(&key);
        if self.policy.idempotency_registry_enabled() && state.completed_keys.insert(key.clone()) {
            state.completed_key_order.push_back(key);
            super::trim_completed_keys(&mut state);
        }
    }

    pub(crate) fn release_in_flight(&self, event_id: &EventId, key: Option<&IdempotencyKey>) {
        let mut state = self.state.lock().expect_value("event queue lock");
        state.in_flight_event_ids.remove(event_id);
        if let Some(key) = key {
            state.in_flight_keys.remove(key);
        }
    }

    pub(crate) fn finalize_shutdown(&self) -> EventQueueClearReport {
        // Terminal cleanup retains the existing queue and idempotency reset;
        // its name distinguishes production shutdown from test-only clearing.
        let mut state = self.state.lock().expect_value("event queue lock");
        let report = EventQueueClearReport {
            queued_event_count: state.queued.len(),
            queued_idempotency_key_count: state.queued_keys.len(),
            in_flight_idempotency_key_count: state.in_flight_keys.len(),
            completed_idempotency_key_count: state.completed_keys.len(),
        };
        state.queued.clear();
        state.queued_event_ids.clear();
        state.queued_keys.clear();
        state.in_flight_event_ids.clear();
        state.in_flight_keys.clear();
        state.completed_keys.clear();
        state.completed_key_order.clear();
        report
    }
}
