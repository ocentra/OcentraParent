#![forbid(unsafe_code)]

#[path = "../src/activity_store_path.rs"]
mod activity_store_path;
#[path = "../src/fields.rs"]
mod fields;
mod json_contract {
    use ocentra_parent_agent_protocol::constants;
    use serde::Serialize;
    use serde_json::Value;

    pub(crate) fn serialize_json_string<T>(value: &T) -> String
    where
        T: Serialize + ?Sized,
    {
        match serde_json::to_string(value) {
            Ok(serialized) => serialized,
            Err(error) => unreachable!("{}: {error}", constants::error::AGENT_EVENT_SERIALIZES),
        }
    }

    pub(crate) fn serialize_json_value<T>(value: T) -> Value
    where
        T: Serialize,
    {
        match serde_json::to_value(value) {
            Ok(serialized) => serialized,
            Err(error) => unreachable!("{}: {error}", constants::error::AGENT_EVENT_SERIALIZES),
        }
    }
}
#[path = "unit/screen_ai_analysis_runtime_tests.rs"]
mod screen_ai_analysis_runtime_tests;
#[path = "../src/screen_ai_cadence_runtime.rs"]
mod screen_ai_cadence_runtime;
#[path = "../src/screen_ai_cadence_runtime_event.rs"]
mod screen_ai_cadence_runtime_event;
#[path = "unit/screen_ai_cadence_runtime_tests.rs"]
mod screen_ai_cadence_runtime_tests;
#[path = "../src/screen_ai_foreground_runtime.rs"]
mod screen_ai_foreground_runtime;
#[path = "../src/screen_ai_foreground_runtime_config.rs"]
mod screen_ai_foreground_runtime_config;
#[path = "unit/screen_ai_foreground_runtime_tests.rs"]
mod screen_ai_foreground_runtime_tests;
#[path = "../src/screen_ai_retention_sweeper_deletion_events.rs"]
mod screen_ai_retention_sweeper_deletion_events;
#[path = "unit/screen_ai_retention_sweeper_deletion_events_tests.rs"]
mod screen_ai_retention_sweeper_deletion_events_tests;
#[path = "../src/screen_ai_retention_sweeper_runtime.rs"]
mod screen_ai_retention_sweeper_runtime;
#[path = "unit/screen_ai_retention_sweeper_runtime_tests.rs"]
mod screen_ai_retention_sweeper_runtime_tests;
#[path = "unit/screen_ai_runtime_clippy_linkage_tests.rs"]
mod screen_ai_runtime_clippy_linkage_tests;
mod screen_ai_service_capture_event_builder {
    use ocentra_parent_agent_protocol::activity::ActivityEvent;
    use ocentra_parent_agent_protocol::activity::ActivityEventKind;
    use ocentra_parent_agent_protocol::activity::ActivityEvidenceKind;
    use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
    use ocentra_parent_agent_protocol::activity::ActivityObserver;
    use ocentra_parent_agent_protocol::activity::ActivitySource;
    use ocentra_parent_agent_protocol::activity::ActivitySubject;
    use ocentra_parent_agent_protocol::activity::ActivitySubjectKind;
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::logging::LogFieldValue;
    use ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisQueueJob;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CATEGORY_UNKNOWN;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CUSTODY_JOURNAL;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_CUSTODY_TEMP_QUEUE;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_DELETION_DELETED;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_IMAGE_FORMAT_PNG;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_PROVIDER_SERVICE_METADATA;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_QUEUE_STATUS_QUEUED;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_ADAPTER_ID;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_LOCAL_USER_REF;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_METADATA_CONFIDENCE;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_MODEL_RUNTIME_REF;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_SERVICE_PARENT_SETTING_REF;
    use ocentra_parent_agent_protocol::ACTIVITY_SCHEMA_VERSION;
    use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;
    use ocentra_parent_screen_capture_adapter::CapturedScreenImage;

    use crate::{
        fields::fields_from_pairs, screen_ai_cadence_runtime_event::ScreenAiServiceCaptureRecord,
    };

    const DEFAULT_MAX_RETRY_COUNT: u64 = 0;
    const DEFAULT_SETTING_VERSION: u64 = 1;

    pub(crate) struct ScreenAiServiceCaptureIds {
        pub(crate) queue_job_id: String,
        result_id: String,
        event_id: String,
        evidence_id: String,
    }

