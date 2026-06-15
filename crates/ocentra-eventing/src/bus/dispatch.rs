use std::panic::AssertUnwindSafe;

use std::sync::Arc;

use futures::{future::join_all, FutureExt};

use crate::clock::SharedEventClock;
use crate::envelope::StoredEventEnvelope;
use crate::error::EventingError;
use crate::execution::HandlerExecutionPolicy;

use super::{EventPublisher, HandlerOutcome, HandlerReport, SubscriberRecord};

pub(super) async fn dispatch_sequential(
    stored: StoredEventEnvelope,
    subscribers: Vec<SubscriberRecord>,
    publisher: EventPublisher,
    policy: HandlerExecutionPolicy,
    clock: SharedEventClock,
) -> Vec<HandlerReport> {
    let mut reports = Vec::new();
    for subscriber in subscribers {
        reports.push(
            dispatch_one(
                stored.clone(),
                subscriber,
                publisher.clone(),
                policy.clone(),
                Arc::clone(&clock),
            )
            .await,
        );
    }
    reports
}

pub(super) async fn dispatch_concurrent(
    stored: StoredEventEnvelope,
    subscribers: Vec<SubscriberRecord>,
    publisher: EventPublisher,
    policy: HandlerExecutionPolicy,
    clock: SharedEventClock,
) -> Vec<HandlerReport> {
    join_all(subscribers.into_iter().map(|subscriber| {
        dispatch_one(
            stored.clone(),
            subscriber,
            publisher.clone(),
            policy.clone(),
            Arc::clone(&clock),
        )
    }))
    .await
}

async fn dispatch_one(
    stored: StoredEventEnvelope,
    subscriber: SubscriberRecord,
    publisher: EventPublisher,
    policy: HandlerExecutionPolicy,
    clock: SharedEventClock,
) -> HandlerReport {
    let subscriber_id = subscriber.id.clone();
    let target_handler = subscriber.target_handler.clone();
    for attempt in 1..=policy.max_attempts() {
        if stored.is_deadline_expired(clock.now()) {
            return HandlerReport::new(
                &stored,
                subscriber_id,
                target_handler,
                HandlerOutcome::DeadlineExpired,
                Some(EventingError::EventDeadlineExpired {
                    event_type: stored.contract.event_type.clone(),
                }),
                attempt - 1,
            );
        }
        match dispatch_attempt(
            stored.clone(),
            &subscriber,
            publisher.clone(),
            &policy,
            Arc::clone(&clock),
        )
        .await
        {
            AttemptOutcome::Handled => {
                return HandlerReport::new(
                    &stored,
                    subscriber_id,
                    target_handler,
                    HandlerOutcome::Handled,
                    None,
                    attempt,
                );
            }
            AttemptOutcome::Failed(error) if attempt == policy.max_attempts() => {
                return HandlerReport::new(
                    &stored,
                    subscriber_id,
                    target_handler,
                    HandlerOutcome::Failed,
                    Some(error),
                    attempt,
                );
            }
            AttemptOutcome::TimedOut if attempt == policy.max_attempts() => {
                return HandlerReport::new(
                    &stored,
                    subscriber_id.clone(),
                    target_handler,
                    HandlerOutcome::TimedOut,
                    Some(EventingError::HandlerTimedOut {
                        subscriber_id: subscriber_id.clone(),
                    }),
                    attempt,
                );
            }
            AttemptOutcome::Panicked => {
                return HandlerReport::new(
                    &stored,
                    subscriber_id.clone(),
                    target_handler,
                    HandlerOutcome::Panicked,
                    Some(EventingError::HandlerPanicked {
                        subscriber_id: subscriber_id.clone(),
                    }),
                    attempt,
                );
            }
            AttemptOutcome::Failed(_) | AttemptOutcome::TimedOut => {}
        }
    }
    unreachable!("handler execution policy guarantees at least one attempt")
}

async fn dispatch_attempt(
    stored: StoredEventEnvelope,
    subscriber: &SubscriberRecord,
    publisher: EventPublisher,
    policy: &HandlerExecutionPolicy,
    clock: SharedEventClock,
) -> AttemptOutcome {
    let attempt = AssertUnwindSafe((subscriber.handler)(stored, publisher)).catch_unwind();
    let result = match policy.timeout() {
        Some(timeout) => {
            tokio::select! {
                result = attempt => Ok(result),
                _ = clock.sleep(timeout) => Err(AttemptOutcome::TimedOut),
            }
        }
        None => Ok(attempt.await),
    };
    match result {
        Ok(Ok(Ok(()))) => AttemptOutcome::Handled,
        Ok(Ok(Err(error))) => AttemptOutcome::Failed(error),
        Ok(Err(_)) => AttemptOutcome::Panicked,
        Err(outcome) => outcome,
    }
}

enum AttemptOutcome {
    Handled,
    Failed(EventingError),
    TimedOut,
    Panicked,
}
