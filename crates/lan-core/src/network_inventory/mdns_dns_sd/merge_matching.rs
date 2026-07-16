use std::collections::{HashMap, HashSet};

use super::super::LanNetworkInventoryDevice;
use super::merge_device::{
    apply_mdns_service_instance, ensure_mdns_scan_source, mdns_network_inventory_device,
};
use super::MdnsDnsSdServiceInstance;
use super::MdnsServiceIndexes;
use super::{mdns_address_key, mdns_hostname_key, mdns_service_instance_key};

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
    let matching_instances = indexes.service_by_hostname.get(&hostname_key)?.to_vec();
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