    impl ScreenAiServiceCaptureIds {
        pub(crate) fn new(
            queue_job_id_prefix: &str,
            result_id_prefix: &str,
            event_id_prefix: &str,
            evidence_id_prefix: &str,
            epoch_seconds: u64,
            sequence_index: u64,
        ) -> Self {
            Self {
                queue_job_id: suffixed_id(queue_job_id_prefix, epoch_seconds, sequence_index),
                result_id: suffixed_id(result_id_prefix, epoch_seconds, sequence_index),
                event_id: suffixed_id(event_id_prefix, epoch_seconds, sequence_index),
                evidence_id: suffixed_id(evidence_id_prefix, epoch_seconds, sequence_index),
            }
        }
    }

    pub(crate) fn screen_queue_job(
        record: &ScreenAiServiceCaptureRecord<'_>,
        ids: &ScreenAiServiceCaptureIds,
        image_digest: &str,
    ) -> ScreenAnalysisQueueJob {
        ScreenAnalysisQueueJob {
            schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
            queue_job_id: ids.queue_job_id.clone(),
            created_at: record.clock.timestamp.clone(),
            not_before: record.clock.timestamp.clone(),
            expires_at: record
                .clock
                .expires_after_seconds(record.temporary_image_ttl_seconds),
            last_attempt_at: None,
            capture_reason: record.capture_reason.to_string(),
            capture_scope: SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW.to_string(),
            source_id: record.source_id.to_string(),
            adapter_id: SCREEN_SERVICE_ADAPTER_ID.to_string(),
            device_ref: constants::peer::LOCAL_DEV_AGENT.to_string(),
            local_user_ref: SCREEN_SERVICE_LOCAL_USER_REF.to_string(),
            parent_setting_ref: SCREEN_SERVICE_PARENT_SETTING_REF.to_string(),
            setting_version: DEFAULT_SETTING_VERSION,
            related_evidence_refs: Vec::new(),
            encrypted_image_ref: record.paths.queue_dir.to_string_lossy().to_string(),
            image_digest: image_digest.to_string(),
            image_byte_size: record.image.png_bytes.len() as u64,
            image_format: SCREEN_IMAGE_FORMAT_PNG.to_string(),
            status: SCREEN_QUEUE_STATUS_QUEUED.to_string(),
            attempt_count: 0,
            max_retry_count: DEFAULT_MAX_RETRY_COUNT,
            failure_reason: None,
            unavailable_reason: None,
            deletion_required: true,
            deleted_at: None,
            deletion_status: SCREEN_DELETION_DELETED.to_string(),
            deletion_proof_ref: None,
            custody_state: SCREEN_CUSTODY_TEMP_QUEUE.to_string(),
        }
    }

    pub(crate) fn screen_analysis_event(
        record: &ScreenAiServiceCaptureRecord<'_>,
        ids: &ScreenAiServiceCaptureIds,
        job: &ScreenAnalysisQueueJob,
        image_digest: &str,
    ) -> ActivityEvent {
        let evidence = screen_analysis_evidence(ids, job, image_digest);
        ActivityEvent {
            schema_version: ACTIVITY_SCHEMA_VERSION,
            event_id: ids.event_id.clone(),
            observed_at: record.clock.timestamp.clone(),
            source: ActivitySource {
                device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                platform: std::env::consts::OS.to_string(),
                observer: ActivityObserver::LocalAi,
                source_id: record.source_id.to_string(),
            },
            kind: ActivityEventKind::ScreenAnalysisSummarized,
            subject: ActivitySubject {
                kind: ActivitySubjectKind::Device,
                subject_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
                display_name: record.image.metadata.title.clone(),
            },
            fields: fields_from_pairs(screen_analysis_fields(record, ids, job, image_digest)),
            evidence,
        }
    }

    fn screen_analysis_evidence(
        ids: &ScreenAiServiceCaptureIds,
        job: &ScreenAnalysisQueueJob,
        image_digest: &str,
    ) -> Vec<ActivityEvidenceRef> {
        vec![ActivityEvidenceRef {
            evidence_id: ids.evidence_id.clone(),
            kind: ActivityEvidenceKind::Screenshot,
            digest: Some(image_digest.to_string()),
            uri: Some(job.encrypted_image_ref.clone()),
        }]
    }

    fn screen_analysis_fields(
        record: &ScreenAiServiceCaptureRecord<'_>,
        ids: &ScreenAiServiceCaptureIds,
        job: &ScreenAnalysisQueueJob,
        image_digest: &str,
    ) -> Vec<(&'static str, LogFieldValue)> {
        let mut fields = Vec::new();
        fields.extend(screen_analysis_identity_fields(record, ids, job));
        fields.extend(screen_analysis_model_fields(record));
        fields.extend(screen_analysis_capture_fields(
            job,
            image_digest,
            record.image,
        ));
        fields
    }

