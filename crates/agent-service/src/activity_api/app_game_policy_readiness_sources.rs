use ocentra_parent_agent_protocol::{
    ActivityEvidenceKind, ActivityEvidenceRef, AppGameServiceReadModel,
    APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE, APP_GAME_CLASSIFICATION_POSSIBLY_GAME,
    APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS,
};

pub(super) fn policy_evidence_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    push_evidence(&mut evidence, evidence_claim_refs(model));
    push_evidence(&mut evidence, identity_refs(model));
    evidence
}

pub(super) fn approval_authority_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.approval_authority_rows {
        push_local_db_row_evidence(&mut evidence, &row.authority_id);
    }
    evidence
}

pub(super) fn approval_action_result_refs(
    model: &AppGameServiceReadModel,
) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.approval_action_result_rows {
        push_local_db_row_evidence(&mut evidence, &row.result_id);
    }
    evidence
}

pub(super) fn platform_authority_row_refs(
    model: &AppGameServiceReadModel,
) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for matrix in &model.platform_authority_matrices {
        for row in &matrix.rows {
            push_local_db_row_evidence(&mut evidence, &row.row_id);
        }
    }
    evidence
}

pub(super) fn ai_classifier_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.ai_classifier_result_rows {
        push_local_db_row_evidence(&mut evidence, &row.classifier_run_id);
        for evidence_ref in &row.source_evidence_refs {
            push_local_db_row_evidence(&mut evidence, evidence_ref);
        }
    }
    evidence
}

pub(super) fn category_candidate_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.inventory_rows {
        for candidate in &row.category_candidates {
            push_evidence(&mut evidence, candidate.evidence.clone());
            if let Some(catalog_ref) = &candidate.catalog_ref {
                push_local_db_row_evidence(&mut evidence, catalog_ref);
            }
        }
    }
    evidence
}

pub(super) fn unknown_review_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.inventory_rows {
        if is_unknown_review_classification(&row.classification_state) {
            push_evidence(&mut evidence, row.evidence.clone());
            push_local_db_row_evidence(&mut evidence, &row.inventory_entry_id);
        }
    }
    for row in &model.running_now_rows {
        if is_unknown_review_classification(&row.classification_state) {
            push_evidence(&mut evidence, row.evidence.clone());
            push_local_db_row_evidence(&mut evidence, &row.runtime_evidence_id);
        }
    }
    for row in &model.foreground_now_rows {
        if is_unknown_review_classification(&row.classification_state) {
            push_evidence(&mut evidence, row.evidence.clone());
            push_local_db_row_evidence(&mut evidence, &row.foreground_evidence_id);
        }
    }
    for row in &model.launcher_rows {
        if is_unknown_review_classification(&row.classification_state) {
            push_evidence(&mut evidence, row.evidence.clone());
            push_local_db_row_evidence(&mut evidence, &row.launcher_evidence_id);
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

pub(super) fn category_candidate_row_count(model: &AppGameServiceReadModel) -> u64 {
    model
        .inventory_rows
        .iter()
        .map(|row| row.category_candidates.len() as u64)
        .sum()
}

pub(super) fn unknown_review_row_count(model: &AppGameServiceReadModel) -> u64 {
    let inventory = model
        .inventory_rows
        .iter()
        .filter(|row| is_unknown_review_classification(&row.classification_state))
        .count() as u64;
    let runtime = model
        .running_now_rows
        .iter()
        .filter(|row| is_unknown_review_classification(&row.classification_state))
        .count() as u64;
    let foreground = model
        .foreground_now_rows
        .iter()
        .filter(|row| is_unknown_review_classification(&row.classification_state))
        .count() as u64;
    let launcher = model
        .launcher_rows
        .iter()
        .filter(|row| is_unknown_review_classification(&row.classification_state))
        .count() as u64;
    inventory + runtime + foreground + launcher
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

fn is_unknown_review_classification(classification_state: &str) -> bool {
    classification_state == APP_GAME_CLASSIFICATION_UNKNOWN_PROCESS
        || classification_state == APP_GAME_CLASSIFICATION_POSSIBLY_GAME
        || classification_state == APP_GAME_CLASSIFICATION_LAUNCHER_GAME_CANDIDATE
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
