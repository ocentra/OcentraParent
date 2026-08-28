use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use ocentra_parent_agent_protocol::local_ai_runtime::scheduler::LocalAiProviderSchedulerJobClass;
use tokio::sync::Notify;

pub(crate) enum LocalAiProviderRuntimeLaneAdmission {
    Running,
    Queued(LocalAiProviderRuntimeLaneWaiter),
    Rejected,
}

pub(crate) const MAX_PENDING_RUNTIME_JOBS: usize = 32;
const MAX_PRIORITY_STREAK_BEFORE_BACKGROUND: u8 = 4;

pub(crate) struct LocalAiProviderRuntimeLaneQueue {
    running: bool,
    next_sequence: u64,
    priority_streak: u8,
    waiters: Vec<LocalAiProviderQueuedRuntimeJob>,
}

pub(crate) struct LocalAiProviderRuntimeLaneWaiter {
    sequence: u64,
    lane_idle_on_queue: bool,
    pub(crate) notify: Arc<Notify>,
    cancelled: Arc<AtomicBool>,
    admitted: Arc<AtomicBool>,
}

struct LocalAiProviderQueuedRuntimeJob {
    sequence: u64,
    job_class: LocalAiProviderSchedulerJobClass,
    notify: Arc<Notify>,
    cancelled: Arc<AtomicBool>,
}

impl LocalAiProviderRuntimeLaneQueue {
    pub(crate) fn new() -> Self {
        Self {
            running: false,
            next_sequence: 0,
            priority_streak: 0,
            waiters: Vec::new(),
        }
    }

    pub(crate) fn reserve(
        &mut self,
        job_class: LocalAiProviderSchedulerJobClass,
    ) -> LocalAiProviderRuntimeLaneAdmission {
        self.remove_cancelled_waiters();
        if !self.running && self.waiters.is_empty() {
            self.running = true;
            return LocalAiProviderRuntimeLaneAdmission::Running;
        }

        if self.waiters.len() >= MAX_PENDING_RUNTIME_JOBS {
            return LocalAiProviderRuntimeLaneAdmission::Rejected;
        }

        let lane_idle_on_queue = !self.running;
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        let notify = Arc::new(Notify::new());
        let cancelled = Arc::new(AtomicBool::new(false));
        let admitted = Arc::new(AtomicBool::new(false));
        self.waiters.push(LocalAiProviderQueuedRuntimeJob {
            sequence,
            job_class,
            notify: Arc::clone(&notify),
            cancelled: Arc::clone(&cancelled),
        });

        LocalAiProviderRuntimeLaneAdmission::Queued(LocalAiProviderRuntimeLaneWaiter {
            sequence,
            lane_idle_on_queue,
            notify,
            cancelled,
            admitted,
        })
    }

    pub(crate) fn try_admit_queued(&mut self, waiter: &LocalAiProviderRuntimeLaneWaiter) -> bool {
        self.remove_cancelled_waiters();
        if self.running {
            return false;
        }

        let Some(next_index) = self.next_waiter_index() else {
            return false;
        };

        if self.waiters[next_index].sequence != waiter.sequence {
            return false;
        }

        let admitted_job = self.waiters.swap_remove(next_index);
        self.running = true;
        waiter.admitted.store(true, Ordering::Release);
        self.priority_streak = if is_background_job(&admitted_job.job_class) {
            0
        } else {
            self.priority_streak.saturating_add(1)
        };
        true
    }

    pub(crate) fn finish_running(&mut self) -> Vec<Arc<Notify>> {
        self.running = false;
        self.remove_cancelled_waiters();
        self.waiters
            .iter()
            .map(|waiter| Arc::clone(&waiter.notify))
            .collect()
    }

    fn next_waiter_index(&self) -> Option<usize> {
        let candidates = if self.priority_streak >= MAX_PRIORITY_STREAK_BEFORE_BACKGROUND {
            let background = self
                .waiters
                .iter()
                .enumerate()
                .filter(|(_, waiter)| is_background_job(&waiter.job_class))
                .min_by_key(|(_, waiter)| waiter.sequence);
            if background.is_some() {
                return background.map(|(index, _)| index);
            }
            self.waiters.iter().enumerate()
        } else {
            self.waiters.iter().enumerate()
        };

        candidates
            .min_by_key(|(_, waiter)| (job_class_priority(&waiter.job_class), waiter.sequence))
            .map(|(index, _)| index)
    }

    fn remove_cancelled_waiters(&mut self) {
        self.waiters
            .retain(|waiter| !waiter.cancelled.load(Ordering::Acquire));
    }
}

impl LocalAiProviderRuntimeLaneWaiter {
    pub(crate) fn notify_if_lane_idle(&self) {
        if self.lane_idle_on_queue {
            self.notify.notify_one();
        }
    }
}

impl Drop for LocalAiProviderRuntimeLaneWaiter {
    fn drop(&mut self) {
        if !self.admitted.load(Ordering::Acquire) {
            self.cancelled.store(true, Ordering::Release);
            self.notify.notify_waiters();
        }
    }
}

fn is_background_job(job_class: &LocalAiProviderSchedulerJobClass) -> bool {
    matches!(job_class, LocalAiProviderSchedulerJobClass::ParentReport)
}

fn job_class_priority(job_class: &LocalAiProviderSchedulerJobClass) -> u8 {
    match job_class {
        LocalAiProviderSchedulerJobClass::ChildSafety => 0,
        LocalAiProviderSchedulerJobClass::ParentAssistant => 1,
        LocalAiProviderSchedulerJobClass::ParentReport => 2,
    }
}
