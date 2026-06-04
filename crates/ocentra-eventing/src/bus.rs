use std::{
    collections::BTreeMap,
    future::Future,
    sync::{Arc, Mutex},
};

use tokio::{sync::Mutex as AsyncMutex, sync::RwLock, task::JoinHandle};

use crate::{
    AggregateKey, DomainEvent, EventEnvelope, EventMetadata, EventType, EventingError,
    HandlerExecutionPolicy, StoredEventEnvelope,
};

mod dispatch;
mod publisher;
mod reports;
mod subscriber;

use dispatch::{dispatch_concurrent, dispatch_sequential};
use reports::dead_letters_for;
use subscriber::{insert_subscriber, record_for, SubscriberRecord};

pub use publisher::{EventContext, EventPublisher};
pub use reports::{DeadLetter, EventTraceFields, HandlerOutcome, HandlerReport, PublishReport};
pub use subscriber::{EventSubscriber, SubscriptionHandle, SubscriptionReport, UnsubscribeReport};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DispatchMode {
    Sequential,
    Concurrent,
    OrderedByAggregateKey,
}

#[derive(Clone)]
pub struct EventBus {
    registry: Arc<Mutex<BTreeMap<EventType, Vec<SubscriberRecord>>>>,
    journal: Arc<RwLock<Vec<StoredEventEnvelope>>>,
    dead_letters: Arc<RwLock<Vec<DeadLetter>>>,
    aggregate_locks: Arc<Mutex<BTreeMap<AggregateKey, Arc<AsyncMutex<()>>>>>,
    handler_policy: HandlerExecutionPolicy,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(BTreeMap::new())),
            journal: Arc::new(RwLock::new(Vec::new())),
            dead_letters: Arc::new(RwLock::new(Vec::new())),
            aggregate_locks: Arc::new(Mutex::new(BTreeMap::new())),
            handler_policy: HandlerExecutionPolicy::default(),
        }
    }

    pub fn with_handler_policy(policy: HandlerExecutionPolicy) -> Self {
        Self {
            handler_policy: policy,
            ..Self::new()
        }
    }

    pub async fn subscribe<E, F, Fut>(
        &self,
        subscriber: EventSubscriber,
        handler: F,
    ) -> Result<SubscriptionReport, EventingError>
    where
        E: DomainEvent,
        F: Fn(EventContext<E>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), EventingError>> + Send + 'static,
    {
        let report = SubscriptionReport {
            subscriber_id: subscriber.id.clone(),
            event_type: subscriber.event_type.clone(),
            target_handler: subscriber.target_handler.clone(),
        };
        self.insert_subscriber(record_for(subscriber, handler)?)?;
        Ok(report)
    }

    pub async fn subscribe_with_handle<E, F, Fut>(
        &self,
        subscriber: EventSubscriber,
        handler: F,
    ) -> Result<SubscriptionHandle, EventingError>
    where
        E: DomainEvent,
        F: Fn(EventContext<E>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<(), EventingError>> + Send + 'static,
    {
        let report = SubscriptionReport {
            subscriber_id: subscriber.id.clone(),
            event_type: subscriber.event_type.clone(),
            target_handler: subscriber.target_handler.clone(),
        };
        self.insert_subscriber(record_for(subscriber, handler)?)?;
        Ok(SubscriptionHandle::new(Arc::clone(&self.registry), report))
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
        tokio::spawn(async move { bus.publish_with_mode(event, metadata, dispatch_mode).await })
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
        let subscribers = self.subscribers_for(&stored);
        let handler_reports = self
            .dispatch(stored.clone(), subscribers.clone(), dispatch_mode)
            .await;
        let dead_letters = dead_letters_for(&stored, &handler_reports);
        if !dead_letters.is_empty() {
            self.dead_letters.write().await.extend(dead_letters.clone());
        }
        Ok(PublishReport {
            event_id: stored.event_id,
            event_type: stored.contract.event_type,
            dispatch_mode,
            subscriber_count: subscribers.len(),
            handled_count: handler_reports
                .iter()
                .filter(|report| report.outcome == HandlerOutcome::Handled)
                .count(),
            dead_letter_count: dead_letters.len(),
            handler_reports,
        })
    }

    pub async fn journal(&self) -> Vec<StoredEventEnvelope> {
        self.journal.read().await.clone()
    }

    pub async fn dead_letters(&self) -> Vec<DeadLetter> {
        self.dead_letters.read().await.clone()
    }

    fn insert_subscriber(&self, record: SubscriberRecord) -> Result<(), EventingError> {
        insert_subscriber(&self.registry, record)
    }

    fn subscribers_for(&self, stored: &StoredEventEnvelope) -> Vec<SubscriberRecord> {
        let registry = self.registry.lock().expect("event registry lock");
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
    ) -> Vec<HandlerReport> {
        match dispatch_mode {
            DispatchMode::Sequential => {
                dispatch_sequential(
                    stored,
                    subscribers,
                    EventPublisher::new(self.clone()),
                    self.handler_policy.clone(),
                )
                .await
            }
            DispatchMode::Concurrent => {
                dispatch_concurrent(
                    stored,
                    subscribers,
                    EventPublisher::new(self.clone()),
                    self.handler_policy.clone(),
                )
                .await
            }
            DispatchMode::OrderedByAggregateKey => {
                let aggregate_lock = self.aggregate_lock(&stored.aggregate_key);
                let _aggregate_guard = aggregate_lock.lock().await;
                dispatch_sequential(
                    stored,
                    subscribers,
                    EventPublisher::new(self.clone()),
                    self.handler_policy.clone(),
                )
                .await
            }
        }
    }

    fn aggregate_lock(&self, aggregate_key: &AggregateKey) -> Arc<AsyncMutex<()>> {
        let mut locks = self
            .aggregate_locks
            .lock()
            .expect("event aggregate lock map");
        Arc::clone(
            locks
                .entry(aggregate_key.clone())
                .or_insert_with(|| Arc::new(AsyncMutex::new(()))),
        )
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
