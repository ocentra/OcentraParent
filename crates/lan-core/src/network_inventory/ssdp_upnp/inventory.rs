use std::net::SocketAddr;
use std::sync::atomic::AtomicBool;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;

use super::http::parse_allowed_http_location;
use super::http::text::{parse_udn, short_ssdp_label};
use super::merge::{merge_ssdp_network_inventory_device, ssdp_hint_evidence};
use super::{discover_ssdp_upnp_records_at, LanNetworkInventoryDevice, SsdpDiscoveryRecord};

pub(super) fn enrich_ssdp_upnp_devices(
    devices: &mut Vec<LanNetworkInventoryDevice>,
    selected_interface: Option<&str>,
) {
    enrich_ssdp_upnp_devices_with_cancellation(devices, selected_interface, None);
}

pub(crate) fn enrich_ssdp_upnp_devices_with_cancellation(
    devices: &mut Vec<LanNetworkInventoryDevice>,
    selected_interface: Option<&str>,
    cancellation: Option<&AtomicBool>,
) {
    if cancellation.is_some_and(|value| value.load(std::sync::atomic::Ordering::Acquire)) {
        return;
    }
    if let Ok(records) = super::discovery::discover_ssdp_upnp_devices_with_cancellation(
        "ssdp:all",
        SocketAddr::new(
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(239, 255, 255, 250)),
            1900,
        ),
        std::time::Duration::from_millis(super::SSDP_DISCOVERY_TIMEOUT_MS),
        1,
        std::time::Duration::from_millis(super::SSDP_DISCOVERY_TIMEOUT_MS),
        cancellation,
    ) {
        merge_ssdp_records(devices, records, selected_interface);
    }
}

pub(super) fn enrich_ssdp_upnp_devices_for_target(
    devices: &mut Vec<LanNetworkInventoryDevice>,
    target: SocketAddr,
) {
    if let Ok(records) = discover_ssdp_upnp_records_at(target) {
        merge_ssdp_records(devices, records, None);
    }
}

pub(super) fn ssdp_network_inventory_device(
    record: &SsdpDiscoveryRecord,
    selected_interface: Option<&str>,
) -> Option<LanNetworkInventoryDevice> {
    let allowed_location = parse_allowed_http_location(&record.response.location).ok()?;
    let label = record
        .description
        .as_ref()
        .map(|description| description.friendly_name.clone())
        .or_else(|| short_ssdp_label(record.response.device_type.as_deref()))
        .or_else(|| short_ssdp_label(Some(record.response.search_target.as_str())))
        .unwrap_or_else(|| record.response.usn.clone());
    let platform = if record.response.infrastructure {
        constants::lan_pairing::PLATFORM_ROUTER.to_string()
    } else {
        record
            .response
            .device_type
            .as_ref()
            .and_then(|device_type| short_ssdp_label(Some(device_type)))
            .unwrap_or_else(|| constants::lan_pairing::PLATFORM_UNKNOWN.to_string())
    };
    let device_id = record
        .response
        .udn
        .clone()
        .or_else(|| parse_udn(&record.response.usn))
        .unwrap_or_else(|| record.response.usn.clone());
    Some(LanNetworkInventoryDevice {
        device_id,
        label,
        platform,
        ip_address: allowed_location.addr.ip().to_string(),
        mac_address: String::new(),
        hostname: None,
        network_interface: selected_interface.map(str::to_string),
        observed_at: String::new(),
        reachability: LanPairingDeviceReachability::Online,
        agent_status: None,
        scan_sources: vec![constants::lan_pairing::LAN_SCAN_SOURCE_SSDP_UPNP.to_string()],
        used_previous_scan_hint: false,
        service_identity_probe_evidence: ssdp_hint_evidence(record, selected_interface),
    })
}

fn merge_ssdp_records(
    devices: &mut Vec<LanNetworkInventoryDevice>,
    records: Vec<SsdpDiscoveryRecord>,
    selected_interface: Option<&str>,
) {
    for record in records {
        let Some(mut device) = ssdp_network_inventory_device(&record, selected_interface) else {
            continue;
        };
        let Some(existing) = devices.iter_mut().find(|existing| {
            existing.device_id == device.device_id
                || existing.ip_address.eq_ignore_ascii_case(&device.ip_address)
        }) else {
            devices.push(device);
            continue;
        };
        merge_ssdp_network_inventory_device(existing, &mut device);
    }
}
