use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanCanonicalHouseholdDevice, LanDiscoveryEventKind,
};

pub(super) fn device_event_kind(device: &LanCanonicalHouseholdDevice) -> LanDiscoveryEventKind {
    match &device.classification {
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::ChildAgent => {
            LanDiscoveryEventKind::AgentConfirmed
        }
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::UnknownLanDevice => {
            LanDiscoveryEventKind::UnknownDetected
        }
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Phone
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Tablet
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Laptop
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Desktop
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Printer
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Television
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::GameConsole
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::Camera
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::NetworkAttachedStorage
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::InternetOfThings
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::NetworkInfrastructure
        | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification::UnsupportedLanDevice => {
            LanDiscoveryEventKind::DeviceFound
        }
    }
}

pub(super) fn device_event_summary(event_kind: &LanDiscoveryEventKind) -> &'static str {
    match event_kind {
        LanDiscoveryEventKind::AgentConfirmed => "confirmed child agent visible on LAN",
        LanDiscoveryEventKind::UnknownDetected => "unknown LAN device visible for manual review",
        _ => "LAN device visible in discovery snapshot",
    }
}

pub(super) fn reachability_event(
    reachability: &LanPairingDeviceReachability,
) -> Option<(LanDiscoveryEventKind, &'static str)> {
    match reachability {
        LanPairingDeviceReachability::Online => Some((
            LanDiscoveryEventKind::DeviceOnline,
            "LAN device reachable in current scan",
        )),
        LanPairingDeviceReachability::Offline => Some((
            LanDiscoveryEventKind::DeviceOffline,
            "LAN device offline in current scan",
        )),
        LanPairingDeviceReachability::Stale => Some((
            LanDiscoveryEventKind::DeviceUpdated,
            "LAN device stale in current scan",
        )),
    }
}
