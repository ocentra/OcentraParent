use std::{
    collections::HashMap,
    future::Future,
    sync::{Mutex, OnceLock},
};

use crate::{
    local_ai_provider_scheduler_queue::{
        LocalAiProviderRuntimeLaneAdmission, LocalAiProviderRuntimeLaneQueue,
    },
    local_ai_provider_scheduler_state::{
        copy_runtime_fields, decision_for, decrement_queue, increment_queue, status_unavailable,
        status_unavailable_for_device, LocalAiPhysicalDeviceId, LocalAiStatusText,
        LocalAiTimestamp,
    },
    time::timestamp_now,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerDecision;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobClass;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerLifecycle;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerQueue;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerStatus;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;

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

#[path = "local_ai_provider_scheduler_lane.rs"]
mod local_ai_provider_scheduler_lane;
#[path = "local_ai_provider_scheduler_lease.rs"]
mod local_ai_provider_scheduler_lease;
#[path = "local_ai_provider_scheduler_result.rs"]
mod local_ai_provider_scheduler_result;

use local_ai_provider_scheduler_lease::LocalAiProviderRuntimeLaneLease;
use local_ai_provider_scheduler_result::{
    degraded_generation_result, runtime_unavailable_reason, runtime_with_reason,
    unavailable_generation_result,
};

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
                self.record_running_job_for_device(physical_device_id.clone(), &runtime, job_class);
                LocalAiProviderRuntimeLaneLease::running(
                    self,
                    physical_device_id,
                    runtime.clone(),
                    job_class,
                )
            }
            LocalAiProviderRuntimeLaneAdmission::Queued(waiter) => {
                self.queued_runtime_lane_lease(
                    physical_device_id,
                    runtime.clone(),
                    job_class,
                    waiter,
                )
                .await
            }
            LocalAiProviderRuntimeLaneAdmission::Rejected => {
                self.record_degraded_job_for_device(
                    physical_device_id,
                    &runtime,
                    job_class,
                    LocalAiStatusText(SCHEDULER_QUEUE_FULL_REASON.to_string()),
                );
                return degraded_generation_result(&runtime);
            }
        };

        let result = run().await;
        lease.finish(&result);
        result
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
