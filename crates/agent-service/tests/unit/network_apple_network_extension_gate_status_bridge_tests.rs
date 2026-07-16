use std::primitive::str as TestStr;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_apple_network_extension_gate_status::{
    NetworkAppleNetworkExtensionGateCapabilityStatusState, NetworkAppleNetworkExtensionGateStatus,
    NetworkAppleNetworkExtensionGateStatusState, NetworkAppleNetworkExtensionPlatformStatus,
};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::{
    handle_local_command_text_for_test,
    network_apple_network_extension_gate_status_payload_for_test,
};
use serde::de::DeserializeOwned;

#[test]
fn network_apple_network_extension_gate_status_payload_reports_entitlement_ready_without_execution_claims(
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = network_apple_network_extension_gate_status_payload_for_test()?;
    let status: NetworkAppleNetworkExtensionGateStatus = status_value(
        &payload,
        constants::network_flow::FIELD_NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS,
    )?;

    assert_apple_network_extension_status(&status);
    Ok(())
}

#[tokio::test]
async fn websocket_network_apple_network_extension_gate_status_command_reports_payload(
) -> Result<(), Box<dyn std::error::Error>> {
    let body = serde_json::to_string(&command_envelope())?;
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let status: NetworkAppleNetworkExtensionGateStatus = status_value(
        &event.payload,
        constants::network_flow::FIELD_NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS,
    )?;

    assert_eq!(
        event.event,
        AgentEventName::AgentNetworkAppleNetworkExtensionGateStatusReported
    );
    assert_apple_network_extension_status(&status);
    Ok(())
}

fn assert_apple_network_extension_status(status: &NetworkAppleNetworkExtensionGateStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_STATUS_REF
    );
    assert_eq!(
        status.apple_network_extension_gate_ref,
        constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_GATE_REF
    );
    assert_eq!(
        status.platform,
        NetworkAppleNetworkExtensionPlatformStatus::Ios
    );
    assert_eq!(
        status.capability_state,
        NetworkAppleNetworkExtensionGateCapabilityStatusState::AppleDeviceReady
    );
    assert_eq!(
        status.gate_state,
        NetworkAppleNetworkExtensionGateStatusState::AppleEntitlementProofReady
    );
    assert!(status.boundary_reasons.is_empty());
    assert!(status.missing_required_artifacts.is_empty());
    assert!(status.apple_entitlement_proof_ready);
    assert!(!status.supervision_required);
    assert!(!status.supervision_authority_proved);
    assert_artifact_refs(status);
    assert_non_claims(status);
}

fn assert_artifact_refs(status: &NetworkAppleNetworkExtensionGateStatus) {
    assert_eq!(
        status.developer_team_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_DEVELOPER_TEAM_PROOF_REF)
    );
    assert_eq!(
        status.entitlement_approval_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_ENTITLEMENT_APPROVAL_PROOF_REF)
    );
    assert_eq!(
        status.network_extension_declaration_ref.as_deref(),
        Some(constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_DECLARATION_REF)
    );
    assert_eq!(
        status.rollback_plan_ref.as_deref(),
        Some(constants::network_flow::TEST_APPLE_NETWORK_EXTENSION_ROLLBACK_PLAN_REF)
    );
}

fn assert_non_claims(status: &NetworkAppleNetworkExtensionGateStatus) {
    assert!(!status.adapter_apply_authorized);
    assert!(!status.enforcement_command_published);
    assert!(!status.simulator_only_product_support_claimed);
    assert!(!status.live_network_extension_claimed);
    assert!(!status.packet_block_claimed);
    assert!(!status.app_level_control_claimed);
    assert!(!status.exact_url_available);
    assert!(!status.decrypted_payload_available);
    assert!(!status.page_content_available);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::NETWORK_APPLE_NETWORK_EXTENSION_GATE_STATUS_REPORTED
            .to_string(),
        sent_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        source: AgentPeer {
            peer_id: constants::peer::PORTAL_DEV.to_string(),
            role: AgentPeerRole::Portal,
        },
        target: AgentMessageTarget {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: policy_constants::TEST_PARENT_DEVICE_PLATFORM_WINDOWS.to_string(),
            route: AgentRoute::Localhost,
        },
        command: AgentCommandName::AgentNetworkAppleNetworkExtensionGateStatusGet,
        payload: Default::default(),
    }
}

fn status_value<TStatus: DeserializeOwned>(
    payload: &ocentra_parent_agent_protocol::logging::LogFields,
    field: &TestStr,
) -> Result<TStatus, Box<dyn std::error::Error>> {
    let text = match payload.get(field) {
        Some(ocentra_parent_agent_protocol::logging::LogFieldValue::String(text)) => text,
        other => {
            return Err(std::io::Error::other(format!(
                "{}: {other:?}",
                constants::error::AGENT_EVENT_SERIALIZES
            ))
            .into());
        }
    };
    Ok(serde_json::from_str(text)?)
}
