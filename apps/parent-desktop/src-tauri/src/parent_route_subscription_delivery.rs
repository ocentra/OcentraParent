use std::collections::HashSet;

use ocentra_schema::parent_ui_bridge::{
    ParentRouteEventSnapshot, ParentRouteSnapshot, ParentSubscriptionEvent,
};

/// Maximum event identity window mirrored by the portal's bounded event buffer.
pub const PARENT_ROUTE_SUBSCRIPTION_EVENT_ID_WINDOW: usize = 128;

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
        let events = newest_delivery_window(events);
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
        let events = event.events.as_deref().map(newest_delivery_window);
        self.last_event_batch = events.map(<[ParentRouteEventSnapshot]>::to_vec);
        self.delivered_event_ids = events
            .unwrap_or_default()
            .iter()
            .filter_map(|event| {
                event
                    .event_id
                    .as_ref()
                    .map(|event_id| event_id.as_str().to_string())
            })
            .collect();
    }

    /// Returns the bounded identity count retained after the last delivery.
    pub fn tracked_event_id_count(&self) -> usize {
        self.delivered_event_ids.len()
    }
}

fn newest_delivery_window(events: &[ParentRouteEventSnapshot]) -> &[ParentRouteEventSnapshot] {
    let start = events
        .len()
        .saturating_sub(PARENT_ROUTE_SUBSCRIPTION_EVENT_ID_WINDOW);
    &events[start..]
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
