use std::time::Duration;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanBrowserAddDeviceScanSummary, LanDiscoveryEvidenceSource,
};

use crate::network_inventory::api::{
    is_confirmed_agent_status, is_service_identity_probe_status, service_identity_probe_scan_source,
};
use crate::network_inventory::passive_discovery::collection::current_platform_local_neighbor_collection_summaries;

pub(super) fn scan_summary(
    devices: &[LanBrowserAddDeviceDiscoveryDevice],
) -> LanBrowserAddDeviceScanSummary {
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
        passive_local_neighbor_collection_summaries:
            current_platform_local_neighbor_collection_summaries(Duration::from_millis(250)),
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
        && !labels
            .iter()
            .any(|label| label == service_identity_probe_scan_source())
    {
        labels.push(service_identity_probe_scan_source().to_string());
    }
    if devices.iter().any(|device| {
        device.service_identity_probe_evidence.iter().any(|evidence| {
            matches!(
                evidence.evidence_kind,
                ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind::SnmpSysDescr
                    | ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanServiceIdentityProbeEvidenceKind::SnmpSysName
            )
        })
    }) && !labels
        .iter()
        .any(|label| label == constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE)
    {
        labels.push(constants::lan_pairing::LAN_SCAN_SOURCE_ALLOWED_SNMP_RESPONSE.to_string());
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
        LanDiscoveryEvidenceSource::MdnsDnsSdQuery => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD)
        }
        LanDiscoveryEvidenceSource::SsdpUpnpQuery => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP)
        }
        LanDiscoveryEvidenceSource::DnsCache => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_DNS_CACHE)
        }
        LanDiscoveryEvidenceSource::Netbios => {
            Some(constants::lan_pairing::LAN_SCAN_SOURCE_NETBIOS)
        }
        LanDiscoveryEvidenceSource::Llmnr => Some(constants::lan_pairing::LAN_SCAN_SOURCE_LLMNR),
        LanDiscoveryEvidenceSource::PreviousScanSnapshot
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
