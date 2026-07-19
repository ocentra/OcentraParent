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
use ocentra_parent_agent_protocol::lan_pairing::LanPairingText;
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceDiscoveryDevice;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanBrowserAddDeviceReadModel;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanDiscoveryEvidenceSource;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanPairingDiscoverySource;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

#[path = "lan_pairing_browser_add_device_state/discovery_event_history.rs"]
pub(crate) mod discovery_event_history;
#[path = "lan_pairing_browser_add_device_state/discovery_projection.rs"]
pub(crate) mod discovery_projection;
#[path = "lan_pairing_browser_add_device_state/physical_lan_scan.rs"]
pub(crate) mod physical_lan_scan;
#[path = "lan_pairing_browser_add_device_state/registry_projection.rs"]
pub(crate) mod registry_projection;
#[path = "lan_pairing_browser_add_device_state/scan_history.rs"]
pub(crate) mod scan_history;

use crate::fields::fields_from_pairs;
use crate::lan_pairing_browser_add_device_scan::{push_if_absent, same_physical_network_device};
use crate::{lan_pairing::LanPairingRuntime, time::timestamp_now};

use self::discovery_projection::{
    discovery_state_for, expired_pairing_count, pending_pairing_count,
    physical_household_lan_state, selected_device_readiness, serialized_enum_label,
};
use self::physical_lan_scan::{
    network_device_scan_result_for_command, refresh_network_device_scan_history,
    LanNetworkDeviceScanResult,
};
use self::registry_projection::{
    household_device_decisions, merged_known_household_devices_for_read_model, pairing_requests,
    persist_known_household_devices, trusted_device_registry,
};

#[cfg(any(target_os = "macos", target_os = "ios"))]
const APPLE_LAN_DISCOVERY_MANUAL_REQUIRED: bool = true;
#[cfg(not(any(target_os = "macos", target_os = "ios")))]
const APPLE_LAN_DISCOVERY_MANUAL_REQUIRED: bool = false;
pub(crate) fn refresh_browser_discovery_scan_history(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
) {
    let _ = refresh_network_device_scan_history(runtime, command);
}

pub(crate) fn browser_add_device_fields(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    discovery_state: &LanPairingText,
) -> LogFields {
    let model = browser_add_device_read_model(runtime, command, discovery_state);
    let read_model_json = serde_json::to_string(&model).unwrap_or_default();
    fields_from_pairs(vec![
        (
            constants::field::LAN_ADD_DEVICE_READ_MODEL,
            LogFieldValue::String(read_model_json),
        ),
        (
            constants::field::LAN_ADD_DEVICE_STATE,
            LogFieldValue::String(serialized_enum_label(&model.add_device_state).0),
        ),
        (
            constants::field::LAN_DISCOVERY_SOURCE,
            LogFieldValue::String(serialized_enum_label(&model.discovery_source).0),
        ),
        (
            constants::field::LAN_LOCAL_SERVICE_DISCOVERY_STATE,
            LogFieldValue::String(serialized_enum_label(&model.local_service_discovery_state).0),
        ),
        (
            constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE,
            LogFieldValue::String(serialized_enum_label(&model.physical_household_lan_state).0),
        ),
        (
            constants::field::LAN_CLOUD_RELAY_STATE,
            LogFieldValue::String(serialized_enum_label(&model.cloud_relay_state).0),
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
    ])
}

pub(crate) fn browser_add_device_read_model(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    discovery_state: &LanPairingText,
) -> LanBrowserAddDeviceReadModel {
    let scan_result = network_device_scan_result_for_command(runtime, command);
    let observed_at =
        browser_read_model_generated_at(timestamp_now::<String>().into(), &scan_result);
    let history_generated_at = scan_result
        .current_scan_snapshot
        .as_ref()
        .or(scan_result.previous_scan_snapshot.as_ref())
        .map(|snapshot| LanPairingText(snapshot.updated_at.clone()))
        .unwrap_or_else(|| observed_at.clone());
    let selected = runtime.selected_target();
    let trusted_device_registry = trusted_device_registry(runtime);
    let household_device_decisions = household_device_decisions(runtime);
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
        &history_generated_at,
        &network_devices,
    );
    let physical_household_lan_state = physical_household_lan_state(has_network_devices);
    let selected_device_readiness = selected_device_readiness(selected);

    let mut model = build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: history_generated_at.0.clone(),
        discovery_source,
        service_data_available: true,
        platform_data_available,
        add_device_state: discovery_state_for(discovery_state),
        local_service_discovery_state: discovery_state_for(discovery_state),
        physical_household_lan_state,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices,
        pairing_requests: pairing_requests(runtime, &observed_at),
        trusted_device_registry,
        household_device_decisions,
        trusted_device_ids: runtime
            .trusted_device_ids()
            .into_iter()
            .map(|value| value.0)
            .collect(),
        revoked_device_ids: runtime
            .revoked_device_ids()
            .into_iter()
            .map(|value| value.0)
            .collect(),
        selected_device_readiness,
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    });
    let current_canonical_household_devices = model.canonical_household_devices.clone();
    persist_known_household_devices(runtime, &current_canonical_household_devices);
    model.canonical_household_devices = merged_known_household_devices_for_read_model(
        runtime,
        &current_canonical_household_devices,
        &history_generated_at,
    );
    model.discovery_event_history = discovery_event_history::discovery_event_history(
        &scan_result,
        &model,
        &history_generated_at,
    );
    model.generated_at = observed_at.0;
    model
}

pub(crate) fn browser_read_model_generated_at(
    observed_at: LanPairingText,
    _scan_result: &LanNetworkDeviceScanResult,
) -> LanPairingText {
    observed_at
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
    discovery_state: &LanPairingText,
    generated_at: &LanPairingText,
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
    discovery_state: &LanPairingText,
    generated_at: &LanPairingText,
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
    discovery_state: &LanPairingText,
    generated_at: &LanPairingText,
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
    generated_at: &LanPairingText,
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
    child_device.mac_address =
        trimmed_non_empty(&LanPairingText(network_device.mac_address.clone())).map(|value| value.0);
    child_device.hostname =
        Some(network_device.hostname.clone().unwrap_or_else(|| {
            constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME.to_string()
        }));
    child_device.network_interface = network_device.network_interface.clone();
    child_device
}

fn trimmed_non_empty(value: &LanPairingText) -> Option<LanPairingText> {
    let value = value.0.trim();
    (!value.is_empty()).then(|| value.into())
}
