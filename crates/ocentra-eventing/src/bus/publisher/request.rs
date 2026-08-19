use crate::bus::EventBus;
use crate::request::{RequestPayload, RequestRegistry};
use crate::{
    EventClockSleep, EventMetadata, EventingError, PublishReport, RequestEvent, RequestId,
    RequestOptions, RequestReport,
};

use super::EventPublisher;

impl EventPublisher {
    /// Publishes a request as awaited work in this handler's causal chain.
    pub async fn publish_request_on<E>(
        &self,
        target_bus: &EventBus,
        event: E,
        metadata: EventMetadata,
        options: RequestOptions,
    ) -> Result<RequestReport<E::Response>, EventingError>
    where
        E: RequestEvent,
    {
        self.dispatch_chain.ensure_current_handler_task()?;
        self.dispatch_chain.ensure_live()?;
        target_bus.ensure_active()?;
        let request_id = event.request_id()?;
        let mut receiver = target_bus.requests.register::<E>(request_id.clone())?;
        let mut registration =
            CausalRequestRegistration::new(target_bus.requests.clone(), request_id.clone());
        let mut timeout = target_bus.clock.sleep(options.timeout());
        let publish_report = await_causal_publish(
            self,
            target_bus,
            event,
            metadata,
            &mut timeout,
            &mut registration,
        )
        .await?;
        let payload = await_causal_response(&mut receiver, &mut timeout, &mut registration).await?;
        registration.retain_terminal_state();
        let response = payload.decode::<E::Response>(&request_id)?;
        Ok(RequestReport {
            request_id,
            response,
            publish_report,
        })
    }
}

async fn await_causal_publish<E>(
    publisher: &EventPublisher,
    target_bus: &EventBus,
    event: E,
    metadata: EventMetadata,
    timeout: &mut EventClockSleep<'_>,
    registration: &mut CausalRequestRegistration,
) -> Result<PublishReport, EventingError>
where
    E: RequestEvent,
{
    let publish = publisher.publish_on(target_bus, event, metadata);
    tokio::pin!(publish);
    tokio::select! {
        biased;
        result = &mut publish => result,
        _ = timeout.as_mut() => Err(registration.timeout_error()),
    }
}

async fn await_causal_response(
    receiver: &mut tokio::sync::oneshot::Receiver<RequestPayload>,
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
    result: Result<RequestPayload, tokio::sync::oneshot::error::RecvError>,
    registration: &mut CausalRequestRegistration,
) -> Result<RequestPayload, EventingError> {
    match result {
        Ok(payload) => Ok(payload),
        Err(_) => Err(registration.timeout_error()),
    }
}

struct CausalRequestRegistration {
    requests: RequestRegistry,
    request_id: RequestId,
    cancel_on_drop: bool,
}

impl CausalRequestRegistration {
    fn new(requests: RequestRegistry, request_id: RequestId) -> Self {
        Self {
            requests,
            request_id,
            cancel_on_drop: true,
        }
    }

    fn retain_terminal_state(&mut self) {
        self.cancel_on_drop = false;
    }

    fn timeout_error(&mut self) -> EventingError {
        self.requests.timeout(&self.request_id);
        self.retain_terminal_state();
        EventingError::RequestTimedOut {
            request_id: self.request_id.clone(),
        }
    }
}

impl Drop for CausalRequestRegistration {
    fn drop(&mut self) {
        if self.cancel_on_drop {
            self.requests.cancel(&self.request_id);
        }
    }
}
