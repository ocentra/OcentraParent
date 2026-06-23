use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::network_linux_nftables_lab_status::{
    NetworkLinuxNftablesLabCommandStatusKind, NetworkLinuxNftablesLabStatus,
    NetworkLinuxNftablesLabStatusState,
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
    network_linux_nftables_lab_status_bridge::network_linux_nftables_lab_status_payload,
    websocket::handle_command_text_for_test,
};

#[test]
fn network_linux_nftables_lab_status_payload_reports_bounded_lab_execution_only() {
    let payload = network_linux_nftables_lab_status_payload().unwrap_or_else(|error| {
        panic!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES)
    });
    let status: NetworkLinuxNftablesLabStatus = status_value(
        &payload,
        constants::network_flow::FIELD_NETWORK_LINUX_NFTABLES_LAB_STATUS,
    );

    assert_linux_nftables_status(&status);
}

#[tokio::test]
async fn websocket_network_linux_nftables_lab_status_command_reports_payload() {
    let body = serde_json::to_string(&command_envelope()).unwrap_or_else(|error| {
        panic!("{}: {error:?}", constants::error::AGENT_EVENT_SERIALIZES)
    });
    let event = handle_command_text_for_test(&body, LanPairingRuntime::empty(), None).await;
    let status: NetworkLinuxNftablesLabStatus = status_value(
        &event.payload,
        constants::network_flow::FIELD_NETWORK_LINUX_NFTABLES_LAB_STATUS,
    );

    assert_eq!(
        event.event,
        AgentEventName::AgentNetworkLinuxNftablesLabStatusReported
    );
    assert_linux_nftables_status(&status);
}

fn assert_linux_nftables_status(status: &NetworkLinuxNftablesLabStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_LINUX_NFTABLES_LAB_STATUS_REF
    );
    assert_eq!(
        status.lab_ref,
        constants::network_flow::TEST_LINUX_NFTABLES_LAB_REF
    );
    assert_eq!(
        status.linux_adapter_gate_ref,
        constants::network_flow::TEST_LINUX_ADAPTER_GATE_REF
    );
    assert_eq!(
        status.state,
        NetworkLinuxNftablesLabStatusState::ExecutedAndRolledBack
    );
    assert!(status.wsl_host_observed);
    assert!(status.root_permission_observed);
    assert!(status.nft_tool_observed);
    assert_eq!(status.command_count, 6);
    assert_eq!(status.required_command_count, 6);
    assert!(status.table_create_observed);
    assert!(status.chain_create_observed);
    assert!(status.rule_add_observed);
    assert!(status.verify_present_observed);
    assert!(status.rollback_observed);
    assert!(status.verify_removed_observed);
    assert!(status.lab_packet_filter_rule_executed);
    assert!(status.rollback_verified);
    assert_command_evidence(status);
    assert_non_claims(status);
}

fn assert_command_evidence(status: &NetworkLinuxNftablesLabStatus) {
    assert_eq!(status.command_evidence.len(), 6);
    assert_eq!(
        status.command_evidence[0].kind,
        NetworkLinuxNftablesLabCommandStatusKind::CreateTable
    );
    assert_eq!(
        status.command_evidence[0].command_ref,
        constants::network_flow::TEST_LINUX_NFTABLES_CREATE_TABLE_COMMAND_REF
    );
    assert_eq!(status.command_evidence[0].exit_status, 0);
    assert!(status.command_evidence[2].rule_present_after_command);
    assert!(!status.command_evidence[5].table_present_after_command);
}

fn assert_non_claims(status: &NetworkLinuxNftablesLabStatus) {
    assert!(!status.production_enforcement_claimed);
    assert!(!status.persistent_rule_claimed);
    assert!(!status.generic_linux_support_claimed);
    assert!(!status.service_manager_install_claimed);
    assert!(!status.exact_url_available);
    assert!(!status.decrypted_payload_available);
    assert!(!status.page_content_available);
    assert!(!status.policy_engine_execution_claimed);
    assert!(!status.enforcement_command_published);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::NETWORK_LINUX_NFTABLES_LAB_STATUS_REPORTED.to_string(),
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
        command: AgentCommandName::AgentNetworkLinuxNftablesLabStatusGet,
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
