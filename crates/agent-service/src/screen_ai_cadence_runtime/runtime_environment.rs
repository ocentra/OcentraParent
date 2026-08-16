use std::{
    env, fs,
    path::{Path, PathBuf},
};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_SERVICE_ANALYSIS_ENABLED_ENV, SCREEN_SERVICE_CADENCE_ENABLED_ENV,
    SCREEN_SERVICE_CADENCE_MAX_CAPTURES_ENV, SCREEN_SERVICE_CADENCE_MAX_TICKS_ENV,
    SCREEN_SERVICE_CADENCE_RUNTIME_ENABLED_ENV, SCREEN_SERVICE_CADENCE_SECONDS_ENV,
    SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME, SCREEN_SERVICE_QUEUE_DIR_ENV,
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
    screen_ai_cadence_runtime::ScreenAiCadenceRuntimeConfig,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ScreenAiRuntimeEnvVar(&'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScreenAiQueueDir(PathBuf);

const CADENCE_RUNTIME_ENABLED_ENV: ScreenAiRuntimeEnvVar =
    ScreenAiRuntimeEnvVar(SCREEN_SERVICE_CADENCE_RUNTIME_ENABLED_ENV);
const ANALYSIS_ENABLED_ENV: ScreenAiRuntimeEnvVar =
    ScreenAiRuntimeEnvVar(SCREEN_SERVICE_ANALYSIS_ENABLED_ENV);
const CADENCE_ENABLED_ENV: ScreenAiRuntimeEnvVar =
    ScreenAiRuntimeEnvVar(SCREEN_SERVICE_CADENCE_ENABLED_ENV);
const CADENCE_SECONDS_ENV: ScreenAiRuntimeEnvVar =
    ScreenAiRuntimeEnvVar(SCREEN_SERVICE_CADENCE_SECONDS_ENV);
const CADENCE_MAX_CAPTURES_ENV: ScreenAiRuntimeEnvVar =
    ScreenAiRuntimeEnvVar(SCREEN_SERVICE_CADENCE_MAX_CAPTURES_ENV);
const CADENCE_MAX_TICKS_ENV: ScreenAiRuntimeEnvVar =
    ScreenAiRuntimeEnvVar(SCREEN_SERVICE_CADENCE_MAX_TICKS_ENV);
const QUEUE_MAX_PENDING_ENV: ScreenAiRuntimeEnvVar =
    ScreenAiRuntimeEnvVar(SCREEN_SERVICE_QUEUE_MAX_PENDING_ENV);
const TEMPORARY_IMAGE_TTL_SECONDS_ENV: ScreenAiRuntimeEnvVar =
    ScreenAiRuntimeEnvVar(SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_ENV);
const QUEUE_DIR_ENV: ScreenAiRuntimeEnvVar = ScreenAiRuntimeEnvVar(SCREEN_SERVICE_QUEUE_DIR_ENV);

pub(super) fn from_environment() -> Option<ScreenAiCadenceRuntimeConfig> {
    if !env_flag(CADENCE_RUNTIME_ENABLED_ENV, false) {
        return None;
    }
    Some(ScreenAiCadenceRuntimeConfig {
        screen_analysis_enabled: env_flag(ANALYSIS_ENABLED_ENV, true),
        cadence_capture_enabled: env_flag(CADENCE_ENABLED_ENV, true),
        cadence_seconds: env_u64(CADENCE_SECONDS_ENV, super::super::DEFAULT_CADENCE_SECONDS),
        max_captures: env_optional_u64(CADENCE_MAX_CAPTURES_ENV),
        max_ticks: env_optional_u64(CADENCE_MAX_TICKS_ENV),
        max_pending_queue_records: env_u64(
            QUEUE_MAX_PENDING_ENV,
            SCREEN_SERVICE_QUEUE_MAX_PENDING_DEFAULT,
        ),
        temporary_image_ttl_seconds: env_u64(
            TEMPORARY_IMAGE_TTL_SECONDS_ENV,
            SCREEN_SERVICE_TEMPORARY_IMAGE_TTL_SECONDS_DEFAULT,
        ),
        queue_dir: env_path(QUEUE_DIR_ENV).unwrap_or_else(default_queue_dir).0,
        journal_path: activity_journal_path().into(),
        journal_key_path: activity_journal_key_path().into(),
        store_path: activity_db_path().into(),
    })
}

pub(super) fn scheduler_settings(
    config: &ScreenAiCadenceRuntimeConfig,
) -> ScreenCaptureSchedulerSettings {
    ScreenCaptureSchedulerSettings {
        screen_analysis_enabled: config.screen_analysis_enabled,
        trigger_capture_enabled: true,
        cadence_capture_enabled: config.cadence_capture_enabled,
        allowed_scope: ScreenCaptureScope::ActiveWindow,
        cadence_seconds: config.cadence_seconds,
        min_trigger_gap_seconds: config.cadence_seconds,
        enabled_triggers: &[ScreenCaptureScheduleTrigger::TimedCadence],
    }
}

pub(super) fn pending_queue_record_count(queue_dir: &Path) -> Result<u64, ActivityCaptureError> {
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

fn env_flag(env_var: ScreenAiRuntimeEnvVar, default_value: bool) -> bool {
    env::var(env_var.0)
        .ok()
        .map(|value| value == constants::value::TRUE)
        .unwrap_or(default_value)
}

fn env_u64(env_var: ScreenAiRuntimeEnvVar, default_value: u64) -> u64 {
    env::var(env_var.0)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_value)
}

fn env_optional_u64(env_var: ScreenAiRuntimeEnvVar) -> Option<u64> {
    env::var(env_var.0)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
}

fn env_path(env_var: ScreenAiRuntimeEnvVar) -> Option<ScreenAiQueueDir> {
    env::var(env_var.0)
        .ok()
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .map(ScreenAiQueueDir)
}

fn default_queue_dir() -> ScreenAiQueueDir {
    let mut path = env::temp_dir();
    path.push(SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME);
    ScreenAiQueueDir(path)
}
