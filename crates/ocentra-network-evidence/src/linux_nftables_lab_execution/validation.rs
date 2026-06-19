use super::types::{
    NetworkLinuxNftablesLabCommandEvidence, NetworkLinuxNftablesLabCommandKind,
    NetworkLinuxNftablesLabExecutionError, NetworkLinuxNftablesLabExecutionInput,
    NetworkLinuxNftablesLabExecutionState, NetworkLinuxNftablesLabUnsupportedClaims,
};
use crate::linux_adapter_gate::{
    NetworkLinuxAdapterGateProof, NetworkLinuxAdapterGateState, NetworkLinuxAdapterKind,
};

#[path = "../lab_execution_common.rs"]
mod lab_execution_common;

use lab_execution_common::{is_test_net_remote_address, normalize_ref};

pub struct NormalizedLabExecutionInput {
    pub lab_ref: String,
    pub gate_proof: NetworkLinuxAdapterGateProof,
    pub table_name: String,
    pub chain_name: String,
    pub target_remote_address: String,
    pub wsl_host_observed: bool,
    pub root_permission_observed: bool,
    pub nft_tool_observed: bool,
    pub command_evidence: Vec<NetworkLinuxNftablesLabCommandEvidence>,
}

#[derive(Clone, Copy)]
pub struct CommandEvidenceFlags {
    pub create_table: bool,
    pub create_chain: bool,
    pub add_rule: bool,
    pub verify_present: bool,
    pub delete_table: bool,
    pub verify_removed: bool,
}

pub fn normalize_input(
    input: NetworkLinuxNftablesLabExecutionInput,
) -> Result<NormalizedLabExecutionInput, NetworkLinuxNftablesLabExecutionError> {
    reject_unsupported_claims(&input.unsupported_claims)?;
    validate_gate_proof(&input.gate_proof)?;
    let lab_ref =
        normalize_ref(&input.lab_ref).ok_or(NetworkLinuxNftablesLabExecutionError::EmptyLabRef)?;
    let table_name = normalize_ref(&input.table_name)
        .ok_or(NetworkLinuxNftablesLabExecutionError::EmptyTableName)?;
    if !table_name.starts_with("ocentra_parent_lab_") {
        return Err(NetworkLinuxNftablesLabExecutionError::UnsafeTableName);
    }
    let chain_name = normalize_ref(&input.chain_name)
        .ok_or(NetworkLinuxNftablesLabExecutionError::EmptyChainName)?;
    if !chain_name.starts_with("ocentra_parent_lab_") {
        return Err(NetworkLinuxNftablesLabExecutionError::UnsafeChainName);
    }
    let target_remote_address = normalize_ref(&input.target_remote_address)
        .ok_or(NetworkLinuxNftablesLabExecutionError::EmptyTargetRemoteAddress)?;
    if !is_test_net_remote_address(&target_remote_address) {
        return Err(NetworkLinuxNftablesLabExecutionError::UnsafeTargetRemoteAddress);
    }

    Ok(NormalizedLabExecutionInput {
        lab_ref,
        gate_proof: input.gate_proof,
        table_name,
        chain_name,
        target_remote_address,
        wsl_host_observed: input.wsl_host_observed,
        root_permission_observed: input.root_permission_observed,
        nft_tool_observed: input.nft_tool_observed,
        command_evidence: normalize_command_evidence(input.command_evidence)?,
    })
}

pub fn command_flags(evidence: &[NetworkLinuxNftablesLabCommandEvidence]) -> CommandEvidenceFlags {
    CommandEvidenceFlags {
        create_table: has_kind(evidence, NetworkLinuxNftablesLabCommandKind::CreateTable),
        create_chain: has_kind(evidence, NetworkLinuxNftablesLabCommandKind::CreateChain),
        add_rule: has_kind(evidence, NetworkLinuxNftablesLabCommandKind::AddRule),
        verify_present: has_kind(
            evidence,
            NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
        ),
        delete_table: has_kind(evidence, NetworkLinuxNftablesLabCommandKind::DeleteTable),
        verify_removed: has_kind(
            evidence,
            NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
        ),
    }
}

pub fn execution_state(
    wsl_host_observed: bool,
    root_permission_observed: bool,
    nft_tool_observed: bool,
    evidence: &[NetworkLinuxNftablesLabCommandEvidence],
    flags: CommandEvidenceFlags,
) -> Result<NetworkLinuxNftablesLabExecutionState, NetworkLinuxNftablesLabExecutionError> {
    if !wsl_host_observed || !nft_tool_observed {
        return Ok(NetworkLinuxNftablesLabExecutionState::Unavailable);
    }
    if !root_permission_observed
        || evidence.is_empty()
        || !flags.create_table
        || !flags.create_chain
        || !flags.add_rule
        || !flags.verify_present
        || !flags.delete_table
        || !flags.verify_removed
    {
        return Ok(NetworkLinuxNftablesLabExecutionState::ManualRequired);
    }
    validate_successful_execution(evidence)?;
    Ok(NetworkLinuxNftablesLabExecutionState::ExecutedAndRolledBack)
}

