mod canonical_household_device_spine;
mod production_household_proof;
mod signed_discovery_relay_spine;

use canonical_household_device_spine::canonical_household_devices as compose_canonical_household_devices;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingProductionDiscoveryState, LanTrustedDeviceRegistryEntry,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanBrowserAddDevicePairingRequest,
    LanBrowserAddDeviceReadModel, LanBrowserAddDeviceScanSummary, LanCanonicalHouseholdDevice,
    LanDiscoveryEvidenceSource, LanHouseholdDeviceDecision, LanSelectedDeviceReadiness,
};

use crate::network_inventory::{
    is_confirmed_agent_status, is_service_identity_probe_status, service_identity_probe_scan_source,
};
use crate::read_model::{audit_check_labels, honest_non_claims, lan_discovery_source_matrix};
use production_household_proof::production_household_proof_summary;
use signed_discovery_relay_spine::signed_discovery_relay_spine_summary;

pub struct LanAddDeviceReadModelInput {
    pub generated_at: String,
    pub discovery_source:
        ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanPairingDiscoverySource,
    pub add_device_state: LanPairingProductionDiscoveryState,
    pub local_service_discovery_state: LanPairingProductionDiscoveryState,
    pub physical_household_lan_state: LanPairingProductionDiscoveryState,
    pub cloud_relay_state: LanPairingProductionDiscoveryState,
    pub discovered_devices: Vec<LanBrowserAddDeviceDiscoveryDevice>,
    pub pairing_requests: Vec<LanBrowserAddDevicePairingRequest>,
    pub trusted_device_registry: Vec<LanTrustedDeviceRegistryEntry>,
    pub household_device_decisions: Vec<LanHouseholdDeviceDecision>,
    pub trusted_device_ids: Vec<String>,
    pub revoked_device_ids: Vec<String>,
    pub selected_device_readiness: LanSelectedDeviceReadiness,
    pub controller_authority: LanPairingParentAuthority,
    pub observer_authority: LanPairingParentAuthority,
}

pub fn build_lan_add_device_read_model(
    input: LanAddDeviceReadModelInput,
) -> LanBrowserAddDeviceReadModel {
    let scan_summary = scan_summary(&input.discovered_devices);
    let canonical_household_devices = compose_canonical_household_devices(
        &input.discovered_devices,
        &input.trusted_device_registry,
        &input.household_device_decisions,
        &input.generated_at,
    );
    let production_household_proof = production_household_proof_summary(
        &input.generated_at,
        input.physical_household_lan_state.clone(),
        &scan_summary,
        &input.trusted_device_registry,
        &input.household_device_decisions,
        &input.selected_device_readiness,
    );
    let signed_discovery_relay_spine = signed_discovery_relay_spine_summary(
        &input.generated_at,
        input.physical_household_lan_state.clone(),
        &scan_summary,
        &input.trusted_device_registry,
        &input.household_device_decisions,
        &input.selected_device_readiness,
    );

    LanBrowserAddDeviceReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: input.generated_at.clone(),
        discovery_source: input.discovery_source,
        add_device_state: input.add_device_state,
        local_service_discovery_state: input.local_service_discovery_state,
        physical_household_lan_state: input.physical_household_lan_state,
        cloud_relay_state: input.cloud_relay_state,
        scan_summary: scan_summary.clone(),
        discovered_devices: input.discovered_devices,
        canonical_household_devices,
        pairing_requests: input.pairing_requests,
        trusted_device_registry: input.trusted_device_registry,
        household_device_decisions: input.household_device_decisions,
        production_household_proof: Some(production_household_proof),
        signed_discovery_relay_spine: Some(signed_discovery_relay_spine),
        lan_discovery_source_matrix: Some(lan_discovery_source_matrix(
            &input.generated_at,
            &scan_summary,
        )),
        trusted_device_ids: input.trusted_device_ids,
        revoked_device_ids: input.revoked_device_ids,
        selected_device_readiness: input.selected_device_readiness,
        controller_authority: input.controller_authority,
        observer_authority: input.observer_authority,
        route_requirement_labels: constants::lan_pairing::ROUTE_REQUIREMENTS
            .iter()
            .map(|requirement| (*requirement).to_string())
            .collect(),
        audit_check_labels: audit_check_labels(),
        honest_non_claims: honest_non_claims(),
    }
}

