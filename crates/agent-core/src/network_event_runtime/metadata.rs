use ocentra_eventing::{
    envelope::{EventMetadata, EventSource},
    error::EventingError,
    ids::{
        CorrelationId, EventId, RecordedAt, RuntimeInstanceId, SourceComponent, SourceService,
        TargetHandler,
    },
};
use ocentra_parent_agent_protocol::constants;

use crate::NetworkObservation;

use super::{helpers, NetworkRuntimePhase};

pub(super) fn network_event_metadata_for_fallback(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
    target_handler: &str,
) -> Result<EventMetadata, EventingError> {
    network_event_metadata_with_id(
        helpers::network_fallback_event_id(phase, observation, observed_at)?,
        CorrelationId::parse(helpers::network_correlation_id(observation, observed_at))?,
        phase,
        observation,
        observed_at,
        target_handler,
    )
}

pub(super) fn network_event_metadata_for_source_event(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
    target_handler: &str,
    source_event_id: &str,
) -> Result<EventMetadata, EventingError> {
    network_event_metadata_with_id(
        helpers::network_event_id(phase, source_event_id)?,
        helpers::network_source_correlation_id(source_event_id)?,
        phase,
        observation,
        observed_at,
        target_handler,
    )
}

fn network_event_metadata_with_id(
    event_id: EventId,
    correlation_id: CorrelationId,
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
    observed_at: &str,
    target_handler: &str,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        event_id,
        correlation_id,
        network_event_source(phase, observation)?,
        RecordedAt::parse(observed_at)?,
        Some(TargetHandler::parse(target_handler)?),
    ))
}

fn network_event_source(
    phase: NetworkRuntimePhase,
    observation: &NetworkObservation,
) -> Result<EventSource, EventingError> {
    Ok(EventSource::new(
        helpers::event_custody(observation)?,
        phase.runtime_role()?,
        SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
        SourceComponent::parse(constants::network_flow::RUNTIME_COMPONENT_NETWORK_SPINE)?,
        RuntimeInstanceId::parse(constants::network_flow::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
    ))
}
