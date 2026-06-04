use ocentra_parent_agent_protocol::{
    constants, ActivityEvidenceKind, ActivityEvidenceRef, ActivityReadModelState,
    ActivitySurfaceRequest, AppGameServiceReadModel, APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNAVAILABLE, APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
};

pub(super) fn row_device_id(request: &ActivitySurfaceRequest) -> String {
    request
        .scope
        .device_id
        .clone()
        .unwrap_or_else(|| constants::activity_surface::DEFAULT_DEVICE_ID.to_string())
}

pub(super) fn row_state(capability_status: &str) -> ActivityReadModelState {
    match capability_status {
        APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED => ActivityReadModelState::PermissionRequired,
        APP_GAME_CAPABILITY_STATUS_STALE => ActivityReadModelState::Stale,
        APP_GAME_CAPABILITY_STATUS_UNAVAILABLE
        | APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM
        | APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR => ActivityReadModelState::Unavailable,
        _ => ActivityReadModelState::Ready,
    }
}

pub(super) fn push_evidence(target: &mut Vec<ActivityEvidenceRef>, rows: &[ActivityEvidenceRef]) {
    for evidence in rows {
        if target
            .iter()
            .any(|candidate| candidate.evidence_id == evidence.evidence_id)
        {
            continue;
        }
        target.push(evidence.clone());
    }
}

pub(super) fn push_app_game_boundary_evidence(
    target: &mut Vec<ActivityEvidenceRef>,
    model: &AppGameServiceReadModel,
) {
    for row in &model.evidence_claim_rows {
        push_evidence(target, &row.evidence);
        push_local_db_row_evidence(target, &row.claim_id);
    }
    for row in &model.identity_rows {
        push_evidence(target, &row.evidence);
        push_local_db_row_evidence(target, &row.identity_id);
    }
    for row in &model.approval_authority_rows {
        push_local_db_row_evidence(target, &row.authority_id);
    }
    for row in &model.approval_action_result_rows {
        push_local_db_row_evidence(target, &row.result_id);
    }
    for matrix in &model.platform_authority_matrices {
        push_local_db_row_evidence(target, &matrix.matrix_id);
        for row in &matrix.rows {
            push_local_db_row_evidence(target, &row.row_id);
        }
    }
    for row in &model.ai_classifier_result_rows {
        push_local_db_row_evidence(target, &row.classifier_run_id);
        for evidence_ref in &row.source_evidence_refs {
            push_local_db_row_evidence(target, evidence_ref);
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct AppGameBoundaryRowCounts {
    pub evidence_claim_row_count: u64,
    pub identity_row_count: u64,
    pub approval_authority_row_count: u64,
    pub approval_action_result_row_count: u64,
    pub platform_authority_matrix_count: u64,
    pub platform_authority_row_count: u64,
    pub ai_classifier_result_row_count: u64,
}

pub(super) fn app_game_boundary_row_counts(
    model: &AppGameServiceReadModel,
) -> AppGameBoundaryRowCounts {
    AppGameBoundaryRowCounts {
        evidence_claim_row_count: model.evidence_claim_rows.len() as u64,
        identity_row_count: model.identity_rows.len() as u64,
        approval_authority_row_count: model.approval_authority_rows.len() as u64,
        approval_action_result_row_count: model.approval_action_result_rows.len() as u64,
        platform_authority_matrix_count: model.platform_authority_matrices.len() as u64,
        platform_authority_row_count: model
            .platform_authority_matrices
            .iter()
            .map(|matrix| matrix.rows.len() as u64)
            .sum(),
        ai_classifier_result_row_count: model.ai_classifier_result_rows.len() as u64,
    }
}

fn push_local_db_row_evidence(target: &mut Vec<ActivityEvidenceRef>, evidence_id: &str) {
    if evidence_id.is_empty() {
        return;
    }
    push_evidence(
        target,
        &[ActivityEvidenceRef {
            evidence_id: evidence_id.to_string(),
            kind: ActivityEvidenceKind::LocalDbRow,
            digest: None,
            uri: None,
        }],
    );
}
