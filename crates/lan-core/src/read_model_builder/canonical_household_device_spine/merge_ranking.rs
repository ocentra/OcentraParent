use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceClassification, LanCanonicalHouseholdRouteState,
};

#[path = "merge_rank_classification.rs"]
mod classification;
#[path = "merge_rank_discovery_state.rs"]
mod discovery_state;
#[path = "merge_rank_route_state.rs"]
mod route_state;
#[path = "merge_rank_trust_state.rs"]
mod trust_state;

pub(super) fn stronger_classification(
    existing: LanCanonicalHouseholdDeviceClassification,
    incoming: LanCanonicalHouseholdDeviceClassification,
) -> LanCanonicalHouseholdDeviceClassification {
    if classification::rank(&incoming) > classification::rank(&existing) {
        incoming
    } else {
        existing
    }
}

pub(super) fn stronger_discovery_state(
    existing: LanPairingProductionDiscoveryState,
    incoming: LanPairingProductionDiscoveryState,
) -> LanPairingProductionDiscoveryState {
    if discovery_state::rank(&incoming) > discovery_state::rank(&existing) {
        incoming
    } else {
        existing
    }
}

pub(super) fn stronger_trust_state(
    existing: LanPairingTrustState,
    incoming: LanPairingTrustState,
) -> LanPairingTrustState {
    if trust_state::rank(&incoming) > trust_state::rank(&existing) {
        incoming
    } else {
        existing
    }
}

pub(super) fn stronger_route_state(
    existing: LanCanonicalHouseholdRouteState,
    incoming: LanCanonicalHouseholdRouteState,
) -> LanCanonicalHouseholdRouteState {
    if route_state::rank(&incoming) > route_state::rank(&existing) {
        incoming
    } else {
        existing
    }
}
