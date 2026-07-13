use std::primitive::str as TestStr;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_windows_firewall_lab_status::{
    NetworkWindowsFirewallLabCommandStatusKind, NetworkWindowsFirewallLabStatus,
    NetworkWindowsFirewallLabStatusState,
};
use ocentra_parent_agent_protocol::policy_constants;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventName, AgentMessageTarget, AgentPeer,
    AgentPeerRole, AgentRoute,
};
use ocentra_parent_agent_protocol::AGENT_PROTOCOL_SCHEMA_VERSION;
use ocentra_parent_agent_service::test_support::{
    handle_local_command_text_for_test, network_windows_firewall_lab_status_payload_for_test,
};
use serde::de::DeserializeOwned;

#[test]
fn network_windows_firewall_lab_status_payload_reports_bounded_lab_execution_only(
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = network_windows_firewall_lab_status_payload_for_test()?;
    let status: NetworkWindowsFirewallLabStatus = status_value(
        &payload,
        constants::network_flow::FIELD_NETWORK_WINDOWS_FIREWALL_LAB_STATUS,
    )?;

    assert_windows_firewall_status(&status);
    Ok(())
}

#[tokio::test]
async fn websocket_network_windows_firewall_lab_status_command_reports_payload(
) -> Result<(), Box<dyn std::error::Error>> {
    let body = serde_json::to_string(&command_envelope())?;
    let event =
        handle_local_command_text_for_test(crate::test_text::TestText::from_display(body)).await;
    let status: NetworkWindowsFirewallLabStatus = status_value(
        &event.payload,
        constants::network_flow::FIELD_NETWORK_WINDOWS_FIREWALL_LAB_STATUS,
    )?;

    assert_eq!(
        event.event,
        AgentEventName::AgentNetworkWindowsFirewallLabStatusReported
    );
    assert_windows_firewall_status(&status);
    Ok(())
}

fn assert_windows_firewall_status(status: &NetworkWindowsFirewallLabStatus) {
    assert_eq!(
        status.status_ref,
        constants::network_flow::TEST_WINDOWS_FIREWALL_LAB_STATUS_REF
    );
    assert_eq!(
        status.lab_ref,
        constants::network_flow::TEST_WINDOWS_FIREWALL_LAB_REF
    );
    assert_eq!(
        status.firewall_adapter_plan_ref,
        constants::network_flow::TEST_WINDOWS_FIREWALL_ADAPTER_PLAN_REF
    );
    assert_eq!(
        status.state,
        NetworkWindowsFirewallLabStatusState::ExecutedAndRolledBack
    );
    assert!(status.windows_host_observed);
    assert!(status.administrator_permission_observed);
    assert_eq!(status.command_count, 4);
    assert_eq!(status.required_command_count, 4);
    assert!(status.apply_command_observed);
    assert!(status.verify_present_observed);
    assert!(status.rollback_command_observed);
    assert!(status.verify_removed_observed);
    assert!(status.lab_firewall_mutation_executed);
    assert!(status.rollback_verified);
    assert!(status.adapter_apply_authorized);
    assert_command_evidence(status);
    assert_non_claims(status);
}

fn assert_command_evidence(status: &NetworkWindowsFirewallLabStatus) {
    assert_eq!(status.command_evidence.len(), 4);
    assert_eq!(
        status.command_evidence[0].kind,
        NetworkWindowsFirewallLabCommandStatusKind::ApplyRule
    );
    assert_eq!(
        status.command_evidence[0].command_ref,
        constants::network_flow::TEST_WINDOWS_FIREWALL_APPLY_RULE_COMMAND_REF
    );
    assert_eq!(status.command_evidence[0].exit_status, 0);
    assert!(status.command_evidence[1].rule_present_after_command);
    assert!(!status.command_evidence[3].rule_present_after_command);
}

fn assert_non_claims(status: &NetworkWindowsFirewallLabStatus) {
    assert!(!status.production_enforcement_claimed);
    assert!(!status.persistent_rule_claimed);
    assert!(!status.exact_url_available);
    assert!(!status.decrypted_payload_available);
    assert!(!status.page_content_available);
    assert!(!status.host_firewall_mutation_claimed);
    assert!(!status.netsh_command_invoked);
    assert!(!status.powershell_command_invoked);
    assert!(!status.policy_engine_execution_claimed);
    assert!(!status.enforcement_command_published);
}

fn command_envelope() -> AgentCommandEnvelope {
    AgentCommandEnvelope {
        schema_version: AGENT_PROTOCOL_SCHEMA_VERSION,
        message_id: constants::event_id::NETWORK_WINDOWS_FIREWALL_LAB_STATUS_REPORTED.to_string(),
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
        command: AgentCommandName::AgentNetworkWindowsFirewallLabStatusGet,
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
