use std::primitive::str as TestStr;
use std::{
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiGenerationState;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiResourceClass;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobClass;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerLifecycle;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSingletonScope;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use tokio::sync::{Mutex as TokioMutex, Notify};

use crate::local_ai_provider_scheduler::{
    local_ai_provider_scheduler, LocalAiProviderSchedulerRuntime,
};
use crate::local_ai_provider_scheduler_queue::{
    LocalAiProviderRuntimeLaneAdmission, LocalAiProviderRuntimeLaneQueue, MAX_PENDING_RUNTIME_JOBS,
};
use crate::local_ai_provider_scheduler_state::LocalAiPhysicalDeviceId;
use crate::test_invariants::require_ok;

#[test]
fn unavailable_runtime_marks_scheduler_unavailable_without_queue() {
    let scheduler = LocalAiProviderSchedulerRuntime::new();
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

#[tokio::test]
async fn unavailable_runtime_does_not_execute_provider_closure() {
    let scheduler = LocalAiProviderSchedulerRuntime::new();
    let invoked = Arc::new(AtomicBool::new(false));
    let closure_invoked = Arc::clone(&invoked);

    let result = scheduler
        .run_generation_job(
            LocalAiProviderSchedulerJobClass::ChildSafety,
            unavailable_runtime(),
            move || async move {
                closure_invoked.store(true, Ordering::SeqCst);
                completed_result(constants::local_ai_runtime::SCHEDULER_JOB_CHILD_SAFETY)
            },
        )
        .await;

    assert_eq!(result.generation_state, LocalAiGenerationState::Unavailable);
    assert!(!invoked.load(Ordering::SeqCst));
    assert_eq!(
        scheduler.status_snapshot().lifecycle_state,
        LocalAiProviderSchedulerLifecycle::Unavailable
    );
}

#[tokio::test]
async fn aborting_active_generation_releases_lane_and_status() {
    let scheduler = Arc::new(LocalAiProviderSchedulerRuntime::new());
    let holder_started = Arc::new(Notify::new());
    let holder = {
        let scheduler = Arc::clone(&scheduler);
        let runtime = ready_runtime();
        let holder_started = Arc::clone(&holder_started);
        tokio::spawn(async move {
            scheduler
                .run_generation_job(
                    LocalAiProviderSchedulerJobClass::ParentReport,
                    runtime,
                    || async move {
                        holder_started.notify_one();
                        tokio::time::sleep(Duration::from_secs(3600)).await;
                        completed_result(constants::local_ai_runtime::SCHEDULER_JOB_PARENT_REPORT)
                    },
                )
                .await
        })
    };

    holder_started.notified().await;
    let running_status = scheduler.status_snapshot();
    assert_eq!(
        running_status.lifecycle_state,
        LocalAiProviderSchedulerLifecycle::Running
    );
    assert_eq!(
        running_status.current_job_class,
        Some(LocalAiProviderSchedulerJobClass::ParentReport)
    );

    holder.abort();
    assert!(holder.await.is_err());

    let recovered_status = scheduler.status_snapshot();
    assert_eq!(
        recovered_status.lifecycle_state,
        LocalAiProviderSchedulerLifecycle::Idle
    );
    assert_eq!(recovered_status.current_job_class, None);
    assert_eq!(recovered_status.queue.total(), 0);
    assert!(!recovered_status.duplicate_runtime_blocked);

    let result = scheduler
        .run_generation_job(
            LocalAiProviderSchedulerJobClass::ChildSafety,
            ready_runtime(),
            || async {
                completed_result(constants::local_ai_runtime::SCHEDULER_JOB_CHILD_SAFETY)
            },
        )
        .await;
    assert_eq!(result.generation_state, LocalAiGenerationState::Complete);
    assert_idle_singleton_scheduler_status(&scheduler);
}

#[tokio::test]
async fn aborting_queued_generation_removes_queue_state_and_preserves_lane() {
    let scheduler = Arc::new(LocalAiProviderSchedulerRuntime::new());
    let holder_started = Arc::new(Notify::new());
    let release_holder = Arc::new(Notify::new());
    let observed_jobs = Arc::new(TokioMutex::new(Vec::new()));
    let holder = spawn_observed_job(
        Arc::clone(&scheduler),
        ready_runtime(),
        LocalAiProviderSchedulerJobClass::ParentReport,
        constants::local_ai_runtime::SCHEDULER_JOB_PARENT_REPORT,
        Arc::clone(&observed_jobs),
        Some(Arc::clone(&holder_started)),
        Some(Arc::clone(&release_holder)),
    );
    holder_started.notified().await;

    let queued_invoked = Arc::new(AtomicBool::new(false));
    let queued = {
        let scheduler = Arc::clone(&scheduler);
        let queued_invoked = Arc::clone(&queued_invoked);
        tokio::spawn(async move {
            scheduler
                .run_generation_job(
                    LocalAiProviderSchedulerJobClass::ParentAssistant,
                    ready_runtime(),
                    || async move {
                        queued_invoked.store(true, Ordering::SeqCst);
                        completed_result(
                            constants::local_ai_runtime::SCHEDULER_JOB_PARENT_ASSISTANT,
                        )
                    },
                )
                .await
        })
    };

    wait_until_scheduler_status(&scheduler, |status| {
        status.queue.parent_assistant_queued == 1
    })
    .await;
    queued.abort();
    assert!(queued.await.is_err());

    let cancelled_status = scheduler.status_snapshot();
    assert_eq!(
        cancelled_status.lifecycle_state,
        LocalAiProviderSchedulerLifecycle::Running
    );
    assert_eq!(
        cancelled_status.current_job_class,
        Some(LocalAiProviderSchedulerJobClass::ParentReport)
    );
    assert_eq!(cancelled_status.queue.parent_assistant_queued, 0);
    assert_eq!(cancelled_status.queue.total(), 0);
    assert!(cancelled_status.duplicate_runtime_blocked);
    assert!(!queued_invoked.load(Ordering::SeqCst));

    release_holder.notify_one();
    assert_completed_generation(holder.await);

    let result = scheduler
        .run_generation_job(
            LocalAiProviderSchedulerJobClass::ChildSafety,
            ready_runtime(),
            || async {
                completed_result(constants::local_ai_runtime::SCHEDULER_JOB_CHILD_SAFETY)
            },
        )
        .await;
    assert_eq!(result.generation_state, LocalAiGenerationState::Complete);
    assert_idle_singleton_scheduler_status(&scheduler);
}

#[test]
fn runtime_lane_bounds_pending_jobs_and_removes_cancelled_waiters() {
    let mut queue = LocalAiProviderRuntimeLaneQueue::new();
    assert!(matches!(
        queue.reserve(LocalAiProviderSchedulerJobClass::ParentReport),
        LocalAiProviderRuntimeLaneAdmission::Running
    ));

    let mut waiters = Vec::new();
    for _ in 0..MAX_PENDING_RUNTIME_JOBS {
        match queue.reserve(LocalAiProviderSchedulerJobClass::ParentReport) {
            LocalAiProviderRuntimeLaneAdmission::Queued(waiter) => waiters.push(waiter),
            LocalAiProviderRuntimeLaneAdmission::Running
            | LocalAiProviderRuntimeLaneAdmission::Rejected => {
                panic!("queue should accept exactly its bounded pending capacity")
            }
        }
    }
    assert!(matches!(
        queue.reserve(LocalAiProviderSchedulerJobClass::ParentReport),
        LocalAiProviderRuntimeLaneAdmission::Rejected
    ));

    drop(waiters);
    let _ = queue.finish_running();
    assert!(matches!(
        queue.reserve(LocalAiProviderSchedulerJobClass::ParentReport),
        LocalAiProviderRuntimeLaneAdmission::Running
    ));
}

#[tokio::test]
async fn failed_generation_remains_visible_as_degraded_scheduler_state() {
    let scheduler = LocalAiProviderSchedulerRuntime::new();
    let result = scheduler
        .run_generation_job(
            LocalAiProviderSchedulerJobClass::ParentAssistant,
            ready_runtime(),
            || async {
                let mut result =
                    completed_result(constants::local_ai_runtime::SCHEDULER_JOB_PARENT_ASSISTANT);
                result.generation_state = LocalAiGenerationState::Failed;
                result.output_text = None;
                result.exit_code = None;
                result.unavailable_reason = Some(
                    constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_PROCESS_FAILED
                        .to_string(),
                );
                result
            },
        )
        .await;

    assert_eq!(result.generation_state, LocalAiGenerationState::Failed);
    let status = scheduler.status_snapshot();
    assert_eq!(
        status.lifecycle_state,
        LocalAiProviderSchedulerLifecycle::Degraded
    );
    assert_eq!(status.degraded_state, LocalAiDegradedState::InvalidOutput);
    assert_eq!(
        status.unavailable_reason,
        Some(constants::local_ai_runtime::UNAVAILABLE_REASON_RUNTIME_PROCESS_FAILED.to_string())
    );
}

#[tokio::test]
async fn parent_and_child_jobs_share_one_runtime_lane() {
    let scheduler = Arc::new(LocalAiProviderSchedulerRuntime::new());
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
        join_generation_result(child_result).generation_state,
        LocalAiGenerationState::Complete
    );
    assert_eq!(
        join_generation_result(parent_result).generation_state,
        LocalAiGenerationState::Complete
    );
    assert_eq!(max_active_jobs.load(Ordering::SeqCst), 1);
    assert_idle_singleton_scheduler_status(&scheduler);
}

#[tokio::test]
async fn physical_devices_use_independent_runtime_lanes_without_duplicate_loads_per_device() {
    let scheduler = Arc::new(LocalAiProviderSchedulerRuntime::new());
    let runtime = ready_runtime();
    let active_jobs = Arc::new(AtomicUsize::new(0));
    let max_active_jobs = Arc::new(AtomicUsize::new(0));

    let first_device = spawn_observed_device_job(
        Arc::clone(&scheduler),
        LocalAiPhysicalDeviceId(constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string()),
        runtime.clone(),
        Arc::clone(&active_jobs),
        Arc::clone(&max_active_jobs),
    );
    let second_device = spawn_observed_device_job(
        Arc::clone(&scheduler),
        LocalAiPhysicalDeviceId(
            constants::local_ai_runtime::PHYSICAL_DEVICE_SECOND_LOCAL.to_string(),
        ),
        runtime,
        Arc::clone(&active_jobs),
        Arc::clone(&max_active_jobs),
    );

    let (first_result, second_result) = tokio::join!(first_device, second_device);

    assert_completed_generation(first_result);
    assert_completed_generation(second_result);
    assert_eq!(max_active_jobs.load(Ordering::SeqCst), 2);
    assert_idle_singleton_scheduler_status_for_device(
        &scheduler,
        &LocalAiPhysicalDeviceId(constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string()),
    );
    assert_idle_singleton_scheduler_status_for_device(
        &scheduler,
        &LocalAiPhysicalDeviceId(
            constants::local_ai_runtime::PHYSICAL_DEVICE_SECOND_LOCAL.to_string(),
        ),
    );
}

#[tokio::test]
async fn child_safety_job_preempts_queued_parent_job_after_runtime_lane_frees() {
    let scheduler = Arc::new(LocalAiProviderSchedulerRuntime::new());
    let runtime = ready_runtime();
    let holder_started = Arc::new(Notify::new());
    let release_holder = Arc::new(Notify::new());
    let observed_jobs = Arc::new(TokioMutex::new(Vec::new()));

    let holder = spawn_observed_job(
        Arc::clone(&scheduler),
        runtime.clone(),
        LocalAiProviderSchedulerJobClass::ParentReport,
        constants::local_ai_runtime::SCHEDULER_JOB_PARENT_REPORT,
        Arc::clone(&observed_jobs),
        Some(Arc::clone(&holder_started)),
        Some(Arc::clone(&release_holder)),
    );

    holder_started.notified().await;

    let parent = spawn_observed_job(
        Arc::clone(&scheduler),
        runtime.clone(),
        LocalAiProviderSchedulerJobClass::ParentAssistant,
        constants::local_ai_runtime::SCHEDULER_JOB_PARENT_ASSISTANT,
        Arc::clone(&observed_jobs),
        None,
        None,
    );

    wait_until_scheduler_status(&scheduler, |status| {
        status.queue.parent_assistant_queued == 1
    })
    .await;

    let child = spawn_observed_job(
        Arc::clone(&scheduler),
        runtime,
        LocalAiProviderSchedulerJobClass::ChildSafety,
        constants::local_ai_runtime::SCHEDULER_JOB_CHILD_SAFETY,
        Arc::clone(&observed_jobs),
        None,
        None,
    );

    wait_until_scheduler_status(&scheduler, |status| {
        status.queue.parent_assistant_queued == 1 && status.queue.child_safety_queued == 1
    })
    .await;

    let queued_status = scheduler.status_snapshot();
    assert_eq!(
        queued_status.lifecycle_state,
        LocalAiProviderSchedulerLifecycle::Queued
    );
    assert_eq!(
        queued_status.current_job_class,
        Some(LocalAiProviderSchedulerJobClass::ParentReport)
    );
    assert!(queued_status.duplicate_runtime_blocked);
    assert_eq!(
        queued_status.runtime_reference_id,
        constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
    );

    release_holder.notify_one();

    let (holder_result, parent_result, child_result) = tokio::join!(holder, parent, child);

    assert_completed_generation(holder_result);
    assert_completed_generation(parent_result);
    assert_completed_generation(child_result);
    assert_observed_job_order(&observed_jobs).await;
    assert_eq!(
        scheduler.status_snapshot().lifecycle_state,
        LocalAiProviderSchedulerLifecycle::Idle
    );
}

#[test]
fn singleton_scheduler_and_queue_record_helpers_stay_linked() {
    let _ = local_ai_provider_scheduler();
    let scheduler = LocalAiProviderSchedulerRuntime::new();
    let decision = scheduler.record_queued_job(
        &ready_runtime(),
        LocalAiProviderSchedulerJobClass::ParentAssistant,
    );

    assert_eq!(
        decision.job_status,
        LocalAiProviderSchedulerJobStatus::Queued
    );
    assert_eq!(scheduler.status_snapshot().queue.parent_assistant_queued, 1);
}

async fn observed_job_result(
    message_id: &'static TestStr,
    active_jobs: Arc<AtomicUsize>,
    max_active_jobs: Arc<AtomicUsize>,
) -> LocalAiChatGenerationResult {
    let active = active_jobs.fetch_add(1, Ordering::SeqCst) + 1;
    max_active_jobs.fetch_max(active, Ordering::SeqCst);
    tokio::time::sleep(Duration::from_millis(25)).await;
    active_jobs.fetch_sub(1, Ordering::SeqCst);
    completed_result(message_id)
}

fn assert_completed_generation(
    result: Result<LocalAiChatGenerationResult, tokio::task::JoinError>,
) {
    assert_eq!(
        join_generation_result(result).generation_state,
        LocalAiGenerationState::Complete
    );
}

fn join_generation_result(
    result: Result<LocalAiChatGenerationResult, tokio::task::JoinError>,
) -> LocalAiChatGenerationResult {
    require_ok(result, constants::error::LOCAL_AI_RUNTIME_SPAWNS)
}

async fn assert_observed_job_order(
    observed_jobs: &TokioMutex<Vec<LocalAiProviderSchedulerJobClass>>,
) {
    let observed = observed_jobs.lock().await.clone();
    assert_eq!(
        observed,
        vec![
            LocalAiProviderSchedulerJobClass::ParentReport,
            LocalAiProviderSchedulerJobClass::ChildSafety,
            LocalAiProviderSchedulerJobClass::ParentAssistant,
        ]
    );
}

fn assert_idle_singleton_scheduler_status(scheduler: &LocalAiProviderSchedulerRuntime) {
    assert_idle_singleton_scheduler_status_for_device(
        scheduler,
        &LocalAiPhysicalDeviceId(constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string()),
    );
}

fn assert_idle_singleton_scheduler_status_for_device(
    scheduler: &LocalAiProviderSchedulerRuntime,
    physical_device_id: &LocalAiPhysicalDeviceId,
) {
    let status = scheduler.status_snapshot();
    let status = if physical_device_id.0 == constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL {
        status
    } else {
        scheduler.status_snapshot_for_device(physical_device_id.clone())
    };
    assert_eq!(
        status.lifecycle_state,
        LocalAiProviderSchedulerLifecycle::Idle
    );
    assert_eq!(
        status.singleton_scope,
        LocalAiProviderSingletonScope::PhysicalDevice
    );
    assert_eq!(status.physical_device_id, physical_device_id.0);
    assert_eq!(
        status.runtime_reference_id,
        constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
    );
    assert!(!status.duplicate_runtime_blocked);
}

fn spawn_observed_device_job(
    scheduler: Arc<LocalAiProviderSchedulerRuntime>,
    physical_device_id: LocalAiPhysicalDeviceId,
    runtime: LocalModelRuntimeStatus,
    active_jobs: Arc<AtomicUsize>,
    max_active_jobs: Arc<AtomicUsize>,
) -> tokio::task::JoinHandle<LocalAiChatGenerationResult> {
    tokio::spawn(async move {
        scheduler
            .run_generation_job_for_device(
                physical_device_id.clone(),
                LocalAiProviderSchedulerJobClass::ParentAssistant,
                runtime,
                || async move {
                    observed_job_result(
                        Box::leak(physical_device_id.0.into_boxed_str()),
                        active_jobs,
                        max_active_jobs,
                    )
                    .await
                },
            )
            .await
    })
}

fn spawn_observed_job(
    scheduler: Arc<LocalAiProviderSchedulerRuntime>,
    runtime: LocalModelRuntimeStatus,
    job_class: LocalAiProviderSchedulerJobClass,
    message_id: &'static TestStr,
    observed_jobs: Arc<TokioMutex<Vec<LocalAiProviderSchedulerJobClass>>>,
    started: Option<Arc<Notify>>,
    release: Option<Arc<Notify>>,
) -> tokio::task::JoinHandle<LocalAiChatGenerationResult> {
    tokio::spawn(async move {
        scheduler
            .run_generation_job(job_class, runtime, || async move {
                observed_jobs.lock().await.push(job_class);
                if let Some(started) = started {
                    started.notify_one();
                }
                if let Some(release) = release {
                    release.notified().await;
                }
                completed_result(message_id)
            })
            .await
    })
}

async fn wait_until_scheduler_status(
    scheduler: &LocalAiProviderSchedulerRuntime,
    condition: impl Fn(&LocalAiProviderSchedulerStatus) -> bool,
) {
    for _ in 0..100 {
        if condition(&scheduler.status_snapshot()) {
            return;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    assert!(condition(&scheduler.status_snapshot()));
}

fn ready_runtime() -> LocalModelRuntimeStatus {
    LocalModelRuntimeStatus {
        runtime_reference_id: constants::local_ai_runtime::RUNTIME_REFERENCE_LOCAL_LLAMA_CLI
            .to_string(),
        provider_id: constants::local_ai_runtime::PROVIDER_ID_LOCAL_LLAMA_CLI.to_string(),
        model_id: constants::local_ai_runtime::MODEL_ID_DEFAULT_GEMMA_4.to_string(),
        model_reference: constants::local_ai_runtime::MODEL_REFERENCE_DEFAULT_GEMMA_4.to_string(),
        privacy_mode: ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiProviderPrivacyMode::LocalOnly,
        adapter_boundary: ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiAdapterBoundary::LocalAdapterReady,
        execution_state: ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiExecutionState::DryRunReady,
        provider_source: ocentra_parent_agent_protocol::local_ai_runtime_boundary::LocalAiProviderSource::LocalModelCache,
        load_state: ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiModelLoadState::Loaded,
        capability_flags: vec![
            ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiCapabilityFlag::ChatCompletion,
        ],
        resource_class: LocalAiResourceClass::Cpu,
        degraded_state: ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState::None,
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

fn completed_result(message_id: &TestStr) -> LocalAiChatGenerationResult {
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
