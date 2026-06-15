use std::sync::Arc;

use tokio::task::JoinHandle;

use crate::envelope::{DomainEvent, EventEnvelope, EventMetadata, StoredEventEnvelope};
use crate::error::EventingError;
use crate::ids::RequestId;
use crate::journal::policy::JournalDispatchPhase;
use crate::queue::policy::QueueDisposition;
use crate::queue::state::NoSubscriberQueueDecision;
use crate::request::{RequestCompletionReport, RequestEvent, RequestOptions, RequestReport};
use crate::sync::lock_unpoison;

use super::{
    dispatch::{dispatch_concurrent, dispatch_sequential},
    reports::{dead_letters_for, empty_publish_report, DeadLetter, DeadLetterReason},
    DispatchMode, EventBus, EventPublisher, HandlerOutcome, HandlerReport, PublishReport,
    SubscriberRecord,
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

    pub async fn publish_request<E>(
        &self,
        event: E,
        metadata: EventMetadata,
        options: RequestOptions,
    ) -> Result<RequestReport<E::Response>, EventingError>
    where
        E: RequestEvent,
    {
        self.ensure_active()?;
        let request_id = event.request_id()?;
        let receiver = self.requests.register(request_id.clone())?;
        let bus = self.clone();
        let mut publish = tokio::spawn(async move { bus.publish(event, metadata).await });
        let timeout = self.clock.sleep(options.timeout());
        tokio::pin!(timeout);
        tokio::pin!(receiver);
        let mut publish_done = false;
        let mut receiver_done = false;
        let mut publish_report = None;
        let mut response_payload = None;
        loop {
            tokio::select! {
                result = &mut publish, if !publish_done => {
                    publish_done = true;
                    match request_publish_result(result) {
                        Ok(report) => publish_report = Some(report),
                        Err(error) => {
                            self.requests.cancel(&request_id);
                            return Err(error);
                        }
                    }
                }
                payload = &mut receiver, if !receiver_done => {
                    receiver_done = true;
                    response_payload = payload.ok();
                    if response_payload.is_none() {
                        self.requests.timeout(&request_id);
                        return Err(EventingError::RequestTimedOut { request_id });
                    }
                }
                _ = &mut timeout => {
                    self.requests.timeout(&request_id);
                    abort_request_publish(&mut publish, publish_done).await;
                    return Err(EventingError::RequestTimedOut { request_id });
                }
            }
            if publish_report.is_some() && response_payload.is_some() {
                let Some(publish_report) = publish_report.take() else {
                    unreachable!("publish report was checked as present");
                };
                let Some(payload) = response_payload.take() else {
                    unreachable!("response payload was checked as present");
                };
                let response = payload.decode::<E::Response>(&request_id)?;
                return Ok(RequestReport {
                    request_id,
                    response,
                    publish_report,
                });
            }
        }
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
        self.ensure_active()?;
        let stored = EventEnvelope::from_event(event, metadata)?.store()?;
        if stored.is_deadline_expired(self.clock.now()) {
            return self
                .dead_letter_expired_deadline(stored, dispatch_mode)
                .await;
        }
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
        self.requests.complete(request_id, response)
    }

    async fn publish_without_subscribers(
        &self,
        stored: StoredEventEnvelope,
        dispatch_mode: DispatchMode,
    ) -> Result<PublishReport, EventingError> {
        match self
            .queue
            .enqueue_no_subscriber(stored.clone(), self.clock.now())?
        {
            NoSubscriberQueueDecision::Dispatch(queue_report) => {
                self.record_stored_snapshot(&stored).await;
                Ok(empty_publish_report(
                    &stored,
                    dispatch_mode,
                    queue_report,
                    0,
                ))
            }
            NoSubscriberQueueDecision::Queued(queue_report) => {
                self.record_stored_snapshot(&stored).await;
                Ok(empty_publish_report(
                    &stored,
                    dispatch_mode,
                    queue_report,
                    0,
                ))
            }
            NoSubscriberQueueDecision::QueuedWithDeadLetter(
                queue_report,
                dropped,
                reason,
                error,
            ) => {
                let dropped = *dropped;
                self.record_stored_snapshot(&stored).await;
                let dead_letter = DeadLetter::for_queue(&dropped, reason, error);
                self.queue
                    .mark_completed(&dropped.event_id, dropped.idempotency_key.clone());
                self.record_dead_letter(dead_letter).await;
                Ok(empty_publish_report(
                    &stored,
                    dispatch_mode,
                    queue_report,
                    1,
                ))
            }
            NoSubscriberQueueDecision::DeadLetter(queue_report, reason, error) => {
                self.record_stored_snapshot(&stored).await;
                let dead_letter = DeadLetter::for_queue(&stored, reason, error);
                self.queue
                    .mark_completed(&stored.event_id, stored.idempotency_key.clone());
                self.record_dead_letter(dead_letter).await;
                Ok(empty_publish_report(
                    &stored,
                    dispatch_mode,
                    queue_report,
                    1,
                ))
            }
        }
    }

    async fn dead_letter_expired_deadline(
        &self,
        stored: StoredEventEnvelope,
        dispatch_mode: DispatchMode,
    ) -> Result<PublishReport, EventingError> {
        self.record_stored_snapshot(&stored).await;
        let dead_letter = DeadLetter::for_queue(
            &stored,
            DeadLetterReason::DeadlineExpired,
            EventingError::EventDeadlineExpired {
                event_type: stored.contract.event_type.clone(),
            },
        );
        self.queue
            .mark_completed(&stored.event_id, stored.idempotency_key.clone());
        self.record_dead_letter(dead_letter).await;
        Ok(empty_publish_report(
            &stored,
            dispatch_mode,
            self.queue
                .report(QueueDisposition::DeadLetteredDeadlineExpired),
            1,
        ))
    }

    pub(super) async fn dispatch_stored(
        &self,
        stored: StoredEventEnvelope,
        subscribers: Vec<SubscriberRecord>,
        dispatch_mode: DispatchMode,
        queue_report: crate::queue::policy::QueueReport,
        write_journal: bool,
    ) -> Result<PublishReport, EventingError> {
        self.dispatch_stored_checked(
            stored,
            subscribers,
            dispatch_mode,
            queue_report,
            write_journal,
        )
        .await
        .map_err(DispatchStoredError::into_error)
    }

    pub(super) async fn dispatch_stored_checked(
        &self,
        stored: StoredEventEnvelope,
        subscribers: Vec<SubscriberRecord>,
        dispatch_mode: DispatchMode,
        queue_report: crate::queue::policy::QueueReport,
        write_journal: bool,
    ) -> Result<PublishReport, DispatchStoredError> {
        let reservation = self.queue.reserve_dispatch(&stored)?;
        let _active_dispatch = self.active_dispatches.enter();
        if write_journal {
            self.record_stored_snapshot(&stored).await;
        }
        self.append_journal_phase(&stored, JournalDispatchPhase::BeforeDispatch)
            .await
            .map_err(DispatchStoredError::BeforeDispatch)?;
        let handler_reports = self
            .dispatch(stored.clone(), subscribers.clone(), dispatch_mode)
            .await;
        reservation.complete();
        let dead_letters = dead_letters_for(&stored, &handler_reports);
        if !dead_letters.is_empty() {
            self.record_dead_letters(dead_letters.clone()).await;
        }
        self.append_journal_phase(&stored, JournalDispatchPhase::AfterDispatch)
            .await
            .map_err(DispatchStoredError::AfterDispatch)?;
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

    pub(super) fn subscribers_for(&self, stored: &StoredEventEnvelope) -> Vec<SubscriberRecord> {
        let registry = lock_unpoison(&self.registry);
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
                    Arc::clone(&self.clock),
                )
                .await
            }
            DispatchMode::Concurrent => {
                dispatch_concurrent(
                    stored,
                    subscribers,
                    EventPublisher::new(self.clone()),
                    self.handler_policy.clone(),
                    Arc::clone(&self.clock),
                )
                .await
            }
            DispatchMode::OrderedByAggregateKey => {
                let aggregate_key = stored.aggregate_key.clone();
                let aggregate_gate = self.aggregate_gate(&aggregate_key);
                let Ok(aggregate_permit) = Arc::clone(&aggregate_gate).acquire_owned().await else {
                    unreachable!("aggregate ordering gate remains open");
                };
                let reports = dispatch_sequential(
                    stored,
                    subscribers,
                    EventPublisher::new(self.clone()),
                    self.handler_policy.clone(),
                    Arc::clone(&self.clock),
                )
                .await;
                drop(aggregate_permit);
                self.release_idle_aggregate_gate(&aggregate_key, &aggregate_gate);
                reports
            }
        }
    }
}

pub(super) enum DispatchStoredError {
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

fn request_publish_result(
    result: Result<Result<PublishReport, EventingError>, tokio::task::JoinError>,
) -> Result<PublishReport, EventingError> {
    result
        .map_err(|error| EventingError::invalid_value("request_publish_task", error.to_string()))?
}

async fn abort_request_publish(
    publish: &mut JoinHandle<Result<PublishReport, EventingError>>,
    publish_done: bool,
) {
    if publish_done {
        return;
    }
    publish.abort();
    let _ = publish.await;
}
