use ocentra_lan_core::network_inventory::LanNetworkInventoryDevice;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanDiscoveryEventKind, LanDiscoveryEventRow,
};

use super::event_row::{
    discovery_event_row, keyed_discovery_event_id, push_discovery_event_row,
    reachability_discovery_event_id,
};
use super::{current_scan_snapshot, scan_session_id_for_result, scan_session_key};
use crate::lan_pairing_browser_add_device_state::physical_lan_scan::LanNetworkDeviceScanResult;
use crate::time::timestamp_now;

const LAN_DISCOVERY_DEVICE_FOUND_EVENT_PREFIX: &str = "lan-discovery-device-found-";
const LAN_DISCOVERY_DEVICE_UPDATED_EVENT_PREFIX: &str = "lan-discovery-device-updated-";
const LAN_DISCOVERY_DEVICE_ONLINE_SEGMENT: &str = "device-online";
const LAN_DISCOVERY_DEVICE_OFFLINE_SEGMENT: &str = "device-offline";
const LAN_DISCOVERY_AGENT_DISCOVERED_EVENT_PREFIX: &str = "lan-discovery-agent-discovered-";
const LAN_DISCOVERY_DEVICE_FOUND_SUMMARY_PREFIX: &str = "Discovered ";
const LAN_DISCOVERY_DEVICE_UPDATED_SUMMARY_PREFIX: &str = "Updated ";
const LAN_DISCOVERY_DEVICE_UPDATED_SUMMARY_SUFFIX: &str = " network identity";
const LAN_DISCOVERY_AGENT_DISCOVERED_SUMMARY_PREFIX: &str = "Detected agent signature on ";
const LAN_DISCOVERY_DEVICE_ONLINE_SUMMARY_SUFFIX: &str = " is online";
const LAN_DISCOVERY_DEVICE_OFFLINE_SUMMARY_SUFFIX: &str = " is offline";

pub(super) fn push_scan_device_event_rows(
    rows: &mut Vec<LanDiscoveryEventRow>,
    scan_result: &LanNetworkDeviceScanResult,
) {
    let previous_devices = scan_result
        .previous_scan_snapshot
        .as_ref()
        .map(|snapshot| snapshot.devices.as_slice())
        .unwrap_or_default();
    let scan_session_id = scan_session_id_for_result(scan_result);

    for device in &scan_result.devices {
        let context = NetworkDeviceEventContext::new(scan_result, scan_session_id.clone(), device);
        let previous_device = previous_devices
            .iter()
            .find(|previous_device| same_network_device(previous_device, device));
        push_device_found_row(rows, &context, device, previous_device.is_none());
        if let Some(previous_device) = previous_device {
            push_device_update_rows(rows, &context, previous_device, device);
        }
        push_agent_discovered_row(rows, &context, device);
    }
}

struct NetworkDeviceEventContext {
    observed_at: LanPairingText,
    device_label: LanPairingText,
    scan_key: LanPairingText,
    scan_session_id: Option<LanPairingText>,
}

impl NetworkDeviceEventContext {
    fn new(
        scan_result: &LanNetworkDeviceScanResult,
        scan_session_id: Option<LanPairingText>,
        device: &LanNetworkInventoryDevice,
    ) -> Self {
        let observed_at = scan_event_occurred_at(scan_result);
        let scan_key = scan_session_key(scan_session_id.as_ref(), observed_at.clone());
        Self {
            observed_at,
            device_label: physical_device_label(device),
            scan_key,
            scan_session_id,
        }
    }
}

fn push_device_found_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    context: &NetworkDeviceEventContext,
    device: &LanNetworkInventoryDevice,
    is_new_device: bool,
) {
    if !is_new_device {
        return;
    }
    push_discovery_event_row(
        rows,
        discovery_event_row(
            keyed_discovery_event_id(
                LAN_DISCOVERY_DEVICE_FOUND_EVENT_PREFIX.to_string().into(),
                &context.scan_key,
                &LanPairingText(device.device_id.clone()),
            ),
            LanDiscoveryEventKind::DeviceFound,
            context.observed_at.clone(),
            context.scan_session_id.clone(),
            Some(LanPairingText(device.device_id.clone())),
            None,
            LanPairingText({
                let mut summary = String::from(LAN_DISCOVERY_DEVICE_FOUND_SUMMARY_PREFIX);
                summary.push_str(&context.device_label.0);
                summary
            }),
        ),
    );
}

fn push_device_update_rows(
    rows: &mut Vec<LanDiscoveryEventRow>,
    context: &NetworkDeviceEventContext,
    previous_device: &LanNetworkInventoryDevice,
    device: &LanNetworkInventoryDevice,
) {
    if network_device_identity_changed(previous_device, device) {
        push_identity_updated_row(rows, context, device);
    }
    if previous_device.reachability != device.reachability {
        push_reachability_changed_row(rows, context, device);
    }
}

