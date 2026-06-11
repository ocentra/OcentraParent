use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceKind, ActivityEvidenceRef, AppGameNotificationReadinessReadModel,
    AppGameNotificationReadinessRow, AppGameServiceReadModel, LogFieldValue, LogFields,
    APP_GAME_CONTROL_ACTION_STATUS_ENFORCED,
    APP_GAME_NOTIFICATION_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_APPROVAL_REQUEST,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_SUSPICIOUS_UNKNOWN,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT,
    APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_UNAVAILABLE,
    APP_GAME_NOTIFICATION_READINESS_REASON_APPROVAL_REQUEST,
    APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE,
    APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN,
    APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
    APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED,
    APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
    APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE,
    APP_GAME_NOTIFICATION_READINESS_STATUS_NO_ROWS, APP_GAME_NOTIFICATION_READINESS_STATUS_PARTIAL,
    APP_GAME_NOTIFICATION_READINESS_STATUS_READY, APP_GAME_SCHEMA_VERSION,
};

use crate::fields::fields_from_pairs;

pub fn app_game_notification_readiness_from_service_model(
    model: AppGameServiceReadModel,
    local_outbox_runtime_claimed: bool,
) -> AppGameNotificationReadinessReadModel {
    let rows = notification_rows(&model);
    let returned = rows.len() as u64;
    let ready_intent_count = count_rows_with_state(
        &rows,
        APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
    );
    let manual_required_count =
        count_rows_with_state(&rows, APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED);
    let unavailable_count =
        count_rows_with_state(&rows, APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE);
    let adapter_dispatch_claimed = model
        .approval_action_result_rows
        .iter()
        .any(|row| row.result_status == APP_GAME_CONTROL_ACTION_STATUS_ENFORCED);

    AppGameNotificationReadinessReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: model.generated_at,
        custody_label: APP_GAME_NOTIFICATION_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status: notification_readiness_status(ready_intent_count, unavailable_count),
        returned,
        ready_intent_count,
        manual_required_count,
        unavailable_count,
        provider_delivery_claimed: false,
        provider_receipt_ingestion_claimed: false,
        local_outbox_runtime_claimed,
        scheduler_runtime_claimed: false,
        adapter_dispatch_claimed,
        parent_ui_claimed: false,
        child_delivery_claimed: false,
        rows,
    }
}

pub fn app_game_notification_readiness_payload(
    read_model: &AppGameNotificationReadinessReadModel,
) -> LogFields {
    fields_from_pairs(vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(read_model.custody_label.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(read_model.capability_status.clone()),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        (
            constants::field::APP_GAME_NOTIFICATION_READINESS_READ_MODEL,
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ])
}

fn notification_rows(model: &AppGameServiceReadModel) -> Vec<AppGameNotificationReadinessRow> {
    let mut rows = Vec::new();
    let policy_ready = policy_evaluation_ready(model);
    let policy_evidence = policy_evidence_refs(model);
    let approval_evidence = approval_authority_refs(model);

    if policy_ready {
        rows.push(notification_row(
            APP_GAME_NOTIFICATION_READINESS_REASON_TIME_LIMIT_EXCEEDED,
            APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
            policy_evidence.len() as u64,
            APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_TIME_LIMIT,
            policy_evidence.clone(),
        ));
    }

    if !policy_evidence.is_empty() && !approval_evidence.is_empty() {
        let mut evidence = policy_evidence.clone();
        push_evidence(&mut evidence, approval_evidence);
        rows.push(notification_row(
            APP_GAME_NOTIFICATION_READINESS_REASON_APPROVAL_REQUEST,
            APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
            evidence.len() as u64,
            APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_APPROVAL_REQUEST,
            evidence,
        ));
    }

    if !model.evidence_claim_rows.is_empty() {
        rows.push(notification_row(
            APP_GAME_NOTIFICATION_READINESS_REASON_SUSPICIOUS_UNKNOWN,
            APP_GAME_NOTIFICATION_READINESS_STATE_READY_FOR_LOCAL_INTENT,
            model.evidence_claim_rows.len() as u64,
            APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_SUSPICIOUS_UNKNOWN,
            evidence_claim_refs(model),
        ));
    }

    if !policy_ready || model.ai_classifier_result_rows.is_empty() {
        rows.push(notification_row(
            APP_GAME_NOTIFICATION_READINESS_REASON_MANUAL_REQUIRED,
            APP_GAME_NOTIFICATION_READINESS_STATE_MANUAL_REQUIRED,
            manual_required_count(model),
            APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_MANUAL_REQUIRED,
            manual_required_refs(model),
        ));
    }

    if app_game_boundary_row_count(model) == 0 {
        rows.push(notification_row(
            APP_GAME_NOTIFICATION_READINESS_REASON_CAPABILITY_UNAVAILABLE,
            APP_GAME_NOTIFICATION_READINESS_STATE_UNAVAILABLE,
            0,
            APP_GAME_NOTIFICATION_READINESS_MINIMAL_PAYLOAD_UNAVAILABLE,
            Vec::new(),
        ));
    }

    rows
}

fn notification_row(
    reason: &'static str,
    readiness_state: &'static str,
    row_count: u64,
    minimal_payload_ref: &'static str,
    evidence: Vec<ActivityEvidenceRef>,
) -> AppGameNotificationReadinessRow {
    AppGameNotificationReadinessRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: reason.to_string(),
        reason: reason.to_string(),
        readiness_state: readiness_state.to_string(),
        row_count,
        minimal_payload_ref: minimal_payload_ref.to_string(),
        evidence_reference_ids: evidence.iter().map(|row| row.evidence_id.clone()).collect(),
        evidence,
    }
}

