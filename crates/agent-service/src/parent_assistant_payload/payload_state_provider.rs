use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantProviderState;

use super::ParentAssistantTextRef;

pub(super) fn provider_state_value(
    state: ParentAssistantProviderState,
) -> ParentAssistantTextRef<'static> {
    match state {
        ParentAssistantProviderState::Configured => {
            ParentAssistantTextRef(constants::parent_assistant::PROVIDER_CONFIGURED)
        }
        ParentAssistantProviderState::Degraded => {
            ParentAssistantTextRef(constants::parent_assistant::PROVIDER_DEGRADED)
        }
        ParentAssistantProviderState::Unavailable => {
            ParentAssistantTextRef(constants::parent_assistant::PROVIDER_UNAVAILABLE)
        }
    }
}
