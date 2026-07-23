use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::screen_evidence::{
    ScreenAnalysisResult, ScreenCategoryCandidate, ScreenEvidenceQueueHealth,
    ScreenEvidenceRecentSummary, SCREEN_CUSTODY_QUERY_STORE, SCREEN_DELETION_DELETED,
    SCREEN_DELETION_DELETE_FAILED, SCREEN_DELETION_EXPIRED_DELETED, SCREEN_DELETION_REQUIRED,
    SCREEN_QUEUE_STATUS_DELETED, SCREEN_QUEUE_STATUS_EXPIRED, SCREEN_QUEUE_STATUS_FAILED,
    SCREEN_QUEUE_STATUS_QUEUED,
};
use ocentra_parent_agent_protocol::SCREEN_EVIDENCE_SCHEMA_VERSION;
use rusqlite::{params, Connection};

use crate::ActivityStoreError;

#[path = "activity_store_screen_evidence/helpers.rs"]
mod helpers;

use self::helpers::{
    bool_field, non_empty_string_list_field, number_field, string_field, string_list_field,
};

const UNBOUNDED_SCREEN_QUEUE_HEALTH_LIMIT: i64 = -1;

pub(crate) fn screen_evidence_recent_summary(
    connection: &Connection,
    limit: u64,
    generated_at: &str,
) -> Result<ScreenEvidenceRecentSummary, ActivityStoreError> {
    let queue_health_results =
        screen_analysis_results(connection, UNBOUNDED_SCREEN_QUEUE_HEALTH_LIMIT)?;
    let result_limit = usize::try_from(limit).unwrap_or(usize::MAX);
    let results = queue_health_results
        .iter()
        .take(result_limit)
        .cloned()
        .collect();
    Ok(summary_from_results(
        limit,
        generated_at,
        results,
        &queue_health_results,
    ))
}

fn screen_analysis_results(
    connection: &Connection,
    limit: i64,
) -> Result<Vec<ScreenAnalysisResult>, ActivityStoreError> {
    let mut statement =
        connection.prepare(constants::sqlite::SELECT_RECENT_SCREEN_ANALYSIS_ACTIVITY)?;
    let rows = statement.query_map(
        params![
            constants::activity_event_kind::SCREEN_ANALYSIS_SUMMARIZED,
            constants::activity_observer::LOCAL_AI,
            limit
        ],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, String>(3)?,
            ))
        },
    )?;
    let mut results = Vec::new();
    for row in rows {
        let (_event_id, observed_at, fields_json, evidence_json) = row?;
        if let Some(result) = result_from_json(observed_at, &fields_json, &evidence_json)? {
            results.push(result);
        }
    }
    Ok(results)
}

pub(crate) fn screen_evidence_result_for_queue_job(
    connection: &Connection,
    queue_job_id: &str,
) -> Result<Option<ScreenAnalysisResult>, ActivityStoreError> {
    let mut statement = connection
        .prepare(constants::sqlite::SELECT_LATEST_SCREEN_ANALYSIS_ACTIVITY_FOR_QUEUE_JOB)?;
    let mut rows = statement.query(params![
        constants::activity_event_kind::SCREEN_ANALYSIS_SUMMARIZED,
        constants::activity_observer::LOCAL_AI,
        queue_job_id,
    ])?;
    let Some(row) = rows.next()? else {
        return Ok(None);
    };
    let observed_at = row.get(1)?;
    let fields_json = row.get::<_, String>(2)?;
    let evidence_json = row.get::<_, String>(3)?;
    result_from_json(observed_at, &fields_json, &evidence_json)
}

fn result_from_json(
    observed_at: String,
    fields_json: &str,
    evidence_json: &str,
) -> Result<Option<ScreenAnalysisResult>, ActivityStoreError> {
    let fields = serde_json::from_str::<LogFields>(fields_json)?;
    let evidence = serde_json::from_str::<Vec<ActivityEvidenceRef>>(evidence_json)?;
    Ok(result_from_fields(observed_at, &fields, evidence))
}

fn summary_from_results(
    limit: u64,
    generated_at: &str,
    results: Vec<ScreenAnalysisResult>,
    queue_health_results: &[ScreenAnalysisResult],
) -> ScreenEvidenceRecentSummary {
    let latest = results.first();
    ScreenEvidenceRecentSummary {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        custody_state: SCREEN_CUSTODY_QUERY_STORE.to_string(),
        limit,
        returned: results.len() as u64,
        queue_health: queue_health(generated_at, latest, queue_health_results),
        latest_result_id: latest.map(|result| result.screen_analysis_result_id.clone()),
        latest_summary: latest.map(|result| result.summary.clone()),
        latest_primary_category: latest.and_then(|result| result.primary_category.clone()),
        latest_confidence: latest.map(|result| result.confidence),
        latest_image_deletion_state: latest.map(|result| result.image_deletion_state.clone()),
        latest_policy_eligible: latest.map(|result| result.policy_eligible),
        evidence: latest
            .map(|result| result.source_evidence_refs.clone())
            .unwrap_or_default(),
        results,
    }
}

fn queue_health(
    generated_at: &str,
    latest: Option<&ScreenAnalysisResult>,
    results: &[ScreenAnalysisResult],
) -> ScreenEvidenceQueueHealth {
    ScreenEvidenceQueueHealth {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        generated_at: generated_at.to_string(),
        custody_state: SCREEN_CUSTODY_QUERY_STORE.to_string(),
        pending_count: queue_status_count(results, SCREEN_QUEUE_STATUS_QUEUED),
        expired_count: deletion_state_count(results, SCREEN_DELETION_EXPIRED_DELETED),
        delete_pending_count: deletion_state_count(results, SCREEN_DELETION_REQUIRED),
        delete_failed_count: deletion_state_count(results, SCREEN_DELETION_DELETE_FAILED),
        latest_queue_job_id: latest.map(|result| result.queue_job_id.clone()),
        latest_status: latest.map(queue_status_from_result),
        last_successful_analysis_at: latest.map(|result| result.analyzed_at.clone()),
    }
}

