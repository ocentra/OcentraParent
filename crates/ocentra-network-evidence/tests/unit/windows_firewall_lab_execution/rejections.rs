use ocentra_network_evidence::{
    windows_firewall_adapter::{
        NetworkWindowsFirewallAdapterProofInput, NetworkWindowsFirewallCapabilityState,
        NetworkWindowsFirewallTargetKind,
    },
    windows_firewall_lab_execution::prove_network_windows_firewall_lab_execution,
    windows_firewall_lab_execution::types::{
        NetworkWindowsFirewallLabCommandEvidence, NetworkWindowsFirewallLabCommandKind,
        NetworkWindowsFirewallLabExecutionError, NetworkWindowsFirewallLabExecutionInput,
        NetworkWindowsFirewallLabUnsupportedClaims,
    },
};

use super::fixtures::{
    adapter_input, adapter_proof, command, lab_execution_input, unsupported_claims,
};

#[test]
fn windows_firewall_lab_requires_apply_ready_remote_address_adapter_proof() {
    let proof =
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            adapter_proof: adapter_proof(NetworkWindowsFirewallAdapterProofInput {
                capability_state: NetworkWindowsFirewallCapabilityState::ManualRequired,
                ..adapter_input(NetworkWindowsFirewallTargetKind::RemoteAddress)
            }),
            ..lab_execution_input()
        });

    assert_eq!(
        proof,
        Err(NetworkWindowsFirewallLabExecutionError::AdapterProofNotApplyReady)
    );

    let process_target =
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            adapter_proof: adapter_proof(adapter_input(NetworkWindowsFirewallTargetKind::App)),
            ..lab_execution_input()
        });

    assert_eq!(
        process_target,
        Err(NetworkWindowsFirewallLabExecutionError::UnsupportedTargetKind)
    );
}

#[test]
fn windows_firewall_lab_requires_safe_rule_name_and_test_net_target() {
    assert_eq!(
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            rule_name: "OcentraParentProductionBlock".to_owned(),
            ..lab_execution_input()
        }),
        Err(NetworkWindowsFirewallLabExecutionError::UnsafeRuleName)
    );
    assert_eq!(
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            target_remote_address: "8.8.8.8".to_owned(),
            ..lab_execution_input()
        }),
        Err(NetworkWindowsFirewallLabExecutionError::UnsafeTargetRemoteAddress)
    );
}

#[test]
fn windows_firewall_lab_rejects_failed_or_persistent_command_evidence() {
    assert_eq!(
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            command_evidence: vec![
                NetworkWindowsFirewallLabCommandEvidence {
                    exit_status: 1,
                    ..command(NetworkWindowsFirewallLabCommandKind::ApplyRule, true)
                },
                command(
                    NetworkWindowsFirewallLabCommandKind::VerifyRulePresent,
                    true
                ),
                command(NetworkWindowsFirewallLabCommandKind::RollbackRule, false),
                command(
                    NetworkWindowsFirewallLabCommandKind::VerifyRuleRemoved,
                    false
                ),
            ],
            ..lab_execution_input()
        }),
        Err(
            NetworkWindowsFirewallLabExecutionError::CommandEvidenceFailure(
                NetworkWindowsFirewallLabCommandKind::ApplyRule
            )
        )
    );
    assert_eq!(
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            command_evidence: vec![
                command(NetworkWindowsFirewallLabCommandKind::ApplyRule, true),
                command(
                    NetworkWindowsFirewallLabCommandKind::VerifyRulePresent,
                    false
                ),
                command(NetworkWindowsFirewallLabCommandKind::RollbackRule, false),
                command(
                    NetworkWindowsFirewallLabCommandKind::VerifyRuleRemoved,
                    false
                ),
            ],
            ..lab_execution_input()
        }),
        Err(NetworkWindowsFirewallLabExecutionError::ApplyRuleNotObserved)
    );
    assert_eq!(
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            command_evidence: vec![
                command(NetworkWindowsFirewallLabCommandKind::ApplyRule, true),
                command(
                    NetworkWindowsFirewallLabCommandKind::VerifyRulePresent,
                    true
                ),
                command(NetworkWindowsFirewallLabCommandKind::RollbackRule, false),
                command(
                    NetworkWindowsFirewallLabCommandKind::VerifyRuleRemoved,
                    true
                ),
            ],
            ..lab_execution_input()
        }),
        Err(NetworkWindowsFirewallLabExecutionError::RollbackRuleStillPresent)
    );
}

#[test]
fn windows_firewall_lab_rejects_unsupported_claims() {
    assert_eq!(
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            unsupported_claims: NetworkWindowsFirewallLabUnsupportedClaims {
                production_enforcement_claimed: true,
                ..unsupported_claims()
            },
            ..lab_execution_input()
        }),
        Err(NetworkWindowsFirewallLabExecutionError::ProductionEnforcementClaimRejected)
    );
    assert_eq!(
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            unsupported_claims: NetworkWindowsFirewallLabUnsupportedClaims {
                persistent_rule_claimed: true,
                ..unsupported_claims()
            },
            ..lab_execution_input()
        }),
        Err(NetworkWindowsFirewallLabExecutionError::PersistentRuleClaimRejected)
    );
    assert_eq!(
        prove_network_windows_firewall_lab_execution(NetworkWindowsFirewallLabExecutionInput {
            unsupported_claims: NetworkWindowsFirewallLabUnsupportedClaims {
                enforcement_command_published: true,
                ..unsupported_claims()
            },
            ..lab_execution_input()
        }),
        Err(NetworkWindowsFirewallLabExecutionError::EnforcementCommandPublishedRejected)
    );
}
