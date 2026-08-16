use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantAnswerState;

use super::ParentAssistantTextRef;

pub(super) fn answer_state_value(
    state: ParentAssistantAnswerState,
) -> ParentAssistantTextRef<'static> {
    match state {
        ParentAssistantAnswerState::Answered => {
            ParentAssistantTextRef(constants::parent_assistant::ANSWER_ANSWERED)
        }
        ParentAssistantAnswerState::Queued => {
            ParentAssistantTextRef(constants::parent_assistant::ANSWER_QUEUED)
        }
        ParentAssistantAnswerState::Degraded => {
            ParentAssistantTextRef(constants::parent_assistant::ANSWER_DEGRADED)
        }
        ParentAssistantAnswerState::Unavailable => {
            ParentAssistantTextRef(constants::parent_assistant::ANSWER_UNAVAILABLE)
        }
    }
}
