use ocentra_eventing::{
    envelope::EventMetadata, envelope::EventSource, error::EventingError, ids::CorrelationId,
    ids::EventCustody, ids::EventId, ids::RecordedAt, ids::RuntimeInstanceId, ids::SourceComponent,
    ids::SourceService, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::ScreenRuntimePhase;

use crate::{
    screen_event_runtime_input::{
        ScreenRuntimeCaptureInput, ScreenRuntimeDeletionInput, ScreenRuntimeInput,
    },
    screen_event_runtime_refs::screen_correlation_id,
    screen_event_runtime_state::custody_state,
};

pub(crate) fn screen_event_metadata(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeInput,
    observed_at: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(screen_correlation_id(&input.queue_job_id))?,
        screen_event_source(phase)?,
        RecordedAt::parse(observed_at)?,
        Some(TargetHandler::parse(phase.target_handler())?),
    ))
}

pub(crate) fn screen_capture_event_metadata(
    phase: ScreenRuntimePhase,
    input: &ScreenRuntimeCaptureInput,
    observed_at: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(screen_correlation_id(&input.queue_job_id))?,
        screen_event_source(phase)?,
        RecordedAt::parse(observed_at)?,
        Some(TargetHandler::parse(phase.target_handler())?),
    ))
}

pub(crate) fn screen_deletion_event_metadata(
    input: &ScreenRuntimeDeletionInput,
    observed_at: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(screen_correlation_id(&input.queue_job_id))?,
        screen_event_source(ScreenRuntimePhase::DeletionCommitted)?,
        RecordedAt::parse(observed_at)?,
        Some(TargetHandler::parse(
            ScreenRuntimePhase::DeletionCommitted.target_handler(),
        )?),
    ))
}

fn screen_event_source(phase: ScreenRuntimePhase) -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(custody_state(phase))?,
        phase.runtime_role()?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(constants::screen_flow::RUNTIME_COMPONENT_SCREEN_SPINE)?,
        RuntimeInstanceId::parse(constants::screen_flow::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
    ))
}