    fn screen_analysis_identity_fields(
        record: &ScreenAiServiceCaptureRecord<'_>,
        ids: &ScreenAiServiceCaptureIds,
        job: &ScreenAnalysisQueueJob,
    ) -> Vec<(&'static str, LogFieldValue)> {
        vec![
            string_field(
                constants::field::SCREEN_ANALYSIS_RESULT_ID,
                ids.result_id.clone(),
            ),
            string_field(
                constants::field::SCREEN_QUEUE_JOB_ID,
                job.queue_job_id.clone(),
            ),
            string_field(constants::field::SCREEN_SUMMARY, record.summary),
            string_field(
                constants::field::SCREEN_PRIMARY_CATEGORY,
                SCREEN_CATEGORY_UNKNOWN,
            ),
        ]
    }

    fn screen_analysis_model_fields(
        record: &ScreenAiServiceCaptureRecord<'_>,
    ) -> Vec<(&'static str, LogFieldValue)> {
        vec![
            number_field(
                constants::field::SCREEN_CONFIDENCE,
                SCREEN_SERVICE_METADATA_CONFIDENCE,
            ),
            string_field(
                constants::field::SCREEN_IMAGE_DELETION_STATE,
                SCREEN_DELETION_DELETED,
            ),
            bool_field(constants::field::SCREEN_POLICY_ELIGIBLE, false),
            string_field(
                constants::field::SCREEN_MODEL_RUNTIME_REF,
                SCREEN_SERVICE_MODEL_RUNTIME_REF,
            ),
            string_field(constants::field::SCREEN_MODEL_ID, record.model_id),
            string_field(
                constants::field::SCREEN_PROVIDER_KIND,
                SCREEN_PROVIDER_SERVICE_METADATA,
            ),
            string_field(
                constants::field::SCREEN_TEMPLATE_VERSION,
                record.template_version,
            ),
        ]
    }

    fn screen_analysis_capture_fields(
        job: &ScreenAnalysisQueueJob,
        image_digest: &str,
        image: &CapturedScreenImage,
    ) -> Vec<(&'static str, LogFieldValue)> {
        vec![
            string_field(
                constants::field::SCREEN_CAPTURE_REASON,
                job.capture_reason.clone(),
            ),
            string_field(
                constants::field::SCREEN_CAPTURE_SCOPE,
                job.capture_scope.clone(),
            ),
            string_field(
                constants::field::CAPABILITY_STATUS,
                image.metadata.status.as_protocol_str(),
            ),
            string_field(
                constants::field::SCREEN_IMAGE_DIGEST,
                image_digest.to_string(),
            ),
            string_field(
                constants::field::SCREEN_CUSTODY_STATE,
                SCREEN_CUSTODY_JOURNAL,
            ),
        ]
    }

    fn suffixed_id(prefix: &str, epoch_seconds: u64, tick_index: u64) -> String {
        let mut id = String::from(prefix);
        id.push_str(&epoch_seconds.to_string());
        id.push(constants::delimiter::HYPHEN);
        id.push_str(&tick_index.to_string());
        id
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
}
#[path = "../src/screen_ai_service_event_bridge.rs"]
mod screen_ai_service_event_bridge;
#[path = "unit/screen_ai_service_event_bridge_tests.rs"]
mod screen_ai_service_event_bridge_tests;
#[path = "../src/screen_ai_service_event_subscription.rs"]
pub(crate) mod screen_ai_service_event_subscription;
#[path = "unit/screen_ai_service_event_subscription_tests.rs"]
mod screen_ai_service_event_subscription_tests;
#[path = "support/test_invariants.rs"]
mod test_invariants;
#[path = "../src/time.rs"]
mod time;

mod activity_capture {
    use std::{fs, path::Path};

    use ocentra_parent_agent_core::{
        activity_store::ActivityStore,
        journal::ActivityJournal,
        journal_crypto::{JournalKey, JOURNAL_KEY_BYTES},
    };
    use ocentra_parent_agent_protocol::activity::ActivityEvent;

    #[path = "../../src/activity_capture/errors.rs"]
    mod errors;

    pub(crate) type ActivityCaptureError = errors::ActivityCaptureError;

