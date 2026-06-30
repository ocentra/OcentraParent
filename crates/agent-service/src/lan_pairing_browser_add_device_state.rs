use ocentra_lan_core::network_inventory::{
    discovery_evidence_sources_for_network_device, local_agent_device_ref,
    LanNetworkInventoryDevice,
};
use ocentra_lan_core::read_model_builder::{
    build_lan_add_device_read_model, LanAddDeviceReadModelInput,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceReachability;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDeviceRef;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingDiscoveryRuntimeStatus;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingNetworkMode;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingProductionDiscoveryState;
use ocentra_parent_agent_protocol::lan_pairing::LanPairingTrustState;
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDevicePairingRequest;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDeviceClassification;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventHistory;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventHistoryState;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventKind;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEventRow;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanHouseholdDeviceDecision;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanPairingDiscoverySource;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

#[path = "lan_pairing_browser_add_device_state/discovery_projection.rs"]
pub(crate) mod discovery_projection;
#[path = "lan_pairing_browser_add_device_state/physical_lan_scan.rs"]
pub(crate) mod physical_lan_scan;
#[path = "lan_pairing_browser_add_device_state/scan_history.rs"]
pub(crate) mod scan_history;

use crate::lan_pairing_browser_add_device_scan::{push_if_absent, same_physical_network_device};
use crate::{lan_pairing::LanPairingRuntime, time::timestamp_now};

use self::discovery_projection::{
    discovery_state_for, expired_pairing_count, pairing_request_state, pending_pairing_count,
    physical_household_lan_state, selected_device_readiness, serialized_enum_label,
};
use self::physical_lan_scan::{
    network_device_scan_result_for_command, refresh_network_device_scan_history,
    LanNetworkDeviceScanResult,
};

#[cfg(any(target_os = "macos", target_os = "ios"))]
const APPLE_LAN_DISCOVERY_MANUAL_REQUIRED: bool = true;
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
const APPLE_LAN_DISCOVERY_MANUAL_REQUIRED: bool = false;
const LAN_DISCOVERY_INTERFACE_CHANGED_EVENT_PREFIX: &str = "lan-discovery-interface-changed-";
const LAN_DISCOVERY_SCAN_STARTED_SUMMARY: &str = "LAN scan started";
const LAN_DISCOVERY_SCAN_FINISHED_SUMMARY_PREFIX: &str = "LAN scan finished with ";
const LAN_DISCOVERY_SCAN_FINISHED_SUMMARY_SUFFIX: &str = " visible devices";
const LAN_DISCOVERY_DEVICE_UPDATED_EVENT_PREFIX: &str = "lan-discovery-device-updated-";
const LAN_DISCOVERY_EVENT_PREFIX: &str = "lan-discovery-";
const LAN_DISCOVERY_DEVICE_ONLINE_SEGMENT: &str = "device-online";
const LAN_DISCOVERY_DEVICE_OFFLINE_SEGMENT: &str = "device-offline";
const LAN_DISCOVERY_AGENT_DISCOVERED_EVENT_PREFIX: &str = "lan-discovery-agent-discovered-";
const LAN_DISCOVERY_EVIDENCE_FOUND_EVENT_PREFIX: &str = "lan-discovery-evidence-found-";
const LAN_DISCOVERY_AGENT_CONFIRMED_EVENT_PREFIX: &str = "lan-discovery-agent-confirmed-";
const LAN_DISCOVERY_DEVICE_OFFLINE_EVENT_PREFIX: &str = "lan-discovery-device-offline-";
const LAN_DISCOVERY_UNKNOWN_DETECTED_EVENT_PREFIX: &str = "lan-discovery-unknown-detected-";
const LAN_DISCOVERY_OBSERVED_SUMMARY_PREFIX: &str = "Observed ";
const LAN_DISCOVERY_EVIDENCE_FOR_SEPARATOR: &str = " evidence for ";
const LAN_DISCOVERY_AGENT_CONFIRMED_SUMMARY_PREFIX: &str = "Confirmed child agent inventory for ";
const SCAN_SESSION_KEY_REPLACEMENT: &str = "-";

pub(crate) fn refresh_browser_discovery_scan_history(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
) {
    let _ = refresh_network_device_scan_history(runtime, command);
}

pub(crate) fn browser_add_device_pairs(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    discovery_state: &str,
) -> Vec<(&'static str, LogFieldValue)> {
    let model = browser_add_device_read_model(runtime, command, discovery_state);
    let read_model_json = serde_json::to_string(&model).unwrap_or_default();
    vec![
        (
            constants::field::LAN_ADD_DEVICE_READ_MODEL,
            LogFieldValue::String(read_model_json),
        ),
        (
            constants::field::LAN_ADD_DEVICE_STATE,
            LogFieldValue::String(serialized_enum_label(&model.add_device_state)),
        ),
        (
            constants::field::LAN_DISCOVERY_SOURCE,
            LogFieldValue::String(serialized_enum_label(&model.discovery_source)),
        ),
        (
            constants::field::LAN_LOCAL_SERVICE_DISCOVERY_STATE,
            LogFieldValue::String(serialized_enum_label(&model.local_service_discovery_state)),
        ),
        (
            constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE,
            LogFieldValue::String(serialized_enum_label(&model.physical_household_lan_state)),
        ),
        (
            constants::field::LAN_CLOUD_RELAY_STATE,
            LogFieldValue::String(serialized_enum_label(&model.cloud_relay_state)),
        ),
        (
            constants::field::LAN_SELECTED_DEVICE_READY,
            LogFieldValue::Boolean(model.selected_device_readiness.ready_for_control),
        ),
        (
            constants::field::LAN_PENDING_PAIRING_COUNT,
            LogFieldValue::Number(pending_pairing_count(&model) as f64),
        ),
        (
            constants::field::LAN_EXPIRED_PAIRING_COUNT,
            LogFieldValue::Number(expired_pairing_count(&model) as f64),
        ),
        (
            constants::field::LAN_ROUTE_REQUIREMENT_LABELS,
            LogFieldValue::String(
                model
                    .route_requirement_labels
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
        (
            constants::field::LAN_AUDIT_CHECK_LABELS,
            LogFieldValue::String(
                model
                    .audit_check_labels
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
        (
            constants::field::LAN_HONEST_NON_CLAIMS,
            LogFieldValue::String(
                model
                    .honest_non_claims
                    .join(&constants::delimiter::LIST.to_string()),
            ),
        ),
    ]
}

pub(crate) fn browser_add_device_read_model(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    discovery_state: &str,
) -> LanBrowserAddDeviceReadModel {
    let generated_at = timestamp_now();
    let selected = runtime.selected_target();
    let trusted_device_registry = trusted_device_registry(runtime);
    let household_device_decisions = household_device_decisions(runtime);
    let scan_result = network_device_scan_result_for_command(runtime, command);
    let platform_data_available = platform_data_available_for_scan_result(&scan_result);
    let network_devices = scan_result.devices.clone();
    let has_network_devices = !network_devices.is_empty();
    let discovery_source = if has_network_devices {
        LanPairingDiscoverySource::PhysicalHouseholdLan
    } else {
        LanPairingDiscoverySource::LocalService
    };
    let discovered_devices = discovered_devices(
        runtime,
        command,
        discovery_state,
        &generated_at,
        &network_devices,
    );
    let physical_household_lan_state = physical_household_lan_state(has_network_devices);
    let selected_device_readiness = selected_device_readiness(selected);

    let mut model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: generated_at.clone(),
        discovery_source,
        service_data_available: true,
        platform_data_available,
        add_device_state: discovery_state_for(discovery_state),
        local_service_discovery_state: discovery_state_for(discovery_state),
        physical_household_lan_state,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices,
        pairing_requests: pairing_requests(runtime, &generated_at),
        trusted_device_registry,
        household_device_decisions,
        trusted_device_ids: runtime.trusted_device_ids(),
        revoked_device_ids: runtime.revoked_device_ids(),
        selected_device_readiness,
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    });
    let current_canonical_household_devices = model.canonical_household_devices.clone();
    persist_known_household_devices(runtime, &current_canonical_household_devices);
    model.canonical_household_devices = merged_known_household_devices_for_read_model(
        runtime,
        &current_canonical_household_devices,
        &generated_at,
    );
    model.discovery_event_history = discovery_event_history(&scan_result, &model);
    model
}

fn platform_data_available_for_scan_result(scan_result: &LanNetworkDeviceScanResult) -> bool {
    platform_data_available_for_scan_result_with_manual_required_override(
        scan_result,
        apple_lan_discovery_is_manual_required(),
    )
}

pub(crate) fn platform_data_available_for_scan_result_with_manual_required_override(
    scan_result: &LanNetworkDeviceScanResult,
    manual_required_platform: bool,
) -> bool {
    scan_result
        .current_scan_snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.metadata.as_ref())
        .and_then(|metadata| metadata.scan_plan.selected_interface.as_ref())
        .is_some()
        || manual_required_platform
}

fn apple_lan_discovery_is_manual_required() -> bool {
    APPLE_LAN_DISCOVERY_MANUAL_REQUIRED
}

fn discovered_devices(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    discovery_state: &str,
    generated_at: &str,
    network_devices: &[LanNetworkInventoryDevice],
) -> Vec<LanBrowserAddDeviceDiscoveryDevice> {
    let mut devices = registry_discovered_devices(runtime, command, discovery_state, generated_at);
    let local_agent = local_agent_discovery_device(command, discovery_state, generated_at);
    push_if_absent(&mut devices, local_agent.clone());

    for network_device in network_devices {
        if same_physical_network_device(&local_agent.child_device, network_device) {
            continue;
        }
        push_if_absent(
            &mut devices,
            network_neighbor_discovery_device(command, generated_at, network_device),
        );
    }

    devices
}

fn registry_discovered_devices(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    discovery_state: &str,
    generated_at: &str,
) -> Vec<LanBrowserAddDeviceDiscoveryDevice> {
    let reachability = runtime
        .selected_target()
        .map(|target| target.reachability)
        .unwrap_or(LanPairingDeviceReachability::Online);
    runtime
        .registry
        .lock()
        .map(|registry| {
            registry
                .entries()
                .iter()
                .map(|entry| LanBrowserAddDeviceDiscoveryDevice {
                    schema_version: constants::lan_pairing::SCHEMA_VERSION,
                    discovered_at: generated_at.to_string(),
                    child_device: entry.child_device.clone(),
                    agent_peer_id: command.source.peer_id.clone(),
                    pairing_id: Some(entry.pairing_id.clone()),
                    route_id: entry.route_id.clone(),
                    network_mode: LanPairingNetworkMode::LocalNetwork,
                    reachability: reachability.clone(),
                    address_ref: constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string(),
                    discovery_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
                    discovery_state: discovery_state_for(discovery_state),
                    evidence_sources: vec![LanDiscoveryEvidenceSource::LocalService],
                    hint_sources: Vec::new(),
                    service_identity_probe_evidence: Vec::new(),
                })
                .collect()
        })
        .unwrap_or_default()
}

fn local_agent_discovery_device(
    command: &AgentCommandEnvelope,
    discovery_state: &str,
    generated_at: &str,
) -> LanBrowserAddDeviceDiscoveryDevice {
    LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: generated_at.to_string(),
        child_device: local_agent_device_ref(
            command.target.device_id.clone(),
            command.target.platform.clone(),
        ),
        agent_peer_id: command.source.peer_id.clone(),
        pairing_id: None,
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string(),
        discovery_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
        discovery_state: discovery_state_for(discovery_state),
        evidence_sources: vec![LanDiscoveryEvidenceSource::LocalService],
        hint_sources: Vec::new(),
        service_identity_probe_evidence: Vec::new(),
    }
}

fn network_neighbor_discovery_device(
    command: &AgentCommandEnvelope,
    generated_at: &str,
    network_device: &LanNetworkInventoryDevice,
) -> LanBrowserAddDeviceDiscoveryDevice {
    LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: generated_at.to_string(),
        child_device: network_neighbor_child_device(network_device),
        agent_peer_id: command.source.peer_id.clone(),
        pairing_id: None,
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: network_device.reachability.clone(),
        address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
        discovery_status: LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
        evidence_sources: discovery_evidence_sources_for_network_device(network_device),
        hint_sources: network_neighbor_hint_sources(network_device),
        service_identity_probe_evidence: network_device.service_identity_probe_evidence.clone(),
    }
}

