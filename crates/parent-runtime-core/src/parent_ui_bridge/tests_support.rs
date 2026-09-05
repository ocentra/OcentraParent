use std::collections::BTreeMap;
use std::sync::{Mutex, OnceLock};

use ocentra_lan_core::read_model_builder::{
    build_lan_add_device_read_model, LanAddDeviceReadModelInput,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::lan_pairing::{
    LanPairingDeviceReachability, LanPairingDeviceRef, LanPairingDiscoveryRuntimeStatus,
    LanPairingNetworkMode, LanPairingProductionDiscoveryState, LanPairingTrustState,
};
use ocentra_parent_agent_protocol::lan_pairing_authority::LanPairingParentAuthority;
use ocentra_parent_agent_protocol::lan_pairing_browser_add_device_state::{
    LanBrowserAddDeviceDiscoveryDevice, LanBrowserAddDeviceReadModel, LanDiscoveryEvidenceSource,
    LanPairingDiscoverySource, LanSelectedDeviceReadiness,
};
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogLevel};
use ocentra_parent_agent_protocol::transport::{
    AgentCommandName, AgentEventEnvelope, AgentEventName, AgentPeer, AgentPeerRole,
};
use ocentra_parent_runtime_core::parent_ui_bridge::{
    lan_replay_rejection_episode::ParentRouteSubscriptionLoadState,
    projection::{ParentAgentServiceProjection, ParentAgentServiceProjectionResponse},
};
use ocentra_schema::parent_ui_bridge::{
    ParentRouteId, ParentRouteSnapshot, ParentSubscriptionEvent, ParentUiAction,
    ParentUiActionResult,
};

pub(super) const REQUEST_MESSAGE_ID_CORRELATION: &str = "test-request-message-id";

pub(super) fn projection_response(
    command: AgentCommandName,
    response_event: AgentEventEnvelope,
) -> ParentAgentServiceProjectionResponse {
    require_ok(
        ParentAgentServiceProjectionResponse::from_envelopes(
            command,
            REQUEST_MESSAGE_ID_CORRELATION.to_string(),
            vec![ready_event(), response_event],
        ),
        "typed agent response projects",
    )
}

pub(super) fn projected_route_snapshot(
    route: ParentRouteId,
    responses: Vec<ParentAgentServiceProjectionResponse>,
) -> ParentRouteSnapshot {
    ParentAgentServiceProjection::new(responses).route_snapshot(route)
}

pub(super) fn projected_subscription_event(
    route: ParentRouteId,
    responses: Vec<ParentAgentServiceProjectionResponse>,
) -> ParentSubscriptionEvent {
    let mut state = ParentRouteSubscriptionLoadState::default();
    ParentAgentServiceProjection::new(responses).subscription_event(&mut state, route)
}

pub(super) fn projected_action_result(
    action: &ParentUiAction,
    responses: Vec<ParentAgentServiceProjectionResponse>,
) -> ParentUiActionResult {
    require_ok(
        ParentAgentServiceProjection::new(responses).action_result(action),
        "typed action response projects",
    )
}

pub(super) fn require_ok<T, E: std::fmt::Debug>(result: Result<T, E>, context: &'static str) -> T {
    match result {
        Ok(value) => value,
        Err(error) => std::panic::resume_unwind(Box::new(format!("{context}: {error:?}"))),
    }
}

fn env_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

pub(super) fn with_isolated_agent_addr<T>(action: impl FnOnce() -> T) -> T {
    let _guard = require_ok(env_lock().lock(), "agent env lock remains available");
    let previous = std::env::var(constants::env_var::AGENT_ADDR).ok();
    std::env::remove_var(constants::env_var::AGENT_ADDR);
    let result = action();
    if let Some(value) = previous {
        std::env::set_var(constants::env_var::AGENT_ADDR, value);
    }
    result
}

fn ready_event() -> AgentEventEnvelope {
    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.connection.ready-1".to_string(),
        correlation_id: "ready".to_string(),
        sent_at: "2026-06-23T00:00:00Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentConnectionReady,
        severity: LogLevel::Info,
        payload: BTreeMap::new().into(),
        snapshot: None,
    }
}

pub(super) fn lan_event(
    event_name: AgentEventName,
    read_model: &LanBrowserAddDeviceReadModel,
) -> AgentEventEnvelope {
    let mut payload = BTreeMap::new();
    payload.insert(
        constants::field::LAN_ADD_DEVICE_READ_MODEL.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&read_model),
            "LAN read model serializes",
        )),
    );
    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.lan-pairing.event-1".to_string(),
        correlation_id: "lan".to_string(),
        sent_at: "2026-06-23T00:00:00Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: event_name,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}

