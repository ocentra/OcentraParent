use super::{
    crate_name, AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName,
    AgentIdentity, AgentLogEntry, AgentLogSnapshot, AgentMessageTarget, AgentPairingProof,
    AgentPeer, AgentPeerRole, AgentRoute, AgentRouteSecurityPolicy, LogFieldValue, LogFields,
    LogLevel, LogSource, AGENT_PROTOCOL_SCHEMA_VERSION, LOG_SCHEMA_VERSION,
};

#[test]
fn crate_name_identifies_agent_protocol_boundary() {
    assert_eq!(crate_name(), "ocentra-parent-agent-protocol");
}

#[test]
fn agent_log_snapshot_serializes_to_typescript_contract_shape() {
    let mut fields = LogFields::new();
    fields.insert("captureEnabled".to_string(), LogFieldValue::Boolean(false));
    fields.insert("pid".to_string(), LogFieldValue::Number(1000.0));
    fields.insert("mode".to_string(), LogFieldValue::String("dev".to_string()));
    fields.insert("remoteSync".to_string(), LogFieldValue::Null(()));

    let snapshot = AgentLogSnapshot {
        schema_version: LOG_SCHEMA_VERSION,
        agent: AgentIdentity {
            device_id: "local-dev".to_string(),
            hostname: "devbox".to_string(),
            platform: "windows".to_string(),
            service_version: "0.1.0".to_string(),
        },
        entries: vec![AgentLogEntry {
            id: "dev-startup".to_string(),
            timestamp: "2026-05-19T00:00:00Z".to_string(),
            level: LogLevel::Info,
            source: LogSource::AgentService,
            message: "Agent service localhost API is reachable.".to_string(),
            fields,
        }],
    };

    let serialized = serde_json::to_value(snapshot).expect("snapshot serializes");

    assert_eq!(serialized["schemaVersion"], 1);
    assert_eq!(serialized["agent"]["deviceId"], "local-dev");
    assert_eq!(serialized["entries"][0]["level"], "info");
    assert_eq!(serialized["entries"][0]["source"], "agent-service");
    assert_eq!(serialized["entries"][0]["fields"]["captureEnabled"], false);
    assert!(serialized["entries"][0]["fields"]["remoteSync"].is_null());
}

#[test]
fn websocket_command_envelope_serializes_to_typescript_contract_shape() {
    let command = AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: "cmd-1".to_string(),
        sent_at: "2026-05-19T00:00:00Z".to_string(),
        source: AgentPeer {
            peer_id: "portal-dev".to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: "local-dev-agent".to_string(),
            platform: "windows".to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentHealthCheck,
        payload: LogFields::new(),
    };

    let serialized = serde_json::to_value(command).expect("command serializes");

    assert_eq!(serialized["schemaVersion"], 1);
    assert_eq!(serialized["source"]["role"], "portal");
    assert_eq!(serialized["target"]["route"], "localhost");
    assert_eq!(serialized["command"], "agent.health.check");
}

#[test]
fn local_network_route_serializes_to_typescript_contract_shape() {
    let serialized = serde_json::to_value(AgentRoute::LocalNetwork).expect("route serializes");

    assert_eq!(serialized, "local-network");
}

#[test]
fn local_network_route_security_rejects_anonymous_control() {
    let policy = AgentRouteSecurityPolicy {
        route: AgentRoute::LocalNetwork,
        requires_pairing: true,
        allows_anonymous_control: false,
    };

    let serialized = serde_json::to_value(policy).expect("route security serializes");

    assert_eq!(serialized["route"], "local-network");
    assert_eq!(serialized["requiresPairing"], true);
    assert_eq!(serialized["allowsAnonymousControl"], false);
}

#[test]
fn pairing_proof_serializes_without_raw_pairing_token() {
    let proof = AgentPairingProof {
        pairing_id: "pairing-local-dev".to_string(),
        device_id: "local-dev-agent".to_string(),
        parent_peer_id: "portal-dev".to_string(),
        issued_at: "2026-05-19T00:00:00Z".to_string(),
        expires_at: "2026-05-19T00:05:00Z".to_string(),
        token_hash: "sha256:local-dev-token-hash".to_string(),
    };

    let serialized = serde_json::to_value(proof).expect("pairing proof serializes");
    let serialized_text =
        serde_json::to_string(&serialized).expect("pairing proof serializes to text");

    assert_eq!(serialized["tokenHash"], "sha256:local-dev-token-hash");
    assert!(!serialized_text.contains("rawToken"));
}

#[test]
fn websocket_event_envelope_serializes_to_typescript_contract_shape() {
    let mut payload = LogFields::new();
    payload.insert("online".to_string(), LogFieldValue::Boolean(true));

    let event = AgentEventEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        event_id: "evt-1".to_string(),
        correlation_id: "cmd-1".to_string(),
        sent_at: "2026-05-19T00:00:01Z".to_string(),
        source: AgentPeer {
            peer_id: "local-dev-agent".to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: "portal-dev".to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentHealthReported,
        severity: LogLevel::Info,
        payload,
        snapshot: None,
    };

    let serialized = serde_json::to_value(event).expect("event serializes");

    assert_eq!(serialized["schemaVersion"], 1);
    assert_eq!(serialized["source"]["role"], "agent-service");
    assert_eq!(serialized["event"], "agent.health.reported");
    assert_eq!(serialized["payload"]["online"], true);
    assert!(serialized["snapshot"].is_null());
}
