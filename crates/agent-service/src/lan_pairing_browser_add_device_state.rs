use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, LanBrowserAddDeviceDiscoveryDevice,
    LanBrowserAddDevicePairingRequest, LanBrowserAddDeviceReadModel, LanPairingDeviceReachability,
    LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus, LanPairingDiscoverySource,
    LanPairingNetworkMode, LanPairingParentAuthority, LanPairingProductionDiscoveryState,
    LanPairingTrustState, LanSelectedDeviceReadiness, LogFieldValue,
};

use crate::{lan_pairing::LanPairingRuntime, time::timestamp_now};

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
    LanBrowserAddDeviceReadModel {
        schema_version: constants::lan_pairing::SCHEMA_VERSION,
        generated_at: generated_at.clone(),
        discovery_source: LanPairingDiscoverySource::LocalService,
        add_device_state: discovery_state_for(discovery_state),
        local_service_discovery_state: discovery_state_for(discovery_state),
        physical_household_lan_state: LanPairingProductionDiscoveryState::ManualRequired,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: discovered_devices(runtime, command, discovery_state, &generated_at),
        pairing_requests: pairing_requests(runtime, &generated_at),
        trusted_device_registry,
        trusted_device_ids: runtime.trusted_device_ids(),
        revoked_device_ids: runtime.revoked_device_ids(),
        selected_device_readiness: selected_device_readiness(selected),
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

fn discovered_devices(
    runtime: &LanPairingRuntime,
    command: &AgentCommandEnvelope,
    discovery_state: &str,
    generated_at: &str,
) -> Vec<LanBrowserAddDeviceDiscoveryDevice> {
    let mut devices: Vec<LanBrowserAddDeviceDiscoveryDevice> = runtime
        .selected_target()
        .map(|target| target.reachability)
        .into_iter()
        .chain(std::iter::once(LanPairingDeviceReachability::Online))
        .next()
        .map(|reachability| (reachability, runtime.registry.lock()))
        .and_then(|(reachability, registry)| {
            registry.ok().map(|registry| {
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
                        address_ref: constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET
                            .to_string(),
                        discovery_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
                        discovery_state: discovery_state_for(discovery_state),
                    })
                    .collect()
            })
        })
        .unwrap_or_default();

    if devices.is_empty() {
        devices.push(LanBrowserAddDeviceDiscoveryDevice {
            schema_version: constants::lan_pairing::SCHEMA_VERSION,
            discovered_at: generated_at.to_string(),
            child_device: LanPairingDeviceRef {
                device_id: command.target.device_id.clone(),
                child_profile_id: None,
                label: command.target.device_id.clone(),
                platform: command.target.platform.clone(),
            },
            agent_peer_id: command.source.peer_id.clone(),
            route_id: constants::lan_pairing::ROUTE_ID_LOCAL_NETWORK.to_string(),
            network_mode: LanPairingNetworkMode::LocalNetwork,
            reachability: LanPairingDeviceReachability::Online,
            address_ref: constants::lan_pairing::ADDRESS_REF_DIRECT_WEBSOCKET.to_string(),
            discovery_status: LanPairingDiscoveryRuntimeStatus::WebsocketDirect,
            discovery_state: discovery_state_for(discovery_state),
        });
    }

    devices
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
