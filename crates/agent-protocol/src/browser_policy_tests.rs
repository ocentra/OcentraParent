use super::{
    constants, policy_constants as policy, AgentCommandEnvelope, AgentCommandName,
    AgentEventEnvelope, AgentEventName, AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute,
    BrowserPolicyPatch, BrowserPolicyPatchRequest, BrowserPolicyRejectionReason,
    BrowserPolicyUpdateKind, BrowserPolicyUpdateResponse, BrowserPolicyUpdateStatus, LogFieldValue,
    LogFields, LogLevel, AGENT_PROTOCOL_SCHEMA_VERSION,
};

#[test]
fn browser_policy_patch_command_serializes_to_typescript_contract_shape() {
    let request = BrowserPolicyPatchRequest {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id: constants::browser_policy::REQUEST_ID.to_string(),
        kind: BrowserPolicyUpdateKind::Patch,
        policy_id: constants::browser_policy::POLICY_ID.to_string(),
        base_revision_id: constants::browser_policy::REVISION_ID.to_string(),
        patches: vec![BrowserPolicyPatch {
            op: constants::browser_policy::PATCH_OPERATION_REPLACE.to_string(),
            field_id: constants::browser_policy::FIELD_ID_ENABLED.to_string(),
            writes_to: constants::browser_policy::WRITES_TO_ENABLED.to_string(),
            value: serde_json::Value::Bool(true),
        }],
    };
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::BROWSER_POLICY_REQUEST.to_string(),
        LogFieldValue::String(serde_json::to_string(&request).expect("request serializes")),
    );
    payload.insert(
        constants::field::BROWSER_POLICY_UPDATE_KIND.to_string(),
        LogFieldValue::String(BrowserPolicyUpdateKind::Patch.as_protocol_str().to_string()),
    );

    let command = AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: "cmd-browser-policy".to_string(),
        sent_at: "2026-05-28T17:30:00Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: constants::enforcement::PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentBrowserPolicyPatch,
        payload,
    };
    let serialized = serde_json::to_value(command).expect("browser policy command serializes");
    let request_text = serialized["payload"][constants::field::BROWSER_POLICY_REQUEST]
        .as_str()
        .expect("request is encoded as JSON text");
    let request_value: serde_json::Value =
        serde_json::from_str(request_text).expect("request payload decodes as JSON");

    assert_eq!(
        serialized["command"],
        constants::browser_policy::COMMAND_PATCH
    );
    assert_eq!(
        serialized["payload"][constants::field::BROWSER_POLICY_UPDATE_KIND],
        constants::browser_policy::UPDATE_KIND_PATCH
    );
    assert_eq!(
        request_value["kind"],
        constants::browser_policy::UPDATE_KIND_PATCH
    );
    assert_eq!(
        request_value["patches"][0]["writesTo"],
        constants::browser_policy::WRITES_TO_ENABLED
    );
}

#[test]
fn browser_policy_rejected_event_serializes_typed_reason() {
    let response = BrowserPolicyUpdateResponse {
        schema_version: policy::CONTRACT_SCHEMA_VERSION_V0_6.to_string(),
        request_id: constants::browser_policy::REQUEST_ID.to_string(),
        kind: BrowserPolicyUpdateKind::Patch,
        status: BrowserPolicyUpdateStatus::Rejected,
        policy: None,
        effective_policy: None,
        capability_registry: None,
        rejection_reason: Some(BrowserPolicyRejectionReason::ScaffoldUnavailable),
        audit_event_id: None,
        message: Some(constants::browser_policy::SCAFFOLD_UNAVAILABLE_MESSAGE.to_string()),
    };
    let mut payload = LogFields::new();
    payload.insert(
        constants::field::BROWSER_POLICY_RESPONSE.to_string(),
        LogFieldValue::String(serde_json::to_string(&response).expect("response serializes")),
    );
    payload.insert(
        constants::field::BROWSER_POLICY_REJECTION_REASON.to_string(),
        LogFieldValue::String(
            constants::browser_policy::REJECTION_SCAFFOLD_UNAVAILABLE.to_string(),
        ),
    );

    let event = AgentEventEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        event_id: constants::event_id::BROWSER_POLICY_PATCH_REJECTED.to_string(),
        correlation_id: "cmd-browser-policy".to_string(),
        sent_at: "2026-05-28T17:30:01Z".to_string(),
        source: AgentPeer {
            peer_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            role: AgentPeerRole::AgentService,
        },
        target: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        event: AgentEventName::AgentBrowserPolicyPatchRejected,
        severity: LogLevel::Warn,
        payload,
        snapshot: None,
    };
    let serialized = serde_json::to_value(event).expect("browser policy event serializes");
    let response_text = serialized["payload"][constants::field::BROWSER_POLICY_RESPONSE]
        .as_str()
        .expect("response is encoded as JSON text");
    let response_value: serde_json::Value =
        serde_json::from_str(response_text).expect("response payload decodes as JSON");

    assert_eq!(
        serialized["event"],
        constants::browser_policy::EVENT_PATCH_REJECTED
    );
    assert_eq!(
        serialized["payload"][constants::field::BROWSER_POLICY_REJECTION_REASON],
        constants::browser_policy::REJECTION_SCAFFOLD_UNAVAILABLE
    );
    assert_eq!(
        response_value["rejectionReason"],
        constants::browser_policy::REJECTION_SCAFFOLD_UNAVAILABLE
    );
}
