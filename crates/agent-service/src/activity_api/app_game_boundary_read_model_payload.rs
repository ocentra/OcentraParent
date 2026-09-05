use ocentra_parent_agent_core::activity_store_app_game::app_game_performance_health::app_game_performance_health;
use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::app_game::{AppGameServiceReadModel, APP_GAME_SCHEMA_VERSION};
use ocentra_parent_agent_protocol::app_game_boundary_read_model::{
    AppGameBoundaryReadModel, AppGameBoundaryReadModelRow,
    APP_GAME_BOUNDARY_KIND_AI_CLASSIFIER_RESULT, APP_GAME_BOUNDARY_KIND_APPROVAL_ACTION_RESULT,
    APP_GAME_BOUNDARY_KIND_APPROVAL_AUTHORITY, APP_GAME_BOUNDARY_KIND_EVIDENCE_CLAIM,
    APP_GAME_BOUNDARY_KIND_IDENTITY, APP_GAME_BOUNDARY_KIND_PLATFORM_AUTHORITY_MATRIX,
    APP_GAME_BOUNDARY_KIND_PLATFORM_AUTHORITY_ROW,
    APP_GAME_BOUNDARY_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    APP_GAME_BOUNDARY_READ_MODEL_STATUS_NO_ROWS,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use super::app_game_boundary_read_model_payload_rows::{
    push_boundary_row, push_evidence, push_local_db_row_evidence, BoundaryKindText, EvidenceIdText,
};
use crate::fields::fields_from_pairs;

#[derive(Clone, Debug, PartialEq)]
pub(super) struct FieldPairs(pub(super) Vec<(&'static str, LogFieldValue)>);

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
    let performance_health = app_game_performance_health(&model);

    AppGameBoundaryReadModel {
        schema_version: APP_GAME_SCHEMA_VERSION,
        generated_at: model.generated_at,
        custody_label: APP_GAME_BOUNDARY_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        capability_status,
        performance_health,
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
    fields_from_pairs(read_model_pairs(read_model).0)
}

fn read_model_pairs(read_model: &AppGameBoundaryReadModel) -> FieldPairs {
    FieldPairs(vec![
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
            LogFieldValue::String(serde_json::to_string(read_model).unwrap_or_default()),
        ),
    ])
}

fn boundary_rows(model: &AppGameServiceReadModel) -> Vec<AppGameBoundaryReadModelRow> {
    let mut rows = Vec::new();
    push_boundary_row(
        &mut rows,
        BoundaryKindText(APP_GAME_BOUNDARY_KIND_EVIDENCE_CLAIM),
        model.evidence_claim_rows.len() as u64,
        evidence_claim_refs(model),
    );
    push_boundary_row(
        &mut rows,
        BoundaryKindText(APP_GAME_BOUNDARY_KIND_IDENTITY),
        model.identity_rows.len() as u64,
        identity_refs(model),
    );
    push_boundary_row(
        &mut rows,
        BoundaryKindText(APP_GAME_BOUNDARY_KIND_APPROVAL_AUTHORITY),
        model.approval_authority_rows.len() as u64,
        approval_authority_refs(model),
    );
    push_boundary_row(
        &mut rows,
        BoundaryKindText(APP_GAME_BOUNDARY_KIND_APPROVAL_ACTION_RESULT),
        model.approval_action_result_rows.len() as u64,
        approval_action_result_refs(model),
    );
    push_boundary_row(
        &mut rows,
        BoundaryKindText(APP_GAME_BOUNDARY_KIND_PLATFORM_AUTHORITY_MATRIX),
        model.platform_authority_matrices.len() as u64,
        platform_authority_matrix_refs(model),
    );
    push_boundary_row(
        &mut rows,
        BoundaryKindText(APP_GAME_BOUNDARY_KIND_PLATFORM_AUTHORITY_ROW),
        platform_authority_row_count(model),
        platform_authority_row_refs(model),
    );
    push_boundary_row(
        &mut rows,
        BoundaryKindText(APP_GAME_BOUNDARY_KIND_AI_CLASSIFIER_RESULT),
        model.ai_classifier_result_rows.len() as u64,
        ai_classifier_refs(model),
    );
    rows
}

fn evidence_claim_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.evidence_claim_rows {
        push_evidence(&mut evidence, row.evidence.clone());
        push_local_db_row_evidence(&mut evidence, EvidenceIdText(row.claim_id.clone()));
    }
    evidence
}

fn identity_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.identity_rows {
        push_evidence(&mut evidence, row.evidence.clone());
        push_local_db_row_evidence(&mut evidence, EvidenceIdText(row.identity_id.clone()));
    }
    evidence
}

fn approval_authority_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.approval_authority_rows {
        push_local_db_row_evidence(&mut evidence, EvidenceIdText(row.authority_id.clone()));
    }
    evidence
}

fn approval_action_result_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.approval_action_result_rows {
        push_local_db_row_evidence(&mut evidence, EvidenceIdText(row.result_id.clone()));
    }
    evidence
}

fn platform_authority_matrix_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for matrix in &model.platform_authority_matrices {
        push_local_db_row_evidence(&mut evidence, EvidenceIdText(matrix.matrix_id.clone()));
    }
    evidence
}

fn platform_authority_row_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for matrix in &model.platform_authority_matrices {
        for row in &matrix.rows {
            push_local_db_row_evidence(&mut evidence, EvidenceIdText(row.row_id.clone()));
        }
    }
    evidence
}

fn ai_classifier_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.ai_classifier_result_rows {
        push_local_db_row_evidence(&mut evidence, EvidenceIdText(row.classifier_run_id.clone()));
        for evidence_ref in &row.source_evidence_refs {
            push_local_db_row_evidence(&mut evidence, EvidenceIdText(evidence_ref.clone()));
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
