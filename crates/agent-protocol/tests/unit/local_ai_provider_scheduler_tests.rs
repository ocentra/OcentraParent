use super::{
    constants, LocalAiDegradedState, LocalAiProviderSchedulerDecision,
    LocalAiProviderSchedulerJobClass, LocalAiProviderSchedulerJobStatus,
    LocalAiProviderSchedulerLifecycle, LocalAiProviderSchedulerQueue,
    LocalAiProviderSchedulerStatus, LocalAiProviderSingletonScope, LocalAiResourceClass,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn local_ai_provider_scheduler_status_serializes_singleton_device_shape() {
    let status = LocalAiProviderSchedulerStatus {
        physical_device_id: constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
        singleton_scope: LocalAiProviderSingletonScope::PhysicalDevice,
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string(),
        model_reference: constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4.to_string(),
        resource_class: LocalAiResourceClass::Cpu,
        lifecycle_state: LocalAiProviderSchedulerLifecycle::Queued,
        current_job_class: Some(LocalAiProviderSchedulerJobClass::ChildSafety),
        queue: LocalAiProviderSchedulerQueue {
            child_safety_queued: 1,
            parent_assistant_queued: 1,
            parent_report_queued: 0,
        },
        duplicate_runtime_blocked: true,
        degraded_state: LocalAiDegradedState::Overloaded,
        unavailable_reason: None,
        last_checked_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
    };

    let serialized = serde_json::to_value(status).expect_value("scheduler status serializes");

    assert_eq!(
        serialized["physicalDeviceId"],
        constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL
    );
    assert_eq!(
        serialized["singletonScope"],
        constants::local_ai_runtime::SINGLETON_SCOPE_PHYSICAL_DEVICE
    );
    assert_eq!(
        serialized["currentJobClass"],
        constants::local_ai_runtime::SCHEDULER_JOB_CHILD_SAFETY
    );
    assert_eq!(serialized["queue"]["childSafetyQueued"], 1);
    assert_eq!(serialized["queue"]["parentAssistantQueued"], 1);
    assert_eq!(serialized["duplicateRuntimeBlocked"], true);
}

#[test]
fn local_ai_provider_scheduler_decision_serializes_queue_decision() {
    let decision = LocalAiProviderSchedulerDecision {
        physical_device_id: constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
        job_class: LocalAiProviderSchedulerJobClass::ParentAssistant,
        job_status: LocalAiProviderSchedulerJobStatus::Queued,
        selected_runtime_reference_id: Some(
            constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI.to_string(),
        ),
        queue_position: Some(2),
        unavailable_reason: None,
        duplicate_runtime_blocked: true,
    };

    let serialized = serde_json::to_value(decision).expect_value("scheduler decision serializes");

    assert_eq!(
        serialized["jobClass"],
        constants::local_ai_runtime::SCHEDULER_JOB_PARENT_ASSISTANT
    );
    assert_eq!(
        serialized["jobStatus"],
        constants::local_ai_runtime::SCHEDULER_JOB_STATUS_QUEUED
    );
    assert_eq!(serialized["queuePosition"], 2);
    assert_eq!(serialized["duplicateRuntimeBlocked"], true);
}