    pub(crate) fn record_activity_events_to_paths(
        journal_path: &Path,
        key_path: &Path,
        store_path: &Path,
        events: &[ActivityEvent],
    ) -> Result<
        ocentra_parent_agent_protocol::activity_query::ActivityIngestStatus,
        ActivityCaptureError,
    > {
        let key = load_or_create_journal_key(key_path)?;
        let mut journal = ActivityJournal::open(journal_path.to_path_buf(), key)?;
        let existing_line_count = journal.lines()?.len();
        for event in events {
            journal.append(event)?;
        }
        let mut appended_events = Vec::new();
        for line in journal.lines()?.into_iter().skip(existing_line_count) {
            appended_events.push(journal.decrypt_line(&line)?);
        }
        let store = ActivityStore::open(store_path)?;
        Ok(store.ingest_events(&appended_events)?)
    }

    fn load_or_create_journal_key(path: &Path) -> Result<JournalKey, ActivityCaptureError> {
        match fs::read(path) {
            Ok(bytes) => journal_key_from_bytes(&bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                let key = JournalKey::generate();
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::write(path, key.as_bytes())?;
                Ok(key)
            }
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
}

mod activity_surface_read_models {
    use ocentra_parent_agent_protocol::activity_surface::ActivityScreenReadModelRow;
    use ocentra_parent_agent_protocol::screen_evidence::ScreenAnalysisResult;
    use ocentra_parent_agent_protocol::{constants, ACTIVITY_SURFACE_SCHEMA_VERSION};

    pub(crate) fn activity_screen_row_from_result(
        result: ScreenAnalysisResult,
    ) -> ActivityScreenReadModelRow {
        let _ = ACTIVITY_SURFACE_SCHEMA_VERSION;
        ActivityScreenReadModelRow {
            row_id: result.screen_analysis_result_id,
            label: result.summary,
            device_id: constants::activity_surface::DEFAULT_DEVICE_ID.to_string(),
            state: ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState::Ready,
            total_ms: 0,
            foreground_ms: 0,
            background_ms: 0,
            capture_reason: result.capture_reason,
            capture_scope: result.capture_scope,
            capability_status: result.capability_status,
            queue_job_id: result.queue_job_id,
            model_runtime_ref: result.model_runtime_ref,
            model_id: result.model_id,
            provider_kind: result.provider_kind,
            prompt_or_template_version: result.prompt_or_template_version,
            primary_category: result.primary_category,
            confidence: result.confidence,
            image_deletion_state: result.image_deletion_state,
            raw_image_retained: result.raw_image_retained,
            policy_eligible: result.policy_eligible,
            image_digest: result.image_digest,
            custody_state: result.custody_state,
            evidence: result.source_evidence_refs,
            policy_decision_ref: result.policy_decision_ref,
            policy_action: result.policy_action,
            policy_reason_codes: result.policy_reason_codes,
            parent_rule_refs: result.parent_rule_refs,
            local_model_runtime_refs: result.local_model_runtime_refs,
            parent_explanation_refs: result.parent_explanation_refs,
            explanation_reasons: result.explanation_reasons,
            deletion_reasons: result.deletion_reasons,
            ocr_text_snippets: result.ocr_text_snippets,
            redaction_notes: result.redaction_notes,
        }
    }
}

mod screen_ai_analysis_runtime {
    #[path = "../../src/screen_ai_analysis_runtime/adapter.rs"]
    pub(crate) mod adapter;
    #[path = "../../src/screen_ai_analysis_runtime/adapter_output_fields.rs"]
    pub(crate) mod adapter_output_fields;
    #[path = "../../src/screen_ai_analysis_runtime/adapter_process.rs"]
    pub(crate) mod adapter_process;
    #[path = "../../src/screen_ai_analysis_runtime/adapter_redaction.rs"]
    pub(crate) mod adapter_redaction;
    #[path = "../unit/screen_ai_analysis_runtime_adapter_tests.rs"]
    mod adapter_tests;
    #[path = "../../src/screen_ai_analysis_runtime/config.rs"]
    pub(crate) mod config;
    #[path = "../../src/screen_ai_analysis_runtime/event_record.rs"]
    pub(crate) mod event_record;
    #[path = "../unit/screen_ai_analysis_runtime_event_record_tests.rs"]
    mod event_record_tests;
    #[path = "../../src/screen_ai_analysis_runtime/queue.rs"]
    pub(crate) mod queue;

    type ScreenAiAnalysisCycleClock = config::ScreenAiAnalysisCycleClock;
    type ScreenAiAnalysisCycleOutcome = config::ScreenAiAnalysisCycleOutcome;
    type ScreenAiAnalysisRuntimeConfig = config::ScreenAiAnalysisRuntimeConfig;

    use ocentra_parent_agent_core::{
        activity_store::ActivityStore, screen_evidence_queue::ScreenEvidenceQueue,
    };
    use ocentra_parent_agent_protocol::activity_surface::ActivityScreenReadModelRow;
    use ocentra_parent_agent_protocol::constants;
    use ocentra_parent_agent_protocol::screen_evidence::SCREEN_PROVIDER_SERVICE_METADATA;

    use crate::{
        activity_capture::{record_activity_events_to_paths, ActivityCaptureError},
        activity_surface_read_models::activity_screen_row_from_result,
        screen_ai_service_event_subscription::ScreenAiServiceEventRuntime,
    };

    use ocentra_parent_agent_protocol::local_ai_runtime::status::LocalModelRuntimeStatus;

    use self::queue::{
        first_queued_screen_image, load_existing_screen_key, metadata_result_for_queue_job,
    };

    pub(crate) async fn record_screen_ai_analysis_cycle_with_events(
        config: &ScreenAiAnalysisRuntimeConfig,
        clock: ScreenAiAnalysisCycleClock,
        event_runtime: Option<&ScreenAiServiceEventRuntime>,
    ) -> Result<ScreenAiAnalysisCycleOutcome, ActivityCaptureError> {
        if !config.screen_analysis_enabled {
            return Ok(ScreenAiAnalysisCycleOutcome::Suppressed);
        }
        let Some(key) = load_existing_screen_key(&config.journal_key_path)? else {
            return Ok(ScreenAiAnalysisCycleOutcome::QueueEmpty);
        };
        let queue = ScreenEvidenceQueue::open(&config.queue_dir, key)?;
        let Some(image) = first_queued_screen_image(&queue, config.max_queue_scan)? else {
            return Ok(ScreenAiAnalysisCycleOutcome::QueueEmpty);
        };
        let metadata =
            metadata_result_for_queue_job(&config.store_path, &image.queue_job_id, &clock)?;
        if metadata
            .as_ref()
            .is_some_and(|result| result.provider_kind != SCREEN_PROVIDER_SERVICE_METADATA)
        {
            queue.remove_entries(std::slice::from_ref(&image.queue_job_id))?;
            return Ok(ScreenAiAnalysisCycleOutcome::AlreadyAnalyzed {
                queue_job_id: image.queue_job_id,
            });
        }

        let generation = adapter::run_adapter(config, &image, metadata.as_ref()).await;
        let event_record = event_record::analysis_event_record(
            &image,
            metadata.as_ref(),
            &clock,
            &generation,
            &config.ocr_redaction_policy,
        );
        let outcome =
            event_record::outcome_for_generation(&image.queue_job_id, &generation, &event_record);
        record_activity_events_to_paths(
            &config.journal_path,
            &config.journal_key_path,
            &config.store_path,
            &[event_record::screen_analysis_event(&event_record)],
        )?;
        if let (Some(runtime), Some(row)) = (
            event_runtime,
            latest_analysis_row_for_queue_job(config, &image.queue_job_id, &clock.timestamp)?,
        ) {
            let _ = runtime
                .publish_row_ready(
                    row,
                    constants::screen_flow::SCREEN_ACTION_EVENT_REF,
                    &clock.timestamp,
                )
                .await;
        }
        queue.remove_entries(std::slice::from_ref(&image.queue_job_id))?;
        Ok(outcome)
    }

    pub(crate) fn adapter_runtime_status(
        command: Option<&std::path::Path>,
        timestamp: &str,
    ) -> LocalModelRuntimeStatus {
        adapter::runtime_status(command, timestamp)
    }

    fn latest_analysis_row_for_queue_job(
        config: &ScreenAiAnalysisRuntimeConfig,
        queue_job_id: &str,
        generated_at: &str,
    ) -> Result<Option<ActivityScreenReadModelRow>, ActivityCaptureError> {
        let store = ActivityStore::open(&config.store_path)?;
        let summary = store.screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            generated_at,
        )?;
        Ok(summary
            .results
            .into_iter()
            .find(|result| {
                result.queue_job_id == queue_job_id
                    && result.provider_kind != SCREEN_PROVIDER_SERVICE_METADATA
            })
            .map(activity_screen_row_from_result))
    }
}

mod screen_ai_service_event_subscription_live_view {
    use super::screen_ai_service_event_subscription::live_view_service_runtime;
    #[path = "../unit/live_view_service_runtime_tests.rs"]
    mod live_view_service_runtime_tests;
}
