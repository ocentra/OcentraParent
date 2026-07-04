use super::super::*;

pub(super) fn validate_thresholds(
    thresholds: &NetworkRiskBudgetThresholds,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    let ordered = thresholds.monitor_points <= thresholds.ask_parent_points
        && thresholds.ask_parent_points <= thresholds.warn_child_points
        && thresholds.warn_child_points <= thresholds.limit_points
        && thresholds.limit_points <= thresholds.block_points
        && thresholds.block_points <= thresholds.max_points
        && thresholds.monitor_points > 0;
    if ordered {
        Ok(())
    } else {
        Err(NetworkRiskBudgetThresholdError::InvalidThresholdOrder)
    }
}

pub(super) fn validate_policy(
    policy: &NetworkRiskBudgetHouseholdPolicy,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    if policy.household_policy_ref.trim().is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyHouseholdPolicyRef);
    }
    if policy.parent_rule_refs.is_empty() {
        return Err(NetworkRiskBudgetThresholdError::EmptyParentRuleRefs);
    }
    if policy
        .parent_rule_refs
        .iter()
        .any(|parent_rule_ref| parent_rule_ref.trim().is_empty())
    {
        return Err(NetworkRiskBudgetThresholdError::EmptyParentRuleRef);
    }
    Ok(())
}

pub(super) fn validate_safe_behavior_credit(
    policy: &NetworkRiskBudgetHouseholdPolicy,
) -> Result<(), NetworkRiskBudgetThresholdError> {
    let proof_refs_exist = policy
        .safe_behavior_credit_expiry_ref
        .as_ref()
        .is_some_and(|value| !value.trim().is_empty())
        && policy
            .safe_behavior_audit_reason_ref
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty())
        && policy
            .safe_behavior_ui_explanation_ref
            .as_ref()
            .is_some_and(|value| !value.trim().is_empty());
    if policy.safe_behavior_credit_cap_points > 0 && proof_refs_exist {
        Ok(())
    } else {
        Err(NetworkRiskBudgetThresholdError::SafeBehaviorCreditRequiresPolicyProof)
    }
}