fn network_neighbor_hint_sources(
    network_device: &LanNetworkInventoryDevice,
) -> Vec<LanDiscoveryEvidenceSource> {
    if network_device.used_previous_scan_hint {
        vec![LanDiscoveryEvidenceSource::PreviousScanSnapshot]
    } else {
        Vec::new()
    }
}

pub(crate) fn network_neighbor_child_device(
    network_device: &LanNetworkInventoryDevice,
) -> LanPairingDeviceRef {
    let mut child_device = LanPairingDeviceRef::new(
        network_device.device_id.clone(),
        None,
        network_device.label.clone(),
        network_device.platform.clone(),
    );
    child_device.ip_address = Some(network_device.ip_address.clone());
    child_device.mac_address = trimmed_non_empty(&network_device.mac_address);
    child_device.hostname =
        Some(network_device.hostname.clone().unwrap_or_else(|| {
            constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME.to_string()
        }));
    child_device.network_interface = network_device.network_interface.clone();
    child_device
}

pub(super) fn trusted_device_registry(
    runtime: &LanPairingRuntime,
) -> Vec<ocentra_parent_agent_protocol::lan_pairing::LanTrustedDeviceRegistryEntry> {
    runtime
        .registry
        .lock()
        .map(|registry| registry.entries().to_vec())
        .unwrap_or_default()
}

