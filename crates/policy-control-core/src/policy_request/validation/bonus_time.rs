#![forbid(unsafe_code)]

use ocentra_eventing::error::EventingError;
use ocentra_parent_agent_protocol::constants::policy_control;

use super::super::{PolicyRequestKind, PolicyRequestScope, PolicyTemporaryOverride};
use crate::policy_source::PolicyRuleAction;

pub(super) fn validate_override(
    override_record: &PolicyTemporaryOverride,
) -> Result<(), EventingError> {
    if override_record.request_kind == PolicyRequestKind::BonusTime {
        return validate_bonus_time_override(override_record);
    }
    if override_record.approved_bonus_minutes.is_some() {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_APPROVED_BONUS_MINUTES,
            value: "only-bonus-time-overrides-may-carry-bonus-minutes".to_string(),
        });
    }
    Ok(())
}

fn validate_bonus_time_override(
    override_record: &PolicyTemporaryOverride,
) -> Result<(), EventingError> {
    if override_record.approved_bonus_minutes.is_none() {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_APPROVED_BONUS_MINUTES,
            value: policy_control::request::VALUE_BONUS_TIME_APPROVAL_REQUIRES_MINUTES.to_string(),
        });
    }
    if !matches!(
        override_record.approved_action,
        PolicyRuleAction::Allow | PolicyRuleAction::TimeLimit
    ) {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_APPROVED_BONUS_MINUTES,
            value: "bonus-time-overrides-require-allow-or-time-limit".to_string(),
        });
    }
    Ok(())
}

pub(super) fn validate_request_scope(scope: &PolicyRequestScope) -> Result<(), EventingError> {
    if scope.request_kind == PolicyRequestKind::BonusTime {
        return validate_bonus_time_request(scope);
    }
    if scope.requested_bonus_minutes.is_some() {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_REQUESTED_BONUS_MINUTES,
            value: "only-bonus-time-requests-may-include-minutes".to_string(),
        });
    }
    Ok(())
}

fn validate_bonus_time_request(scope: &PolicyRequestScope) -> Result<(), EventingError> {
    if scope.requested_bonus_minutes.is_none() {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_REQUESTED_BONUS_MINUTES,
            value: policy_control::request::VALUE_BONUS_TIME_REQUEST_REQUIRES_MINUTES.to_string(),
        });
    }
    if !matches!(
        scope.requested_action,
        PolicyRuleAction::Allow | PolicyRuleAction::TimeLimit
    ) {
        return Err(EventingError::InvalidValue {
            field: policy_control::request::FIELD_REQUESTED_BONUS_MINUTES,
            value: "bonus-time-requests-require-allow-or-time-limit".to_string(),
        });
    }
    Ok(())
}
