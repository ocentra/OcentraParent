use std::{
    env, fs,
    path::{Path, PathBuf},
    time::Duration,
};

use ocentra_parent_agent_core::{
    journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    screen_evidence_queue::{ScreenEvidenceExpiredQueueEntry, ScreenEvidenceQueue},
};
use ocentra_parent_agent_protocol as parent_protocol;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef, ActivityObserver,
    ActivitySource, ActivitySubject, ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_CAPABILITY_READY, SCREEN_CATEGORY_UNKNOWN, SCREEN_CUSTODY_JOURNAL,
    SCREEN_DELETION_EXPIRED_DELETED, SCREEN_PROVIDER_SERVICE_METADATA,
    SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME, SCREEN_SERVICE_METADATA_CONFIDENCE,
    SCREEN_SERVICE_QUEUE_DIR_ENV, SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
    SCREEN_SERVICE_RETENTION_EVENT_ID_PREFIX, SCREEN_SERVICE_RETENTION_EVIDENCE_ID_PREFIX,
    SCREEN_SERVICE_RETENTION_MODEL_ID, SCREEN_SERVICE_RETENTION_MODEL_RUNTIME_REF,
    SCREEN_SERVICE_RETENTION_RESULT_ID_PREFIX, SCREEN_SERVICE_RETENTION_SUMMARY_EXPIRED_DELETED,
    SCREEN_SERVICE_RETENTION_SWEEPER_MAX_SWEEPS_ENV,
    SCREEN_SERVICE_RETENTION_SWEEPER_MAX_TICKS_ENV,
    SCREEN_SERVICE_RETENTION_SWEEPER_POLL_SECONDS_ENV,
    SCREEN_SERVICE_RETENTION_SWEEPER_RUNTIME_ENABLED_ENV,
    SCREEN_SERVICE_RETENTION_SWEEPER_SOURCE_ID, SCREEN_SERVICE_RETENTION_TEMPLATE_VERSION,
};

use crate::{
    activity_capture::{record_activity_events_to_paths, ActivityCaptureError},
    activity_store_path::{activity_db_path, activity_journal_key_path, activity_journal_path},
    fields::fields_from_pairs,
    screen_ai_retention_sweeper_deletion_events::publish_screen_retention_deletion_events,
    time::timestamp_now,
};

