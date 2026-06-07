use std::{env, fs, path::PathBuf};

use ocentra_parent_agent_protocol::{
    constants, SCREEN_SERVICE_ANALYSIS_ADAPTER_COMMAND_ENV,
    SCREEN_SERVICE_ANALYSIS_ADAPTER_TIMEOUT_MS_ENV,
    SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS,
    SCREEN_SERVICE_ANALYSIS_DEFAULT_MAX_QUEUE_SCAN, SCREEN_SERVICE_ANALYSIS_DEFAULT_POLL_SECONDS,
    SCREEN_SERVICE_ANALYSIS_ENABLED_ENV, SCREEN_SERVICE_ANALYSIS_MAX_JOBS_ENV,
    SCREEN_SERVICE_ANALYSIS_MAX_TICKS_ENV, SCREEN_SERVICE_ANALYSIS_POLL_SECONDS_ENV,
    SCREEN_SERVICE_ANALYSIS_RUNTIME_ENABLED_ENV, SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME,
    SCREEN_SERVICE_QUEUE_DIR_ENV,
};
use serde_json::Value;

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
    pub(crate) ocr_redaction_policy: ScreenOcrRedactionPolicy,
    pub(crate) queue_dir: PathBuf,
    pub(crate) journal_path: PathBuf,
    pub(crate) journal_key_path: PathBuf,
    pub(crate) store_path: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenOcrRedactionPolicy {
    pub(crate) ocr_text_enabled: bool,
    pub(crate) snippet_limit: usize,
    pub(crate) redaction_mode: String,
    pub(crate) text_retention_mode: String,
    pub(crate) credential_suppression_enabled: bool,
    pub(crate) pii_redaction_enabled: bool,
    pub(crate) parent_setting_ref: Option<String>,
    pub(crate) setting_version: Option<u64>,
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
            screen_analysis_enabled: env_flag(SCREEN_SERVICE_ANALYSIS_ENABLED_ENV, true),
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
            ocr_redaction_policy: env_path(
                constants::local_ai_runtime::SCREEN_SERVICE_OCR_REDACTION_POLICY_PATH_ENV,
            )
            .and_then(|path| ScreenOcrRedactionPolicy::from_file(&path))
            .unwrap_or_default(),
            queue_dir: env_path(SCREEN_SERVICE_QUEUE_DIR_ENV).unwrap_or_else(default_queue_dir),
            journal_path: activity_journal_path(),
            journal_key_path: activity_journal_key_path(),
            store_path: activity_db_path(),
        })
    }
}

impl Default for ScreenOcrRedactionPolicy {
    fn default() -> Self {
        Self {
            ocr_text_enabled: true,
            snippet_limit: constants::local_ai_runtime::SCREEN_SERVICE_OCR_SNIPPET_LIMIT,
            redaction_mode:
                constants::local_ai_runtime::SCREEN_OCR_REDACTION_MODE_LOCAL_SENSITIVE_TEXT
                    .to_string(),
            text_retention_mode:
                constants::local_ai_runtime::SCREEN_OCR_TEXT_RETENTION_REDACTED_SNIPPETS.to_string(),
            credential_suppression_enabled: true,
            pii_redaction_enabled: true,
            parent_setting_ref: None,
            setting_version: None,
        }
    }
}

impl ScreenOcrRedactionPolicy {
    fn from_file(path: &PathBuf) -> Option<Self> {
        let value: Value = serde_json::from_str(&fs::read_to_string(path).ok()?).ok()?;
        let mut policy = Self::default();
        policy.ocr_text_enabled = optional_bool(
            &value,
            constants::field::SCREEN_OCR_TEXT_ENABLED,
            policy.ocr_text_enabled,
        );
        policy.snippet_limit = optional_usize(
            &value,
            constants::field::SCREEN_OCR_SNIPPET_LIMIT,
            policy.snippet_limit,
        )
        .min(constants::local_ai_runtime::SCREEN_SERVICE_OCR_SNIPPET_LIMIT);
        policy.redaction_mode = optional_string(
            &value,
            constants::field::SCREEN_OCR_REDACTION_MODE,
            &policy.redaction_mode,
        );
        policy.text_retention_mode = optional_string(
            &value,
            constants::field::SCREEN_OCR_TEXT_RETENTION_MODE,
            &policy.text_retention_mode,
        );
        policy.credential_suppression_enabled = optional_bool(
            &value,
            constants::field::SCREEN_OCR_CREDENTIAL_SUPPRESSION_ENABLED,
            policy.credential_suppression_enabled,
        );
        policy.pii_redaction_enabled = optional_bool(
            &value,
            constants::field::SCREEN_OCR_PII_REDACTION_ENABLED,
            policy.pii_redaction_enabled,
        );
        policy.parent_setting_ref = value
            .get(constants::field::SCREEN_PARENT_SETTING_REF)
            .and_then(Value::as_str)
            .filter(|candidate| !candidate.is_empty())
            .map(str::to_string);
        policy.setting_version = value
            .get(constants::field::SCREEN_SETTING_VERSION)
            .and_then(Value::as_u64);
        Some(policy.normalized())
    }

    fn normalized(mut self) -> Self {
        if !self.ocr_text_enabled
            || self.text_retention_mode
                == constants::local_ai_runtime::SCREEN_OCR_TEXT_RETENTION_DISABLED
            || self.redaction_mode
                == constants::local_ai_runtime::SCREEN_OCR_REDACTION_MODE_DISABLED
        {
            self.ocr_text_enabled = false;
            self.snippet_limit = 0;
            self.text_retention_mode =
                constants::local_ai_runtime::SCREEN_OCR_TEXT_RETENTION_DISABLED.to_string();
            self.redaction_mode =
                constants::local_ai_runtime::SCREEN_OCR_REDACTION_MODE_DISABLED.to_string();
            self.pii_redaction_enabled = false;
        }
        self
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

fn optional_bool(value: &Value, field: &str, default_value: bool) -> bool {
    value
        .get(field)
        .and_then(Value::as_bool)
        .unwrap_or(default_value)
}

fn optional_usize(value: &Value, field: &str, default_value: usize) -> usize {
    value
        .get(field)
        .and_then(Value::as_u64)
        .and_then(|candidate| usize::try_from(candidate).ok())
        .unwrap_or(default_value)
}

fn optional_string(value: &Value, field: &str, default_value: &str) -> String {
    value
        .get(field)
        .and_then(Value::as_str)
        .filter(|candidate| !candidate.is_empty())
        .unwrap_or(default_value)
        .to_string()
}

fn default_queue_dir() -> PathBuf {
    let mut path = env::temp_dir();
    path.push(SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME);
    path
}
