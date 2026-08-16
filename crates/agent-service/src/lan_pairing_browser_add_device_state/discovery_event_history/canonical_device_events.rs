use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingText, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceReadModel, LanCanonicalHouseholdDevice,
    LanCanonicalHouseholdDeviceClassification, LanDiscoveryEventKind, LanDiscoveryEventRow,
};

use super::event_row::{
    agent_confirmed_summary, discovery_event_row, evidence_found_summary, keyed_discovery_event_id,
    push_discovery_event_row,
};
use super::{latest_rfc3339_timestamp, scan_session_id_for_result, scan_session_key};
use crate::lan_pairing_browser_add_device_state::discovery_projection::serialized_enum_label;
use crate::lan_pairing_browser_add_device_state::physical_lan_scan::LanNetworkDeviceScanResult;

const LAN_DISCOVERY_EVIDENCE_FOUND_EVENT_PREFIX: &str = "lan-discovery-evidence-found-";
const LAN_DISCOVERY_AGENT_CONFIRMED_EVENT_PREFIX: &str = "lan-discovery-agent-confirmed-";
const LAN_DISCOVERY_DEVICE_OFFLINE_EVENT_PREFIX: &str = "lan-discovery-device-offline-";
const LAN_DISCOVERY_UNKNOWN_DETECTED_EVENT_PREFIX: &str = "lan-discovery-unknown-detected-";
const LAN_DISCOVERY_UNKNOWN_DETECTED_SUMMARY_PREFIX: &str = "Detected unknown LAN device ";
const LAN_DISCOVERY_DEVICE_OFFLINE_SUMMARY_SUFFIX: &str = " is offline";

pub(super) fn push_canonical_household_event_rows(
    rows: &mut Vec<LanDiscoveryEventRow>,
    scan_result: &LanNetworkDeviceScanResult,
    read_model: &LanBrowserAddDeviceReadModel,
) {
    let scan_session_id = scan_session_id_for_result(scan_result);
    for device in &read_model.canonical_household_devices {
        let observed_at = canonical_device_observed_at(device, read_model);
        let scan_key = scan_session_key(scan_session_id.as_ref(), observed_at.clone());
        let context = CanonicalDeviceEventContext {
            observed_at,
            scan_key,
            scan_session_id: scan_session_id.clone(),
        };

        push_evidence_found_rows(rows, &context, device);
        push_unknown_detected_row(rows, &context, device);
        push_agent_confirmed_row(rows, &context, device);
        push_device_offline_row(rows, &context, device);
    }
}

struct CanonicalDeviceEventContext {
    observed_at: LanPairingText,
    scan_key: LanPairingText,
    scan_session_id: Option<LanPairingText>,
}

fn push_evidence_found_rows(
    rows: &mut Vec<LanDiscoveryEventRow>,
    context: &CanonicalDeviceEventContext,
    device: &LanCanonicalHouseholdDevice,
) {
    for record in &device.network_identity.evidence_records {
        let evidence_id = LanPairingText(record.evidence_id.clone());
        push_discovery_event_row(
            rows,
            discovery_event_row(
                keyed_discovery_event_id(
                    LAN_DISCOVERY_EVIDENCE_FOUND_EVENT_PREFIX.to_string().into(),
                    &context.scan_key,
                    &evidence_id,
                ),
                LanDiscoveryEventKind::EvidenceFound,
                LanPairingText(record.last_seen_at.clone()),
                context.scan_session_id.clone(),
                Some(LanPairingText(device.canonical_device_id.clone())),
                Some(evidence_id),
                evidence_found_summary(
                    &serialized_enum_label(&record.evidence_kind),
                    &LanPairingText(device.display_name.clone()),
                ),
            ),
        );
    }
}

