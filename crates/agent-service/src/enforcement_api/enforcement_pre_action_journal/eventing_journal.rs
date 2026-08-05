use std::path::PathBuf;

use ocentra_eventing::{
    envelope::{EventEnvelope, EventMetadata, EventSource},
    error::EventingError,
    ids::{
        CorrelationId, EventCustody, EventId, RecordedAt, RuntimeInstanceId, RuntimeRole,
        SourceComponent, SourceService,
    },
    journal::{ndjson::NdjsonEventJournal, ndjson::NdjsonJournalOptions, JournalAppend},
};
use ocentra_parent_agent_protocol::{
    constants::enforcement, enforcement::EnforcementAuditJournalEvent,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementEventingJournalPath {
    pub(crate) path: PathBuf,
}

pub(crate) async fn append_enforcement_audit_journal_event(
    journal_path: EnforcementEventingJournalPath,
    event: EnforcementAuditJournalEvent,
    correlation_id: CorrelationId,
) -> Result<JournalAppend, EventingError> {
    let path = journal_path.path;
    if let Some(parent) = path.parent() {
        tokio::fs::create_dir_all(parent)
            .await
            .map_err(|error| EventingError::JournalIo {
                path: parent.display().to_string(),
                reason: error.to_string(),
            })?;
    }
    let journal = NdjsonEventJournal::with_options(path, NdjsonJournalOptions::hash_chain());
    let metadata = EventMetadata::from_parts(
        EventId::parse(event.audit_event_id.clone())?,
        correlation_id,
        event_source()?,
        RecordedAt::parse(event.observed_at.clone())?,
        None,
    );
    let stored = EventEnvelope::from_event(event, metadata)?.store()?;
    journal.append_idempotent(&stored).await
}

fn event_source() -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(enforcement::EVENTING_CUSTODY_LOCAL_AUDIT)?,
        RuntimeRole::parse(enforcement::EVENTING_RUNTIME_ROLE_AGENT)?,
        SourceService::parse(enforcement::SOURCE_ID_AGENT_SERVICE)?,
        SourceComponent::parse(enforcement::EVENTING_SOURCE_COMPONENT)?,
        RuntimeInstanceId::parse(enforcement::EVENTING_RUNTIME_INSTANCE)?,
    ))
}
