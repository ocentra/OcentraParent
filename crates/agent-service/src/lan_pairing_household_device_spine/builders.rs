use super::values::{
    canonical_device_id, child_agent_inventory_for, classification_for, confidence_for_discovery,
    display_name_for, network_identity_for, role_badges_for, route_id_for, route_state_for,
    source_for_discovery, state_from_trust, surfaces_for,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingNetworkMode;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceConfidence;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceRole;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdRouteState;

pub(super) fn device_from_discovery(
    discovered: &LanBrowserAddDeviceDiscoveryDevice,
) -> LanCanonicalHouseholdDevice {
    let device = &discovered.child_device;
    let classification = classification_for(device, false);
    let is_child_agent = classification == LanCanonicalHouseholdDeviceClassification::ChildAgent;
    let source = source_for_discovery(&discovered.discovery_status);
    let network_identity = network_identity_for(
        device,
        discovered.reachability.clone(),
        confidence_for_discovery(&discovered.discovery_status),
        &source,
    );
    LanCanonicalHouseholdDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        canonical_device_id: canonical_device_id(device),
        display_name: display_name_for(device),
        role_badges: role_badges_for(is_child_agent, &discovered.discovery_status),
        enrollable: is_child_agent,
        discovery_state: discovered.discovery_state.clone(),
        trust_state: LanPairingTrustState::Unpaired,
        route_id: route_id_for(is_child_agent, Some(discovered.route_id.clone())),
        route_state: route_state_for(is_child_agent, &discovered.discovery_status),
        network_mode: discovered.network_mode.clone(),
        source_labels: vec![source],
        child_agent_inventory: child_agent_inventory_for(
            is_child_agent,
            device,
            LanPairingTrustState::Unpaired,
            route_state_for(is_child_agent, &discovered.discovery_status),
        ),
        policy_target_surfaces: surfaces_for(is_child_agent),
        network_identity,
        classification,
    }
}

pub(super) fn device_from_registry(
    entry: &LanTrustedDeviceRegistryEntry,
) -> LanCanonicalHouseholdDevice {
    let device = &entry.child_device;
    let network_identity = network_identity_for(
        device,
        LanPairingDeviceReachability::Stale,
        LanCanonicalHouseholdDeviceConfidence::ManualRequired,
        &LanCanonicalHouseholdDeviceSource::TrustedRegistry,
    );
    LanCanonicalHouseholdDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        canonical_device_id: canonical_device_id(device),
        display_name: display_name_for(device),
        classification: LanCanonicalHouseholdDeviceClassification::ChildAgent,
        role_badges: vec![LanCanonicalHouseholdDeviceRole::ChildAgent],
        enrollable: true,
        discovery_state: state_from_trust(&entry.trust_state),
        trust_state: entry.trust_state.clone(),
        route_id: Some(entry.route_id.clone()),
        route_state: LanCanonicalHouseholdRouteState::LocalNetwork,
        network_mode: LanPairingNetworkMode::LocalNetwork,
        source_labels: vec![LanCanonicalHouseholdDeviceSource::TrustedRegistry],
        network_identity,
        child_agent_inventory: child_agent_inventory_for(
            false,
            device,
            entry.trust_state.clone(),
            LanCanonicalHouseholdRouteState::LocalNetwork,
        ),
        policy_target_surfaces: surfaces_for(true),
    }
}
