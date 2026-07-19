use std::collections::{BTreeMap, VecDeque};
use std::net::TcpListener;
use std::sync::{mpsc, Arc, Mutex, OnceLock};
use std::thread;
use std::time::Duration;

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
    AgentEventEnvelope, AgentEventName, AgentPeer, AgentPeerRole,
};
use serde_json::Value;
use tungstenite::{
    accept_hdr,
    handshake::server::{Request, Response},
    Message,
};

pub(super) const REQUEST_MESSAGE_ID_CORRELATION: &str = "test-request-message-id";

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

pub(super) fn with_agent_addr<T>(address: &str, action: impl FnOnce() -> T) -> T {
    let _guard = require_ok(env_lock().lock(), "agent env lock remains available");
    let previous = std::env::var(constants::env_var::AGENT_ADDR).ok();
    std::env::set_var(constants::env_var::AGENT_ADDR, address);
    let result = action();
    if let Some(value) = previous {
        std::env::set_var(constants::env_var::AGENT_ADDR, value);
    } else {
        std::env::remove_var(constants::env_var::AGENT_ADDR);
    }
    result
}

pub(super) fn start_lan_local_server(
    event_name: AgentEventName,
    read_model: LanBrowserAddDeviceReadModel,
) -> String {
    let (address, _capture) = start_lan_local_server_with_capture(event_name, read_model);
    address
}

pub(super) fn start_lan_local_server_with_capture(
    event_name: AgentEventName,
    read_model: LanBrowserAddDeviceReadModel,
) -> (String, mpsc::Receiver<CapturedLanRequest>) {
    let response_event = lan_event(event_name, &read_model);
    drop(read_model);
    start_local_server_with_capture_response(response_event)
}

pub(super) fn start_local_server_with_capture_response(
    response_event: AgentEventEnvelope,
) -> (String, mpsc::Receiver<CapturedLanRequest>) {
    start_local_server_with_capture_responses(vec![response_event])
}

pub(super) fn start_local_server_with_capture_responses(
    response_events: Vec<AgentEventEnvelope>,
) -> (String, mpsc::Receiver<CapturedLanRequest>) {
    start_local_server_with_capture_responses_inner(response_events, false)
}

pub(super) fn start_local_server_with_ready_only() -> (String, mpsc::Receiver<CapturedLanRequest>) {
    start_local_server_with_capture_responses_inner(Vec::new(), true)
}

fn start_local_server_with_capture_responses_inner(
    response_events: Vec<AgentEventEnvelope>,
    accept_without_response: bool,
) -> (String, mpsc::Receiver<CapturedLanRequest>) {
    let listener = require_ok(TcpListener::bind("127.0.0.1:0"), "local listener binds");
    let address = require_ok(listener.local_addr(), "local listener exposes address");
    let (tx, rx) = mpsc::channel();
    let observed_origin = Arc::new(Mutex::new(None::<String>));
    let response_queue = Arc::new(Mutex::new(VecDeque::from(response_events)));
    let response_queue_for_thread = Arc::clone(&response_queue);
    thread::spawn(move || loop {
        let next_response = {
            let mut queue = require_ok(
                response_queue_for_thread.lock(),
                "response queue lock remains available",
            );
            queue.pop_front()
        };
        if next_response.is_none() && !accept_without_response {
            break;
        }
        let (stream, _) = require_ok(listener.accept(), "local listener accepts");
        let header_origin = Arc::clone(&observed_origin);
        let socket = accept_hdr(stream, move |request: &Request, response: Response| {
            *require_ok(
                header_origin.lock(),
                "captured header origin lock remains available",
            ) = request_origin(request);
            Ok(response)
        });
        let mut socket = require_ok(socket, "local websocket handshake succeeds");
        send_json_message(&mut socket, &ready_event(), "ready event sends");
        let command_text = expect_text_message(require_ok(socket.read(), "command reads"));
        let command: Value = require_ok(serde_json::from_str(&command_text), "command parses");
        let command_message_id = command
            .get("messageId")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_string();
        let _ = tx.send(CapturedLanRequest {
            origin: require_ok(
                observed_origin.lock(),
                "captured origin lock remains available",
            )
            .clone(),
            command,
        });
        if let Some(mut response_event) = next_response {
            if response_event.correlation_id == REQUEST_MESSAGE_ID_CORRELATION {
                assert!(
                    !command_message_id.is_empty(),
                    "correlated local response requires command messageId"
                );
                response_event.correlation_id = command_message_id;
            }
            send_json_message(&mut socket, &response_event, "response event sends");
        } else {
            thread::sleep(Duration::from_millis(750));
            break;
        }
    });
    (address.to_string(), rx)
}

fn request_origin(request: &Request) -> Option<String> {
    request
        .headers()
        .get("origin")
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned)
}

fn send_json_message(
    socket: &mut tungstenite::WebSocket<std::net::TcpStream>,
    value: &AgentEventEnvelope,
    context: &'static str,
) {
    require_ok(
        socket.send(Message::Text(require_ok(
            serde_json::to_string(value),
            "agent event serializes",
        ))),
        context,
    );
}

fn expect_text_message(message: Message) -> String {
    let is_text = matches!(message, Message::Text(_));
    assert!(
        is_text,
        "local agent receives one text command as text frame"
    );
    match message {
        Message::Text(text) => text,
        _ => String::new(),
    }
}

#[derive(Debug)]
pub(super) struct CapturedLanRequest {
    pub(super) origin: Option<String>,
    pub(super) command: Value,
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
