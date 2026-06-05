use std::time::Duration;

use super::fixtures::{
    metadata, metadata_with_event_id, subscriber, test_event_with_idempotency, TestEvent,
    TEST_LABEL, TEST_SUBSCRIBER, TEST_TARGET,
};
use crate::{DeadLetterReason, EventBus, EventQueuePolicy, EventingError, RequestOptions};

const IN_MEMORY_RETENTION_PROBE_COUNT: usize = 4097;
const EXPECTED_IN_MEMORY_RETENTION_LIMIT: usize = 4096;

#[tokio::test]
async fn metrics_snapshot_reports_queue_dead_letter_journal_and_request_counts() {
    let policy = EventQueuePolicy::no_subscriber_queue(1)
        .expect("queue policy is valid")
        .with_idempotency_registry();
    let bus = EventBus::with_queue_policy(policy);
    bus.publish(
        test_event_with_idempotency(TEST_LABEL, "metrics-queued-idempotency"),
        metadata_with_event_id(TEST_TARGET, "metrics-event-1"),
    )
    .await
    .expect("first event queues");
    bus.publish(
        test_event_with_idempotency("overflow", "metrics-overflow-idempotency"),
        metadata_with_event_id(TEST_TARGET, "metrics-event-2"),
    )
    .await
    .expect("overflow drops oldest and keeps newest");

    let queued = bus.metrics_snapshot().await;
    assert_eq!(queued.subscription_count, 0);
    assert_eq!(queued.stored_event_count, 2);
    assert_eq!(queued.dead_letter_count, 1);
    assert_eq!(queued.queue.queued_event_count, 1);
    assert_eq!(queued.queue.queued_event_id_count, 1);
    assert_eq!(queued.queue.queued_idempotency_key_count, 1);
    assert_eq!(queued.queue.completed_idempotency_key_count, 1);
    assert_eq!(queued.queue.capacity, Some(1));

    bus.subscribe::<TestEvent, _, _>(subscriber(TEST_SUBSCRIBER, TEST_TARGET), |_| async {
        Ok(())
    })
    .await
    .expect("subscriber drains queue");
    let drained = bus.metrics_snapshot().await;
    assert_eq!(drained.subscription_count, 1);
    assert_eq!(drained.queue.queued_event_count, 0);
    assert_eq!(drained.queue.completed_idempotency_key_count, 2);

    let timeout = bus
        .publish_request(
            SlowMetricsRequest::new(),
            metadata(TEST_TARGET),
            RequestOptions::with_timeout(Duration::from_millis(1)).expect("timeout parses"),
        )
        .await;
    assert!(matches!(
        timeout,
        Err(EventingError::RequestTimedOut { .. })
    ));
    let timed_out = bus.metrics_snapshot().await;
    assert_eq!(timed_out.requests.pending_request_count, 0);
    assert_eq!(timed_out.requests.timed_out_request_count, 1);
    assert_eq!(timed_out.dead_letter_count, 1);
    assert_eq!(
        bus.dead_letters().await[0].reason,
        DeadLetterReason::QueueOverflow
    );
}

#[tokio::test]
async fn metrics_snapshot_reports_bounded_in_memory_event_retention() {
    let bus = EventBus::new();
    for index in 0..IN_MEMORY_RETENTION_PROBE_COUNT {
        bus.publish(
            test_event_with_idempotency(TEST_LABEL, &format!("retention-key-{index}")),
            metadata_with_event_id(TEST_TARGET, &format!("retention-event-{index}")),
        )
        .await
        .expect("retention probe event publishes");
    }

    let metrics = bus.metrics_snapshot().await;
    let journal = bus.journal().await;

    assert_eq!(
        metrics.stored_event_count,
        EXPECTED_IN_MEMORY_RETENTION_LIMIT
    );
    assert_eq!(journal.len(), EXPECTED_IN_MEMORY_RETENTION_LIMIT);
    assert_eq!(
        journal
            .first()
            .expect("retained journal has first entry")
            .event_id
            .as_str(),
        "retention-event-1"
    );
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct SlowMetricsRequest {
    request_id: crate::RequestId,
}

impl SlowMetricsRequest {
    fn new() -> Self {
        Self {
            request_id: crate::RequestId::parse("metrics-request-1").expect("request id parses"),
        }
    }
}

impl crate::DomainEvent for SlowMetricsRequest {
    fn contract(&self) -> Result<crate::EventContract, EventingError> {
        Ok(crate::EventContract::new(
            crate::EventType::parse("eventing.metrics.request")?,
            crate::SchemaVersion::new(1)?,
        ))
    }

    fn aggregate_key(&self) -> Result<crate::AggregateKey, EventingError> {
        crate::AggregateKey::parse("metrics-request-aggregate")
    }

    fn idempotency_key(&self) -> Result<crate::IdempotencyKey, EventingError> {
        crate::IdempotencyKey::parse("metrics-request-idempotency")
    }
}

impl crate::RequestEvent for SlowMetricsRequest {
    type Response = MetricsResponse;

    fn request_id(&self) -> Result<crate::RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
struct MetricsResponse {
    decision: String,
}

impl crate::EventResponseContract for MetricsResponse {}
