#![forbid(unsafe_code)]

mod bus;
mod envelope;
mod error;
mod ids;
mod registrar;

pub use bus::{
    DeadLetter, DispatchMode, EventBus, EventContext, EventPublisher, EventSubscriber,
    HandlerOutcome, HandlerReport, PublishReport, SubscriptionHandle, SubscriptionReport,
    UnsubscribeReport,
};
pub use envelope::{
    DomainEvent, EventContract, EventCustody, EventEnvelope, EventMetadata, EventSource,
    RuntimeRole, StoredEventEnvelope,
};
pub use error::EventingError;
pub use ids::{
    AggregateKey, CorrelationId, EventId, EventType, IdempotencyKey, RecordedAt, RuntimeInstanceId,
    SchemaVersion, SourceComponent, SourceService, SubscriberId, TargetHandler,
};
pub use registrar::{EventRegistrar, RegistrarDisposeReport};

#[cfg(test)]
mod tests;
