use std::path::PathBuf;

use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;

use crate::screen_ai_cadence_runtime_event::ScreenAiServiceCaptureClock;

#[path = "screen_ai_cadence_runtime/runtime.rs"]
mod runtime;

const DEFAULT_CADENCE_SECONDS: u64 = 60;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiCadenceRuntimeConfig {
    pub(crate) screen_analysis_enabled: bool,
    pub(crate) cadence_capture_enabled: bool,
    pub(crate) cadence_seconds: u64,
    pub(crate) max_captures: Option<u64>,
    pub(crate) max_ticks: Option<u64>,
    pub(crate) max_pending_queue_records: u64,
    pub(crate) temporary_image_ttl_seconds: u64,
    pub(crate) queue_dir: PathBuf,
    pub(crate) journal_path: PathBuf,
    pub(crate) journal_key_path: PathBuf,
    pub(crate) store_path: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ScreenAiCadenceTickOutcome {
    Recorded {
        queue_job_id: String,
    },
    Suppressed,
    CaptureUnavailable {
        status: ActivityCaptureCapabilityStatus,
    },
    QueueBackpressure {
        pending_count: u64,
        max_pending_queue_records: u64,
    },
}

pub(crate) type ScreenAiCadenceTickClock = ScreenAiServiceCaptureClock;

pub(crate) fn spawn_screen_ai_cadence_runtime() {
    runtime::spawn_screen_ai_cadence_runtime();
}
