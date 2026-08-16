pub mod types;
mod validation;

use types::{
    NetworkWindowsFirewallLabExecutionError, NetworkWindowsFirewallLabExecutionInput,
    NetworkWindowsFirewallLabExecutionProof, NetworkWindowsFirewallLabExecutionState,
};

use validation::{command_flags, execution_state, normalize_input};

pub fn prove_network_windows_firewall_lab_execution(
    input: NetworkWindowsFirewallLabExecutionInput,
) -> Result<NetworkWindowsFirewallLabExecutionProof, NetworkWindowsFirewallLabExecutionError> {
    let normalized = normalize_input(input)?;
    let flags = command_flags(&normalized.command_evidence);
    let state = execution_state(
        normalized.windows_host_observed,
        normalized.administrator_permission_observed,
        &normalized.command_evidence,
        flags,
    )?;

    Ok(NetworkWindowsFirewallLabExecutionProof {
        lab_ref: normalized.lab_ref,
        adapter_plan_ref: normalized.adapter_proof.firewall_adapter_plan_ref,
        policy_decision_ref: normalized.adapter_proof.policy_decision_ref,
        parent_rule_ref: normalized.adapter_proof.parent_rule_ref,
        evidence_refs: normalized.adapter_proof.evidence_refs,
        rule_name: normalized.rule_name,
        target_remote_address: normalized.target_remote_address,
        command_count: normalized.command_evidence.len(),
        apply_command_observed: flags.apply,
        verify_present_observed: flags.verify_present,
        rollback_command_observed: flags.rollback,
        verify_removed_observed: flags.verify_removed,
        lab_firewall_mutation_executed: state
            == NetworkWindowsFirewallLabExecutionState::ExecutedAndRolledBack,
        rollback_verified: state == NetworkWindowsFirewallLabExecutionState::ExecutedAndRolledBack,
        state,
        command_evidence: normalized.command_evidence,
        production_enforcement_claimed: false,
        persistent_rule_claimed: false,
        exact_url_available: false,
        decrypted_payload_available: false,
        page_content_available: false,
        policy_engine_execution_claimed: false,
        enforcement_command_published: false,
    })
}
