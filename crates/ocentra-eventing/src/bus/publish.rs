use tokio::task::JoinHandle;

use crate::{
    DomainEvent, EventMetadata, EventingError, JournalAppend, RequestCompletionReport,
    RequestEvent, RequestId, RequestOptions, RequestReport, StoredEventEnvelope,
};

use super::{
    dispatch_chain::DispatchChain, reports::dead_letter::DeadLetter, DispatchMode, EventBus,
    PublishReport, SubscriberRecord,
};

mod flow;
mod request;

pub(crate) enum DispatchStoredError {
    BeforeDispatch(EventingError),
    AfterDispatch(EventingError),
}

impl DispatchStoredError {
    fn into_error(self) -> EventingError {
        match self {
            Self::BeforeDispatch(error) | Self::AfterDispatch(error) => error,
        }
    }
}

impl From<EventingError> for DispatchStoredError {
    fn from(error: EventingError) -> Self {
        Self::BeforeDispatch(error)
    }
}

impl EventBus {
    /// Publishes a root event with no handler-owned causal dispatch chain.
    ///
    /// Handler code that awaits a nested publication must use the
    /// [`super::publisher::EventPublisher`] supplied by its event context. A
    /// captured `EventBus` represents independent root work and will wait for
    /// any existing ordered aggregate owner.
    pub async fn publish<E>(
        &self,
        event: E,
        metadata: EventMetadata,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        flow::publish_with_mode(self, event, metadata, DispatchMode::Sequential).await
    }

    pub async fn publish_and_wait<E>(
        &self,
        event: E,
        metadata: EventMetadata,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        self.publish(event, metadata).await
    }

    /// Spawns an independent root publication.
    ///
    /// Handler-owned spawned work must instead clone the publisher from its
    /// event context so ordered causality survives the spawn boundary.
    pub fn publish_detached<E>(
        &self,
        event: E,
        metadata: EventMetadata,
        dispatch_mode: DispatchMode,
    ) -> JoinHandle<Result<PublishReport, EventingError>>
    where
        E: DomainEvent,
    {
        let bus = self.clone();
        tokio::spawn(
            async move { flow::publish_with_mode(&bus, event, metadata, dispatch_mode).await },
        )
    }

    pub async fn publish_request<E>(
        &self,
        event: E,
        metadata: EventMetadata,
        options: RequestOptions,
    ) -> Result<RequestReport<E::Response>, EventingError>
    where
        E: RequestEvent,
    {
        request::publish_request(self, event, metadata, options).await
    }

    /// Publishes a root event with an explicit dispatch mode.
    ///
    /// Handler code must use its context publisher for causal nested work.
    pub async fn publish_with_mode<E>(
        &self,
        event: E,
        metadata: EventMetadata,
        dispatch_mode: DispatchMode,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        flow::publish_with_mode(self, event, metadata, dispatch_mode).await
    }

    pub(super) async fn publish_in_chain<E>(
        &self,
        event: E,
        metadata: EventMetadata,
        dispatch_mode: DispatchMode,
        dispatch_chain: DispatchChain,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        flow::publish_with_mode_in_chain(self, event, metadata, dispatch_mode, dispatch_chain).await
    }

    /// Publishes only after the selected before-dispatch journal append passes
    /// the caller's durable-receipt predicate. This is an authorization-boundary
    /// API: a failed predicate prevents every subscriber from observing the event.
    pub async fn publish_with_mode_and_before_dispatch_receipt_validator<E>(
        &self,
        event: E,
        metadata: EventMetadata,
        dispatch_mode: DispatchMode,
        validator: fn(&JournalAppend) -> Result<(), EventingError>,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        flow::publish_with_mode_and_before_dispatch_receipt_validator(
            self,
            event,
            metadata,
            dispatch_mode,
            validator,
        )
        .await
    }

    pub async fn journal(&self) -> Vec<StoredEventEnvelope> {
        self.stored_journal.read().await.clone()
    }

    pub async fn dead_letters(&self) -> Vec<DeadLetter> {
        self.dead_letters.read().await.clone()
    }

    pub(super) async fn complete_request<E>(
        &self,
        request_id: RequestId,
        response: E::Response,
    ) -> Result<RequestCompletionReport, EventingError>
    where
        E: RequestEvent,
    {
        self.requests.complete::<E>(request_id, response)
    }
}