pub(super) fn household_device_decisions(
    runtime: &LanPairingRuntime,
) -> Vec<LanHouseholdDeviceDecision> {
    runtime
        .registry
        .lock()
        .map(|registry| registry.household_device_decisions().to_vec())
        .unwrap_or_default()
}

pub(super) fn known_household_devices(
    runtime: &LanPairingRuntime,
) -> Vec<LanCanonicalHouseholdDevice> {
    runtime
        .registry
        .lock()
        .map(|registry| registry.known_household_devices().to_vec())
        .unwrap_or_default()
}

fn persist_known_household_devices(
    runtime: &LanPairingRuntime,
    devices: &[LanCanonicalHouseholdDevice],
) {
    let Ok(mut registry) = runtime.registry.lock() else {
        return;
    };
    if registry.merge_known_household_devices(devices.to_vec()) {
        let _ = runtime.persist_registry(&registry);
    }
}

fn merged_known_household_devices_for_read_model(
    runtime: &LanPairingRuntime,
    current_devices: &[LanCanonicalHouseholdDevice],
    observed_at: &str,
) -> Vec<LanCanonicalHouseholdDevice> {
    runtime
        .registry
        .lock()
        .map(|registry| {
            registry.known_household_devices_for_read_model(current_devices, observed_at)
        })
        .unwrap_or_else(|_| current_devices.to_vec())
}

