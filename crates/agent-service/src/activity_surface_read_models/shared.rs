use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::activity_surface::source_status::ActivityAppGameSourceStatusRow;
use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReadModelState, ActivitySurfaceRequest,
};
use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameRuntimeEvidenceRow,
    AppGameServiceReadModel, APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED, APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNAVAILABLE, APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
};
use ocentra_parent_agent_protocol::constants;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct DeviceId(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct CapabilityStatus(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SourceKind(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct ObservedAt(pub(crate) String);

#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct EvidenceId(pub(crate) String);

pub(super) struct SourceStatusRowInput<'a> {
    pub(super) target: &'a mut Vec<ActivityAppGameSourceStatusRow>,
    pub(super) source_kind: SourceKind,
    pub(super) observed_at: ObservedAt,
    pub(super) capability_status: CapabilityStatus,
    pub(super) evidence: &'a [ActivityEvidenceRef],
}

#[path = "shared_status.rs"]
mod shared_status;

pub(super) fn row_device_id(request: &ActivitySurfaceRequest) -> DeviceId {
    DeviceId(
        request
            .scope
            .device_id
            .clone()
            .unwrap_or_else(|| constants::activity_surface::DEFAULT_DEVICE_ID.to_string()),
    )
}

pub(super) fn row_state(capability_status: &CapabilityStatus) -> ActivityReadModelState {
    match capability_status.0.as_str() {
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
    shared_status::push_app_game_boundary_evidence(target, model);
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
    shared_status::app_game_source_status_rows(
        model,
        inventory_filter,
        runtime_filter,
        foreground_filter,
        include_launcher_rows,
    )
}
