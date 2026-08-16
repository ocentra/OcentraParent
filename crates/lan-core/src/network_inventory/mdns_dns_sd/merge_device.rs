use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanServiceIdentityProbeEvidence, LanServiceIdentityProbeEvidenceKind,
};

use super::super::super::neighbor_support::normalize_neighbor_hostname;
use super::super::text::sanitize_mdns_text;
use super::super::LanNetworkInventoryDevice;
use super::compact_mdns_identifier;
use super::MdnsDnsSdServiceInstance;

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

    populate_hostname_from_service_instance(device, service_instance);
    populate_label_from_service_instance(device, service_instance);
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

fn populate_hostname_from_service_instance(
    device: &mut LanNetworkInventoryDevice,
    service_instance: &MdnsDnsSdServiceInstance,
) {
    if device.hostname.is_none() {
        if let Some(target_hostname) = service_instance.target_hostname.as_ref() {
            if let Some(hostname) = normalize_neighbor_hostname(target_hostname) {
                device.hostname = Some(hostname);
            }
        }
    }
}

fn populate_label_from_service_instance(
    device: &mut LanNetworkInventoryDevice,
    service_instance: &MdnsDnsSdServiceInstance,
) {
    if should_replace_device_label(&device.label) {
        if let Some(display_name) = service_instance.display_name.as_ref() {
            if let Some(display_name) = sanitize_mdns_text(display_name) {
                device.label = display_name;
            }
        }
    }
}
