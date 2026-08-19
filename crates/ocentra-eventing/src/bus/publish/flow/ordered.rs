use std::sync::Arc;

use crate::bus::dispatch::dispatch_sequential;
use crate::bus::dispatch_chain::OrderedDispatchAdmission;
use crate::bus::publisher::EventPublisher;
use crate::bus::{EventBus, SubscriberRecord};

pub(super) async fn dispatch(
    bus: &EventBus,
    stored: crate::StoredEventEnvelope,
    subscribers: Vec<SubscriberRecord>,
    admission: &OrderedDispatchAdmission,
) -> Vec<crate::bus::reports::handler::HandlerReport> {
    let reports = dispatch_sequential(
        stored,
        subscribers,
        EventPublisher::for_dispatch(bus.clone(), admission.chain().clone()),
        bus.handler_policy.clone(),
        Arc::clone(&bus.clock),
    )
    .await;
    reports
}
