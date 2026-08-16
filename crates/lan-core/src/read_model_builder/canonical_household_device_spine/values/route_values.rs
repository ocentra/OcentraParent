use ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDeviceRole, LanCanonicalHouseholdRouteState,
};

pub(super) fn role_badges_for(
    is_child_agent: bool,
    status: &LanPairingDiscoveryRuntimeStatus,
) -> Vec<LanCanonicalHouseholdDeviceRole> {
    if !is_child_agent {
        return Vec::new();
    }
    let mut roles = vec![LanCanonicalHouseholdDeviceRole::ChildAgent];
    if *status == LanPairingDiscoveryRuntimeStatus::WebsocketDirect {
        roles.push(LanCanonicalHouseholdDeviceRole::Portal);
        roles.push(LanCanonicalHouseholdDeviceRole::ParentController);
    }
    roles
}

pub(super) fn route_id_for(is_child_agent: bool, route_id: Option<String>) -> Option<String> {
    if is_child_agent {
        route_id
    } else {
        None
    }
}

pub(super) fn route_state_for(
    is_child_agent: bool,
    status: &LanPairingDiscoveryRuntimeStatus,
) -> LanCanonicalHouseholdRouteState {
    if !is_child_agent {
        return LanCanonicalHouseholdRouteState::Unavailable;
    }
    match status {
        LanPairingDiscoveryRuntimeStatus::WebsocketDirect => {
            LanCanonicalHouseholdRouteState::LocalNetwork
        }
        LanPairingDiscoveryRuntimeStatus::NetworkNeighbor => {
            LanCanonicalHouseholdRouteState::ManualRequired
        }
        LanPairingDiscoveryRuntimeStatus::PlannedUnsupported => {
            LanCanonicalHouseholdRouteState::Unavailable
        }
    }
}
