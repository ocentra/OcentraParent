use ocentra_parent_agent_protocol::constants;

use crate::{
    select_household_ai_provider_route,
    test_text::{TestResult, TestText},
    HouseholdAiProviderCandidate, HouseholdAiProviderClass, HouseholdAiProviderResourceState,
    HouseholdAiProviderTrustState, HouseholdAiRouteDecisionState, HouseholdAiRouteRejectionReason,
    HouseholdAiRouteRequest,
};

#[test]
fn household_ai_provider_route_prefers_parent_desktop_then_laptop_then_child_desktop() {
    let request = HouseholdAiRouteRequest::heavy_screen_job();
    let selection = select_household_ai_provider_route(
        &request,
        &[
            HouseholdAiProviderCandidate::child_desktop(),
            HouseholdAiProviderCandidate::household_laptop(),
            HouseholdAiProviderCandidate::parent_desktop(),
        ],
    );

    assert_eq!(
        selection.selected_provider_peer_id,
        Some(constants::household_mesh::TEST_PARENT_DESKTOP_PROVIDER_ID.to_string())
    );
    assert_eq!(
        selection.selected_provider_class,
        Some(HouseholdAiProviderClass::DesktopPreferred)
    );
    assert_eq!(
        selection.selected_reason_label,
        constants::household_mesh::ROUTE_REASON_SELECTED_DESKTOP
    );
}

#[test]
fn household_ai_provider_route_rejects_stale_offline_revoked_and_custody_mismatch() {
    let request = HouseholdAiRouteRequest::heavy_screen_job();

    assert_route_rejection(
        &request,
        |candidate| candidate.trust_state = HouseholdAiProviderTrustState::Stale,
        HouseholdAiRouteRejectionReason::StaleProvider,
    );
    assert_route_rejection(
        &request,
        |candidate| candidate.trust_state = HouseholdAiProviderTrustState::Offline,
        HouseholdAiRouteRejectionReason::OfflineProvider,
    );
    assert_route_rejection(
        &request,
        |candidate| candidate.trust_state = HouseholdAiProviderTrustState::Revoked,
        HouseholdAiRouteRejectionReason::RevokedProvider,
    );
    assert_route_rejection(
        &request,
        |candidate| {
            candidate.custody_label = constants::eventing_source::CUSTODY_LOCAL_ONLY.to_string()
        },
        HouseholdAiRouteRejectionReason::CustodyMismatch,
    );
}

#[test]
fn household_ai_provider_route_keeps_mobile_dormant_when_desktop_is_available() -> TestResult {
    let selection = select_household_ai_provider_route(
        &HouseholdAiRouteRequest::heavy_screen_job(),
        &[
            HouseholdAiProviderCandidate::parent_mobile(),
            HouseholdAiProviderCandidate::parent_desktop(),
        ],
    );
    let mobile = selection
        .candidate_decisions
        .iter()
        .find(|decision| decision.provider_class == HouseholdAiProviderClass::MobileDormant)
        .ok_or_else(|| {
            TestText::from_display(constants::household_mesh::ERROR_ROUTE_SELECTS_PROVIDER)
        })?;

    assert_eq!(
        selection.selected_provider_peer_id,
        Some(constants::household_mesh::TEST_PARENT_DESKTOP_PROVIDER_ID.to_string())
    );
    assert_eq!(mobile.state, HouseholdAiRouteDecisionState::Dormant);
    assert_eq!(
        mobile.rejection_reason,
        Some(HouseholdAiRouteRejectionReason::MobileDormantDesktopAvailable)
    );

    Ok(())
}

#[test]
fn household_ai_provider_route_allows_mobile_only_for_explicit_light_fallback() {
    let request = HouseholdAiRouteRequest::mobile_light_fallback_job();
    let selection = select_household_ai_provider_route(
        &request,
        &[HouseholdAiProviderCandidate::parent_mobile()],
    );

    assert_eq!(
        selection.selected_provider_peer_id,
        Some(constants::household_mesh::TEST_PARENT_MOBILE_PROVIDER_ID.to_string())
    );
    assert_eq!(
        selection.selected_reason_label,
        constants::household_mesh::ROUTE_REASON_MOBILE_FALLBACK_ALLOWED
    );
}

#[test]
fn household_ai_provider_route_rejects_mobile_fallback_for_low_battery_or_thermal() {
    let request = HouseholdAiRouteRequest::mobile_light_fallback_job();
    assert_mobile_rejection(&request, |candidate| {
        candidate.resource_policy.battery_ok = false
    });
    assert_mobile_rejection(&request, |candidate| {
        candidate.resource_policy.thermal_ok = false
    });
    assert_mobile_rejection(&request, |candidate| {
        candidate.resource_policy.fallback_policy_allows_mobile = false
    });
}

#[test]
fn household_ai_provider_route_rejects_degraded_and_unsupported_mobile_candidates() {
    let request = HouseholdAiRouteRequest::heavy_screen_job();
    assert_mobile_route_rejection(
        &request,
        |candidate| candidate.resource_state = HouseholdAiProviderResourceState::Degraded,
        HouseholdAiRouteRejectionReason::ResourceDegraded,
    );
    assert_mobile_route_rejection(
        &request,
        |candidate| candidate.supports_heavy_screen_vision = false,
        HouseholdAiRouteRejectionReason::UnsupportedCapability,
    );
}

fn assert_route_rejection(
    request: &HouseholdAiRouteRequest,
    mutate: impl FnOnce(&mut HouseholdAiProviderCandidate),
    expected: HouseholdAiRouteRejectionReason,
) {
    let mut candidate = HouseholdAiProviderCandidate::parent_desktop();
    mutate(&mut candidate);
    let selection = select_household_ai_provider_route(request, &[candidate]);
    assert_eq!(selection.selected_provider_peer_id, None);
    assert_eq!(
        selection.candidate_decisions[0].state,
        HouseholdAiRouteDecisionState::Rejected
    );
    assert_eq!(
        selection.candidate_decisions[0].rejection_reason,
        Some(expected)
    );
}

fn assert_mobile_route_rejection(
    request: &HouseholdAiRouteRequest,
    mutate: impl FnOnce(&mut HouseholdAiProviderCandidate),
    expected: HouseholdAiRouteRejectionReason,
) {
    let mut candidate = HouseholdAiProviderCandidate::parent_mobile();
    candidate.supports_heavy_screen_vision = true;
    mutate(&mut candidate);
    let selection = select_household_ai_provider_route(request, &[candidate]);
    assert_eq!(selection.selected_provider_peer_id, None);
    assert_eq!(
        selection.candidate_decisions[0].state,
        HouseholdAiRouteDecisionState::Rejected
    );
    assert_eq!(
        selection.candidate_decisions[0].rejection_reason,
        Some(expected)
    );
}

fn assert_mobile_rejection(
    request: &HouseholdAiRouteRequest,
    mutate: impl FnOnce(&mut HouseholdAiProviderCandidate),
) {
    let mut candidate = HouseholdAiProviderCandidate::parent_mobile();
    mutate(&mut candidate);
    let selection = select_household_ai_provider_route(request, &[candidate]);
    assert_eq!(selection.selected_provider_peer_id, None);
    assert_eq!(
        selection.candidate_decisions[0].state,
        HouseholdAiRouteDecisionState::Dormant
    );
    assert_eq!(
        selection.candidate_decisions[0].rejection_reason,
        Some(HouseholdAiRouteRejectionReason::MobileFallbackDenied)
    );
}
