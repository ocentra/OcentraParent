use ocentra_network_evidence::linux_adapter_gate::{
    NetworkLinuxAdapterCapabilityState, NetworkLinuxAdapterGateInput, NetworkLinuxAdapterKind,
};
use ocentra_network_evidence::linux_nftables_lab_execution::{
    prove_network_linux_nftables_lab_execution,
    types::{
        NetworkLinuxNftablesLabCommandEvidence, NetworkLinuxNftablesLabCommandKind,
        NetworkLinuxNftablesLabExecutionError, NetworkLinuxNftablesLabExecutionInput,
        NetworkLinuxNftablesLabUnsupportedClaims,
    },
};

use super::fixtures::{command, gate_input, gate_proof, lab_execution_input, unsupported_claims};

#[test]
fn linux_nftables_lab_requires_distro_ready_nftables_gate_proof() {
    let manual_gate =
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            gate_proof: gate_proof(NetworkLinuxAdapterGateInput {
                capability_state: NetworkLinuxAdapterCapabilityState::ManualRequired,
                ..gate_input(NetworkLinuxAdapterKind::Nftables)
            }),
            ..lab_execution_input()
        });

    assert_eq!(
        manual_gate,
        Err(NetworkLinuxNftablesLabExecutionError::GateProofNotDistroReady)
    );

    let ebpf_gate =
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            gate_proof: gate_proof(gate_input(NetworkLinuxAdapterKind::Ebpf)),
            ..lab_execution_input()
        });

    assert_eq!(
        ebpf_gate,
        Err(NetworkLinuxNftablesLabExecutionError::UnsupportedAdapterKind)
    );
}

#[test]
fn linux_nftables_lab_requires_safe_names_and_test_net_target() {
    assert_eq!(
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            table_name: "production_filter".to_owned(),
            ..lab_execution_input()
        }),
        Err(NetworkLinuxNftablesLabExecutionError::UnsafeTableName)
    );
    assert_eq!(
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            chain_name: "output".to_owned(),
            ..lab_execution_input()
        }),
        Err(NetworkLinuxNftablesLabExecutionError::UnsafeChainName)
    );
    assert_eq!(
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            target_remote_address: "8.8.8.8".to_owned(),
            ..lab_execution_input()
        }),
        Err(NetworkLinuxNftablesLabExecutionError::UnsafeTargetRemoteAddress)
    );
}

#[test]
fn linux_nftables_lab_rejects_failed_command_evidence() {
    assert_eq!(
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            command_evidence: vec![
                command(
                    NetworkLinuxNftablesLabCommandKind::CreateTable,
                    true,
                    false,
                    false
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::CreateChain,
                    true,
                    true,
                    false
                ),
                NetworkLinuxNftablesLabCommandEvidence {
                    exit_status: 1,
                    ..command(
                        NetworkLinuxNftablesLabCommandKind::AddRule,
                        true,
                        true,
                        true
                    )
                },
                command(
                    NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
                    true,
                    true,
                    true
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::DeleteTable,
                    false,
                    false,
                    false
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
                    false,
                    false,
                    false
                ),
            ],
            ..lab_execution_input()
        }),
        Err(
            NetworkLinuxNftablesLabExecutionError::CommandEvidenceFailure(
                NetworkLinuxNftablesLabCommandKind::AddRule
            )
        )
    );
}

#[test]
fn linux_nftables_lab_rejects_rule_verification_without_chain_observation() {
    assert_eq!(
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            command_evidence: vec![
                command(
                    NetworkLinuxNftablesLabCommandKind::CreateTable,
                    true,
                    false,
                    false
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::CreateChain,
                    true,
                    true,
                    false
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::AddRule,
                    true,
                    true,
                    true
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
                    true,
                    false,
                    true
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::DeleteTable,
                    false,
                    false,
                    false
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
                    false,
                    false,
                    false
                ),
            ],
            ..lab_execution_input()
        }),
        Err(NetworkLinuxNftablesLabExecutionError::RuleNotObserved)
    );
}

#[test]
fn linux_nftables_lab_rejects_rollback_that_leaves_table_present() {
    assert_eq!(
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            command_evidence: vec![
                command(
                    NetworkLinuxNftablesLabCommandKind::CreateTable,
                    true,
                    false,
                    false
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::CreateChain,
                    true,
                    true,
                    false
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::AddRule,
                    true,
                    true,
                    true
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
                    true,
                    true,
                    true
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::DeleteTable,
                    false,
                    false,
                    false
                ),
                command(
                    NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
                    true,
                    false,
                    false
                ),
            ],
            ..lab_execution_input()
        }),
        Err(NetworkLinuxNftablesLabExecutionError::RollbackTableStillPresent)
    );
}

#[test]
fn linux_nftables_lab_rejects_unsupported_claims() {
    assert_eq!(
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            unsupported_claims: NetworkLinuxNftablesLabUnsupportedClaims {
                production_enforcement_claimed: true,
                ..unsupported_claims()
            },
            ..lab_execution_input()
        }),
        Err(NetworkLinuxNftablesLabExecutionError::ProductionEnforcementClaimRejected)
    );
    assert_eq!(
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            unsupported_claims: NetworkLinuxNftablesLabUnsupportedClaims {
                generic_linux_support_claimed: true,
                ..unsupported_claims()
            },
            ..lab_execution_input()
        }),
        Err(NetworkLinuxNftablesLabExecutionError::GenericLinuxSupportClaimRejected)
    );
    assert_eq!(
        prove_network_linux_nftables_lab_execution(NetworkLinuxNftablesLabExecutionInput {
            unsupported_claims: NetworkLinuxNftablesLabUnsupportedClaims {
                enforcement_command_published: true,
                ..unsupported_claims()
            },
            ..lab_execution_input()
        }),
        Err(NetworkLinuxNftablesLabExecutionError::EnforcementCommandPublishedRejected)
    );
}
