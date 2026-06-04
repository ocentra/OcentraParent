use std::{
    env, fs,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_core::ForegroundWindowObservation;
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, SCREEN_SERVICE_ANALYSIS_ENABLED_ENV,
    SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME, SCREEN_SERVICE_FOREGROUND_ENABLED_ENV,
    SCREEN_SERVICE_FOREGROUND_KEY_APP_PREFIX, SCREEN_SERVICE_FOREGROUND_KEY_PID_PREFIX,
    SCREEN_SERVICE_FOREGROUND_KEY_TITLE_PREFIX, SCREEN_SERVICE_FOREGROUND_KEY_WINDOW_PREFIX,
    SCREEN_SERVICE_FOREGROUND_MAX_CAPTURES_ENV, SCREEN_SERVICE_FOREGROUND_MAX_TICKS_ENV,
    SCREEN_SERVICE_FOREGROUND_MIN_GAP_SECONDS_ENV, SCREEN_SERVICE_FOREGROUND_POLL_SECONDS_ENV,
    SCREEN_SERVICE_FOREGROUND_RUNTIME_ENABLED_ENV, SCREEN_SERVICE_QUEUE_DIR_ENV,
    SCREEN_SERVICE_QUEUE_MAX_PENDING_DEFAULT, SCREEN_SERVICE_QUEUE_MAX_PENDING_ENV,
    SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_DEFAULT,
    SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_ENV,
};
use ocentra_parent_screen_capture_adapter::{
    trigger_scheduler::{ScreenCaptureScheduleTrigger, ScreenCaptureSchedulerSettings},
    ScreenCaptureScope,
};

use crate::{
    activity_capture::ActivityCaptureError,
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
};

const DEFAULT_FOREGROUND_POLL_SECONDS: u64 = 2;
const DEFAULT_FOREGROUND_MIN_GAP_SECONDS: u64 = 2;
const FOREGROUND_TRIGGERS: [ScreenCaptureScheduleTrigger; 1] =
    [ScreenCaptureScheduleTrigger::NativeAppForegroundStart];

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiForegroundRuntimeConfig {
    pub(crate) screen_analysis_enabled: bool,
    pub(crate) foreground_capture_enabled: bool,
    pub(crate) poll_seconds: u64,
    pub(crate) min_trigger_gap_seconds: u64,
    pub(crate) max_captures: Option<u64>,
    pub(crate) max_ticks: Option<u64>,
    pub(crate) max_pending_queue_records: u64,
    pub(crate) temporary_image_ttl_seconds: u64,
    pub(crate) queue_dir: PathBuf,
    pub(crate) journal_path: PathBuf,
    pub(crate) journal_key_path: PathBuf,
    pub(crate) store_path: PathBuf,
}

impl ScreenAiForegroundRuntimeConfig {
    pub(crate) fn from_environment() -> Option<Self> {
        if !env_flag(SCREEN_SERVICE_FOREGROUND_RUNTIME_ENABLED_ENV, false) {
            return None;
        }
        Some(Self {
            screen_analysis_enabled: env_flag(SCREEN_SERVICE_ANALYSIS_ENABLED_ENV, true),
            foreground_capture_enabled: env_flag(SCREEN_SERVICE_FOREGROUND_ENABLED_ENV, true),
            poll_seconds: env_u64(
                SCREEN_SERVICE_FOREGROUND_POLL_SECONDS_ENV,
                DEFAULT_FOREGROUND_POLL_SECONDS,
            ),
            min_trigger_gap_seconds: env_u64(
                SCREEN_SERVICE_FOREGROUND_MIN_GAP_SECONDS_ENV,
                DEFAULT_FOREGROUND_MIN_GAP_SECONDS,
            ),
            max_captures: env_optional_u64(SCREEN_SERVICE_FOREGROUND_MAX_CAPTURES_ENV),
            max_ticks: env_optional_u64(SCREEN_SERVICE_FOREGROUND_MAX_TICKS_ENV),
            max_pending_queue_records: env_u64(
                SCREEN_SERVICE_QUEUE_MAX_PENDING_ENV,
                SCREEN_SERVICE_QUEUE_MAX_PENDING_DEFAULT,
            ),
            temporary_image_ttl_seconds: env_u64(
                SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_ENV,
                SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_DEFAULT,
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
            trigger_capture_enabled: self.foreground_capture_enabled,
            cadence_capture_enabled: false,
            allowed_scope: ScreenCaptureScope::ActiveWindow,
            cadence_seconds: self.poll_seconds,
            min_trigger_gap_seconds: self.min_trigger_gap_seconds,
            enabled_triggers: &FOREGROUND_TRIGGERS,
        }
    }
}

pub(crate) fn foreground_key(observation: &ForegroundWindowObservation) -> Option<String> {
    if observation.status != ActivityCaptureCapabilityStatus::Available {
        return None;
    }
    if let Some(window_id) = observation
        .window_id
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        return Some(prefixed_key(
            SCREEN_SERVICE_FOREGROUND_KEY_WINDOW_PREFIX,
            window_id,
        ));
    }
    if let Some(pid) = observation.pid {
        return Some(prefixed_key(
            SCREEN_SERVICE_FOREGROUND_KEY_PID_PREFIX,
            &pid.to_string(),
        ));
    }
    if let Some(title) = observation
        .title
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        return Some(prefixed_key(
            SCREEN_SERVICE_FOREGROUND_KEY_TITLE_PREFIX,
            title,
        ));
    }
    observation
        .app_name
        .as_deref()
        .filter(|value| !value.is_empty())
        .map(|value| prefixed_key(SCREEN_SERVICE_FOREGROUND_KEY_APP_PREFIX, value))
}

pub(crate) fn pending_queue_record_count(queue_dir: &Path) -> Result<u64, ActivityCaptureError> {
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

fn prefixed_key(prefix: &str, value: &str) -> String {
    let mut key = String::from(prefix);
    key.push_str(value);
    key
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

fn default_queue_dir() -> PathBuf {
    let mut path = env::temp_dir();
    path.push(SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME);
    path
}
