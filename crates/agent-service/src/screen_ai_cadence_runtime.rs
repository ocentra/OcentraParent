use std::{
    env, fs,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, SCREEN_SERVICE_ANALYSIS_ENABLED_ENV,
    SCREEN_SERVICE_CADENCE_ENABLED_ENV, SCREEN_SERVICE_CADENCE_MAX_CAPTURES_ENV,
    SCREEN_SERVICE_CADENCE_MAX_TICKS_ENV, SCREEN_SERVICE_CADENCE_RUNTIME_ENABLED_ENV,
    SCREEN_SERVICE_CADENCE_SECONDS_ENV, SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME,
    SCREEN_SERVICE_QUEUE_DIR_ENV, SCREEN_SERVICE_QUEUE_MAX_PENDING_DEFAULT,
    SCREEN_SERVICE_QUEUE_MAX_PENDING_ENV,
};
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
    screen_ai_cadence_runtime_event::record_captured_screen_image_to_paths,
    time::timestamp_now,
};

const DEFAULT_CADENCE_SECONDS: u64 = 60;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiCadenceRuntimeConfig {
    pub(crate) screen_analysis_enabled: bool,
    pub(crate) cadence_capture_enabled: bool,
    pub(crate) cadence_seconds: u64,
    pub(crate) max_captures: Option<u64>,
    pub(crate) max_ticks: Option<u64>,
    pub(crate) max_pending_queue_records: u64,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiCadenceTickClock {
    pub(crate) epoch_seconds: u64,
    pub(crate) timestamp: String,
}

pub(crate) fn spawn_screen_ai_cadence_runtime() {
    if let Some(config) = ScreenAiCadenceRuntimeConfig::from_environment() {
        tokio::spawn(async move {
            run_screen_ai_cadence_runtime(config).await;
        });
    }
}

async fn run_screen_ai_cadence_runtime(config: ScreenAiCadenceRuntimeConfig) {
    let mut interval = tokio::time::interval(Duration::from_secs(config.cadence_seconds));
    let mut state = ScreenCaptureSchedulerState {
        last_capture_at_epoch_seconds: None,
    };
    let mut capture_count = 0;
    let mut tick_count = 0;
    loop {
        interval.tick().await;
        tick_count += 1;
        let clock = ScreenAiCadenceTickClock::from_system_time();
        let epoch_seconds = clock.epoch_seconds;
        let outcome = record_screen_ai_cadence_tick(&config, state, clock, tick_count);
        if let Ok(ScreenAiCadenceTickOutcome::Recorded { .. }) = outcome {
            state.last_capture_at_epoch_seconds = Some(epoch_seconds);
            capture_count += 1;
        }
        if config.max_captures.is_some_and(|max| capture_count >= max) {
            break;
        }
        if config.max_ticks.is_some_and(|max| tick_count >= max) {
            break;
        }
    }
}

pub(crate) fn record_screen_ai_cadence_tick(
    config: &ScreenAiCadenceRuntimeConfig,
    state: ScreenCaptureSchedulerState,
    clock: ScreenAiCadenceTickClock,
    tick_index: u64,
) -> Result<ScreenAiCadenceTickOutcome, ActivityCaptureError> {
    let decision = evaluate_screen_capture_schedule(
        &config.scheduler_settings(),
        state,
        ScreenCaptureTriggerInput {
            observed_at_epoch_seconds: clock.epoch_seconds,
            trigger: ScreenCaptureScheduleTrigger::TimedCadence,
            requested_scope: Some(ScreenCaptureScope::ActiveWindow),
        },
    );
    if !matches!(
        decision,
        ScreenCaptureScheduleDecision::EnqueueCapture { .. }
    ) {
        return Ok(ScreenAiCadenceTickOutcome::Suppressed);
    }
    let pending_count = pending_queue_record_count(&config.queue_dir)?;
    if pending_count >= config.max_pending_queue_records {
        return Ok(ScreenAiCadenceTickOutcome::QueueBackpressure {
            pending_count,
            max_pending_queue_records: config.max_pending_queue_records,
        });
    }

    match capture_active_window_png() {
        ScreenCaptureAttempt::Captured(image) => {
            let queue_job_id =
                record_captured_screen_image_to_paths(config, &image, clock, tick_index)?;
            Ok(ScreenAiCadenceTickOutcome::Recorded { queue_job_id })
        }
        ScreenCaptureAttempt::Degraded(metadata) => {
            Ok(ScreenAiCadenceTickOutcome::CaptureUnavailable {
                status: metadata.status,
            })
        }
    }
}

impl ScreenAiCadenceRuntimeConfig {
    pub(crate) fn from_environment() -> Option<Self> {
        if !env_flag(SCREEN_SERVICE_CADENCE_RUNTIME_ENABLED_ENV, false) {
            return None;
        }
        Some(Self {
            screen_analysis_enabled: env_flag(SCREEN_SERVICE_ANALYSIS_ENABLED_ENV, true),
            cadence_capture_enabled: env_flag(SCREEN_SERVICE_CADENCE_ENABLED_ENV, true),
            cadence_seconds: env_u64(SCREEN_SERVICE_CADENCE_SECONDS_ENV, DEFAULT_CADENCE_SECONDS),
            max_captures: env_optional_u64(SCREEN_SERVICE_CADENCE_MAX_CAPTURES_ENV),
            max_ticks: env_optional_u64(SCREEN_SERVICE_CADENCE_MAX_TICKS_ENV),
            max_pending_queue_records: env_u64(
                SCREEN_SERVICE_QUEUE_MAX_PENDING_ENV,
                SCREEN_SERVICE_QUEUE_MAX_PENDING_DEFAULT,
            ),
            queue_dir: env_path(SCREEN_SERVICE_QUEUE_DIR_ENV).unwrap_or_else(default_queue_dir),
            journal_path: activity_journal_path(),
            journal_key_path: activity_journal_key_path(),
            store_path: activity_db_path(),
        })
    }

    pub(crate) fn scheduler_settings(&self) -> ScreenCaptureSchedulerSettings {
        ScreenCaptureSchedulerSettings {
            screen_analysis_enabled: self.screen_analysis_enabled,
            trigger_capture_enabled: true,
            cadence_capture_enabled: self.cadence_capture_enabled,
            allowed_scope: ScreenCaptureScope::ActiveWindow,
            cadence_seconds: self.cadence_seconds,
            min_trigger_gap_seconds: self.cadence_seconds,
            enabled_triggers: &[ScreenCaptureScheduleTrigger::TimedCadence],
        }
    }
}

impl ScreenAiCadenceTickClock {
    fn from_system_time() -> Self {
        Self {
            epoch_seconds: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|duration| duration.as_secs())
                .unwrap_or_default(),
            timestamp: timestamp_now(),
        }
    }
}

fn env_flag(name: &str, default_value: bool) -> bool {
    env::var(name)
        .ok()
        .map(|value| value == constants::value::TRUE)
        .unwrap_or(default_value)
}

fn env_u64(name: &str, default_value: u64) -> u64 {
    env::var(name)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_value)
}

fn env_optional_u64(name: &str) -> Option<u64> {
    env::var(name)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
}

fn env_path(name: &str) -> Option<PathBuf> {
    env::var(name)
        .ok()
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn pending_queue_record_count(queue_dir: &Path) -> Result<u64, ActivityCaptureError> {
    let path = queue_dir.join(constants::activity_store::SCREEN_EVIDENCE_QUEUE_FILE_NAME);
    match fs::read_to_string(path) {
        Ok(contents) => Ok(contents
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count() as u64),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(0),
        Err(error) => Err(error.into()),
    }
}

fn default_queue_dir() -> PathBuf {
    let mut path = env::temp_dir();
    path.push(SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME);
    path
}
