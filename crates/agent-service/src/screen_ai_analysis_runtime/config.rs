use std::{env, path::PathBuf};

use ocentra_parent_agent_protocol::{
    constants, SCREEN_SERVICE_ANALYSIS_ADAPTER_COMMAND_ENV,
    SCREEN_SERVICE_ANALYSIS_ADAPTER_TIMEOUT_MS_ENV,
    SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS,
    SCREEN_SERVICE_ANALYSIS_DEFAULT_MAX_QUEUE_SCAN, SCREEN_SERVICE_ANALYSIS_DEFAULT_POLL_SECONDS,
    SCREEN_SERVICE_ANALYSIS_MAX_JOBS_ENV, SCREEN_SERVICE_ANALYSIS_MAX_TICKS_ENV,
    SCREEN_SERVICE_ANALYSIS_POLL_SECONDS_ENV, SCREEN_SERVICE_ANALYSIS_RUNTIME_ENABLED_ENV,
    SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME, SCREEN_SERVICE_QUEUE_DIR_ENV,
};

use crate::{
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    time::timestamp_now,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiAnalysisRuntimeConfig {
    pub(crate) screen_analysis_enabled: bool,
    pub(crate) poll_seconds: u64,
    pub(crate) max_jobs: Option<u64>,
    pub(crate) max_ticks: Option<u64>,
    pub(crate) max_queue_scan: usize,
    pub(crate) adapter_timeout_ms: u64,
    pub(crate) adapter_command: Option<PathBuf>,
    pub(crate) queue_dir: PathBuf,
    pub(crate) journal_path: PathBuf,
    pub(crate) journal_key_path: PathBuf,
    pub(crate) store_path: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ScreenAiAnalysisCycleOutcome {
    Suppressed,
    QueueEmpty,
    AlreadyAnalyzed {
        queue_job_id: String,
    },
    ProviderUnavailable {
        queue_job_id: String,
    },
    InvalidOutput {
        queue_job_id: String,
    },
    Recorded {
        queue_job_id: String,
        provider_kind: String,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiAnalysisCycleClock {
    pub(crate) epoch_seconds: u64,
    pub(crate) timestamp: String,
}

impl ScreenAiAnalysisRuntimeConfig {
    pub(crate) fn from_environment() -> Option<Self> {
        if !env_flag(SCREEN_SERVICE_ANALYSIS_RUNTIME_ENABLED_ENV, false) {
            return None;
        }
        Some(Self {
            screen_analysis_enabled: true,
            poll_seconds: env_u64(
                SCREEN_SERVICE_ANALYSIS_POLL_SECONDS_ENV,
                SCREEN_SERVICE_ANALYSIS_DEFAULT_POLL_SECONDS,
            ),
            max_jobs: env_optional_u64(SCREEN_SERVICE_ANALYSIS_MAX_JOBS_ENV),
            max_ticks: env_optional_u64(SCREEN_SERVICE_ANALYSIS_MAX_TICKS_ENV),
            max_queue_scan: SCREEN_SERVICE_ANALYSIS_DEFAULT_MAX_QUEUE_SCAN,
            adapter_timeout_ms: env_u64(
                SCREEN_SERVICE_ANALYSIS_ADAPTER_TIMEOUT_MS_ENV,
                SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS,
            ),
            adapter_command: env_path(SCREEN_SERVICE_ANALYSIS_ADAPTER_COMMAND_ENV),
            queue_dir: env_path(SCREEN_SERVICE_QUEUE_DIR_ENV).unwrap_or_else(default_queue_dir),
            journal_path: activity_journal_path(),
            journal_key_path: activity_journal_key_path(),
            store_path: activity_db_path(),
        })
    }
}

impl ScreenAiAnalysisCycleClock {
    #[cfg(test)]
    pub(crate) fn from_parts(epoch_seconds: u64, timestamp: String) -> Self {
        Self {
            epoch_seconds,
            timestamp,
        }
    }

    pub(crate) fn from_system_time() -> Self {
        Self {
            epoch_seconds: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
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

fn default_queue_dir() -> PathBuf {
    let mut path = env::temp_dir();
    path.push(SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME);
    path
}
