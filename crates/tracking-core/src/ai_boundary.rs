use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::tracking::identifiers::TrackingEvidenceRef;
use ocentra_parent_agent_protocol::tracking::runtime_event::{
    TrackingAiAnalysisRequestedEvent, TrackingNearbyPlaceClassifiedEvent,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TrackingAiBoundaryDecision {
    pub decision_state: &'static str,
    pub accepted_evidence_refs: Vec<TrackingEvidenceRef>,
}

pub fn validate_tracking_ai_result_as_evidence(
    request: &TrackingAiAnalysisRequestedEvent,
    result: &TrackingNearbyPlaceClassifiedEvent,
) -> TrackingAiBoundaryDecision {
    if result.child_device_id != request.child_device_id
        || result.child_profile_id != request.child_profile_id
    {
        return rejected(constants::tracking_runtime::AI_RESULT_REJECTED_WRONG_CHILD_OR_DEVICE_REF);
    }

    if result.source_ai_request_id != request.ai_request_id {
        return rejected(constants::tracking_runtime::AI_RESULT_REJECTED_STALE_CORRELATION);
    }

    if result.source_observed_at != request.source_observed_at {
        return rejected(constants::tracking_runtime::AI_RESULT_REJECTED_STALE_CORRELATION);
    }

    if result.evidence_refs.is_empty() {
        return rejected(constants::tracking_runtime::AI_RESULT_REJECTED_MISSING_EVIDENCE_REF);
    }

    if !result
        .evidence_refs
        .iter()
        .all(|evidence_ref| request.evidence_refs.contains(evidence_ref))
    {
        return rejected(constants::tracking_runtime::AI_RESULT_REJECTED_HALLUCINATED_EVIDENCE_REF);
    }

    TrackingAiBoundaryDecision {
        decision_state: constants::tracking_runtime::AI_RESULT_ACCEPTED_AS_EVIDENCE,
        accepted_evidence_refs: result.evidence_refs.clone(),
    }
}

fn rejected(decision_state: &'static str) -> TrackingAiBoundaryDecision {
    TrackingAiBoundaryDecision {
        decision_state,
        accepted_evidence_refs: Vec::new(),
    }
}
