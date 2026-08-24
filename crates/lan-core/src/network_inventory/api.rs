use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus, LanPairingNetworkMode,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanDiscoveryEvidenceSource,
};

use crate::network_inventory_hardware::{local_hardware_profile, local_network_identity};

use super::active_refresh::{scan_plan_for_identity, targeted_arp_refresh_evidence_for_identity};
use super::helpers::{
    discovered_child_device_ref, discovery_hint_sources, discovery_state_for_reachability,
};
use super::neighbor_support::{discovery_evidence_source_from_scan_source, effective_scan_sources};
use super::service_identity::{self, AllowedSnmpResponseObserver};
use super::{
    LanDiscoveryRefreshMode, LanManualInterfaceSelection, LanNetworkInventoryDevice,
    LanPassiveRuntimeLocalNetworkIdentity, LanTargetedArpRefreshEvidence,
};

pub(super) mod cancellation;

pub fn discover_lan_network_devices() -> Vec<LanNetworkInventoryDevice> {
    discover_lan_network_devices_with_hints(&[], &[])
}

pub fn passive_runtime_local_network_identity() -> LanPassiveRuntimeLocalNetworkIdentity {
    LanPassiveRuntimeLocalNetworkIdentity::from_local_network_identity(local_network_identity())
}

pub fn discover_lan_network_devices_with_hints(
    trusted_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
) -> Vec<LanNetworkInventoryDevice> {
    discover_lan_network_devices_with_hints_and_refresh_mode(
        trusted_devices,
        previous_devices,
        LanDiscoveryRefreshMode::Passive,
    )
}

pub fn discover_lan_network_devices_with_hints_and_refresh_mode(
    trusted_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
) -> Vec<LanNetworkInventoryDevice> {
    discover_lan_network_devices_with_hints_refresh_mode_and_probe_suppression(
        trusted_devices,
        previous_devices,
        refresh_mode,
        trusted_devices,
    )
}

pub fn discover_lan_network_devices_with_hints_refresh_mode_and_probe_suppression(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    probe_suppression_devices: &[LanPairingDeviceRef],
) -> Vec<LanNetworkInventoryDevice> {
    let selected_interface = service_identity_selected_interface_scope(None);
    discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression(
        identity_hint_devices,
        previous_devices,
        refresh_mode,
        identity_hint_devices,
        probe_suppression_devices,
        selected_interface.as_deref(),
    )
}

pub fn discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface_scope: Option<&str>,
) -> Vec<LanNetworkInventoryDevice> {
    discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression_and_allowed_snmp_observer(
        identity_hint_devices,
        previous_devices,
        refresh_mode,
        active_refresh_suppression_devices,
        probe_suppression_devices,
        selected_interface_scope,
        None,
    )
}

pub fn discover_lan_network_devices_with_hints_refresh_mode_and_scan_and_probe_suppression_and_allowed_snmp_observer(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    probe_suppression_devices: &[LanPairingDeviceRef],
    selected_interface_scope: Option<&str>,
    allowed_snmp_response_observer: AllowedSnmpResponseObserver<'_>,
) -> Vec<LanNetworkInventoryDevice> {
    cancellation::discover_lan_network_devices_with_cancellation(
        identity_hint_devices,
        previous_devices,
        refresh_mode,
        active_refresh_suppression_devices,
        probe_suppression_devices,
        selected_interface_scope,
        allowed_snmp_response_observer,
        None,
    )
}

pub fn service_identity_selected_interface_scope(
    selected_interface_scope: Option<&str>,
) -> Option<String> {
    selected_interface_scope
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_string)
        .or_else(|| local_network_identity().and_then(|identity| identity.network_interface))
}

pub fn plan_lan_discovery_scan(
    trusted_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
) -> super::LanDiscoveryScanPlan {
    plan_lan_discovery_scan_with_active_refresh_suppression(
        trusted_devices,
        previous_devices,
        refresh_mode,
        trusted_devices,
    )
}

pub fn plan_lan_discovery_scan_with_active_refresh_suppression(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
) -> super::LanDiscoveryScanPlan {
    let identity = local_network_identity();
    scan_plan_for_identity(
        identity.as_ref(),
        identity_hint_devices,
        previous_devices,
        refresh_mode,
        active_refresh_suppression_devices,
    )
}

