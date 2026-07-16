use ocentra_parent_agent_protocol::constants;
use serde::{Deserialize, Serialize};

use crate::{
    household_ai_provider_route_labels::{
        route_rank, route_reason_label, route_state_for_rejection,
    },
    household_ai_provider_route_state::{
        HouseholdAiProviderClass, HouseholdAiProviderResourcePolicy,
        HouseholdAiProviderResourceState, HouseholdAiProviderTrustState,
        HouseholdAiRouteDecisionState, HouseholdAiRouteRejectionReason, HouseholdAiWorkClass,
    },
    screen_household_mesh_runtime_state::custody_label,
};

#[path = "household_ai_provider_route_rejection.rs"]
mod household_ai_provider_route_rejection;
#[path = "household_ai_provider_route_rejection_mobile.rs"]
mod household_ai_provider_route_rejection_mobile;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAiProviderCandidate {
    pub provider_peer_id: String,
    pub provider_class: HouseholdAiProviderClass,
    pub trust_state: HouseholdAiProviderTrustState,
    pub resource_state: HouseholdAiProviderResourceState,
    pub custody_label: String,
    pub supports_heavy_screen_vision: bool,
    pub supports_light_text: bool,
    pub resource_policy: HouseholdAiProviderResourcePolicy,
}

impl HouseholdAiProviderCandidate {
    pub fn parent_desktop() -> Self {
        Self::trusted(
            constants::household_mesh::TEST_PARENT_DESKTOP_PROVIDER_ID,
            HouseholdAiProviderClass::DesktopPreferred,
        )
    }

    pub fn household_laptop() -> Self {
        Self::trusted(
            constants::household_mesh::TEST_OTHER_LAPTOP_PROVIDER_ID,
            HouseholdAiProviderClass::LaptopPreferred,
        )
    }

    pub fn child_desktop() -> Self {
        Self::trusted(
            constants::household_mesh::TEST_CHILD_DESKTOP_PROVIDER_ID,
            HouseholdAiProviderClass::ChildDesktopLocal,
        )
    }

    pub fn parent_mobile() -> Self {
        let mut candidate = Self::trusted(
            constants::household_mesh::TEST_PARENT_MOBILE_PROVIDER_ID,
            HouseholdAiProviderClass::MobileDormant,
        );
        candidate.supports_heavy_screen_vision = false;
        candidate
    }

