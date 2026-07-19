use std::collections::HashSet;

use ocentra_schema::parent_ui_bridge::{
    ParentRouteEventSnapshot, ParentRouteSnapshot, ParentSubscriptionEvent,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ParentRouteSubscriptionDelivery {
    Emitted,
    Suppressed,
}

pub struct ParentRouteSubscriptionDeliveryState {
    last_snapshot: Option<ParentRouteSnapshot>,
    last_event_batch: Option<Vec<ParentRouteEventSnapshot>>,
    delivered_event_ids: HashSet<String>,
}

impl ParentRouteSubscriptionDeliveryState {
    pub fn new(initial_snapshot: ParentRouteSnapshot) -> Self {
        Self {
            last_snapshot: Some(initial_snapshot),
            last_event_batch: None,
            delivered_event_ids: HashSet::new(),
        }
    }

    fn should_emit(&self, event: &ParentSubscriptionEvent) -> bool {
        self.last_snapshot.as_ref() != Some(&event.snapshot)
            || self.has_undelivered_event_batch(event.events.as_deref())
    }

    fn has_undelivered_event_batch(&self, events: Option<&[ParentRouteEventSnapshot]>) -> bool {
        let Some(events) = events else {
            return false;
        };
        let has_new_event_id = events.iter().any(|event| {
            event
                .event_id
                .as_ref()
                .is_some_and(|event_id| !self.delivered_event_ids.contains(event_id.as_str()))
        });
        let has_changed_anonymous_event = events.iter().any(|event| event.event_id.is_none())
            && self.last_event_batch.as_deref() != Some(events);

        has_new_event_id || has_changed_anonymous_event
    }

    fn record_delivery(&mut self, event: &ParentSubscriptionEvent) {
        self.last_snapshot = Some(event.snapshot.clone());
        self.last_event_batch = event.events.clone();
        if let Some(events) = event.events.as_deref() {
            self.delivered_event_ids
                .extend(events.iter().filter_map(|event| {
                    event
                        .event_id
                        .as_ref()
                        .map(|event_id| event_id.as_str().to_string())
                }));
        }
    }
}

pub fn deliver_parent_route_subscription_event<E>(
    state: &mut ParentRouteSubscriptionDeliveryState,
    event: &ParentSubscriptionEvent,
    emit: impl FnOnce(&ParentSubscriptionEvent) -> Result<(), E>,
) -> Result<ParentRouteSubscriptionDelivery, E> {
    if !state.should_emit(event) {
        return Ok(ParentRouteSubscriptionDelivery::Suppressed);
    }

    emit(event)?;
    state.record_delivery(event);
    Ok(ParentRouteSubscriptionDelivery::Emitted)
}
