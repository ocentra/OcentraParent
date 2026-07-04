use super::{discovery, route, trust};

pub(super) fn stronger_discovery_state(
    existing: ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState,
    incoming: ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState,
) -> ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState {
    discovery::stronger_discovery_state(existing, incoming)
}

pub(super) fn stronger_route_state(
    existing: ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState,
    incoming: ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState,
) -> ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState{
    route::stronger_route_state(existing, incoming)
}

pub(super) fn stronger_trust_state(
    existing: ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState,
    incoming: ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState,
) -> ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState {
    trust::stronger_trust_state(existing, incoming)
}
