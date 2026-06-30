use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::str::FromStr;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};

use super::super::neighbor_support::normalize_neighbor_hostname;
use super::super::LanNetworkInventoryDevice;
use super::text::{display_name_from_instance_name, sanitize_mdns_text};
use super::{MdnsDnsSdDiscovery, MdnsDnsSdServiceInstance};

pub fn merge_mdns_dns_sd_discovery(
    devices: &mut Vec<LanNetworkInventoryDevice>,
    discovery: &MdnsDnsSdDiscovery,
) {
    merge_mdns_dns_sd_discovery_with_selected_interface(devices, discovery, None);
}

pub fn merge_mdns_dns_sd_discovery_with_selected_interface(
    devices: &mut Vec<LanNetworkInventoryDevice>,
    discovery: &MdnsDnsSdDiscovery,
    selected_interface: Option<&str>,
) {
    let indexes = MdnsServiceIndexes::from_discovery(discovery);
    let device_hostname_counts = device_hostname_counts(devices);
    let mut matched_addresses = HashSet::new();
    let mut matched_service_instances = HashSet::new();
    merge_matching_mdns_instances(
        devices,
        &indexes,
        &device_hostname_counts,
        &mut matched_addresses,
        &mut matched_service_instances,
    );
    append_unmatched_mdns_devices(
        devices,
        indexes.service_by_address,
        &matched_addresses,
        &matched_service_instances,
        selected_interface,
        discovery.observed_at.as_str(),
    );
}

pub struct MdnsServiceIndexes<'a> {
    service_by_address: HashMap<String, Vec<&'a MdnsDnsSdServiceInstance>>,
    service_by_hostname: HashMap<String, Vec<&'a MdnsDnsSdServiceInstance>>,
}

impl<'a> MdnsServiceIndexes<'a> {
    fn from_discovery(discovery: &'a MdnsDnsSdDiscovery) -> Self {
        let mut service_by_address = HashMap::new();
        let mut service_by_hostname = HashMap::new();
        for service_instance in &discovery.service_instances {
            for address in &service_instance.addresses {
                service_by_address
                    .entry(mdns_address_key(address))
                    .or_insert_with(Vec::new)
                    .push(service_instance);
            }
            if let Some(hostname_key) =
                mdns_hostname_key(service_instance.target_hostname.as_deref())
            {
                service_by_hostname
                    .entry(hostname_key)
                    .or_insert_with(Vec::new)
                    .push(service_instance);
            }
        }
        Self {
            service_by_address,
            service_by_hostname,
        }
    }
}

pub fn device_hostname_counts(devices: &[LanNetworkInventoryDevice]) -> HashMap<String, usize> {
    let mut counts = HashMap::<String, usize>::new();
    for device in devices {
        if let Some(hostname_key) = mdns_hostname_key(device.hostname.as_deref()) {
            *counts.entry(hostname_key).or_insert(0) += 1;
        }
    }
    counts
}

pub fn merge_matching_mdns_instances(
    devices: &mut [LanNetworkInventoryDevice],
    indexes: &MdnsServiceIndexes<'_>,
    device_hostname_counts: &HashMap<String, usize>,
    matched_addresses: &mut HashSet<String>,
    matched_service_instances: &mut HashSet<(String, String)>,
) {
    for device in devices {
        let device_address_key = mdns_address_key(&device.ip_address);
        let direct_address_match = indexes.service_by_address.get(&device_address_key).cloned();
        let used_direct_address_match = direct_address_match.is_some();
        let service_instances = direct_address_match.or_else(|| {
            matching_mdns_instances_for_hostname(device, indexes, device_hostname_counts)
        });
        let Some(service_instances) = service_instances else {
            continue;
        };
        if used_direct_address_match {
            matched_addresses.insert(device_address_key);
        }
        ensure_mdns_scan_source(device);
        for service_instance in service_instances {
            matched_service_instances.insert(mdns_service_instance_key(service_instance));
            apply_mdns_service_instance(device, service_instance);
        }
    }
}

pub fn matching_mdns_instances_for_hostname<'a>(
    device: &LanNetworkInventoryDevice,
    indexes: &'a MdnsServiceIndexes<'a>,
    device_hostname_counts: &HashMap<String, usize>,
) -> Option<Vec<&'a MdnsDnsSdServiceInstance>> {
    let hostname_key = mdns_hostname_key(device.hostname.as_deref())?;
    if device_hostname_counts.get(&hostname_key).copied() != Some(1) {
        return None;
    }
    let matching_instances = indexes
        .service_by_hostname
        .get(&hostname_key)?
        .iter()
        .copied()
        .collect::<Vec<_>>();
    if matching_instances.is_empty() {
        None
    } else {
        Some(matching_instances)
    }
}

pub fn append_unmatched_mdns_devices(
    devices: &mut Vec<LanNetworkInventoryDevice>,
    service_by_address: HashMap<String, Vec<&MdnsDnsSdServiceInstance>>,
    matched_addresses: &HashSet<String>,
    matched_service_instances: &HashSet<(String, String)>,
    selected_interface: Option<&str>,
    observed_at: &str,
) {
    for (address, service_instances) in service_by_address {
        if matched_addresses.contains(&address) {
            continue;
        }
        let unmatched_service_instances = service_instances
            .into_iter()
            .filter(|service_instance| {
                !matched_service_instances.contains(&mdns_service_instance_key(service_instance))
            })
            .collect::<Vec<_>>();
        if let Some(device) = mdns_network_inventory_device(
            &address,
            &unmatched_service_instances,
            selected_interface,
            observed_at,
        ) {
            devices.push(device);
        }
    }
}

