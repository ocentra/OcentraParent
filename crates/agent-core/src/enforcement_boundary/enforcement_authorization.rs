use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementCapabilityState, EnforcementMode,
};

use super::enforcement_action::enforcement_action;
use super::enforcement_mode::enforcement_mode;
use super::enforcement_validation::validate_intent_decision;
use super::{EnforcementAdapterRequest, EnforcementBoundaryInput, EnforcementBoundaryRejection};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementAuthorizationOutcome {
    pub action: EnforcementAction,
    pub adapter_request: Option<EnforcementAdapterRequest>,
}

pub fn authorize_enforcement_boundary(
    input: EnforcementBoundaryInput,
) -> Result<EnforcementAuthorizationOutcome, EnforcementBoundaryRejection> {
    let EnforcementBoundaryInput {
        intent,
        decision,
        capability,
        action_id,
        result_id,
        audit_event_id,
        timer_event_id,
        rollback_token,
        policy_version,
        requested_at,
        completed_at,
        adapter_outcome,
        timer_event_kind,
    } = input;
    let input = EnforcementBoundaryInput {
        intent,
        decision,
        capability,
        action_id,
        result_id,
        audit_event_id,
        timer_event_id,
        rollback_token,
        policy_version,
        requested_at,
        completed_at,
        adapter_outcome,
        timer_event_kind,
    };
    validate_intent_decision(&input.intent, &input.decision)?;
    let mode = enforcement_mode(&input.intent)?;
    let action = enforcement_action(&input, mode);
    let adapter_request = authorized_adapter_request(&action);

    Ok(EnforcementAuthorizationOutcome {
        action,
        adapter_request,
    })
}

fn authorized_adapter_request(action: &EnforcementAction) -> Option<EnforcementAdapterRequest> {
    if action.dry_run
        || matches!(
            action.mode,
            EnforcementMode::ObserveOnly | EnforcementMode::AskParent | EnforcementMode::TimeLimit
        )
        || matches!(
            action.capability.capability_state,
            EnforcementCapabilityState::Unavailable
                | EnforcementCapabilityState::DryRun
                | EnforcementCapabilityState::ObserveOnly
                | EnforcementCapabilityState::ManualRequired
        )
        || !action.capability.supported_actions.contains(&action.mode)
    {
        return None;
    }

    Some(EnforcementAdapterRequest {
        action_id: action.action_id.clone(),
        adapter_kind: action.adapter_kind,
        mode: action.mode,
    })
}
