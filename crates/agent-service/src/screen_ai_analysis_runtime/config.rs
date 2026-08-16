use std::{env, fs, path::PathBuf};

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_ADAPTER_COMMAND_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_ADAPTER_TIMEOUT_MS_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_DEFAULT_MAX_QUEUE_SCAN;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_DEFAULT_POLL_SECONDS;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_ENABLED_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_MAX_JOBS_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_MAX_TICKS_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_POLL_SECONDS_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ANALYSIS_RUNTIME_ENABLED_ENV;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME;
use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_QUEUE_DIR_ENV;
use serde_json::Value;

use crate::{
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    time::timestamp_now,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ScreenAiEnvVar(&'static str);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ScreenAiFieldName(&'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScreenAiText(String);

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScreenAiPath(PathBuf);

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
        if !env_flag(
            ScreenAiEnvVar(SCREEN_SERVICE_ANALYSIS_RUNTIME_ENABLED_ENV),
            false,
        ) {
            return None;
        }
        Some(Self {
            screen_analysis_enabled: env_flag(
                ScreenAiEnvVar(SCREEN_SERVICE_ANALYSIS_ENABLED_ENV),
                true,
            ),
            poll_seconds: env_u64(
                ScreenAiEnvVar(SCREEN_SERVICE_ANALYSIS_POLL_SECONDS_ENV),
                SCREEN_SERVICE_ANALYSIS_DEFAULT_POLL_SECONDS,
            ),
            max_jobs: env_optional_u64(ScreenAiEnvVar(SCREEN_SERVICE_ANALYSIS_MAX_JOBS_ENV)),
            max_ticks: env_optional_u64(ScreenAiEnvVar(SCREEN_SERVICE_ANALYSIS_MAX_TICKS_ENV)),
            max_queue_scan: SCREEN_SERVICE_ANALYSIS_DEFAULT_MAX_QUEUE_SCAN,
            adapter_timeout_ms: env_u64(
                ScreenAiEnvVar(SCREEN_SERVICE_ANALYSIS_ADAPTER_TIMEOUT_MS_ENV),
                SCREEN_SERVICE_ANALYSIS_DEFAULT_ADAPTER_TIMEOUT_MS,
            ),
            adapter_command: env_path(ScreenAiEnvVar(SCREEN_SERVICE_ANALYSIS_ADAPTER_COMMAND_ENV))
                .map(|path| path.0),
            ocr_redaction_policy: env_path(ScreenAiEnvVar(
                constants::local_ai_runtime::SCREEN_SERVICE_OCR_REDACTION_POLICY_PATH_ENV,
            ))
            .and_then(|path| ScreenOcrRedactionPolicy::from_file(path.0))
            .unwrap_or_default(),
            queue_dir: env_path(ScreenAiEnvVar(SCREEN_SERVICE_QUEUE_DIR_ENV))
                .map(|path| path.0)
                .unwrap_or_else(|| default_queue_dir().0),
            journal_path: activity_journal_path().into(),
            journal_key_path: activity_journal_key_path().into(),
            store_path: activity_db_path().into(),
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
    fn from_file(path: impl AsRef<std::path::Path>) -> Option<Self> {
        let value: Value = serde_json::from_str(&fs::read_to_string(path.as_ref()).ok()?).ok()?;
        let mut policy = Self::default();
        policy.ocr_text_enabled = optional_bool(
            &value,
            ScreenAiFieldName(constants::field::SCREEN_OCR_TEXT_ENABLED),
            policy.ocr_text_enabled,
        );
        policy.snippet_limit = optional_usize(
            &value,
            ScreenAiFieldName(constants::field::SCREEN_OCR_SNIPPET_LIMIT),
            policy.snippet_limit,
        )
        .min(constants::local_ai_runtime::SCREEN_SERVICE_OCR_SNIPPET_LIMIT);
        policy.redaction_mode = optional_string(
            &value,
            ScreenAiFieldName(constants::field::SCREEN_OCR_REDACTION_MODE),
            ScreenAiText(policy.redaction_mode.clone()),
        )
        .0;
        policy.text_retention_mode = optional_string(
            &value,
            ScreenAiFieldName(constants::field::SCREEN_OCR_TEXT_RETENTION_MODE),
            ScreenAiText(policy.text_retention_mode.clone()),
        )
        .0;
        policy.credential_suppression_enabled = optional_bool(
            &value,
            ScreenAiFieldName(constants::field::SCREEN_OCR_CREDENTIAL_SUPPRESSION_ENABLED),
            policy.credential_suppression_enabled,
        );
        policy.pii_redaction_enabled = optional_bool(
            &value,
            ScreenAiFieldName(constants::field::SCREEN_OCR_PII_REDACTION_ENABLED),
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

fn env_flag(env_var_name: ScreenAiEnvVar, default_value: bool) -> bool {
    env::var(env_var_name.0)
        .ok()
        .map(|value| value == constants::value::TRUE)
        .unwrap_or(default_value)
}

fn env_u64(env_var_name: ScreenAiEnvVar, default_value: u64) -> u64 {
    env::var(env_var_name.0)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_value)
}

fn env_optional_u64(env_var_name: ScreenAiEnvVar) -> Option<u64> {
    env::var(env_var_name.0)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
}

fn env_path(env_var_name: ScreenAiEnvVar) -> Option<ScreenAiPath> {
    env::var(env_var_name.0)
        .ok()
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
        .map(ScreenAiPath)
}

fn optional_bool(value: &Value, field: ScreenAiFieldName, default_value: bool) -> bool {
    value
        .get(field.0)
        .and_then(Value::as_bool)
        .unwrap_or(default_value)
}

fn optional_usize(value: &Value, field: ScreenAiFieldName, default_value: usize) -> usize {
    value
        .get(field.0)
        .and_then(Value::as_u64)
        .and_then(|candidate| usize::try_from(candidate).ok())
        .unwrap_or(default_value)
}

fn optional_string(
    value: &Value,
    field: ScreenAiFieldName,
    default_value: ScreenAiText,
) -> ScreenAiText {
    value
        .get(field.0)
        .and_then(Value::as_str)
        .filter(|candidate| !candidate.is_empty())
        .map(|candidate| ScreenAiText(candidate.to_string()))
        .unwrap_or(default_value)
}

fn default_queue_dir() -> ScreenAiPath {
    let mut path = env::temp_dir();
    path.push(SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME);
    ScreenAiPath(path)
}
