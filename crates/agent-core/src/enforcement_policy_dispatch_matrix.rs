use ocentra_parent_agent_protocol::activity::policy::PolicyAction;
use ocentra_parent_agent_protocol::enforcement::EnforcementCapabilityState;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::{
    EnforcementPolicyDispatchApprovalState, EnforcementPolicyDispatchOutcomeState,
    EnforcementPolicyDispatchProofLevel, EnforcementPolicyDispatchReadModelEntry,
    EnforcementPolicyDispatchRejectionReason,
};
use ocentra_parent_agent_protocol::enforcement_product_control_spine::V08EnforcementProductControlParentAction;

pub(super) fn validate_entry_matrix(
    entry: &EnforcementPolicyDispatchReadModelEntry,
) -> Result<(), EnforcementPolicyDispatchRejectionReason> {
    if entry.matrix_row.source_state != entry.intent.source_state {
        return Err(EnforcementPolicyDispatchRejectionReason::SourceNotReady);
    }
    if entry.matrix_row.platform.as_protocol_str() != entry.intent.device.platform.as_str() {
        return Err(EnforcementPolicyDispatchRejectionReason::WrongDevice);
    }
    if entry.matrix_row.requested_action != entry.intent.requested_parent_action {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }
    if entry.intent.requested_policy_action != policy_action_for(entry.matrix_row.requested_action)
    {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }
    if entry.child_reason_code.trim().is_empty()
        || !entry.reason_codes.contains(&entry.child_reason_code)
    {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }
    if entry.child_reason_code != entry.matrix_row.child_reason_code {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }
    if entry.matrix_row.outcome_state == EnforcementPolicyDispatchOutcomeState::DryRunOnly
        && !entry.intent.dry_run
    {
        return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
    }

    match entry.matrix_row.outcome_state {
        EnforcementPolicyDispatchOutcomeState::DispatchReady => {
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
        }
        EnforcementPolicyDispatchOutcomeState::ManualRequired => {
            if entry.matrix_row.capability_state != EnforcementCapabilityState::ManualRequired {
                return Err(EnforcementPolicyDispatchRejectionReason::AdapterManualRequired);
            }
            if entry.matrix_row.proof_level != EnforcementPolicyDispatchProofLevel::ManualRequired
                || entry.matrix_row.rejection_reason
                    != EnforcementPolicyDispatchRejectionReason::AdapterManualRequired
            {
                return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
            }
        }
        EnforcementPolicyDispatchOutcomeState::Rejected => {
            if entry.matrix_row.rejection_reason == EnforcementPolicyDispatchRejectionReason::None {
                return Err(EnforcementPolicyDispatchRejectionReason::BroadClaimNotProved);
            }
        }
        EnforcementPolicyDispatchOutcomeState::ReportOnly
        | EnforcementPolicyDispatchOutcomeState::DryRunOnly
        | EnforcementPolicyDispatchOutcomeState::Degraded
        | EnforcementPolicyDispatchOutcomeState::Unavailable => {}
    }

    Ok(())
}

fn policy_action_for(action: V08EnforcementProductControlParentAction) -> PolicyAction {
    match action {
        V08EnforcementProductControlParentAction::Warn => PolicyAction::Warn,
        V08EnforcementProductControlParentAction::TimeLimit => PolicyAction::TimeLimit,
        V08EnforcementProductControlParentAction::BlockScopedProcess => PolicyAction::Block,
        V08EnforcementProductControlParentAction::AskParent => PolicyAction::AskParent,
        V08EnforcementProductControlParentAction::Observe
        | V08EnforcementProductControlParentAction::DryRunPreview
        | V08EnforcementProductControlParentAction::ReportOnly => PolicyAction::Allow,
    }
}
