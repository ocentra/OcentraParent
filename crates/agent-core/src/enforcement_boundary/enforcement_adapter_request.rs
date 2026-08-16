use ocentra_parent_agent_protocol::enforcement::{
    EnforcementAction, EnforcementMode, EnforcementResult, EnforcementResultStatus,
};

use super::EnforcementAdapterRequest;

pub(super) fn adapter_request(
    action: &EnforcementAction,
    result: &EnforcementResult,
) -> Option<EnforcementAdapterRequest> {
    if action.dry_run
        || result.status != EnforcementResultStatus::WouldEnforce
        || matches!(
            action.mode,
            EnforcementMode::ObserveOnly | EnforcementMode::AskParent | EnforcementMode::TimeLimit
        )
    {
        return None;
    }

    Some(EnforcementAdapterRequest {
        action_id: action.action_id.clone(),
        adapter_kind: action.adapter_kind,
        mode: action.mode,
    })
}