pub fn canonical_household_devices(
    discovered_devices: &[LanBrowserAddDeviceDiscoveryDevice],
    trusted_registry: &[LanTrustedDeviceRegistryEntry],
    household_device_decisions: &[LanHouseholdDeviceDecision],
    observed_at: &str,
) -> Vec<LanCanonicalHouseholdDevice> {
    compose_canonical_household_devices(
        discovered_devices,
        trusted_registry,
        household_device_decisions,
        observed_at,
    )
}

fn scan_summary(devices: &[LanBrowserAddDeviceDiscoveryDevice]) -> LanBrowserAddDeviceScanSummary {
    let agent_device_count = devices.iter().filter(|device| has_agent(device)).count() as u32;
    let infrastructure_device_count = devices
        .iter()
        .filter(|device| is_infrastructure(device))
        .count() as u32;
    let passive_device_count = devices
        .iter()
        .filter(|device| {
            device.discovery_status
                == ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus::NetworkNeighbor
                && !is_infrastructure(device)
        })
        .count() as u32;
    let unsupported_device_count =
        devices.iter().filter(|device| !has_agent(device)).count() as u32;

    LanBrowserAddDeviceScanSummary {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        source_labels: scan_source_labels(devices),
        scanned_device_count: devices.len() as u32,
        agent_device_count,
        passive_device_count,
        infrastructure_device_count,
        unsupported_device_count,
    }
}

fn scan_source_labels(devices: &[LanBrowserAddDeviceDiscoveryDevice]) -> Vec<String> {
    let mut labels = vec![constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE.to_string()];
    for label in devices
        .iter()
        .flat_map(|device| device.evidence_sources.iter())
        .filter_map(scan_source_label)
    {
        if !labels.iter().any(|existing| existing == label) {
            labels.push(label.to_string());
        }
    }
    if devices
        .iter()
        .any(|device| is_service_identity_probe_status(device.child_device.agent_status.as_deref()))
    {
        labels.push(service_identity_probe_scan_source().to_string());
    }
    if devices.iter().any(|device| {
        device
            .hint_sources
            .contains(&LanDiscoveryEvidenceSource::PreviousScanSnapshot)
    }) {
        labels.push(constants::lan_pairing::LAN_SCAN_SOURCE_PREVIOUS_SCAN_SNAPSHOT.to_string());
    }
    labels
}

fn scan_source_label(source: &LanDiscoveryEvidenceSource) -> Option<&'static str> {
    match source {
        LanDiscoveryEvidenceSource::LocalService => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_LOCAL_SERVICE)
        }
        LanDiscoveryEvidenceSource::WindowsNeighborTable => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_WINDOWS_NEIGHBOR)
        }
        LanDiscoveryEvidenceSource::LinuxProcNetArp => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_PROC_NET_ARP)
        }
        LanDiscoveryEvidenceSource::LinuxIpNeigh => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_LINUX_IP_NEIGH)
        }
        LanDiscoveryEvidenceSource::MacosArp => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_MACOS_ARP)
        }
        LanDiscoveryEvidenceSource::ServiceIdentityProbe => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_SERVICE_IDENTITY_PROBE)
        }
        LanDiscoveryEvidenceSource::PreviousScanSnapshot
        | LanDiscoveryEvidenceSource::DnsCache
        | LanDiscoveryEvidenceSource::Netbios
        | LanDiscoveryEvidenceSource::TrustedRegistry
        | LanDiscoveryEvidenceSource::ParentAssignment
        | LanDiscoveryEvidenceSource::ChildAgentHello
        | LanDiscoveryEvidenceSource::ChildAgentHeartbeat => None,
    }
}

fn has_agent(device: &LanBrowserAddDeviceDiscoveryDevice) -> bool {
    is_confirmed_agent_status(device.child_device.agent_status.as_deref())
}

fn is_infrastructure(device: &LanBrowserAddDeviceDiscoveryDevice) -> bool {
    device.child_device.platform == constants::lan_pairing::PLATFORM_ROUTER
}
