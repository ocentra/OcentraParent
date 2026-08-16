use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerLifecycle;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantRunState;

pub(super) fn run_state_for_status(
    runtime_unavailable: bool,
    lifecycle_state: &LocalAiProviderSchedulerLifecycle,
) -> ParentAssistantRunState {
    if runtime_unavailable {
        return ParentAssistantRunState::Unavailable;
    }

    match lifecycle_state {
        LocalAiProviderSchedulerLifecycle::Running => ParentAssistantRunState::Active,
        LocalAiProviderSchedulerLifecycle::Queued => ParentAssistantRunState::Queued,
        LocalAiProviderSchedulerLifecycle::Degraded => ParentAssistantRunState::Degraded,
        LocalAiProviderSchedulerLifecycle::Unavailable => ParentAssistantRunState::Unavailable,
        LocalAiProviderSchedulerLifecycle::Idle => ParentAssistantRunState::Completed,
    }
}
