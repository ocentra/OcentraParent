use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use ocentra_parent_agent_protocol::{
    constants, LocalAiChatGenerationResult, LocalAiGenerationState,
    LocalAiProviderSchedulerJobClass, LocalAiProviderSchedulerJobStatus,
    LocalAiProviderSchedulerLifecycle, LocalAiResourceClass, LocalModelRuntimeStatus,
};

use crate::local_ai_provider_scheduler::LocalAiProviderSchedulerRuntime;

#[test]
fn unavailable_runtime_marks_scheduler_unavailable_without_queue() {
    let scheduler = LocalAiProviderSchedulerRuntime::new_for_test();
    let runtime = unavailable_runtime();

    let decision = scheduler
        .record_unavailable_job(&runtime, LocalAiProviderSchedulerJobClass::ParentAssistant);
    let status = scheduler.status_snapshot();

    assert_eq!(
        decision.job_status,
        LocalAiProviderSchedulerJobStatus::Unavailable
    );
    assert_eq!(
        decision.unavailable_reason,
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string())
    );
    assert_eq!(
        status.lifecycle_state,
        LocalAiProviderSchedulerLifecycle::Unavailable
    );
    assert_eq!(status.queue.total(), 0);
    assert_eq!(status.current_job_class, None);
}

#[test]
fn child_safety_queue_promotes_before_parent_assistant_queue() {
    let scheduler = LocalAiProviderSchedulerRuntime::new_for_test();
    let runtime = ready_runtime();

    scheduler.record_running_job(&runtime, LocalAiProviderSchedulerJobClass::ParentReport);
    scheduler.record_queued_job(&runtime, LocalAiProviderSchedulerJobClass::ParentAssistant);
    scheduler.record_queued_job(&runtime, LocalAiProviderSchedulerJobClass::ChildSafety);

    let status = scheduler.complete_current_job(&runtime);

    assert_eq!(
        status.lifecycle_state,
        LocalAiProviderSchedulerLifecycle::Running
    );
    assert_eq!(
        status.current_job_class,
        Some(LocalAiProviderSchedulerJobClass::ChildSafety)
    );
    assert_eq!(status.queue.child_safety_queued, 0);
    assert_eq!(status.queue.parent_assistant_queued, 1);
    assert!(status.duplicate_runtime_blocked);
}

#[tokio::test]
async fn parent_and_child_jobs_share_one_runtime_lane() {
    let scheduler = Arc::new(LocalAiProviderSchedulerRuntime::new_for_test());
    let runtime = ready_runtime();
    let active_jobs = Arc::new(AtomicUsize::new(0));
    let max_active_jobs = Arc::new(AtomicUsize::new(0));

    let child_scheduler = Arc::clone(&scheduler);
    let child_runtime = runtime.clone();
    let child_active = Arc::clone(&active_jobs);
    let child_max = Arc::clone(&max_active_jobs);
    let child = tokio::spawn(async move {
        child_scheduler
            .run_generation_job(
                LocalAiProviderSchedulerJobClass::ChildSafety,
                child_runtime,
                || async move {
                    observed_job_result(
                        constants::local_ai_runtime::SCHEDULER_JOB_CHILD_SAFETY,
                        child_active,
                        child_max,
                    )
                    .await
                },
            )
            .await
    });

    let parent_scheduler = Arc::clone(&scheduler);
    let parent_runtime = runtime.clone();
    let parent_active = Arc::clone(&active_jobs);
    let parent_max = Arc::clone(&max_active_jobs);
    let parent = tokio::spawn(async move {
        parent_scheduler
            .run_generation_job(
                LocalAiProviderSchedulerJobClass::ParentAssistant,
                parent_runtime,
                || async move {
                    observed_job_result(
                        constants::local_ai_runtime::SCHEDULER_JOB_PARENT_ASSISTANT,
                        parent_active,
                        parent_max,
                    )
                    .await
                },
            )
            .await
    });

    let (child_result, parent_result) = tokio::join!(child, parent);

    assert_eq!(
        child_result
            .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS)
            .generation_state,
        LocalAiGenerationState::Complete
    );
    assert_eq!(
        parent_result
            .expect(constants::error::LOCAL_AI_RUNTIME_SPAWNS)
            .generation_state,
        LocalAiGenerationState::Complete
    );
    assert_eq!(max_active_jobs.load(Ordering::SeqCst), 1);
    assert_eq!(
        scheduler.status_snapshot().lifecycle_state,
        LocalAiProviderSchedulerLifecycle::Idle
    );
}

async fn observed_job_result(
    message_id: &'static str,
    active_jobs: Arc<AtomicUsize>,
    max_active_jobs: Arc<AtomicUsize>,
) -> LocalAiChatGenerationResult {
    let active = active_jobs.fetch_add(1, Ordering::SeqCst) + 1;
    max_active_jobs.fetch_max(active, Ordering::SeqCst);
    tokio::time::sleep(Duration::from_millis(25)).await;
    active_jobs.fetch_sub(1, Ordering::SeqCst);
    completed_result(message_id)
}

fn ready_runtime() -> LocalModelRuntimeStatus {
    LocalModelRuntimeStatus {
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string(),
        model_reference: constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4.to_string(),
        privacy_mode: ocentra_parent_agent_protocol::LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: ocentra_parent_agent_protocol::LocalAiAdapterBoundary::LocalAdapterReady,
        execution_state: ocentra_parent_agent_protocol::LocalAiExecutionState::DryRunReady,
        provider_source: ocentra_parent_agent_protocol::LocalAiProviderSource::LocalModelCache,
        load_state: ocentra_parent_agent_protocol::LocalAiModelLoadState::Loaded,
        capability_flags: vec![
            ocentra_parent_agent_protocol::LocalAiCapabilityFlag::ChatCompletion,
        ],
        resource_class: LocalAiResourceClass::Cpu,
        degraded_state: ocentra_parent_agent_protocol::LocalAiDegradedState::None,
        last_checked_at: constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
        unavailable_reason: None,
    }
}

fn unavailable_runtime() -> LocalModelRuntimeStatus {
    let mut runtime = ready_runtime();
    runtime.runtime_reference_id =
        constants::local_ai_runtime::RUNTIME_REFERENCE_DEV_UNCONFIGURED.to_string();
    runtime.provider_id = constants::local_ai_runtime::PROVIDER_ID_UNCONFIGURED.to_string();
    runtime.model_id = constants::local_ai_runtime::MODEL_ID_UNCONFIGURED.to_string();
    runtime.model_reference = constants::local_ai_runtime::MODEL_REFERENCE_UNCONFIGURED.to_string();
    runtime.unavailable_reason =
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED.to_string());
    runtime
}

fn completed_result(message_id: &str) -> LocalAiChatGenerationResult {
    LocalAiChatGenerationResult {
        local_ai_result_id: {
            let mut value = constants::local_ai_runtime::RESULT_ID_PREFIX.to_string();
            value.push_str(message_id);
            value
        },
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string(),
        model_reference: constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4.to_string(),
        generation_state: LocalAiGenerationState::Complete,
        output_text: Some(constants::local_ai_runtime::TEST_PROMPT.to_string()),
        prompt_char_count: constants::local_ai_runtime::TEST_PROMPT.chars().count() as u64,
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        duration_ms: 1,
        exit_code: Some(0),
        stderr_byte_size: 0,
        unavailable_reason: None,
    }
}
