use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceKind, ActivityEvidenceRef, AppGameBoundaryReadModel,
    AppGameBoundaryReadModelRow, AppGameServiceReadModel, LogFieldValue, LogFields,
    APP_GAME_BOUNDARY_KIND_AI_CLASSIFIER_RESULT, APP_GAME_BOUNDARY_KIND_APPROVAL_ACTION_RESULT,
    APP_GAME_BOUNDARY_KIND_APPROVAL_AUTHORITY, APP_GAME_BOUNDARY_KIND_EVIDENCE_CLAIM,
    APP_GAME_BOUNDARY_KIND_IDENTITY, APP_GAME_BOUNDARY_KIND_PLATFORM_AUTHORITY_MATRIX,
    APP_GAME_BOUNDARY_KIND_PLATFORM_AUTHORITY_ROW,
    APP_GAME_BOUNDARY_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    APP_GAME_BOUNDARY_READ_MODEL_STATUS_NO_ROWS, APP_GAME_SCHEMA_VERSION,
};

use crate::fields::fields_from_pairs;

type FieldPair = (&'static str, LogFieldValue);

pub fn app_game_boundary_read_model_from_service_model(
    model: AppGameServiceReadModel,
) -> AppGameBoundaryReadModel {
    let rows = boundary_rows(&model);
    let returned = rows.len() as u64;
    let platform_authority_row_count = platform_authority_row_count(&model);
    let capability_status = if rows.is_empty() {
        APP_GAME_BOUNDARY_READ_MODEL_STATUS_NO_ROWS.to_string()
    } else {
        model.capability_status.clone()
    };

    AppGameBoundaryReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: model.generated_at,
        custody_label: APP_GAME_BOUNDARY_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status,
        returned,
        evidence_claim_row_count: model.evidence_claim_rows.len() as u64,
        identity_row_count: model.identity_rows.len() as u64,
        approval_authority_row_count: model.approval_authority_rows.len() as u64,
        approval_action_result_row_count: model.approval_action_result_rows.len() as u64,
        platform_authority_matrix_count: model.platform_authority_matrices.len() as u64,
        platform_authority_row_count,
        ai_classifier_result_row_count: model.ai_classifier_result_rows.len() as u64,
        rows,
    }
}

pub fn app_game_boundary_read_model_payload(read_model: &AppGameBoundaryReadModel) -> LogFields {
    fields_from_pairs(read_model_pairs(read_model))
}

fn read_model_pairs(read_model: &AppGameBoundaryReadModel) -> Vec<FieldPair> {
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
            constants::field::APP_GAME_BOUNDARY_READ_MODEL,
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ]
}

fn boundary_rows(model: &AppGameServiceReadModel) -> Vec<AppGameBoundaryReadModelRow> {
    let mut rows = Vec::new();
    push_boundary_row(
        &mut rows,
        APP_GAME_BOUNDARY_KIND_EVIDENCE_CLAIM,
        model.evidence_claim_rows.len() as u64,
        evidence_claim_refs(model),
    );
    push_boundary_row(
        &mut rows,
        APP_GAME_BOUNDARY_KIND_IDENTITY,
        model.identity_rows.len() as u64,
        identity_refs(model),
    );
    push_boundary_row(
        &mut rows,
        APP_GAME_BOUNDARY_KIND_APPROVAL_AUTHORITY,
        model.approval_authority_rows.len() as u64,
        approval_authority_refs(model),
    );
    push_boundary_row(
        &mut rows,
        APP_GAME_BOUNDARY_KIND_APPROVAL_ACTION_RESULT,
        model.approval_action_result_rows.len() as u64,
        approval_action_result_refs(model),
    );
    push_boundary_row(
        &mut rows,
        APP_GAME_BOUNDARY_KIND_PLATFORM_AUTHORITY_MATRIX,
        model.platform_authority_matrices.len() as u64,
        platform_authority_matrix_refs(model),
    );
    push_boundary_row(
        &mut rows,
        APP_GAME_BOUNDARY_KIND_PLATFORM_AUTHORITY_ROW,
        platform_authority_row_count(model),
        platform_authority_row_refs(model),
    );
    push_boundary_row(
        &mut rows,
        APP_GAME_BOUNDARY_KIND_AI_CLASSIFIER_RESULT,
        model.ai_classifier_result_rows.len() as u64,
        ai_classifier_refs(model),
    );
    rows
}

fn push_boundary_row(
    rows: &mut Vec<AppGameBoundaryReadModelRow>,
    boundary_kind: &'static str,
    row_count: u64,
    evidence: Vec<ActivityEvidenceRef>,
) {
    if row_count == 0 {
        return;
    }
    rows.push(AppGameBoundaryReadModelRow {
        schema_version: APP_GAME_SCHEMA_VERSION,
        row_id: boundary_kind.to_string(),
        boundary_kind: boundary_kind.to_string(),
        row_count,
        evidence_reference_ids: evidence.iter().map(|row| row.evidence_id.clone()).collect(),
        evidence,
    });
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

fn platform_authority_matrix_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for matrix in &model.platform_authority_matrices {
        push_local_db_row_evidence(&mut evidence, &matrix.matrix_id);
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
