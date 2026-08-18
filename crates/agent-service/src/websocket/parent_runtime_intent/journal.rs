use std::path::PathBuf;

use ocentra_eventing::{
    envelope::{DomainEvent, EventEnvelope, EventMetadata, EventSource},
    error::EventingError,
    ids::{
        CorrelationId, EventCustody, EventId, RecordedAt, RuntimeInstanceId, RuntimeRole,
        SourceComponent, SourceService,
    },
    journal::{
        ndjson::{NdjsonEventJournal, NdjsonJournalOptions},
        policy::JournalDispatchPhase,
        JournalAppend,
    },
};
use ocentra_parent_agent_protocol::constants;

use crate::activity_store_path::activity_db_path;

use super::IngressEventId;

pub(super) fn event_metadata(
    command: &ocentra_parent_agent_protocol::transport::AgentCommandEnvelope,
    event_id: &IngressEventId,
) -> Result<EventMetadata, EventingError> {
    let mut correlation =
        String::from(constants::parent_controller::CORRELATION_PARENT_CHILD_RUNTIME_PREFIX);
    correlation.push_str(&command.message_id);
    Ok(EventMetadata::from_parts(
        EventId::parse(event_id.0.clone())?,
        CorrelationId::parse(correlation)?,
        EventSource::new(
            EventCustody::parse(constants::eventing_source::CUSTODY_LOCAL_JOURNAL)?,
            RuntimeRole::parse(constants::eventing_source::ROLE_CONTROLLER)?,
            SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
            SourceComponent::parse(
                constants::parent_controller::RUNTIME_COMPONENT_PARENT_CHILD_SPINE,
            )?,
            RuntimeInstanceId::parse(
                constants::parent_controller::RUNTIME_INSTANCE_LOCAL_PARENT_CONTROLLER,
            )?,
        ),
        RecordedAt::parse(command.sent_at.as_str())?,
        None,
    ))
}

pub(super) async fn persist_before_dispatch<E: DomainEvent>(
    event: &E,
    metadata: EventMetadata,
) -> Result<JournalAppend, EventingError> {
    let stored = EventEnvelope::from_event(event.clone(), metadata)?.store()?;
    let path = parent_runtime_intent_journal_path();
    if let Some(parent) = path
        .0
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
    NdjsonEventJournal::with_options(path.0, NdjsonJournalOptions::hash_chain())
        .append_phase_idempotent_by_event_id(&stored, JournalDispatchPhase::BeforeDispatch)
        .await
}

struct ParentRuntimeIntentJournalPath(PathBuf);

fn parent_runtime_intent_journal_path() -> ParentRuntimeIntentJournalPath {
    let mut path = activity_db_path().0;
    path.set_extension(constants::parent_controller::EVENTING_JOURNAL_EXTENSION);
    ParentRuntimeIntentJournalPath(path)
}
