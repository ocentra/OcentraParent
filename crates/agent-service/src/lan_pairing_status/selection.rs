use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing::LanSelectedRouteTarget;

pub(crate) fn child_device_id(selected: Option<&LanSelectedRouteTarget>) -> LanPairingText {
    selected
        .map(|target| LanPairingText(target.selected_child_device_id.clone()))
        .unwrap_or_else(|| LanPairingText(String::new()))
}

pub(crate) fn route_id(selected: Option<&LanSelectedRouteTarget>) -> LanPairingText {
    selected
        .map(|target| LanPairingText(target.route_id.clone()))
        .unwrap_or_else(|| LanPairingText(String::new()))
}

pub(crate) fn pairing_id(selected: Option<&LanSelectedRouteTarget>) -> LanPairingText {
    selected
        .and_then(|target| target.pairing_id.clone().map(LanPairingText))
        .unwrap_or_else(|| LanPairingText(String::new()))
}

pub(crate) fn route_trust_state(selected: Option<&LanSelectedRouteTarget>) -> LanPairingText {
    match selected.map(|target| &target.trust_state) {
        Some(LanPairingTrustState::Paired) => constants::value::LAN_PAIRING_PAIRED.into(),
        Some(LanPairingTrustState::Unpaired) => constants::value::LAN_PAIRING_UNPAIRED.into(),
        Some(LanPairingTrustState::Pairing) => constants::value::LAN_PAIRING_PAIRING.into(),
        Some(LanPairingTrustState::Revoked) => constants::value::LAN_PAIRING_REVOKED.into(),
        Some(LanPairingTrustState::Expired) => constants::value::LAN_PAIRING_EXPIRED.into(),
        None => constants::value::EMPTY.into(),
    }
}

pub(crate) fn reachability(selected: Option<&LanSelectedRouteTarget>) -> LanPairingText {
    match selected.map(|target| &target.reachability) {
        Some(LanPairingDeviceReachability::Online) => {
            constants::value::LAN_REACHABILITY_ONLINE.into()
        }
        Some(LanPairingDeviceReachability::Offline) => {
            constants::value::LAN_REACHABILITY_OFFLINE.into()
        }
        Some(LanPairingDeviceReachability::Stale) => {
            constants::value::LAN_REACHABILITY_STALE.into()
        }
        None => constants::value::EMPTY.into(),
    }
}

pub(crate) fn stale_at(selected: Option<&LanSelectedRouteTarget>) -> LanPairingText {
    selected
        .and_then(|target| target.stale_at.clone().map(LanPairingText))
        .unwrap_or_else(|| LanPairingText(String::new()))
}

pub(crate) fn offline_at(selected: Option<&LanSelectedRouteTarget>) -> LanPairingText {
    selected
        .and_then(|target| target.offline_at.clone().map(LanPairingText))
        .unwrap_or_else(|| LanPairingText(String::new()))
}
