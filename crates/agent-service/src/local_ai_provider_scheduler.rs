use std::{future::Future, sync::OnceLock};

use ocentra_parent_agent_protocol::{
    constants, LocalAiChatGenerationResult, LocalAiDegradedState, LocalAiProviderSchedulerDecision,
    LocalAiProviderSchedulerJobClass, LocalAiProviderSchedulerJobStatus,
    LocalAiProviderSchedulerLifecycle, LocalAiProviderSchedulerQueue,
    LocalAiProviderSchedulerStatus, LocalModelRuntimeStatus,
};
use tokio::sync::Mutex;

use crate::{
    local_ai_provider_scheduler_state::{
        copy_runtime_fields, decision_for, decrement_queue, increment_queue, status_unavailable,
    },
    time::timestamp_now,
};

#[cfg(test)]
use crate::local_ai_provider_scheduler_state::take_next_queued_job;

static LOCAL_AI_PROVIDER_SCHEDULER: OnceLock<LocalAiProviderSchedulerRuntime> = OnceLock::new();

pub(crate) fn local_ai_provider_scheduler() -> &'static LocalAiProviderSchedulerRuntime {
    LOCAL_AI_PROVIDER_SCHEDULER.get_or_init(LocalAiProviderSchedulerRuntime::new)
}

pub(crate) struct LocalAiProviderSchedulerRuntime {
    lane: Mutex<()>,
    state: std::sync::Mutex<LocalAiProviderSchedulerStatus>,
}

impl LocalAiProviderSchedulerRuntime {
    pub(crate) fn new() -> Self {
        Self {
            lane: Mutex::new(()),
            state: std::sync::Mutex::new(status_unavailable(timestamp_now())),
        }
    }

    #[cfg(test)]
    pub(crate) fn new_for_test() -> Self {
        Self {
            lane: Mutex::new(()),
            state: std::sync::Mutex::new(status_unavailable(
                constants::local_ai_runtime::TEST_CHECKED_AT.to_string(),
            )),
        }
    }

    #[cfg(test)]
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

    #[cfg(test)]
    pub(crate) fn complete_current_job(
        &self,
        runtime: &LocalModelRuntimeStatus,
    ) -> LocalAiProviderSchedulerStatus {
        let mut status = self
            .state
            .lock()
            .expect(constants::error::AGENT_EVENT_SERIALIZES);
        if let Some(next_job_class) = take_next_queued_job(&mut status.queue) {
            status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Running;
            status.current_job_class = Some(next_job_class);
            status.degraded_state = if status.queue.total() > 0 {
                LocalAiDegradedState::Overloaded
            } else {
                LocalAiDegradedState::None
            };
        } else {
            status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Idle;
            status.current_job_class = None;
            status.degraded_state = LocalAiDegradedState::None;
        }
        status.unavailable_reason = None;
        copy_runtime_fields(&mut status, runtime);
        status.clone()
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

        if let Ok(guard) = self.lane.try_lock() {
            self.record_running_job(&runtime, job_class);
            let result = run().await;
            drop(guard);
            self.finish_runtime_lane(&runtime);
            return result;
        }

        self.record_queued_job(&runtime, job_class.clone());
        let guard = self.lane.lock().await;
        self.record_running_job(&runtime, job_class);
        let result = run().await;
        drop(guard);
        self.finish_runtime_lane(&runtime);
        result
    }

    fn finish_runtime_lane(&self, runtime: &LocalModelRuntimeStatus) {
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
