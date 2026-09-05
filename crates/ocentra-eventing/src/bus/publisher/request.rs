use crate::bus::EventBus;
use crate::request::RequestRegistry;
use crate::{EventMetadata, EventingError, RequestEvent, RequestId, RequestOptions, RequestReport};

use super::EventPublisher;

mod request_await;

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
        let (publish_report, response_payload) = match request_await::publish(
            self,
            target_bus,
            event,
            metadata,
            &mut receiver,
            &mut timeout,
            &mut registration,
        )
        .await
        {
            Ok(report) => report,
            Err(error) => {
                registration.cancel_for_publish_failure();
                return Err(error);
            }
        };
        let payload = match response_payload {
            Some(payload) => payload,
            None => request_await::response(&mut receiver, &mut timeout, &mut registration).await?,
        };
        registration.retain_terminal_state();
        let response = payload.decode::<E::Response>(&request_id)?;
        Ok(RequestReport {
            request_id,
            response,
            publish_report,
        })
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

    fn cancelled_error(&mut self) -> EventingError {
        self.requests.cancel(&self.request_id);
        self.retain_terminal_state();
        EventingError::RequestCancelled {
            request_id: self.request_id.clone(),
        }
    }

    fn cancel_for_publish_failure(&mut self) {
        if self.cancel_on_drop {
            self.requests.cancel(&self.request_id);
            self.cancel_on_drop = false;
        }
    }
}

impl Drop for CausalRequestRegistration {
    fn drop(&mut self) {
        if self.cancel_on_drop {
            self.requests.cancel_pending(&self.request_id);
        }
    }
}
