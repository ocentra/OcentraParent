use std::{future::Future, time::Duration};

use crate::{EventingError, SharedEventClock};

use super::super::EventPublisher;
use super::outcome::AttemptOutcome;

pub(super) async fn wait<F>(
    attempt: F,
    publisher: &EventPublisher,
    timeout: Option<Duration>,
    clock: SharedEventClock,
) -> Result<F::Output, AttemptOutcome>
where
    F: Future,
{
    match timeout {
        Some(timeout) => {
            // CANCEL-SAFE: dropping the handler future cancels awaited work;
            // the handler-scope guard also invalidates every spawned descendant.
            tokio::select! {
                biased;
                _ = publisher.causal_scope_cancelled() => cancelled(),
                result = attempt => Ok(result),
                _ = clock.sleep(timeout) => Err(AttemptOutcome::TimedOut),
            }
        }
        None => {
            // CANCEL-SAFE: dropping the handler future cancels awaited work;
            // the handler-scope guard also invalidates every spawned descendant.
            tokio::select! {
                biased;
                _ = publisher.causal_scope_cancelled() => cancelled(),
                result = attempt => Ok(result),
            }
        }
    }
}

fn cancelled<T>() -> Result<T, AttemptOutcome> {
    Err(AttemptOutcome::Failed(
        EventingError::CausalDispatchCancelled,
    ))
}
