use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, LanBrowserAddDeviceDiscoveryDevice,
    LanBrowserAddDevicePairingRequest, LanBrowserAddDeviceReadModel, LanHouseholdDeviceDecision,
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus,
    LanPairingDiscoverySource, LanPairingNetworkMode, LanPairingParentAuthority,
    LanPairingProductionDiscoveryState, LanPairingTrustState, LanSelectedDeviceReadiness,
    LogFieldValue,
};

mod production_household_proof;
mod signed_discovery_relay_spine;
mod source_matrix;

use crate::lan_network_inventory;
use crate::lan_pairing_browser_add_device_scan::{
    push_if_absent, same_physical_network_device, scan_summary,
};
use crate::lan_pairing_household_device_spine;
use crate::{lan_pairing::LanPairingRuntime, time::timestamp_now};

use self::production_household_proof::production_household_proof_summary;
use self::signed_discovery_relay_spine::signed_discovery_relay_spine_summary;
use self::source_matrix::lan_discovery_source_matrix;

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
            LogFieldValue::String(discovery_state.to_string()),
        ),
        (
            constants::field::LAN_DISCOVERY_SOURCE,
            LogFieldValue::String(constants::value::LAN_DISCOVERY_SOURCE_LOCAL_SERVICE.to_string()),
        ),
        (
            constants::field::LAN_LOCAL_SERVICE_DISCOVERY_STATE,
            LogFieldValue::String(discovery_state.to_string()),
        ),
        (
            constants::field::LAN_PHYSICAL_HOUSEHOLD_LAN_STATE,
            LogFieldValue::String(
                constants::value::LAN_DISCOVERY_STATE_MANUAL_REQUIRED.to_string(),
            ),
        ),
        (
            constants::field::LAN_CLOUD_RELAY_STATE,
            LogFieldValue::String(constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE.to_string()),
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

fn browser_add_device_read_model(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    discovery_state: &str,
) -> LanBrowserAddDeviceReadModel {
    let generated_at = timestamp_now();
    let selected = runtime.selected_target();
    let trusted_device_registry = trusted_device_registry(runtime);
    let household_device_decisions = household_device_decisions(runtime);
    let network_devices = lan_network_inventory::discover_lan_network_devices();
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
    let canonical_household_devices =
        lan_pairing_household_device_spine::canonical_household_devices(
            &discovered_devices,
            &trusted_device_registry,
            &household_device_decisions,
        );
    let scan_summary = scan_summary(&discovered_devices);
    let selected_device_readiness = selected_device_readiness(selected);
    let production_household_proof = production_household_proof_summary(
        &generated_at,
        physical_household_lan_state.clone(),
        &scan_summary,
        &trusted_device_registry,
        &household_device_decisions,
        &selected_device_readiness,
    );
    let signed_discovery_relay_spine = signed_discovery_relay_spine_summary(
        &generated_at,
        physical_household_lan_state.clone(),
        &scan_summary,
        &trusted_device_registry,
        &household_device_decisions,
        &selected_device_readiness,
    );
    let lan_discovery_source_matrix = lan_discovery_source_matrix(&generated_at, &scan_summary);
    LanBrowserAddDeviceReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: generated_at.clone(),
        discovery_source,
        add_device_state: discovery_state_for(discovery_state),
        local_service_discovery_state: discovery_state_for(discovery_state),
        physical_household_lan_state,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        scan_summary,
        discovered_devices,
        canonical_household_devices,
        pairing_requests: pairing_requests(runtime, &generated_at),
        trusted_device_registry,
        household_device_decisions,
        production_household_proof: Some(production_household_proof),
        signed_discovery_relay_spine: Some(signed_discovery_relay_spine),
        lan_discovery_source_matrix: Some(lan_discovery_source_matrix),
        trusted_device_ids: runtime.trusted_device_ids(),
        revoked_device_ids: runtime.revoked_device_ids(),
        selected_device_readiness,
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
        route_requirement_labels: constants::lan_pairing::ROUTE_REQUIREMENTS
            .iter()
            .map(|requirement| (*requirement).to_string())
            .collect(),
        audit_check_labels: audit_check_labels(),
        honest_non_claims: honest_non_claims(),
    }
}

