#![forbid(unsafe_code)]

use super::super::PolicyContractAction;
use super::super::{
    PolicyContractApprovalKind, PolicyContractApprovalRequest, PolicyContractOverrideGrant,
    PolicyContractOverrideType, PolicyContractValidationResult,
};

pub(crate) fn validate_policy_override_grant_type_rules(
    grant: &PolicyContractOverrideGrant,
    approval: &PolicyContractApprovalRequest,
) -> PolicyContractValidationResult {
    match grant.override_type {
        PolicyContractOverrideType::TemporaryAllow => {
            if grant.action != PolicyContractAction::Allow || grant.bonus_time_minutes.is_some() {
                return Err(
                    "temporary-allow overrides must resolve to allow without bonus time".into(),
                );
            }
        }
        PolicyContractOverrideType::TemporaryBlock => {
            if grant.action != PolicyContractAction::Block || grant.bonus_time_minutes.is_some() {
                return Err(
                    "temporary-block overrides must resolve to block without bonus time".into(),
                );
            }
        }
        PolicyContractOverrideType::BonusTime => {
            if approval.kind != PolicyContractApprovalKind::BonusTime {
                return Err("bonus-time overrides require a bonus-time approval request".into());
            }
            if !matches!(
                grant.action,
                PolicyContractAction::Allow | PolicyContractAction::TimeLimit
            ) {
                return Err(
                    "bonus-time overrides must keep the action within allow or time-limit".into(),
                );
            }
        }
    }

    Ok(())
}
