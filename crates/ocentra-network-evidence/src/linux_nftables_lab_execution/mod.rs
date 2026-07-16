pub mod types;
mod validation;

use types::{
    NetworkLinuxNftablesLabExecutionError, NetworkLinuxNftablesLabExecutionInput,
    NetworkLinuxNftablesLabExecutionProof, NetworkLinuxNftablesLabExecutionState,
};

use validation::{command_flags, execution_state, normalize_input};

pub fn prove_network_linux_nftables_lab_execution(
    input: NetworkLinuxNftablesLabExecutionInput,
) -> Result<NetworkLinuxNftablesLabExecutionProof, NetworkLinuxNftablesLabExecutionError> {
    let normalized = normalize_input(input)?;
    let flags = command_flags(&normalized.command_evidence);
    let state = execution_state(
        normalized.wsl_host_observed,
        normalized.root_permission_observed,
        normalized.nft_tool_observed,
        &normalized.command_evidence,
        flags,
    )?;

    Ok(NetworkLinuxNftablesLabExecutionProof {
        lab_ref: normalized.lab_ref,
        linux_adapter_gate_ref: normalized.gate_proof.linux_adapter_gate_ref,
        policy_decision_ref: normalized.gate_proof.policy_decision_ref,
        parent_rule_ref: normalized.gate_proof.parent_rule_ref,
        evidence_refs: normalized.gate_proof.evidence_refs,
        distro_ref: normalized.gate_proof.distro_ref,
        kernel_ref: normalized.gate_proof.kernel_ref,
        table_name: normalized.table_name,
        chain_name: normalized.chain_name,
        target_remote_address: normalized.target_remote_address,
        state,
        wsl_host_observed: normalized.wsl_host_observed,
        root_permission_observed: normalized.root_permission_observed,
        nft_tool_observed: normalized.nft_tool_observed,
        command_count: normalized.command_evidence.len(),
        table_create_observed: flags.create_table,
        chain_create_observed: flags.create_chain,
        rule_add_observed: flags.add_rule,
        verify_present_observed: flags.verify_present,
        rollback_observed: flags.delete_table,
        verify_removed_observed: flags.verify_removed,
        lab_packet_filter_rule_executed: state
            == NetworkLinuxNftablesLabExecutionState::ExecutedAndRolledBack,
        rollback_verified: state == NetworkLinuxNftablesLabExecutionState::ExecutedAndRolledBack,
        command_evidence: normalized.command_evidence,
        production_enforcement_claimed: false,
        persistent_rule_claimed: false,
        generic_linux_support_claimed: false,
        service_manager_install_claimed: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        policy_engine_execution_claimed: false,
        enforcement_command_published: false,
    })
}