fn push_unknown_detected_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    context: &CanonicalDeviceEventContext,
    device: &LanCanonicalHouseholdDevice,
) {
    if device.classification != LanCanonicalHouseholdDeviceClassification::UnknownLanDevice {
        return;
    }
    push_discovery_event_row(
        rows,
        discovery_event_row(
            keyed_discovery_event_id(
                LAN_DISCOVERY_UNKNOWN_DETECTED_EVENT_PREFIX
                    .to_string()
                    .into(),
                &context.scan_key,
                &LanPairingText(device.canonical_device_id.clone()),
            ),
            LanDiscoveryEventKind::UnknownDetected,
            context.observed_at.clone(),
            context.scan_session_id.clone(),
            Some(LanPairingText(device.canonical_device_id.clone())),
            None,
            LanPairingText({
                let mut summary = String::from(LAN_DISCOVERY_UNKNOWN_DETECTED_SUMMARY_PREFIX);
                summary.push_str(&device.display_name);
                summary
            }),
        ),
    );
}

fn push_agent_confirmed_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    context: &CanonicalDeviceEventContext,
    device: &LanCanonicalHouseholdDevice,
) {
    if !canonical_device_has_agent_confirmation(device) {
        return;
    }
    push_discovery_event_row(
        rows,
        discovery_event_row(
            keyed_discovery_event_id(
                LAN_DISCOVERY_AGENT_CONFIRMED_EVENT_PREFIX
                    .to_string()
                    .into(),
                &context.scan_key,
                &LanPairingText(device.canonical_device_id.clone()),
            ),
            LanDiscoveryEventKind::AgentConfirmed,
            context.observed_at.clone(),
            context.scan_session_id.clone(),
            Some(LanPairingText(device.canonical_device_id.clone())),
            None,
            agent_confirmed_summary(&LanPairingText(device.display_name.clone())),
        ),
    );
}

fn canonical_device_has_agent_confirmation(device: &LanCanonicalHouseholdDevice) -> bool {
    device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent
        && device.child_agent_inventory.is_some()
        && (device.trust_state == LanPairingTrustState::Paired
            || !device.network_identity.evidence_records.is_empty())
}

fn push_device_offline_row(
    rows: &mut Vec<LanDiscoveryEventRow>,
    context: &CanonicalDeviceEventContext,
    device: &LanCanonicalHouseholdDevice,
) {
    if device.network_identity.reachability != LanPairingDeviceReachability::Offline {
        return;
    }
    let offline_at = device
        .network_identity
        .offline_at
        .clone()
        .unwrap_or_else(|| context.observed_at.0.clone());
    push_discovery_event_row(
        rows,
        discovery_event_row(
            keyed_discovery_event_id(
                LAN_DISCOVERY_DEVICE_OFFLINE_EVENT_PREFIX.to_string().into(),
                &context.scan_key,
                &LanPairingText(device.canonical_device_id.clone()),
            ),
            LanDiscoveryEventKind::DeviceOffline,
            LanPairingText(offline_at),
            context.scan_session_id.clone(),
            Some(LanPairingText(device.canonical_device_id.clone())),
            None,
            LanPairingText({
                let mut summary = device.display_name.clone();
                summary.push_str(LAN_DISCOVERY_DEVICE_OFFLINE_SUMMARY_SUFFIX);
                summary
            }),
        ),
    );
}

fn canonical_device_observed_at(
    device: &LanCanonicalHouseholdDevice,
    read_model: &LanBrowserAddDeviceReadModel,
) -> LanPairingText {
    latest_rfc3339_timestamp(
        device
            .network_identity
            .evidence_records
            .iter()
            .map(|record| LanPairingText(record.last_seen_at.clone()))
            .chain(
                device
                    .network_identity
                    .offline_at
                    .iter()
                    .cloned()
                    .map(LanPairingText),
            )
            .chain(
                device
                    .network_identity
                    .stale_at
                    .iter()
                    .cloned()
                    .map(LanPairingText),
            ),
    )
    .unwrap_or_else(|| LanPairingText(read_model.generated_at.clone()))
}