const DEFAULT_POLL_SECONDS: u64 = 5;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiRetentionSweeperRuntimeConfig {
    pub(crate) poll_seconds: u64,
    pub(crate) max_sweeps: Option<u64>,
    pub(crate) max_ticks: Option<u64>,
    pub(crate) queue_dir: PathBuf,
    pub(crate) journal_path: PathBuf,
    pub(crate) journal_key_path: PathBuf,
    pub(crate) store_path: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ScreenAiRetentionSweeperClock {
    pub(crate) timestamp: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum ScreenAiRetentionSweeperOutcome {
    QueueEmpty,
    NoExpired {
        pending_count: u64,
    },
    Swept {
        expired_entries: Vec<ScreenEvidenceExpiredQueueEntry>,
        retained_count: u64,
    },
}

pub(crate) fn spawn_screen_ai_retention_sweeper_runtime() {
    if let Some(config) = ScreenAiRetentionSweeperRuntimeConfig::from_environment() {
        tokio::spawn(async move {
            run_screen_ai_retention_sweeper_runtime(config).await;
        });
    }
}

async fn run_screen_ai_retention_sweeper_runtime(config: ScreenAiRetentionSweeperRuntimeConfig) {
    let mut interval = tokio::time::interval(Duration::from_secs(config.poll_seconds));
    let mut sweep_count = 0;
    let mut tick_count = 0;
    loop {
        interval.tick().await;
        tick_count += 1;
        let clock = ScreenAiRetentionSweeperClock::now();
        let observed_at = clock.timestamp.clone();
        let outcome = record_screen_ai_retention_sweeper_tick(&config, clock);
        if let Ok(ScreenAiRetentionSweeperOutcome::Swept {
            expired_entries, ..
        }) = outcome
        {
            let _ = publish_screen_retention_deletion_events(
                &config.store_path,
                &expired_entries,
                &observed_at,
            )
            .await;
            sweep_count += 1;
        }
        if config.max_sweeps.is_some_and(|max| sweep_count >= max) {
            break;
        }
        if config.max_ticks.is_some_and(|max| tick_count >= max) {
            break;
        }
    }
}

pub(crate) fn record_screen_ai_retention_sweeper_tick(
    config: &ScreenAiRetentionSweeperRuntimeConfig,
    clock: ScreenAiRetentionSweeperClock,
) -> Result<ScreenAiRetentionSweeperOutcome, ActivityCaptureError> {
    let ScreenAiRetentionSweeperClock { timestamp } = clock;
    let Some(key) = load_existing_screen_key(&config.journal_key_path)? else {
        return Ok(ScreenAiRetentionSweeperOutcome::QueueEmpty);
    };
    let queue = ScreenEvidenceQueue::open(&config.queue_dir, key)?;
    let sweep = queue
        .remove_expired_entries(&timestamp, SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX)?;
    if !sweep.expired_entries.is_empty() {
        let events = sweep
            .expired_entries
            .iter()
            .map(|entry| expired_entry_event(entry, &timestamp))
            .collect::<Vec<_>>();
        record_activity_events_to_paths(
            &config.journal_path,
            &config.journal_key_path,
            &config.store_path,
            &events,
        )?;
        return Ok(ScreenAiRetentionSweeperOutcome::Swept {
            expired_entries: sweep.expired_entries,
            retained_count: sweep.retained_count,
        });
    }
    if sweep.retained_count == 0 {
        Ok(ScreenAiRetentionSweeperOutcome::QueueEmpty)
    } else {
        Ok(ScreenAiRetentionSweeperOutcome::NoExpired {
            pending_count: sweep.retained_count,
        })
    }
}

impl ScreenAiRetentionSweeperRuntimeConfig {
    pub(crate) fn from_environment() -> Option<Self> {
        if !env_flag(SCREEN_SERVICE_RETENTION_SWEEPER_RUNTIME_ENABLED_ENV, false) {
            return None;
        }
        Some(Self {
            poll_seconds: env_u64(
                SCREEN_SERVICE_RETENTION_SWEEPER_POLL_SECONDS_ENV,
                DEFAULT_POLL_SECONDS,
            ),
            max_sweeps: env_optional_u64(SCREEN_SERVICE_RETENTION_SWEEPER_MAX_SWEEPS_ENV),
            max_ticks: env_optional_u64(SCREEN_SERVICE_RETENTION_SWEEPER_MAX_TICKS_ENV),
            queue_dir: env_path(SCREEN_SERVICE_QUEUE_DIR_ENV).unwrap_or_else(default_queue_dir),
            journal_path: activity_journal_path(),
            journal_key_path: activity_journal_key_path(),
            store_path: activity_db_path(),
        })
    }
}

impl ScreenAiRetentionSweeperClock {
    pub(crate) fn now() -> Self {
        Self {
            timestamp: timestamp_now(),
        }
    }
}

fn load_existing_screen_key(path: &Path) -> Result<Option<JournalKey>, ActivityCaptureError> {
    match fs::read(path) {
        Ok(bytes) => journal_key_from_bytes(&bytes).map(Some),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err(ActivityCaptureError::Io),
    }
}

fn journal_key_from_bytes(bytes: &[u8]) -> Result<JournalKey, ActivityCaptureError> {
    if bytes.len() != JOURNAL_KEY_BYTES {
        return Err(ActivityCaptureError::InvalidKeyLength);
    }
    let mut key = [0; JOURNAL_KEY_BYTES];
    key.copy_from_slice(bytes);
    Ok(JournalKey::from_bytes(key))
}

fn env_flag(env_var_name: &str, default_value: bool) -> bool {
    env::var(env_var_name)
        .ok()
        .map(|value| value == constants::value::TRUE)
        .unwrap_or(default_value)
}

fn env_u64(env_var_name: &str, default_value: u64) -> u64 {
    env::var(env_var_name)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
        .unwrap_or(default_value)
}

fn env_optional_u64(env_var_name: &str) -> Option<u64> {
    env::var(env_var_name)
        .ok()
        .and_then(|value| value.parse::<u64>().ok())
        .filter(|value| *value > 0)
}

fn env_path(env_var_name: &str) -> Option<PathBuf> {
    env::var(env_var_name)
        .ok()
        .filter(|value| !value.is_empty())
        .map(PathBuf::from)
}

fn default_queue_dir() -> PathBuf {
    let mut path = env::temp_dir();
    path.push(SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME);
    path
}

fn expired_entry_event(
    entry: &ScreenEvidenceExpiredQueueEntry,
    observed_at: &str,
) -> ActivityEvent {
    let mut evidence_id = String::from(SCREEN_SERVICE_RETENTION_EVIDENCE_ID_PREFIX);
    evidence_id.push_str(&entry.queue_job_id);
    let mut event_id = String::from(SCREEN_SERVICE_RETENTION_EVENT_ID_PREFIX);
    event_id.push_str(&entry.queue_job_id);
    let evidence = vec![ActivityEvidenceRef {
        evidence_id,
        kind: ActivityEvidenceKind::JournalEntry,
        digest: Some(entry.image_digest.clone()),
        uri: None,
    }];
    ActivityEvent {
        schema_version: parent_protocol::ACTIVITY_SCHEMA_VERSION,
        event_id,
        observed_at: observed_at.to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::LocalAi,
            source_id: SCREEN_SERVICE_RETENTION_SWEEPER_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::ScreenAnalysisSummarized,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            display_name: Some(SCREEN_SERVICE_RETENTION_SWEEPER_SOURCE_ID.to_string()),
        },
        fields: fields_from_pairs(expired_entry_fields(entry)),
        evidence,
    }
}

fn expired_entry_fields(
    entry: &ScreenEvidenceExpiredQueueEntry,
) -> Vec<(&'static str, LogFieldValue)> {
    let mut result_id = String::from(SCREEN_SERVICE_RETENTION_RESULT_ID_PREFIX);
    result_id.push_str(&entry.queue_job_id);
    vec![
        string_field(constants::field::SCREEN_ANALYSIS_RESULT_ID, result_id),
        string_field(
            constants::field::SCREEN_QUEUE_JOB_ID,
            entry.queue_job_id.clone(),
        ),
        string_field(
            constants::field::SCREEN_SUMMARY,
            SCREEN_SERVICE_RETENTION_SUMMARY_EXPIRED_DELETED,
        ),
        string_field(
            constants::field::SCREEN_PRIMARY_CATEGORY,
            SCREEN_CATEGORY_UNKNOWN,
        ),
        number_field(
            constants::field::SCREEN_CONFIDENCE,
            SCREEN_SERVICE_METADATA_CONFIDENCE,
        ),
        string_field(
            constants::field::SCREEN_IMAGE_DELETION_STATE,
            SCREEN_DELETION_EXPIRED_DELETED,
        ),
        string_field(
            constants::field::SCREEN_DELETION_REASONS,
            entry.deletion_proof_ref.clone(),
        ),
        bool_field(constants::field::SCREEN_POLICY_ELIGIBLE, false),
        string_field(
            constants::field::SCREEN_MODEL_RUNTIME_REF,
            SCREEN_SERVICE_RETENTION_MODEL_RUNTIME_REF,
        ),
        string_field(
            constants::field::SCREEN_MODEL_ID,
            SCREEN_SERVICE_RETENTION_MODEL_ID,
        ),
        string_field(
            constants::field::SCREEN_PROVIDER_KIND,
            SCREEN_PROVIDER_SERVICE_METADATA,
        ),
        string_field(
            constants::field::SCREEN_TEMPLATE_VERSION,
            SCREEN_SERVICE_RETENTION_TEMPLATE_VERSION,
        ),
        string_field(
            constants::field::SCREEN_CAPTURE_REASON,
            constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE,
        ),
        string_field(
            constants::field::SCREEN_CAPTURE_SCOPE,
            ocentra_parent_agent_protocol::screen_evidence::SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW,
        ),
        string_field(constants::field::CAPABILITY_STATUS, SCREEN_CAPABILITY_READY),
        string_field(
            constants::field::SCREEN_IMAGE_DIGEST,
            entry.image_digest.clone(),
        ),
        string_field(
            constants::field::SCREEN_CUSTODY_STATE,
            SCREEN_CUSTODY_JOURNAL,
        ),
    ]
}

fn string_field(key: &'static str, value: impl Into<String>) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::String(value.into()))
}

fn number_field(key: &'static str, value: f64) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::Number(value))
}

fn bool_field(key: &'static str, value: bool) -> (&'static str, LogFieldValue) {
    (key, LogFieldValue::Boolean(value))
}
