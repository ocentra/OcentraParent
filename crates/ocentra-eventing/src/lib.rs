#![forbid(unsafe_code)]

mod bus;
mod clock;
mod compatibility;
mod compatibility_markdown;
mod contract_registry;
mod delivery;
mod envelope;
mod error;
mod execution;
mod ids;
mod journal;
mod queue;
mod registrar;
mod replay;
mod request;
mod testkit;
mod topology;

pub use bus::{
    dead_letter_recorded_event_type, DeadLetter, DeadLetterEvent, DeadLetterReason, DispatchMode,
    EventBus, EventBusClearReport, EventBusShutdownReport, EventContext, EventMetricsSnapshot,
    EventPublisher, EventQueueMetrics, EventRequestMetrics, EventSubscriber, EventTraceFields,
    HandlerOutcome, HandlerReport, PublishReport, QueueDrainReport, ShutdownMode,
    SubscriptionHandle, SubscriptionReport, UnsubscribeReport,
};
pub use clock::{
    EventClock, EventClockInstant, EventClockSleep, ManualEventClock, SharedEventClock,
    SystemEventClock,
};
pub use compatibility::{
    EventCompatibilityEntry, EventCompatibilityMatrix, EventCompatibilityStatus,
};
pub use contract_registry::{
    EventContractDescriptor, EventContractRegistry, EventContractRegistryDocumentation,
};
pub use delivery::{
    decide_event_delivery_route, EventDeliveryBackpressurePolicy, EventDeliveryDecisionError,
    EventDeliveryDecisionInput, EventDeliveryDecisionProof, EventDeliveryDecisionState,
    EventDeliveryRequiredArtifact, EventDeliveryRouteKind, EventDeliverySubscriberFilter,
};
pub use envelope::{
    DomainEvent, EventContract, EventEnvelope, EventMetadata, EventPriority, EventSource,
    StoredEventEnvelope, StoredEventPayload,
};
pub use error::EventingError;
pub use execution::HandlerExecutionPolicy;
pub use ids::{
    AggregateKey, CausationId, CorrelationId, EventCustody, EventId, EventNamespace, EventType,
    IdempotencyKey, JournalHash, RecordedAt, RequestId, RuntimeInstanceId, RuntimeRole,
    SchemaVersion, SourceComponent, SourceService, SubscriberId, TargetHandler,
};
pub use journal::{
    EventJournal, JournalAppend, JournalDispatchPhase, JournalFlushPolicy, JournalHashChain,
    JournalMode, JournalPolicy, JournalSelector, NdjsonEventJournal, NdjsonJournalEntry,
    NdjsonJournalOptions, SharedEventJournal,
};
pub use queue::{
    EventQueuePolicy, NoSubscriberQueuePolicy, QueueDisposition, QueueOverflowPolicy, QueueReport,
};
pub use registrar::{EventRegistrar, RegistrarDisposeReport};
pub use replay::{ReplayCursor, ReplayFilter, ReplayMode, ReplayReadReport, ReplayRecord};
pub(crate) use request::RequestRegistry;
pub use request::{
    EventResponseContract, RequestCompletionOutcome, RequestCompletionReport, RequestEvent,
    RequestOptions, RequestReport,
};
pub use testkit::EventRecorder;
pub use topology::{
    EventTopologyEntry, EventTopologyFamilyVariant, EventTopologyManifest, EventTopologyPublisher,
    EventTopologyStatus, EventTopologySubscriber, EventTopologySubscriberTarget,
};

#[cfg(test)]
mod tests;
