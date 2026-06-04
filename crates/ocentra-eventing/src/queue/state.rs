use std::{
    collections::{BTreeSet, VecDeque},
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::{
    DeadLetterReason, EventClockInstant, EventingError, IdempotencyKey, StoredEventEnvelope,
};

use super::{
    EventQueuePolicy, NoSubscriberQueuePolicy, QueueDisposition, QueueOverflowPolicy, QueueReport,
};

#[derive(Clone)]
pub(crate) struct EventQueue {
    policy: EventQueuePolicy,
    state: Arc<Mutex<EventQueueState>>,
}

impl EventQueue {
    pub(crate) fn new(policy: EventQueuePolicy) -> Self {
        Self {
            policy,
            state: Arc::new(Mutex::new(EventQueueState::default())),
        }
    }

    pub(crate) fn policy(&self) -> &EventQueuePolicy {
        &self.policy
    }

    pub(crate) fn report(&self, disposition: QueueDisposition) -> QueueReport {
        let queued_count = self.state.lock().expect("event queue lock").queued.len();
        QueueReport {
            disposition,
            queued_count,
            capacity: self.policy.capacity(),
        }
    }

    pub(crate) fn enqueue_no_subscriber(
        &self,
        stored: StoredEventEnvelope,
        now: EventClockInstant,
    ) -> Result<NoSubscriberQueueDecision, EventingError> {
        match self.policy.no_subscriber() {
            NoSubscriberQueuePolicy::DispatchWithoutSubscribers => Ok(
                NoSubscriberQueueDecision::Dispatch(self.report(QueueDisposition::Dispatched)),
            ),
            NoSubscriberQueuePolicy::DeadLetter => Ok(NoSubscriberQueueDecision::DeadLetter(
                self.report(QueueDisposition::DeadLetteredNoSubscriber),
                DeadLetterReason::NoSubscriber,
                EventingError::NoSubscriber {
                    event_type: stored.contract.event_type.as_str().to_string(),
                },
            )),
            NoSubscriberQueuePolicy::Queue => self.try_enqueue(stored, now),
        }
    }

    pub(crate) fn reserve_dispatch(
        &self,
        stored: &StoredEventEnvelope,
    ) -> Result<DispatchReservation, EventingError> {
        if !self.policy.idempotency_registry_enabled() {
            return Ok(DispatchReservation::new(self.clone(), None));
        }
        let mut state = self.state.lock().expect("event queue lock");
        let key = stored.idempotency_key.clone();
        if state.completed_keys.contains(&key) || state.queued_keys.contains(&key) {
            return Err(EventingError::DuplicateIdempotencyKey {
                idempotency_key: key.as_str().to_string(),
            });
        }
        if !state.in_flight_keys.insert(key.clone()) {
            return Err(EventingError::DuplicateInFlight {
                idempotency_key: key.as_str().to_string(),
            });
        }
        Ok(DispatchReservation::new(self.clone(), Some(key)))
    }

    pub(crate) fn take_queued(&self) -> Vec<QueuedEnvelope> {
        let mut state = self.state.lock().expect("event queue lock");
        let queued = state.queued.drain(..).collect::<Vec<_>>();
        state.queued_keys.clear();
        queued
    }

    pub(crate) fn requeue(&self, queued: QueuedEnvelope) {
        let mut state = self.state.lock().expect("event queue lock");
        state
            .queued_keys
            .insert(queued.stored.idempotency_key.clone());
        state.queued.push_back(queued);
    }

    pub(crate) fn mark_completed(&self, key: IdempotencyKey) {
        let mut state = self.state.lock().expect("event queue lock");
        state.in_flight_keys.remove(&key);
        state.completed_keys.insert(key);
    }

    pub(crate) fn release_in_flight(&self, key: &IdempotencyKey) {
        let mut state = self.state.lock().expect("event queue lock");
        state.in_flight_keys.remove(key);
    }

    fn try_enqueue(
        &self,
        stored: StoredEventEnvelope,
        now: EventClockInstant,
    ) -> Result<NoSubscriberQueueDecision, EventingError> {
        let Some(capacity) = self.policy.capacity() else {
            return Err(EventingError::InvalidQueuePolicy {
                reason: String::from("queue capacity is not configured"),
            });
        };
        let mut state = self.state.lock().expect("event queue lock");
        let key = stored.idempotency_key.clone();
        if self.policy.idempotency_registry_enabled()
            && (state.completed_keys.contains(&key)
                || state.queued_keys.contains(&key)
                || state.in_flight_keys.contains(&key))
        {
            return Err(EventingError::DuplicateIdempotencyKey {
                idempotency_key: key.as_str().to_string(),
            });
        }
        if state.queued.len() >= capacity {
            return self.overflow_decision(stored, state.queued.len(), capacity);
        }
        if self.policy.idempotency_registry_enabled() {
            state.queued_keys.insert(key);
        }
        state.queued.push_back(QueuedEnvelope {
            stored,
            enqueued_at: now,
        });
        Ok(NoSubscriberQueueDecision::Queued(QueueReport {
            disposition: QueueDisposition::QueuedNoSubscriber,
            queued_count: state.queued.len(),
            capacity: self.policy.capacity(),
        }))
    }

    fn overflow_decision(
        &self,
        stored: StoredEventEnvelope,
        queued_count: usize,
        capacity: usize,
    ) -> Result<NoSubscriberQueueDecision, EventingError> {
        match self.policy.overflow() {
            QueueOverflowPolicy::RejectPublish => Err(EventingError::QueueCapacityExceeded {
                event_type: stored.contract.event_type.as_str().to_string(),
                capacity,
            }),
            QueueOverflowPolicy::DeadLetterRejected => Ok(NoSubscriberQueueDecision::DeadLetter(
                QueueReport {
                    disposition: QueueDisposition::DeadLetteredQueueOverflow,
                    queued_count,
                    capacity: self.policy.capacity(),
                },
                DeadLetterReason::QueueOverflow,
                EventingError::QueueCapacityExceeded {
                    event_type: stored.contract.event_type.as_str().to_string(),
                    capacity,
                },
            )),
        }
    }
}

#[derive(Default)]
struct EventQueueState {
    queued: VecDeque<QueuedEnvelope>,
    queued_keys: BTreeSet<IdempotencyKey>,
    in_flight_keys: BTreeSet<IdempotencyKey>,
    completed_keys: BTreeSet<IdempotencyKey>,
}

pub(crate) struct QueuedEnvelope {
    pub(crate) stored: StoredEventEnvelope,
    enqueued_at: EventClockInstant,
}

impl QueuedEnvelope {
    pub(crate) fn is_expired(&self, now: EventClockInstant, ttl: Option<Duration>) -> bool {
        ttl.is_some_and(|ttl| now.duration_since(self.enqueued_at) >= ttl)
    }
}

pub(crate) enum NoSubscriberQueueDecision {
    Dispatch(QueueReport),
    Queued(QueueReport),
    DeadLetter(QueueReport, DeadLetterReason, EventingError),
}

pub(crate) struct DispatchReservation {
    queue: EventQueue,
    key: Option<IdempotencyKey>,
}

impl DispatchReservation {
    fn new(queue: EventQueue, key: Option<IdempotencyKey>) -> Self {
        Self { queue, key }
    }

    pub(crate) fn complete(mut self) {
        if let Some(key) = self.key.take() {
            self.queue.mark_completed(key);
        }
    }
}

impl Drop for DispatchReservation {
    fn drop(&mut self) {
        if let Some(key) = self.key.take() {
            self.queue.release_in_flight(&key);
        }
    }
}
