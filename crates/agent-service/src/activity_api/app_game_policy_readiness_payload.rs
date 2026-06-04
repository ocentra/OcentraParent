use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceKind, ActivityEvidenceRef, AppGamePolicyReadinessReadModel,
    AppGamePolicyReadinessRow, AppGameServiceReadModel, LogFieldValue, LogFields,
    APP_GAME_POLICY_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    APP_GAME_POLICY_READINESS_KIND_AI_CLASSIFIER_CONTEXT,
    APP_GAME_POLICY_READINESS_KIND_APPROVAL_ACTION_RESULT,
    APP_GAME_POLICY_READINESS_KIND_APPROVAL_AUTHORITY,
    APP_GAME_POLICY_READINESS_KIND_PLATFORM_AUTHORITY,
    APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE,
    APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED, APP_GAME_POLICY_READINESS_STATE_MISSING,
    APP_GAME_POLICY_READINESS_STATE_READY, APP_GAME_POLICY_READINESS_STATUS_NO_ROWS,
    APP_GAME_POLICY_READINESS_STATUS_PARTIAL, APP_GAME_POLICY_READINESS_STATUS_READY,
    APP_GAME_SCHEMA_VERSION,
};

use crate::fields::fields_from_pairs;

type FieldPair = (&'static str, LogFieldValue);

pub fn app_game_policy_readiness_from_service_model(
    model: AppGameServiceReadModel,
) -> AppGamePolicyReadinessReadModel {
    let platform_authority_row_count = platform_authority_row_count(&model);
    let rows = readiness_rows(&model);
    let returned = rows.len() as u64;
    let policy_evaluation_ready = !model.evidence_claim_rows.is_empty()
        && !model.identity_rows.is_empty()
        && !model.approval_authority_rows.is_empty()
        && platform_authority_row_count > 0;
    let manual_review_required = rows
        .iter()
        .any(|row| row.readiness_state != APP_GAME_POLICY_READINESS_STATE_READY);
    let capability_status = policy_readiness_status(&model, policy_evaluation_ready);

    AppGamePolicyReadinessReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: model.generated_at,
        custody_label: APP_GAME_POLICY_READINESS_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status,
        returned,
        policy_evaluation_ready,
        manual_review_required,
        adapter_dispatch_claimed: false,
        evidence_claim_row_count: model.evidence_claim_rows.len() as u64,
        identity_row_count: model.identity_rows.len() as u64,
        approval_authority_row_count: model.approval_authority_rows.len() as u64,
        approval_action_result_row_count: model.approval_action_result_rows.len() as u64,
        platform_authority_row_count,
        ai_classifier_result_row_count: model.ai_classifier_result_rows.len() as u64,
        rows,
    }
}

pub fn app_game_policy_readiness_payload(
    read_model: &AppGamePolicyReadinessReadModel,
) -> LogFields {
    fields_from_pairs(read_model_pairs(read_model))
}

fn read_model_pairs(read_model: &AppGamePolicyReadinessReadModel) -> Vec<FieldPair> {
    vec![
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
            constants::field::APP_GAME_POLICY_READINESS_READ_MODEL,
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ]
}

fn readiness_rows(model: &AppGameServiceReadModel) -> Vec<AppGamePolicyReadinessRow> {
    let policy_evidence_count =
        model.evidence_claim_rows.len() as u64 + model.identity_rows.len() as u64;
    let has_policy_evidence =
        !model.evidence_claim_rows.is_empty() && !model.identity_rows.is_empty();
    vec![
        readiness_row(
            APP_GAME_POLICY_READINESS_KIND_POLICY_EVIDENCE,
            if has_policy_evidence {
                APP_GAME_POLICY_READINESS_STATE_READY
            } else {
                APP_GAME_POLICY_READINESS_STATE_MISSING
            },
            policy_evidence_count,
            policy_evidence_refs(model),
        ),
        readiness_row(
            APP_GAME_POLICY_READINESS_KIND_APPROVAL_AUTHORITY,
            if model.approval_authority_rows.is_empty() {
                APP_GAME_POLICY_READINESS_STATE_MISSING
            } else {
                APP_GAME_POLICY_READINESS_STATE_READY
            },
            model.approval_authority_rows.len() as u64,
            approval_authority_refs(model),
        ),
        readiness_row(
            APP_GAME_POLICY_READINESS_KIND_APPROVAL_ACTION_RESULT,
            if model.approval_action_result_rows.is_empty() {
                APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED
            } else {
                APP_GAME_POLICY_READINESS_STATE_READY
            },
            model.approval_action_result_rows.len() as u64,
            approval_action_result_refs(model),
        ),
        readiness_row(
            APP_GAME_POLICY_READINESS_KIND_PLATFORM_AUTHORITY,
            if platform_authority_row_count(model) == 0 {
                APP_GAME_POLICY_READINESS_STATE_MISSING
            } else {
                APP_GAME_POLICY_READINESS_STATE_READY
            },
            platform_authority_row_count(model),
            platform_authority_row_refs(model),
        ),
        readiness_row(
            APP_GAME_POLICY_READINESS_KIND_AI_CLASSIFIER_CONTEXT,
            if model.ai_classifier_result_rows.is_empty() {
                APP_GAME_POLICY_READINESS_STATE_MANUAL_REQUIRED
            } else {
                APP_GAME_POLICY_READINESS_STATE_READY
            },
            model.ai_classifier_result_rows.len() as u64,
            ai_classifier_refs(model),
        ),
    ]
}

fn readiness_row(
    readiness_kind: &'static str,
    readiness_state: &'static str,
    row_count: u64,
    evidence: Vec<ActivityEvidenceRef>,
) -> AppGamePolicyReadinessRow {
    AppGamePolicyReadinessRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: readiness_kind.to_string(),
        readiness_kind: readiness_kind.to_string(),
        readiness_state: readiness_state.to_string(),
        row_count,
        evidence_reference_ids: evidence.iter().map(|row| row.evidence_id.clone()).collect(),
        evidence,
    }
}

fn policy_readiness_status(
    model: &AppGameServiceReadModel,
    policy_evaluation_ready: bool,
) -> String {
    if app_game_boundary_row_count(model) == 0 {
        APP_GAME_POLICY_READINESS_STATUS_NO_ROWS.to_string()
    } else if policy_evaluation_ready {
        APP_GAME_POLICY_READINESS_STATUS_READY.to_string()
    } else {
        APP_GAME_POLICY_READINESS_STATUS_PARTIAL.to_string()
    }
}

fn policy_evidence_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    push_evidence(&mut evidence, evidence_claim_refs(model));
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

fn approval_action_result_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.approval_action_result_rows {
        push_local_db_row_evidence(&mut evidence, &row.result_id);
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

fn ai_classifier_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.ai_classifier_result_rows {
        push_local_db_row_evidence(&mut evidence, &row.classifier_run_id);
        for evidence_ref in &row.source_evidence_refs {
            push_local_db_row_evidence(&mut evidence, evidence_ref);
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
