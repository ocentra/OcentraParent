use ocentra_parent_agent_protocol::constants::enforcement as enforcement_constants;
use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementAdapterResultCode, EnforcementResult, EnforcementResultStatus,
    EnforcementRollbackState, EnforcementUnavailableReason,
};

use super::enforcement_result_capability::capability_state_result;
use super::enforcement_result_parts::EnforcementResultParts;
use super::enforcement_unavailable_status::build_unavailable_status;
use super::{EnforcementBoundaryInput, EnforcementBoundaryRejection};

pub(super) fn enforcement_result(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
) -> Result<EnforcementResult, EnforcementBoundaryRejection> {
    if action.dry_run {
        return Ok(dry_run_result(input, action));
    }
    if let Some(result) = capability_state_result(input, action) {
        return Ok(result);
    }
    if let Some(result) = no_adapter_action_result(input, action) {
        return Ok(result);
    }
    if !input.capability.supported_actions.contains(&action.mode) {
        return Ok(result(
            input,
            action,
            EnforcementResultParts {
                status: EnforcementResultStatus::Unavailable,
                adapter_result_code: EnforcementAdapterResultCode::AdapterUnavailable,
                completed_at: input.completed_at.clone(),
                unavailable_reason: Some(
                    enforcement_constants::UNAVAILABLE_UNSUPPORTED_ACTION.to_string(),
                ),
                failed_reason: None,
                rollback_token: action.rollback_token.clone(),
                rollback_state: EnforcementRollbackState::Unavailable,
                unavailable_status_reason: Some(EnforcementUnavailableReason::UnsupportedAction),
            },
        ));
    }

    let Some(adapter_outcome) = input.adapter_outcome.clone() else {
        return Err(EnforcementBoundaryRejection::AdapterResultRequired);
    };

    Ok(adapter_completed_result(input, action, adapter_outcome))
}

fn dry_run_result(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
) -> EnforcementResult {
    result(
        input,
        action,
        EnforcementResultParts {
            status: EnforcementResultStatus::WouldEnforce,
            adapter_result_code: EnforcementAdapterResultCode::DryRunNoAction,
            completed_at: input.completed_at.clone(),
            unavailable_reason: None,
            failed_reason: None,
            rollback_token: action.rollback_token.clone(),
            rollback_state: EnforcementRollbackState::NotRequired,
            unavailable_status_reason: None,
        },
    )
}

fn no_adapter_action_result(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
) -> Option<EnforcementResult> {
    matches!(
        action.mode,
        ocentra_parent_agent_protocol::enforcement::EnforcementMode::ObserveOnly
            | ocentra_parent_agent_protocol::enforcement::EnforcementMode::AskParent
            | ocentra_parent_agent_protocol::enforcement::EnforcementMode::TimeLimit
    )
    .then(|| {
        result(
            input,
            action,
            EnforcementResultParts {
                status: EnforcementResultStatus::NoOp,
                adapter_result_code: EnforcementAdapterResultCode::NoOp,
                completed_at: input.completed_at.clone(),
                unavailable_reason: None,
                failed_reason: None,
                rollback_token: action.rollback_token.clone(),
                rollback_state: EnforcementRollbackState::NotRequired,
                unavailable_status_reason: None,
            },
        )
    })
}

fn adapter_completed_result(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
    adapter_outcome: crate::enforcement_adapter::EnforcementAdapterOutcome,
) -> EnforcementResult {
    let unavailable_status_reason =
        super::enforcement_unavailable_status::adapter_unavailable_reason(&adapter_outcome);
    result(
        input,
        action,
        EnforcementResultParts {
            status: adapter_outcome.status,
            adapter_result_code: adapter_outcome.adapter_result_code,
            completed_at: adapter_outcome.completed_at,
            unavailable_reason: adapter_outcome.unavailable_reason,
            failed_reason: adapter_outcome.failed_reason,
            rollback_token: adapter_outcome
                .rollback_token
                .or_else(|| action.rollback_token.clone()),
            rollback_state: adapter_outcome.rollback_state,
            unavailable_status_reason,
        },
    )
}

pub(super) fn result(
    input: &EnforcementBoundaryInput,
    action: &EnforcementAction,
    parts: EnforcementResultParts,
) -> EnforcementResult {
    EnforcementResult {
        schema_version: input.decision.schema_version.clone(),
        result_id: input.result_id.clone(),
        action_id: action.action_id.clone(),
        status: parts.status,
        adapter_result_code: parts.adapter_result_code,
        started_at: input.requested_at.clone(),
        completed_at: parts.completed_at,
        rollback_token: parts.rollback_token,
        rollback_state: parts.rollback_state,
        unavailable_reason: parts.unavailable_reason,
        unavailable_status: parts.unavailable_status_reason.map(|reason| {
            build_unavailable_status(&input.decision.schema_version, &input.capability, reason)
        }),
        failed_reason: parts.failed_reason,
        next_check_at: action.expires_at.clone(),
        capability: input.capability.clone(),
    }
}
