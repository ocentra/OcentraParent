#![forbid(unsafe_code)]

mod bus;
mod envelope;
mod error;
mod execution;
mod ids;
mod queue;
mod registrar;
mod testkit;

pub use bus::{
    DeadLetter, DeadLetterEvent, DeadLetterReason, DispatchMode, EventBus, EventContext,
    EventPublisher, EventSubscriber, EventTraceFields, HandlerOutcome, HandlerReport,
    PublishReport, QueueDrainReport, SubscriptionHandle, SubscriptionReport, UnsubscribeReport,
    DEAD_LETTER_RECORDED_EVENT_TYPE,
};
pub use envelope::{
    DomainEvent, EventContract, EventCustody, EventEnvelope, EventMetadata, EventSource,
    RuntimeRole, StoredEventEnvelope,
};
pub use error::EventingError;
pub use execution::HandlerExecutionPolicy;
pub use ids::{
    AggregateKey, CorrelationId, EventId, EventType, IdempotencyKey, RecordedAt, RuntimeInstanceId,
    SchemaVersion, SourceComponent, SourceService, SubscriberId, TargetHandler,
};
pub use queue::{
    EventQueuePolicy, NoSubscriberQueuePolicy, QueueDisposition, QueueOverflowPolicy, QueueReport,
};
pub use registrar::{EventRegistrar, RegistrarDisposeReport};
pub use testkit::EventRecorder;

#[cfg(test)]
mod tests;
