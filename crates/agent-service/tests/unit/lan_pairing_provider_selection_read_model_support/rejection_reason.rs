use super::{
    LanAiProviderRoutingState, LanPairingDeviceReachability, LanPairingRejectionReason,
    LanPairingTrustState, LanSelectedRouteTarget,
};

pub(super) fn selected_route_rejection_reason(
    selected: Option<&LanSelectedRouteTarget>,
    routing_state: &LanAiProviderRoutingState,
) -> Option<LanPairingRejectionReason> {
    let selected = selected?;
    reachability_rejection(&selected.reachability)
        .or_else(|| trust_state_rejection(&selected.trust_state))
        .or_else(|| provider_unavailable_rejection(&selected.trust_state, routing_state))
}

fn reachability_rejection(
    reachability: &LanPairingDeviceReachability,
) -> Option<LanPairingRejectionReason> {
    match reachability {
        LanPairingDeviceReachability::Offline => Some(LanPairingRejectionReason::Offline),
        LanPairingDeviceReachability::Stale => Some(LanPairingRejectionReason::Stale),
        LanPairingDeviceReachability::Online => None,
    }
}

fn trust_state_rejection(trust_state: &LanPairingTrustState) -> Option<LanPairingRejectionReason> {
    match trust_state {
        LanPairingTrustState::Revoked => Some(LanPairingRejectionReason::Revoked),
        LanPairingTrustState::Expired => Some(LanPairingRejectionReason::Expired),
        LanPairingTrustState::Unpaired => Some(LanPairingRejectionReason::Anonymous),
        LanPairingTrustState::Pairing | LanPairingTrustState::Paired => None,
    }
}

fn provider_unavailable_rejection(
    trust_state: &LanPairingTrustState,
    routing_state: &LanAiProviderRoutingState,
) -> Option<LanPairingRejectionReason> {
    (*routing_state == LanAiProviderRoutingState::Unavailable)
        .then(|| provider_unavailable_for_trust_state(trust_state))
        .flatten()
}

fn provider_unavailable_for_trust_state(
    trust_state: &LanPairingTrustState,
) -> Option<LanPairingRejectionReason> {
    match trust_state {
        LanPairingTrustState::Pairing | LanPairingTrustState::Paired => {
            Some(LanPairingRejectionReason::LanAiProviderUnavailable)
        }
        LanPairingTrustState::Revoked
        | LanPairingTrustState::Expired
        | LanPairingTrustState::Unpaired => None,
    }
}
