use std::primitive::str as TestStr;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_linux_nftables_lab_status::{
    NetworkLinuxNftablesLabCommandStatusKind, NetworkLinuxNftablesLabStatus,
    NetworkLinuxNftablesLabStatusState,
};
use ocentra_parent_agent_service::test_support::network_linux_nftables_lab_status_payload_for_test;
use serde::de::DeserializeOwned;

#[test]
fn network_linux_nftables_lab_status_payload_reports_bounded_lab_execution_only(
) -> Result<(), Box<dyn std::error::Error>> {
    let payload = network_linux_nftables_lab_status_payload_for_test()?;
    let status: NetworkLinuxNftablesLabStatus = status_value(
        &payload,
        constants::network_flow::FIELD_NETWORK_LINUX_NFTABLES_LAB_STATUS,
    )?;

    assert_linux_nftables_status(&status);
    Ok(())
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
