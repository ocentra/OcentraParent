use std::{collections::HashMap, future::Future, sync::OnceLock};

use ocentra_parent_agent_protocol::{
    constants, LocalAiChatGenerationResult, LocalAiDegradedState, LocalAiProviderSchedulerDecision,
    LocalAiProviderSchedulerJobClass, LocalAiProviderSchedulerJobStatus,
    LocalAiProviderSchedulerLifecycle, LocalAiProviderSchedulerQueue,
    LocalAiProviderSchedulerStatus, LocalModelRuntimeStatus,
};
use tokio::sync::Mutex;

use crate::{
    local_ai_provider_scheduler_queue::{
        LocalAiProviderRuntimeLaneAdmission, LocalAiProviderRuntimeLaneQueue,
        LocalAiProviderRuntimeLaneWaiter,
    },
    local_ai_provider_scheduler_state::{
        copy_runtime_fields, decision_for, decrement_queue, increment_queue, status_unavailable,
        status_unavailable_for_device,
    },
    time::timestamp_now,
};

static LOCAL_AI_PROVIDER_SCHEDULER: OnceLock<LocalAiProviderSchedulerRuntime> = OnceLock::new();

pub(crate) fn local_ai_provider_scheduler() -> &'static LocalAiProviderSchedulerRuntime {
    LOCAL_AI_PROVIDER_SCHEDULER.get_or_init(LocalAiProviderSchedulerRuntime::new)
}

pub(crate) struct LocalAiProviderSchedulerRuntime {
    lanes: Mutex<HashMap<String, LocalAiProviderRuntimeLaneQueue>>,
    states: std::sync::Mutex<HashMap<String, LocalAiProviderSchedulerStatus>>,
}

