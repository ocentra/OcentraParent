use std::sync::Arc;

use tokio::sync::Mutex;

use super::fixtures::{
    metadata, subscriber, test_event, TestEvent, OTHER_EVENT_TYPE, OTHER_SUBSCRIBER, OTHER_TARGET,
    TEST_LABEL, TEST_SUBSCRIBER, TEST_TARGET,
};
use crate::{
    DispatchMode, EventBus, EventEnvelope, EventType, EventingError, RecordedAt, SchemaVersion,
};

#[tokio::test]
async fn event_bus_dispatches_typed_envelope_and_stores_serialized_boundary() {
    let bus = EventBus::new();
    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |context| {
        let handled = Arc::clone(&handled_clone);
        async move {
            handled.lock().await.push(context.envelope.payload.label);
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

#[test]
fn stored_decode_rejects_contract_mismatch() {
    let envelope = EventEnvelope::from_event(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .expect("envelope builds");
    let mut stored = envelope.store().expect("stored envelope builds");
    stored.contract.event_type = EventType::parse(OTHER_EVENT_TYPE).expect("other event parses");

    let decoded = stored.decode::<super::fixtures::TestEvent>();

    assert!(matches!(
        decoded,
        Err(EventingError::ContractMismatch { .. })
    ));
}