fn physical_household_lan_state(has_network_devices: bool) -> LanPairingProductionDiscoveryState {
    if has_network_devices {
        LanPairingProductionDiscoveryState::Discovered
    } else {
        LanPairingProductionDiscoveryState::ManualRequired
    }
}

fn discovered_devices(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    discovery_state: &str,
    generated_at: &str,
    network_devices: &[lan_network_inventory::LanNetworkInventoryDevice],
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
                    route_id: entry.route_id.clone(),
                    network_mode: LanPairingNetworkMode::LocalNetwork,
                    reachability: reachability.clone(),
                    address_ref: constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string(),
                    discovery_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
                    discovery_state: discovery_state_for(discovery_state),
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
        child_device: lan_network_inventory::local_agent_device_ref(
            command.target.device_id.clone(),
            command.target.platform.clone(),
        ),
        agent_peer_id: command.source.peer_id.clone(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: LanPairingDeviceReachability::Online,
        address_ref: constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string(),
        discovery_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
        discovery_state: discovery_state_for(discovery_state),
    }
}

fn network_neighbor_discovery_device(
    command: &AgentCommandEnvelope,
    generated_at: &str,
    network_device: &lan_network_inventory::LanNetworkInventoryDevice,
) -> LanBrowserAddDeviceDiscoveryDevice {
    LanBrowserAddDeviceDiscoveryDevice {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        discovered_at: generated_at.to_string(),
        child_device: network_neighbor_child_device(network_device),
        agent_peer_id: command.source.peer_id.clone(),
        route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
        network_mode: LanPairingNetworkMode::LocalNetwork,
        reachability: network_device.reachability.clone(),
        address_ref: constants::lan_pairing::ADDRESS_REF_NETWORK_NEIGHBOR.to_string(),
        discovery_status: LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
        discovery_state: LanPairingProductionDiscoveryState::Discovered,
    }
}

fn network_neighbor_child_device(
    network_device: &lan_network_inventory::LanNetworkInventoryDevice,
) -> LanPairingDeviceRef {
    let mut child_device = LanPairingDeviceRef::new(
        network_device.device_id.clone(),
        None,
        network_device.label.clone(),
        network_device.platform.clone(),
    );
    child_device.ip_address = Some(network_device.ip_address.clone());
    child_device.mac_address = Some(network_device.mac_address.clone());
    child_device.hostname =
        Some(network_device.hostname.clone().unwrap_or_else(|| {
            constants::lan_pairing::NETWORK_NEIGHBOR_UNKNOWN_HOSTNAME.to_string()
        }));
    child_device.network_interface = network_device.network_interface.clone();
    child_device
}

fn trusted_device_registry(
    runtime: &LanPairingRuntime,
) -> Vec<ocentra_parent_agent_protocol::LanTrustedDeviceRegistryEntry> {
    runtime
        .registry
        .lock()
        .map(|registry| registry.entries().to_vec())
        .unwrap_or_default()
}