pub(super) fn signed_child_agent_reported_event(
    read_model: &LanBrowserAddDeviceReadModel,
) -> AgentEventEnvelope {
    let mut payload = BTreeMap::new();
    payload.insert(
        constants::field::LAN_ADD_DEVICE_READ_MODEL.to_string(),
        LogFieldValue::String(require_ok(
            serde_json::to_string(&read_model),
            "LAN read model serializes",
        )),
    );
    payload.insert(
        constants::field::LAN_SIGNED_CHILD_AGENT_VERIFICATION.to_string(),
        LogFieldValue::String(
            constants::value::LAN_SIGNED_CHILD_AGENT_VERIFICATION_ACCEPTED.to_string(),
        ),
    );
    payload.insert(
        constants::field::LAN_SIGNED_CHILD_AGENT_MESSAGE_KIND.to_string(),
        LogFieldValue::String("hello".to_string()),
    );
    payload.insert(
        constants::field::LAN_SIGNED_CHILD_AGENT_REPLAY_OBSERVED_COUNT.to_string(),
        LogFieldValue::Number(1.0),
    );

    AgentEventEnvelope {
        schema_version: 1,
        event_id: "agent.lan-pairing.signed-child-agent.reported-1".to_string(),
        correlation_id: "lan-signed-child-agent".to_string(),
        sent_at: "2026-06-23T00:00:00Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentLanPairingSignedChildAgentReported,
        severity: LogLevel::Info,
        payload: payload.into(),
        snapshot: None,
    }
}

pub(super) fn sample_lan_read_model() -> LanBrowserAddDeviceReadModel {
    build_lan_add_device_read_model(LanAddDeviceReadModelInput {
        generated_at: "2026-06-23T00:00:00Z".to_string(),
        discovery_source: LanPairingDiscoverySource::PhysicalHouseholdLan,
        service_data_available: true,
        platform_data_available: true,
        add_device_state: LanPairingProductionDiscoveryState::Discovered,
        local_service_discovery_state: LanPairingProductionDiscoveryState::Discovered,
        physical_household_lan_state: LanPairingProductionDiscoveryState::Discovered,
        cloud_relay_state: LanPairingProductionDiscoveryState::Unavailable,
        discovered_devices: vec![LanBrowserAddDeviceDiscoveryDevice {
            schema_version: 1,
            discovered_at: "2026-06-23T00:00:00Z".to_string(),
            child_device: sample_child_device(),
            agent_peer_id: "local-dev-agent".to_string(),
            route_id: "route-local-network".to_string(),
            network_mode: LanPairingNetworkMode::LocalNetwork,
            reachability: LanPairingDeviceReachability::Online,
            address_ref: "network-neighbor".to_string(),
            discovery_status: LanPairingDiscoveryRuntimeStatus::NetworkNeighbor,
            discovery_state: LanPairingProductionDiscoveryState::Discovered,
            evidence_sources: vec![LanDiscoveryEvidenceSource::WindowsNeighborTable],
            pairing_id: None,
            service_identity_probe_evidence: Vec::new(),
            hint_sources: Vec::new(),
        }],
        pairing_requests: Vec::new(),
        trusted_device_registry: Vec::new(),
        household_device_decisions: Vec::new(),
        trusted_device_ids: Vec::new(),
        revoked_device_ids: Vec::new(),
        selected_device_readiness: LanSelectedDeviceReadiness {
            schema_version: 1,
            selected_child_device_id: None,
            route_id: None,
            pairing_id: None,
            trust_state: LanPairingTrustState::Unpaired,
            reachability: LanPairingDeviceReachability::Offline,
            ready_for_control: false,
            stale_at: None,
            offline_at: None,
        },
        controller_authority: LanPairingParentAuthority::ActiveController,
        observer_authority: LanPairingParentAuthority::Observer,
    })
}

fn sample_child_device() -> LanPairingDeviceRef {
    let mut device = LanPairingDeviceRef::new(
        "network-neighbor-1".to_string(),
        None,
        "Study Laptop".to_string(),
        "windows".to_string(),
    );
    device.ip_address = Some("192.168.1.24".to_string());
    device.mac_address = Some("aa-bb-cc-dd-ee-ff".to_string());
    device.hostname = Some("study-laptop".to_string());
    device.network_interface = Some("Ethernet".to_string());
    device
}
