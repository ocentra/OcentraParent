use crate::{
    DomainEvent, EventEnvelope, EventMetadata, EventingError, RequestCompletionReport,
    RequestEvent, RequestId,
};

use super::{dispatch_chain::DispatchChain, DispatchMode, EventBus, PublishReport};

/// A handler-scoped publisher that preserves ordered-dispatch causality.
///
/// Clones retain the same causal chain, including clones moved into
/// `tokio::spawn`. Handler code must use this publisher for awaited nested
/// publication; publishing through a captured [`EventBus`] starts an
/// independent root publication and cannot safely bypass an aggregate gate
/// held by the handler. A spawned publication must remain awaited by and finish
/// within its handler; cloning this value is not authority to detach work.
#[derive(Clone)]
pub struct EventPublisher {
    bus: EventBus,
    dispatch_chain: DispatchChain,
}

impl EventPublisher {
    pub(super) fn for_dispatch(bus: EventBus, dispatch_chain: DispatchChain) -> Self {
        Self {
            bus,
            dispatch_chain,
        }
    }

    pub async fn publish<E>(
        &self,
        event: E,
        metadata: EventMetadata,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        self.bus
            .publish_in_chain(
                event,
                metadata,
                DispatchMode::Sequential,
                self.dispatch_chain.clone(),
            )
            .await
    }

    pub async fn publish_with_mode<E>(
        &self,
        event: E,
        metadata: EventMetadata,
        dispatch_mode: DispatchMode,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        self.bus
            .publish_in_chain(event, metadata, dispatch_mode, self.dispatch_chain.clone())
            .await
    }

    /// Publishes to another bus while retaining this handler's causal chain.
    ///
    /// This is the cross-bus counterpart to [`Self::publish`]. A raw call on
    /// `target_bus` would create an unrelated root publication and must not be
    /// used for awaited work caused by the current handler.
    pub async fn publish_on<E>(
        &self,
        target_bus: &EventBus,
        event: E,
        metadata: EventMetadata,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        self.publish_on_with_mode(target_bus, event, metadata, DispatchMode::Sequential)
            .await
    }

    /// Publishes to another bus with an explicit dispatch mode while retaining
    /// this handler's causal chain.
    pub async fn publish_on_with_mode<E>(
        &self,
        target_bus: &EventBus,
        event: E,
        metadata: EventMetadata,
        dispatch_mode: DispatchMode,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        target_bus
            .publish_in_chain(event, metadata, dispatch_mode, self.dispatch_chain.clone())
            .await
    }

    pub(crate) async fn complete_request<E>(
        &self,
        request_id: RequestId,
        response: E::Response,
    ) -> Result<RequestCompletionReport, EventingError>
    where
        E: RequestEvent,
    {
        self.bus.complete_request::<E>(request_id, response).await
    }
}

impl std::fmt::Debug for EventPublisher {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("EventPublisher")
    }
}

#[derive(Clone, Debug)]
pub struct EventContext<E>
where
    E: DomainEvent,
{
    envelope: EventEnvelope<E>,
    publisher: EventPublisher,
}

impl<E> EventContext<E>
where
    E: DomainEvent,
{
    pub(super) fn new(envelope: EventEnvelope<E>, publisher: EventPublisher) -> Self {
        Self {
            envelope,
            publisher,
        }
    }

    pub fn envelope(&self) -> &EventEnvelope<E> {
        &self.envelope
    }

    pub fn payload(&self) -> &E {
        self.envelope.payload()
    }

    /// Returns the causal publisher for nested handler work. Clone this value
    /// into spawned tasks; unlike task-local state, the explicit chain survives
    /// the spawn boundary.
    pub fn publisher(&self) -> &EventPublisher {
        &self.publisher
    }
}

impl<E> EventContext<E>
where
    E: RequestEvent,
{
    pub async fn complete_request(
        &self,
        response: E::Response,
    ) -> Result<RequestCompletionReport, EventingError> {
        self.publisher
            .complete_request::<E>(self.payload().request_id()?, response)
            .await
    }
}