fn household_device_decisions(runtime: &LanPairingRuntime) -> Vec<LanHouseholdDeviceDecision> {
    runtime
        .registry
        .lock()
        .map(|registry| registry.household_device_decisions().to_vec())
        .unwrap_or_default()
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

fn selected_device_readiness(
    selected: Option<ocentra_parent_agent_protocol::LanSelectedRouteTarget>,
) -> LanSelectedDeviceReadiness {
    match selected {
        Some(target) => {
            let ready_for_control = target.trust_state == LanPairingTrustState::Paired
                && target.reachability == LanPairingDeviceReachability::Online;
            LanSelectedDeviceReadiness {
                schema_version: constants::lan_pairing::SCHEMA_VERSION,
                selected_child_device_id: Some(target.selected_child_device_id),
                route_id: Some(target.route_id),
                pairing_id: target.pairing_id,
                trust_state: target.trust_state,
                reachability: target.reachability,
                ready_for_control,
                stale_at: target.stale_at,
                offline_at: target.offline_at,
            }
        }
        None => LanSelectedDeviceReadiness {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            selected_child_device_id: None,
            route_id: None,
            pairing_id: None,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Offline,
            ready_for_control: false,
            stale_at: None,
            offline_at: None,
        },
    }
}

fn pairing_request_state(
    accepted: bool,
    observed_at: &str,
    expires_at: &str,
) -> LanPairingProductionDiscoveryState {
    if accepted {
        LanPairingProductionDiscoveryState::Paired
    } else if observed_at > expires_at {
        LanPairingProductionDiscoveryState::Expired
    } else {
        LanPairingProductionDiscoveryState::Pending
    }
}

fn discovery_state_for(state: &str) -> LanPairingProductionDiscoveryState {
    match state {
        constants::value::LAN_DISCOVERY_STATE_PENDING => {
            LanPairingProductionDiscoveryState::Pending
        }
        constants::value::LAN_DISCOVERY_STATE_PAIRED => LanPairingProductionDiscoveryState::Paired,
        constants::value::LAN_DISCOVERY_STATE_REJECTED => {
            LanPairingProductionDiscoveryState::Rejected
        }
        constants::value::LAN_DISCOVERY_STATE_EXPIRED => {
            LanPairingProductionDiscoveryState::Expired
        }
        constants::value::LAN_DISCOVERY_STATE_REVOKED => {
            LanPairingProductionDiscoveryState::Revoked
        }
        constants::value::LAN_DISCOVERY_STATE_STALE => LanPairingProductionDiscoveryState::Stale,
        constants::value::LAN_DISCOVERY_STATE_OFFLINE => {
            LanPairingProductionDiscoveryState::Offline
        }
        constants::value::LAN_DISCOVERY_STATE_MANUAL_REQUIRED => {
            LanPairingProductionDiscoveryState::ManualRequired
        }
        constants::value::LAN_DISCOVERY_STATE_UNAVAILABLE => {
            LanPairingProductionDiscoveryState::Unavailable
        }
        _ => LanPairingProductionDiscoveryState::Discovered,
    }
}

fn pending_pairing_count(model: &LanBrowserAddDeviceReadModel) -> usize {
    model
        .pairing_requests
        .iter()
        .filter(|request| request.pairing_state == LanPairingProductionDiscoveryState::Pending)
        .count()
}

fn expired_pairing_count(model: &LanBrowserAddDeviceReadModel) -> usize {
    model
        .pairing_requests
        .iter()
        .filter(|request| request.pairing_state == LanPairingProductionDiscoveryState::Expired)
        .count()
}

fn audit_check_labels() -> Vec<String> {
    [
        constants::value::LAN_REASON_ANONYMOUS,
        constants::value::LAN_REASON_WRONG_ORIGIN,
        constants::value::LAN_REASON_WRONG_DEVICE,
        constants::value::LAN_REASON_REPLAYED,
        constants::value::LAN_REASON_STALE,
        constants::value::LAN_REASON_REVOKED,
        constants::value::LAN_REASON_OFFLINE,
        constants::value::LAN_REASON_EXPIRED,
    ]
    .iter()
    .map(|label| (*label).to_string())
    .collect()
}

fn honest_non_claims() -> Vec<String> {
    [
        constants::value::LAN_NON_CLAIM_PHYSICAL_HOUSEHOLD_MANUAL_REQUIRED,
        constants::value::LAN_NON_CLAIM_CLOUD_RELAY_NOT_IMPLEMENTED,
        constants::value::LAN_NON_CLAIM_REMOTE_DESKTOP_NOT_IMPLEMENTED,
    ]
    .iter()
    .map(|claim| (*claim).to_string())
    .collect()
}
