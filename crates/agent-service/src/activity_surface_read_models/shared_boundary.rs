use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::app_game::AppGameServiceReadModel;

use crate::activity_surface_read_models::shared::{push_evidence, EvidenceId};

pub(super) fn push_app_game_boundary_evidence(
    target: &mut Vec<ActivityEvidenceRef>,
    model: &AppGameServiceReadModel,
) {
    for row in &model.evidence_claim_rows {
        push_evidence(target, &row.evidence);
        push_local_db_row_evidence(target, EvidenceId(row.claim_id.clone()));
    }
    for row in &model.identity_rows {
        push_evidence(target, &row.evidence);
        push_local_db_row_evidence(target, EvidenceId(row.identity_id.clone()));
    }
    for row in &model.approval_authority_rows {
        push_local_db_row_evidence(target, EvidenceId(row.authority_id.clone()));
    }
    for row in &model.approval_action_result_rows {
        push_local_db_row_evidence(target, EvidenceId(row.result_id.clone()));
    }
    for matrix in &model.platform_authority_matrices {
        push_local_db_row_evidence(target, EvidenceId(matrix.matrix_id.clone()));
        for row in &matrix.rows {
            push_local_db_row_evidence(target, EvidenceId(row.row_id.clone()));
        }
    }
    for row in &model.ai_classifier_result_rows {
        push_local_db_row_evidence(target, EvidenceId(row.classifier_run_id.clone()));
        for evidence_ref in &row.source_evidence_refs {
            push_local_db_row_evidence(target, EvidenceId(evidence_ref.clone()));
        }
    }
}

fn push_local_db_row_evidence(target: &mut Vec<ActivityEvidenceRef>, evidence_id: EvidenceId) {
    if evidence_id.0.is_empty() {
        return;
    }
    push_evidence(
        target,
        &[ActivityEvidenceRef {
            evidence_id: evidence_id.0,
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        }],
    );
}
