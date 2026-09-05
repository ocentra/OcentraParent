#[path = "app_game_policy_readiness_sources/classification_refs.rs"]
mod classification_refs;

use ocentra_app_game_core::app_game_risk_candidate_detection::detect_app_game_risk_candidate;
use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;

#[derive(Clone, Copy)]
pub(super) struct ClassificationStateRef<'a>(pub(super) &'a str);

#[derive(Clone, Copy)]
pub(super) struct LocalDbRowEvidenceIdRef<'a>(pub(super) &'a str);

pub(super) fn policy_evidence_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    push_evidence(&mut evidence, evidence_claim_refs(model));
    push_evidence(&mut evidence, identity_refs(model));
    evidence
}

pub(super) fn approval_authority_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.approval_authority_rows {
        push_local_db_row_evidence(&mut evidence, LocalDbRowEvidenceIdRef(&row.authority_id));
    }
    evidence
}

pub(super) fn approval_action_result_refs(
    model: &AppGameServiceReadModel,
) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.approval_action_result_rows {
        push_local_db_row_evidence(&mut evidence, LocalDbRowEvidenceIdRef(&row.result_id));
    }
    evidence
}

pub(super) fn platform_authority_row_refs(
    model: &AppGameServiceReadModel,
) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for matrix in &model.platform_authority_matrices {
        for row in &matrix.rows {
            push_local_db_row_evidence(&mut evidence, LocalDbRowEvidenceIdRef(&row.row_id));
        }
    }
    evidence
}

pub(super) fn platform_authority_row_count(model: &AppGameServiceReadModel) -> u64 {
    model
        .platform_authority_matrices
        .iter()
        .map(|matrix| matrix.rows.len() as u64)
        .sum()
}

pub(super) fn ai_classifier_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    classification_refs::ai_classifier_refs(model)
}

pub(super) fn category_candidate_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    classification_refs::category_candidate_refs(model)
}

pub(super) fn category_candidate_row_count(model: &AppGameServiceReadModel) -> u64 {
    classification_refs::category_candidate_row_count(model)
}

pub(super) fn category_risk_routing(
    model: &AppGameServiceReadModel,
) -> (u64, Vec<ActivityEvidenceRef>) {
    let mut candidate_count = 0;
    let mut evidence = Vec::new();
    for row in &model.inventory_rows {
        if detect_app_game_risk_candidate(row).candidate.is_none() {
            continue;
        }
        candidate_count += 1;
        push_evidence(&mut evidence, row.evidence.clone());
        push_local_db_row_evidence(
            &mut evidence,
            LocalDbRowEvidenceIdRef(&row.inventory_entry_id),
        );
        for category in &row.category_candidates {
            push_evidence(&mut evidence, category.evidence.clone());
        }
    }
    (candidate_count, evidence)
}

pub(super) fn unknown_review_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    classification_refs::unknown_review_refs(model)
}

pub(super) fn unknown_review_row_count(model: &AppGameServiceReadModel) -> u64 {
    classification_refs::unknown_review_row_count(model)
}

pub(super) fn app_game_boundary_row_count(model: &AppGameServiceReadModel) -> u64 {
    model.evidence_claim_rows.len() as u64
        + model.identity_rows.len() as u64
        + model.approval_authority_rows.len() as u64
        + model.approval_action_result_rows.len() as u64
        + platform_authority_row_count(model)
        + model.ai_classifier_result_rows.len() as u64
        + category_candidate_row_count(model)
        + unknown_review_row_count(model)
}

fn evidence_claim_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.evidence_claim_rows {
        push_evidence(&mut evidence, row.evidence.clone());
        push_local_db_row_evidence(&mut evidence, LocalDbRowEvidenceIdRef(&row.claim_id));
    }
    evidence
}

fn identity_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.identity_rows {
        push_evidence(&mut evidence, row.evidence.clone());
        push_local_db_row_evidence(&mut evidence, LocalDbRowEvidenceIdRef(&row.identity_id));
    }
    evidence
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

pub(super) fn push_local_db_row_evidence(
    target: &mut Vec<ActivityEvidenceRef>,
    evidence_id: LocalDbRowEvidenceIdRef<'_>,
) {
    if evidence_id.0.is_empty() {
        return;
    }
    push_evidence(
        target,
        vec![ActivityEvidenceRef {
            evidence_id: evidence_id.0.to_string(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        }],
    );
}
