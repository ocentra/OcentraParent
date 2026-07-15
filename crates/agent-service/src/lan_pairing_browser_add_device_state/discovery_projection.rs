use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingProductionDiscoveryState, LanPairingText,
    LanPairingTrustState, LanSelectedRouteTarget,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceReadModel, LanSelectedDeviceReadiness,
};

const DISCOVERY_STATE_MAPPINGS: [(&str, LanPairingProductionDiscoveryState); 9] = [
    (
        constants::value::LAN_DISCOVERY_STATE_PENDING,
        LanPairingProductionDiscoveryState::Pending,
    ),
    (
        constants::value::LAN_DISCOVERY_STATE_PAIRED,
        LanPairingProductionDiscoveryState::Paired,
    ),
    (
        constants::value::LAN_DISCOVERY_STATE_REJECTED,
        LanPairingProductionDiscoveryState::Rejected,
    ),
    (
        constants::value::LAN_DISCOVERY_STATE_EXPIRED,
        LanPairingProductionDiscoveryState::Expired,
    ),
    (
        constants::value::LAN_DISCOVERY_STATE_REVOKED,
        LanPairingProductionDiscoveryState::Revoked,
    ),
    (
        constants::value::LAN_DISCOVERY_STATE_STALE,
        LanPairingProductionDiscoveryState::Stale,
    ),
    (
        constants::value::LAN_DISCOVERY_STATE_OFFLINE,
        LanPairingProductionDiscoveryState::Offline,
    ),
    (
        constants::value::LAN_DISCOVERY_STATE_MANUAL_REQUIRED,
        LanPairingProductionDiscoveryState::ManualRequired,
    ),
    (
        constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE,
        LanPairingProductionDiscoveryState::Unavailable,
    ),
];

pub(super) fn physical_household_lan_state(
    has_network_devices: bool,
) -> LanPairingProductionDiscoveryState {
    if has_network_devices {
        LanPairingProductionDiscoveryState::Discovered
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

pub(crate) fn selected_device_readiness(
    selected: Option<LanSelectedRouteTarget>,
) -> LanSelectedDeviceReadiness {
    match selected {
        Some(target) => {
            let route_id = non_empty_text(&LanPairingText(target.route_id.clone()));
            let ready_for_control = route_id.is_some()
                && target.trust_state == LanPairingTrustState::Paired
                && target.reachability == LanPairingDeviceReachability::Online;
            LanSelectedDeviceReadiness {
                schema_version: constants::lan_pairing::SCHEMA_VERSION,
                selected_child_device_id: Some(target.selected_child_device_id),
                route_id: route_id.map(|value| value.0),
                pairing_id: target.pairing_id,
                trust_state: target.trust_state,
                reachability: target.reachability,
                ready_for_control,
                stale_at: target.stale_at,
                offline_at: target.offline_at,
            }
        }
        None => LanSelectedDeviceReadiness {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: None,
            route_id: None,
            pairing_id: None,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Offline,
            ready_for_control: false,
            stale_at: None,
            offline_at: None,
        },
    }
}

fn non_empty_text(value: &LanPairingText) -> Option<LanPairingText> {
    let trimmed = value.0.trim();
    (!trimmed.is_empty()).then(|| LanPairingText(trimmed.to_string()))
}

pub(super) fn discovery_state_for(state: &LanPairingText) -> LanPairingProductionDiscoveryState {
    DISCOVERY_STATE_MAPPINGS
        .iter()
        .find(|(candidate, _)| *candidate == state.0.as_str())
        .map(|(_, mapped)| mapped.clone())
        .unwrap_or(LanPairingProductionDiscoveryState::Discovered)
}

pub(super) fn pending_pairing_count(model: &LanBrowserAddDeviceReadModel) -> usize {
    model
        .pairing_requests
        .iter()
        .filter(|request| request.pairing_state == LanPairingProductionDiscoveryState::Pending)
        .count()
}

pub(super) fn expired_pairing_count(model: &LanBrowserAddDeviceReadModel) -> usize {
    model
        .pairing_requests
        .iter()
        .filter(|request| request.pairing_state == LanPairingProductionDiscoveryState::Expired)
        .count()
}

pub(super) fn serialized_enum_label(value: impl serde::Serialize) -> LanPairingText {
    serde_json::to_value(value)
        .ok()
        .and_then(|json| json.as_str().map(|value| LanPairingText(value.to_owned())))
        .unwrap_or_else(|| LanPairingText(constants::value::EMPTY.to_string()))
}

pub(super) fn pairing_request_state(
    accepted: bool,
    observed_at: LanPairingText,
    expires_at: LanPairingText,
) -> LanPairingProductionDiscoveryState {
    let observed_at = observed_at.0;
    let expires_at = expires_at.0;
    if accepted {
        LanPairingProductionDiscoveryState::Paired
    } else if observed_at > expires_at {
        LanPairingProductionDiscoveryState::Expired
    } else {
        LanPairingProductionDiscoveryState::Pending
    }
}
