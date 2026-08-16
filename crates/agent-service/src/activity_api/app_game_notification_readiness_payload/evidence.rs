use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;
use ocentra_parent_agent_protocol::AppGameNotificationReadinessRow;

#[derive(Clone, Copy)]
pub(super) struct NotificationReadinessTextRef<'a>(pub(super) &'a str);

#[derive(Clone, Copy)]
struct LocalDbRowEvidenceIdRef<'a>(&'a str);

pub(super) fn manual_required_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = policy_evidence_refs(model);
    push_evidence(&mut evidence, approval_authority_refs(model));
    push_evidence(&mut evidence, platform_authority_row_refs(model));
    evidence
}

pub(super) fn policy_evidence_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = evidence_claim_refs(model);
    push_evidence(&mut evidence, identity_refs(model));
    evidence
}

pub(super) fn evidence_claim_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
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

pub(super) fn approval_authority_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
    let mut evidence = Vec::new();
    for row in &model.approval_authority_rows {
        push_local_db_row_evidence(&mut evidence, LocalDbRowEvidenceIdRef(&row.authority_id));
    }
    evidence
}

fn platform_authority_row_refs(model: &AppGameServiceReadModel) -> Vec<ActivityEvidenceRef> {
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

pub(super) fn app_game_boundary_row_count(model: &AppGameServiceReadModel) -> u64 {
    model.evidence_claim_rows.len() as u64
        + model.identity_rows.len() as u64
        + model.approval_authority_rows.len() as u64
        + model.approval_action_result_rows.len() as u64
        + platform_authority_row_count(model)
        + model.ai_classifier_result_rows.len() as u64
}

pub(super) fn count_rows_with_state(
    rows: &[AppGameNotificationReadinessRow],
    state: NotificationReadinessTextRef<'_>,
) -> u64 {
    rows.iter()
        .filter(|row| row.readiness_state == state.0)
        .count() as u64
}

pub(super) fn push_evidence(target: &mut Vec<ActivityEvidenceRef>, rows: Vec<ActivityEvidenceRef>) {
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

fn push_local_db_row_evidence(
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
