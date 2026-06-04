use std::{
    collections::BTreeMap,
    future::Future,
    sync::{Arc, Mutex},
};

use tokio::{sync::Mutex as AsyncMutex, sync::RwLock};

use crate::{
    queue::EventQueue, AggregateKey, DomainEvent, EventQueuePolicy, EventType, EventingError,
    HandlerExecutionPolicy, StoredEventEnvelope,
};

mod dispatch;
mod publish;
mod publisher;
mod reports;
mod subscriber;

use subscriber::{insert_subscriber, record_for, SubscriberRecord};

pub use publisher::{EventContext, EventPublisher};
pub use reports::{
    DeadLetter, DeadLetterEvent, DeadLetterReason, EventTraceFields, HandlerOutcome, HandlerReport,
    PublishReport, QueueDrainReport, DEAD_LETTER_RECORDED_EVENT_TYPE,
};
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
    queue: EventQueue,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            registry: Arc::new(Mutex::new(BTreeMap::new())),
            journal: Arc::new(RwLock::new(Vec::new())),
            dead_letters: Arc::new(RwLock::new(Vec::new())),
            aggregate_locks: Arc::new(Mutex::new(BTreeMap::new())),
            handler_policy: HandlerExecutionPolicy::default(),
            queue: EventQueue::new(EventQueuePolicy::default()),
        }
    }

    pub fn with_handler_policy(policy: HandlerExecutionPolicy) -> Self {
        Self {
            handler_policy: policy,
            ..Self::new()
        }
    }

    pub fn with_queue_policy(policy: EventQueuePolicy) -> Self {
        Self {
            queue: EventQueue::new(policy),
            ..Self::new()
        }
    }

    pub fn with_policies(
        handler_policy: HandlerExecutionPolicy,
        queue_policy: EventQueuePolicy,
    ) -> Self {
        Self {
            handler_policy,
            queue: EventQueue::new(queue_policy),
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

    fn insert_subscriber(&self, record: SubscriberRecord) -> Result<(), EventingError> {
        insert_subscriber(&self.registry, record)
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
