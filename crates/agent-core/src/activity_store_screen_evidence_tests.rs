use ocentra_parent_agent_protocol::{
    constants, ActivityEvent, ActivityEventKind, ActivityEvidenceKind, ActivityEvidenceRef,
    ActivityObserver, ActivitySource, ActivitySubject, ActivitySubjectKind, LogFieldValue,
    LogFields, SCREEN_CAPABILITY_READY, SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST,
    SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW, SCREEN_CATEGORY_SCHOOL, SCREEN_CUSTODY_JOURNAL,
    SCREEN_DELETION_DELETED, SCREEN_POLICY_CONFIDENCE_READY, SCREEN_PROVIDER_LOCAL_VISION,
};

use super::ActivityStore;

#[test]
fn activity_store_reports_screen_summary_from_local_ai_events() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);

    store
        .ingest_events(&[screen_summary_event()])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

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
}

#[test]
fn activity_store_reports_empty_screen_summary_without_inventing_results() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);

    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(summary.returned, 0);
    assert_eq!(summary.latest_result_id, None);
    assert_eq!(summary.queue_health.latest_status, None);
}

#[test]
fn activity_store_skips_incomplete_screen_summary_rows() {
    let store = ActivityStore::open_in_memory().expect(constants::error::ACTIVITY_STORE_OPENS);

    store
        .ingest_events(&[incomplete_screen_summary_event()])
        .expect(constants::error::ACTIVITY_STORE_INGESTS);
    let summary = store
        .screen_evidence_recent_summary(
            constants::activity_store::DEFAULT_RECENT_LIMIT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .expect(constants::error::ACTIVITY_STORE_QUERIES);

    assert_eq!(summary.returned, 0);
    assert_eq!(summary.results.len(), 0);
    assert_eq!(summary.latest_result_id, None);
    assert_eq!(summary.latest_summary, None);
    assert_eq!(summary.latest_primary_category, None);
    assert_eq!(summary.latest_policy_eligible, None);
    assert_eq!(summary.queue_health.latest_status, None);
    assert_eq!(summary.evidence.len(), 0);
}

fn screen_summary_event() -> ActivityEvent {
    let mut fields = LogFields::new();
    fields.insert(
        constants::field::SCREEN_ANALYSIS_RESULT_ID.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_RESULT_ID.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_QUEUE_JOB_ID.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_SUMMARY.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_SUMMARY.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_PRIMARY_CATEGORY.to_string(),
        LogFieldValue::String(SCREEN_CATEGORY_SCHOOL.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_CONFIDENCE.to_string(),
        LogFieldValue::Number(SCREEN_POLICY_CONFIDENCE_READY),
    );
    fields.insert(
        constants::field::SCREEN_IMAGE_DELETION_STATE.to_string(),
        LogFieldValue::String(SCREEN_DELETION_DELETED.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_POLICY_ELIGIBLE.to_string(),
        LogFieldValue::Boolean(true),
    );
    fields.insert(
        constants::field::SCREEN_MODEL_RUNTIME_REF.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_MODEL_RUNTIME_REF.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_MODEL_ID.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_MODEL_ID.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_PROVIDER_KIND.to_string(),
        LogFieldValue::String(SCREEN_PROVIDER_LOCAL_VISION.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_TEMPLATE_VERSION.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_TEMPLATE_VERSION.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_CAPTURE_REASON.to_string(),
        LogFieldValue::String(SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_CAPTURE_SCOPE.to_string(),
        LogFieldValue::String(SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW.to_string()),
    );
    fields.insert(
        constants::field::CAPABILITY_STATUS.to_string(),
        LogFieldValue::String(SCREEN_CAPABILITY_READY.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_IMAGE_DIGEST.to_string(),
        LogFieldValue::String(constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string()),
    );
    fields.insert(
        constants::field::SCREEN_CUSTODY_STATE.to_string(),
        LogFieldValue::String(SCREEN_CUSTODY_JOURNAL.to_string()),
    );

    screen_event(fields)
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
        schema_version: ocentra_parent_agent_protocol::ACTIVITY_SCHEMA_VERSION,
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
