use std::{future::Future, sync::OnceLock};

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
    },
    time::timestamp_now,
};

static LOCAL_AI_PROVIDER_SCHEDULER: OnceLock<LocalAiProviderSchedulerRuntime> = OnceLock::new();

pub(crate) fn local_ai_provider_scheduler() -> &'static LocalAiProviderSchedulerRuntime {
    LOCAL_AI_PROVIDER_SCHEDULER.get_or_init(LocalAiProviderSchedulerRuntime::new)
}

pub(crate) struct LocalAiProviderSchedulerRuntime {
    lane: Mutex<LocalAiProviderRuntimeLaneQueue>,
    state: std::sync::Mutex<LocalAiProviderSchedulerStatus>,
}

impl LocalAiProviderSchedulerRuntime {
    pub(crate) fn new() -> Self {
        Self {
            lane: Mutex::new(LocalAiProviderRuntimeLaneQueue::new()),
            state: std::sync::Mutex::new(status_unavailable(timestamp_now())),
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_test() -> Self {
        Self {
            lane: Mutex::new(LocalAiProviderRuntimeLaneQueue::new()),
            state: std::sync::Mutex::new(status_unavailable(
                constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
            )),
        }
    }

    pub(crate) fn status_snapshot(&self) -> LocalAiProviderSchedulerStatus {
        self.state
            .lock()
            .expect(constants::error::AGENT_EVENT_SERIALIZES)
            .clone()
    }

    pub(crate) fn record_queued_job(
        &self,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderSchedulerDecision {
        let mut status = self
            .state
            .lock()
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        increment_queue(&mut status.queue, &job_class);
        status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Queued;
        status.duplicate_runtime_blocked = true;
        status.degraded_state = LocalAiDegradedState::Overloaded;
        copy_runtime_fields(&mut status, runtime);
        decision_for(
            runtime,
            job_class,
            LocalAiProviderSchedulerJobStatus::Queued,
            Some(status.queue.total()),
            None,
            true,
        )
    }

    pub(crate) fn record_running_job(
        &self,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderSchedulerDecision {
        let mut status = self
            .state
            .lock()
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
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
        copy_runtime_fields(&mut status, runtime);
        decision_for(
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
        let reason = runtime
            .unavailable_reason
            .as_deref()
            .unwrap_or(constants::local_ai_runtime::UNAVAILABLE_REASON_UNCONFIGURED);
        let mut status = self
            .state
            .lock()
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Unavailable;
        status.current_job_class = None;
        status.queue = LocalAiProviderSchedulerQueue::default();
        status.duplicate_runtime_blocked = false;
        status.degraded_state = LocalAiDegradedState::ProviderUnavailable;
        status.unavailable_reason = Some(reason.to_string());
        copy_runtime_fields(&mut status, runtime);
        decision_for(
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
        if runtime.unavailable_reason.is_some() {
            self.record_unavailable_job(&runtime, job_class);
            return run().await;
        }

        match self.reserve_runtime_lane(job_class.clone()).await {
            LocalAiProviderRuntimeLaneAdmission::Running => {
                self.record_running_job(&runtime, job_class.clone());
            }
            LocalAiProviderRuntimeLaneAdmission::Queued(waiter) => {
                self.record_queued_job(&runtime, job_class.clone());
                waiter.notify_if_lane_idle();
                self.wait_for_runtime_lane(waiter, &runtime, job_class.clone())
                    .await;
            }
        }

        let result = run().await;
        self.finish_runtime_lane(&runtime).await;
        result
    }

    async fn reserve_runtime_lane(
        &self,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderRuntimeLaneAdmission {
        let mut lane = self.lane.lock().await;
        lane.reserve(job_class)
    }

    async fn wait_for_runtime_lane(
        &self,
        waiter: LocalAiProviderRuntimeLaneWaiter,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
    ) {
        loop {
            waiter.notify.notified().await;
            let admitted = {
                let mut lane = self.lane.lock().await;
                lane.try_admit_queued(&waiter)
            };
            if admitted {
                self.record_running_job(runtime, job_class);
                return;
            }
        }
    }

    async fn finish_runtime_lane(&self, runtime: &LocalModelRuntimeStatus) {
        let waiting_jobs = {
            let mut lane = self.lane.lock().await;
            let waiting_jobs = lane.finish_running();
            self.finish_runtime_lane_state(runtime);
            waiting_jobs
        };
        for waiting_job in waiting_jobs {
            waiting_job.notify_one();
        }
    }

    fn finish_runtime_lane_state(&self, runtime: &LocalModelRuntimeStatus) {
        let mut status = self
            .state
            .lock()
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        status.current_job_class = None;
        status.lifecycle_state = if status.queue.total() > 0 {
            LocalAiProviderSchedulerLifecycle::Queued
        } else {
            LocalAiProviderSchedulerLifecycle::Idle
        };
        status.degraded_state = if status.queue.total() > 0 {
            LocalAiDegradedState::Overloaded
        } else {
            LocalAiDegradedState::None
        };
        status.unavailable_reason = None;
        copy_runtime_fields(&mut status, runtime);
    }
}
