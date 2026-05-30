use ocentra_parent_agent_protocol::{
    constants, LanPairingDeviceReachability, LanPairingTrustState, LanSelectedRouteTarget,
};

pub(super) fn child_device_id(selected: Option<&LanSelectedRouteTarget>) -> String {
    selected
        .map(|target| target.selected_child_device_id.clone())
        .unwrap_or_default()
}

pub(super) fn route_id(selected: Option<&LanSelectedRouteTarget>) -> String {
    selected
        .map(|target| target.route_id.clone())
        .unwrap_or_default()
}

pub(super) fn pairing_id(selected: Option<&LanSelectedRouteTarget>) -> String {
    selected
        .and_then(|target| target.pairing_id.clone())
        .unwrap_or_default()
}

pub(super) fn route_trust_state(selected: Option<&LanSelectedRouteTarget>) -> &'static str {
    match selected.map(|target| &target.trust_state) {
        Some(LanPairingTrustState::Paired) => constants::value::LAN_PAIRING_PAIRED,
        Some(LanPairingTrustState::Unpaired) => constants::value::LAN_PAIRING_UNPAIRED,
        Some(LanPairingTrustState::Pairing) => constants::value::LAN_PAIRING_PAIRING,
        Some(LanPairingTrustState::Revoked) => constants::value::LAN_PAIRING_REVOKED,
        Some(LanPairingTrustState::Expired) => constants::value::LAN_PAIRING_EXPIRED,
        None => constants::value::EMPTY,
    }
}

pub(super) fn reachability(selected: Option<&LanSelectedRouteTarget>) -> &'static str {
    match selected.map(|target| &target.reachability) {
        Some(LanPairingDeviceReachability::Online) => constants::value::LAN_REACHABILITY_ONLINE,
        Some(LanPairingDeviceReachability::Offline) => constants::value::LAN_REACHABILITY_OFFLINE,
        Some(LanPairingDeviceReachability::Stale) => constants::value::LAN_REACHABILITY_STALE,
        None => constants::value::EMPTY,
    }
}

pub(super) fn stale_at(selected: Option<&LanSelectedRouteTarget>) -> String {
    selected
        .and_then(|target| target.stale_at.clone())
        .unwrap_or_default()
}

pub(super) fn offline_at(selected: Option<&LanSelectedRouteTarget>) -> String {
    selected
        .and_then(|target| target.offline_at.clone())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use ocentra_parent_agent_protocol::{LanPairingNetworkMode, LanSelectedRouteTarget};

    use super::*;

    #[test]
    fn route_trust_state_reports_pairing_selected_target() {
        let target = LanSelectedRouteTarget {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: constants::lan_pairing::CHILD_DEVICE_ID.to_string(),
            route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            pairing_id: Some(constants::lan_pairing::PAIRING_ID.to_string()),
            trust_state: LanPairingTrustState::Pairing,
            network_mode: LanPairingNetworkMode::LocalNetwork,
            reachability: LanPairingDeviceReachability::Online,
            stale_at: None,
            offline_at: None,
        };

        assert_eq!(
            route_trust_state(Some(&target)),
            constants::value::LAN_PAIRING_PAIRING
        );
    }
}
