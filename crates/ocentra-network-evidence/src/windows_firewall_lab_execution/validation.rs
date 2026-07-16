mod execution;
mod normalization;

use super::types::{
    NetworkWindowsFirewallLabCommandEvidence, NetworkWindowsFirewallLabCommandKind,
    NetworkWindowsFirewallLabExecutionError, NetworkWindowsFirewallLabExecutionInput,
    NetworkWindowsFirewallLabExecutionState, NetworkWindowsFirewallLabUnsupportedClaims,
};
use crate::lab_execution_common::{is_test_net_remote_address, normalize_ref};
use crate::{
    NetworkWindowsFirewallAdapterProof, NetworkWindowsFirewallProofState,
    NetworkWindowsFirewallTargetKind,
};

pub struct NormalizedLabExecutionInput {
    pub lab_ref: String,
    pub adapter_proof: NetworkWindowsFirewallAdapterProof,
    pub rule_name: String,
    pub target_remote_address: String,
    pub windows_host_observed: bool,
    pub administrator_permission_observed: bool,
    pub command_evidence: Vec<NetworkWindowsFirewallLabCommandEvidence>,
}

#[derive(Clone, Copy)]
pub struct CommandEvidenceFlags {
    pub apply: bool,
    pub verify_present: bool,
    pub rollback: bool,
    pub verify_removed: bool,
}

pub fn normalize_input(
    input: NetworkWindowsFirewallLabExecutionInput,
) -> Result<NormalizedLabExecutionInput, NetworkWindowsFirewallLabExecutionError> {
    reject_unsupported_claims(&input.unsupported_claims)?;
    validate_adapter_proof(&input.adapter_proof)?;
    let lab_ref = normalize_ref(&input.lab_ref)
        .ok_or(NetworkWindowsFirewallLabExecutionError::EmptyLabRef)?;
    let rule_name = normalize_ref(&input.rule_name)
        .ok_or(NetworkWindowsFirewallLabExecutionError::EmptyRuleName)?;
    if !rule_name.starts_with("OcentraParentNetworkLab-") {
        return Err(NetworkWindowsFirewallLabExecutionError::UnsafeRuleName);
    }
    let target_remote_address = normalize_ref(&input.target_remote_address)
        .ok_or(NetworkWindowsFirewallLabExecutionError::EmptyTargetRemoteAddress)?;
    if !is_test_net_remote_address(&target_remote_address) {
        return Err(NetworkWindowsFirewallLabExecutionError::UnsafeTargetRemoteAddress);
    }

    Ok(NormalizedLabExecutionInput {
        lab_ref,
        adapter_proof: input.adapter_proof,
        rule_name,
        target_remote_address,
        windows_host_observed: input.windows_host_observed,
        administrator_permission_observed: input.administrator_permission_observed,
        command_evidence: normalize_command_evidence(input.command_evidence)?,
    })
}

pub fn command_flags(
    evidence: &[NetworkWindowsFirewallLabCommandEvidence],
) -> CommandEvidenceFlags {
    CommandEvidenceFlags {
        apply: has_kind(evidence, NetworkWindowsFirewallLabCommandKind::ApplyRule),
        verify_present: has_kind(
            evidence,
            NetworkWindowsFirewallLabCommandKind::VerifyRulePresent,
        ),
        rollback: has_kind(evidence, NetworkWindowsFirewallLabCommandKind::RollbackRule),
        verify_removed: has_kind(
            evidence,
            NetworkWindowsFirewallLabCommandKind::VerifyRuleRemoved,
        ),
    }
}

pub fn execution_state(
    windows_host_observed: bool,
    administrator_permission_observed: bool,
    evidence: &[NetworkWindowsFirewallLabCommandEvidence],
    flags: CommandEvidenceFlags,
) -> Result<NetworkWindowsFirewallLabExecutionState, NetworkWindowsFirewallLabExecutionError> {
    if !windows_host_observed {
        return Ok(NetworkWindowsFirewallLabExecutionState::Unavailable);
    }
    if !administrator_permission_observed
        || evidence.is_empty()
        || !flags.apply
        || !flags.verify_present
        || !flags.rollback
        || !flags.verify_removed
    {
        return Ok(NetworkWindowsFirewallLabExecutionState::ManualRequired);
    }
    validate_successful_execution(evidence)?;
    Ok(NetworkWindowsFirewallLabExecutionState::ExecutedAndRolledBack)
}

fn validate_adapter_proof(
    adapter_proof: &NetworkWindowsFirewallAdapterProof,
) -> Result<(), NetworkWindowsFirewallLabExecutionError> {
    normalization::validate_adapter_proof(adapter_proof)
}

fn reject_unsupported_claims(
    claims: &NetworkWindowsFirewallLabUnsupportedClaims,
) -> Result<(), NetworkWindowsFirewallLabExecutionError> {
    normalization::reject_unsupported_claims(claims)
}

fn normalize_command_evidence(
    evidence: Vec<NetworkWindowsFirewallLabCommandEvidence>,
) -> Result<Vec<NetworkWindowsFirewallLabCommandEvidence>, NetworkWindowsFirewallLabExecutionError>
{
    normalization::normalize_command_evidence(evidence)
}

fn validate_successful_execution(
    evidence: &[NetworkWindowsFirewallLabCommandEvidence],
) -> Result<(), NetworkWindowsFirewallLabExecutionError> {
    execution::validate_successful_execution(evidence)
}

fn has_kind(
    evidence: &[NetworkWindowsFirewallLabCommandEvidence],
    kind: NetworkWindowsFirewallLabCommandKind,
) -> bool {
    evidence.iter().any(|command| command.kind == kind)
}
