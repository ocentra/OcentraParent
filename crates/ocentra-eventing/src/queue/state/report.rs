use crate::queue::policy::QueueDisposition;

use super::EventQueue;
use crate::bus::reports::EventQueueMetrics;
use crate::ExpectValue;

impl EventQueue {
    pub(crate) fn report(
        &self,
        disposition: QueueDisposition,
    ) -> crate::queue::policy::QueueReport {
        let queued_count = self
            .state
            .lock()
            .expect_value("event queue lock")
            .queued
            .len();
        crate::queue::policy::QueueReport {
            disposition,
            queued_count,
            capacity: self.policy.capacity(),
        }
    }

    pub(crate) fn metrics(&self) -> EventQueueMetrics {
        let state = self.state.lock().expect_value("event queue lock");
        EventQueueMetrics {
            queued_event_count: state.queued.len(),
            queued_event_id_count: state.queued_event_ids.len(),
            queued_idempotency_key_count: state.queued_keys.len(),
            in_flight_event_id_count: state.in_flight_event_ids.len(),
            in_flight_idempotency_key_count: state.in_flight_keys.len(),
            completed_idempotency_key_count: state.completed_keys.len(),
            capacity: self.policy.capacity(),
        }
    }
}
