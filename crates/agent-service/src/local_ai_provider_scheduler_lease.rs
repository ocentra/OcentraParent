use ocentra_parent_agent_protocol::local_ai_runtime::generation::LocalAiChatGenerationResult;
use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobClass;
use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;

use crate::local_ai_provider_scheduler_queue::LocalAiProviderRuntimeLaneWaiter;
use crate::local_ai_provider_scheduler_state::LocalAiPhysicalDeviceId;

use super::LocalAiProviderSchedulerRuntime;

pub(super) struct LocalAiProviderRuntimeLaneLease<'a> {
    scheduler: &'a LocalAiProviderSchedulerRuntime,
    physical_device_id: LocalAiPhysicalDeviceId,
    runtime: LocalModelRuntimeStatus,
    job_class: LocalAiProviderSchedulerJobClass,
    waiter: Option<LocalAiProviderRuntimeLaneWaiter>,
    finished: bool,
}

impl<'a> LocalAiProviderRuntimeLaneLease<'a> {
    pub(super) fn running(
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

    pub(super) fn queued(
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

    pub(super) fn waiter(&self) -> Option<&LocalAiProviderRuntimeLaneWaiter> {
        self.waiter.as_ref()
    }

    pub(super) fn finish(&mut self, result: &LocalAiChatGenerationResult) {
        if self.finished {
            return;
        }
        self.scheduler
            .finish_runtime_lane(self.physical_device_id.clone(), &self.runtime, result);
        self.finished = true;
    }
}

impl Drop for LocalAiProviderRuntimeLaneLease<'_> {
    fn drop(&mut self) {
        if self.finished {
            return;
        }
        let Some(waiter) = self.waiter.as_ref() else {
            self.scheduler
                .finish_runtime_lane_without_result(self.physical_device_id.clone(), &self.runtime);
            return;
        };
        if waiter.is_admitted() {
            self.scheduler
                .finish_runtime_lane_without_result(self.physical_device_id.clone(), &self.runtime);
            return;
        }
        self.scheduler.cancel_queued_runtime_lane_job(
            self.physical_device_id.clone(),
            &self.runtime,
            self.job_class,
            waiter,
        );
    }
}
