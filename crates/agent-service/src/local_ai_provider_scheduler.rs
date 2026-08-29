use std::{
    collections::HashMap,
    future::Future,
    sync::{Mutex, OnceLock},
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::{
    LocalAiDegradedState, LocalAiGenerationState, LocalAiModelLoadState,
};
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerDecision;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobClass;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerLifecycle;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerQueue;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;
use ocentra_parent_agent_protocol::local_ai_runtime_boundary::{
    LocalAiAdapterBoundary, LocalAiExecutionState, LocalAiProviderSource,
};
use crate::{
    local_ai_provider_scheduler_queue::{
        LocalAiProviderRuntimeLaneAdmission, LocalAiProviderRuntimeLaneQueue,
        LocalAiProviderRuntimeLaneWaiter,
    },
    local_ai_provider_scheduler_state::{
        copy_runtime_fields, decision_for, decrement_queue, increment_queue, status_unavailable,
        status_unavailable_for_device, LocalAiPhysicalDeviceId, LocalAiStatusText,
        LocalAiTimestamp,
    },
    time::timestamp_now,
};

static LOCAL_AI_PROVIDER_SCHEDULER: OnceLock<LocalAiProviderSchedulerRuntime> = OnceLock::new();

const SCHEDULER_UNAVAILABLE_RESULT_ID: &str = "local-ai-scheduler-unavailable";
const SCHEDULER_DEGRADED_RESULT_ID: &str = "local-ai-scheduler-degraded";
const SCHEDULER_QUEUE_FULL_REASON: &str = "local-ai-provider-scheduler-queue-full";
const SCHEDULER_RUNTIME_NOT_READY_REASON: &str = "local-ai-provider-runtime-not-ready";
const SCHEDULER_ADAPTER_UNAVAILABLE_REASON: &str = "local-ai-provider-adapter-unavailable";
const SCHEDULER_EXECUTION_DISABLED_REASON: &str = "local-ai-provider-execution-disabled";
const SCHEDULER_PROVIDER_SOURCE_UNAVAILABLE_REASON: &str = "local-ai-provider-source-unavailable";
const SCHEDULER_MODEL_NOT_READY_REASON: &str = "local-ai-provider-model-not-ready";
const SCHEDULER_CAPABILITY_UNAVAILABLE_REASON: &str = "local-ai-provider-capability-unavailable";

pub(crate) fn local_ai_provider_scheduler() -> &'static LocalAiProviderSchedulerRuntime {
    LOCAL_AI_PROVIDER_SCHEDULER.get_or_init(LocalAiProviderSchedulerRuntime::new)
}

pub(crate) struct LocalAiProviderSchedulerRuntime {
    lanes: Mutex<HashMap<LocalAiPhysicalDeviceId, LocalAiProviderRuntimeLaneQueue>>,
    states: Mutex<HashMap<LocalAiPhysicalDeviceId, LocalAiProviderSchedulerStatus>>,
}

impl LocalAiProviderSchedulerRuntime {
    pub(crate) fn new() -> Self {
        Self {
            lanes: Mutex::new(HashMap::new()),
            states: Mutex::new(HashMap::from([(
                LocalAiPhysicalDeviceId(
                    constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
                ),
                status_unavailable(LocalAiTimestamp(timestamp_now())),
            )])),
        }
    }

    pub(crate) fn status_snapshot(&self) -> LocalAiProviderSchedulerStatus {
        self.status_snapshot_for_device(LocalAiPhysicalDeviceId(
            constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
        ))
    }

    pub(crate) fn status_snapshot_for_device(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
    ) -> LocalAiProviderSchedulerStatus {
        self.states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .get(&physical_device_id)
            .cloned()
            .unwrap_or_else(|| {
                status_unavailable_for_device(physical_device_id, LocalAiTimestamp(timestamp_now()))
            })
    }

