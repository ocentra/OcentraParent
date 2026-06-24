use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::network_windows_wfp_gate_status::{
    NetworkWindowsWfpGateCapabilityStatusState, NetworkWindowsWfpGateStatus,
    NetworkWindowsWfpGateStatusState,
};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentCommandName;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use ocentra_parent_agent_protocol::transport::AgentMessageTarget;
use ocentra_parent_agent_protocol::transport::AgentPeer;
use ocentra_parent_agent_protocol::transport::AgentPeerRole;
use ocentra_parent_agent_protocol::transport::AgentRoute;
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use serde::de::DeserializeOwned;

use crate::{
    lan_pairing::LanPairingRuntime,
    network_windows_wfp_gate_status_bridge::network_windows_wfp_gate_status_payload,
    websocket::handle_command_text_for_test,
};

#[test]
fn network_windows_wfp_gate_status_payload_reports_lab_ready_without_execution_claims() {
    let payload = network_windows_wfp_gate_status_payload()
        .unwrap_or_else(|error| panic!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES));
    let status: NetworkWindowsWfpGateStatus = status_value(
        &payload,
        constants::network_flow::FIELD_NETWORK_WINDOWS_WFP_GATE_STATUS,
    );

    assert_windows_wfp_status(&status);
}

#[tokio::test]
async fn websocket_network_windows_wfp_gate_status_command_reports_payload() {
    let body = serde_json::to_string(&command_envelope())
        .unwrap_or_else(|error| panic!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES));
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let status: NetworkWindowsWfpGateStatus = status_value(
        &event.payload,
        constants::network_flow::FIELD_NETWORK_WINDOWS_WFP_GATE_STATUS,
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentNetworkWindowsWfpGateStatusReported
    );
    assert_windows_wfp_status(&status);
}

fn assert_windows_wfp_status(status: &NetworkWindowsWfpGateStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_WINDOWS_WFP_GATE_STATUS_REF
    );
    assert_eq!(
        status.wfp_gate_ref,
        constants::network_flow::TEST_WINDOWS_WFP_GATE_REF
    );
    assert_eq!(
        status.capability_state,
        NetworkWindowsWfpGateCapabilityStatusState::LabReady
    );
    assert_eq!(
        status.gate_state,
        NetworkWindowsWfpGateStatusState::LabProofReady
    );
    assert!(status.boundary_reasons.is_empty());
    assert!(status.missing_required_artifacts.is_empty());
    assert!(status.wfp_lab_proof_ready);
    assert_non_claims(status);
    assert_artifact_refs(status);
}

fn assert_non_claims(status: &NetworkWindowsWfpGateStatus) {
    assert!(!status.adapter_apply_authorized);
    assert!(!status.enforcement_command_published);
    assert!(!status.live_driver_install_claimed);
    assert!(!status.callout_registration_claimed);
    assert!(!status.packet_block_claimed);
    assert!(!status.kernel_payload_inspection_claimed);
    assert!(!status.command_invocation_claimed);
    assert!(!status.exact_url_available);
    assert!(!status.decrypted_payload_available);
    assert!(!status.page_content_available);
}

fn assert_artifact_refs(status: &NetworkWindowsWfpGateStatus) {
    assert_eq!(
        status.administrator_permission_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_WINDOWS_WFP_ADMIN_PERMISSION_PROOF_REF)
    );
    assert_eq!(
        status.driver_signing_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_WINDOWS_WFP_DRIVER_SIGNING_PROOF_REF)
    );
    assert_eq!(
        status.driver_package_proof_ref.as_deref(),
        Some(constants::network_flow::TEST_WINDOWS_WFP_DRIVER_PACKAGE_PROOF_REF)
    );
    assert_eq!(
        status.lab_result_artifact_ref.as_deref(),
        Some(constants::network_flow::TEST_WINDOWS_WFP_LAB_RESULT_ARTIFACT_REF)
    );
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::NETWORK_WINDOWS_WFP_GATE_STATUS_REPORTED.to_string(),
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
        command: AgentCommandName::AgentNetworkWindowsWfpGateStatusGet,
        payload: Default::default(),
    }
}

fn status_value<TStatus: DeserializeOwned>(
    payload: &ocentra_parent_agent_protocol::logging::LogFields,
    field: &str,
) -> TStatus {
    match payload.get(field) {
        Some(LogFieldValue::String(text)) => serde_json::from_str(text).unwrap_or_else(|error| {
            panic!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES)
        }),
        other => panic!(
            "{}: missing or non-string payload field {field}: {other:?}",
            constants::error::AGENT_EVENT_SERIALIZES
        ),
    }
}
