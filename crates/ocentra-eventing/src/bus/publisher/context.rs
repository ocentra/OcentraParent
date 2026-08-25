use crate::{
    DomainEvent, EventEnvelope, EventingError, RequestCompletionReport, RequestEvent, RequestId,
};

use super::{EventContext, EventPublisher};

impl EventPublisher {
    pub async fn complete_request<E>(
        &self,
        request_id: RequestId,
        response: E::Response,
    ) -> Result<RequestCompletionReport, EventingError>
    where
        E: RequestEvent,
    {
        self.dispatch_chain.ensure_current_handler_task()?;
        self.dispatch_chain.ensure_live()?;
        self.bus.complete_request::<E>(request_id, response)
    }
}

impl<E> EventContext<E>
where
    E: DomainEvent,
{
    pub(crate) fn new(envelope: EventEnvelope<E>, publisher: EventPublisher) -> Self {
        Self {
            envelope,
            publisher,
        }
    }

    /// Borrows the complete event envelope delivered to this handler.
    pub fn envelope(&self) -> &EventEnvelope<E> {
        &self.envelope
    }

    /// Borrows the typed domain payload carried by this event.
    pub fn payload(&self) -> &E {
        self.envelope.payload()
    }

    /// Returns the causal publisher for nested handler work. Publication must
    /// be awaited directly by this handler task; spawned clones fail closed.
    pub fn publisher(&self) -> &EventPublisher {
        &self.publisher
    }
}

impl<E> EventContext<E>
where
    E: RequestEvent,
{
    /// Completes this request through the same handler-owned causal scope.
    pub async fn complete_request(
        &self,
        response: E::Response,
    ) -> Result<RequestCompletionReport, EventingError> {
        self.publisher
            .complete_request::<E>(self.payload().request_id()?, response)
            .await
    }
}
