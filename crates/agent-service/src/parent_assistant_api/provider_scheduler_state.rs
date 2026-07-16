use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerLifecycle;
use ocentra_parent_agent_protocol::parent_assistant::ParentAssistantProviderState;

pub(super) fn provider_state_for_status(
    runtime_unavailable: bool,
    busy: bool,
    degraded_state: &LocalAiDegradedState,
) -> ParentAssistantProviderState {
    if runtime_unavailable {
        return ParentAssistantProviderState::Unavailable;
    }

    if busy || *degraded_state != LocalAiDegradedState::None {
        return ParentAssistantProviderState::Degraded;
    }

    ParentAssistantProviderState::Configured
}

pub(super) fn scheduler_job_status_for_status(
    runtime_unavailable: bool,
    lifecycle_state: &LocalAiProviderSchedulerLifecycle,
) -> LocalAiProviderSchedulerJobStatus {
    if runtime_unavailable {
        return LocalAiProviderSchedulerJobStatus::Unavailable;
    }

    match lifecycle_state {
        LocalAiProviderSchedulerLifecycle::Running => LocalAiProviderSchedulerJobStatus::Running,
        LocalAiProviderSchedulerLifecycle::Queued => LocalAiProviderSchedulerJobStatus::Queued,
        LocalAiProviderSchedulerLifecycle::Degraded => LocalAiProviderSchedulerJobStatus::Degraded,
        LocalAiProviderSchedulerLifecycle::Unavailable => {
            LocalAiProviderSchedulerJobStatus::Unavailable
        }
        LocalAiProviderSchedulerLifecycle::Idle => LocalAiProviderSchedulerJobStatus::Complete,
    }
}
