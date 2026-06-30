use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingProductionDiscoveryState, LanPairingTrustState,
};

pub(super) fn stronger_discovery_state(
    existing: LanPairingProductionDiscoveryState,
    incoming: LanPairingProductionDiscoveryState,
) -> LanPairingProductionDiscoveryState {
    if discovery_state_rank(&incoming) > discovery_state_rank(&existing) {
        incoming
    } else {
        existing
    }
}

fn discovery_state_rank(state: &LanPairingProductionDiscoveryState) -> u8 {
    match state {
        LanPairingProductionDiscoveryState::Revoked => 8,
        LanPairingProductionDiscoveryState::Rejected => 7,
        LanPairingProductionDiscoveryState::Expired => 6,
        LanPairingProductionDiscoveryState::Paired => 5,
        LanPairingProductionDiscoveryState::Discovered => 4,
        LanPairingProductionDiscoveryState::Stale => 3,
        LanPairingProductionDiscoveryState::Offline => 2,
        LanPairingProductionDiscoveryState::ManualRequired => 1,
        LanPairingProductionDiscoveryState::Pending
        | LanPairingProductionDiscoveryState::Unavailable => 0,
    }
}

pub(super) fn stronger_trust_state(
    existing: LanPairingTrustState,
    incoming: LanPairingTrustState,
) -> LanPairingTrustState {
    if trust_state_rank(&incoming) > trust_state_rank(&existing) {
        incoming
    } else {
        existing
    }
}

fn trust_state_rank(state: &LanPairingTrustState) -> u8 {
    match state {
        LanPairingTrustState::Revoked => 5,
        LanPairingTrustState::Expired => 4,
        LanPairingTrustState::Paired => 3,
        LanPairingTrustState::Pairing => 2,
        LanPairingTrustState::Unpaired => 1,
    }
}

pub(super) fn stronger_route_state(
    existing: ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState,
    incoming: ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState,
) -> ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState{
    if route_state_rank(&incoming) > route_state_rank(&existing) {
        incoming
    } else {
        existing
    }
}

fn route_state_rank(
    state: &ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState,
) -> u8 {
    match state {
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState::LocalNetwork => 4,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState::Localhost => 3,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState::ManualRequired => 2,
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState::Unavailable => 1,
    }
}
