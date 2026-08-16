use ocentra_eventing::{
    envelope::EventMetadata, envelope::EventSource, error::EventingError, ids::CorrelationId,
    ids::EventCustody, ids::EventId, ids::RecordedAt, ids::RuntimeInstanceId, ids::RuntimeRole,
    ids::SourceComponent, ids::SourceService, ids::TargetHandler,
};
use ocentra_parent_agent_protocol::child_domain_runtime::{
    ChildDomainAiAnalysisRequestedEvent, ChildDomainAiRequestId, ChildDomainEvidenceRecordedEvent,
    ChildDomainEvidenceRef, ChildDomainObservationId, ChildDomainObservedAt,
    ChildDomainObservedEvent, ChildDomainPolicyEvaluationRequestedEvent,
    ChildDomainPolicyRequestId, ChildDomainPolicyViolationId,
};
use ocentra_parent_agent_protocol::constants;

mod ai_analysis_requested_event;
mod evidence_recorded_event;
mod hop_correlation_ref;
mod hop_source_component_and_runtime_role;
mod hop_target_handler;
mod policy_evaluation_requested_event;

pub(super) fn child_domain_evidence_recorded_event(
    event: &ChildDomainObservedEvent,
) -> Result<ChildDomainEvidenceRecordedEvent, EventingError> {
    evidence_recorded_event::child_domain_evidence_recorded_event(event)
}

pub(super) fn child_domain_ai_analysis_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Result<Option<ChildDomainAiAnalysisRequestedEvent>, EventingError> {
    ai_analysis_requested_event::child_domain_ai_analysis_requested_event(event)
}

pub(super) fn child_domain_policy_evaluation_requested_event(
    event: &ChildDomainEvidenceRecordedEvent,
) -> Result<Option<ChildDomainPolicyEvaluationRequestedEvent>, EventingError> {
    policy_evaluation_requested_event::child_domain_policy_evaluation_requested_event(event)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ChildDomainRuntimeHop<'a> {
    Observed(&'a ChildDomainObservationId),
    EvidenceRecorded(&'a ChildDomainEvidenceRef),
    AiAnalysisRequested(&'a ChildDomainEvidenceRef),
    AiAnalysisCompleted(&'a ChildDomainAiRequestId),
    PolicyEvaluationRequested(&'a ChildDomainEvidenceRef),
    PolicyEvaluationRequestedFromAi(&'a ChildDomainAiRequestId),
    PolicyViolationDetected(&'a ChildDomainPolicyRequestId),
    NotificationRequested(&'a ChildDomainPolicyViolationId),
}

pub(super) fn child_domain_runtime_metadata(
    hop: ChildDomainRuntimeHop<'_>,
    recorded_at: &ChildDomainObservedAt,
) -> Result<EventMetadata, EventingError> {
    Ok(EventMetadata::from_parts(
        EventId::generated(),
        CorrelationId::parse(child_domain_runtime_correlation_id(hop))?,
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

fn child_domain_runtime_correlation_id(hop: ChildDomainRuntimeHop<'_>) -> String {
    let mut value = String::from(constants::child_domain_runtime::CORRELATION_PREFIX);
    value.push_str(hop.correlation_ref());
    value
}
