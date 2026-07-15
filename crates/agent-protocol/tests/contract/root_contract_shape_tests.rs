use super::{
    crate_name, AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope, AgentEventName,
    AgentIdentity, AgentLogEntry, AgentLogSnapshot, AgentMessageTarget, AgentPeer, AgentPeerRole,
    AgentRoute, LogFieldValue, LogFields, LogLevel, LogSource, AGENT_PROTOCOL_SCHEMA_VERSION,
    LOG_SCHEMA_VERSION,
};
use crate::logging::DevLogEntry;
use ocentra_eventing::expect_value::ExpectValue;

fn to_json<T: serde::Serialize>(value: T) -> serde_json::Value {
    serde_json::to_value(value).expect_value("root contract shape serializes")
}

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
            schema_version: LOG_SCHEMA_VERSION,
            id: "dev-startup".to_string(),
            timestamp: "2026-05-19T00:00:00Z".to_string(),
            level: LogLevel::Info,
            source: LogSource::AgentService,
            message: "Agent service localhost API is reachable.".to_string(),
            fields,
        }],
    };

    let serialized = to_json(snapshot);

    assert_eq!(serialized["schemaVersion"], 1);
    assert_eq!(serialized["agent"]["deviceId"], "local-dev");
    assert_eq!(serialized["entries"][0]["schemaVersion"], 1);
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

    let serialized = to_json(command);

    assert_eq!(serialized["schemaVersion"], 1);
    assert_eq!(serialized["source"]["role"], "portal");
    assert_eq!(serialized["target"]["route"], "localhost");
    assert_eq!(serialized["command"], "agent.health.check");
}

#[test]
fn policy_preview_command_and_event_names_serialize_to_contract_shape() {
    let command = to_json(AgentCommandName::AgentPolicyPreviewReadModelGet);
    let event = to_json(AgentEventName::AgentPolicyPreviewReadModelReported);

    assert_eq!(command, "agent.policy.preview.read-model.get");
    assert_eq!(event, "agent.policy.preview.read-model.reported");
}

#[test]
fn enforcement_command_and_event_names_serialize_to_contract_shape() {
    let execute_command = to_json(AgentCommandName::AgentEnforcementExecute);
    let recover_command = to_json(AgentCommandName::AgentEnforcementTimerRecover);
    let expire_command = to_json(AgentCommandName::AgentEnforcementTimerExpire);
    let cancel_command = to_json(AgentCommandName::AgentEnforcementOverrideCancel);
    let audit_event = to_json(AgentEventName::AgentEnforcementAuditReported);
    let timer_event = to_json(AgentEventName::AgentEnforcementTimerReported);

    assert_eq!(execute_command, "agent.enforcement.execute");
    assert_eq!(recover_command, "agent.enforcement.timer.recover");
    assert_eq!(expire_command, "agent.enforcement.timer.expire");
    assert_eq!(cancel_command, "agent.enforcement.override.cancel");
    assert_eq!(audit_event, "agent.enforcement.audit.reported");
    assert_eq!(timer_event, "agent.enforcement.timer.reported");
}

#[test]
fn browser_intervention_command_and_event_names_serialize_to_contract_shape() {
    let command = to_json(AgentCommandName::AgentBrowserInterventionReadModelGet);
    let event = to_json(AgentEventName::AgentBrowserInterventionReadModelReported);

    assert_eq!(command, "agent.browser.intervention.read-model.get");
    assert_eq!(event, "agent.browser.intervention.read-model.reported");
}

#[test]
fn network_runtime_stream_command_and_event_names_serialize_to_contract_shape() {
    let command = to_json(AgentCommandName::AgentNetworkRuntimeEventChainStreamGet);
    let event = to_json(AgentEventName::AgentNetworkRuntimeEventChainStreamReported);

    assert_eq!(command, "agent.network.runtime.event-chain.stream.get");
    assert_eq!(event, "agent.network.runtime.event-chain.stream.reported");
}

#[test]
fn browser_runtime_stream_command_and_event_names_serialize_to_contract_shape() {
    let command = to_json(AgentCommandName::AgentBrowserRuntimeEventChainStreamGet);
    let event = to_json(AgentEventName::AgentBrowserRuntimeEventChainStreamReported);

    assert_eq!(command, "agent.browser.runtime.event-chain.stream.get");
    assert_eq!(event, "agent.browser.runtime.event-chain.stream.reported");
}

#[test]
fn network_remote_delivery_status_command_and_event_names_serialize_to_contract_shape() {
    let command = to_json(AgentCommandName::AgentNetworkRemoteDeliveryStatusGet);
    let event = to_json(AgentEventName::AgentNetworkRemoteDeliveryStatusReported);

    assert_eq!(command, "agent.network.remote-delivery.status.get");
    assert_eq!(event, "agent.network.remote-delivery.status.reported");
}

