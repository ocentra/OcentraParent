use std::{env, path::PathBuf, time::Duration};

use ocentra_parent_agent_core::screen_evidence_queue::{
    ScreenEvidenceExpiredQueueEntry, ScreenEvidenceOutboxFailure, ScreenEvidenceQueue,
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
    screen_ai_service_event_subscription::{ObservedAtText, ScreenAiServiceEventRuntime},
    time::timestamp_now,
};

#[path = "screen_ai_retention_sweeper_failure_events.rs"]
mod failure_events;
#[path = "screen_ai_retention_sweeper_runtime/key_loader.rs"]
mod key_loader;
#[path = "screen_ai_retention_sweeper_runtime/outbox_projection.rs"]
mod outbox_projection;

use failure_events::outbox_failure_event;

const DEFAULT_POLL_SECONDS: u64 = 5;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct ScreenAiEnvVar(&'static str);

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScreenAiPath(PathBuf);

#[derive(Clone, Debug, PartialEq, Eq)]
struct ScreenAiObservedAt(String);

struct RetentionEventFields(Vec<(&'static str, LogFieldValue)>);
struct RetentionEventId(String);
struct RetentionEvidenceId(String);

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

pub(crate) async fn run_screen_ai_retention_blocking<T, F>(
    operation: F,
) -> Result<T, tokio::task::JoinError>
where
    T: Send + 'static,
    F: FnOnce() -> T + Send + 'static,
{
    tokio::task::spawn_blocking(operation).await
}

async fn run_screen_ai_retention_sweeper_runtime(config: ScreenAiRetentionSweeperRuntimeConfig) {
    let Ok(event_runtime) = ScreenAiServiceEventRuntime::start().await else {
        return;
    };
    let mut interval = tokio::time::interval(Duration::from_secs(config.poll_seconds));
    let mut sweep_count = 0;
    let mut tick_count = 0;
    loop {
        interval.tick().await;
        tick_count += 1;
        let clock = ScreenAiRetentionSweeperClock::now();
        let observed_at = clock.timestamp.clone();
        let tick_config = config.clone();
        let Ok(outcome) = run_screen_ai_retention_blocking(move || {
            record_screen_ai_retention_sweeper_tick(&tick_config, clock)
        })
        .await
        else {
            return;
        };
        if let Ok(ScreenAiRetentionSweeperOutcome::Swept {
            expired_entries, ..
        }) = outcome
        {
            let published = publish_screen_retention_deletion_events(
                &event_runtime,
                &config.store_path,
                &expired_entries,
                ObservedAtText(observed_at),
            )
            .await;
            let finalization_config = config.clone();
            let Ok(_) = run_screen_ai_retention_blocking(move || {
                finalize_published_deletion_outbox(&finalization_config, &published)
            })
            .await
            else {
                return;
            };
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

pub(crate) fn acknowledge_published_deletion_outbox(
    config: &ScreenAiRetentionSweeperRuntimeConfig,
    published: &[crate::screen_ai_retention_sweeper_deletion_events::ScreenAiRetentionSweeperDeletionEventOutcome],
) -> Result<u64, ActivityCaptureError> {
    if published.is_empty() {
        return Ok(0);
    }
    let Some(key) = key_loader::load_existing_screen_key(&config.journal_key_path)? else {
        return Err(ActivityCaptureError::Io);
    };
    let queue = ScreenEvidenceQueue::open(&config.queue_dir, key)?;
    let queue_job_ids = published
        .iter()
        .map(|outcome| outcome.queue_job_id.clone())
        .collect::<Vec<_>>();
    let acknowledged = queue.acknowledge_expired_entries(&queue_job_ids)?;
    if acknowledged != queue_job_ids.len() as u64 {
        return Err(ActivityCaptureError::Io);
    }
    Ok(acknowledged)
}

pub(crate) fn finalize_published_deletion_outbox(
    config: &ScreenAiRetentionSweeperRuntimeConfig,
    published: &[crate::screen_ai_retention_sweeper_deletion_events::ScreenAiRetentionSweeperDeletionEventOutcome],
) -> Result<u64, ActivityCaptureError> {
    acknowledge_published_deletion_outbox(config, published)
}

pub(crate) fn record_screen_ai_retention_sweeper_tick(
    config: &ScreenAiRetentionSweeperRuntimeConfig,
    clock: ScreenAiRetentionSweeperClock,
) -> Result<ScreenAiRetentionSweeperOutcome, ActivityCaptureError> {
    let ScreenAiRetentionSweeperClock { timestamp } = clock;
    let observed_at = ScreenAiObservedAt(timestamp);
    let Some(key) = key_loader::load_existing_screen_key(&config.journal_key_path)? else {
        return Ok(ScreenAiRetentionSweeperOutcome::QueueEmpty);
    };
    let queue = ScreenEvidenceQueue::open(&config.queue_dir, key)?;
    let sweep_result = queue.remove_expired_entries(
        observed_at.0.as_str(),
        SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX,
    );
    let failure_outcome = sweep_result
        .as_ref()
        .err()
        .map(|_| record_retryable_sweep_failure(&queue, config, &observed_at))
        .transpose()?
        .flatten();
    if let Some(outcome) = failure_outcome {
        return Ok(outcome);
    }
    let sweep = sweep_result?;
    let mut events = sweep
        .expired_entries
        .iter()
        .map(|entry| expired_entry_event(entry, observed_at.clone()))
        .collect::<Vec<_>>();
    events.extend(
        sweep
            .outbox_failures
            .iter()
            .map(|failure| outbox_failure_event(failure, observed_at.clone())),
    );
    if !events.is_empty() {
        record_activity_events_to_paths(
            &config.journal_path,
            &config.journal_key_path,
            &config.store_path,
            &events,
        )?;
        outbox_projection::acknowledge_projected_outbox_failures(&queue, &sweep.outbox_failures)?;
    }
    if !sweep.expired_entries.is_empty() {
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

fn record_retryable_sweep_failure(
    queue: &ScreenEvidenceQueue,
    config: &ScreenAiRetentionSweeperRuntimeConfig,
    observed_at: &ScreenAiObservedAt,
) -> Result<Option<ScreenAiRetentionSweeperOutcome>, ActivityCaptureError> {
    let entries = queue.read_decrypted_entries(usize::MAX)?;
    let now = chrono::DateTime::parse_from_rfc3339(observed_at.0.as_str()).ok();
    let failures = entries
        .iter()
        .filter(|entry| {
            entry
                .expires_at
                .as_deref()
                .and_then(|expires_at| chrono::DateTime::parse_from_rfc3339(expires_at).ok())
                .zip(now.as_ref())
                .is_none_or(|(expires_at, now)| expires_at <= *now)
        })
        .map(|entry| {
            let mut deletion_proof_ref =
                String::from(SCREEN_SERVICE_RETENTION_DELETE_PROOF_ID_PREFIX);
            deletion_proof_ref.push_str(&entry.queue_job_id);
            ScreenEvidenceOutboxFailure {
                queue_job_id: entry.queue_job_id.clone(),
                malformed_record_digest: entry.image_digest.clone(),
                deletion_proof_ref,
            }
        })
        .collect::<Vec<_>>();
    (!failures.is_empty())
        .then(|| {
            let events = failures
                .iter()
                .map(|failure| outbox_failure_event(failure, observed_at.clone()))
                .collect::<Vec<_>>();
            record_activity_events_to_paths(
                &config.journal_path,
                &config.journal_key_path,
                &config.store_path,
                &events,
            )?;
            Ok(ScreenAiRetentionSweeperOutcome::NoExpired {
                pending_count: u64::try_from(entries.len()).unwrap_or(u64::MAX),
            })
        })
        .transpose()
}

impl ScreenAiRetentionSweeperRuntimeConfig {
    pub(crate) fn from_environment() -> Option<Self> {
        if !env_flag(
            ScreenAiEnvVar(SCREEN_SERVICE_RETENTION_SWEEPER_RUNTIME_ENABLED_ENV),
            true,
        ) {
            return None;
        }
        Some(Self {
            poll_seconds: env_u64(
                ScreenAiEnvVar(SCREEN_SERVICE_RETENTION_SWEEPER_POLL_SECONDS_ENV),
                DEFAULT_POLL_SECONDS,
            ),
            max_sweeps: env_optional_u64(ScreenAiEnvVar(
                SCREEN_SERVICE_RETENTION_SWEEPER_MAX_SWEEPS_ENV,
            )),
            max_ticks: env_optional_u64(ScreenAiEnvVar(
                SCREEN_SERVICE_RETENTION_SWEEPER_MAX_TICKS_ENV,
            )),
            queue_dir: env_path(ScreenAiEnvVar(SCREEN_SERVICE_QUEUE_DIR_ENV))
                .map(|path| path.0)
                .unwrap_or_else(|| {
                    let mut path = env::temp_dir();
                    path.push(SCREEN_SERVICE_DEFAULT_QUEUE_DIR_NAME);
                    path
                }),
            journal_path: activity_journal_path().into(),
            journal_key_path: activity_journal_key_path().into(),
            store_path: activity_db_path().into(),
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

fn expired_entry_event(
    entry: &ScreenEvidenceExpiredQueueEntry,
    observed_at: ScreenAiObservedAt,
) -> ActivityEvent {
    let ScreenAiObservedAt(observed_at) = observed_at;
    ActivityEvent {
        schema_version: parent_protocol::ACTIVITY_SCHEMA_VERSION,
        event_id: retention_event_id(entry).0,
        observed_at,
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
        fields: fields_from_pairs(retention_event_fields(entry).0),
        evidence: expired_entry_evidence(entry),
    }
}

fn expired_entry_evidence(entry: &ScreenEvidenceExpiredQueueEntry) -> Vec<ActivityEvidenceRef> {
    vec![ActivityEvidenceRef {
        evidence_id: retention_evidence_id(entry).0,
        kind: ActivityEvidenceKind::JournalEntry,
        digest: Some(entry.image_digest.clone()),
        uri: None,
    }]
}

fn retention_event_fields(entry: &ScreenEvidenceExpiredQueueEntry) -> RetentionEventFields {
    let retention_result_id = || {
        let mut id = String::from(SCREEN_SERVICE_RETENTION_RESULT_ID_PREFIX);
        id.push_str(&entry.queue_job_id);
        id
    };
    let string_field = |key: &'static str, value: String| (key, LogFieldValue::String(value));
    let number_field = |key: &'static str, value: f64| (key, LogFieldValue::Number(value));
    let bool_field = |key: &'static str, value: bool| (key, LogFieldValue::Boolean(value));
    RetentionEventFields(vec![
        string_field(
            constants::field::SCREEN_ANALYSIS_RESULT_ID,
            retention_result_id(),
        ),
        string_field(
            constants::field::SCREEN_QUEUE_JOB_ID,
            entry.queue_job_id.clone(),
        ),
        string_field(
            constants::field::SCREEN_SUMMARY,
            SCREEN_SERVICE_RETENTION_SUMMARY_EXPIRED_DELETED.to_string(),
        ),
        string_field(
            constants::field::SCREEN_PRIMARY_CATEGORY,
            SCREEN_CATEGORY_UNKNOWN.to_string(),
        ),
        number_field(
            constants::field::SCREEN_CONFIDENCE,
            SCREEN_SERVICE_METADATA_CONFIDENCE,
        ),
        string_field(
            constants::field::SCREEN_IMAGE_DELETION_STATE,
            SCREEN_DELETION_EXPIRED_DELETED.to_string(),
        ),
        string_field(
            constants::field::SCREEN_DELETION_REASONS,
            entry.deletion_proof_ref.clone(),
        ),
        bool_field(constants::field::SCREEN_POLICY_ELIGIBLE, false),
        string_field(
            constants::field::SCREEN_MODEL_RUNTIME_REF,
            SCREEN_SERVICE_RETENTION_MODEL_RUNTIME_REF.to_string(),
        ),
        string_field(
            constants::field::SCREEN_MODEL_ID,
            SCREEN_SERVICE_RETENTION_MODEL_ID.to_string(),
        ),
        string_field(
            constants::field::SCREEN_PROVIDER_KIND,
            SCREEN_PROVIDER_SERVICE_METADATA.to_string(),
        ),
        string_field(
            constants::field::SCREEN_TEMPLATE_VERSION,
            SCREEN_SERVICE_RETENTION_TEMPLATE_VERSION.to_string(),
        ),
        string_field(
            constants::field::SCREEN_CAPTURE_REASON,
            constants::activity_capture::SCREEN_TRIGGER_TIMED_CADENCE.to_string(),
        ),
        string_field(
            constants::field::SCREEN_CAPTURE_SCOPE,
            ocentra_parent_agent_protocol::screen_evidence::SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW
                .to_string(),
        ),
        string_field(
            constants::field::CAPABILITY_STATUS,
            SCREEN_CAPABILITY_READY.to_string(),
        ),
        string_field(
            constants::field::SCREEN_IMAGE_DIGEST,
            entry.image_digest.clone(),
        ),
        string_field(
            constants::field::SCREEN_CUSTODY_STATE,
            SCREEN_CUSTODY_JOURNAL.to_string(),
        ),
    ])
}

fn retention_event_id(entry: &ScreenEvidenceExpiredQueueEntry) -> RetentionEventId {
    let mut id = String::from(SCREEN_SERVICE_RETENTION_EVENT_ID_PREFIX);
    id.push_str(&entry.queue_job_id);
    RetentionEventId(id)
}

fn retention_evidence_id(entry: &ScreenEvidenceExpiredQueueEntry) -> RetentionEvidenceId {
    let mut id = String::from(SCREEN_SERVICE_RETENTION_EVIDENCE_ID_PREFIX);
    id.push_str(&entry.queue_job_id);
    RetentionEvidenceId(id)
}
