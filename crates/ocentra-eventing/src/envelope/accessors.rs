use super::{
    DomainEvent, EventClockInstant, EventContract, EventEnvelope, EventPriority, EventSource,
};
use crate::{AggregateKey, CausationId, EventId, IdempotencyKey, RecordedAt, TargetHandler};

impl<E> EventEnvelope<E>
where
    E: DomainEvent,
{
    pub fn contract(&self) -> &EventContract {
        &self.contract
    }

    pub fn event_id(&self) -> &EventId {
        &self.event_id
    }

    pub fn correlation_id(&self) -> &crate::CorrelationId {
        &self.correlation_id
    }

    pub fn causation_id(&self) -> Option<&CausationId> {
        self.causation_id.as_ref()
    }

    pub fn aggregate_key(&self) -> &AggregateKey {
        &self.aggregate_key
    }

    pub fn idempotency_key(&self) -> &IdempotencyKey {
        &self.idempotency_key
    }

    pub fn source(&self) -> &EventSource {
        &self.source
    }

    pub fn observed_at(&self) -> &RecordedAt {
        &self.observed_at
    }

    pub fn target_handler(&self) -> Option<&TargetHandler> {
        self.target_handler.as_ref()
    }

    pub fn priority(&self) -> EventPriority {
        self.priority
    }

    pub fn deadline(&self) -> Option<EventClockInstant> {
        self.deadline
    }

    pub fn payload(&self) -> &E {
        &self.payload
    }

    pub fn into_payload(self) -> E {
        self.payload
    }
}
