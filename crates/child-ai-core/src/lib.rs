#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::{
    child_domain_ai_analysis_completed_event, constants, ChildDomainAiAnalysisCompletedEvent,
    ChildDomainAiAnalysisRequestedEvent,
    TrackingAiAnalysisRequestedEvent,
    TrackingConfidenceBasis, TrackingNearbyPlaceClassifiedEvent, TrackingPlaceCategory,
};

pub const CRATE_NAME: &str = "ocentra-child-ai-core";

pub fn classify_tracking_nearby_place(
    event: &TrackingAiAnalysisRequestedEvent,
) -> TrackingNearbyPlaceClassifiedEvent {
    TrackingNearbyPlaceClassifiedEvent {
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        source_ai_request_id: event.ai_request_id.clone(),
        evidence_refs: event.evidence_refs.clone(),
        place_category: TrackingPlaceCategory::parse(
            constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL,
        )
        .expect(constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL),
        confidence_basis: TrackingConfidenceBasis::parse(
            constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT,
        )
        .expect(constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT),
        parent_action_requirement: event.parent_action_requirement.clone(),
    }
}

pub fn complete_child_domain_ai_analysis(
    event: &ChildDomainAiAnalysisRequestedEvent,
) -> ChildDomainAiAnalysisCompletedEvent {
    child_domain_ai_analysis_completed_event(event)
}
