use ocentra_parent_agent_protocol::enforcement::EnforcementCapabilityState;
use ocentra_parent_agent_protocol::enforcement_policy_dispatch::{
    EnforcementPolicyDispatchOutcomeState, EnforcementPolicyDispatchReadModelEntry,
    EnforcementPolicyDispatchRejectionReason,
};

pub(super) fn validate_entry_matrix(
    entry: &EnforcementPolicyDispatchReadModelEntry,
) -> Result<(), EnforcementPolicyDispatchRejectionReason> {
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
            if entry.matrix_row.capability_state != EnforcementCapabilityState::Supported {
                return Err(EnforcementPolicyDispatchRejectionReason::AdapterUnavailable);
            }
            if entry.matrix_row.rejection_reason != EnforcementPolicyDispatchRejectionReason::None {
                return Err(entry.matrix_row.rejection_reason);
            }
        }
        EnforcementPolicyDispatchOutcomeState::ManualRequired => {
            if entry.matrix_row.capability_state != EnforcementCapabilityState::ManualRequired {
                return Err(EnforcementPolicyDispatchRejectionReason::AdapterManualRequired);
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
