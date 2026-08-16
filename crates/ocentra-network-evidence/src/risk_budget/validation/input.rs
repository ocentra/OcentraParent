use super::super::*;

pub(super) fn validate_input(
    input: &NetworkRiskBudgetThresholdInput,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    if input.evaluation_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyEvaluationRef);
    }
    if input.child_profile_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyChildProfileRef);
    }
    if input.risk_budget_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyRiskBudgetRef);
    }
    if input.cascade_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyCascadeRef);
    }
    super::validate_claims(input)?;
    super::validate_thresholds(&input.thresholds)?;
    super::validate_policy(&input.household_policy)?;
    if input.signals.is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptySignals);
    }

    let mut signal_refs = std::collections::BTreeSet::new();
    let mut safe_behavior_credit_requested = false;
    for signal in &input.signals {
        super::validate_signal(signal)?;
        if !signal_refs.insert(signal.signal_ref.as_str()) {
            return Err(NetworkRiskBudgetThresholdError::DuplicateSignalRef);
        }
        safe_behavior_credit_requested |= signal.safe_behavior_credit_points > 0;
    }
    if safe_behavior_credit_requested {
        super::validate_safe_behavior_credit(&input.household_policy)?;
    }
    for prior_event in &input.prior_events {
        if prior_event.event_ref.trim().is_empty() {
            return Err(NetworkRiskBudgetThresholdError::EmptyPriorEventRef);
        }
    }
    Ok(())
}
