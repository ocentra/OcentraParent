use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;

use crate::screen_ai_cadence_runtime_event::ScreenAiServiceCaptureClock;

#[path = "screen_ai_foreground_runtime/runtime.rs"]
mod runtime;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ScreenAiForegroundTickOutcome {
    Recorded {
        queue_job_id: String,
        foreground_key: String,
    },
    NoForegroundChange {
        foreground_key: String,
    },
    Suppressed,
    ForegroundUnavailable {
        status: ActivityCaptureCapabilityStatus,
    },
    CaptureUnavailable {
        status: ActivityCaptureCapabilityStatus,
    },
    QueueBackpressure {
        pending_count: u64,
        max_pending_queue_records: u64,
    },
}

pub(crate) type ScreenAiForegroundTickClock = ScreenAiServiceCaptureClock;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiForegroundQueueJobId(String);

impl std::fmt::Display for ScreenAiForegroundQueueJobId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(&self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiForegroundKey(pub(crate) String);

pub(crate) fn spawn_screen_ai_foreground_runtime() {
    runtime::spawn_screen_ai_foreground_runtime();
}