fn validate_gate_proof(
    gate_proof: &NetworkLinuxAdapterGateProof,
) -> Result<(), NetworkLinuxNftablesLabExecutionError> {
    if gate_proof.gate_state != NetworkLinuxAdapterGateState::DistroProofReady
        || !gate_proof.distro_proof_ready
    {
        return Err(NetworkLinuxNftablesLabExecutionError::GateProofNotDistroReady);
    }
    if gate_proof.adapter_kind != NetworkLinuxAdapterKind::Nftables {
        return Err(NetworkLinuxNftablesLabExecutionError::UnsupportedAdapterKind);
    }
    Ok(())
}

fn reject_unsupported_claims(
    claims: &NetworkLinuxNftablesLabUnsupportedClaims,
) -> Result<(), NetworkLinuxNftablesLabExecutionError> {
    if claims.production_enforcement_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::ProductionEnforcementClaimRejected);
    }
    if claims.persistent_rule_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::PersistentRuleClaimRejected);
    }
    if claims.generic_linux_support_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::GenericLinuxSupportClaimRejected);
    }
    if claims.service_manager_install_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::ServiceManagerInstallClaimRejected);
    }
    if claims.exact_url_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::ExactUrlClaimRejected);
    }
    if claims.decrypted_payload_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::DecryptedPayloadClaimRejected);
    }
    if claims.page_content_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::PageContentClaimRejected);
    }
    if claims.policy_engine_execution_claimed {
        return Err(NetworkLinuxNftablesLabExecutionError::PolicyEngineExecutionClaimRejected);
    }
    if claims.enforcement_command_published {
        return Err(NetworkLinuxNftablesLabExecutionError::EnforcementCommandPublishedRejected);
    }
    Ok(())
}

fn normalize_command_evidence(
    evidence: Vec<NetworkLinuxNftablesLabCommandEvidence>,
) -> Result<Vec<NetworkLinuxNftablesLabCommandEvidence>, NetworkLinuxNftablesLabExecutionError> {
    let mut normalized = Vec::new();
    for mut command in evidence {
        if has_kind(&normalized, command.kind) {
            return Err(
                NetworkLinuxNftablesLabExecutionError::DuplicateCommandEvidence(command.kind),
            );
        }
        command.command_ref = normalize_ref(&command.command_ref).ok_or(
            NetworkLinuxNftablesLabExecutionError::EmptyCommandRef(command.kind),
        )?;
        command.output_sha256 = normalize_ref(&command.output_sha256)
            .ok_or(NetworkLinuxNftablesLabExecutionError::EmptyCommandOutputHash(command.kind))?;
        normalized.push(command);
    }
    Ok(normalized)
}

fn validate_successful_execution(
    evidence: &[NetworkLinuxNftablesLabCommandEvidence],
) -> Result<(), NetworkLinuxNftablesLabExecutionError> {
    let verify_present = command(
        evidence,
        NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
    )?;
    let verify_removed = command(
        evidence,
        NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
    )?;
    for kind in required_command_kinds() {
        let command = command(evidence, kind)?;
        if command.exit_status != 0 {
            return Err(NetworkLinuxNftablesLabExecutionError::CommandEvidenceFailure(kind));
        }
    }
    if !verify_present.table_present_after_command
        || !verify_present.chain_present_after_command
        || !verify_present.rule_present_after_command
    {
        return Err(NetworkLinuxNftablesLabExecutionError::RuleNotObserved);
    }
    if verify_removed.table_present_after_command {
        return Err(NetworkLinuxNftablesLabExecutionError::RollbackTableStillPresent);
    }
    Ok(())
}

fn required_command_kinds() -> [NetworkLinuxNftablesLabCommandKind; 6] {
    [
        NetworkLinuxNftablesLabCommandKind::CreateTable,
        NetworkLinuxNftablesLabCommandKind::CreateChain,
        NetworkLinuxNftablesLabCommandKind::AddRule,
        NetworkLinuxNftablesLabCommandKind::VerifyRulePresent,
        NetworkLinuxNftablesLabCommandKind::DeleteTable,
        NetworkLinuxNftablesLabCommandKind::VerifyTableRemoved,
    ]
}

fn command(
    evidence: &[NetworkLinuxNftablesLabCommandEvidence],
    kind: NetworkLinuxNftablesLabCommandKind,
) -> Result<&NetworkLinuxNftablesLabCommandEvidence, NetworkLinuxNftablesLabExecutionError> {
    evidence
        .iter()
        .find(|command| command.kind == kind)
        .ok_or(NetworkLinuxNftablesLabExecutionError::MissingCommandEvidence(kind))
}

fn has_kind(
    evidence: &[NetworkLinuxNftablesLabCommandEvidence],
    kind: NetworkLinuxNftablesLabCommandKind,
) -> bool {
    evidence.iter().any(|command| command.kind == kind)
}
