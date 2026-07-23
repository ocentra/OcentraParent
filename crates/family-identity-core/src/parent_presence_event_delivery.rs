use std::path::{Path, PathBuf};
use std::thread;

use ocentra_eventing::envelope::{
    DomainEvent, EventContract, EventEnvelope, EventMetadata, EventSource, StoredEventEnvelope,
};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    AggregateKey, EventCustody, EventType, IdempotencyKey, RecordedAt, RuntimeInstanceId,
    RuntimeRole, SchemaVersion, SourceComponent, SourceService,
};
use ocentra_eventing::journal::ndjson::{NdjsonEventJournal, NdjsonJournalOptions};

use crate::parent_presence::{
    ParentPresenceCustodyDecisionArtifact, ParentPresenceObservedAt,
    ParentPresenceStorageFailureReason,
};

const EVENT_TYPE: &str = "family-identity.parent-presence-custody-decision";
const AGGREGATE_KEY: &str = "family-identity.parent-presence-custody";
const EVENT_CUSTODY: &str = "family-identity-core";
const RUNTIME_ROLE: &str = "parent";
const SOURCE_SERVICE: &str = "parent-presence";
const SOURCE_COMPONENT: &str = "family-identity-core";
const RUNTIME_INSTANCE: &str = "parent-presence-custody";
const JOURNAL_SUFFIX: &str = ".custody-decisions.ndjson";

pub(crate) struct PendingCustodyDecision {
    pub(crate) decision_id: String,
    pub(crate) envelope_json: String,
}

pub(crate) struct ParentPresenceDecisionDelivery {
    journal: NdjsonEventJournal,
    journal_path: PathBuf,
}

impl ParentPresenceDecisionDelivery {
    pub(crate) fn for_store_path(store_path: &Path) -> Self {
        let journal_path = journal_path_for_store(store_path);
        let journal =
            NdjsonEventJournal::with_options(&journal_path, NdjsonJournalOptions::hash_chain());
        Self {
            journal,
            journal_path,
        }
    }

    pub(crate) fn prepare(
        &self,
        artifact: &ParentPresenceCustodyDecisionArtifact,
        observed_at: &ParentPresenceObservedAt,
    ) -> Result<PendingCustodyDecision, ParentPresenceStorageFailureReason> {
        let envelope = stored_envelope(artifact, observed_at)?;
        let envelope_json = serde_json::to_string(&envelope)
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
        Ok(PendingCustodyDecision {
            decision_id: artifact.decision_id.as_str().to_owned(),
            envelope_json,
        })
    }

    pub(crate) fn append_pending(
        &self,
        pending: &PendingCustodyDecision,
    ) -> Result<(), ParentPresenceStorageFailureReason> {
        let envelope = serde_json::from_str::<StoredEventEnvelope>(&pending.envelope_json)
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
        if envelope.event_id.as_str() != pending.decision_id {
            return Err(ParentPresenceStorageFailureReason::CustodyUnavailable);
        }
        append_on_isolated_runtime(self.journal.clone(), envelope)
    }

    #[cfg(debug_assertions)]
    pub(crate) fn journal_path(&self) -> &Path {
        &self.journal_path
    }

    #[cfg(debug_assertions)]
    pub(crate) fn inject_next_sync_failure_for_debug(&self) {
        self.journal.inject_next_sync_failure_for_debug();
    }

    #[cfg(debug_assertions)]
    pub(crate) fn inject_next_partial_write_failure_for_debug(&self) {
        self.journal.inject_next_partial_write_failure_for_debug();
    }

    #[cfg(debug_assertions)]
    pub(crate) fn inject_next_directory_sync_failure_for_debug(&self) {
        self.journal.inject_next_directory_sync_failure_for_debug();
    }
}

fn journal_path_for_store(store_path: &Path) -> PathBuf {
    let mut path = store_path.as_os_str().to_os_string();
    path.push(JOURNAL_SUFFIX);
    PathBuf::from(path)
}

impl DomainEvent for ParentPresenceCustodyDecisionArtifact {
    fn contract(&self) -> Result<EventContract, EventingError> {
        Ok(EventContract::new(
            EventType::parse(EVENT_TYPE)?,
            SchemaVersion::new(1)?,
        ))
    }

    fn aggregate_key(&self) -> Result<AggregateKey, EventingError> {
        AggregateKey::parse(AGGREGATE_KEY)
    }

    fn idempotency_key(&self) -> Result<IdempotencyKey, EventingError> {
        IdempotencyKey::parse(self.decision_id.as_str())
    }
}

fn stored_envelope(
    artifact: &ParentPresenceCustodyDecisionArtifact,
    observed_at: &ParentPresenceObservedAt,
) -> Result<StoredEventEnvelope, ParentPresenceStorageFailureReason> {
    let metadata = EventMetadata::from_parts(
        artifact.decision_id.clone(),
        artifact.correlation_id.clone(),
        event_source()?,
        RecordedAt::parse(observed_at.canonical.clone()).map_err(eventing_unavailable)?,
        None,
    );
    EventEnvelope::from_event(artifact.clone(), metadata)
        .and_then(|envelope| envelope.store())
        .map_err(eventing_unavailable)
}

fn event_source() -> Result<EventSource, ParentPresenceStorageFailureReason> {
    Ok(EventSource::new(
        EventCustody::parse(EVENT_CUSTODY).map_err(eventing_unavailable)?,
        RuntimeRole::parse(RUNTIME_ROLE).map_err(eventing_unavailable)?,
        SourceService::parse(SOURCE_SERVICE).map_err(eventing_unavailable)?,
        SourceComponent::parse(SOURCE_COMPONENT).map_err(eventing_unavailable)?,
        RuntimeInstanceId::parse(RUNTIME_INSTANCE).map_err(eventing_unavailable)?,
    ))
}

fn append_on_isolated_runtime(
    journal: NdjsonEventJournal,
    envelope: StoredEventEnvelope,
) -> Result<(), ParentPresenceStorageFailureReason> {
    thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .map_err(|_error| ParentPresenceStorageFailureReason::CustodyUnavailable)?;
        runtime
            .block_on(journal.append_idempotent(&envelope))
            .map(|_append| ())
            .map_err(eventing_unavailable)
    })
    .join()
    .map_err(|_panic| ParentPresenceStorageFailureReason::CustodyUnavailable)?
}

fn eventing_unavailable(_error: EventingError) -> ParentPresenceStorageFailureReason {
    ParentPresenceStorageFailureReason::CustodyUnavailable
}
