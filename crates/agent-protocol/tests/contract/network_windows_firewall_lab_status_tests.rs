use crate::{
    constants,
    network_windows_firewall_lab_status::{
        NetworkWindowsFirewallLabCommandStatusKind, NetworkWindowsFirewallLabCommandStatusRow,
        NetworkWindowsFirewallLabStatus, NetworkWindowsFirewallLabStatusState,
    },
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn windows_firewall_lab_status_serializes_to_camel_case_contract_shape() {
    let status = NetworkWindowsFirewallLabStatus {
        status_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_LAB_STATUS_REF.to_string(),
        lab_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_LAB_REF.to_string(),
        firewall_adapter_plan_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_ADAPTER_PLAN_REF
            .to_string(),
        policy_decision_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_POLICY_DECISION_REF
            .to_string(),
        parent_rule_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_PARENT_RULE_REF.to_string(),
        evidence_refs: vec![constants::network_flow::TEST_WINDOWS_FIREWALL_EVIDENCE_REF.to_string()],
        windows_os_scope_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_OS_SCOPE_REF
            .to_string(),
        target_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_TARGET_REF.to_string(),
        firewall_rule_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_RULE_REF.to_string(),
        rule_name: constants::network_flow::TEST_WINDOWS_FIREWALL_RULE_NAME.to_string(),
        target_remote_address: constants::network_flow::TEST_WINDOWS_FIREWALL_TARGET_REMOTE_ADDRESS
            .to_string(),
        state: NetworkWindowsFirewallLabStatusState::ExecutedAndRolledBack,
        windows_host_observed: true,
        administrator_permission_observed: true,
        command_count: 1,
        required_command_count: 4,
        apply_command_observed: true,
        verify_present_observed: true,
        rollback_command_observed: true,
        verify_removed_observed: true,
        lab_firewall_mutation_executed: true,
        rollback_verified: true,
        adapter_apply_authorized: true,
        command_evidence: vec![command_row()],
        ..NetworkWindowsFirewallLabStatus::default()
    };

    let serialized = serde_json::to_value(status).expect_value("status serializes: {error}");

    assert_eq!(
        serialized["statusRef"],
        constants::network_flow::TEST_WINDOWS_FIREWALL_LAB_STATUS_REF
    );
    assert_eq!(serialized["state"], "executed-and-rolled-back");
    assert_eq!(serialized["commandEvidence"][0]["kind"], "apply-rule");
    assert_eq!(serialized["requiredCommandCount"], 4);
    assert_eq!(serialized["exactUrlAvailable"], false);
    assert_eq!(serialized["enforcementCommandPublished"], false);
}

fn command_row() -> NetworkWindowsFirewallLabCommandStatusRow {
    NetworkWindowsFirewallLabCommandStatusRow {
        kind: NetworkWindowsFirewallLabCommandStatusKind::ApplyRule,
        command_ref: constants::network_flow::TEST_WINDOWS_FIREWALL_APPLY_RULE_COMMAND_REF
            .to_string(),
        exit_status: 0,
        output_sha256: constants::network_flow::TEST_WINDOWS_FIREWALL_APPLY_RULE_OUTPUT_SHA256
            .to_string(),
        rule_present_after_command: true,
    }
}
