use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::{
    AggregateKey, CorrelationId, DispatchMode, DomainEvent, EventBus, EventContract, EventCustody,
    EventEnvelope, EventMetadata, EventSource, EventSubscriber, EventType, EventingError,
    IdempotencyKey, RecordedAt, RuntimeInstanceId, RuntimeRole, SchemaVersion, SourceComponent,
    SourceService, SubscriberId, TargetHandler,
};

const TEST_EVENT_TYPE: &str = "eventing.test.observed";
const OTHER_EVENT_TYPE: &str = "eventing.test.other";
const TEST_EVENT_ID: &str = "event-test-1";
const TEST_CORRELATION_ID: &str = "correlation-test-1";
const TEST_AGGREGATE: &str = "aggregate-test-1";
const TEST_IDEMPOTENCY: &str = "idempotency-test-1";
const TEST_SOURCE_SERVICE: &str = "eventing-test-service";
const TEST_SOURCE_COMPONENT: &str = "eventing-test-component";
const TEST_INSTANCE: &str = "eventing-test-instance";
const TEST_TARGET: &str = "eventing-test-handler";
const OTHER_TARGET: &str = "eventing-other-handler";
const TEST_SUBSCRIBER: &str = "eventing-test-subscriber";
const OTHER_SUBSCRIBER: &str = "eventing-other-subscriber";
const TEST_OBSERVED_AT: &str = "2026-06-03T22:30:00Z";
const TEST_LABEL: &str = "typed envelope proof";

#[tokio::test]
async fn event_bus_dispatches_typed_envelope_and_stores_serialized_boundary() {
    let bus = EventBus::new();
    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |envelope| {
        let handled = Arc::clone(&handled_clone);
        async move {
            handled.lock().await.push(envelope.payload.label);
            Ok(())
        }
    })
    .await
    .expect("subscriber registers");

    let report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .expect("publish succeeds");
    let journal = bus.journal().await;
    let decoded: EventEnvelope<TestEvent> = journal[0].decode().expect("stored envelope decodes");

    assert_eq!(report.subscriber_count, 1);
    assert_eq!(report.handled_count, 1);
    assert_eq!(handled.lock().await.as_slice(), &[TEST_LABEL.to_string()]);
    assert_eq!(decoded.payload.label, TEST_LABEL);
    assert_eq!(journal.len(), 1);
}

#[tokio::test]
async fn target_handler_filter_prevents_wrong_handler_delivery() {
    let bus = EventBus::new();
    let handled = Arc::new(Mutex::new(0_usize));
    let handled_clone = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |_| {
        let handled = Arc::clone(&handled_clone);
        async move {
            *handled.lock().await += 1;
            Ok(())
        }
    })
    .await
    .expect("subscriber registers");
    bus.subscribe::<TestEvent, _, _>(subscriber(OTHER_SUBSCRIBER, OTHER_TARGET), |_| async {
        Ok(())
    })
    .await
    .expect("second subscriber registers");

    let report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .expect("publish succeeds");

    assert_eq!(report.subscriber_count, 1);
    assert_eq!(*handled.lock().await, 1);
}

#[tokio::test]
async fn concurrent_dispatch_records_handler_dead_letter_without_losing_journal() {
    let bus = EventBus::new();
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), |_| async {
        Err(EventingError::empty_value("handler_failure"))
    })
    .await
    .expect("subscriber registers");

    let report = bus
        .publish_with_mode(
            test_event(TEST_LABEL),
            metadata(TEST_TARGET),
            DispatchMode::Concurrent,
        )
        .await
        .expect("publish succeeds");
    let dead_letters = bus.dead_letters().await;

    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(report.handled_count, 0);
    assert_eq!(bus.journal().await.len(), 1);
    assert_eq!(dead_letters[0].target_handler.as_str(), TEST_TARGET);
}

#[tokio::test]
async fn duplicate_subscriber_ids_are_rejected() {
    let bus = EventBus::new();
    let duplicate = subscriber(TEST_SUBSCRIBER, TEST_TARGET);
    bus.subscribe::<TestEvent, _, _>(duplicate.clone(), |_| async { Ok(()) })
        .await
        .expect("first subscriber registers");

    let result = bus
        .subscribe::<TestEvent, _, _>(duplicate, |_| async { Ok(()) })
        .await;

    assert!(matches!(
        result,
        Err(EventingError::DuplicateSubscriber { .. })
    ));
}

#[test]
fn eventing_newtypes_reject_empty_values_and_zero_versions() {
    assert!(EventType::parse("").is_err());
    assert!(RecordedAt::parse(" ").is_err());
    assert!(SchemaVersion::new(0).is_err());
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct TestEvent {
    label: String,
    aggregate_key: AggregateKey,
    idempotency_key: IdempotencyKey,
    event_type: EventType,
}

impl DomainEvent for TestEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            self.event_type.clone(),
            SchemaVersion::new(1)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        Ok(self.aggregate_key.clone())
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        Ok(self.idempotency_key.clone())
    }
}

fn test_event(label: &str) -> TestEvent {
    TestEvent {
        label: label.to_string(),
        aggregate_key: AggregateKey::parse(TEST_AGGREGATE).expect("aggregate key parses"),
        idempotency_key: IdempotencyKey::parse(TEST_IDEMPOTENCY).expect("idempotency key parses"),
        event_type: EventType::parse(TEST_EVENT_TYPE).expect("event type parses"),
    }
}

fn metadata(target: &str) -> EventMetadata {
    EventMetadata::from_parts(
        crate::EventId::parse(TEST_EVENT_ID).expect("event id parses"),
        CorrelationId::parse(TEST_CORRELATION_ID).expect("correlation id parses"),
        source(),
        RecordedAt::parse(TEST_OBSERVED_AT).expect("recorded at parses"),
        Some(TargetHandler::parse(target).expect("target handler parses")),
    )
}

fn source() -> EventSource {
    EventSource::new(
        EventCustody::LocalOnly,
        RuntimeRole::ChildAgent,
        SourceService::parse(TEST_SOURCE_SERVICE).expect("source service parses"),
        SourceComponent::parse(TEST_SOURCE_COMPONENT).expect("source component parses"),
        RuntimeInstanceId::parse(TEST_INSTANCE).expect("runtime instance parses"),
    )
}

fn subscriber(id: &str, target: &str) -> EventSubscriber {
    EventSubscriber::new(
        SubscriberId::parse(id).expect("subscriber id parses"),
        EventType::parse(TEST_EVENT_TYPE).expect("event type parses"),
        TargetHandler::parse(target).expect("target handler parses"),
    )
}

#[test]
fn stored_decode_rejects_contract_mismatch() {
    let envelope = EventEnvelope::from_event(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .expect("envelope builds");
    let mut stored = envelope.store().expect("stored envelope builds");
    stored.contract.event_type = EventType::parse(OTHER_EVENT_TYPE).expect("other event parses");

    let decoded = stored.decode::<TestEvent>();

    assert!(matches!(
        decoded,
        Err(EventingError::ContractMismatch { .. })
    ));
}
