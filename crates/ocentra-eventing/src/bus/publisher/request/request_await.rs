use crate::bus::EventBus;
use crate::request::{RequestCompletionSignal, RequestPayload};
use crate::{EventClockSleep, EventingError, PublishReport, RequestEvent};

use super::{CausalRequestRegistration, EventPublisher};

pub(super) async fn publish<E>(
    publisher: &EventPublisher,
    target_bus: &EventBus,
    event: E,
    metadata: crate::EventMetadata,
    receiver: &mut tokio::sync::oneshot::Receiver<RequestCompletionSignal>,
    timeout: &mut EventClockSleep<'_>,
    registration: &mut CausalRequestRegistration,
) -> Result<(PublishReport, Option<RequestPayload>), EventingError>
where
    E: RequestEvent,
{
    let publish = publisher.publish_on(target_bus, event, metadata);
    tokio::pin!(publish);
    tokio::select! {
        biased;
        result = &mut publish => result.map(|report| (report, None)),
        result = receiver => {
            let payload = receive_payload(result, registration)?;
            let publish_report = tokio::select! {
                biased;
                result = &mut publish => result?,
                _ = timeout.as_mut() => return Err(registration.timeout_error()),
            };
            Ok((publish_report, Some(payload)))
        }
        _ = timeout.as_mut() => Err(registration.timeout_error()),
    }
}

pub(super) async fn response(
    receiver: &mut tokio::sync::oneshot::Receiver<RequestCompletionSignal>,
    timeout: &mut EventClockSleep<'_>,
    registration: &mut CausalRequestRegistration,
) -> Result<RequestPayload, EventingError> {
    tokio::select! {
        biased;
        result = receiver => receive_payload(result, registration),
        _ = timeout.as_mut() => Err(registration.timeout_error()),
    }
}

fn receive_payload(
    result: Result<RequestCompletionSignal, tokio::sync::oneshot::error::RecvError>,
    registration: &mut CausalRequestRegistration,
) -> Result<RequestPayload, EventingError> {
    match result {
        Ok(RequestCompletionSignal::Response(payload)) => Ok(payload),
        Ok(RequestCompletionSignal::TimedOut) => Err(registration.timeout_error()),
        Ok(RequestCompletionSignal::Cancelled) | Err(_) => Err(registration.cancelled_error()),
    }
}
