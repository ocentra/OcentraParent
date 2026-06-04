use std::sync::Arc;

use tokio::{sync::Mutex as AsyncMutex, task::JoinHandle};

use crate::{
    queue::NoSubscriberQueueDecision, AggregateKey, DomainEvent, EventEnvelope, EventMetadata,
    EventingError, QueueDisposition, StoredEventEnvelope,
};

use super::{
    dispatch::{dispatch_concurrent, dispatch_sequential},
    reports::{dead_letters_for, empty_publish_report, DeadLetter, DeadLetterReason},
    DispatchMode, EventBus, EventPublisher, HandlerOutcome, HandlerReport, PublishReport,
    QueueDrainReport, SubscriberRecord,
};

impl EventBus {
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
        let subscribers = self.subscribers_for(&stored);
        if subscribers.is_empty() {
            return self
                .publish_without_subscribers(stored, dispatch_mode)
                .await;
        }
        self.dispatch_stored(
            stored,
            subscribers,
            dispatch_mode,
            self.queue.report(QueueDisposition::Dispatched),
            true,
        )
        .await
    }

    pub async fn drain_queued(
        &self,
        dispatch_mode: DispatchMode,
    ) -> Result<QueueDrainReport, EventingError> {
        let queued = self.queue.take_queued();
        let queued_before = queued.len();
        let mut expired_count = 0_usize;
        let mut dispatch_reports = Vec::new();

        for queued_envelope in queued {
            if queued_envelope.is_expired(self.queue.policy().ttl()) {
                expired_count += 1;
                let dead_letter = DeadLetter::for_queue(
                    &queued_envelope.stored,
                    DeadLetterReason::QueueExpired,
                    EventingError::NoSubscriber {
                        event_type: queued_envelope
                            .stored
                            .contract
                            .event_type
                            .as_str()
                            .to_string(),
                    },
                );
                self.queue
                    .mark_completed(queued_envelope.stored.idempotency_key.clone());
                self.dead_letters.write().await.push(dead_letter);
                continue;
            }

            let subscribers = self.subscribers_for(&queued_envelope.stored);
            if subscribers.is_empty() {
                self.queue.requeue(queued_envelope);
                continue;
            }

            let report = self
                .dispatch_stored(
                    queued_envelope.stored,
                    subscribers,
                    dispatch_mode,
                    self.queue.report(QueueDisposition::Dispatched),
                    false,
                )
                .await?;
            dispatch_reports.push(report);
        }

        Ok(QueueDrainReport {
            queued_before,
            dispatched_count: dispatch_reports.len(),
            expired_count,
            remaining_count: self.queue.report(QueueDisposition::Dispatched).queued_count,
            dispatch_reports,
        })
    }

    pub async fn journal(&self) -> Vec<StoredEventEnvelope> {
        self.journal.read().await.clone()
    }

    pub async fn dead_letters(&self) -> Vec<DeadLetter> {
        self.dead_letters.read().await.clone()
    }

    async fn publish_without_subscribers(
        &self,
        stored: StoredEventEnvelope,
        dispatch_mode: DispatchMode,
    ) -> Result<PublishReport, EventingError> {
        match self.queue.enqueue_no_subscriber(stored.clone())? {
            NoSubscriberQueueDecision::Dispatch(queue_report)
            | NoSubscriberQueueDecision::Queued(queue_report) => {
                self.journal.write().await.push(stored.clone());
                Ok(empty_publish_report(
                    &stored,
                    dispatch_mode,
                    queue_report,
                    0,
                ))
            }
            NoSubscriberQueueDecision::DeadLetter(queue_report, reason, error) => {
                self.journal.write().await.push(stored.clone());
                let dead_letter = DeadLetter::for_queue(&stored, reason, error);
                self.queue.mark_completed(stored.idempotency_key.clone());
                self.dead_letters.write().await.push(dead_letter);
                Ok(empty_publish_report(
                    &stored,
                    dispatch_mode,
                    queue_report,
                    1,
                ))
            }
        }
    }

    async fn dispatch_stored(
        &self,
        stored: StoredEventEnvelope,
        subscribers: Vec<SubscriberRecord>,
        dispatch_mode: DispatchMode,
        queue_report: crate::QueueReport,
        write_journal: bool,
    ) -> Result<PublishReport, EventingError> {
        let reservation = self.queue.reserve_dispatch(&stored)?;
        if write_journal {
            self.journal.write().await.push(stored.clone());
        }
        let handler_reports = self
            .dispatch(stored.clone(), subscribers.clone(), dispatch_mode)
            .await;
        reservation.complete();
        let dead_letters = dead_letters_for(&stored, &handler_reports);
        if !dead_letters.is_empty() {
            self.dead_letters.write().await.extend(dead_letters.clone());
        }
        Ok(PublishReport {
            event_id: stored.event_id,
            event_type: stored.contract.event_type,
            dispatch_mode,
            queue_report,
            subscriber_count: subscribers.len(),
            handled_count: handler_reports
                .iter()
                .filter(|report| report.outcome == HandlerOutcome::Handled)
                .count(),
            dead_letter_count: dead_letters.len(),
            handler_reports,
        })
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
