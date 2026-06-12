#![forbid(unsafe_code)]

use ocentra_parent_agent_protocol::{
    child_domain_ref, constants, ChildDomainAiAnalysisRequestedEvent,
    ChildDomainPolicyEvaluationRequestedEvent, TrackingAiAnalysisRequestedEvent,
    TrackingNearbyPlaceClassifiedEvent,
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
        place_category: constants::tracking_runtime::PLACE_CATEGORY_HOSPITAL.to_string(),
        confidence_basis: constants::tracking_runtime::CONFIDENCE_BASIS_AI_BOUNDARY_CONTRACT
            .to_string(),
    }
}

pub fn complete_child_domain_ai_analysis(
    event: &ChildDomainAiAnalysisRequestedEvent,
) -> ChildDomainPolicyEvaluationRequestedEvent {
    ChildDomainPolicyEvaluationRequestedEvent {
        event_type: child_domain_policy_event_type(&event.domain).to_string(),
        domain: event.domain.clone(),
        child_device_id: event.child_device_id.clone(),
        child_profile_id: event.child_profile_id.clone(),
        policy_request_id: child_domain_ref(
            &event.domain,
            constants::child_domain_runtime::DEFAULT_POLICY_REQUEST_ID_SUFFIX,
        ),
        evidence_refs: event.evidence_refs.clone(),
        source_fact_ref: event.ai_request_id.clone(),
    }
}

fn child_domain_policy_event_type(domain: &str) -> &'static str {
    match domain {
        constants::child_domain_runtime::DOMAIN_BROWSER => {
            constants::child_domain_runtime::BROWSER_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
        }
        constants::child_domain_runtime::DOMAIN_SCREEN => {
            constants::child_domain_runtime::SCREEN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
        }
        constants::child_domain_runtime::DOMAIN_SCREEN_LIVE_VIEW => {
            constants::child_domain_runtime::SCREEN_LIVE_VIEW_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
        }
        constants::child_domain_runtime::DOMAIN_APP_GAME => {
            constants::child_domain_runtime::APP_GAME_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
        }
        constants::child_domain_runtime::DOMAIN_LAN => {
            constants::child_domain_runtime::LAN_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
        }
        constants::child_domain_runtime::DOMAIN_NETWORK => {
            constants::child_domain_runtime::NETWORK_POLICY_EVALUATION_REQUESTED_EVENT_TYPE
        }
        _ => constants::child_domain_runtime::AI_ANALYSIS_COMPLETED_EVENT_TYPE,
    }
}
