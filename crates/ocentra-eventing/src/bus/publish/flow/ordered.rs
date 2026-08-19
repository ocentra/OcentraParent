use std::sync::Arc;

use crate::bus::dispatch::dispatch_sequential;
use crate::bus::publisher::EventPublisher;
use crate::bus::{EventBus, SubscriberRecord};
use crate::{AggregateKey, ExpectValue};

tokio::task_local! {
    static ACTIVE_ORDERED_AGGREGATE: AggregateKey;
}

pub(super) async fn dispatch(
    bus: &EventBus,
    stored: crate::StoredEventEnvelope,
    subscribers: Vec<SubscriberRecord>,
) -> Vec<crate::bus::reports::handler::HandlerReport> {
    let aggregate_key = stored.aggregate_key.clone();
    if ACTIVE_ORDERED_AGGREGATE
        .try_with(|active| active == &aggregate_key)
        .unwrap_or(false)
    {
        // A handler may publish another event for its current aggregate and
        // await it. Re-acquiring the outer semaphore would deadlock that task;
        // the enclosing ordered dispatch already owns this serialization slot.
        return dispatch_sequential(
            stored,
            subscribers,
            EventPublisher::new(bus.clone()),
            bus.handler_policy.clone(),
            Arc::clone(&bus.clock),
        )
        .await;
    }

    let aggregate_gate = bus.aggregate_gate(&aggregate_key);
    let aggregate_permit = Arc::clone(&aggregate_gate)
        .acquire_owned()
        .await
        .expect_value("aggregate ordering gate remains open");
    let reports = ACTIVE_ORDERED_AGGREGATE
        .scope(
            aggregate_key.clone(),
            dispatch_sequential(
                stored,
                subscribers,
                EventPublisher::new(bus.clone()),
                bus.handler_policy.clone(),
                Arc::clone(&bus.clock),
            ),
        )
        .await;
    drop(aggregate_permit);
    bus.release_idle_aggregate_gate(&aggregate_key, &aggregate_gate);
    reports
}