fn discovery_event_history(
    scan_result: &LanNetworkDeviceScanResult,
    read_model: &LanBrowserAddDeviceReadModel,
) -> LanDiscoveryEventHistory {
    let rows = ordered_discovery_event_rows(scan_result, read_model);
    let latest = rows.last();
    LanDiscoveryEventHistory {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: read_model.generated_at.clone(),
        state: discovery_event_history_state(scan_result, &rows, read_model),
        latest_event_id: latest.map(|event| event.event_id.clone()),
        latest_observed_at: latest.map(|event| event.occurred_at.clone()),
        rows,
    }
}

pub(crate) fn discovery_event_history_state(
    scan_result: &LanNetworkDeviceScanResult,
    rows: &[LanDiscoveryEventRow],
    read_model: &LanBrowserAddDeviceReadModel,
) -> LanDiscoveryEventHistoryState {
    if read_model.add_device_state == LanPairingProductionDiscoveryState::Unavailable
        || read_model.physical_household_lan_state
            == LanPairingProductionDiscoveryState::Unavailable
    {
        return LanDiscoveryEventHistoryState::Unavailable;
    }
    if scan_history_snapshot_is_degraded(scan_result) {
        return LanDiscoveryEventHistoryState::Degraded;
    }
    if has_agent_offline_history_state(read_model) {
        return LanDiscoveryEventHistoryState::AgentOffline;
    }
    if has_material_discovery_event_rows(rows) {
        return LanDiscoveryEventHistoryState::Ready;
    }
    if scan_history_snapshot_is_available(scan_result) {
        return LanDiscoveryEventHistoryState::Empty;
    }
    if read_model.physical_household_lan_state == LanPairingProductionDiscoveryState::ManualRequired
    {
        return LanDiscoveryEventHistoryState::ManualRequired;
    }
    LanDiscoveryEventHistoryState::Empty
}

