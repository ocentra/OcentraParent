use std::fmt::Display;

use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef, ActivityObserver,
    ActivitySource, ActivitySubject, ActivitySubjectKind, ACTIVITY_SCHEMA_VERSION,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::screen_evidence::{
    SCREEN_CAPABILITY_READY, SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST,
    SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW, SCREEN_CATEGORY_SCHOOL, SCREEN_CUSTODY_JOURNAL,
    SCREEN_DELETION_DELETED, SCREEN_DELETION_DELETE_FAILED, SCREEN_DELETION_EXPIRED_DELETED,
    SCREEN_DELETION_REQUIRED, SCREEN_POLICY_CONFIDENCE_READY, SCREEN_PROVIDER_LOCAL_VISION,
    SCREEN_QUEUE_STATUS_FAILED,
};

use crate::test_text::{test_some as some, TestResult, TestText};
use crate::ActivityStore;

#[test]
fn activity_store_reports_screen_summary_from_local_ai_events() -> TestResult {
    let store = ActivityStore::open_in_memory().map_err(|error| {
        TestText::from_display(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;

    store
        .ingest_events(&[screen_summary_event()])
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_INGESTS
            ))
        })?;
    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_QUERIES
            ))
        })?;

    assert_eq!(summary.returned, 1);
    assert_eq!(
        summary.generated_at,
        constants::activity_store::TEST_SECOND_OBSERVED_AT
    );
    assert_eq!(
        summary.latest_result_id,
        Some(constants::activity_store::TEST_SCREEN_RESULT_ID.to_string())
    );
    assert_eq!(
        summary.latest_primary_category,
        Some(SCREEN_CATEGORY_SCHOOL.to_string())
    );
    assert_eq!(
        summary.latest_confidence,
        Some(SCREEN_POLICY_CONFIDENCE_READY)
    );
    assert_eq!(
        summary.latest_image_deletion_state,
        Some(SCREEN_DELETION_DELETED.to_string())
    );
    assert_eq!(summary.latest_policy_eligible, Some(true));
    let result = some(
        summary.results.first(),
        constants::error::ACTIVITY_STORE_QUERIES,
    )?;
    assert_eq!(
        result.policy_decision_ref,
        Some(constants::activity_store::TEST_POLICY_DECISION_ID.to_string())
    );
    assert_eq!(
        result.policy_action,
        Some(constants::activity_store::TEST_POLICY_ACTION_ALLOW.to_string())
    );
    assert_eq!(
        result.parent_rule_refs,
        vec![constants::activity_store::TEST_POLICY_RULE_ID.to_string()]
    );
    assert_eq!(
        result.parent_explanation_refs,
        vec![constants::activity_store::TEST_PARENT_EXPLANATION_ID.to_string()]
    );
    assert_eq!(
        result.local_model_runtime_refs,
        vec![constants::activity_store::TEST_SCREEN_MODEL_RUNTIME_REF.to_string()]
    );
    assert_eq!(
        result.ocr_text_snippets,
        vec![constants::activity_store::TEST_SCREEN_OCR_SNIPPET_REDACTED.to_string()]
    );
    assert_eq!(
        result.redaction_notes,
        vec![constants::activity_store::TEST_SCREEN_REDACTION_NOTE_PII.to_string()]
    );
    Ok(())
}

#[test]
fn activity_store_reports_empty_screen_summary_without_inventing_results() -> TestResult {
    let store = ActivityStore::open_in_memory().map_err(|error| {
        TestText::from_display(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;

    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_QUERIES
            ))
        })?;

    assert_eq!(summary.returned, 0);
    assert_eq!(summary.latest_result_id, None);
    assert_eq!(summary.queue_health.latest_status, None);
    Ok(())
}

#[test]
fn activity_store_surfaces_screen_delete_failed_queue_health() -> TestResult {
    let store = ActivityStore::open_in_memory().map_err(|error| {
        TestText::from_display(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;

    store
        .ingest_events(&[delete_failed_screen_summary_event()])
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_INGESTS
            ))
        })?;
    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_QUERIES
            ))
        })?;

    assert_eq!(summary.returned, 1);
    assert_eq!(summary.queue_health.delete_failed_count, 1);
    assert_eq!(
        summary.queue_health.latest_status,
        Some(SCREEN_QUEUE_STATUS_FAILED.to_string())
    );
    assert_eq!(
        summary.latest_image_deletion_state,
        Some(SCREEN_DELETION_DELETE_FAILED.to_string())
    );
    assert_eq!(summary.latest_policy_eligible, Some(false));
    Ok(())
}

