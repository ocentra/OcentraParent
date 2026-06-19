use super::types::{
    NetworkWindowsFirewallLabCommandEvidence, NetworkWindowsFirewallLabCommandKind,
    NetworkWindowsFirewallLabExecutionError, NetworkWindowsFirewallLabExecutionInput,
    NetworkWindowsFirewallLabExecutionState, NetworkWindowsFirewallLabUnsupportedClaims,
};
use crate::{
    NetworkWindowsFirewallAdapterProof, NetworkWindowsFirewallProofState,
    NetworkWindowsFirewallTargetKind,
};

#[path = "../lab_execution_common.rs"]
mod lab_execution_common;

use lab_execution_common::{is_test_net_remote_address, normalize_ref};

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
    if adapter_proof.proof_state != NetworkWindowsFirewallProofState::ApplyReady
        || !adapter_proof.adapter_apply_authorized
    {
        return Err(NetworkWindowsFirewallLabExecutionError::AdapterProofNotApplyReady);
    }
    if adapter_proof.target_kind != NetworkWindowsFirewallTargetKind::RemoteAddress {
        return Err(NetworkWindowsFirewallLabExecutionError::UnsupportedTargetKind);
    }
    Ok(())
}

fn reject_unsupported_claims(
    claims: &NetworkWindowsFirewallLabUnsupportedClaims,
) -> Result<(), NetworkWindowsFirewallLabExecutionError> {
    if claims.production_enforcement_claimed {
        return Err(NetworkWindowsFirewallLabExecutionError::ProductionEnforcementClaimRejected);
    }
    if claims.persistent_rule_claimed {
        return Err(NetworkWindowsFirewallLabExecutionError::PersistentRuleClaimRejected);
    }
    if claims.exact_url_claimed {
        return Err(NetworkWindowsFirewallLabExecutionError::ExactUrlClaimRejected);
    }
    if claims.decrypted_payload_claimed {
        return Err(NetworkWindowsFirewallLabExecutionError::DecryptedPayloadClaimRejected);
    }
    if claims.page_content_claimed {
        return Err(NetworkWindowsFirewallLabExecutionError::PageContentClaimRejected);
    }
    if claims.policy_engine_execution_claimed {
        return Err(NetworkWindowsFirewallLabExecutionError::PolicyEngineExecutionClaimRejected);
    }
    if claims.enforcement_command_published {
        return Err(NetworkWindowsFirewallLabExecutionError::EnforcementCommandPublishedRejected);
    }
    Ok(())
}

fn normalize_command_evidence(
    evidence: Vec<NetworkWindowsFirewallLabCommandEvidence>,
) -> Result<Vec<NetworkWindowsFirewallLabCommandEvidence>, NetworkWindowsFirewallLabExecutionError>
{
    let mut normalized = Vec::new();
    for mut command in evidence {
        if has_kind(&normalized, command.kind) {
            return Err(
                NetworkWindowsFirewallLabExecutionError::DuplicateCommandEvidence(command.kind),
            );
        }
        command.command_ref = normalize_ref(&command.command_ref).ok_or(
            NetworkWindowsFirewallLabExecutionError::EmptyCommandRef(command.kind),
        )?;
        command.output_sha256 = normalize_ref(&command.output_sha256)
            .ok_or(NetworkWindowsFirewallLabExecutionError::EmptyCommandOutputHash(command.kind))?;
        normalized.push(command);
    }
    Ok(normalized)
}

fn validate_successful_execution(
    evidence: &[NetworkWindowsFirewallLabCommandEvidence],
) -> Result<(), NetworkWindowsFirewallLabExecutionError> {
    let apply = command(evidence, NetworkWindowsFirewallLabCommandKind::ApplyRule)?;
    let verify_present = command(
        evidence,
        NetworkWindowsFirewallLabCommandKind::VerifyRulePresent,
    )?;
    let rollback = command(evidence, NetworkWindowsFirewallLabCommandKind::RollbackRule)?;
    let verify_removed = command(
        evidence,
        NetworkWindowsFirewallLabCommandKind::VerifyRuleRemoved,
    )?;
    for command in [apply, verify_present, rollback, verify_removed] {
        if command.exit_status != 0 {
            return Err(
                NetworkWindowsFirewallLabExecutionError::CommandEvidenceFailure(command.kind),
            );
        }
    }
    if !verify_present.rule_present_after_command {
        return Err(NetworkWindowsFirewallLabExecutionError::ApplyRuleNotObserved);
    }
    if verify_removed.rule_present_after_command {
        return Err(NetworkWindowsFirewallLabExecutionError::RollbackRuleStillPresent);
    }
    Ok(())
}

fn command(
    evidence: &[NetworkWindowsFirewallLabCommandEvidence],
    kind: NetworkWindowsFirewallLabCommandKind,
) -> Result<&NetworkWindowsFirewallLabCommandEvidence, NetworkWindowsFirewallLabExecutionError> {
    evidence
        .iter()
        .find(|command| command.kind == kind)
        .ok_or(NetworkWindowsFirewallLabExecutionError::MissingCommandEvidence(kind))
}

fn has_kind(
    evidence: &[NetworkWindowsFirewallLabCommandEvidence],
    kind: NetworkWindowsFirewallLabCommandKind,
) -> bool {
    evidence.iter().any(|command| command.kind == kind)
}