    pub(crate) fn record_queued_job(
        &self,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderSchedulerDecision {
        self.record_queued_job_for_device(
            LocalAiPhysicalDeviceId(constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string()),
            runtime,
            job_class,
        )
    }

    pub(crate) fn record_queued_job_for_device(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderSchedulerDecision {
        let mut states = self
            .states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let status = status_for_device(&mut states, physical_device_id.clone(), runtime);
        increment_queue(&mut status.queue, &job_class);
        status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Queued;
        status.duplicate_runtime_blocked = true;
        status.degraded_state = LocalAiDegradedState::Overloaded;
        copy_runtime_fields(status, runtime);
        decision_for(
            physical_device_id,
            runtime,
            job_class,
            LocalAiProviderSchedulerJobStatus::Queued,
            Some(status.queue.total()),
            None,
            true,
        )
    }

    pub(crate) fn record_running_job_for_device(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderSchedulerDecision {
        let mut states = self
            .states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let status = status_for_device(&mut states, physical_device_id.clone(), runtime);
        decrement_queue(&mut status.queue, &job_class);
        status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Running;
        status.current_job_class = Some(job_class);
        status.duplicate_runtime_blocked = true;
        status.degraded_state = if status.queue.total() > 0 {
            LocalAiDegradedState::Overloaded
        } else {
            LocalAiDegradedState::None
        };
        status.unavailable_reason = None;
        copy_runtime_fields(status, runtime);
        decision_for(
            physical_device_id,
            runtime,
            job_class,
            LocalAiProviderSchedulerJobStatus::Running,
            None,
            None,
            true,
        )
    }

    pub(crate) fn record_unavailable_job(
        &self,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderSchedulerDecision {
        self.record_unavailable_job_for_device(
            LocalAiPhysicalDeviceId(constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string()),
            runtime,
            job_class,
        )
    }

    pub(crate) fn record_unavailable_job_for_device(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderSchedulerDecision {
        let reason = runtime
            .unavailable_reason
            .as_deref()
            .unwrap_or(constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED);
        let mut states = self
            .states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let status = status_for_device(&mut states, physical_device_id.clone(), runtime);
        status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Unavailable;
        status.current_job_class = None;
        status.queue = LocalAiProviderSchedulerQueue::default();
        status.duplicate_runtime_blocked = false;
        status.degraded_state = LocalAiDegradedState::ProviderUnavailable;
        status.unavailable_reason = Some(reason.to_string());
        copy_runtime_fields(status, runtime);
        decision_for(
            physical_device_id,
            runtime,
            job_class,
            LocalAiProviderSchedulerJobStatus::Unavailable,
            None,
            Some(LocalAiStatusText(reason.to_string())),
            false,
        )
    }

    pub(crate) async fn run_generation_job<F, Fut>(
        &self,
        job_class: LocalAiProviderSchedulerJobClass,
        runtime: LocalModelRuntimeStatus,
        run: F,
    ) -> LocalAiChatGenerationResult
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = LocalAiChatGenerationResult>,
    {
        self.run_generation_job_for_device(
            LocalAiPhysicalDeviceId(constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string()),
            job_class,
            runtime,
            run,
        )
        .await
    }

    pub(crate) async fn run_generation_job_for_device<F, Fut>(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        job_class: LocalAiProviderSchedulerJobClass,
        runtime: LocalModelRuntimeStatus,
        run: F,
    ) -> LocalAiChatGenerationResult
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = LocalAiChatGenerationResult>,
    {
        if let Some(reason) = runtime_unavailable_reason(&runtime) {
            let unavailable_runtime = runtime_with_reason(&runtime, reason);
            self.record_unavailable_job_for_device(
                physical_device_id,
                &unavailable_runtime,
                job_class,
            );
            return unavailable_generation_result(&unavailable_runtime);
        }

        let mut lease = match self.reserve_runtime_lane(physical_device_id.clone(), job_class) {
            LocalAiProviderRuntimeLaneAdmission::Running => {
                self.record_running_job_for_device(
                    physical_device_id.clone(),
                    &runtime,
                    job_class,
                );
                LocalAiProviderRuntimeLaneLease::running(
                    self,
                    physical_device_id,
                    runtime.clone(),
                    job_class,
                )
            }
            LocalAiProviderRuntimeLaneAdmission::Queued(waiter) => {
                self.record_queued_job_for_device(
                    physical_device_id.clone(),
                    &runtime,
                    job_class,
                );
                waiter.notify_if_lane_idle();
                let lease = LocalAiProviderRuntimeLaneLease::queued(
                    self,
                    physical_device_id.clone(),
                    runtime.clone(),
                    job_class,
                    waiter,
                );
                if let Some(waiter) = lease.waiter.as_ref() {
                    self.wait_for_runtime_lane(physical_device_id, waiter, &runtime, job_class)
                        .await;
                }
                lease
            }
            LocalAiProviderRuntimeLaneAdmission::Rejected => {
                self.record_degraded_job_for_device(
                    physical_device_id,
                    &runtime,
                    job_class,
                    SCHEDULER_QUEUE_FULL_REASON,
                );
                return degraded_generation_result(&runtime);
            }
        };

        let result = run().await;
        lease.finish(&result);
        result
    }

    fn reserve_runtime_lane(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderRuntimeLaneAdmission {
        let mut lanes = self
            .lanes
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        lanes
            .entry(physical_device_id)
            .or_insert_with(LocalAiProviderRuntimeLaneQueue::new)
            .reserve(job_class)
    }

    async fn wait_for_runtime_lane(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        waiter: &LocalAiProviderRuntimeLaneWaiter,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) {
        loop {
            waiter.notify.notified().await;
            let admitted = {
                let mut lanes = self
                    .lanes
                    .lock()
                    .unwrap_or_else(std::sync::PoisonError::into_inner);
                lanes
                    .entry(physical_device_id.clone())
                    .or_insert_with(LocalAiProviderRuntimeLaneQueue::new)
                    .try_admit_queued(waiter)
            };
            if admitted {
                self.record_running_job_for_device(physical_device_id, runtime, job_class);
                return;
            }
        }
    }

    fn finish_runtime_lane(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        runtime: &LocalModelRuntimeStatus,
        result: &LocalAiChatGenerationResult,
    ) {
        let waiting_jobs = {
            let mut lanes = self
                .lanes
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let waiting_jobs = lanes
                .entry(physical_device_id.clone())
                .or_insert_with(LocalAiProviderRuntimeLaneQueue::new)
                .finish_running();
        self.finish_runtime_lane_state(physical_device_id, runtime, Some(result));
            waiting_jobs
        };
        for waiting_job in waiting_jobs {
            waiting_job.notify_one();
        }
    }

    fn finish_runtime_lane_without_result(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        runtime: &LocalModelRuntimeStatus,
    ) {
        let waiting_jobs = {
            let mut lanes = self
                .lanes
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let waiting_jobs = lanes
                .entry(physical_device_id.clone())
                .or_insert_with(LocalAiProviderRuntimeLaneQueue::new)
                .finish_running();
            self.finish_runtime_lane_state(physical_device_id, runtime, None);
            waiting_jobs
        };
        for waiting_job in waiting_jobs {
            waiting_job.notify_one();
        }
    }

    fn cancel_queued_runtime_lane_job(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
        waiter: &LocalAiProviderRuntimeLaneWaiter,
    ) {
        waiter.cancel();
        {
            let mut lanes = self
                .lanes
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            let lane = lanes
                .entry(physical_device_id.clone())
                .or_insert_with(LocalAiProviderRuntimeLaneQueue::new);
            lane.cancel_waiter(waiter);
            if lane.is_idle() {
                lane.notify_next_waiter();
            }
        }

        let mut states = self
            .states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let status = status_for_device(&mut states, physical_device_id, runtime);
        decrement_queue(&mut status.queue, &job_class);
        status.lifecycle_state = if status.current_job_class.is_some() {
            LocalAiProviderSchedulerLifecycle::Running
        } else if status.queue.total() > 0 {
            LocalAiProviderSchedulerLifecycle::Queued
        } else {
            LocalAiProviderSchedulerLifecycle::Idle
        };
        status.duplicate_runtime_blocked =
            status.current_job_class.is_some() || status.queue.total() > 0;
        status.degraded_state = if status.queue.total() > 0 {
            LocalAiDegradedState::Overloaded
        } else {
            LocalAiDegradedState::None
        };
        status.unavailable_reason = None;
        copy_runtime_fields(status, runtime);
    }

    fn finish_runtime_lane_state(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        runtime: &LocalModelRuntimeStatus,
        result: Option<&LocalAiChatGenerationResult>,
    ) {
        let mut states = self
            .states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let status = status_for_device(&mut states, physical_device_id, runtime);
        status.current_job_class = None;
        let result_degraded_state = result.and_then(degraded_state_for_generation);
        status.lifecycle_state = if status.queue.total() > 0 {
            if result_degraded_state.is_some() {
                LocalAiProviderSchedulerLifecycle::Degraded
            } else {
                LocalAiProviderSchedulerLifecycle::Queued
            }
        } else if result_degraded_state.is_some() {
            LocalAiProviderSchedulerLifecycle::Degraded
        } else {
            LocalAiProviderSchedulerLifecycle::Idle
        };
        status.duplicate_runtime_blocked = status.queue.total() > 0;
        status.degraded_state = result_degraded_state.unwrap_or_else(|| {
            if status.queue.total() > 0 {
                LocalAiDegradedState::Overloaded
            } else {
                LocalAiDegradedState::None
            }
        });
        status.unavailable_reason = result.and_then(|result| result.unavailable_reason.clone());
        copy_runtime_fields(status, runtime);
    }

    fn record_degraded_job_for_device(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
        reason: &'static str,
    ) -> LocalAiProviderSchedulerDecision {
        let mut states = self
            .states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let status = status_for_device(&mut states, physical_device_id.clone(), runtime);
        status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Degraded;
        status.duplicate_runtime_blocked = true;
        status.degraded_state = LocalAiDegradedState::Overloaded;
        status.unavailable_reason = Some(reason.to_string());
        copy_runtime_fields(status, runtime);
        decision_for(
            physical_device_id,
            runtime,
            job_class,
            LocalAiProviderSchedulerJobStatus::Degraded,
            Some(status.queue.total()),
            Some(LocalAiStatusText(reason.to_string())),
            true,
        )
    }
}

struct LocalAiProviderRuntimeLaneLease<'a> {
    scheduler: &'a LocalAiProviderSchedulerRuntime,
    physical_device_id: LocalAiPhysicalDeviceId,
    runtime: LocalModelRuntimeStatus,
    job_class: LocalAiProviderSchedulerJobClass,
    waiter: Option<LocalAiProviderRuntimeLaneWaiter>,
    finished: bool,
}

impl<'a> LocalAiProviderRuntimeLaneLease<'a> {
    fn running(
        scheduler: &'a LocalAiProviderSchedulerRuntime,
        physical_device_id: LocalAiPhysicalDeviceId,
        runtime: LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> Self {
        Self {
            scheduler,
            physical_device_id,
            runtime,
            job_class,
            waiter: None,
            finished: false,
        }
    }

    fn queued(
        scheduler: &'a LocalAiProviderSchedulerRuntime,
        physical_device_id: LocalAiPhysicalDeviceId,
        runtime: LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
        waiter: LocalAiProviderRuntimeLaneWaiter,
    ) -> Self {
        Self {
            scheduler,
            physical_device_id,
            runtime,
            job_class,
            waiter: Some(waiter),
            finished: false,
        }
    }

    fn finish(&mut self, result: &LocalAiChatGenerationResult) {
        if self.finished {
            return;
        }
        self.scheduler.finish_runtime_lane(
            self.physical_device_id.clone(),
            &self.runtime,
            result,
        );
        self.finished = true;
    }
}

impl Drop for LocalAiProviderRuntimeLaneLease<'_> {
    fn drop(&mut self) {
        if self.finished {
            return;
        }
        if let Some(waiter) = self.waiter.as_ref() {
            if !waiter.is_admitted() {
                self.scheduler.cancel_queued_runtime_lane_job(
                    self.physical_device_id.clone(),
                    &self.runtime,
                    self.job_class,
                    waiter,
                );
                return;
            }
        }
        self.scheduler.finish_runtime_lane_without_result(
            self.physical_device_id.clone(),
            &self.runtime,
        );
    }
}

fn runtime_with_reason(
    runtime: &LocalModelRuntimeStatus,
    reason: String,
) -> LocalModelRuntimeStatus {
    let mut unavailable_runtime = runtime.clone();
    unavailable_runtime.unavailable_reason = Some(reason);
    unavailable_runtime
}

fn runtime_unavailable_reason(runtime: &LocalModelRuntimeStatus) -> Option<String> {
    if let Some(reason) = runtime.unavailable_reason.as_deref() {
        return Some(reason.to_string());
    }
    if runtime.resource_class
        == ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiResourceClass::RemoteUnavailable
    {
        return Some(SCHEDULER_RUNTIME_NOT_READY_REASON.to_string());
    }
    if runtime.adapter_boundary != LocalAiAdapterBoundary::LocalAdapterReady {
        return Some(SCHEDULER_ADAPTER_UNAVAILABLE_REASON.to_string());
    }
    if matches!(
        runtime.execution_state,
        LocalAiExecutionState::Disabled | LocalAiExecutionState::Failed
    ) {
        return Some(SCHEDULER_EXECUTION_DISABLED_REASON.to_string());
    }
    if runtime.provider_source == LocalAiProviderSource::Unavailable {
        return Some(SCHEDULER_PROVIDER_SOURCE_UNAVAILABLE_REASON.to_string());
    }
    if runtime.load_state != LocalAiModelLoadState::Loaded {
        return Some(SCHEDULER_MODEL_NOT_READY_REASON.to_string());
    }
    if runtime.capability_flags.is_empty() {
        return Some(SCHEDULER_CAPABILITY_UNAVAILABLE_REASON.to_string());
    }
    None
}

fn unavailable_generation_result(runtime: &LocalModelRuntimeStatus) -> LocalAiChatGenerationResult {
    LocalAiChatGenerationResult {
        local_ai_result_id: format!(
            "{SCHEDULER_UNAVAILABLE_RESULT_ID}:{}",
            runtime.runtime_reference_id
        ),
        runtime_reference_id: runtime.runtime_reference_id.clone(),
        provider_id: runtime.provider_id.clone(),
        model_id: runtime.model_id.clone(),
        model_reference: runtime.model_reference.clone(),
        generation_state: LocalAiGenerationState::Unavailable,
        output_text: None,
        prompt_char_count: 0,
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        duration_ms: 0,
        exit_code: None,
        stderr_byte_size: 0,
        unavailable_reason: runtime.unavailable_reason.clone(),
    }
}

fn degraded_generation_result(runtime: &LocalModelRuntimeStatus) -> LocalAiChatGenerationResult {
    LocalAiChatGenerationResult {
        local_ai_result_id: format!(
            "{SCHEDULER_DEGRADED_RESULT_ID}:{}",
            runtime.runtime_reference_id
        ),
        runtime_reference_id: runtime.runtime_reference_id.clone(),
        provider_id: runtime.provider_id.clone(),
        model_id: runtime.model_id.clone(),
        model_reference: runtime.model_reference.clone(),
        generation_state: LocalAiGenerationState::Failed,
        output_text: None,
        prompt_char_count: 0,
        max_output_tokens: constants::local_ai_runtime::DEFAULT_GENERATION_MAX_TOKENS,
        timeout_ms: constants::local_ai_runtime::DEFAULT_GENERATION_TIMEOUT_MS,
        duration_ms: 0,
        exit_code: None,
        stderr_byte_size: 0,
        unavailable_reason: Some(SCHEDULER_QUEUE_FULL_REASON.to_string()),
    }
}

fn degraded_state_for_generation(
    result: &LocalAiChatGenerationResult,
) -> Option<LocalAiDegradedState> {
    match result.generation_state {
        LocalAiGenerationState::Complete | LocalAiGenerationState::Running => None,
        LocalAiGenerationState::Unavailable => Some(LocalAiDegradedState::ProviderUnavailable),
        LocalAiGenerationState::TimedOut => Some(LocalAiDegradedState::Overloaded),
        LocalAiGenerationState::Failed => Some(LocalAiDegradedState::InvalidOutput),
    }
}

fn status_for_device<'a>(
    states: &'a mut HashMap<LocalAiPhysicalDeviceId, LocalAiProviderSchedulerStatus>,
    physical_device_id: LocalAiPhysicalDeviceId,
    runtime: &LocalModelRuntimeStatus,
) -> &'a mut LocalAiProviderSchedulerStatus {
    states.entry(physical_device_id.clone()).or_insert_with(|| {
        status_unavailable_for_device(
            physical_device_id,
            LocalAiTimestamp(runtime.last_checked_at.clone()),
        )
    })
}
