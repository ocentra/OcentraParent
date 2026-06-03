#![forbid(unsafe_code)]

mod bus;
mod envelope;
mod error;
mod ids;

pub use bus::{
    DeadLetter, DispatchMode, EventBus, EventSubscriber, PublishReport, SubscriptionReport,
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

#[cfg(test)]
mod tests;
