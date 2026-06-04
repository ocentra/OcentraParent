#![forbid(unsafe_code)]

mod bus;
mod clock;
mod contract_registry;
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
    DeadLetter, DeadLetterEvent, DeadLetterReason, DispatchMode, EventBus, EventBusClearReport,
    EventContext, EventPublisher, EventSubscriber, EventTraceFields, HandlerOutcome, HandlerReport,
    PublishReport, QueueDrainReport, SubscriptionHandle, SubscriptionReport, UnsubscribeReport,
    DEAD_LETTER_RECORDED_EVENT_TYPE,
};
pub use clock::{
    EventClock, EventClockInstant, EventClockSleep, ManualEventClock, SharedEventClock,
    SystemEventClock,
};
pub use contract_registry::{
    EventContractDescriptor, EventContractRegistry, EventContractRegistryDocumentation,
};
pub use envelope::{
    DomainEvent, EventContract, EventCustody, EventEnvelope, EventMetadata, EventSource,
    RuntimeRole, StoredEventEnvelope,
};
pub use error::EventingError;
pub use execution::HandlerExecutionPolicy;
pub use ids::{
    AggregateKey, CorrelationId, EventId, EventNamespace, EventType, IdempotencyKey, JournalHash,
    RecordedAt, RequestId, RuntimeInstanceId, SchemaVersion, SourceComponent, SourceService,
    SubscriberId, TargetHandler,
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