#[test]
fn network_linux_nftables_lab_status_command_and_event_names_serialize_to_contract_shape() {
    let command = to_json(AgentCommandName::AgentNetworkLinuxNftablesLabStatusGet);
    let event = to_json(AgentEventName::AgentNetworkLinuxNftablesLabStatusReported);

    assert_eq!(command, "agent.network.linux-nftables-lab.status.get");
    assert_eq!(event, "agent.network.linux-nftables-lab.status.reported");
}

#[test]
fn network_windows_firewall_lab_status_command_and_event_names_serialize_to_contract_shape() {
    let command = to_json(AgentCommandName::AgentNetworkWindowsFirewallLabStatusGet);
    let event = to_json(AgentEventName::AgentNetworkWindowsFirewallLabStatusReported);

    assert_eq!(command, "agent.network.windows-firewall-lab.status.get");
    assert_eq!(event, "agent.network.windows-firewall-lab.status.reported");
}

#[test]
fn network_windows_wfp_gate_status_command_and_event_names_serialize_to_contract_shape() {
    let command = to_json(AgentCommandName::AgentNetworkWindowsWfpGateStatusGet);
    let event = to_json(AgentEventName::AgentNetworkWindowsWfpGateStatusReported);

    assert_eq!(command, "agent.network.windows-wfp-gate.status.get");
    assert_eq!(event, "agent.network.windows-wfp-gate.status.reported");
}

#[test]
fn network_android_vpn_service_gate_status_command_and_event_names_serialize_to_contract_shape() {
    let command = to_json(AgentCommandName::AgentNetworkAndroidVpnServiceGateStatusGet);
    let event = to_json(AgentEventName::AgentNetworkAndroidVpnServiceGateStatusReported);

    assert_eq!(command, "agent.network.android-vpn-service-gate.status.get");
    assert_eq!(
        event,
        "agent.network.android-vpn-service-gate.status.reported"
    );
}

#[test]
fn network_apple_network_extension_gate_status_command_and_event_names_serialize_to_contract_shape()
{
    let command = to_json(AgentCommandName::AgentNetworkAppleNetworkExtensionGateStatusGet);
    let event = to_json(AgentEventName::AgentNetworkAppleNetworkExtensionGateStatusReported);

    assert_eq!(
        command,
        "agent.network.apple-network-extension-gate.status.get"
    );
    assert_eq!(
        event,
        "agent.network.apple-network-extension-gate.status.reported"
    );
}

#[test]
fn browser_inventory_command_and_event_names_serialize_to_contract_shape() {
    let command = to_json(AgentCommandName::AgentBrowserInventoryReadModelGet);
    let event = to_json(AgentEventName::AgentBrowserInventoryReadModelReported);

    assert_eq!(command, "agent.browser.inventory.read-model.get");
    assert_eq!(event, "agent.browser.inventory.read-model.reported");
}

#[test]
fn app_game_timer_parent_surface_command_and_event_names_serialize_to_contract_shape() {
    let command = to_json(AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet);
    let event = to_json(AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported);

    assert_eq!(
        command,
        "agent.activity.app-game.timer-parent-surface.read-model.get"
    );
    assert_eq!(
        event,
        "agent.activity.app-game.timer-parent-surface.read-model.reported"
    );
}

#[test]
fn app_game_adapter_dispatch_execute_command_and_event_names_serialize_to_contract_shape() {
    let command = to_json(AgentCommandName::AgentActivityAppGameAdapterDispatchExecute);
    let event = to_json(AgentEventName::AgentActivityAppGameAdapterDispatchExecuted);

    assert_eq!(command, "agent.activity.app-game.adapter-dispatch.execute");
    assert_eq!(event, "agent.activity.app-game.adapter-dispatch.executed");
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

    let serialized = to_json(event);

    assert_eq!(serialized["schemaVersion"], 1);
    assert_eq!(serialized["source"]["role"], "agent-service");
    assert_eq!(serialized["event"], "agent.health.reported");
    assert_eq!(serialized["payload"]["online"], true);
    assert!(serialized["snapshot"].is_null());
}

#[test]
fn dev_log_entry_serializes_to_typescript_ndjson_shape() {
    let entry = DevLogEntry {
        schema_version: LOG_SCHEMA_VERSION,
        id: "agent-log-1".to_string(),
        timestamp: "2026-05-20T00:00:00Z".to_string(),
        level: LogLevel::Info,
        source: LogSource::AgentService,
        message: "Agent service dev runtime started.".to_string(),
        fields: LogFields::new(),
    };

    let serialized = to_json(entry);

    assert_eq!(serialized["schemaVersion"], 1);
    assert_eq!(serialized["source"], "agent-service");
    assert_eq!(serialized["message"], "Agent service dev runtime started.");
}
