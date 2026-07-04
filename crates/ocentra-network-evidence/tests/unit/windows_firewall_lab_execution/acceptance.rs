use ocentra_eventing::expect_value::ExpectValue;
use ocentra_network_evidence::windows_firewall_lab_execution::{
    prove_network_windows_firewall_lab_execution,
    types::{
        NetworkWindowsFirewallLabCommandKind, NetworkWindowsFirewallLabExecutionInput,
        NetworkWindowsFirewallLabExecutionState,
    },
};

use super::fixtures::{command, lab_execution_input};

#[test]
fn windows_firewall_lab_accepts_bounded_apply_verify_rollback_evidence() {
    let proof = prove_network_windows_firewall_lab_execution(lab_execution_input())
        .expect_value("complete lab evidence should prove bounded execution and rollback");

    assert_eq!(
        proof.state,
        NetworkWindowsFirewallLabExecutionState::ExecutedAndRolledBack
    );
    assert_eq!(proof.lab_ref, "windows-firewall-lab-execution-row38a");
    assert_eq!(proof.rule_name, "OcentraParentNetworkLab-row38a");
    assert_eq!(proof.target_remote_address, "203.0.113.254");
    assert_eq!(proof.command_count, 4);
    assert!(proof.apply_command_observed);
    assert!(proof.verify_present_observed);
    assert!(proof.rollback_command_observed);
    assert!(proof.verify_removed_observed);
    assert!(proof.lab_firewall_mutation_executed);
    assert!(proof.rollback_verified);
    assert!(!proof.production_enforcement_claimed);
    assert!(!proof.persistent_rule_claimed);
    assert!(!proof.exact_url_available);
    assert!(!proof.decrypted_payload_available);
    assert!(!proof.page_content_available);
    assert!(!proof.policy_engine_execution_claimed);
    assert!(!proof.enforcement_command_published);
}

#[test]
fn windows_firewall_lab_is_manual_required_without_admin_or_commands() {
    let no_admin =
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            administrator_permission_observed: false,
            command_evidence: Vec::new(),
            ..lab_execution_input()
        })
        .expect_value("missing admin should stay explicit instead of failing");

    assert_eq!(
        no_admin.state,
        NetworkWindowsFirewallLabExecutionState::ManualRequired
    );
    assert!(!no_admin.lab_firewall_mutation_executed);
    assert!(!no_admin.rollback_verified);

    let missing_commands =
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            command_evidence: vec![command(
                NetworkWindowsFirewallLabCommandKind::ApplyRule,
                true,
            )],
            ..lab_execution_input()
        })
        .expect_value("partial lab command evidence should stay manual-required");

    assert_eq!(
        missing_commands.state,
        NetworkWindowsFirewallLabExecutionState::ManualRequired
    );
    assert!(missing_commands.apply_command_observed);
    assert!(!missing_commands.verify_present_observed);
}

#[test]
fn windows_firewall_lab_reports_unavailable_without_windows_host() {
    let proof =
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            windows_host_observed: false,
            command_evidence: Vec::new(),
            ..lab_execution_input()
        })
        .expect_value("non-Windows host should report unavailable");

    assert_eq!(
        proof.state,
        NetworkWindowsFirewallLabExecutionState::Unavailable
    );
    assert!(!proof.lab_firewall_mutation_executed);
}
