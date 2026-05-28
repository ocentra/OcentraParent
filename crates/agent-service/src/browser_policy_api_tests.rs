use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, AgentCommandEnvelope, AgentCommandName,
    AgentMessageTarget, AgentPeer, AgentPeerRole, AgentRoute, BrowserPolicyPatch,
    BrowserPolicyPatchRequest, BrowserPolicyRejectionReason, BrowserPolicyUpdateKind,
    BrowserPolicyUpdateResponse, BrowserPolicyUpdateStatus, LogFieldValue, LogFields,
    AGENT_PROTOCOL_SCHEMA_VERSION,
};

use crate::{lan_pairing::LanPairingRuntime, websocket::handle_command_text_for_test};

#[tokio::test]
async fn browser_policy_patch_returns_typed_scaffold_unavailable_response() {
    let event = handle_command_text_for_test(
        &serde_json::to_string(&browser_policy_patch_command())
            .expect(constants::error::AGENT_EVENT_SERIALIZES),
        LanPairingRuntime::empty(),
        None,
    )
    .await;
    let response = match event.payload.get(constants::field::BROWSER_POLICY_RESPONSE) {
        Some(LogFieldValue::String(text)) => {
            serde_json::from_str::<BrowserPolicyUpdateResponse>(text)
                .expect(constants::error::AGENT_EVENT_SERIALIZES)
        }
        _ => unreachable!(),
    };

    assert_eq!(
        event.event,
        ocentra_parent_agent_protocol::AgentEventName::AgentBrowserPolicyPatchRejected
    );
    assert_eq!(response.kind, BrowserPolicyUpdateKind::Patch);
    assert_eq!(response.status, BrowserPolicyUpdateStatus::Rejected);
    assert_eq!(
        response.rejection_reason,
        Some(BrowserPolicyRejectionReason::ScaffoldUnavailable)
    );
    assert_eq!(
        event
            .payload
            .get(constants::field::BROWSER_POLICY_REJECTION_REASON),
        Some(&LogFieldValue::String(
            constants::browser_policy::REJECTION_SCAFFOLD_UNAVAILABLE.to_string()
        ))
    );
}

fn browser_policy_patch_command() -> AgentCommandEnvelope {
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
        LogFieldValue::String(
            serde_json::to_string(&request).expect(constants::error::AGENT_EVENT_SERIALIZES),
        ),
    );
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::browser_policy::COMMAND_MESSAGE_ID.to_string(),
        sent_at: constants::browser_policy::TEST_SENT_AT.to_string(),
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
    }
}
