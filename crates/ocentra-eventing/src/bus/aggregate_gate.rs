use std::sync::Arc;

use tokio::sync::Semaphore;

use crate::AggregateKey;
use crate::ExpectValue;

use super::EventBus;

impl EventBus {
    pub(super) fn aggregate_gate(&self, aggregate_key: &AggregateKey) -> Arc<Semaphore> {
        let mut gates = self
            .aggregate_gates
            .lock()
            .expect_value("event aggregate gate map");
        Arc::clone(
            gates
                .entry(aggregate_key.clone())
                .or_insert_with(|| Arc::new(Semaphore::new(1))),
        )
    }

    pub(super) fn release_idle_aggregate_gate(
        &self,
        aggregate_key: &AggregateKey,
        aggregate_gate: &Arc<Semaphore>,
    ) {
        if aggregate_gate.available_permits() == 0 || Arc::strong_count(aggregate_gate) > 2 {
            return;
        }
        let mut gates = self
            .aggregate_gates
            .lock()
            .expect_value("event aggregate gate map");
        if gates
            .get(aggregate_key)
            .is_some_and(|current| Arc::ptr_eq(current, aggregate_gate))
            && aggregate_gate.available_permits() == 1
            && Arc::strong_count(aggregate_gate) <= 2
        {
            gates.remove(aggregate_key);
        }
    }
}
