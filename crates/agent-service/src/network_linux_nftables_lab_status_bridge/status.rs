use ocentra_network_evidence::linux_nftables_lab_execution::types::{
    NetworkLinuxNftablesLabCommandEvidence, NetworkLinuxNftablesLabCommandKind,
    NetworkLinuxNftablesLabExecutionProof, NetworkLinuxNftablesLabExecutionState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_linux_nftables_lab_status::{
    NetworkLinuxNftablesLabCommandStatusKind, NetworkLinuxNftablesLabCommandStatusRow,
    NetworkLinuxNftablesLabStatus, NetworkLinuxNftablesLabStatusState,
};

use super::REQUIRED_COMMAND_COUNT;

pub(super) fn status_from_proof(
    proof: &NetworkLinuxNftablesLabExecutionProof,
) -> NetworkLinuxNftablesLabStatus {
    NetworkLinuxNftablesLabStatus {
        status_ref: constants::network_flow::TEST_LINUX_NFTABLES_LAB_STATUS_REF.to_string(),
        lab_ref: proof.lab_ref.clone(),
        linux_adapter_gate_ref: proof.linux_adapter_gate_ref.clone(),
        policy_decision_ref: proof.policy_decision_ref.clone(),
        parent_rule_ref: proof.parent_rule_ref.clone(),
        evidence_refs: proof.evidence_refs.clone(),
        distro_ref: proof.distro_ref.clone(),
        kernel_ref: proof.kernel_ref.clone(),
        table_name: proof.table_name.clone(),
        chain_name: proof.chain_name.clone(),
        target_remote_address: proof.target_remote_address.clone(),
        state: protocol_state(proof.state),
        wsl_host_observed: proof.wsl_host_observed,
        root_permission_observed: proof.root_permission_observed,
        nft_tool_observed: proof.nft_tool_observed,
        command_count: count(proof.command_count),
        required_command_count: REQUIRED_COMMAND_COUNT,
        table_create_observed: proof.table_create_observed,
        chain_create_observed: proof.chain_create_observed,
        rule_add_observed: proof.rule_add_observed,
        verify_present_observed: proof.verify_present_observed,
        rollback_observed: proof.rollback_observed,
        verify_removed_observed: proof.verify_removed_observed,
        lab_packet_filter_rule_executed: proof.lab_packet_filter_rule_executed,
        rollback_verified: proof.rollback_verified,
        production_enforcement_claimed: proof.production_enforcement_claimed,
        persistent_rule_claimed: proof.persistent_rule_claimed,
        generic_linux_support_claimed: proof.generic_linux_support_claimed,
        service_manager_install_claimed: proof.service_manager_install_claimed,
        exact_url_available: proof.exact_url_available,
        decrypted_payload_available: proof.decrypted_payload_available,
        page_content_available: proof.page_content_available,
        policy_engine_execution_claimed: proof.policy_engine_execution_claimed,
        enforcement_command_published: proof.enforcement_command_published,
        command_evidence: proof.command_evidence.iter().map(command_row).collect(),
    }
}

fn command_row(
    command: &NetworkLinuxNftablesLabCommandEvidence,
) -> NetworkLinuxNftablesLabCommandStatusRow {
    NetworkLinuxNftablesLabCommandStatusRow {
        kind: protocol_command_kind(command.kind),
        command_ref: command.command_ref.clone(),
        exit_status: command.exit_status,
        output_sha256: command.output_sha256.clone(),
        table_present_after_command: command.table_present_after_command,
        chain_present_after_command: command.chain_present_after_command,
        rule_present_after_command: command.rule_present_after_command,
    }
}

fn protocol_state(
    state: NetworkLinuxNftablesLabExecutionState,
) -> NetworkLinuxNftablesLabStatusState {
    match state {
        NetworkLinuxNftablesLabExecutionState::ExecutedAndRolledBack => {
            NetworkLinuxNftablesLabStatusState::ExecutedAndRolledBack
        }
        NetworkLinuxNftablesLabExecutionState::ManualRequired => {
            NetworkLinuxNftablesLabStatusState::ManualRequired
        }
        NetworkLinuxNftablesLabExecutionState::Unavailable => {
            NetworkLinuxNftablesLabStatusState::Unavailable
        }
    }
}

fn protocol_command_kind(
    kind: NetworkLinuxNftablesLabCommandKind,
) -> NetworkLinuxNftablesLabCommandStatusKind {
    match kind {
        NetworkLinuxNftablesLabCommandKind::CreateTable => {
            NetworkLinuxNftablesLabCommandStatusKind::CreateTable
        }
        NetworkLinuxNftablesLabCommandKind::CreateChain => {
            NetworkLinuxNftablesLabCommandStatusKind::CreateChain
        }
        NetworkLinuxNftablesLabCommandKind::AddRule => {
            NetworkLinuxNftablesLabCommandStatusKind::AddRule
        }
        NetworkLinuxNftablesLabCommandKind::VerifyRulePresent => {
            NetworkLinuxNftablesLabCommandStatusKind::VerifyRulePresent
        }
        NetworkLinuxNftablesLabCommandKind::DeleteTable => {
            NetworkLinuxNftablesLabCommandStatusKind::DeleteTable
        }
        NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved => {
            NetworkLinuxNftablesLabCommandStatusKind::VerifyTableRemoved
        }
    }
}

fn count(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}