pub fn plan_lan_discovery_scan_with_manual_interface_selection(
    identity_hint_devices: &[LanPairingDeviceRef],
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
    manual_interface_selection: Option<LanManualInterfaceSelection>,
) -> super::LanDiscoveryScanPlan {
    let manual_identity =
        manual_interface_selection.and_then(LanManualInterfaceSelection::into_identity);
    let automatic_identity = local_network_identity();
    let selected_identity = manual_identity.as_ref().or(automatic_identity.as_ref());
    scan_plan_for_identity(
        selected_identity,
        identity_hint_devices,
        previous_devices,
        refresh_mode,
        active_refresh_suppression_devices,
    )
}

pub fn targeted_arp_refresh_evidence_for_scan(
    previous_devices: &[LanNetworkInventoryDevice],
    refresh_mode: LanDiscoveryRefreshMode,
    active_refresh_suppression_devices: &[LanPairingDeviceRef],
) -> Vec<LanTargetedArpRefreshEvidence> {
    if refresh_mode != LanDiscoveryRefreshMode::ActiveSubnetRefresh {
        return Vec::new();
    }
    let identity = local_network_identity();
    targeted_arp_refresh_evidence_for_identity(
        identity.as_ref(),
        active_refresh_suppression_devices,
        previous_devices,
    )
}

pub fn local_agent_device_ref(local_device_id: String, platform: String) -> LanPairingDeviceRef {
    let hardware_profile = local_hardware_profile();
    let network_identity = local_network_identity();
    let hostname = hardware_profile.hostname.clone();
    let label = hostname
        .clone()
        .unwrap_or_else(|| constants::lan_pairing::LOCAL_AGENT_LABEL.to_string());
    let mut device = LanPairingDeviceRef::new(local_device_id, None, label, platform);
    device.hostname = hostname;
    if let Some(identity) = network_identity {
        device.ip_address = identity.ip_address;
        device.mac_address = identity.mac_address;
        device.network_interface = identity.network_interface;
    }
    device.agent_status = Some(constants::lan_pairing::LOCAL_AGENT_STATUS.to_string());
    device.hardware_profile = Some(hardware_profile.into_protocol_profile());
    device
}

pub fn discovery_evidence_sources_for_network_device(
    device: &LanNetworkInventoryDevice,
) -> Vec<LanDiscoveryEvidenceSource> {
    let mut sources = effective_scan_sources(device)
        .into_iter()
        .filter_map(|scan_source| discovery_evidence_source_from_scan_source(&scan_source))
        .collect::<Vec<_>>();
    if is_service_identity_probe_status(device.agent_status.as_deref())
        && !sources.contains(&LanDiscoveryEvidenceSource::ServiceIdentityProbe)
    {
        sources.push(LanDiscoveryEvidenceSource::ServiceIdentityProbe);
    }
    let mut unique_sources = Vec::new();
    for source in sources {
        if !unique_sources.contains(&source) {
            unique_sources.push(source);
        }
    }
    unique_sources
}

pub fn discovered_devices_from_network_inventory(
    network_devices: &[LanNetworkInventoryDevice],
    generated_at: &str,
) -> Vec<LanBrowserAddDeviceDiscoveryDevice> {
    network_devices
        .iter()
        .map(|network_device| LanBrowserAddDeviceDiscoveryDevice {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            discovered_at: if network_device.observed_at.is_empty() {
                generated_at.to_string()
            } else {
                network_device.observed_at.clone()
            },
            child_device: discovered_child_device_ref(network_device),
            agent_peer_id: network_device.device_id.clone(),
            pairing_id: None,
            route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            network_mode: LanPairingNetworkMode::LocalNetwork,
            reachability: network_device.reachability.clone(),
            address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
            discovery_status: LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
            discovery_state: discovery_state_for_reachability(&network_device.reachability),
            evidence_sources: discovery_evidence_sources_for_network_device(network_device),
            hint_sources: discovery_hint_sources(network_device),
            service_identity_probe_evidence: network_device.service_identity_probe_evidence.clone(),
        })
        .collect()
}

pub(crate) fn service_identity_probe_scan_source() -> &'static str {
    service_identity::service_identity_probe_scan_source()
}

pub(crate) fn is_confirmed_agent_status(status: Option<&str>) -> bool {
    service_identity::is_confirmed_agent_status(status)
}

pub(crate) fn is_service_identity_probe_status(status: Option<&str>) -> bool {
    service_identity::is_service_identity_probe_status(status)
}
