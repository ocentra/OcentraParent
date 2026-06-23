use ocentra_parent_agent_protocol::activity::{ActivityEvidenceKind, ActivityEvidenceRef};
use ocentra_parent_agent_protocol::activity_surface::source_status::ActivityAppGameSourceStatusRow;
use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReadModelState, ActivitySurfaceRequest,
};
use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameLauncherEvidenceRow,
    AppGameRuntimeEvidenceRow, AppGameServiceReadModel, APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_DEGRADED, APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED,
    APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED, APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED,
    APP_GAME_CAPABILITY_STATUS_STALE, APP_GAME_CAPABILITY_STATUS_UNAVAILABLE,
    APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
};
use ocentra_parent_agent_protocol::constants;

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

pub(super) fn app_game_source_status_rows(
    model: &AppGameServiceReadModel,
    inventory_filter: fn(&AppGameInventoryEvidenceRow) -> bool,
    runtime_filter: fn(&AppGameRuntimeEvidenceRow) -> bool,
    foreground_filter: fn(&AppGameForegroundEvidenceRow) -> bool,
    include_launcher_rows: bool,
) -> Vec<ActivityAppGameSourceStatusRow> {
    let mut rows = Vec::new();
    for row in model
        .inventory_rows
        .iter()
        .filter(|row| inventory_filter(row))
    {
        push_source_status_row(
            &mut rows,
            &row.source_kind,
            &row.observed_at,
            &row.capability_status,
            &row.evidence,
        );
    }
    for row in model
        .running_now_rows
        .iter()
        .filter(|row| runtime_filter(row))
    {
        push_source_status_row(
            &mut rows,
            &row.observation_mode,
            &row.observed_at,
            &row.capability_status,
            &row.evidence,
        );
    }
    for row in model
        .foreground_now_rows
        .iter()
        .filter(|row| foreground_filter(row))
    {
        push_source_status_row(
            &mut rows,
            &row.observation_mode,
            &row.observed_at,
            &row.capability_status,
            &row.evidence,
        );
    }
    if include_launcher_rows {
        for row in &model.launcher_rows {
            push_launcher_source_status_row(&mut rows, row);
        }
    }
    rows.sort_by(|left, right| left.source_kind.cmp(&right.source_kind));
    rows
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

fn push_launcher_source_status_row(
    target: &mut Vec<ActivityAppGameSourceStatusRow>,
    row: &AppGameLauncherEvidenceRow,
) {
    push_source_status_row(
        target,
        &row.observation_mode,
        &row.observed_at,
        &row.capability_status,
        &row.evidence,
    );
}

fn push_source_status_row(
    target: &mut Vec<ActivityAppGameSourceStatusRow>,
    source_kind: &str,
    observed_at: &str,
    capability_status: &str,
    evidence: &[ActivityEvidenceRef],
) {
    if let Some(row) = target
        .iter_mut()
        .find(|candidate| candidate.source_kind == source_kind)
    {
        row.row_count += 1;
        if row
            .last_observed_at
            .as_deref()
            .map(|current| observed_at > current)
            .unwrap_or(true)
        {
            row.last_observed_at = Some(observed_at.to_string());
        }
        if source_status_precedence(capability_status)
            < source_status_precedence(&row.capability_status)
        {
            row.capability_status = capability_status.to_string();
            row.state = row_state(capability_status);
        }
        push_evidence(&mut row.evidence, evidence);
        return;
    }
    let mut source_evidence = Vec::new();
    push_evidence(&mut source_evidence, evidence);
    target.push(ActivityAppGameSourceStatusRow {
        source_kind: source_kind.to_string(),
        state: row_state(capability_status),
        row_count: 1,
        last_observed_at: Some(observed_at.to_string()),
        capability_status: capability_status.to_string(),
        evidence: source_evidence,
    });
}

fn source_status_precedence(capability_status: &str) -> u8 {
    match capability_status {
        APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR => 0,
        APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED => 1,
        APP_GAME_CAPABILITY_STATUS_DEGRADED => 2,
        APP_GAME_CAPABILITY_STATUS_STALE => 3,
        APP_GAME_CAPABILITY_STATUS_UNAVAILABLE => 4,
        APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM => 5,
        APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED => 6,
        APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED => 7,
        _ => 8,
    }
}
