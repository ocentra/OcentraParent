use super::{LanAiProviderRoutingState, LanPairingDeviceReachability};
use super::{
    LanPairingProductionDiscoveryState, LanPairingRuntime, LanPairingTrustState,
    LanProviderSelectionLifecycleState, LanSelectedRouteTarget,
};
use std::string::String as TestString;

pub(super) fn lifecycle_state_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: &LanAiProviderRoutingState,
) -> LanProviderSelectionLifecycleState {
    match (selected, routing_state) {
        (None, _) => LanProviderSelectionLifecycleState::CandidateUnavailable,
        (Some(_), LanAiProviderRoutingState::AuthorizedResult) => {
            LanProviderSelectionLifecycleState::CandidateSelected
        }
        (Some(_), LanAiProviderRoutingState::Busy | LanAiProviderRoutingState::Degraded) => {
            LanProviderSelectionLifecycleState::CandidateDegraded
        }
        (Some(_), LanAiProviderRoutingState::UnsupportedCapability) => {
            LanProviderSelectionLifecycleState::CandidateRejected
        }
        (Some(_), LanAiProviderRoutingState::Unavailable) => {
            LanProviderSelectionLifecycleState::CandidateUnavailable
        }
    }
}

pub(super) fn discovery_state_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
) -> LanPairingProductionDiscoveryState {
    selected.map_or(LanPairingProductionDiscoveryState::Unavailable, |target| {
        discovery_state_from_reachability(&target.reachability)
    })
}

pub(super) fn trust_state_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
) -> LanPairingTrustState {
    selected
        .map(|target| target.trust_state)
        .unwrap_or(LanPairingTrustState::Unpaired)
}

pub(super) fn reachability_for_selected(
    selected: Option<&LanSelectedRouteTarget>,
) -> LanPairingDeviceReachability {
    selected
        .map(|target| target.reachability.clone())
        .unwrap_or(LanPairingDeviceReachability::Offline)
}

pub(super) fn route_id_for_selected(selected: Option<&LanSelectedRouteTarget>) -> TestString {
    selected
        .map(|target| target.route_id.clone())
        .unwrap_or_else(|| super::constants::lan_pairing::ROUTE_ID_UNSUPPORTED.to_string())
}

pub(super) fn provider_peer_id(runtime: &LanPairingRuntime) -> TestString {
    runtime.device_role_read_model().physical_device_id
}

fn discovery_state_from_reachability(
    reachability: &LanPairingDeviceReachability,
) -> LanPairingProductionDiscoveryState {
    match reachability {
        LanPairingDeviceReachability::Online => LanPairingProductionDiscoveryState::Paired,
        LanPairingDeviceReachability::Offline => LanPairingProductionDiscoveryState::Offline,
        LanPairingDeviceReachability::Stale => LanPairingProductionDiscoveryState::Stale,
    }
}