fn has_agent_offline_history_state(read_model: &LanBrowserAddDeviceReadModel) -> bool {
    let readiness = &read_model.selected_device_readiness;
    readiness.selected_child_device_id.is_some()
        && readiness.route_id.is_none()
        && readiness.trust_state == LanPairingTrustState::Paired
        && readiness.reachability == LanPairingDeviceReachability::Online
        && !readiness.ready_for_control
}

fn has_material_discovery_event_rows(rows: &[LanDiscoveryEventRow]) -> bool {
    rows.iter().any(|row| {
        !matches!(
            row.event_kind,
            LanDiscoveryEventKind::InterfaceChanged
                | LanDiscoveryEventKind::ScanStarted
                | LanDiscoveryEventKind::ScanFinished
        )
    })
}

fn scan_history_snapshot_is_available(scan_result: &LanNetworkDeviceScanResult) -> bool {
    current_scan_snapshot(scan_result)
        .and_then(|snapshot| snapshot.metadata.as_ref())
        .is_some()
}

fn scan_history_snapshot_is_degraded(scan_result: &LanNetworkDeviceScanResult) -> bool {
    [
        scan_result.current_scan_snapshot.as_ref(),
        scan_result.previous_scan_snapshot.as_ref(),
    ]
    .into_iter()
    .flatten()
    .any(|snapshot| snapshot.metadata.is_none())
}

pub(crate) fn ordered_discovery_event_rows(
    scan_result: &LanNetworkDeviceScanResult,
    read_model: &LanBrowserAddDeviceReadModel,
) -> Vec<LanDiscoveryEventRow> {
    let mut rows = Vec::new();
    push_scan_metadata_event_rows(&mut rows, scan_result, read_model);
    push_scan_device_event_rows(&mut rows, scan_result);
    push_canonical_household_event_rows(&mut rows, scan_result, read_model);
    rows.sort_by(|left, right| {
        left.occurred_at
            .cmp(&right.occurred_at)
            .then_with(|| left.event_id.cmp(&right.event_id))
    });
    for index in 1..rows.len() {
        let previous_event_id = rows[index - 1].event_id.clone();
        rows[index].previous_event_id = Some(previous_event_id);
    }
    rows
}

fn push_scan_metadata_event_rows(
    rows: &mut Vec<LanDiscoveryEventRow>,
    scan_result: &LanNetworkDeviceScanResult,
    read_model: &LanBrowserAddDeviceReadModel,
) {
    let previous_metadata = scan_result
        .previous_scan_snapshot
        .as_ref()
        .and_then(|snapshot| snapshot.metadata.as_ref());
    let current_snapshot = current_scan_snapshot(scan_result);
    let current_metadata = current_snapshot.and_then(|snapshot| snapshot.metadata.as_ref());
    let scan_session_id = scan_session_id_for_result(scan_result);
    let scan_time = current_snapshot
        .map(|snapshot| snapshot.updated_at.clone())
        .unwrap_or_else(|| read_model.generated_at.clone());

    if let (Some(previous_metadata), Some(current_metadata)) = (previous_metadata, current_metadata)
    {
        if previous_metadata.scan_plan.selected_interface
            != current_metadata.scan_plan.selected_interface
            || previous_metadata.scan_plan.ipv4_cidr != current_metadata.scan_plan.ipv4_cidr
        {
            let next_interface = current_metadata
                .scan_plan
                .selected_interface
                .clone()
                .unwrap_or_else(|| constants::value::EMPTY.to_string());
            push_discovery_event_row(
                rows,
                discovery_event_row(
                    interface_changed_event_id(scan_session_id.as_deref(), scan_time.as_str()),
                    LanDiscoveryEventKind::InterfaceChanged,
                    scan_time.clone(),
                    scan_session_id.clone(),
                    None,
                    None,
                    format!("LAN scan interface changed to {next_interface}"),
                ),
            );
        }
    }

    if current_metadata.is_some() && !scan_result.reused_recent_snapshot {
        let scan_key = scan_session_key(scan_session_id.as_deref(), &scan_time);
        push_discovery_event_row(
            rows,
            discovery_event_row(
                format!("lan-discovery-scan-started-{scan_key}"),
                LanDiscoveryEventKind::ScanStarted,
                scan_time.clone(),
                scan_session_id.clone(),
                None,
                None,
                LAN_DISCOVERY_SCAN_STARTED_SUMMARY.to_string(),
            ),
        );
        push_discovery_event_row(
            rows,
            discovery_event_row(
                format!("lan-discovery-scan-finished-{scan_key}"),
                LanDiscoveryEventKind::ScanFinished,
                scan_time,
                scan_session_id,
                None,
                None,
                scan_finished_summary(read_model.scan_summary.scanned_device_count),
            ),
        );
    }
}

