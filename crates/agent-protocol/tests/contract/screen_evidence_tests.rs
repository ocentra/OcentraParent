use super::{
    constants, ActivityEvidenceKind, ActivityEvidenceRef, ScreenAnalysisQueueJob,
    ScreenAnalysisResult, ScreenCategoryCandidate, ScreenEvidenceQueueHealth,
    ScreenEvidenceRecentSummary, SCREEN_CAPABILITY_READY, SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST,
    SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW, SCREEN_CATEGORY_SCHOOL, SCREEN_CUSTODY_JOURNAL,
    SCREEN_CUSTODY_QUERY_STORE, SCREEN_CUSTODY_TEMP_QUEUE, SCREEN_DELETION_DELETED,
    SCREEN_DELETION_DELETE_FAILED, SCREEN_DELETION_REQUIRED, SCREEN_EVIDENCE_SCHEMA_VERSION,
    SCREEN_IMAGE_FORMAT_PNG, SCREEN_POLICY_CONFIDENCE_READY, SCREEN_PROVIDER_LOCAL_VISION,
    SCREEN_QUEUE_STATUS_DELETED, SCREEN_QUEUE_STATUS_FAILED, SCREEN_QUEUE_STATUS_QUEUED,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn screen_analysis_queue_job_serializes_temp_encrypted_queue_shape() {
    let job = queue_job();
    let serialized =
        serde_json::to_value(job).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["queueJobId"],
        constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID
    );
    assert_eq!(serialized["deletionRequired"], true);
    assert_eq!(serialized["custodyState"], SCREEN_CUSTODY_TEMP_QUEUE);
    assert_eq!(serialized["status"], SCREEN_QUEUE_STATUS_QUEUED);
}

#[test]
fn screen_analysis_queue_job_serializes_delete_failed_custody_state() {
    let mut job = queue_job();
    job.status = SCREEN_QUEUE_STATUS_FAILED.to_string();
    job.deletion_status = SCREEN_DELETION_DELETE_FAILED.to_string();
    job.failure_reason = Some(constants::activity_store::TEST_SCREEN_DELETION_REASONS.to_string());
    let serialized =
        serde_json::to_value(job).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["status"], SCREEN_QUEUE_STATUS_FAILED);
    assert_eq!(serialized["deletionStatus"], SCREEN_DELETION_DELETE_FAILED);
    assert_eq!(serialized["deletedAt"], serde_json::Value::Null);
    assert_eq!(serialized["deletionProofRef"], serde_json::Value::Null);
}

#[test]
fn screen_analysis_result_serializes_without_raw_image_payload() {
    let result = analysis_result(Vec::new());
    let serialized =
        serde_json::to_value(result).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["screenAnalysisResultId"],
        constants::activity_store::TEST_SCREEN_RESULT_ID
    );
    assert_eq!(serialized["rawImageRetained"], false);
    assert!(serialized.get("encryptedImageRef").is_none());
    assert_eq!(serialized["imageDeletionState"], SCREEN_DELETION_DELETED);
}

#[test]
fn screen_evidence_recent_summary_serializes_flat_read_model() {
    let evidence = vec![journal_evidence()];
    let summary = ScreenEvidenceRecentSummary {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody_state: SCREEN_CUSTODY_QUERY_STORE.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 1,
        queue_health: ScreenEvidenceQueueHealth {
            schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
            generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
            custody_state: SCREEN_CUSTODY_QUERY_STORE.to_string(),
            pending_count: 0,
            expired_count: 0,
            delete_pending_count: 0,
            delete_failed_count: 0,
            latest_queue_job_id: Some(
                constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
            ),
            latest_status: Some(SCREEN_QUEUE_STATUS_DELETED.to_string()),
            last_successful_analysis_at: Some(
                constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
            ),
        },
        latest_result_id: Some(constants::activity_store::TEST_SCREEN_RESULT_ID.to_string()),
        latest_summary: Some(constants::activity_store::TEST_SCREEN_SUMMARY.to_string()),
        latest_primary_category: Some(SCREEN_CATEGORY_SCHOOL.to_string()),
        latest_confidence: Some(SCREEN_POLICY_CONFIDENCE_READY),
        latest_image_deletion_state: Some(SCREEN_DELETION_DELETED.to_string()),
        latest_policy_eligible: Some(true),
        evidence: evidence.clone(),
        results: vec![analysis_result(evidence)],
    };
    let serialized =
        serde_json::to_value(summary).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["returned"], 1);
    assert_eq!(serialized["latestPrimaryCategory"], SCREEN_CATEGORY_SCHOOL);
    assert_eq!(
        serialized["queueHealth"]["latestStatus"],
        SCREEN_QUEUE_STATUS_DELETED
    );
}

