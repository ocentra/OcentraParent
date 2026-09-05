use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::lifecycle::LocalAiDegradedState;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::{
    LocalAiProviderSchedulerDecision, LocalAiProviderSchedulerJobClass,
    LocalAiProviderSchedulerJobStatus, LocalAiProviderSchedulerLifecycle,
};
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;

use crate::local_ai_provider_scheduler_queue::{
    LocalAiProviderRuntimeLaneAdmission, LocalAiProviderRuntimeLaneQueue,
    LocalAiProviderRuntimeLaneWaiter,
};
use crate::local_ai_provider_scheduler_state::{
    copy_runtime_fields, decision_for, decrement_queue, LocalAiPhysicalDeviceId, LocalAiStatusText,
};

use super::local_ai_provider_scheduler_lease::LocalAiProviderRuntimeLaneLease;
use super::local_ai_provider_scheduler_result::degraded_state_for_generation;
use super::{status_for_device, LocalAiProviderSchedulerRuntime};

impl LocalAiProviderSchedulerRuntime {
    pub(super) fn reserve_runtime_lane(
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

    pub(super) async fn queued_runtime_lane_lease(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        runtime: LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
        waiter: LocalAiProviderRuntimeLaneWaiter,
    ) -> LocalAiProviderRuntimeLaneLease<'_> {
        self.record_queued_job_for_device(physical_device_id.clone(), &runtime, job_class);
        waiter.notify_if_lane_idle();
        let lease = LocalAiProviderRuntimeLaneLease::queued(
            self,
            physical_device_id.clone(),
            runtime.clone(),
            job_class,
            waiter,
        );
        if let Some(waiter) = lease.waiter() {
            self.wait_for_runtime_lane(physical_device_id, waiter, &runtime, job_class)
                .await;
        }
        lease
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

    pub(super) fn finish_runtime_lane(
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

    pub(super) fn finish_runtime_lane_without_result(
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

    pub(super) fn cancel_queued_runtime_lane_job(
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
        let has_current_job = status.current_job_class.is_some();
        let has_queued_jobs = status.queue.total() > 0;
        status.lifecycle_state = cancelled_lifecycle(has_current_job, has_queued_jobs);
        status.duplicate_runtime_blocked = has_current_job || has_queued_jobs;
        status.degraded_state = if has_queued_jobs {
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
        let has_queued_jobs = status.queue.total() > 0;
        status.lifecycle_state =
            finished_lifecycle(has_queued_jobs, result_degraded_state.is_some());
        status.duplicate_runtime_blocked = has_queued_jobs;
        status.degraded_state = result_degraded_state.unwrap_or(if has_queued_jobs {
            LocalAiDegradedState::Overloaded
        } else {
            LocalAiDegradedState::None
        });
        status.unavailable_reason = result.and_then(|result| result.unavailable_reason.clone());
        copy_runtime_fields(status, runtime);
    }

    pub(super) fn record_degraded_job_for_device(
        &self,
        physical_device_id: LocalAiPhysicalDeviceId,
        runtime: &LocalModelRuntimeStatus,
        job_class: LocalAiProviderSchedulerJobClass,
        reason: LocalAiStatusText,
    ) -> LocalAiProviderSchedulerDecision {
        let mut states = self
            .states
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        let status = status_for_device(&mut states, physical_device_id.clone(), runtime);
        status.lifecycle_state = LocalAiProviderSchedulerLifecycle::Degraded;
        status.duplicate_runtime_blocked = true;
        status.degraded_state = LocalAiDegradedState::Overloaded;
        status.unavailable_reason = Some(reason.0.clone());
        copy_runtime_fields(status, runtime);
        decision_for(
            physical_device_id,
            runtime,
            job_class,
            LocalAiProviderSchedulerJobStatus::Degraded,
            Some(status.queue.total()),
            Some(reason),
            true,
        )
    }
}

fn cancelled_lifecycle(
    has_current_job: bool,
    has_queued_jobs: bool,
) -> LocalAiProviderSchedulerLifecycle {
    [
        (has_current_job, LocalAiProviderSchedulerLifecycle::Running),
        (
            !has_current_job && has_queued_jobs,
            LocalAiProviderSchedulerLifecycle::Queued,
        ),
    ]
    .into_iter()
    .find_map(|(selected, state)| selected.then_some(state))
    .unwrap_or(LocalAiProviderSchedulerLifecycle::Idle)
}

fn finished_lifecycle(
    has_queued_jobs: bool,
    has_degraded_result: bool,
) -> LocalAiProviderSchedulerLifecycle {
    [
        (
            has_degraded_result,
            LocalAiProviderSchedulerLifecycle::Degraded,
        ),
        (has_queued_jobs, LocalAiProviderSchedulerLifecycle::Queued),
    ]
    .into_iter()
    .find_map(|(selected, state)| selected.then_some(state))
    .unwrap_or(LocalAiProviderSchedulerLifecycle::Idle)
}