fn push_identity_updated_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    context: &NetworkDeviceEventContext,
    device: &LanNetworkInventoryDevice,
) {
    push_discovery_event_row(
        rows,
        discovery_event_row(
            keyed_discovery_event_id(
                LAN_DISCOVERY_DEVICE_UPDATED_EVENT_PREFIX.to_string().into(),
                &context.scan_key,
                &LanPairingText(device.device_id.clone()),
            ),
            LanDiscoveryEventKind::DeviceUpdated,
            context.observed_at.clone(),
            context.scan_session_id.clone(),
            Some(LanPairingText(device.device_id.clone())),
            None,
            LanPairingText({
                let mut summary = String::from(LAN_DISCOVERY_DEVICE_UPDATED_SUMMARY_PREFIX);
                summary.push_str(&context.device_label.0);
                summary.push_str(LAN_DISCOVERY_DEVICE_UPDATED_SUMMARY_SUFFIX);
                summary
            }),
        ),
    );
}

fn push_reachability_changed_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    context: &NetworkDeviceEventContext,
    device: &LanNetworkInventoryDevice,
) {
    let (kind, segment, summary) = match device.reachability {
        LanPairingDeviceReachability::Online => (
            LanDiscoveryEventKind::DeviceOnline,
            LAN_DISCOVERY_DEVICE_ONLINE_SEGMENT,
            LanPairingText({
                let mut summary = context.device_label.0.clone();
                summary.push_str(LAN_DISCOVERY_DEVICE_ONLINE_SUMMARY_SUFFIX);
                summary
            }),
        ),
        LanPairingDeviceReachability::Offline => (
            LanDiscoveryEventKind::DeviceOffline,
            LAN_DISCOVERY_DEVICE_OFFLINE_SEGMENT,
            LanPairingText({
                let mut summary = context.device_label.0.clone();
                summary.push_str(LAN_DISCOVERY_DEVICE_OFFLINE_SUMMARY_SUFFIX);
                summary
            }),
        ),
        LanPairingDeviceReachability::Stale => return,
    };
    push_discovery_event_row(
        rows,
        discovery_event_row(
            reachability_discovery_event_id(
                &segment.to_string().into(),
                &context.scan_key,
                &LanPairingText(device.device_id.clone()),
            ),
            kind,
            context.observed_at.clone(),
            context.scan_session_id.clone(),
            Some(LanPairingText(device.device_id.clone())),
            None,
            summary,
        ),
    );
}

fn push_agent_discovered_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    context: &NetworkDeviceEventContext,
    device: &LanNetworkInventoryDevice,
) {
    if device.agent_status.is_none() {
        return;
    }
    push_discovery_event_row(
        rows,
        discovery_event_row(
            keyed_discovery_event_id(
                LAN_DISCOVERY_AGENT_DISCOVERED_EVENT_PREFIX
                    .to_string()
                    .into(),
                &context.scan_key,
                &LanPairingText(device.device_id.clone()),
            ),
            LanDiscoveryEventKind::AgentDiscovered,
            context.observed_at.clone(),
            context.scan_session_id.clone(),
            Some(LanPairingText(device.device_id.clone())),
            None,
            LanPairingText({
                let mut summary = String::from(LAN_DISCOVERY_AGENT_DISCOVERED_SUMMARY_PREFIX);
                summary.push_str(&context.device_label.0);
                summary
            }),
        ),
    );
}

fn scan_event_occurred_at(scan_result: &LanNetworkDeviceScanResult) -> LanPairingText {
    current_scan_snapshot(scan_result)
        .map(|snapshot| LanPairingText(snapshot.updated_at.clone()))
        .unwrap_or_else(|| LanPairingText(timestamp_now()))
}

fn same_network_device(
    left: &LanNetworkInventoryDevice,
    right: &LanNetworkInventoryDevice,
) -> bool {
    left.device_id == right.device_id
        || same_non_empty_text(
            &LanPairingText(left.mac_address.clone()),
            &LanPairingText(right.mac_address.clone()),
        )
        || same_non_empty_text(
            &LanPairingText(left.ip_address.clone()),
            &LanPairingText(right.ip_address.clone()),
        )
}

fn network_device_identity_changed(
    previous: &LanNetworkInventoryDevice,
    current: &LanNetworkInventoryDevice,
) -> bool {
    previous.label != current.label
        || previous.platform != current.platform
        || previous.hostname != current.hostname
        || previous.network_interface != current.network_interface
        || previous.scan_sources != current.scan_sources
        || previous.used_previous_scan_hint != current.used_previous_scan_hint
}

fn physical_device_label(device: &LanNetworkInventoryDevice) -> LanPairingText {
    device
        .hostname
        .clone()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| device.label.clone())
        .into()
}

fn same_non_empty_text(left: &LanPairingText, right: &LanPairingText) -> bool {
    let left = left.0.trim();
    let right = right.0.trim();
    !left.is_empty() && !right.is_empty() && left.eq_ignore_ascii_case(right)
}
