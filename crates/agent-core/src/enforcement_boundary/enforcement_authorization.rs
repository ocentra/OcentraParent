use ocentra_parent_agent_protocol::{
    EnforcementAction, EnforcementCapabilityState, EnforcementMode,
};

use super::{
    enforcement_action, enforcement_mode, validate_intent_decision, EnforcementAdapterRequest,
    EnforcementBoundaryInput, EnforcementBoundaryRejection,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EnforcementAuthorizationOutcome {
    pub action: EnforcementAction,
    pub adapter_request: Option<EnforcementAdapterRequest>,
}

pub fn authorize_enforcement_boundary(
    input: EnforcementBoundaryInput,
) -> Result<EnforcementAuthorizationOutcome, EnforcementBoundaryRejection> {
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