#[test]
fn activity_store_queue_health_uses_latest_deletion_state_per_job() -> TestResult {
    let store = ActivityStore::open_in_memory().map_err(|error| {
        TestText::from_display(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;
    let pending = screen_event_with_deletion_state(
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        "screen-health-pending",
        SCREEN_DELETION_REQUIRED,
    );
    let deleted = screen_event_with_deletion_state(
        constants::activity_store::TEST_THIRD_OBSERVED_AT,
        "screen-health-deleted",
        SCREEN_DELETION_DELETED,
    );
    let expired = screen_event_with_deletion_state(
        constants::activity_store::TEST_THIRD_OBSERVED_AT,
        "screen-health-expired",
        SCREEN_DELETION_EXPIRED_DELETED,
    );

    store
        .ingest_events(&[pending, deleted, expired])
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_INGESTS
            ))
        })?;
    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        )
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_QUERIES
            ))
        })?;

    assert_eq!(summary.queue_health.delete_pending_count, 0);
    assert_eq!(summary.queue_health.expired_count, 1);
    assert_eq!(summary.queue_health.delete_failed_count, 0);
    Ok(())
}

#[test]
fn activity_store_skips_incomplete_screen_summary_rows() -> TestResult {
    let store = ActivityStore::open_in_memory().map_err(|error| {
        TestText::from_display(format!(
            "{}: {error:?}",
            constants::error::ACTIVITY_STORE_OPENS
        ))
    })?;

    store
        .ingest_events(&[incomplete_screen_summary_event()])
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_INGESTS
            ))
        })?;
    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .map_err(|error| {
            TestText::from_display(format!(
                "{}: {error:?}",
                constants::error::ACTIVITY_STORE_QUERIES
            ))
        })?;

    assert_eq!(summary.returned, 0);
    assert_eq!(summary.results.len(), 0);
    assert_eq!(summary.latest_result_id, None);
    assert_eq!(summary.latest_summary, None);
    assert_eq!(summary.latest_primary_category, None);
    assert_eq!(summary.latest_policy_eligible, None);
    assert_eq!(summary.queue_health.latest_status, None);
    assert_eq!(summary.evidence.len(), 0);
    Ok(())
}

fn screen_summary_event() -> ActivityEvent {
    let mut fields = LogFields::new();
    insert_screen_summary_core_fields(&mut fields);
    insert_screen_summary_capture_fields(&mut fields);
    insert_screen_summary_policy_fields(&mut fields);

    screen_event(fields)
}

fn delete_failed_screen_summary_event() -> ActivityEvent {
    let mut fields = LogFields::new();
    insert_screen_summary_core_fields(&mut fields);
    insert_screen_summary_capture_fields(&mut fields);
    insert_log_field(
        &mut fields,
        constants::field::SCREEN_IMAGE_DELETION_STATE,
        LogFieldValue::String(SCREEN_DELETION_DELETE_FAILED.to_string()),
    );
    insert_log_field(
        &mut fields,
        constants::field::SCREEN_POLICY_ELIGIBLE,
        LogFieldValue::Boolean(false),
    );

    screen_event(fields)
}

fn screen_event_with_deletion_state(
    observed_at: &str,
    event_id: &str,
    deletion_state: &str,
) -> ActivityEvent {
    let mut event = screen_summary_event();
    event.observed_at = observed_at.to_string();
    event.event_id = event_id.to_string();
    insert_log_field(
        &mut event.fields,
        constants::field::SCREEN_IMAGE_DELETION_STATE,
        LogFieldValue::String(deletion_state.to_string()),
    );
    event
}

