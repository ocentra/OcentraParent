use super::{
    has_kind, NetworkWindowsFirewallAdapterProof, NetworkWindowsFirewallLabCommandEvidence,
    NetworkWindowsFirewallLabExecutionError, NetworkWindowsFirewallLabUnsupportedClaims,
    NetworkWindowsFirewallProofState, NetworkWindowsFirewallTargetKind,
};
use crate::lab_execution_common::normalize_ref;

pub(super) fn validate_adapter_proof(
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

pub(super) fn reject_unsupported_claims(
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

pub(super) fn normalize_command_evidence(
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
