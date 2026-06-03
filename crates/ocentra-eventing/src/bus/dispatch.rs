use std::panic::AssertUnwindSafe;

use futures::{future::join_all, FutureExt};

use crate::{EventingError, StoredEventEnvelope};

use super::{EventPublisher, HandlerOutcome, HandlerReport, SubscriberRecord};

pub(super) async fn dispatch_sequential(
    stored: StoredEventEnvelope,
    subscribers: Vec<SubscriberRecord>,
    publisher: EventPublisher,
) -> Vec<HandlerReport> {
    let mut reports = Vec::new();
    for subscriber in subscribers {
        reports.push(dispatch_one(stored.clone(), subscriber, publisher.clone()).await);
    }
    reports
}

pub(super) async fn dispatch_concurrent(
    stored: StoredEventEnvelope,
    subscribers: Vec<SubscriberRecord>,
    publisher: EventPublisher,
) -> Vec<HandlerReport> {
    join_all(
        subscribers
            .into_iter()
            .map(|subscriber| dispatch_one(stored.clone(), subscriber, publisher.clone())),
    )
    .await
}

async fn dispatch_one(
    stored: StoredEventEnvelope,
    subscriber: SubscriberRecord,
    publisher: EventPublisher,
) -> HandlerReport {
    let subscriber_id = subscriber.id.clone();
    let target_handler = subscriber.target_handler.clone();
    let result = AssertUnwindSafe((subscriber.handler)(stored, publisher))
        .catch_unwind()
        .await;
    match result {
        Ok(Ok(())) => HandlerReport {
            subscriber_id,
            target_handler,
            outcome: HandlerOutcome::Handled,
            error: None,
        },
        Ok(Err(error)) => HandlerReport {
            subscriber_id,
            target_handler,
            outcome: HandlerOutcome::Failed,
            error: Some(error),
        },
        Err(_) => HandlerReport {
            error: Some(EventingError::HandlerPanicked {
                subscriber_id: subscriber_id.as_str().to_string(),
            }),
            subscriber_id,
            target_handler,
            outcome: HandlerOutcome::Panicked,
        },
    }
}
