use ocentra_parent_agent_protocol::enforcement_policy_dispatch::{
    EnforcementPolicyDispatchOutcomeState, EnforcementPolicyDispatchReadModelEntry,
    EnforcementPolicyDispatchRejectionReason,
};

#[path = "enforcement_policy_dispatch_matrix_action.rs"]
mod enforcement_policy_dispatch_matrix_action;
#[path = "enforcement_policy_dispatch_matrix_outcome.rs"]
mod enforcement_policy_dispatch_matrix_outcome;

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
    if entry.intent.requested_policy_action
        != enforcement_policy_dispatch_matrix_action::policy_action_for(
            entry.matrix_row.requested_action,
        )
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

    enforcement_policy_dispatch_matrix_outcome::validate_outcome(entry)
}