fn push_scan_device_event_rows(
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
        let observed_at = scan_event_occurred_at(scan_result);
        let device_label = physical_device_label(device);
        let scan_key = scan_session_key(scan_session_id.as_deref(), &observed_at);
        let previous_device = previous_devices
            .iter()
            .find(|previous_device| same_network_device(previous_device, device));

        if previous_device.is_none() {
            push_discovery_event_row(
                rows,
                discovery_event_row(
                    format!("lan-discovery-device-found-{scan_key}-{}", device.device_id),
                    LanDiscoveryEventKind::DeviceFound,
                    observed_at.clone(),
                    scan_session_id.clone(),
                    Some(device.device_id.clone()),
                    None,
                    format!("Discovered {device_label}"),
                ),
            );
        }

        if let Some(previous_device) = previous_device {
            if network_device_identity_changed(previous_device, device) {
                push_discovery_event_row(
                    rows,
                    discovery_event_row(
                        keyed_discovery_event_id(
                            LAN_DISCOVERY_DEVICE_UPDATED_EVENT_PREFIX,
                            &scan_key,
                            &device.device_id,
                        ),
                        LanDiscoveryEventKind::DeviceUpdated,
                        observed_at.clone(),
                        scan_session_id.clone(),
                        Some(device.device_id.clone()),
                        None,
                        format!("Updated {device_label} network identity"),
                    ),
                );
            }
            if previous_device.reachability != device.reachability {
                match device.reachability {
                    LanPairingDeviceReachability::Online
                    | LanPairingDeviceReachability::Offline => {
                        let kind = if device.reachability == LanPairingDeviceReachability::Online {
                            LanDiscoveryEventKind::DeviceOnline
                        } else {
                            LanDiscoveryEventKind::DeviceOffline
                        };
                        let status_summary =
                            if device.reachability == LanPairingDeviceReachability::Online {
                                format!("{device_label} is online")
                            } else {
                                format!("{device_label} is offline")
                            };
                        push_discovery_event_row(
                            rows,
                            discovery_event_row(
                                reachability_discovery_event_id(
                                    if device.reachability == LanPairingDeviceReachability::Online {
                                        LAN_DISCOVERY_DEVICE_ONLINE_SEGMENT
                                    } else {
                                        LAN_DISCOVERY_DEVICE_OFFLINE_SEGMENT
                                    },
                                    &scan_key,
                                    &device.device_id,
                                ),
                                kind,
                                observed_at.clone(),
                                scan_session_id.clone(),
                                Some(device.device_id.clone()),
                                None,
                                status_summary,
                            ),
                        );
                    }
                    LanPairingDeviceReachability::Stale => {}
                }
            }
        }

        if device.agent_status.is_some() {
            push_discovery_event_row(
                rows,
                discovery_event_row(
                    keyed_discovery_event_id(
                        LAN_DISCOVERY_AGENT_DISCOVERED_EVENT_PREFIX,
                        &scan_key,
                        &device.device_id,
                    ),
                    LanDiscoveryEventKind::AgentDiscovered,
                    observed_at,
                    scan_session_id.clone(),
                    Some(device.device_id.clone()),
                    None,
                    format!("Detected agent signature on {device_label}"),
                ),
            );
        }
    }
}

