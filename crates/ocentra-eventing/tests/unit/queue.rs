use crate::ExpectValue;
use std::{sync::Arc, time::Duration};

use std::sync::atomic::{AtomicUsize, Ordering};
use std::{future::Future, pin::Pin, sync::Mutex as StdMutex};
use tokio::sync::{Mutex, Notify};

use super::fixtures::{
    metadata, metadata_with_event_id, subscriber, subscriber_for_event, test_event,
    test_event_for_type_with_aggregate_and_idempotency, test_event_with_idempotency, TestEvent,
    TestText, OTHER_EVENT_TYPE, OTHER_SUBSCRIBER, OTHER_TARGET, TEST_LABEL, TEST_SUBSCRIBER,
    TEST_TARGET,
};
use crate::{
    DispatchMode, DomainEvent, EventBus, EventClock, EventJournal, EventQueuePolicy, EventingError,
    HandlerExecutionPolicy, JournalAppend, JournalPolicy, JournalSelector, ManualEventClock,
    QueueDisposition, QueueOverflowPolicy, StoredEventEnvelope,
};
use ocentra_eventing::bus::reports::dead_letter::{DeadLetterReason, DeadLetterRetryState};
use ocentra_eventing::journal::policy::JournalDispatchPhase;
use ocentra_eventing::journal::{JournalAppendDurability, JournalHashVersion};
use ocentra_eventing::queue::policy::NoSubscriberQueuePolicy;

#[path = "queue/dead_letter.rs"]
mod dead_letter;
#[path = "queue/dead_letter_identity.rs"]
mod dead_letter_identity;
#[path = "queue/no_subscriber.rs"]
mod no_subscriber;
#[path = "queue/no_subscriber_journal.rs"]
mod no_subscriber_journal;
#[path = "queue/overflow.rs"]
mod overflow;

fn failing_journal_result(
    call: usize,
    fail_once_on: usize,
    hash_version: JournalHashVersion,
) -> Result<JournalAppend, EventingError> {
    if call == fail_once_on {
        return Err(EventingError::JournalIo {
            path: String::from("failing-journal"),
            reason: String::from("intentional one-shot append failure"),
        });
    }

    Ok(JournalAppend {
        sequence: call as u64,
        previous_hash: None,
        current_hash: Some(
            crate::JournalHash::parse(format!("journal-hash-{call}"))
                .expect_value("journal hash parses"),
        ),
        hash_version,
        durability: JournalAppendDurability::Synchronized,
        requested_durability: JournalAppendDurability::Synchronized,
        synchronization_hash: None,
    })
}

fn require_verified_v3_receipt(append: &JournalAppend) -> Result<(), EventingError> {
    if append.has_verified_synchronization_proof() {
        return Ok(());
    }
    Err(EventingError::InvalidHandlerPolicy {
        reason: "test requires a verified V3 synchronization receipt".to_owned(),
    })
}

struct FailingJournal {
    calls: StdMutex<usize>,
    phases: StdMutex<Vec<JournalDispatchPhase>>,
    fail_once_on: usize,
    hash_version: JournalHashVersion,
}

impl FailingJournal {
    fn fail_once_on(call: usize) -> Self {
        Self {
            calls: StdMutex::new(0),
            phases: StdMutex::new(Vec::new()),
            fail_once_on: call,
            hash_version: JournalHashVersion::V2,
        }
    }

    fn with_invalid_v3_receipt() -> Self {
        Self {
            calls: StdMutex::new(0),
            phases: StdMutex::new(Vec::new()),
            fail_once_on: usize::MAX,
            hash_version: JournalHashVersion::V3,
        }
    }

    fn calls(&self) -> usize {
        *self.calls.lock().expect_value("failing journal lock")
    }

    fn phases(&self) -> Vec<JournalDispatchPhase> {
        self.phases
            .lock()
            .expect_value("failing journal phase lock")
            .clone()
    }
}

impl EventJournal for FailingJournal {
    fn append<'a>(
        &'a self,
        _envelope: &'a StoredEventEnvelope,
    ) -> Pin<Box<dyn Future<Output = Result<JournalAppend, EventingError>> + Send + 'a>> {
        Box::pin(async move {
            let call = {
                let mut calls = self.calls.lock().expect_value("failing journal lock");
                *calls += 1;
                *calls
            };
            failing_journal_result(call, self.fail_once_on, self.hash_version)
        })
    }

    fn append_phase<'a>(
        &'a self,
        envelope: &'a StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> Pin<Box<dyn Future<Output = Result<JournalAppend, EventingError>> + Send + 'a>> {
        self.phases
            .lock()
            .expect_value("failing journal phase lock")
            .push(phase);
        self.append(envelope)
    }

    fn append_phase_idempotent<'a>(
        &'a self,
        envelope: &'a StoredEventEnvelope,
        phase: JournalDispatchPhase,
    ) -> Pin<Box<dyn Future<Output = Result<JournalAppend, EventingError>> + Send + 'a>> {
        self.append_phase(envelope, phase)
    }
}