impl LocalAiProviderSchedulerRuntime {
    pub(crate) fn new() -> Self {
        Self {
            lanes: Mutex::new(HashMap::new()),
            states: std::sync::Mutex::new(HashMap::from([(
                constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
                status_unavailable(timestamp_now()),
            )])),
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_test() -> Self {
        Self {
            lanes: Mutex::new(HashMap::new()),
            states: std::sync::Mutex::new(HashMap::from([(
                constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL.to_string(),
                status_unavailable(constants::local_ai_runtime::TEST_CHECKED_AT.to_string()),
            )])),
        }
    }

    pub(crate) fn status_snapshot(&self) -> LocalAiProviderSchedulerStatus {
        self.status_snapshot_for_device(constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL)
    }

    pub(crate) fn status_snapshot_for_device(
        &self,
        physical_device_id: &str,
    ) -> LocalAiProviderSchedulerStatus {
        self.states
            .lock()
            .expect(constants::error::AGENT_EVENT_SERIALIZES)
            .get(physical_device_id)
            .cloned()
            .unwrap_or_else(|| status_unavailable_for_device(physical_device_id, timestamp_now()))
    }

    pub(crate) fn record_queued_job(
        &self,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderSchedulerDecision {
        self.record_queued_job_for_device(
            constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL,
            runtime,
            job_class,
        )
    }

    pub(crate) fn record_queued_job_for_device(
        &self,
        physical_device_id: &str,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderSchedulerDecision {
        let mut states = self
            .states
            .lock()
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        let status = status_for_device(&mut states, physical_device_id, runtime);
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

    #[cfg(test)]
    pub(crate) fn record_running_job(
        &self,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderSchedulerDecision {
        self.record_running_job_for_device(
            constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL,
            runtime,
            job_class,
        )
    }

    pub(crate) fn record_running_job_for_device(
        &self,
        physical_device_id: &str,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderSchedulerDecision {
        let mut states = self
            .states
            .lock()
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        let status = status_for_device(&mut states, physical_device_id, runtime);
        decrement_queue(&mut status.queue, &job_class);
        status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Running;
        status.current_job_class = Some(job_class.clone());
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
            constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL,
            runtime,
            job_class,
        )
    }

    pub(crate) fn record_unavailable_job_for_device(
        &self,
        physical_device_id: &str,
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
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        let status = status_for_device(&mut states, physical_device_id, runtime);
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
            Some(reason),
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
            constants::local_ai_runtime::PHYSICAL_DEVICE_LOCAL,
            job_class,
            runtime,
            run,
        )
        .await
    }

    pub(crate) async fn run_generation_job_for_device<F, Fut>(
        &self,
        physical_device_id: &str,
        job_class: LocalAiProviderSchedulerJobClass,
        runtime: LocalModelRuntimeStatus,
        run: F,
    ) -> LocalAiChatGenerationResult
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = LocalAiChatGenerationResult>,
    {
        if runtime.unavailable_reason.is_some() {
            self.record_unavailable_job_for_device(physical_device_id, &runtime, job_class);
            return run().await;
        }

        match self
            .reserve_runtime_lane(physical_device_id, job_class.clone())
            .await
        {
            LocalAiProviderRuntimeLaneAdmission::Running => {
                self.record_running_job_for_device(physical_device_id, &runtime, job_class.clone());
            }
            LocalAiProviderRuntimeLaneAdmission::Queued(waiter) => {
                self.record_queued_job_for_device(physical_device_id, &runtime, job_class.clone());
                waiter.notify_if_lane_idle();
                self.wait_for_runtime_lane(physical_device_id, waiter, &runtime, job_class.clone())
                    .await;
            }
        }

        let result = run().await;
        self.finish_runtime_lane(physical_device_id, &runtime).await;
        result
    }

    async fn reserve_runtime_lane(
        &self,
        physical_device_id: &str,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderRuntimeLaneAdmission {
        let mut lanes = self.lanes.lock().await;
        lanes
            .entry(physical_device_id.to_string())
            .or_insert_with(LocalAiProviderRuntimeLaneQueue::new)
            .reserve(job_class)
    }

    async fn wait_for_runtime_lane(
        &self,
        physical_device_id: &str,
        waiter: LocalAiProviderRuntimeLaneWaiter,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) {
        loop {
            waiter.notify.notified().await;
            let admitted = {
                let mut lanes = self.lanes.lock().await;
                lanes
                    .entry(physical_device_id.to_string())
                    .or_insert_with(LocalAiProviderRuntimeLaneQueue::new)
                    .try_admit_queued(&waiter)
            };
            if admitted {
                self.record_running_job_for_device(physical_device_id, runtime, job_class);
                return;
            }
        }
    }

    async fn finish_runtime_lane(
        &self,
        physical_device_id: &str,
        runtime: &LocalModelRuntimeStatus,
    ) {
        let waiting_jobs = {
            let mut lanes = self.lanes.lock().await;
            let waiting_jobs = lanes
                .entry(physical_device_id.to_string())
                .or_insert_with(LocalAiProviderRuntimeLaneQueue::new)
                .finish_running();
            self.finish_runtime_lane_state(physical_device_id, runtime);
            waiting_jobs
        };
        for waiting_job in waiting_jobs {
            waiting_job.notify_one();
        }
    }

    fn finish_runtime_lane_state(
        &self,
        physical_device_id: &str,
        runtime: &LocalModelRuntimeStatus,
    ) {
        let mut states = self
            .states
            .lock()
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        let status = status_for_device(&mut states, physical_device_id, runtime);
        status.current_job_class = None;
        status.lifecycle_state = if status.queue.total() > 0 {
            LocalAiProviderSchedulerLifecycle::Queued
        } else {
            LocalAiProviderSchedulerLifecycle::Idle
        };
        status.duplicate_runtime_blocked = status.queue.total() > 0;
        status.degraded_state = if status.queue.total() > 0 {
            LocalAiDegradedState::Overloaded
        } else {
            LocalAiDegradedState::None
        };
        status.unavailable_reason = None;
        copy_runtime_fields(status, runtime);
    }
}

fn status_for_device<'a>(
    states: &'a mut HashMap<String, LocalAiProviderSchedulerStatus>,
    physical_device_id: &str,
    runtime: &LocalModelRuntimeStatus,
) -> &'a mut LocalAiProviderSchedulerStatus {
    states
        .entry(physical_device_id.to_string())
        .or_insert_with(|| {
            status_unavailable_for_device(physical_device_id, runtime.last_checked_at.clone())
        })
}
