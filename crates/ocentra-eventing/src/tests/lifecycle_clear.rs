use std::{sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::sync::Notify;

use super::fixtures::{
    metadata, metadata_with_event_id, subscriber, subscriber_for_event, test_event_for_type,
    test_event_with_idempotency, OTHER_EVENT_TYPE, OTHER_SUBSCRIBER, OTHER_TARGET, TEST_SUBSCRIBER,
    TEST_TARGET,
};
use crate::bus::DispatchMode;
use crate::bus::EventBus;
use crate::envelope::{DomainEvent, EventContract};
use crate::error::EventingError;
use crate::execution::HandlerExecutionPolicy;
use crate::ids::{AggregateKey, IdempotencyKey, RequestId, SchemaVersion};
use crate::queue::policy::EventQueuePolicy;
use crate::request::{EventResponseContract, RequestEvent, RequestOptions};

const CLEAR_REQUEST_EVENT_TYPE: &str = "eventing.lifecycle.clear.request";
const CLEAR_REQUEST_ID: &str = "eventing-lifecycle-clear-request";
const CLEAR_REQUEST_AGGREGATE: &str = "eventing-lifecycle-clear-aggregate";
const CLEAR_REQUEST_IDEMPOTENCY: &str = "eventing-lifecycle-clear-idempotency";

#[tokio::test]
async fn clear_for_test_reports_and_resets_local_bus_state() -> Result<(), Box<dyn Error>> {
    let queue_policy = EventQueuePolicy::no_subscriber_queue(4)?;
    let bus = EventBus::with_policies(HandlerExecutionPolicy::default(), queue_policy);
    bus.subscribe::<super::fixtures::TestEvent, _, _>(
        subscriber(TEST_SUBSCRIBER, TEST_TARGET),
        |_| async { Ok(()) },
    )
    .await?;
    bus.subscribe::<super::fixtures::TestEvent, _, _>(
        subscriber_for_event(OTHER_SUBSCRIBER, OTHER_TARGET, OTHER_EVENT_TYPE),
        |_| async { Err(EventingError::empty_value("lifecycle_clear_failure")) },
    )
    .await?;
    bus.publish(
        test_event_with_idempotency("queued", "lifecycle-clear-queued"),
        metadata_with_event_id(OTHER_TARGET, "lifecycle-clear-event-1"),
    )
    .await?;
    bus.publish_with_mode(
        test_event_for_type("failed", OTHER_EVENT_TYPE),
        metadata_with_event_id(OTHER_TARGET, "lifecycle-clear-event-2"),
        DispatchMode::OrderedByAggregateKey,
    )
    .await?;

    let clear_report = bus.clear_for_test().await;
    let dead_letters_after_clear = bus.dead_letters().await;
    let journal_after_clear = bus.journal().await;
    bus.subscribe::<super::fixtures::TestEvent, _, _>(
        subscriber(TEST_SUBSCRIBER, TEST_TARGET),
        |_| async { Ok(()) },
    )
    .await?;
    let publish_after_clear = bus
        .publish(
            test_event_with_idempotency("after-clear", "lifecycle-clear-after"),
            metadata(TEST_TARGET),
        )
        .await?;

    assert_eq!(clear_report.subscription_count, 2);
    assert_eq!(clear_report.stored_journal_count, 2);
    assert_eq!(clear_report.dead_letter_count, 1);
    assert_eq!(clear_report.aggregate_gate_count, 0);
    assert_eq!(clear_report.queued_event_count, 1);
    assert_eq!(clear_report.queued_idempotency_key_count, 0);
    assert_eq!(clear_report.completed_idempotency_key_count, 0);
    assert_eq!(dead_letters_after_clear.len(), 0);
    assert_eq!(journal_after_clear.len(), 0);
    assert_eq!(publish_after_clear.handled_count, 1);
    Ok(())
}

#[tokio::test]
async fn clear_for_test_cancels_pending_request_completion() -> Result<(), Box<dyn Error>> {
    let bus = EventBus::new();
    let handler_seen = Arc::new(Notify::new());
    let handler_seen_clone = Arc::clone(&handler_seen);
    bus.subscribe::<ClearRequestEvent, _, _>(
        subscriber_for_event(
            "lifecycle-clear-request-subscriber",
            TEST_TARGET,
            CLEAR_REQUEST_EVENT_TYPE,
        ),
        move |_| {
            let handler_seen = Arc::clone(&handler_seen_clone);
            async move {
                handler_seen.notify_one();
                Ok(())
            }
        },
    )
    .await?;
    let request_bus = bus.clone();
    let request_event = ClearRequestEvent::new()?;
    let request_timeout = RequestOptions::with_timeout(Duration::from_secs(60))?;
    let request = tokio::spawn(async move {
        request_bus
            .publish_request(
                request_event,
                metadata(TEST_TARGET),
                request_timeout,
            )
            .await
    });

    handler_seen.notified().await;
    let clear_report = bus.clear_for_test().await;
    let result = request.await?;

    assert_eq!(clear_report.pending_request_count, 1);
    assert!(matches!(result, Err(EventingError::RequestTimedOut { .. })));
    Ok(())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ClearRequestEvent {
    request_id: RequestId,
}

impl ClearRequestEvent {
    fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            request_id: RequestId::parse(CLEAR_REQUEST_ID)?,
        })
    }
}

impl DomainEvent for ClearRequestEvent {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            crate::ids::EventType::parse(CLEAR_REQUEST_EVENT_TYPE)?,
            SchemaVersion::new(1)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(CLEAR_REQUEST_AGGREGATE)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(CLEAR_REQUEST_IDEMPOTENCY)
    }
}

impl RequestEvent for ClearRequestEvent {
    type Response = ClearResponse;

    fn request_id(&self) -> Result<RequestId, EventingError> {
        Ok(self.request_id.clone())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct ClearResponse {
    decision: String,
}

impl EventResponseContract for ClearResponse {}
