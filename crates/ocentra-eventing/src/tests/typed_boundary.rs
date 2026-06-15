use std::sync::Arc;

use tokio::sync::Mutex;

use super::fixtures::{
    metadata, subscriber, test_event, TestEvent, OTHER_EVENT_TYPE, OTHER_SUBSCRIBER, OTHER_TARGET,
    TEST_LABEL, TEST_SUBSCRIBER, TEST_TARGET,
};
use crate::bus::DispatchMode;
use crate::bus::EventBus;
use crate::envelope::{EventEnvelope, EventPriority};
use crate::error::EventingError;
use crate::ids::{CausationId, EventNamespace, EventType, RecordedAt, SchemaVersion};

#[tokio::test]
async fn event_bus_dispatches_typed_envelope_and_stores_serialized_boundary() -> Result<(), String> {
    let bus = EventBus::new();
    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |context| {
        let handled = Arc::clone(&handled_clone);
        async move {
            handled.lock().await.push(context.payload().label.clone());
            Ok(())
        }
    })
    .await
    .map_err(|err| err.to_string())?;

    let causation_id = CausationId::parse("causation-test-1").map_err(|err| err.to_string())?;
    let metadata = metadata(TEST_TARGET)
        .with_causation_id(causation_id)
        .with_priority(EventPriority::High);
    let report = bus
        .publish(test_event(TEST_LABEL), metadata)
        .await
        .map_err(|err| err.to_string())?;
    let journal = bus.journal().await;
    let decoded: EventEnvelope<TestEvent> = journal[0].decode().map_err(|err| err.to_string())?;

    assert_eq!(report.subscriber_count, 1);
    assert_eq!(report.handled_count, 1);
    assert_eq!(handled.lock().await.as_slice(), &[TEST_LABEL.to_string()]);
    assert_eq!(decoded.payload.label, TEST_LABEL);
    let stored_causation_id = match decoded.causation_id.as_ref() {
        Some(causation_id) => causation_id.as_str(),
        None => return Err("causation id is stored".to_string()),
    };
    assert_eq!(stored_causation_id, "causation-test-1");
    assert_eq!(decoded.priority, EventPriority::High);
    assert_eq!(journal.len(), 1);

    Ok(())
}

#[tokio::test]
async fn target_handler_filter_prevents_wrong_handler_delivery() -> Result<(), String> {
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
    .map_err(|err| err.to_string())?;
    bus.subscribe::<TestEvent, _, _>(subscriber(OTHER_SUBSCRIBER, OTHER_TARGET), |_| async {
        Ok(())
    })
    .await
    .map_err(|err| err.to_string())?;

    let report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .map_err(|err| err.to_string())?;

    assert_eq!(report.subscriber_count, 1);
    assert_eq!(*handled.lock().await, 1);

    Ok(())
}

#[tokio::test]
async fn concurrent_dispatch_records_handler_dead_letter_without_losing_journal(
) -> Result<(), String> {
    let bus = EventBus::new();
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), |_| async {
        Err(EventingError::empty_value("handler_failure"))
    })
    .await
    .map_err(|err| err.to_string())?;

    let report = bus
        .publish_with_mode(
            test_event(TEST_LABEL),
            metadata(TEST_TARGET),
            DispatchMode::Concurrent,
        )
        .await
        .map_err(|err| err.to_string())?;
    let dead_letters = bus.dead_letters().await;

    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(report.handled_count, 0);
    assert_eq!(bus.journal().await.len(), 1);
    let target_handler = match dead_letters[0].target_handler.as_ref() {
        Some(target_handler) => target_handler.as_str(),
        None => return Err("handler dead letter has target".to_string()),
    };
    assert_eq!(target_handler, TEST_TARGET);

    Ok(())
}

#[tokio::test]
async fn duplicate_subscriber_ids_are_rejected() -> Result<(), String> {
    let bus = EventBus::new();
    let duplicate = subscriber(TEST_SUBSCRIBER, TEST_TARGET);
    bus.subscribe::<TestEvent, _, _>(duplicate.clone(), |_| async { Ok(()) })
        .await
        .map_err(|err| err.to_string())?;

    let result = bus
        .subscribe::<TestEvent, _, _>(duplicate, |_| async { Ok(()) })
        .await;

    assert!(matches!(
        result,
        Err(EventingError::DuplicateSubscriber { .. })
    ));

    Ok(())
}

#[test]
fn eventing_newtypes_reject_empty_values_and_zero_versions() {
    assert!(EventType::parse("").is_err());
    assert!(EventType::parse(".leading").is_err());
    assert!(EventType::parse("trailing.").is_err());
    assert!(EventType::parse("empty..segment").is_err());
    assert!(EventType::parse("eventing/slash-taxonomy/observed").is_ok());
    assert!(RecordedAt::parse(" ").is_err());
    assert!(SchemaVersion::new(0).is_err());
}

#[test]
fn event_namespaces_match_dot_and_slash_event_taxonomy() -> Result<(), String> {
    let slash_event = EventType::parse("network/transport/observed").map_err(|err| err.to_string())?;
    let dot_event = EventType::parse("network.transport.observed").map_err(|err| err.to_string())?;
    let network_namespace = EventNamespace::parse("network").map_err(|err| err.to_string())?;

    let slash_namespace =
        EventNamespace::from_event_type(&slash_event).map_err(|err| err.to_string())?;
    assert_eq!(slash_namespace.as_str(), "network");
    assert!(network_namespace.matches_event_type(&slash_event));
    assert!(network_namespace.matches_event_type(&dot_event));

    Ok(())
}

#[test]
fn stored_decode_rejects_contract_mismatch() -> Result<(), String> {
    let envelope =
        EventEnvelope::from_event(test_event(TEST_LABEL), metadata(TEST_TARGET)).map_err(|err| err.to_string())?;
    let mut stored = envelope.store().map_err(|err| err.to_string())?;
    stored.contract.event_type = EventType::parse(OTHER_EVENT_TYPE).map_err(|err| err.to_string())?;

    let decoded = stored.decode::<super::fixtures::TestEvent>();

    assert!(matches!(
        decoded,
        Err(EventingError::ContractMismatch { .. })
    ));

    Ok(())
}
