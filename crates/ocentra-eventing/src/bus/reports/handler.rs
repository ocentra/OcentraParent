use crate::{
    CorrelationId, EventId, EventType, EventingError, JournalAppend, QueueReport, SubscriberId,
    TargetHandler,
};

use super::DispatchMode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HandlerOutcome {
    Handled,
    Failed,
    TimedOut,
    DeadlineExpired,
    Panicked,
}

/// Consumer work observed during one publication.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EventConsumerOutcome {
    /// No matching consumer was registered when the event was published.
    Unregistered,
    /// Every registered consumer completed its handler work.
    Handled,
    /// At least one registered consumer failed to complete its handler work.
    Failed,
}

impl HandlerOutcome {
    pub(crate) fn dead_letter_reason(self) -> super::dead_letter::DeadLetterReason {
        match self {
            Self::Handled => super::dead_letter::DeadLetterReason::HandlerFailed,
            Self::Failed => super::dead_letter::DeadLetterReason::HandlerFailed,
            Self::TimedOut => super::dead_letter::DeadLetterReason::HandlerTimedOut,
            Self::DeadlineExpired => super::dead_letter::DeadLetterReason::HandlerDeadlineExpired,
            Self::Panicked => super::dead_letter::DeadLetterReason::HandlerPanicked,
        }
    }
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
    pub(crate) fn new(
        stored: &crate::StoredEventEnvelope,
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
    pub queue_report: QueueReport,
    pub subscriber_count: usize,
    pub handled_count: usize,
    pub dead_letter_count: usize,
    pub handler_reports: Vec<HandlerReport>,
    /// Successful durable journal writes performed for this publication.
    pub journal_appends: Vec<JournalAppend>,
}

impl PublishReport {
    pub fn no_subscribers(&self) -> bool {
        self.subscriber_count == 0
    }

    /// Classifies actual consumer execution without treating an empty route as success.
    pub fn consumer_outcome(&self) -> EventConsumerOutcome {
        if self.no_subscribers() {
            return EventConsumerOutcome::Unregistered;
        }
        if self.dead_letter_count == 0
            && self.handled_count == self.subscriber_count
            && self.handler_reports.len() == self.subscriber_count
            && self
                .handler_reports
                .iter()
                .all(|report| report.outcome == HandlerOutcome::Handled)
        {
            return EventConsumerOutcome::Handled;
        }
        EventConsumerOutcome::Failed
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QueueDrainReport {
    pub queued_before: usize,
    pub dispatched_count: usize,
    pub expired_count: usize,
    pub remaining_count: usize,
    pub dispatch_reports: Vec<PublishReport>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventMetricsSnapshot {
    pub subscription_count: usize,
    pub stored_event_count: usize,
    pub dead_letter_count: usize,
    pub queue: super::EventQueueMetrics,
    pub requests: super::EventRequestMetrics,
}
