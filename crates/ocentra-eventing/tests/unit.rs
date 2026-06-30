use ocentra_eventing::bus;
use ocentra_eventing::bus::publisher::EventPublisher;
use ocentra_eventing::bus::reports::{
    dead_letter_recorded_event_type, DeadLetterReason, HandlerOutcome,
};
use ocentra_eventing::bus::subscriber::EventSubscriber;
use ocentra_eventing::bus::{DispatchMode, EventBus, ShutdownMode};
use ocentra_eventing::clock::{EventClock, ManualEventClock};
use ocentra_eventing::envelope::{
    DomainEvent, EventContract, EventMetadata, EventSource, StoredEventEnvelope,
};
use ocentra_eventing::error;
use ocentra_eventing::error::EventingError;
use ocentra_eventing::execution::HandlerExecutionPolicy;
use ocentra_eventing::expect_value::ExpectValue;
use ocentra_eventing::ids::{
    AggregateKey, CorrelationId, EventCustody, EventId, EventType, IdempotencyKey, JournalHash,
    RecordedAt, RequestId, RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent,
    SourceService, SubscriberId, TargetHandler,
};
use ocentra_eventing::journal::policy::{JournalPolicy, JournalSelector};
use ocentra_eventing::journal::{EventJournal, JournalAppend};
use ocentra_eventing::queue;
use ocentra_eventing::queue::policy::{EventQueuePolicy, QueueDisposition};
use ocentra_eventing::registrar::EventRegistrar;
use ocentra_eventing::request;
use ocentra_eventing::request::{
    EventResponseContract, RequestCompletionOutcome, RequestEvent, RequestOptions,
};
use ocentra_eventing::testkit::EventRecorder;

#[path = "unit/clock_manual.rs"]
mod clock_manual;
#[path = "unit/envelope.rs"]
mod envelope;
#[path = "unit/fixtures.rs"]
mod fixtures;
#[path = "unit/handler_policy.rs"]
mod handler_policy;
#[path = "unit/ids.rs"]
mod ids;
#[path = "unit/lifecycle.rs"]
mod lifecycle;
#[path = "unit/lifecycle_clear.rs"]
mod lifecycle_clear;
#[path = "unit/metrics.rs"]
mod metrics;
#[path = "unit/production_shutdown.rs"]
mod production_shutdown;
#[path = "unit/queue.rs"]
mod queue_tests;
#[path = "unit/request_response.rs"]
mod request_response;
#[path = "unit/request_response_support.rs"]
mod request_response_support;
