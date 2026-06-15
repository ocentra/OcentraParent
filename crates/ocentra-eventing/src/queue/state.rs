use std::{
    collections::{BTreeSet, VecDeque},
    sync::{Arc, Mutex},
    time::Duration,
};

use crate::bus::reports::{DeadLetterReason, EventQueueMetrics};
use crate::clock::EventClockInstant;
use crate::envelope::StoredEventEnvelope;
use crate::error::EventingError;
use crate::ids::{EventId, EventType, IdempotencyKey};
use crate::sync::lock_unpoison;

use super::{
    policy::{EventQueuePolicy, NoSubscriberQueuePolicy, QueueDisposition, QueueOverflowPolicy, QueueReport},
    reservation::DispatchReservation,
};

const COMPLETED_IDEMPOTENCY_RETENTION_LIMIT: usize = 4096;

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
        let queued_count = lock_unpoison(&self.state).queued.len();
        QueueReport {
            disposition,
            queued_count,
            capacity: self.policy.capacity(),
        }
    }

    pub(crate) fn metrics(&self) -> EventQueueMetrics {
        let state = lock_unpoison(&self.state);
        EventQueueMetrics {
            queued_event_count: state.queued.len(),
            queued_event_id_count: state.queued_event_ids.len(),
            queued_idempotency_key_count: state.queued_keys.len(),
            in_flight_event_id_count: state.in_flight_event_ids.len(),
            in_flight_idempotency_key_count: state.in_flight_keys.len(),
            completed_idempotency_key_count: state.completed_keys.len(),
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
                {
                    let event_type = stored.contract.event_type;
                    EventingError::NoSubscriber { event_type }
                },
            )),
            NoSubscriberQueuePolicy::Queue => self.try_enqueue(stored, now),
        }
    }

    pub(crate) fn reserve_dispatch(
        &self,
        stored: &StoredEventEnvelope,
    ) -> Result<DispatchReservation, EventingError> {
        let mut state = lock_unpoison(&self.state);
        let event_id = stored.event_id.clone();
        if state.queued_event_ids.contains(&event_id)
            || !state.in_flight_event_ids.insert(event_id.clone())
        {
            return Err(EventingError::DuplicateEventId { event_id });
        }
        if !self.policy.idempotency_registry_enabled() {
            return Ok(DispatchReservation::new(self.clone(), Some(event_id), None));
        }
        let key = stored.idempotency_key.clone();
        if state.completed_keys.contains(&key) || state.queued_keys.contains(&key) {
            state.in_flight_event_ids.remove(&event_id);
            return Err(EventingError::DuplicateIdempotencyKey {
                idempotency_key: key,
            });
        }
        if !state.in_flight_keys.insert(key.clone()) {
            state.in_flight_event_ids.remove(&event_id);
            return Err(EventingError::DuplicateInFlight {
                idempotency_key: key,
            });
        }
        Ok(DispatchReservation::new(
            self.clone(),
            Some(event_id),
            Some(key),
        ))
    }

    pub(crate) fn queued_count(&self, event_type: Option<&EventType>) -> usize {
        let state = lock_unpoison(&self.state);
        state
            .queued
            .iter()
            .filter(|queued| {
                event_type.is_none_or(|event_type| queued.matches_event_type(event_type))
            })
            .count()
    }

    pub(crate) fn take_next_queued(
        &self,
        event_type: Option<&EventType>,
    ) -> Option<QueuedEnvelope> {
        let mut state = lock_unpoison(&self.state);
        let position = state.queued.iter().position(|queued| {
            event_type.is_none_or(|event_type| queued.matches_event_type(event_type))
        })?;
        let Some(queued) = state.queued.remove(position) else {
            unreachable!("queued position was selected from queue");
        };
        state.queued_event_ids.remove(&queued.stored.event_id);
        if self.policy.idempotency_registry_enabled() {
            state.queued_keys.remove(&queued.stored.idempotency_key);
        }
        Some(queued)
    }

    pub(crate) fn take_all_queued(&self) -> Vec<QueuedEnvelope> {
        let mut state = lock_unpoison(&self.state);
        let queued = state.queued.drain(..).collect();
        state.queued_event_ids.clear();
        state.queued_keys.clear();
        queued
    }

    pub(crate) fn requeue(&self, queued: QueuedEnvelope) {
        let mut state = lock_unpoison(&self.state);
        state
            .queued_event_ids
            .insert(queued.stored.event_id.clone());
        if self.policy.idempotency_registry_enabled() {
            state
                .queued_keys
                .insert(queued.stored.idempotency_key.clone());
        }
        state.queued.push_back(queued);
    }

    pub(crate) fn mark_completed(&self, event_id: &EventId, key: IdempotencyKey) {
        let mut state = lock_unpoison(&self.state);
        state.in_flight_event_ids.remove(event_id);
        state.in_flight_keys.remove(&key);
        if self.policy.idempotency_registry_enabled() && state.completed_keys.insert(key.clone()) {
            state.completed_key_order.push_back(key);
            trim_completed_keys(&mut state);
        }
    }

    pub(crate) fn release_in_flight(&self, event_id: &EventId, key: Option<&IdempotencyKey>) {
        let mut state = lock_unpoison(&self.state);
        state.in_flight_event_ids.remove(event_id);
        if let Some(key) = key {
            state.in_flight_keys.remove(key);
        }
    }

    pub(crate) fn clear_for_test(&self) -> EventQueueClearReport {
        let mut state = lock_unpoison(&self.state);
        let report = EventQueueClearReport {
            queued_event_count: state.queued.len(),
            queued_idempotency_key_count: state.queued_keys.len(),
            in_flight_idempotency_key_count: state.in_flight_keys.len(),
            completed_idempotency_key_count: state.completed_keys.len(),
        };
        state.queued.clear();
        state.queued_event_ids.clear();
        state.queued_keys.clear();
        state.in_flight_event_ids.clear();
        state.in_flight_keys.clear();
        state.completed_keys.clear();
        state.completed_key_order.clear();
        report
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
        let mut state = lock_unpoison(&self.state);
        let event_id = stored.event_id.clone();
        if state.queued_event_ids.contains(&event_id)
            || state.in_flight_event_ids.contains(&event_id)
        {
            return Err(EventingError::DuplicateEventId { event_id });
        }
        let key = stored.idempotency_key.clone();
        if self.policy.idempotency_registry_enabled()
            && (state.completed_keys.contains(&key)
                || state.queued_keys.contains(&key)
                || state.in_flight_keys.contains(&key))
        {
            return Err(EventingError::DuplicateIdempotencyKey {
                idempotency_key: key,
            });
        }
        if state.queued.len() >= capacity {
            return self.overflow_decision(stored, &mut state, capacity, now);
        }
        state.queued_event_ids.insert(stored.event_id.clone());
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
        state: &mut EventQueueState,
        capacity: usize,
        now: EventClockInstant,
    ) -> Result<NoSubscriberQueueDecision, EventingError> {
        match self.policy.overflow() {
            QueueOverflowPolicy::RejectPublish => {
                let event_type = stored.contract.event_type;
                Err(EventingError::QueueCapacityExceeded {
                    event_type,
                    capacity,
                })
            }
            QueueOverflowPolicy::DeadLetterRejected => Ok(NoSubscriberQueueDecision::DeadLetter(
                QueueReport {
                    disposition: QueueDisposition::DeadLetteredQueueOverflow,
                    queued_count: state.queued.len(),
                    capacity: self.policy.capacity(),
                },
                DeadLetterReason::QueueOverflow,
                {
                    let event_type = stored.contract.event_type;
                    EventingError::QueueCapacityExceeded {
                        event_type,
                        capacity,
                    }
                },
            )),
            QueueOverflowPolicy::DropOldestAndDeadLetter => {
                let Some(dropped) = state.queued.pop_front() else {
                    return Err(EventingError::InvalidQueuePolicy {
                        reason: String::from("drop-oldest overflow requires a queued event"),
                    });
                };
                state.queued_event_ids.remove(&dropped.stored.event_id);
                state.queued_keys.remove(&dropped.stored.idempotency_key);
                state.queued_event_ids.insert(stored.event_id.clone());
                if self.policy.idempotency_registry_enabled() {
                    state.queued_keys.insert(stored.idempotency_key.clone());
                }
                let dropped_event_type = dropped.stored.contract.event_type.clone();
                state.queued.push_back(QueuedEnvelope {
                    stored,
                    enqueued_at: now,
                });
                Ok(NoSubscriberQueueDecision::QueuedWithDeadLetter(
                    QueueReport {
                        disposition: QueueDisposition::DeadLetteredQueueOverflow,
                        queued_count: state.queued.len(),
                        capacity: self.policy.capacity(),
                    },
                    Box::new(dropped.stored),
                    DeadLetterReason::QueueOverflow,
                    EventingError::QueueCapacityExceeded {
                        event_type: dropped_event_type,
                        capacity,
                    },
                ))
            }
        }
    }
}

