use crate::{
    CorrelationId, EventId, EventType, EventingError, StoredEventEnvelope, SubscriberId,
    TargetHandler,
};

use super::DispatchMode;

#[derive(Clone, Debug, PartialEq)]
pub struct DeadLetter {
    pub envelope: StoredEventEnvelope,
    pub subscriber_id: SubscriberId,
    pub target_handler: TargetHandler,
    pub error: EventingError,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandlerOutcome {
    Handled,
    Failed,
    TimedOut,
    Panicked,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HandlerReport {
    pub subscriber_id: SubscriberId,
    pub target_handler: TargetHandler,
    pub outcome: HandlerOutcome,
    pub error: Option<EventingError>,
    pub attempts: usize,
    pub trace: EventTraceFields,
}

impl HandlerReport {
    pub(super) fn new(
        stored: &StoredEventEnvelope,
        subscriber_id: SubscriberId,
        target_handler: TargetHandler,
        outcome: HandlerOutcome,
        error: Option<EventingError>,
        attempts: usize,
    ) -> Self {
        let trace = EventTraceFields {
            event_id: stored.event_id.clone(),
            event_type: stored.contract.event_type.clone(),
            correlation_id: stored.correlation_id.clone(),
            subscriber_id: subscriber_id.clone(),
            target_handler: target_handler.clone(),
            outcome,
            attempts,
        };
        Self {
            subscriber_id,
            target_handler,
            outcome,
            error,
            attempts,
            trace,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventTraceFields {
    pub event_id: EventId,
    pub event_type: EventType,
    pub correlation_id: CorrelationId,
    pub subscriber_id: SubscriberId,
    pub target_handler: TargetHandler,
    pub outcome: HandlerOutcome,
    pub attempts: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublishReport {
    pub event_id: EventId,
    pub event_type: EventType,
    pub dispatch_mode: DispatchMode,
    pub subscriber_count: usize,
    pub handled_count: usize,
    pub dead_letter_count: usize,
    pub handler_reports: Vec<HandlerReport>,
}

impl PublishReport {
    pub fn no_subscribers(&self) -> bool {
        self.subscriber_count == 0
    }
}

pub(super) fn dead_letters_for(
    stored: &StoredEventEnvelope,
    reports: &[HandlerReport],
) -> Vec<DeadLetter> {
    reports
        .iter()
        .filter_map(|report| {
            report.error.clone().map(|error| DeadLetter {
                envelope: stored.clone(),
                subscriber_id: report.subscriber_id.clone(),
                target_handler: report.target_handler.clone(),
                error,
            })
        })
        .collect()
}