fn deletion_state_count(results: &[ScreenAnalysisResult], state: &str) -> u64 {
    let mut observed_jobs = std::collections::HashSet::new();
    results
        .iter()
        // The query is newest-first. Count only the first row for each job so
        // historical pending/deleted/expired transitions do not inflate health.
        .filter(|result| observed_jobs.insert(result.queue_job_id.as_str()))
        .filter(|result| result.image_deletion_state == state)
        .count() as u64
}

fn queue_status_count(results: &[ScreenAnalysisResult], status: &str) -> u64 {
    let mut observed_jobs = std::collections::HashSet::new();
    results
        .iter()
        .filter(|result| observed_jobs.insert(result.queue_job_id.as_str()))
        .filter(|result| queue_status_from_result(result) == status)
        .count() as u64
}

fn queue_status_from_result(result: &ScreenAnalysisResult) -> String {
    match result.image_deletion_state.as_str() {
        SCREEN_DELETION_DELETE_FAILED => SCREEN_QUEUE_STATUS_FAILED.to_string(),
        SCREEN_DELETION_EXPIRED_DELETED => SCREEN_QUEUE_STATUS_EXPIRED.to_string(),
        SCREEN_DELETION_DELETED => SCREEN_QUEUE_STATUS_DELETED.to_string(),
        SCREEN_DELETION_REQUIRED => SCREEN_QUEUE_STATUS_QUEUED.to_string(),
        _ => SCREEN_QUEUE_STATUS_FAILED.to_string(),
    }
}

fn result_from_fields(
    observed_at: String,
    fields: &LogFields,
    evidence: Vec<ActivityEvidenceRef>,
) -> Option<ScreenAnalysisResult> {
    let confidence = number_field(fields, constants::field::SCREEN_CONFIDENCE)?;
    let primary_category = string_field(fields, constants::field::SCREEN_PRIMARY_CATEGORY)?;
    let model_runtime_ref = string_field(fields, constants::field::SCREEN_MODEL_RUNTIME_REF)?;
    let local_model_runtime_refs =
        non_empty_string_list_field(fields, constants::field::SCREEN_LOCAL_MODEL_RUNTIME_REFS)
            .unwrap_or_else(|| vec![model_runtime_ref.clone()]);
    Some(ScreenAnalysisResult {
        schema_version: SCREEN_EVIDENCE_SCHEMA_VERSION,
        screen_analysis_result_id: string_field(
            fields,
            constants::field::SCREEN_ANALYSIS_RESULT_ID,
        )?,
        queue_job_id: string_field(fields, constants::field::SCREEN_QUEUE_JOB_ID)?,
        analyzed_at: observed_at,
        model_runtime_ref,
        model_id: string_field(fields, constants::field::SCREEN_MODEL_ID)?,
        provider_kind: string_field(fields, constants::field::SCREEN_PROVIDER_KIND)?,
        prompt_or_template_version: string_field(
            fields,
            constants::field::SCREEN_TEMPLATE_VERSION,
        )?,
        capture_reason: string_field(fields, constants::field::SCREEN_CAPTURE_REASON)?,
        capture_scope: string_field(fields, constants::field::SCREEN_CAPTURE_SCOPE)?,
        capability_status: string_field(fields, constants::field::CAPABILITY_STATUS)?,
        summary: string_field(fields, constants::field::SCREEN_SUMMARY)?,
        visible_category_candidates: vec![ScreenCategoryCandidate {
            category: primary_category.clone(),
            confidence,
            evidence_refs: evidence.clone(),
        }],
        primary_category: Some(primary_category),
        risk_signals: Vec::new(),
        ocr_text_snippets: string_list_field(fields, constants::field::SCREEN_OCR_TEXT_SNIPPETS),
        redaction_notes: string_list_field(fields, constants::field::SCREEN_REDACTION_NOTES),
        confidence,
        uncertainty_reason: None,
        source_evidence_refs: evidence,
        image_digest: string_field(fields, constants::field::SCREEN_IMAGE_DIGEST)?,
        raw_image_retained: false,
        image_deletion_state: string_field(fields, constants::field::SCREEN_IMAGE_DELETION_STATE)?,
        custody_state: string_field(fields, constants::field::SCREEN_CUSTODY_STATE)?,
        policy_eligible: bool_field(fields, constants::field::SCREEN_POLICY_ELIGIBLE)?,
        policy_decision_ref: string_field(fields, constants::field::POLICY_DECISION_ID),
        policy_action: string_field(fields, constants::field::POLICY_ACTION),
        policy_reason_codes: string_list_field(fields, constants::field::POLICY_REASON_CODES),
        parent_rule_refs: string_list_field(fields, constants::field::POLICY_RULE_IDS),
        local_model_runtime_refs,
        parent_explanation_refs: string_list_field(
            fields,
            constants::field::SCREEN_PARENT_EXPLANATION_REFS,
        ),
        explanation_reasons: string_list_field(
            fields,
            constants::field::SCREEN_EXPLANATION_REASONS,
        ),
        deletion_reasons: string_list_field(fields, constants::field::SCREEN_DELETION_REASONS),
    })
}
