use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

pub(super) fn from_windows_state(state: Option<String>) -> LanPairingDeviceReachability {
    let state = state.map(|value| value.to_ascii_lowercase());
    match state.as_deref() {
        Some(
            constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE_NUMBER
            | constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_PERMANENT_NUMBER
            | constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_REACHABLE
            | constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_PERMANENT,
        ) => LanPairingDeviceReachability::Online,
        Some(
            constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_STALE_NUMBER
            | constants::lan_pairing::WINDOWS_NEIGHBOR_STATE_STALE,
        ) => LanPairingDeviceReachability::Stale,
        _ => LanPairingDeviceReachability::Offline,
    }
}
