use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementAdapterResultCode, EnforcementResult, EnforcementResultStatus,
    EnforcementRollbackState, EnforcementUnavailableReason, EnforcementUnavailableStatus,
};

use super::EnforcementTimerTransitionIds;

#[derive(Clone)]
pub(super) struct TransitionResultOverride {
    pub adapter_result_code: EnforcementAdapterResultCode,
    pub rollback_state: EnforcementRollbackState,
    pub unavailable_reason: Option<String>,
    pub failed_reason: Option<String>,
    pub rollback_token: Option<String>,
    pub parent_override: Option<ocentra_parent_agent_protocol::enforcement::ParentActionReference>,
}

pub(super) fn transition_result(
    action: &EnforcementAction,
    ids: &EnforcementTimerTransitionIds,
    status: EnforcementResultStatus,
    result_override: &TransitionResultOverride,
) -> EnforcementResult {
    let unavailable_status =
        unavailable_reason_from_transition(result_override).map(|unavailable_reason| {
            EnforcementUnavailableStatus {
                schema_version: action.schema_version.clone(),
                capability: action.capability.clone(),
                unavailable_reason,
                retryable: matches!(
                    unavailable_reason,
                    EnforcementUnavailableReason::AdapterUnavailable
                        | EnforcementUnavailableReason::AdapterError
                ),
                checked_at: action.capability.last_checked_at.clone(),
            }
        });

    EnforcementResult {
        schema_version: action.schema_version.clone(),
        result_id: ids.result_id.clone(),
        action_id: action.action_id.clone(),
        status,
        adapter_result_code: result_override.adapter_result_code,
        started_at: ids.observed_at.clone(),
        completed_at: Some(ids.observed_at.clone()),
        rollback_token: result_override
            .rollback_token
            .clone()
            .or_else(|| action.rollback_token.clone()),
        rollback_state: result_override.rollback_state,
        unavailable_reason: result_override.unavailable_reason.clone(),
        unavailable_status,
        failed_reason: result_override.failed_reason.clone(),
        next_check_at: next_check_at(action, status),
        capability: action.capability.clone(),
    }
}

pub(super) fn unavailable_reason_from_transition(
    result_override: &TransitionResultOverride,
) -> Option<EnforcementUnavailableReason> {
    result_override
        .unavailable_reason
        .as_deref()
        .and_then(unavailable_reason_from_protocol_str)
        .or_else(|| {
            (result_override.failed_reason.is_some())
                .then_some(EnforcementUnavailableReason::AdapterError)
        })
}

fn unavailable_reason_from_protocol_str(reason: &str) -> Option<EnforcementUnavailableReason> {
    match reason {
        enforcement_constants::UNAVAILABLE_UNSUPPORTED_PLATFORM => {
            Some(EnforcementUnavailableReason::UnsupportedPlatform)
        }
        enforcement_constants::UNAVAILABLE_UNSUPPORTED_ACTION => {
            Some(EnforcementUnavailableReason::UnsupportedAction)
        }
        enforcement_constants::UNAVAILABLE_MISSING_PERMISSION => {
            Some(EnforcementUnavailableReason::MissingPermission)
        }
        enforcement_constants::UNAVAILABLE_MISSING_DEPENDENCY => {
            Some(EnforcementUnavailableReason::MissingDependency)
        }
        enforcement_constants::UNAVAILABLE_ADAPTER_UNAVAILABLE => {
            Some(EnforcementUnavailableReason::AdapterUnavailable)
        }
        enforcement_constants::UNAVAILABLE_ADAPTER_ERROR => {
            Some(EnforcementUnavailableReason::AdapterError)
        }
        enforcement_constants::UNAVAILABLE_MANUAL_REQUIRED => {
            Some(EnforcementUnavailableReason::ManualRequired)
        }
        _ => None,
    }
}

fn next_check_at(action: &EnforcementAction, status: EnforcementResultStatus) -> Option<String> {
    match status {
        EnforcementResultStatus::NoOp => action.expires_at.clone(),
        _ => None,
    }
}
