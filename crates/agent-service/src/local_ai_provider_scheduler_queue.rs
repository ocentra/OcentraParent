use std::sync::Arc;

use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobClass;
use tokio::sync::Notify;

pub(crate) enum LocalAiProviderRuntimeLaneAdmission {
    Running,
    Queued(LocalAiProviderRuntimeLaneWaiter),
}

pub(crate) struct LocalAiProviderRuntimeLaneQueue {
    running: bool,
    next_sequence: u64,
    waiters: Vec<LocalAiProviderQueuedRuntimeJob>,
}

pub(crate) struct LocalAiProviderRuntimeLaneWaiter {
    sequence: u64,
    lane_idle_on_queue: bool,
    pub(crate) notify: Arc<Notify>,
}

struct LocalAiProviderQueuedRuntimeJob {
    sequence: u64,
    job_class: LocalAiProviderSchedulerJobClass,
    notify: Arc<Notify>,
}

impl LocalAiProviderRuntimeLaneQueue {
    pub(crate) fn new() -> Self {
        Self {
            running: false,
            next_sequence: 0,
            waiters: Vec::new(),
        }
    }

    pub(crate) fn reserve(
        &mut self,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderRuntimeLaneAdmission {
        if !self.running && self.waiters.is_empty() {
            self.running = true;
            return LocalAiProviderRuntimeLaneAdmission::Running;
        }

        let lane_idle_on_queue = !self.running;
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        let notify = Arc::new(Notify::new());
        self.waiters.push(LocalAiProviderQueuedRuntimeJob {
            sequence,
            job_class,
            notify: Arc::clone(&notify),
        });

        LocalAiProviderRuntimeLaneAdmission::Queued(LocalAiProviderRuntimeLaneWaiter {
            sequence,
            lane_idle_on_queue,
            notify,
        })
    }

    pub(crate) fn try_admit_queued(&mut self, waiter: &LocalAiProviderRuntimeLaneWaiter) -> bool {
        if self.running {
            return false;
        }

        let Some(next_index) = self.next_waiter_index() else {
            return false;
        };

        if self.waiters[next_index].sequence != waiter.sequence {
            return false;
        }

        self.waiters.swap_remove(next_index);
        self.running = true;
        true
    }

    pub(crate) fn finish_running(&mut self) -> Vec<Arc<Notify>> {
        self.running = false;
        self.waiters
            .iter()
            .map(|waiter| Arc::clone(&waiter.notify))
            .collect()
    }

    fn next_waiter_index(&self) -> Option<usize> {
        self.waiters
            .iter()
            .enumerate()
            .min_by_key(|(_, waiter)| (job_class_priority(&waiter.job_class), waiter.sequence))
            .map(|(index, _)| index)
    }
}

impl LocalAiProviderRuntimeLaneWaiter {
    pub(crate) fn notify_if_lane_idle(&self) {
        if self.lane_idle_on_queue {
            self.notify.notify_one();
        }
    }
}

fn job_class_priority(job_class: &LocalAiProviderSchedulerJobClass) -> u8 {
    match job_class {
        LocalAiProviderSchedulerJobClass::ChildSafety => 0,
        LocalAiProviderSchedulerJobClass::ParentAssistant => 1,
        LocalAiProviderSchedulerJobClass::ParentReport => 2,
    }
}