#[derive(Default)]
struct EventQueueState {
    queued: VecDeque<QueuedEnvelope>,
    queued_event_ids: BTreeSet<EventId>,
    in_flight_event_ids: BTreeSet<EventId>,
    queued_keys: BTreeSet<IdempotencyKey>,
    in_flight_keys: BTreeSet<IdempotencyKey>,
    completed_keys: BTreeSet<IdempotencyKey>,
    completed_key_order: VecDeque<IdempotencyKey>,
}

#[derive(Clone)]
pub(crate) struct QueuedEnvelope {
    pub(crate) stored: StoredEventEnvelope,
    enqueued_at: EventClockInstant,
}

impl QueuedEnvelope {
    pub(crate) fn is_expired(&self, now: EventClockInstant, ttl: Option<Duration>) -> bool {
        ttl.is_some_and(|ttl| now.duration_since(self.enqueued_at) >= ttl)
    }

    fn matches_event_type(&self, event_type: &EventType) -> bool {
        &self.stored.contract.event_type == event_type
    }
}

pub(crate) enum NoSubscriberQueueDecision {
    Dispatch(QueueReport),
    Queued(QueueReport),
    QueuedWithDeadLetter(
        QueueReport,
        Box<StoredEventEnvelope>,
        DeadLetterReason,
        EventingError,
    ),
    DeadLetter(QueueReport, DeadLetterReason, EventingError),
}

pub(crate) struct EventQueueClearReport {
    pub(crate) queued_event_count: usize,
    pub(crate) queued_idempotency_key_count: usize,
    pub(crate) in_flight_idempotency_key_count: usize,
    pub(crate) completed_idempotency_key_count: usize,
}

fn trim_completed_keys(state: &mut EventQueueState) {
    while state.completed_key_order.len() > COMPLETED_IDEMPOTENCY_RETENTION_LIMIT {
        if let Some(expired) = state.completed_key_order.pop_front() {
            state.completed_keys.remove(&expired);
        }
    }
}
