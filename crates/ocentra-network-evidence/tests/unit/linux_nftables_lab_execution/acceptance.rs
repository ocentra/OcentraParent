use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::linux_nftables_lab_execution::{
    prove_network_linux_nftables_lab_execution,
    types::{
        NetworkLinuxNftablesLabCommandKind, NetworkLinuxNftablesLabExecutionInput,
        NetworkLinuxNftablesLabExecutionState,
    },
};

use super::fixtures::{command, lab_execution_input};

#[test]
fn linux_nftables_lab_accepts_bounded_table_chain_rule_rollback_evidence() {
    let proof = prove_network_linux_nftables_lab_execution(lab_execution_input())
        .expect_value("complete nftables lab evidence should prove bounded execution and rollback");

    assert_eq!(
        proof.state,
        NetworkLinuxNftablesLabExecutionState::ExecutedAndRolledBack
    );
    assert_eq!(proof.lab_ref, "linux-nftables-lab-execution-row42a");
    assert_eq!(proof.table_name, "ocentra_parent_lab_row42a");
    assert_eq!(proof.chain_name, "ocentra_parent_lab_chain_row42a");
    assert_eq!(proof.target_remote_address, "203.0.113.253");
    assert!(proof.wsl_host_observed);
    assert!(proof.root_permission_observed);
    assert!(proof.nft_tool_observed);
    assert_eq!(proof.command_count, 6);
    assert!(proof.table_create_observed);
    assert!(proof.chain_create_observed);
    assert!(proof.rule_add_observed);
    assert!(proof.verify_present_observed);
    assert!(proof.rollback_observed);
    assert!(proof.verify_removed_observed);
    assert!(proof.lab_packet_filter_rule_executed);
    assert!(proof.rollback_verified);
    assert!(!proof.production_enforcement_claimed);
    assert!(!proof.persistent_rule_claimed);
    assert!(!proof.generic_linux_support_claimed);
    assert!(!proof.service_manager_install_claimed);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.page_content_available);
    assert!(!proof.policy_engine_execution_claimed);
    assert!(!proof.enforcement_command_published);
}

#[test]
fn linux_nftables_lab_is_manual_required_without_root_or_commands() {
    let no_root =
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            root_permission_observed: false,
            command_evidence: Vec::new(),
            ..lab_execution_input()
        })
        .expect_value("missing root should stay explicit instead of failing");

    assert_eq!(
        no_root.state,
        NetworkLinuxNftablesLabExecutionState::ManualRequired
    );
    assert!(no_root.wsl_host_observed);
    assert!(!no_root.root_permission_observed);
    assert!(no_root.nft_tool_observed);
    assert!(!no_root.lab_packet_filter_rule_executed);
    assert!(!no_root.rollback_verified);

    let missing_commands =
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            command_evidence: vec![command(
                NetworkLinuxNftablesLabCommandKind::CreateTable,
                true,
                false,
                false,
            )],
            ..lab_execution_input()
        })
        .expect_value("partial lab command evidence should stay manual-required");

    assert_eq!(
        missing_commands.state,
        NetworkLinuxNftablesLabExecutionState::ManualRequired
    );
    assert!(missing_commands.table_create_observed);
    assert!(!missing_commands.verify_present_observed);
}

#[test]
fn linux_nftables_lab_reports_unavailable_without_wsl_or_nft_tool() {
    let no_wsl =
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            wsl_host_observed: false,
            command_evidence: Vec::new(),
            ..lab_execution_input()
        })
        .expect_value("missing WSL host should report unavailable");

    assert_eq!(
        no_wsl.state,
        NetworkLinuxNftablesLabExecutionState::Unavailable
    );
    assert!(!no_wsl.wsl_host_observed);
    assert!(no_wsl.root_permission_observed);
    assert!(no_wsl.nft_tool_observed);

    let no_nft =
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            nft_tool_observed: false,
            command_evidence: Vec::new(),
            ..lab_execution_input()
        })
        .expect_value("missing nft binary should report unavailable");

    assert_eq!(
        no_nft.state,
        NetworkLinuxNftablesLabExecutionState::Unavailable
    );
    assert!(no_nft.wsl_host_observed);
    assert!(no_nft.root_permission_observed);
    assert!(!no_nft.nft_tool_observed);
    assert!(!no_nft.lab_packet_filter_rule_executed);
}
