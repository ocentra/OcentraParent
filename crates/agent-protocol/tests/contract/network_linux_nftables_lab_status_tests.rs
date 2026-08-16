use crate::{
    constants,
    network_linux_nftables_lab_status::{
        NetworkLinuxNftablesLabCommandStatusKind, NetworkLinuxNftablesLabCommandStatusRow,
        NetworkLinuxNftablesLabStatus, NetworkLinuxNftablesLabStatusState,
    },
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn linux_nftables_lab_status_serializes_to_camel_case_contract_shape() {
    let status = NetworkLinuxNftablesLabStatus {
        status_ref: constants::network_flow::TEST_LINUX_NFTABLES_LAB_STATUS_REF.to_string(),
        lab_ref: constants::network_flow::TEST_LINUX_NFTABLES_LAB_REF.to_string(),
        linux_adapter_gate_ref: constants::network_flow::TEST_LINUX_ADAPTER_GATE_REF.to_string(),
        policy_decision_ref: constants::network_flow::TEST_LINUX_ADAPTER_POLICY_DECISION_REF
            .to_string(),
        parent_rule_ref: constants::network_flow::TEST_LINUX_ADAPTER_PARENT_RULE_REF.to_string(),
        evidence_refs: vec![constants::network_flow::TEST_LINUX_ADAPTER_EVIDENCE_REF.to_string()],
        distro_ref: constants::network_flow::TEST_LINUX_DISTRO_REF.to_string(),
        kernel_ref: constants::network_flow::TEST_LINUX_KERNEL_REF.to_string(),
        table_name: constants::network_flow::TEST_LINUX_NFTABLES_TABLE_NAME.to_string(),
        chain_name: constants::network_flow::TEST_LINUX_NFTABLES_CHAIN_NAME.to_string(),
        target_remote_address: constants::network_flow::TEST_LINUX_NFTABLES_TARGET_REMOTE_ADDRESS
            .to_string(),
        state: NetworkLinuxNftablesLabStatusState::ExecutedAndRolledBack,
        wsl_host_observed: true,
        root_permission_observed: true,
        nft_tool_observed: true,
        command_count: 1,
        required_command_count: 6,
        table_create_observed: true,
        chain_create_observed: true,
        rule_add_observed: true,
        verify_present_observed: true,
        rollback_observed: true,
        verify_removed_observed: true,
        lab_packet_filter_rule_executed: true,
        rollback_verified: true,
        command_evidence: vec![command_row()],
        ..NetworkLinuxNftablesLabStatus::default()
    };

    let serialized = serde_json::to_value(status).expect_value("status serializes: {error}");

    assert_eq!(
        serialized["statusRef"],
        constants::network_flow::TEST_LINUX_NFTABLES_LAB_STATUS_REF
    );
    assert_eq!(serialized["state"], "executed-and-rolled-back");
    assert_eq!(serialized["commandEvidence"][0]["kind"], "create-table");
    assert_eq!(serialized["requiredCommandCount"], 6);
    assert_eq!(serialized["exactUrlAvailable"], false);
    assert_eq!(serialized["enforcementCommandPublished"], false);
}

fn command_row() -> NetworkLinuxNftablesLabCommandStatusRow {
    NetworkLinuxNftablesLabCommandStatusRow {
        kind: NetworkLinuxNftablesLabCommandStatusKind::CreateTable,
        command_ref: constants::network_flow::TEST_LINUX_NFTABLES_CREATE_TABLE_COMMAND_REF
            .to_string(),
        exit_status: 0,
        output_sha256: constants::network_flow::TEST_LINUX_NFTABLES_CREATE_TABLE_OUTPUT_SHA256
            .to_string(),
        table_present_after_command: true,
        chain_present_after_command: false,
        rule_present_after_command: false,
    }
}
