use std::panic::AssertUnwindSafe;

use futures::FutureExt;

use crate::{HandlerExecutionPolicy, SharedEventClock, StoredEventEnvelope};

use super::super::EventPublisher;
use super::super::SubscriberRecord;
use super::outcome::AttemptOutcome;
use super::waiting;

pub(super) async fn dispatch_attempt(
    stored: StoredEventEnvelope,
    subscriber: &SubscriberRecord,
    publisher: EventPublisher,
    policy: &HandlerExecutionPolicy,
    clock: SharedEventClock,
) -> AttemptOutcome {
    let scoped = publisher.scoped_for_handler();
    let publisher = scoped.publisher;
    // CLONE-JUSTIFICATION: the handler owns its publisher while the attempt
    // supervisor retains the same causal scope for cancellation observation.
    let attempt = AssertUnwindSafe((subscriber.handler)(stored, publisher.clone())).catch_unwind();
    let result = waiting::wait(attempt, &publisher, policy.timeout(), clock).await;
    scoped.guard.cancel();
    match result {
        Ok(Ok(Ok(()))) => AttemptOutcome::Handled,
        Ok(Ok(Err(error))) => AttemptOutcome::Failed(error),
        Ok(Err(_)) => AttemptOutcome::Panicked,
        Err(outcome) => outcome,
    }
}
