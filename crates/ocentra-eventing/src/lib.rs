#![forbid(unsafe_code)]

pub mod bus;
pub mod clock;
pub mod compatibility;
pub mod compatibility_markdown;
pub mod contract_registry;
pub mod delivery;
pub mod envelope;
pub mod error;
pub mod execution;
pub mod expect_value;
pub mod ids;
pub mod journal;
pub mod queue;
pub mod registrar;
pub mod replay;
pub mod request;
pub mod testkit;
pub mod topology;

use bus::publisher::{EventContext, EventPublisher, RootEventPublisher};
use bus::reports::dead_letter::{dead_letter_recorded_event_type, DeadLetter, DeadLetterEvent};
use bus::reports::handler::{
    EventMetricsSnapshot, EventTraceFields, HandlerOutcome, HandlerReport, PublishReport,
    QueueDrainReport,
};
use bus::subscriber::{EventSubscriber, SubscriptionHandle, SubscriptionReport, UnsubscribeReport};
use bus::{DispatchMode, EventBusClearReport, EventBusShutdownReport, ShutdownMode};
use clock::{
    EventClock, EventClockInstant, EventClockSleep, ManualEventClock, SharedEventClock,
    SystemEventClock,
};
use compatibility::{EventCompatibilityEntry, EventCompatibilityMatrix, EventCompatibilityStatus};
use contract_registry::{
    EventContractDescriptor, EventContractRegistry, EventContractRegistryDocumentation,
};
use delivery::decide_event_delivery_route;
use delivery::validation::{
    EventDeliveryBackpressurePolicy, EventDeliveryDecisionError, EventDeliveryDecisionInput,
    EventDeliveryDecisionProof, EventDeliveryDecisionState, EventDeliveryRequiredArtifact,
    EventDeliveryRouteKind, EventDeliverySubscriberFilter,
};
use envelope::{
    DomainEvent, EventContract, EventEnvelope, EventMetadata, EventPriority, EventSource,
    StoredEventEnvelope, StoredEventPayload,
};
use error::EventingError;
use execution::HandlerExecutionPolicy;
use expect_value::ExpectValue;
use ids::{
    AggregateKey, CausationId, CorrelationId, EventCustody, EventId, EventNamespace, EventType,
    IdempotencyKey, JournalHash, RecordedAt, RequestId, RuntimeInstanceId, RuntimeRole,
    SchemaVersion, SourceComponent, SourceService, SubscriberId, TargetHandler,
};
use journal::ndjson::{
    JournalFlushPolicy, JournalHashChain, NdjsonEventJournal, NdjsonJournalEntry,
    NdjsonJournalOptions,
};
use journal::policy::{JournalDispatchPhase, JournalMode, JournalPolicy, JournalSelector};
use journal::{EventJournal, JournalAppend, SharedEventJournal};
use queue::policy::{
    EventQueuePolicy, NoSubscriberQueuePolicy, QueueDisposition, QueueOverflowPolicy, QueueReport,
};
use queue::state::{EventQueue, EventQueueClearReport, QueuedEnvelope};
use registrar::{EventRegistrar, RegistrarDisposeReport};
use replay::{
    ReplayActionReport, ReplayCursor, ReplayFilter, ReplayMode, ReplayReadReport, ReplayRecord,
};
use request::RequestRegistry;
use request::{
    EventResponseContract, RequestCompletionOutcome, RequestCompletionReport, RequestEvent,
    RequestOptions, RequestReport,
};
use testkit::EventRecorder;
use topology::{
    EventTopologyEntry, EventTopologyFamilyVariant, EventTopologyManifest, EventTopologyPublisher,
    EventTopologyStatus, EventTopologySubscriber, EventTopologySubscriberTarget,
};

