use ocentra_eventing::{
    envelope::EventMetadata, envelope::EventSource, error::EventingError, ids::CorrelationId,
    ids::EventCustody, ids::EventId, ids::RecordedAt, ids::RuntimeInstanceId, ids::RuntimeRole,
    ids::SourceComponent, ids::SourceService, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::TrackingTimestamp;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum TrackingRuntimeHop {
    LocationObserved,
    EvidenceRecorded,
    GeofenceTransitionDetected,
    ExpectedPlaceStateEvaluated,
    ChildCheckInRecorded,
    AiAnalysisRequested,
    NearbyPlaceClassified,
    PolicyViolationDetected,
    ParentNotificationRequested,
}

pub(super) fn tracking_runtime_metadata(
    hop: TrackingRuntimeHop,
    correlation_suffix: &str,
    recorded_at: &TrackingTimestamp,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        tracking_runtime_correlation_id(correlation_suffix)?,
        EventSource::new(
            EventCustody::parse(constants::child_agent::CUSTODY_CHILD_AGENT_RUNTIME)?,
            RuntimeRole::parse(hop.runtime_role())?,
            SourceService::parse(constants::peer::LOCAL_DEV_AGENT)?,
            SourceComponent::parse(hop.source_component())?,
            RuntimeInstanceId::parse(constants::child_agent::RUNTIME_INSTANCE_LOCAL_CHILD_AGENT)?,
        ),
        RecordedAt::parse(recorded_at.as_str())?,
        Some(TargetHandler::parse(hop.target_handler())?),
    ))
}

fn tracking_runtime_correlation_id(
    correlation_suffix: &str,
) -> Result<CorrelationId, EventingError> {
    let mut value = String::from(constants::tracking_runtime::CORRELATION_PREFIX);
    value.push_str(correlation_suffix);
    CorrelationId::parse(value)
}