pub fn mdns_address_key(value: &str) -> String {
    IpAddr::from_str(value)
        .map(|address| address.to_string())
        .unwrap_or_else(|_| value.trim().to_ascii_lowercase())
}

pub fn mdns_hostname_key(value: Option<&str>) -> Option<String> {
    value
        .and_then(normalize_neighbor_hostname)
        .map(|hostname| hostname.to_ascii_lowercase())
}

pub fn mdns_service_instance_key(service_instance: &MdnsDnsSdServiceInstance) -> (String, String) {
    (
        service_instance.service_type.to_ascii_lowercase(),
        service_instance.instance_name.to_ascii_lowercase(),
    )
}

pub fn ensure_mdns_scan_source(device: &mut LanNetworkInventoryDevice) {
    if !device
        .scan_sources
        .iter()
        .any(|source| source == constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD)
    {
        device
            .scan_sources
            .push(constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD.to_string());
    }
}

pub fn apply_mdns_service_instance(
    device: &mut LanNetworkInventoryDevice,
    service_instance: &MdnsDnsSdServiceInstance,
) {
    push_mdns_hint(
        &mut device.service_identity_probe_evidence,
        LanServiceIdentityProbeEvidenceKind::MdnsServiceType,
        service_instance.service_type.as_str(),
        device.network_interface.clone(),
    );
    push_mdns_hint(
        &mut device.service_identity_probe_evidence,
        LanServiceIdentityProbeEvidenceKind::MdnsInstanceName,
        service_instance.instance_name.as_str(),
        device.network_interface.clone(),
    );

    if device.hostname.is_none() {
        if let Some(target_hostname) = service_instance.target_hostname.as_ref() {
            if let Some(hostname) = normalize_neighbor_hostname(target_hostname) {
                device.hostname = Some(hostname);
            }
        }
    }

    if should_replace_device_label(&device.label) {
        if let Some(display_name) = service_instance.display_name.as_ref() {
            if let Some(display_name) = sanitize_mdns_text(display_name) {
                device.label = display_name;
            }
        }
    }
}

pub fn should_replace_device_label(label: &str) -> bool {
    label.is_empty() || label.starts_with(constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX)
}

pub fn mdns_network_inventory_device(
    address: &str,
    service_instances: &[&MdnsDnsSdServiceInstance],
    selected_interface: Option<&str>,
    observed_at: &str,
) -> Option<LanNetworkInventoryDevice> {
    let service_instance = service_instances.first().copied()?;
    let label = mdns_device_label(address, service_instance);
    let hostname = service_instance
        .target_hostname
        .as_deref()
        .and_then(normalize_neighbor_hostname);
    let mut device = LanNetworkInventoryDevice {
        device_id: mdns_network_inventory_device_id(service_instance, address),
        label,
        platform: constants::lan_pairing::PLATFORM_UNKNOWN.to_string(),
        ip_address: address.to_string(),
        mac_address: String::new(),
        hostname,
        network_interface: selected_interface.map(str::to_string),
        observed_at: observed_at.to_string(),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_MDNS_DNS_SD.to_string()],
        used_previous_scan_hint: false,
        service_identity_probe_evidence: Vec::new(),
    };
    for service_instance in service_instances {
        apply_mdns_service_instance(&mut device, service_instance);
    }
    Some(device)
}

pub fn mdns_device_label(address: &str, service_instance: &MdnsDnsSdServiceInstance) -> String {
    service_instance
        .display_name
        .as_deref()
        .and_then(sanitize_mdns_text)
        .or_else(|| {
            service_instance
                .target_hostname
                .as_deref()
                .and_then(sanitize_mdns_text)
        })
        .unwrap_or_else(|| {
            format!(
                "{}{}",
                constants::lan_pairing::NETWORK_NEIGHBOR_LABEL_PREFIX,
                address
            )
        })
}

pub fn mdns_network_inventory_device_id(
    service_instance: &MdnsDnsSdServiceInstance,
    address: &str,
) -> String {
    let mut id = String::from(constants::lan_pairing::NETWORK_NEIGHBOR_DEVICE_PREFIX);
    id.push_str("mdns-");
    id.push_str(&compact_mdns_identifier(&format!(
        "{}-{}-{}",
        service_instance.service_type, service_instance.instance_name, address
    )));
    id
}

pub fn compact_mdns_identifier(value: &str) -> String {
    let compacted = value
        .chars()
        .filter(|character| character.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect::<String>();
    if compacted.is_empty() {
        "unknown".to_string()
    } else {
        compacted
    }
}

pub fn push_mdns_hint(
    records: &mut Vec<LanServiceIdentityProbeEvidence>,
    evidence_kind: LanServiceIdentityProbeEvidenceKind,
    value: &str,
    selected_interface: Option<String>,
) {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return;
    }
    if let Some(existing) = records.iter_mut().find(|record| {
        record.evidence_kind == evidence_kind && record.value.eq_ignore_ascii_case(trimmed)
    }) {
        if existing.selected_interface.is_none() {
            existing.selected_interface = selected_interface;
        }
        return;
    }
    records.push(LanServiceIdentityProbeEvidence {
        evidence_kind,
        value: trimmed.to_string(),
        selected_interface,
    });
}
