use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::{
    bus::publisher::RootEventPublisher, DomainEvent, EventEnvelope, EventSubscriber, EventingError,
    ExpectValue, SubscriptionHandle,
};

pub struct EventRecorder<E>
where
    E: DomainEvent + Clone + Send + Sync + 'static,
{
    events: Arc<Mutex<Vec<EventEnvelope<E>>>>,
    handle: SubscriptionHandle,
    _event: PhantomData<E>,
}

impl<E> EventRecorder<E>
where
    E: DomainEvent + Clone + Send + Sync + 'static,
{
    pub async fn attach(
        bus: &RootEventPublisher,
        subscriber: EventSubscriber,
    ) -> Result<Self, EventingError> {
        let events = Arc::new(Mutex::new(Vec::new()));
        let recorded_events = Arc::clone(&events);
        let handle = bus
            .subscribe_with_handle::<E, _, _>(subscriber, move |context| {
                let recorded_events = Arc::clone(&recorded_events);
                async move {
                    recorded_events
                        .lock()
                        .expect_value("event recorder lock")
                        .push(context.envelope().clone());
                    Ok(())
                }
            })
            .await?;
        Ok(Self {
            events,
            handle,
            _event: PhantomData,
        })
    }

    pub async fn recorded(&self) -> Vec<EventEnvelope<E>> {
        self.events
            .lock()
            .expect_value("event recorder lock")
            .clone()
    }

    pub fn unsubscribe(&self) -> bool {
        self.handle.unsubscribe().removed
    }
}
