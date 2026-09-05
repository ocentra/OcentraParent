use std::fmt::{Display, Formatter, Result as FormatResult};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantProviderState;

pub(crate) struct ParentAssistantProviderStateValue(&'static str);

impl Display for ParentAssistantProviderStateValue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> FormatResult {
        formatter.write_str(self.0)
    }
}

pub(crate) fn parent_assistant_provider_state_value(
    state: ParentAssistantProviderState,
) -> ParentAssistantProviderStateValue {
    ParentAssistantProviderStateValue(match state {
        ParentAssistantProviderState::Configured => {
            constants::parent_assistant::PROVIDER_CONFIGURED
        }
        ParentAssistantProviderState::Degraded => constants::parent_assistant::PROVIDER_DEGRADED,
        ParentAssistantProviderState::Unavailable => {
            constants::parent_assistant::PROVIDER_UNAVAILABLE
        }
    })
}
