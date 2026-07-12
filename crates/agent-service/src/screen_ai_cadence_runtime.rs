use std::{
    env, fs,
    path::{Path, PathBuf},
    time::Duration,
};

use ocentra_parent_agent_protocol::activity_capture::ActivityCaptureCapabilityStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_ENABLED_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_CADENCE_ENABLED_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_CADENCE_MAX_CAPTURES_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_CADENCE_MAX_TICKS_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_CADENCE_RUNTIME_ENABLED_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_CADENCE_SECONDS_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_EVENT_ID_PREFIX;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_EVIDENCE_ID_PREFIX;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_MODEL_ID;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_QUEUE_DIR_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_QUEUE_JOB_ID_PREFIX;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_QUEUE_MAX_PENDING_DEFAULT;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_QUEUE_MAX_PENDING_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_RESULT_ID_PREFIX;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_SOURCE_ID;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_SUMMARY_CAPTURED;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_TEMPLATE_VERSION;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_DEFAULT;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_ENV;
use ocentra_parent_screen_capture_adapter::{
    capture_active_window_png,
    trigger_scheduler::{
        evaluate_screen_capture_schedule, ScreenCaptureScheduleDecision,
        ScreenCaptureScheduleTrigger, ScreenCaptureSchedulerSettings, ScreenCaptureSchedulerState,
        ScreenCaptureTriggerInput,
    },
    ScreenCaptureAttempt, ScreenCaptureScope,
};

use crate::{
    activity_capture::ActivityCaptureError,
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    screen_ai_cadence_runtime_event::{
        record_captured_screen_image_to_paths, ScreenAiServiceCaptureClock,
        ScreenAiServiceCapturePaths, ScreenAiServiceCaptureRecord,
    },
    screen_ai_service_event_bridge::publish_screen_capture_queue_events_for_queue_job,
    screen_ai_service_event_bridge::ScreenAiQueueJobId,
    screen_ai_service_event_subscription::ObservedAtText,
};

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

async fn run_screen_ai_cadence_runtime(config: ScreenAiCadenceRuntimeConfig) {
    runtime::run_screen_ai_cadence_runtime(config).await;
}

pub(crate) fn record_screen_ai_cadence_tick(
    config: &ScreenAiCadenceRuntimeConfig,
    state: ScreenCaptureSchedulerState,
    clock: ScreenAiCadenceTickClock,
    tick_index: u64,
) -> Result<ScreenAiCadenceTickOutcome, ActivityCaptureError> {
    runtime::record_screen_ai_cadence_tick(config, state, clock, tick_index)
}

impl ScreenAiCadenceRuntimeConfig {
    pub(crate) fn from_environment() -> Option<Self> {
        runtime::from_environment()
    }

    pub(crate) fn scheduler_settings(&self) -> ScreenCaptureSchedulerSettings {
        runtime::scheduler_settings(self)
    }
}
