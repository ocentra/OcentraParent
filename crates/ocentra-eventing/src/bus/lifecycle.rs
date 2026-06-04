use super::{EventBus, EventBusClearReport};

impl EventBus {
    pub async fn clear_for_test(&self) -> EventBusClearReport {
        let subscription_count = {
            let mut registry = self.registry.lock().expect("event registry lock");
            let subscription_count = registry.values().map(Vec::len).sum();
            registry.clear();
            subscription_count
        };
        let stored_journal_count = {
            let mut stored_journal = self.stored_journal.write().await;
            let stored_journal_count = stored_journal.len();
            stored_journal.clear();
            stored_journal_count
        };
        let dead_letter_count = {
            let mut dead_letters = self.dead_letters.write().await;
            let dead_letter_count = dead_letters.len();
            dead_letters.clear();
            dead_letter_count
        };
        let aggregate_gate_count = {
            let mut aggregate_gates = self.aggregate_gates.lock().expect("aggregate gate lock");
            let aggregate_gate_count = aggregate_gates.len();
            aggregate_gates.clear();
            aggregate_gate_count
        };
        let queue_report = self.queue.clear_for_test();
        let request_report = self.requests.clear_for_test();
        EventBusClearReport {
            subscription_count,
            stored_journal_count,
            dead_letter_count,
            aggregate_gate_count,
            queued_event_count: queue_report.queued_event_count,
            queued_idempotency_key_count: queue_report.queued_idempotency_key_count,
            in_flight_idempotency_key_count: queue_report.in_flight_idempotency_key_count,
            completed_idempotency_key_count: queue_report.completed_idempotency_key_count,
            pending_request_count: request_report.pending_request_count,
            completed_request_count: request_report.completed_request_count,
            timed_out_request_count: request_report.timed_out_request_count,
        }
    }
}