fn notification_readiness_status(ready_intent_count: u64, unavailable_count: u64) -> String {
    if ready_intent_count == 0 && unavailable_count > 0 {
        APP_GAME_NOTIFICATION_READINESS_STATUS_NO_ROWS.to_string()
    } else if ready_intent_count >= 3 && unavailable_count == 0 {
        APP_GAME_NOTIFICATION_READINESS_STATUS_READY.to_string()
    } else {
        APP_GAME_NOTIFICATION_READINESS_STATUS_PARTIAL.to_string()
    }
}

fn policy_evaluation_ready(model: &AppGameServiceReadModel) -> bool {
    !model.evidence_claim_rows.is_empty()
        && !model.identity_rows.is_empty()
        && !model.approval_authority_rows.is_empty()
        && platform_authority_row_count(model) > 0
}

fn manual_required_count(model: &AppGameServiceReadModel) -> u64 {
    let mut count = 0;
    if model.identity_rows.is_empty() {
        count += 1;
    }
    if model.approval_authority_rows.is_empty() {
        count += 1;
    }
    if platform_authority_row_count(model) == 0 {
        count += 1;
    }
    if model.ai_classifier_result_rows.is_empty() {
        count += 1;
    }
    count
}

fn manual_required_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = policy_evidence_refs(model);
    push_evidence(&mut evidence, approval_authority_refs(model));
    push_evidence(&mut evidence, platform_authority_row_refs(model));
    evidence
}

fn policy_evidence_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = evidence_claim_refs(model);
    push_evidence(&mut evidence, identity_refs(model));
    evidence
}

fn evidence_claim_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.evidence_claim_rows {
        push_evidence(&mut evidence, row.evidence.clone());
        push_local_db_row_evidence(&mut evidence, &row.claim_id);
    }
    evidence
}

fn identity_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.identity_rows {
        push_evidence(&mut evidence, row.evidence.clone());
        push_local_db_row_evidence(&mut evidence, &row.identity_id);
    }
    evidence
}

fn approval_authority_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.approval_authority_rows {
        push_local_db_row_evidence(&mut evidence, &row.authority_id);
    }
    evidence
}

fn platform_authority_row_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for matrix in &model.platform_authority_matrices {
        for row in &matrix.rows {
            push_local_db_row_evidence(&mut evidence, &row.row_id);
        }
    }
    evidence
}

fn platform_authority_row_count(model: &AppGameServiceReadModel) -> u64 {
    model
        .platform_authority_matrices
        .iter()
        .map(|matrix| matrix.rows.len() as u64)
        .sum()
}

fn app_game_boundary_row_count(model: &AppGameServiceReadModel) -> u64 {
    model.evidence_claim_rows.len() as u64
        + model.identity_rows.len() as u64
        + model.approval_authority_rows.len() as u64
        + model.approval_action_result_rows.len() as u64
        + platform_authority_row_count(model)
        + model.ai_classifier_result_rows.len() as u64
}

fn count_rows_with_state(rows: &[AppGameNotificationReadinessRow], state: &str) -> u64 {
    rows.iter()
        .filter(|row| row.readiness_state == state)
        .count() as u64
}

fn push_evidence(target: &mut Vec<ActivityEvidenceRef>, rows: Vec<ActivityEvidenceRef>) {
    for evidence in rows {
        if target
            .iter()
            .any(|candidate| candidate.evidence_id == evidence.evidence_id)
        {
            continue;
        }
        target.push(evidence);
    }
}

fn push_local_db_row_evidence(target: &mut Vec<ActivityEvidenceRef>, evidence_id: &str) {
    if evidence_id.is_empty() {
        return;
    }
    push_evidence(
        target,
        vec![ActivityEvidenceRef {
            evidence_id: evidence_id.to_string(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        }],
    );
}
