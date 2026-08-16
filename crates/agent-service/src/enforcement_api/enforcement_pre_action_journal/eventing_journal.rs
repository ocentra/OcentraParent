use std::path::PathBuf;

use ocentra_eventing::{
    envelope::{EventEnvelope, EventMetadata, EventSource},
    error::EventingError,
    ids::{
        CorrelationId, EventCustody, EventId, RecordedAt, RuntimeInstanceId, RuntimeRole,
        SourceComponent, SourceService,
    },
    journal::policy::JournalDispatchPhase,
    journal::{ndjson::NdjsonEventJournal, ndjson::NdjsonJournalOptions, JournalAppend},
};
use ocentra_parent_agent_protocol::{
    constants::enforcement, enforcement::EnforcementAuditJournalEvent,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct EnforcementEventingJournalPath {
    pub(crate) path: PathBuf,
}

pub(crate) async fn append_enforcement_audit_journal_event_phase(
    journal_path: EnforcementEventingJournalPath,
    event: EnforcementAuditJournalEvent,
    correlation_id: CorrelationId,
    phase: JournalDispatchPhase,
) -> Result<JournalAppend, EventingError> {
    let path = journal_path.path;
    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
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
    match phase {
        JournalDispatchPhase::BeforeDispatch | JournalDispatchPhase::AfterDispatch => {
            journal
                .append_phase_idempotent_by_event_id(&stored, phase)
                .await
        }
    }
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