fn push_canonical_household_event_rows(
    rows: &mut Vec<LanDiscoveryEventRow>,
    scan_result: &LanNetworkDeviceScanResult,
    read_model: &LanBrowserAddDeviceReadModel,
) {
    let scan_session_id = scan_session_id_for_result(scan_result);
    for device in &read_model.canonical_household_devices {
        let observed_at = canonical_device_observed_at(device, read_model);
        let scan_key = scan_session_key(scan_session_id.as_deref(), &observed_at);

        for record in &device.network_identity.evidence_records {
            push_discovery_event_row(
                rows,
                discovery_event_row(
                    keyed_discovery_event_id(
                        LAN_DISCOVERY_EVIDENCE_FOUND_EVENT_PREFIX,
                        &scan_key,
                        &record.evidence_id,
                    ),
                    LanDiscoveryEventKind::EvidenceFound,
                    record.last_seen_at.clone(),
                    scan_session_id.clone(),
                    Some(device.canonical_device_id.clone()),
                    Some(record.evidence_id.clone()),
                    evidence_found_summary(
                        serialized_enum_label(&record.evidence_kind).as_str(),
                        &device.display_name,
                    ),
                ),
            );
        }

        if device.classification == LanCanonicalHouseholdDeviceClassification::UnknownLanDevice {
            push_discovery_event_row(
                rows,
                discovery_event_row(
                    keyed_discovery_event_id(
                        LAN_DISCOVERY_UNKNOWN_DETECTED_EVENT_PREFIX,
                        &scan_key,
                        &device.canonical_device_id,
                    ),
                    LanDiscoveryEventKind::UnknownDetected,
                    observed_at.clone(),
                    scan_session_id.clone(),
                    Some(device.canonical_device_id.clone()),
                    None,
                    format!("Detected unknown LAN device {}", device.display_name),
                ),
            );
        }

        if device.classification == LanCanonicalHouseholdDeviceClassification::ChildAgent
            && device.child_agent_inventory.is_some()
            && (device.trust_state == LanPairingTrustState::Paired
                || !device.network_identity.evidence_records.is_empty())
        {
            push_discovery_event_row(
                rows,
                discovery_event_row(
                    keyed_discovery_event_id(
                        LAN_DISCOVERY_AGENT_CONFIRMED_EVENT_PREFIX,
                        &scan_key,
                        &device.canonical_device_id,
                    ),
                    LanDiscoveryEventKind::AgentConfirmed,
                    observed_at.clone(),
                    scan_session_id.clone(),
                    Some(device.canonical_device_id.clone()),
                    None,
                    agent_confirmed_summary(&device.display_name),
                ),
            );
        }

        if device.network_identity.reachability == LanPairingDeviceReachability::Offline {
            let offline_at = device
                .network_identity
                .offline_at
                .clone()
                .unwrap_or_else(|| observed_at.clone());
            push_discovery_event_row(
                rows,
                discovery_event_row(
                    keyed_discovery_event_id(
                        LAN_DISCOVERY_DEVICE_OFFLINE_EVENT_PREFIX,
                        &scan_key,
                        &device.canonical_device_id,
                    ),
                    LanDiscoveryEventKind::DeviceOffline,
                    offline_at,
                    scan_session_id.clone(),
                    Some(device.canonical_device_id.clone()),
                    None,
                    format!("{} is offline", device.display_name),
                ),
            );
        }
    }
}

fn current_scan_snapshot(
    scan_result: &LanNetworkDeviceScanResult,
) -> Option<&scan_history::LanScanHistorySnapshot> {
    scan_result
        .current_scan_snapshot
        .as_ref()
        .or(scan_result.previous_scan_snapshot.as_ref())
}

fn scan_session_id_for_result(scan_result: &LanNetworkDeviceScanResult) -> Option<String> {
    current_scan_snapshot(scan_result)
        .and_then(|snapshot| snapshot.metadata.as_ref())
        .map(|metadata| metadata.scan_id.clone())
}

fn scan_session_key(scan_session_id: Option<&str>, fallback: &str) -> String {
    scan_session_id
        .map(|value| value.to_string())
        .unwrap_or_else(|| fallback.replace([':', '.'], SCAN_SESSION_KEY_REPLACEMENT))
}

fn interface_changed_event_id(scan_session_id: Option<&str>, scan_time: &str) -> String {
    let mut event_id = String::from(LAN_DISCOVERY_INTERFACE_CHANGED_EVENT_PREFIX);
    event_id.push_str(&scan_session_key(scan_session_id, scan_time));
    event_id
}

fn keyed_discovery_event_id(prefix: &str, scan_key: &str, entity_id: &str) -> String {
    let mut event_id = String::from(prefix);
    event_id.push_str(scan_key);
    event_id.push('-');
    event_id.push_str(entity_id);
    event_id
}

fn reachability_discovery_event_id(segment: &str, scan_key: &str, entity_id: &str) -> String {
    let mut event_id = String::from(LAN_DISCOVERY_EVENT_PREFIX);
    event_id.push_str(segment);
    event_id.push('-');
    event_id.push_str(scan_key);
    event_id.push('-');
    event_id.push_str(entity_id);
    event_id
}

