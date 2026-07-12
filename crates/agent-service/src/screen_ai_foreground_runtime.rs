use std::time::Duration;

use ocentra_parent_agent_core::window_capture::collect_foreground_window_observation;
use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_FOREGROUND_EVENT_ID_PREFIX;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_FOREGROUND_EVIDENCE_ID_PREFIX;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_FOREGROUND_MODEL_ID;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_FOREGROUND_QUEUE_JOB_ID_PREFIX;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_FOREGROUND_RESULT_ID_PREFIX;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_FOREGROUND_SOURCE_ID;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_FOREGROUND_SUMMARY_CAPTURED;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_FOREGROUND_TEMPLATE_VERSION;
use ocentra_parent_screen_capture_adapter::{
    capture_active_window_png,
    trigger_scheduler::{
        evaluate_screen_capture_schedule, ScreenCaptureScheduleDecision,
        ScreenCaptureScheduleTrigger, ScreenCaptureSchedulerState, ScreenCaptureTriggerInput,
    },
    CapturedScreenImage, ScreenCaptureAttempt, ScreenCaptureScope,
};

use crate::{
    activity_capture::ActivityCaptureError,
    screen_ai_cadence_runtime_event::{
        record_captured_screen_image_to_paths, ScreenAiServiceCaptureClock,
        ScreenAiServiceCapturePaths, ScreenAiServiceCaptureRecord,
    },
    screen_ai_foreground_runtime_config::{
        foreground_key, pending_queue_record_count, ScreenAiForegroundRuntimeConfig,
    },
    screen_ai_service_event_bridge::publish_screen_capture_queue_events_for_queue_job,
    screen_ai_service_event_bridge::ScreenAiQueueJobId,
    screen_ai_service_event_subscription::ObservedAtText,
};

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

async fn run_screen_ai_foreground_runtime(config: ScreenAiForegroundRuntimeConfig) {
    runtime::run_screen_ai_foreground_runtime(config).await;
}

pub(crate) fn record_screen_ai_foreground_captured_image(
    config: &ScreenAiForegroundRuntimeConfig,
    image: &CapturedScreenImage,
    clock: ScreenAiForegroundTickClock,
    sequence_index: u64,
) -> Result<ScreenAiForegroundQueueJobId, ActivityCaptureError> {
    runtime::record_screen_ai_foreground_captured_image(config, image, clock, sequence_index)
}
