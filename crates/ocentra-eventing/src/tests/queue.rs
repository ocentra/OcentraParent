use std::{sync::Arc, time::Duration};

use tokio::sync::{Mutex, Notify};

use super::fixtures::{
    metadata, subscriber, test_event, test_event_with_idempotency, TestEvent, TEST_LABEL,
    TEST_SUBSCRIBER, TEST_TARGET,
};
use crate::{
    DeadLetterReason, DispatchMode, DomainEvent, EventBus, EventQueuePolicy, EventingError,
    QueueDisposition,
};

#[tokio::test]
async fn no_subscriber_queue_drains_after_subscriber_registers() {
    let bus = EventBus::with_queue_policy(
        EventQueuePolicy::no_subscriber_queue(2).expect("queue policy is valid"),
    );
    let queued_report = bus
        .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .expect("no-subscriber publish queues");
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

    let drain = bus
        .drain_queued(DispatchMode::Sequential)
        .await
        .expect("queued event drains");

    assert_eq!(
        queued_report.queue_report.disposition,
        QueueDisposition::QueuedNoSubscriber
    );
    assert_eq!(queued_report.queue_report.queued_count, 1);
    assert_eq!(queued_report.subscriber_count, 0);
    assert_eq!(bus.journal().await.len(), 1);
    assert_eq!(drain.queued_before, 1);
    assert_eq!(drain.dispatched_count, 1);
    assert_eq!(drain.expired_count, 0);
    assert_eq!(drain.remaining_count, 0);
    assert_eq!(drain.dispatch_reports[0].handled_count, 1);
    assert_eq!(handled.lock().await.as_slice(), &[TEST_LABEL.to_string()]);
}

#[tokio::test]
async fn bounded_queue_overflow_dead_letters_rejected_event() {
    let bus = EventBus::with_queue_policy(
        EventQueuePolicy::no_subscriber_queue(1).expect("queue policy is valid"),
    );
    bus.publish(
        test_event_with_idempotency("first", "queue-overflow-first"),
        metadata(TEST_TARGET),
    )
    .await
    .expect("first event queues");
    let report = bus
        .publish(
            test_event_with_idempotency("second", "queue-overflow-second"),
            metadata(TEST_TARGET),
        )
        .await
        .expect("overflow becomes dead letter");
    let dead_letters = bus.dead_letters().await;
    let dead_letter_event = dead_letters[0].as_event();

    assert_eq!(
        report.queue_report.disposition,
        QueueDisposition::DeadLetteredQueueOverflow
    );
    assert_eq!(report.dead_letter_count, 1);
    assert_eq!(dead_letters.len(), 1);
    assert_eq!(dead_letters[0].reason, DeadLetterReason::QueueOverflow);
    assert!(dead_letters[0].subscriber_id.is_none());
    assert!(dead_letters[0].target_handler.is_none());
    assert_eq!(dead_letter_event.reason, DeadLetterReason::QueueOverflow);
    assert_eq!(
        dead_letter_event
            .contract()
            .expect("dead-letter event contract exists")
            .event_type
            .as_str(),
        crate::DEAD_LETTER_RECORDED_EVENT_TYPE
    );
}

#[tokio::test]
async fn queued_event_expires_before_dispatch_when_ttl_elapsed() {
    let policy = EventQueuePolicy::no_subscriber_queue(2)
        .expect("queue policy is valid")
        .with_ttl(Duration::from_millis(5))
        .expect("ttl policy is valid");
    let bus = EventBus::with_queue_policy(policy);
    bus.publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
        .await
        .expect("event queues");
    tokio::time::sleep(Duration::from_millis(20)).await;
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), |_| async {
        Ok(())
    })
    .await
    .expect("subscriber registers");

    let drain = bus
        .drain_queued(DispatchMode::Sequential)
        .await
        .expect("expired queue drain succeeds");
    let dead_letters = bus.dead_letters().await;

    assert_eq!(drain.queued_before, 1);
    assert_eq!(drain.dispatched_count, 0);
    assert_eq!(drain.expired_count, 1);
    assert_eq!(drain.remaining_count, 0);
    assert_eq!(dead_letters[0].reason, DeadLetterReason::QueueExpired);
}

#[tokio::test]
async fn idempotency_registry_rejects_queued_and_completed_duplicates() {
    let policy = EventQueuePolicy::no_subscriber_queue(2)
        .expect("queue policy is valid")
        .with_idempotency_registry();
    let bus = EventBus::with_queue_policy(policy);
    bus.publish(
        test_event_with_idempotency(TEST_LABEL, "idempotency-queue-key"),
        metadata(TEST_TARGET),
    )
    .await
    .expect("first event queues");

    let queued_duplicate = bus
        .publish(
            test_event_with_idempotency("duplicate", "idempotency-queue-key"),
            metadata(TEST_TARGET),
        )
        .await;
    assert!(matches!(
        queued_duplicate,
        Err(EventingError::DuplicateIdempotencyKey { .. })
    ));

    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), |_| async {
        Ok(())
    })
    .await
    .expect("subscriber registers");
    bus.drain_queued(DispatchMode::Sequential)
        .await
        .expect("queued event drains");

    let completed_duplicate = bus
        .publish(
            test_event_with_idempotency("completed", "idempotency-queue-key"),
            metadata(TEST_TARGET),
        )
        .await;
    assert!(matches!(
        completed_duplicate,
        Err(EventingError::DuplicateIdempotencyKey { .. })
    ));
}

#[tokio::test]
async fn in_flight_duplicate_guard_rejects_concurrent_publish() {
    let bus = EventBus::with_queue_policy(EventQueuePolicy::default().with_idempotency_registry());
    let started = Arc::new(Notify::new());
    let started_clone = Arc::clone(&started);
    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), move |_| {
        let started = Arc::clone(&started_clone);
        async move {
            started.notify_waiters();
            tokio::time::sleep(Duration::from_millis(50)).await;
            Ok(())
        }
    })
    .await
    .expect("subscriber registers");

    let first_bus = bus.clone();
    let first = tokio::spawn(async move {
        first_bus
            .publish(
                test_event_with_idempotency(TEST_LABEL, "in-flight-idempotency-key"),
                metadata(TEST_TARGET),
            )
            .await
    });
    started.notified().await;
    let duplicate = bus
        .publish(
            test_event_with_idempotency("duplicate", "in-flight-idempotency-key"),
            metadata(TEST_TARGET),
        )
        .await;
    let first_report = first
        .await
        .expect("first task joins")
        .expect("first publish completes");

    assert!(matches!(
        duplicate,
        Err(EventingError::DuplicateInFlight { .. })
    ));
    assert_eq!(first_report.handled_count, 1);
}
