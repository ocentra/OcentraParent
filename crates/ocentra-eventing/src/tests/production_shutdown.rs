use std::{sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use tokio::sync::{Mutex, Notify};

use super::fixtures::{
    metadata, metadata_with_event_id, subscriber, subscriber_for_event, test_event,
    test_event_for_type, test_event_with_idempotency, OTHER_EVENT_TYPE, OTHER_TARGET, TEST_LABEL,
    TEST_SUBSCRIBER, TEST_TARGET,
};
use crate::bus::reports::DeadLetterReason;
use crate::bus::EventBus;
use crate::bus::ShutdownMode;
use crate::envelope::{DomainEvent, EventContract};
use crate::error::EventingError;
use crate::ids::{AggregateKey, IdempotencyKey, RequestId, SchemaVersion};
use crate::queue::policy::EventQueuePolicy;
use crate::request::{EventResponseContract, RequestEvent, RequestOptions};

const SHUTDOWN_REQUEST_EVENT_TYPE: &str = "eventing.shutdown.request";
const SHUTDOWN_REQUEST_ID: &str = "eventing-shutdown-request";
const SHUTDOWN_REQUEST_AGGREGATE: &str = "eventing-shutdown-aggregate";
const SHUTDOWN_REQUEST_IDEMPOTENCY: &str = "eventing-shutdown-idempotency";

fn must_ok<T, E>(result: Result<T, E>) -> T {
    match result {
        Ok(value) => value,
        Err(_) => std::process::abort(),
    }
}

#[tokio::test]
async fn production_shutdown_drain_dispatches_queue_and_dead_letters_remaining() {
    let bus = EventBus::with_queue_policy(must_ok(EventQueuePolicy::no_subscriber_queue(4)));
    must_ok(
        bus.publish(
            test_event_with_idempotency(TEST_LABEL, "shutdown-drain-dispatch"),
            metadata_with_event_id(TEST_TARGET, "shutdown-drain-event-1"),
        )
        .await,
    );
    must_ok(
        bus.publish(
            test_event_for_type("unmatched", OTHER_EVENT_TYPE),
            metadata_with_event_id(OTHER_TARGET, "shutdown-drain-event-2"),
        )
        .await,
    );

    let handled = Arc::new(Mutex::new(Vec::new()));
    let handled_clone = Arc::clone(&handled);
    must_ok(
        bus.subscribe::<super::fixtures::TestEvent, _, _>(
            subscriber(TEST_SUBSCRIBER, TEST_TARGET),
            move |context| {
                let handled = Arc::clone(&handled_clone);
                async move {
                    handled.lock().await.push(context.payload().label.clone());
                    Ok(())
                }
            },
        )
        .await,
    );

    let report = must_ok(bus.shutdown(ShutdownMode::Drain).await);
    let dead_letters = bus.dead_letters().await;
    let publish_after_shutdown = bus.publish(test_event("after-shutdown"), metadata(TEST_TARGET)).await;
    let subscribe_after_shutdown = bus
        .subscribe::<super::fixtures::TestEvent, _, _>(
            subscriber("shutdown-subscriber-after", TEST_TARGET),
            |_| async { Ok(()) },
        )
        .await;

    assert_eq!(report.mode, ShutdownMode::Drain);
    assert!(!report.already_shutdown);
    assert_eq!(report.subscription_count, 1);
    assert_eq!(report.queued_event_count, 1);
    assert_eq!(report.queued_dispatched_count, 0);
    assert_eq!(report.queued_expired_count, 0);
    assert_eq!(report.queued_dead_lettered_count, 1);
    assert_eq!(report.queued_dropped_count, 0);
    assert_eq!(handled.lock().await.as_slice(), &[TEST_LABEL.to_string()]);
    assert_eq!(dead_letters.len(), 1);
    assert_eq!(dead_letters[0].reason, DeadLetterReason::Shutdown);
    assert!(matches!(
        publish_after_shutdown,
        Err(EventingError::BusShutdown)
    ));
    assert!(matches!(
        subscribe_after_shutdown,
        Err(EventingError::BusShutdown)
    ));
}

#[tokio::test]
async fn production_shutdown_dead_letters_queued_without_dispatch() {
    let bus = EventBus::with_queue_policy(must_ok(EventQueuePolicy::no_subscriber_queue(2)));
    must_ok(
        bus.publish(
            test_event_with_idempotency(TEST_LABEL, "shutdown-dead-letter"),
            metadata(TEST_TARGET),
        )
        .await,
    );

    let report = must_ok(bus.shutdown(ShutdownMode::DeadLetterQueued).await);
    let dead_letters = bus.dead_letters().await;

    assert_eq!(report.queued_event_count, 1);
    assert_eq!(report.queued_dispatched_count, 0);
    assert_eq!(report.queued_dead_lettered_count, 1);
    assert_eq!(dead_letters.len(), 1);
    assert_eq!(dead_letters[0].reason, DeadLetterReason::Shutdown);
}

#[tokio::test]
async fn production_shutdown_waits_for_active_dispatch_before_clearing_state() {
    let bus = EventBus::new();
    let handler_started = Arc::new(Notify::new());
    let release_handler = Arc::new(Notify::new());
    let handled = Arc::new(Mutex::new(0_usize));
    let handler_started_clone = Arc::clone(&handler_started);
    let release_handler_clone = Arc::clone(&release_handler);
    let handled_clone = Arc::clone(&handled);
    must_ok(
        bus.subscribe::<super::fixtures::TestEvent, _, _>(
            subscriber("shutdown-active-dispatch-subscriber", TEST_TARGET),
            move |_| {
                let handler_started = Arc::clone(&handler_started_clone);
                let release_handler = Arc::clone(&release_handler_clone);
                let handled = Arc::clone(&handled_clone);
                async move {
                    handler_started.notify_one();
                    release_handler.notified().await;
                    *handled.lock().await += 1;
                    Ok(())
                }
            },
        )
        .await,
    );

    let publish_bus = bus.clone();
    let publish = tokio::spawn(async move {
        publish_bus
            .publish(test_event(TEST_LABEL), metadata(TEST_TARGET))
            .await
    });
    handler_started.notified().await;
    let shutdown_bus = bus.clone();
    let shutdown = tokio::spawn(async move { shutdown_bus.shutdown(ShutdownMode::Drain).await });
    tokio::time::sleep(Duration::from_millis(10)).await;

    assert!(!shutdown.is_finished());
    release_handler.notify_waiters();
    let report = must_ok(must_ok(shutdown.await));
    let publish_report = must_ok(must_ok(publish.await));

    assert_eq!(report.in_flight_dispatch_count, 1);
    assert_eq!(report.subscription_count, 1);
    assert_eq!(publish_report.handled_count, 1);
    assert_eq!(*handled.lock().await, 1);
    assert!(matches!(
        bus.publish(test_event("after-active-shutdown"), metadata(TEST_TARGET))
            .await,
        Err(EventingError::BusShutdown)
    ));
}

#[tokio::test]
async fn test_only_shutdown_drop_reports_dropped_queued_work() {
    let bus = EventBus::with_queue_policy(must_ok(EventQueuePolicy::no_subscriber_queue(2)));
    must_ok(
        bus.publish(
            test_event_with_idempotency(TEST_LABEL, "shutdown-drop-test-only"),
            metadata(TEST_TARGET),
        )
        .await,
    );

    let report = must_ok(bus.shutdown(ShutdownMode::DropQueuedForTestOnly).await);
    let dead_letters = bus.dead_letters().await;

    assert_eq!(report.queued_event_count, 1);
    assert_eq!(report.queued_dead_lettered_count, 0);
    assert_eq!(report.queued_dropped_count, 1);
    assert!(dead_letters.is_empty());
}

#[tokio::test]
async fn production_shutdown_cancels_pending_request_completion() {
    let bus = EventBus::new();
    let handler_seen = Arc::new(Notify::new());
    let handler_seen_clone = Arc::clone(&handler_seen);
    must_ok(
        bus.subscribe::<ShutdownRequestEvent, _, _>(
            subscriber_for_event(
                "shutdown-request-subscriber",
                TEST_TARGET,
                SHUTDOWN_REQUEST_EVENT_TYPE,
            ),
            move |_| {
                let handler_seen = Arc::clone(&handler_seen_clone);
                async move {
                    handler_seen.notify_one();
                    Ok(())
                }
            },
        )
        .await,
    );

    let request_bus = bus.clone();
    let request = tokio::spawn(async move {
        request_bus
            .publish_request(
                ShutdownRequestEvent::new(),
                metadata(TEST_TARGET),
                must_ok(RequestOptions::with_timeout(Duration::from_secs(60))),
            )
            .await
    });

    handler_seen.notified().await;
    let report = must_ok(bus.shutdown(ShutdownMode::Drain).await);
    let result = must_ok(request.await);
    let second_shutdown = must_ok(bus.shutdown(ShutdownMode::Drain).await);

    assert_eq!(report.pending_request_count, 1);
    assert!(matches!(result, Err(EventingError::RequestTimedOut { .. })));
    assert!(second_shutdown.already_shutdown);
    assert_eq!(second_shutdown.queued_event_count, 0);
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ShutdownRequestEvent {
    request_id: RequestId,
}

impl ShutdownRequestEvent {
    fn new() -> Self {
        Self {
            request_id: must_ok(RequestId::parse(SHUTDOWN_REQUEST_ID)),
        }
    }
}

impl DomainEvent for ShutdownRequestEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            crate::ids::EventType::parse(SHUTDOWN_REQUEST_EVENT_TYPE)?,
            SchemaVersion::new(1)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(SHUTDOWN_REQUEST_AGGREGATE)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(SHUTDOWN_REQUEST_IDEMPOTENCY)
    }
}

impl RequestEvent for ShutdownRequestEvent {
    type Response = ShutdownResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ShutdownResponse {
    decision: String,
}

impl EventResponseContract for ShutdownResponse {}
