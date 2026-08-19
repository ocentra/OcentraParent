use ocentra_eventing::{
    envelope::{EventMetadata, EventSource},
    error::EventingError,
    ids::{
        CorrelationId, EventCustody, EventId, RecordedAt, RuntimeInstanceId, RuntimeRole,
        SourceComponent, SourceService, TargetHandler,
    },
};
use ocentra_parent_agent_protocol::constants;

use super::ObservedAtText;

pub(super) fn screen_service_row_ready_metadata(
    observed_at: &ObservedAtText,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(constants::screen_flow::CORRELATION_SCREEN_RUNTIME_PREFIX)?,
        screen_service_row_ready_source()?,
        RecordedAt::parse(observed_at.0.as_str())?,
        Some(TargetHandler::parse(
            constants::screen_flow::TARGET_SCREEN_SERVICE_EVENT_SUBSCRIBER,
        )?),
    ))
}

fn screen_service_row_ready_source() -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        EventCustody::parse(constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME)?,
        RuntimeRole::parse(constants::eventing_source::ROLE_AGENT)?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(
            constants::screen_flow::RUNTIME_COMPONENT_SCREEN_SERVICE_SUBSCRIBER,
        )?,
        RuntimeInstanceId::parse(constants::screen_flow::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
    ))
}
