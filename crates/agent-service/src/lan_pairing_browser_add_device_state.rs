use chrono::Utc;
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
use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentCommandName};

#[path = "lan_pairing_browser_add_device_state/discovery_event_history.rs"]
pub(crate) mod discovery_event_history;
#[path = "lan_pairing_browser_add_device_state/discovery_projection.rs"]
pub(crate) mod discovery_projection;
#[path = "lan_pairing_browser_add_device_state/physical_lan_scan.rs"]
pub(crate) mod physical_lan_scan;
#[path = "lan_pairing_browser_add_device_state/registry_projection.rs"]
pub(crate) mod registry_projection;
#[path = "lan_pairing_browser_add_device_state/replay_projection.rs"]
mod replay_projection;
#[path = "lan_pairing_browser_add_device_state/scan_history.rs"]
pub(crate) mod scan_history;

use crate::fields::fields_from_pairs;
use crate::lan_pairing_browser_add_device_scan::{push_if_absent, same_physical_network_device};
use crate::lan_pairing_runtime_state::passive_discovery::capability_store::current_runtime_capability;
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
use self::replay_projection::effective_replay_projection;

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
    let mut model = build_live_read_model(
        runtime,
        command,
        discovery_state,
        &scan_result,
        &observed_at,
    );
    let current_canonical_household_devices = model.canonical_household_devices.clone();
    if restore_live_canonical_household_devices(
        &mut model,
        runtime,
        &current_canonical_household_devices,
        &observed_at,
    )
    .is_err()
    {
        model.canonical_household_devices = current_canonical_household_devices.clone();
    }
    let expected_snapshot = scan_result.current_scan_snapshot.as_ref();
    let persisted_projection = if !scan_result.reused_recent_snapshot
        || command.command == AgentCommandName::AgentLanPairingStatusGet
    {
        expected_snapshot.and_then(|expected_snapshot| {
            scan_history::save_replay_canonical_devices(
                runtime,
                expected_snapshot,
                &current_canonical_household_devices,
                &observed_at,
            )
        })
    } else {
        None
    };
    let effective_projection = effective_replay_projection(&scan_result, persisted_projection);
    let history_generated_at = effective_projection
        .as_ref()
        .map(|projection| LanPairingText(projection.generated_at.clone()))
        .unwrap_or_else(|| observed_at.clone());
    let mut replay_history_model = replay_history_model(&model, &history_generated_at);
    let has_persisted_projection = assign_replay_projection(
        &mut replay_history_model,
        &scan_result,
        effective_projection.as_ref(),
        &current_canonical_household_devices,
    );
    model.discovery_event_history = discovery_event_history::replay_discovery_event_history(
        &scan_result,
        &replay_history_model,
        &history_generated_at,
        has_persisted_projection,
    );
    model
}

fn build_live_read_model(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    discovery_state: &LanPairingText,
    scan_result: &LanNetworkDeviceScanResult,
    observed_at: &LanPairingText,
) -> LanBrowserAddDeviceReadModel {
    let network_devices = scan_result.devices.clone();
    let has_current_physical_scan =
        scan_result
            .current_scan_snapshot
            .as_ref()
            .is_some_and(|snapshot| {
                scan_history::scan_history_is_recent(
                    &LanPairingText(snapshot.updated_at.clone()),
                    Utc::now(),
                )
            });
    let has_network_devices = has_current_physical_scan && !network_devices.is_empty();
    let discovery_source = if has_network_devices {
        LanPairingDiscoverySource::PhysicalHouseholdLan
    } else {
        LanPairingDiscoverySource::LocalService
    };
    build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: observed_at.0.clone(),
        discovery_source,
        service_data_available: current_runtime_capability(runtime).service_data_available(),
        platform_data_available: platform_data_available_for_scan_result(scan_result),
        add_device_state: discovery_state_for(discovery_state),
        local_service_discovery_state: discovery_state_for(discovery_state),
        physical_household_lan_state: physical_household_lan_state(has_network_devices),
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: discovered_devices(
            runtime,
            command,
            discovery_state,
            observed_at,
            &network_devices,
        ),
        pairing_requests: pairing_requests(runtime, observed_at),
        trusted_device_registry: trusted_device_registry(runtime),
        household_device_decisions: household_device_decisions(runtime),
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
        selected_device_readiness: selected_device_readiness(runtime.selected_target()),
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    })
}

fn restore_live_canonical_household_devices(
    model: &mut LanBrowserAddDeviceReadModel,
    runtime: &LanPairingRuntime,
    current_canonical_household_devices: &[ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice],
    observed_at: &LanPairingText,
) -> Result<(), ocentra_parent_agent_protocol::lan_pairing::LanPairingRejectionReason> {
    persist_known_household_devices(runtime, current_canonical_household_devices)?;
    model.canonical_household_devices = merged_known_household_devices_for_read_model(
        runtime,
        current_canonical_household_devices,
        observed_at,
    );
    Ok(())
}

fn replay_history_model(
    live_model: &LanBrowserAddDeviceReadModel,
    history_generated_at: &LanPairingText,
) -> LanBrowserAddDeviceReadModel {
    let mut replay_model = live_model.clone();
    replay_model.generated_at = history_generated_at.0.clone();
    replay_model
}

fn assign_replay_projection(
    replay_model: &mut LanBrowserAddDeviceReadModel,
    scan_result: &LanNetworkDeviceScanResult,
    persisted_projection: Option<&scan_history::LanReplayCanonicalProjection>,
    current_canonical_household_devices: &[ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::LanCanonicalHouseholdDevice],
) -> bool {
    let has_persisted_projection = persisted_projection.is_some();
    replay_model.canonical_household_devices = if scan_result.reused_recent_snapshot {
        persisted_projection
            .map(|projection| projection.canonical_devices.clone())
            .unwrap_or_default()
    } else {
        persisted_projection
            .map(|projection| projection.canonical_devices.clone())
            .unwrap_or_else(|| current_canonical_household_devices.to_vec())
    };
    has_persisted_projection
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