fn scan_finished_summary(scanned_device_count: u32) -> String {
    let mut summary = String::from(LAN_DISCOVERY_SCAN_FINISHED_SUMMARY_PREFIX);
    summary.push_str(&scanned_device_count.to_string());
    summary.push_str(LAN_DISCOVERY_SCAN_FINISHED_SUMMARY_SUFFIX);
    summary
}

fn evidence_found_summary(evidence_kind: &str, display_name: &str) -> String {
    let mut summary = String::from(LAN_DISCOVERY_OBSERVED_SUMMARY_PREFIX);
    summary.push_str(evidence_kind);
    summary.push_str(LAN_DISCOVERY_EVIDENCE_FOR_SEPARATOR);
    summary.push_str(display_name);
    summary
}

fn agent_confirmed_summary(display_name: &str) -> String {
    let mut summary = String::from(LAN_DISCOVERY_AGENT_CONFIRMED_SUMMARY_PREFIX);
    summary.push_str(display_name);
    summary
}

fn scan_event_occurred_at(scan_result: &LanNetworkDeviceScanResult) -> String {
    current_scan_snapshot(scan_result)
        .map(|snapshot| snapshot.updated_at.clone())
        .unwrap_or_else(timestamp_now)
}

fn discovery_event_row(
    event_id: String,
    event_kind: LanDiscoveryEventKind,
    occurred_at: String,
    scan_session_id: Option<String>,
    affected_device_id: Option<String>,
    evidence_id: Option<String>,
    summary: String,
) -> LanDiscoveryEventRow {
    LanDiscoveryEventRow {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        event_id,
        event_kind,
        occurred_at,
        previous_event_id: None,
        scan_session_id,
        affected_device_id,
        evidence_id,
        summary,
    }
}

fn push_discovery_event_row(rows: &mut Vec<LanDiscoveryEventRow>, row: LanDiscoveryEventRow) {
    if rows
        .iter()
        .any(|existing| existing.event_id == row.event_id)
    {
        return;
    }
    rows.push(row);
}

fn same_network_device(
    left: &LanNetworkInventoryDevice,
    right: &LanNetworkInventoryDevice,
) -> bool {
    left.device_id == right.device_id
        || same_non_empty_text(&left.mac_address, &right.mac_address)
        || same_non_empty_text(&left.ip_address, &right.ip_address)
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

fn physical_device_label(device: &LanNetworkInventoryDevice) -> String {
    device
        .hostname
        .clone()
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| device.label.clone())
}

fn same_non_empty_text(left: &str, right: &str) -> bool {
    let left = left.trim();
    let right = right.trim();
    !left.is_empty() && !right.is_empty() && left.eq_ignore_ascii_case(right)
}

fn trimmed_non_empty(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_string())
}

fn canonical_device_observed_at(
    device: &LanCanonicalHouseholdDevice,
    read_model: &LanBrowserAddDeviceReadModel,
) -> String {
    device
        .network_identity
        .evidence_records
        .iter()
        .map(|record| record.last_seen_at.clone())
        .max()
        .or_else(|| device.network_identity.offline_at.clone())
        .or_else(|| device.network_identity.stale_at.clone())
        .unwrap_or_else(|| read_model.generated_at.clone())
}

fn pairing_requests(
    runtime: &LanPairingRuntime,
    generated_at: &str,
) -> Vec<LanBrowserAddDevicePairingRequest> {
    runtime
        .challenges
        .lock()
        .map(|challenges| {
            challenges
                .iter()
                .map(|challenge| LanBrowserAddDevicePairingRequest {
                    schema_version: constants::lan_pairing::SCHEMA_VERSION,
                    challenge_id: challenge.challenge_id.clone(),
                    child_device_id: challenge.child_device_id.clone(),
                    parent_device_id: challenge.parent_device_id.clone(),
                    route_id: challenge.route_id.clone(),
                    origin: challenge.origin.clone(),
                    pairing_state: pairing_request_state(
                        challenge.accepted,
                        generated_at,
                        &challenge.expires_at,
                    ),
                    rejection_reason: None,
                    issued_at: challenge.issued_at.clone(),
                    expires_at: challenge.expires_at.clone(),
                })
                .collect()
        })
        .unwrap_or_default()
}
