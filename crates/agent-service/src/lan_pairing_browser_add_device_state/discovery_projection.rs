use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingProductionDiscoveryState, LanPairingTrustState,
    LanSelectedRouteTarget,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceReadModel, LanSelectedDeviceReadiness,
};

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
            let route_id = non_empty_text(&target.route_id);
            let ready_for_control = route_id.is_some()
                && target.trust_state == LanPairingTrustState::Paired
                && target.reachability == LanPairingDeviceReachability::Online;
            LanSelectedDeviceReadiness {
                schema_version: constants::lan_pairing::SCHEMA_VERSION,
                selected_child_device_id: Some(target.selected_child_device_id),
                route_id,
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

fn non_empty_text(value: &str) -> Option<String> {
    let trimmed = value.trim();
    (!trimmed.is_empty()).then(|| trimmed.to_string())
}

pub(super) fn discovery_state_for(state: &str) -> LanPairingProductionDiscoveryState {
    match state {
        constants::value::LAN_DISCOVERY_STATE_PENDING => {
            LanPairingProductionDiscoveryState::Pending
        }
        constants::value::LAN_DISCOVERY_STATE_PAIRED => LanPairingProductionDiscoveryState::Paired,
        constants::value::LAN_DISCOVERY_STATE_REJECTED => {
            LanPairingProductionDiscoveryState::Rejected
        }
        constants::value::LAN_DISCOVERY_STATE_EXPIRED => {
            LanPairingProductionDiscoveryState::Expired
        }
        constants::value::LAN_DISCOVERY_STATE_REVOKED => {
            LanPairingProductionDiscoveryState::Revoked
        }
        constants::value::LAN_DISCOVERY_STATE_STALE => LanPairingProductionDiscoveryState::Stale,
        constants::value::LAN_DISCOVERY_STATE_OFFLINE => {
            LanPairingProductionDiscoveryState::Offline
        }
        constants::value::LAN_DISCOVERY_STATE_MANUAL_REQUIRED => {
            LanPairingProductionDiscoveryState::ManualRequired
        }
        constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE => {
            LanPairingProductionDiscoveryState::Unavailable
        }
        _ => LanPairingProductionDiscoveryState::Discovered,
    }
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

pub(super) fn serialized_enum_label<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|json| json.as_str().map(ToOwned::to_owned))
        .unwrap_or_default()
}

pub(super) fn pairing_request_state(
    accepted: bool,
    observed_at: &str,
    expires_at: &str,
) -> LanPairingProductionDiscoveryState {
    if accepted {
        LanPairingProductionDiscoveryState::Paired
    } else if observed_at > expires_at {
        LanPairingProductionDiscoveryState::Expired
    } else {
        LanPairingProductionDiscoveryState::Pending
    }
}
