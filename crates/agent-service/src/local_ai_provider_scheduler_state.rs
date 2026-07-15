use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiResourceClass;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerDecision;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobClass;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerLifecycle;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerQueue;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSingletonScope;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub(crate) struct LocalAiPhysicalDeviceId(pub(crate) String);

#[derive(Clone, Debug)]
pub(crate) struct LocalAiStatusText(pub(crate) String);

#[derive(Clone, Debug)]
pub(crate) struct LocalAiTimestamp(pub(crate) String);

pub(crate) fn status_unavailable(checked_at: LocalAiTimestamp) -> LocalAiProviderSchedulerStatus {
    status_unavailable_for_device(
        LocalAiPhysicalDeviceId(constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string()),
        checked_at,
    )
}

pub(crate) fn status_unavailable_for_device(
    physical_device_id: LocalAiPhysicalDeviceId,
    checked_at: LocalAiTimestamp,
) -> LocalAiProviderSchedulerStatus {
    LocalAiProviderSchedulerStatus {
        physical_device_id: physical_device_id.0,
        singleton_scope: LocalAiProviderSingletonScope::PhysicalDevice,
        provider_id: constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED.to_string(),
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_DEV_UNCONFIGURED
            .to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_UNCONFIGURED.to_string(),
        model_reference: constants::local_ai_runtime::MODEL_REFERENCE_UNCONFIGURED.to_string(),
        resource_class: LocalAiResourceClass::Cpu,
        lifecycle_state: LocalAiProviderSchedulerLifecycle::Unavailable,
        current_job_class: None,
        queue: LocalAiProviderSchedulerQueue::default(),
        duplicate_runtime_blocked: false,
        degraded_state: LocalAiDegradedState::ProviderUnavailable,
        unavailable_reason: Some(
            constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string(),
        ),
        last_checked_at: checked_at.0,
    }
}

pub(crate) fn copy_runtime_fields(
    status: &mut LocalAiProviderSchedulerStatus,
    runtime: &LocalModelRuntimeStatus,
) {
    status.provider_id = runtime.provider_id.clone();
    status.runtime_reference_id = runtime.runtime_reference_id.clone();
    status.model_id = runtime.model_id.clone();
    status.model_reference = runtime.model_reference.clone();
    status.resource_class = runtime.resource_class;
    status.last_checked_at = runtime.last_checked_at.clone();
}

pub(crate) fn decision_for(
    physical_device_id: LocalAiPhysicalDeviceId,
    runtime: &LocalModelRuntimeStatus,
    job_class: LocalAiProviderSchedulerJobClass,
    job_status: LocalAiProviderSchedulerJobStatus,
    queue_position: Option<u16>,
    unavailable_reason: Option<LocalAiStatusText>,
    duplicate_runtime_blocked: bool,
) -> LocalAiProviderSchedulerDecision {
    LocalAiProviderSchedulerDecision {
        physical_device_id: physical_device_id.0,
        job_class,
        job_status,
        selected_runtime_reference_id: if unavailable_reason.is_some() {
            None
        } else {
            Some(runtime.runtime_reference_id.clone())
        },
        queue_position,
        unavailable_reason: unavailable_reason.map(|reason| reason.0),
        duplicate_runtime_blocked,
    }
}

pub(crate) fn increment_queue(
    queue: &mut LocalAiProviderSchedulerQueue,
    job_class: &LocalAiProviderSchedulerJobClass,
) {
    match job_class {
        LocalAiProviderSchedulerJobClass::ChildSafety => {
            queue.child_safety_queued = queue.child_safety_queued.saturating_add(1);
        }
        LocalAiProviderSchedulerJobClass::ParentAssistant => {
            queue.parent_assistant_queued = queue.parent_assistant_queued.saturating_add(1);
        }
        LocalAiProviderSchedulerJobClass::ParentReport => {
            queue.parent_report_queued = queue.parent_report_queued.saturating_add(1);
        }
    }
}

pub(crate) fn decrement_queue(
    queue: &mut LocalAiProviderSchedulerQueue,
    job_class: &LocalAiProviderSchedulerJobClass,
) {
    match job_class {
        LocalAiProviderSchedulerJobClass::ChildSafety => {
            queue.child_safety_queued = queue.child_safety_queued.saturating_sub(1);
        }
        LocalAiProviderSchedulerJobClass::ParentAssistant => {
            queue.parent_assistant_queued = queue.parent_assistant_queued.saturating_sub(1);
        }
        LocalAiProviderSchedulerJobClass::ParentReport => {
            queue.parent_report_queued = queue.parent_report_queued.saturating_sub(1);
        }
    }
}