// Keep the root aliases live for internal modules and tests without turning
// this crate root into a re-export barrel.
const _: () = {
    let _ = core::mem::size_of::<EventBusClearReport>();
    let _ = core::mem::size_of::<EventBusShutdownReport>();
    let _ = core::mem::size_of::<ShutdownMode>();
    let _ = core::mem::size_of::<EventPublisher>();
    let _ = core::mem::size_of::<RootEventPublisher>();
    let _ = core::mem::size_of::<DeadLetter>();
    let _ = core::mem::size_of::<DeadLetterEvent>();
    let _ = core::mem::size_of::<EventMetricsSnapshot>();
    let _ = core::mem::size_of::<EventTraceFields>();
    let _ = core::mem::size_of::<HandlerOutcome>();
    let _ = core::mem::size_of::<HandlerReport>();
    let _ = core::mem::size_of::<QueueDrainReport>();
    let _ = dead_letter_recorded_event_type;
    let _ = core::mem::size_of::<Option<&dyn EventClock>>();
    let _ = core::mem::size_of::<EventClockSleep>();
    let _ = core::mem::size_of::<ManualEventClock>();
    let _ = core::mem::size_of::<EventCompatibilityEntry>();
    let _ = core::mem::size_of::<EventCompatibilityMatrix>();
    let _ = core::mem::size_of::<EventCompatibilityStatus>();
    let _ = core::mem::size_of::<EventContractDescriptor>();
    let _ = core::mem::size_of::<EventContractRegistryDocumentation>();
    let _ = core::mem::size_of::<EventDeliveryBackpressurePolicy>();
    let _ = core::mem::size_of::<EventDeliveryDecisionError>();
    let _ = core::mem::size_of::<EventDeliveryDecisionInput>();
    let _ = core::mem::size_of::<EventDeliveryDecisionProof>();
    let _ = core::mem::size_of::<EventDeliveryDecisionState>();
    let _ = core::mem::size_of::<EventDeliveryRequiredArtifact>();
    let _ = core::mem::size_of::<EventDeliveryRouteKind>();
    let _ = core::mem::size_of::<EventDeliverySubscriberFilter>();
    let _ = decide_event_delivery_route;
    let _ = core::mem::size_of::<EventPriority>();
    let _ = core::mem::size_of::<EventSource>();
    let _ = core::mem::size_of::<StoredEventPayload>();
    let _ = core::mem::size_of::<Option<&dyn EventJournal>>();
    let _ = core::mem::size_of::<JournalAppend>();
    let _ = core::mem::size_of::<JournalFlushPolicy>();
    let _ = core::mem::size_of::<JournalHashChain>();
    let _ = core::mem::size_of::<NdjsonEventJournal>();
    let _ = core::mem::size_of::<NdjsonJournalEntry>();
    let _ = core::mem::size_of::<NdjsonJournalOptions>();
    let _ = core::mem::size_of::<JournalMode>();
    let _ = core::mem::size_of::<JournalSelector>();
    let _ = core::mem::size_of::<NoSubscriberQueuePolicy>();
    let _ = core::mem::size_of::<QueueOverflowPolicy>();
    let _ = core::mem::size_of::<EventQueueClearReport>();
    let _ = core::mem::size_of::<EventRegistrar>();
    let _ = core::mem::size_of::<RegistrarDisposeReport>();
    let _ = core::mem::size_of::<ReplayCursor>();
    let _ = core::mem::size_of::<ReplayFilter>();
    let _ = core::mem::size_of::<ReplayActionReport>();
    let _ = core::mem::size_of::<ReplayReadReport>();
    let _ = core::mem::size_of::<RequestCompletionOutcome>();
    let _ = core::mem::size_of::<EventTopologyEntry>();
    let _ = core::mem::size_of::<EventTopologyFamilyVariant>();
    let _ = core::mem::size_of::<EventTopologyManifest>();
    let _ = core::mem::size_of::<EventTopologyPublisher>();
    let _ = core::mem::size_of::<EventTopologyStatus>();
    let _ = core::mem::size_of::<EventTopologySubscriber>();
    let _ = core::mem::size_of::<EventTopologySubscriberTarget>();
    fn _touch_event_response_contract<T: EventResponseContract>() {}
    let _ = core::mem::size_of::<EventRecorder<DeadLetterEvent>>();
};
