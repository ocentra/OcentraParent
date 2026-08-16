use std::collections::{HashMap, HashSet};
use std::net::IpAddr;
use std::str::FromStr;

use super::super::neighbor_support::normalize_neighbor_hostname;
use super::super::LanNetworkInventoryDevice;
use super::{MdnsDnsSdDiscovery, MdnsDnsSdServiceInstance};

#[path = "merge_device.rs"]
mod merge_device;
#[path = "merge_matching.rs"]
mod merge_matching;

use merge_matching::{
    append_unmatched_mdns_devices, device_hostname_counts, merge_matching_mdns_instances,
};

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
