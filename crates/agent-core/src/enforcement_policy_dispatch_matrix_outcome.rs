use ocentra_parent_agent_protocol::enforcement::EnforcementCapabilityState;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::{
    EnforcementPolicyDispatchApprovalState, EnforcementPolicyDispatchOutcomeState,
    EnforcementPolicyDispatchProofLevel, EnforcementPolicyDispatchReadModelEntry,
    EnforcementPolicyDispatchRejectionReason,
};

pub(super) fn validate_outcome(
    entry: &EnforcementPolicyDispatchReadModelEntry,
) -> Result<(), EnforcementPolicyDispatchRejectionReason> {
    match entry.matrix_row.outcome_state {
        EnforcementPolicyDispatchOutcomeState::DispatchReady => validate_dispatch_ready(entry),
        EnforcementPolicyDispatchOutcomeState::ManualRequired => validate_manual_required(entry),
        EnforcementPolicyDispatchOutcomeState::Rejected => validate_rejected(entry),
        EnforcementPolicyDispatchOutcomeState::ReportOnly
        | EnforcementPolicyDispatchOutcomeState::DryRunOnly
        | EnforcementPolicyDispatchOutcomeState::Degraded
        | EnforcementPolicyDispatchOutcomeState::Unavailable => Ok(()),
    }
}

fn validate_dispatch_ready(
    entry: &EnforcementPolicyDispatchReadModelEntry,
) -> Result<(), EnforcementPolicyDispatchRejectionReason> {
    if entry.intent.dry_run
        || !matches!(
            entry.approval_state,
            EnforcementPolicyDispatchApprovalState::NotRequired
                | EnforcementPolicyDispatchApprovalState::Approved
                | EnforcementPolicyDispatchApprovalState::OverrideActive
        )
    {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }
    if entry.matrix_row.capability_state != EnforcementCapabilityState::Supported {
        return Err(EnforcementPolicyDispatchRejectionReason::AdapterUnavailable);
    }
    if entry.matrix_row.proof_level != EnforcementPolicyDispatchProofLevel::Implemented {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }
    if entry.matrix_row.rejection_reason != EnforcementPolicyDispatchRejectionReason::None {
        return Err(entry.matrix_row.rejection_reason);
    }
    Ok(())
}

fn validate_manual_required(
    entry: &EnforcementPolicyDispatchReadModelEntry,
) -> Result<(), EnforcementPolicyDispatchRejectionReason> {
    if entry.matrix_row.capability_state != EnforcementCapabilityState::ManualRequired {
        return Err(EnforcementPolicyDispatchRejectionReason::AdapterManualRequired);
    }
    if entry.matrix_row.proof_level != EnforcementPolicyDispatchProofLevel::ManualRequired
        || entry.matrix_row.rejection_reason
            != EnforcementPolicyDispatchRejectionReason::AdapterManualRequired
    {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }
    Ok(())
}

fn validate_rejected(
    entry: &EnforcementPolicyDispatchReadModelEntry,
) -> Result<(), EnforcementPolicyDispatchRejectionReason> {
    if entry.matrix_row.rejection_reason == EnforcementPolicyDispatchRejectionReason::None {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }
    Ok(())
}
