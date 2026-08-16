use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementAdapterResultCode, EnforcementCapabilityState, EnforcementResult,
    EnforcementResultStatus, EnforcementRollbackState, EnforcementUnavailableReason,
};

use super::enforcement_result_parts::EnforcementResultParts;
use super::enforcement_unavailable_status::capability_unavailable_reason;
use super::EnforcementBoundaryInput;

pub(super) fn capability_state_result(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
) -> Option<EnforcementResult> {
    match input.capability.capability_state {
        EnforcementCapabilityState::Unavailable | EnforcementCapabilityState::ManualRequired => {
            let unavailable_reason = capability_unavailable_reason(&input.capability);
            Some(super::enforcement_result::result(
                input,
                action,
                EnforcementResultParts {
                    status: EnforcementResultStatus::Unavailable,
                    adapter_result_code: adapter_result_code_for_unavailable_reason(
                        unavailable_reason,
                    ),
                    completed_at: input.completed_at.clone(),
                    unavailable_reason: Some(unavailable_reason.as_protocol_str().to_string()),
                    failed_reason: None,
                    rollback_token: action.rollback_token.clone(),
                    rollback_state: EnforcementRollbackState::Unavailable,
                    unavailable_status_reason: Some(unavailable_reason),
                },
            ))
        }
        EnforcementCapabilityState::ObserveOnly => Some(super::enforcement_result::result(
            input,
            action,
            EnforcementResultParts {
                status: EnforcementResultStatus::NoOp,
                adapter_result_code: EnforcementAdapterResultCode::LeftRunningObserveOnly,
                completed_at: input.completed_at.clone(),
                unavailable_reason: None,
                failed_reason: None,
                rollback_token: action.rollback_token.clone(),
                rollback_state: EnforcementRollbackState::NotRequired,
                unavailable_status_reason: None,
            },
        )),
        EnforcementCapabilityState::Supported
        | EnforcementCapabilityState::Degraded
        | EnforcementCapabilityState::DryRun => None,
    }
}

fn adapter_result_code_for_unavailable_reason(
    unavailable_reason: EnforcementUnavailableReason,
) -> EnforcementAdapterResultCode {
    match unavailable_reason {
        EnforcementUnavailableReason::UnsupportedPlatform => {
            EnforcementAdapterResultCode::UnsupportedPlatform
        }
        EnforcementUnavailableReason::AdapterError => EnforcementAdapterResultCode::AdapterFailed,
        EnforcementUnavailableReason::UnsupportedAction
        | EnforcementUnavailableReason::MissingPermission
        | EnforcementUnavailableReason::MissingDependency
        | EnforcementUnavailableReason::AdapterUnavailable
        | EnforcementUnavailableReason::ManualRequired => {
            EnforcementAdapterResultCode::AdapterUnavailable
        }
    }
}
