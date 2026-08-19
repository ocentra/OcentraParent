use std::ops::Deref;

use crate::{DomainEvent, EventEnvelope, EventMetadata, EventingError};

use super::{
    dispatch_chain::DispatchChain, handler_scope::HandlerScopeGuard, DispatchMode, EventBus,
    PublishReport,
};

mod context;

/// The explicit authority for independent root publication on one event bus.
///
/// A raw [`EventBus`] deliberately has no root publication methods. Runtime
/// bootstrap receives this capability from an `EventBus` constructor and may
/// delegate it only to code that owns independent root ingress. Event handlers
/// receive [`EventPublisher`] instead, so captured raw buses cannot forge a new
/// causal root.
#[derive(Clone)]
pub struct RootEventPublisher {
    pub(super) bus: EventBus,
    _authority: RootPublicationAuthority,
}

#[derive(Clone)]
struct RootPublicationAuthority;

impl RootEventPublisher {
    pub(super) fn for_bus(bus: EventBus) -> Self {
        Self {
            bus,
            _authority: RootPublicationAuthority,
        }
    }

    /// Borrows the raw bus for inspection or as a handler-scoped causal target,
    /// without transferring root publication, subscription, replay, or drain
    /// authority.
    pub fn event_bus(&self) -> &EventBus {
        &self.bus
    }
}

impl Deref for RootEventPublisher {
    type Target = EventBus;

    fn deref(&self) -> &Self::Target {
        &self.bus
    }
}

impl std::fmt::Debug for RootEventPublisher {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("RootEventPublisher")
    }
}

/// A handler-scoped publisher that preserves ordered-dispatch causality.
///
/// Handler code must use this publisher for awaited nested publication; a
/// captured raw [`EventBus`] has no root publication authority. The publisher
/// is bound to the exact task polling its handler. Moving a clone into
/// `tokio::spawn` is rejected before publication mutates queue, dead-letter, or
/// journal state, because Tokio does not inherit handler task-local identity.
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

    /// Publishes causal nested work on the handler's originating bus.
    pub async fn publish<E>(
        &self,
        event: E,
        metadata: EventMetadata,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        self.publish_causal_on(&self.bus, event, metadata, DispatchMode::Sequential)
            .await
    }

    /// Publishes causal nested work with an explicit dispatch mode.
    pub async fn publish_with_mode<E>(
        &self,
        event: E,
        metadata: EventMetadata,
        dispatch_mode: DispatchMode,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        self.publish_causal_on(&self.bus, event, metadata, dispatch_mode)
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
        self.publish_causal_on(target_bus, event, metadata, dispatch_mode)
            .await
    }

    pub(super) fn scoped_for_handler(&self) -> HandlerScopedPublisher {
        let scoped_chain = self.dispatch_chain.scoped_to_handler();
        HandlerScopedPublisher {
            publisher: Self {
                // CLONE-JUSTIFICATION: a handler receives another handle to
                // the same bus; it receives no root-publication authority.
                bus: self.bus.clone(),
                dispatch_chain: scoped_chain.chain,
            },
            guard: scoped_chain.guard,
        }
    }

    pub(super) async fn causal_scope_cancelled(&self) {
        self.dispatch_chain.cancelled().await;
    }

    async fn publish_causal_on<E>(
        &self,
        target_bus: &EventBus,
        event: E,
        metadata: EventMetadata,
        dispatch_mode: DispatchMode,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        self.dispatch_chain.ensure_current_handler_task()?;
        self.dispatch_chain.ensure_live()?;
        // CLONE-JUSTIFICATION: one causal-chain snapshot moves into publication;
        // a separate snapshot observes ancestor cancellation.
        let dispatch_chain = self.dispatch_chain.clone();
        let publish = target_bus.publish_causal_in_chain(
            event,
            metadata,
            dispatch_mode,
            dispatch_chain.clone(),
        );
        tokio::pin!(publish);
        // CANCEL-SAFE: dropping publication releases every admission lease by
        // RAII, and the biased cancellation branch wins if both become ready.
        tokio::select! {
            biased;
            _ = dispatch_chain.cancelled() => Err(EventingError::CausalDispatchCancelled),
            result = publish => result,
        }
    }
}

pub(super) struct HandlerScopedPublisher {
    pub(super) publisher: EventPublisher,
    pub(super) guard: HandlerScopeGuard,
}

impl std::fmt::Debug for EventPublisher {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str("EventPublisher")
    }
}

/// The immutable event envelope and causal publisher supplied to a handler.
#[derive(Clone, Debug)]
pub struct EventContext<E>
where
    E: DomainEvent,
{
    envelope: EventEnvelope<E>,
    publisher: EventPublisher,
}
