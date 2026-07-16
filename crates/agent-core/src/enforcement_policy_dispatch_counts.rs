use ocentra_parent_agent_protocol::enforcement_policy_dispatch::{
    EnforcementPolicyDispatchOutcomeState, EnforcementPolicyDispatchReadModelEntry,
    EnforcementPolicyDispatchTimerState,
};

use super::EnforcementPolicyDispatchValidation;

pub(super) fn update_validation_counts(
    validation: &mut EnforcementPolicyDispatchValidation,
    entry: &EnforcementPolicyDispatchReadModelEntry,
) {
    match entry.matrix_row.outcome_state {
        EnforcementPolicyDispatchOutcomeState::DispatchReady => {
            validation.dispatch_ready_count += 1;
        }
        EnforcementPolicyDispatchOutcomeState::Rejected => {
            validation.rejected_count += 1;
        }
        EnforcementPolicyDispatchOutcomeState::ManualRequired => {
            validation.manual_required_count += 1;
        }
        EnforcementPolicyDispatchOutcomeState::ReportOnly => {
            validation.report_only_count += 1;
        }
        EnforcementPolicyDispatchOutcomeState::DryRunOnly => {
            validation.dry_run_only_count += 1;
        }
        EnforcementPolicyDispatchOutcomeState::Degraded
        | EnforcementPolicyDispatchOutcomeState::Unavailable => {}
    }

    if entry.timer_state == EnforcementPolicyDispatchTimerState::RecoveryNeeded {
        validation.recovery_needed_count += 1;
    }
}