    fn trusted(provider_peer_id: &'static str, provider_class: HouseholdAiProviderClass) -> Self {
        Self {
            provider_peer_id: provider_peer_id.to_string(),
            provider_class,
            trust_state: HouseholdAiProviderTrustState::Trusted,
            resource_state: HouseholdAiProviderResourceState::Ready,
            custody_label: custody_label().to_string(),
            supports_heavy_screen_vision: true,
            supports_light_text: true,
            resource_policy: HouseholdAiProviderResourcePolicy::desktop_ready(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAiRouteRequest {
    pub job_id: String,
    pub work_class: HouseholdAiWorkClass,
    pub allow_mobile_fallback: bool,
    pub required_custody_label: String,
}

impl HouseholdAiRouteRequest {
    pub fn heavy_screen_job() -> Self {
        Self {
            job_id: constants::household_mesh::TEST_ROUTE_JOB_ID.to_string(),
            work_class: HouseholdAiWorkClass::HeavyScreenVision,
            allow_mobile_fallback: false,
            required_custody_label: custody_label().to_string(),
        }
    }

    pub fn mobile_light_fallback_job() -> Self {
        Self {
            job_id: constants::household_mesh::TEST_ROUTE_JOB_ID.to_string(),
            work_class: HouseholdAiWorkClass::LightText,
            allow_mobile_fallback: true,
            required_custody_label: custody_label().to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAiRouteCandidateDecision {
    pub provider_peer_id: String,
    pub provider_class: HouseholdAiProviderClass,
    pub state: HouseholdAiRouteDecisionState,
    pub rejection_reason: Option<HouseholdAiRouteRejectionReason>,
    pub reason_label: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct HouseholdAiRouteSelection {
    pub job_id: String,
    pub selected_provider_peer_id: Option<String>,
    pub selected_provider_class: Option<HouseholdAiProviderClass>,
    pub selected_reason_label: String,
    pub candidate_decisions: Vec<HouseholdAiRouteCandidateDecision>,
}

pub fn select_household_ai_provider_route(
    request: &HouseholdAiRouteRequest,
    candidates: &[HouseholdAiProviderCandidate],
) -> HouseholdAiRouteSelection {
    let desktop_or_laptop_available = candidates.iter().any(|candidate| {
        matches!(
            candidate.provider_class,
            HouseholdAiProviderClass::DesktopPreferred | HouseholdAiProviderClass::LaptopPreferred
        ) && candidate_rejection_reason(request, candidate, false).is_none()
    });
    let mut decisions = route_candidate_decisions(request, candidates, desktop_or_laptop_available);
    decisions.sort_by_key(|decision| route_rank(decision.provider_class));
    let selected = decisions
        .iter()
        .find(|decision| decision.state == HouseholdAiRouteDecisionState::Selected);
    HouseholdAiRouteSelection {
        job_id: request.job_id.clone(),
        selected_provider_peer_id: selected.map(|decision| decision.provider_peer_id.clone()),
        selected_provider_class: selected.map(|decision| decision.provider_class),
        selected_reason_label: selected
            .map(|decision| decision.reason_label.clone())
            .unwrap_or_else(|| constants::household_mesh::ROUTE_REASON_NO_PROVIDER.to_string()),
        candidate_decisions: decisions,
    }
}

fn route_candidate_decisions(
    request: &HouseholdAiRouteRequest,
    candidates: &[HouseholdAiProviderCandidate],
    desktop_or_laptop_available: bool,
) -> Vec<HouseholdAiRouteCandidateDecision> {
    let mut decisions: Vec<_> = candidates
        .iter()
        .map(|candidate| route_candidate_decision(request, candidate, desktop_or_laptop_available))
        .collect();
    if decisions.is_empty() {
        decisions.push(no_provider_decision());
    }
    decisions
}

fn route_candidate_decision(
    request: &HouseholdAiRouteRequest,
    candidate: &HouseholdAiProviderCandidate,
    desktop_or_laptop_available: bool,
) -> HouseholdAiRouteCandidateDecision {
    let rejection_reason =
        candidate_rejection_reason(request, candidate, desktop_or_laptop_available);
    let state = route_state_for_rejection(candidate.provider_class, rejection_reason);
    HouseholdAiRouteCandidateDecision {
        provider_peer_id: candidate.provider_peer_id.clone(),
        provider_class: candidate.provider_class,
        state,
        rejection_reason,
        reason_label: route_reason_label(candidate.provider_class, rejection_reason).to_string(),
    }
}

fn candidate_rejection_reason(
    request: &HouseholdAiRouteRequest,
    candidate: &HouseholdAiProviderCandidate,
    desktop_or_laptop_available: bool,
) -> Option<HouseholdAiRouteRejectionReason> {
    household_ai_provider_route_rejection::candidate_rejection_reason(
        request,
        candidate,
        desktop_or_laptop_available,
    )
}

fn no_provider_decision() -> HouseholdAiRouteCandidateDecision {
    HouseholdAiRouteCandidateDecision {
        provider_peer_id: constants::value::UNKNOWN_HOST.to_string(),
        provider_class: HouseholdAiProviderClass::MobileDormant,
        state: HouseholdAiRouteDecisionState::Unavailable,
        rejection_reason: Some(HouseholdAiRouteRejectionReason::NoProvider),
        reason_label: constants::household_mesh::ROUTE_REASON_NO_PROVIDER.to_string(),
    }
}
