use ocentra_parent_agent_protocol::{
    constants::v08_enforcement_policy_dispatch as dispatch, EnforcementCapabilityState,
    EnforcementPolicyDispatchOutcomeState, EnforcementPolicyDispatchReadModel,
    EnforcementPolicyDispatchReadModelEntry, EnforcementPolicyDispatchRejectionReason,
    EnforcementPolicyDispatchSourceState, EnforcementPolicyDispatchTimerState,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementPolicyDispatchValidation {
    pub dispatch_ready_count: usize,
    pub dry_run_only_count: usize,
    pub rejected_count: usize,
    pub manual_required_count: usize,
    pub report_only_count: usize,
    pub recovery_needed_count: usize,
}

pub fn validate_enforcement_policy_dispatch_read_model(
    read_model: &EnforcementPolicyDispatchReadModel,
) -> Result<EnforcementPolicyDispatchValidation, EnforcementPolicyDispatchRejectionReason> {
    let mut validation = EnforcementPolicyDispatchValidation {
        dispatch_ready_count: 0,
        dry_run_only_count: 0,
        rejected_count: 0,
        manual_required_count: 0,
        report_only_count: 0,
        recovery_needed_count: 0,
    };

    for entry in &read_model.entries {
        validate_entry_identity(entry)?;
        validate_entry_matrix(entry)?;
        update_validation_counts(&mut validation, entry);
    }

    Ok(validation)
}

fn validate_entry_identity(
    entry: &EnforcementPolicyDispatchReadModelEntry,
) -> Result<(), EnforcementPolicyDispatchRejectionReason> {
    if entry.intent.actor.actor_id.is_empty() {
        return Err(EnforcementPolicyDispatchRejectionReason::MissingActor);
    }
    if entry.intent.device.device_id != dispatch::LOCAL_DEV_AGENT_DEVICE_ID {
        return Err(EnforcementPolicyDispatchRejectionReason::WrongDevice);
    }
    if !has_dispatch_reference_prefix(&entry.intent.policy_decision_id, dispatch::PREFIX_POLICY)
        || !has_dispatch_reference_prefix(
            &entry.intent.policy_decision_ref,
            dispatch::PREFIX_DECISION,
        )
    {
        return Err(EnforcementPolicyDispatchRejectionReason::MissingPolicyDecision);
    }
    if entry.intent.policy_version != dispatch::POLICY_VERSION_V0_8_DISPATCH {
        return Err(EnforcementPolicyDispatchRejectionReason::StalePolicyVersion);
    }
    if !has_dispatch_reference_prefix(&entry.intent.schedule_ref, dispatch::PREFIX_SCHEDULE) {
        return Err(EnforcementPolicyDispatchRejectionReason::MissingScheduleOrBudget);
    }
    if entry.intent.evidence_references.is_empty() {
        return Err(EnforcementPolicyDispatchRejectionReason::MissingEvidence);
    }
    if entry.intent.route_ref != dispatch::LOCAL_DEV_AGENT_ROUTE_REF {
        return Err(EnforcementPolicyDispatchRejectionReason::RouteNotAuthorized);
    }
    if entry.intent.source_state != EnforcementPolicyDispatchSourceState::Ready
        && entry.matrix_row.outcome_state != EnforcementPolicyDispatchOutcomeState::Rejected
    {
        return Err(EnforcementPolicyDispatchRejectionReason::SourceNotReady);
    }

    Ok(())
}

fn validate_entry_matrix(
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

fn has_dispatch_reference_prefix(value: &str, prefix: &str) -> bool {
    value.starts_with(prefix) && value.len() > prefix.len()
}

fn update_validation_counts(
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
