use std::{collections::BTreeMap, future::Future, pin::Pin, sync::Arc};

use futures::future::join_all;
use tokio::sync::RwLock;

use crate::{
    DomainEvent, EventEnvelope, EventId, EventMetadata, EventType, EventingError,
    StoredEventEnvelope, SubscriberId, TargetHandler,
};

type HandlerFuture = Pin<Box<dyn Future<Output = Result<(), EventingError>> + Send>>;
type StoredHandler = dyn Fn(StoredEventEnvelope) -> HandlerFuture + Send + Sync;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DispatchMode {
    Sequential,
    Concurrent,
}

#[derive(Clone)]
pub struct EventBus {
    registry: Arc<RwLock<BTreeMap<EventType, Vec<SubscriberRecord>>>>,
    journal: Arc<RwLock<Vec<StoredEventEnvelope>>>,
    dead_letters: Arc<RwLock<Vec<DeadLetter>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(RwLock::new(BTreeMap::new())),
            journal: Arc::new(RwLock::new(Vec::new())),
            dead_letters: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn subscribe<E, F, Fut>(
        &self,
        subscriber: EventSubscriber,
        handler: F,
    ) -> Result<SubscriptionReport, EventingError>
    where
        E: DomainEvent,
        F: Fn(EventEnvelope<E>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), EventingError>> + Send + 'static,
    {
        let callback = Arc::new(handler);
        let record = SubscriberRecord {
            id: subscriber.id.clone(),
            event_type: subscriber.event_type.clone(),
            target_handler: subscriber.target_handler.clone(),
            handler: Arc::new(move |stored| {
                let callback = Arc::clone(&callback);
                Box::pin(async move {
                    let envelope = stored.decode::<E>()?;
                    callback(envelope).await
                })
            }),
        };
        self.insert_subscriber(record).await?;
        Ok(SubscriptionReport {
            subscriber_id: subscriber.id,
            event_type: subscriber.event_type,
            target_handler: subscriber.target_handler,
        })
    }

    pub async fn publish<E>(
        &self,
        event: E,
        metadata: EventMetadata,
    ) -> Result<PublishReport, EventingError>
    where
        E: DomainEvent,
    {
        self.publish_with_mode(event, metadata, DispatchMode::Sequential)
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
        let stored = EventEnvelope::from_event(event, metadata)?.store()?;
        self.journal.write().await.push(stored.clone());
        let subscribers = self.subscribers_for(&stored).await;
        let dead_letters = self
            .dispatch(stored.clone(), subscribers.clone(), dispatch_mode)
            .await;
        if !dead_letters.is_empty() {
            self.dead_letters.write().await.extend(dead_letters.clone());
        }
        Ok(PublishReport {
            event_id: stored.event_id,
            event_type: stored.contract.event_type,
            dispatch_mode,
            subscriber_count: subscribers.len(),
            handled_count: subscribers.len().saturating_sub(dead_letters.len()),
            dead_letter_count: dead_letters.len(),
        })
    }

    pub async fn journal(&self) -> Vec<StoredEventEnvelope> {
        self.journal.read().await.clone()
    }

    pub async fn dead_letters(&self) -> Vec<DeadLetter> {
        self.dead_letters.read().await.clone()
    }

    async fn insert_subscriber(&self, record: SubscriberRecord) -> Result<(), EventingError> {
        let mut registry = self.registry.write().await;
        let subscribers = registry.entry(record.event_type.clone()).or_default();
        if subscribers
            .iter()
            .any(|subscriber| subscriber.id == record.id)
        {
            return Err(EventingError::DuplicateSubscriber {
                subscriber_id: record.id.as_str().to_string(),
            });
        }
        subscribers.push(record);
        Ok(())
    }

    async fn subscribers_for(&self, stored: &StoredEventEnvelope) -> Vec<SubscriberRecord> {
        let registry = self.registry.read().await;
        let subscribers = registry
            .get(&stored.contract.event_type)
            .cloned()
            .unwrap_or_default();
        match &stored.target_handler {
            Some(target) => subscribers
                .into_iter()
                .filter(|subscriber| &subscriber.target_handler == target)
                .collect(),
            None => subscribers,
        }
    }

    async fn dispatch(
        &self,
        stored: StoredEventEnvelope,
        subscribers: Vec<SubscriberRecord>,
        dispatch_mode: DispatchMode,
    ) -> Vec<DeadLetter> {
        match dispatch_mode {
            DispatchMode::Sequential => dispatch_sequential(stored, subscribers).await,
            DispatchMode::Concurrent => dispatch_concurrent(stored, subscribers).await,
        }
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EventSubscriber {
    pub id: SubscriberId,
    pub event_type: EventType,
    pub target_handler: TargetHandler,
}

impl EventSubscriber {
    pub fn new(id: SubscriberId, event_type: EventType, target_handler: TargetHandler) -> Self {
        Self {
            id,
            event_type,
            target_handler,
        }
    }
}

#[derive(Clone)]
struct SubscriberRecord {
    id: SubscriberId,
    event_type: EventType,
    target_handler: TargetHandler,
    handler: Arc<StoredHandler>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DeadLetter {
    pub envelope: StoredEventEnvelope,
    pub target_handler: TargetHandler,
    pub error: EventingError,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SubscriptionReport {
    pub subscriber_id: SubscriberId,
    pub event_type: EventType,
    pub target_handler: TargetHandler,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PublishReport {
    pub event_id: EventId,
    pub event_type: EventType,
    pub dispatch_mode: DispatchMode,
    pub subscriber_count: usize,
    pub handled_count: usize,
    pub dead_letter_count: usize,
}

impl PublishReport {
    pub fn no_subscribers(&self) -> bool {
        self.subscriber_count == 0
    }
}

async fn dispatch_sequential(
    stored: StoredEventEnvelope,
    subscribers: Vec<SubscriberRecord>,
) -> Vec<DeadLetter> {
    let mut dead_letters = Vec::new();
    for subscriber in subscribers {
        if let Some(dead_letter) = dispatch_one(stored.clone(), subscriber).await {
            dead_letters.push(dead_letter);
        }
    }
    dead_letters
}

async fn dispatch_concurrent(
    stored: StoredEventEnvelope,
    subscribers: Vec<SubscriberRecord>,
) -> Vec<DeadLetter> {
    join_all(
        subscribers
            .into_iter()
            .map(|subscriber| dispatch_one(stored.clone(), subscriber)),
    )
    .await
    .into_iter()
    .flatten()
    .collect()
}

async fn dispatch_one(
    stored: StoredEventEnvelope,
    subscriber: SubscriberRecord,
) -> Option<DeadLetter> {
    match (subscriber.handler)(stored.clone()).await {
        Ok(()) => None,
        Err(error) => Some(DeadLetter {
            envelope: stored,
            target_handler: subscriber.target_handler,
            error,
        }),
    }
}
