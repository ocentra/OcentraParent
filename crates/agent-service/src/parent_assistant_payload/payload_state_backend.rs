use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantBackendState;

use super::ParentAssistantTextRef;

pub(super) fn backend_state_value(
    state: ParentAssistantBackendState,
) -> ParentAssistantTextRef<'static> {
    match state {
        ParentAssistantBackendState::RuntimeBacked => {
            ParentAssistantTextRef(constants::parent_assistant::BACKEND_STATE_RUNTIME_BACKED)
        }
        ParentAssistantBackendState::DurableLocal => {
            ParentAssistantTextRef(constants::parent_assistant::BACKEND_STATE_DURABLE_LOCAL)
        }
        ParentAssistantBackendState::VolatileLocal => {
            ParentAssistantTextRef(constants::parent_assistant::BACKEND_STATE_VOLATILE_LOCAL)
        }
        ParentAssistantBackendState::ContractRequired => {
            ParentAssistantTextRef(constants::parent_assistant::BACKEND_STATE_CONTRACT_REQUIRED)
        }
        ParentAssistantBackendState::Unavailable => {
            ParentAssistantTextRef(constants::parent_assistant::BACKEND_STATE_UNAVAILABLE)
        }
    }
}
