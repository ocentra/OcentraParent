use super::support::expect_agent_event;
use super::*;

pub(super) fn apply(result: &AgentServiceCommandResult) -> Result<(), String> {
    expect_agent_event(
        &result.response_event.event,
        &AgentEventName::AgentActivityAppGameTimerParentPreferenceSetupRequested,
    )
}