pub(crate) fn queue_job() -> ScreenAnalysisQueueJob {
    ScreenAnalysisQueueJob {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
        created_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        not_before: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        expires_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        last_attempt_at: None,
        capture_reason: SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST.to_string(),
        capture_scope: SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW.to_string(),
        source_id: constants::activity_store::TEST_SCREEN_SOURCE_ID.to_string(),
        adapter_id: constants::activity_store::TEST_SCREEN_ADAPTER_ID.to_string(),
        device_ref: constants::peer::LOCAL_DEV_AGENT.to_string(),
        local_user_ref: constants::activity_store::TEST_SCREEN_LOCAL_USER_REF.to_string(),
        parent_setting_ref: constants::activity_store::TEST_SCREEN_PARENT_SETTING_REF.to_string(),
        setting_version: 1,
        related_evidence_refs: vec![journal_evidence()],
        encrypted_image_ref: constants::activity_store::TEST_SCREEN_ENCRYPTED_IMAGE_REF.to_string(),
        image_digest: constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string(),
        image_byte_size: 2048,
        image_format: SCREEN_IMAGE_FORMAT_PNG.to_string(),
        status: SCREEN_QUEUE_STATUS_QUEUED.to_string(),
        attempt_count: 0,
        max_retry_count: 2,
        failure_reason: None,
        unavailable_reason: None,
        deletion_required: true,
        deleted_at: None,
        deletion_status: SCREEN_DELETION_REQUIRED.to_string(),
        deletion_proof_ref: None,
        custody_state: SCREEN_CUSTODY_TEMP_QUEUE.to_string(),
    }
}

pub(crate) fn analysis_result(evidence: Vec<ActivityEvidenceRef>) -> ScreenAnalysisResult {
    ScreenAnalysisResult {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        screen_analysis_result_id: constants::activity_store::TEST_SCREEN_RESULT_ID.to_string(),
        queue_job_id: constants::activity_store::TEST_SCREEN_QUEUE_JOB_ID.to_string(),
        analyzed_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        model_runtime_ref: constants::activity_store::TEST_SCREEN_MODEL_RUNTIME_REF.to_string(),
        model_id: constants::activity_store::TEST_SCREEN_MODEL_ID.to_string(),
        provider_kind: SCREEN_PROVIDER_LOCAL_VISION.to_string(),
        prompt_or_template_version: constants::activity_store::TEST_SCREEN_TEMPLATE_VERSION
            .to_string(),
        capture_reason: SCREEN_CAPTURE_REASON_MANUAL_PARENT_TEST.to_string(),
        capture_scope: SCREEN_CAPTURE_SCOPE_ACTIVE_WINDOW.to_string(),
        capability_status: SCREEN_CAPABILITY_READY.to_string(),
        summary: constants::activity_store::TEST_SCREEN_SUMMARY.to_string(),
        visible_category_candidates: vec![ScreenCategoryCandidate {
            category: SCREEN_CATEGORY_SCHOOL.to_string(),
            confidence: SCREEN_POLICY_CONFIDENCE_READY,
            evidence_refs: evidence.clone(),
        }],
        primary_category: Some(SCREEN_CATEGORY_SCHOOL.to_string()),
        risk_signals: Vec::new(),
        ocr_text_snippets: Vec::new(),
        redaction_notes: Vec::new(),
        confidence: SCREEN_POLICY_CONFIDENCE_READY,
        uncertainty_reason: None,
        source_evidence_refs: evidence,
        image_digest: constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string(),
        raw_image_retained: false,
        image_deletion_state: SCREEN_DELETION_DELETED.to_string(),
        custody_state: SCREEN_CUSTODY_JOURNAL.to_string(),
        policy_eligible: true,
        policy_decision_ref: Some("screen-policy-decision-1".to_string()),
        policy_action: Some("allow".to_string()),
        policy_reason_codes: vec![
            "screen-summary-linked".to_string(),
            "parent-rule-linked".to_string(),
        ],
        parent_rule_refs: vec!["screen-parent-rule-school".to_string()],
        local_model_runtime_refs: vec!["screen-local-runtime-1".to_string()],
        parent_explanation_refs: vec!["screen-parent-explanation-1".to_string()],
        explanation_reasons: vec![
            "screen-summary-cited".to_string(),
            "policy-decision-cited".to_string(),
        ],
        deletion_reasons: vec!["screen-image-deleted".to_string()],
    }
}

fn journal_evidence() -> ActivityEvidenceRef {
    ActivityEvidenceRef {
        evidence_id: constants::activity_store::TEST_JOURNAL_SUFFIX.to_string(),
        kind: ActivityEvidenceKind::JournalEntry,
        digest: Some(constants::activity_store::TEST_SCREEN_IMAGE_DIGEST.to_string()),
        uri: None,
    }
}