fn insert_screen_summary_core_fields(fields: &mut LogFields) {
    insert_log_field(
        fields,
        constants::field::SCREEN_ANALYSIS_RESULT_ID,
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_RESULT_ID.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_QUEUE_JOB_ID,
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_SUMMARY,
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_SUMMARY.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_PRIMARY_CATEGORY,
        LogFieldValue::String(SCREEN_CATEGORY_SCHOOL.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_CONFIDENCE,
        LogFieldValue::Number(SCREEN_POLICY_CONFIDENCE_READY),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_IMAGE_DELETION_STATE,
        LogFieldValue::String(SCREEN_DELETION_DELETED.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_POLICY_ELIGIBLE,
        LogFieldValue::Boolean(true),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_OCR_TEXT_SNIPPETS,
        LogFieldValue::String(
            constants::activity_store::TEST_SCREEN_OCR_SNIPPET_REDACTED.to_string(),
        ),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_REDACTION_NOTES,
        LogFieldValue::String(
            constants::activity_store::TEST_SCREEN_REDACTION_NOTE_PII.to_string(),
        ),
    );
}

fn insert_screen_summary_capture_fields(fields: &mut LogFields) {
    insert_log_field(
        fields,
        constants::field::SCREEN_MODEL_RUNTIME_REF,
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_MODEL_RUNTIME_REF.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_MODEL_ID,
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_MODEL_ID.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_PROVIDER_KIND,
        LogFieldValue::String(SCREEN_PROVIDER_LOCAL_VISION.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_TEMPLATE_VERSION,
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_TEMPLATE_VERSION.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_CAPTURE_REASON,
        LogFieldValue::String(SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_CAPTURE_SCOPE,
        LogFieldValue::String(SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::CAPABILITY_STATUS,
        LogFieldValue::String(SCREEN_CAPABILITY_READY.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_IMAGE_DIGEST,
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_CUSTODY_STATE,
        LogFieldValue::String(SCREEN_CUSTODY_JOURNAL.to_string()),
    );
}

fn insert_screen_summary_policy_fields(fields: &mut LogFields) {
    insert_log_field(
        fields,
        constants::field::POLICY_DECISION_ID,
        LogFieldValue::String(constants::activity_store::TEST_POLICY_DECISION_ID.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::POLICY_ACTION,
        LogFieldValue::String(constants::activity_store::TEST_POLICY_ACTION_ALLOW.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::POLICY_REASON_CODES,
        LogFieldValue::String(constants::activity_store::TEST_POLICY_REASON_CODES.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::POLICY_RULE_IDS,
        LogFieldValue::String(constants::activity_store::TEST_POLICY_RULE_ID.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_PARENT_EXPLANATION_REFS,
        LogFieldValue::String(constants::activity_store::TEST_PARENT_EXPLANATION_ID.to_string()),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_EXPLANATION_REASONS,
        LogFieldValue::String(
            constants::activity_store::TEST_SCREEN_EXPLANATION_REASONS.to_string(),
        ),
    );
    insert_log_field(
        fields,
        constants::field::SCREEN_DELETION_REASONS,
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_DELETION_REASONS.to_string()),
    );
}

fn insert_log_field(fields: &mut LogFields, name: impl Display, value: LogFieldValue) {
    fields.insert(name.to_string(), value);
}

fn incomplete_screen_summary_event() -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::SCREEN_ANALYSIS_RESULT_ID.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_RESULT_ID.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_SUMMARY.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_SUMMARY.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_POLICY_ELIGIBLE.to_string(),
        LogFieldValue::Boolean(true),
    );

    screen_event(fields)
}

fn screen_event(fields: LogFields) -> ActivityEvent {
    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: constants::event_id::ACTIVITY_RECENT_SUMMARY_REPORTED.to_string(),
        observed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::LocalAi,
            source_id: constants::activity_store::TEST_SCREEN_SOURCE_ID.to_string(),
        },
        kind: ActivityEventKind::ScreenAnalysisSummarized,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Device,
            subject_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            display_name: None,
        },
        fields,
        evidence: vec![ActivityEvidenceRef {
            evidence_id: constants::activity_store::TEST_JOURNAL_SUFFIX.to_string(),
            kind: ActivityEvidenceKind::JournalEntry,
            digest: Some(constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string()),
            uri: None,
        }],
    }
}
