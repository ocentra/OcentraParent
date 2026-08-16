use ocentra_parent_agent_protocol::activity_surface::source_status::ActivityAppGameSourceStatusRow;
use ocentra_parent_agent_protocol::app_game::{
    AppGameForegroundEvidenceRow, AppGameInventoryEvidenceRow, AppGameLauncherEvidenceRow,
    AppGameRuntimeEvidenceRow, AppGameServiceReadModel, APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_DEGRADED, APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED,
    APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED, APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED,
    APP_GAME_CAPABILITY_STATUS_STALE, APP_GAME_CAPABILITY_STATUS_UNAVAILABLE,
    APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
};

use crate::activity_surface_read_models::shared::{
    push_evidence, row_state, CapabilityStatus, ObservedAt, SourceKind, SourceStatusRowInput,
};

const SOURCE_STATUS_PRECEDENCE_ORDER: [&str; 8] = [
    APP_GAME_CAPABILITY_STATUS_ADAPTER_ERROR,
    APP_GAME_CAPABILITY_STATUS_PERMISSION_LIMITED,
    APP_GAME_CAPABILITY_STATUS_DEGRADED,
    APP_GAME_CAPABILITY_STATUS_STALE,
    APP_GAME_CAPABILITY_STATUS_UNAVAILABLE,
    APP_GAME_CAPABILITY_STATUS_UNSUPPORTED_PLATFORM,
    APP_GAME_CAPABILITY_STATUS_MANUAL_REQUIRED,
    APP_GAME_CAPABILITY_STATUS_NOT_CLAIMED,
];

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
        push_source_status_row(SourceStatusRowInput {
            target: &mut rows,
            source_kind: SourceKind(row.source_kind.clone()),
            observed_at: ObservedAt(row.observed_at.clone()),
            capability_status: CapabilityStatus(row.capability_status.clone()),
            evidence: &row.evidence,
        });
    }
    for row in model
        .running_now_rows
        .iter()
        .filter(|row| runtime_filter(row))
    {
        push_source_status_row(SourceStatusRowInput {
            target: &mut rows,
            source_kind: SourceKind(row.observation_mode.clone()),
            observed_at: ObservedAt(row.observed_at.clone()),
            capability_status: CapabilityStatus(row.capability_status.clone()),
            evidence: &row.evidence,
        });
    }
    for row in model
        .foreground_now_rows
        .iter()
        .filter(|row| foreground_filter(row))
    {
        push_source_status_row(SourceStatusRowInput {
            target: &mut rows,
            source_kind: SourceKind(row.observation_mode.clone()),
            observed_at: ObservedAt(row.observed_at.clone()),
            capability_status: CapabilityStatus(row.capability_status.clone()),
            evidence: &row.evidence,
        });
    }
    if include_launcher_rows {
        for row in &model.launcher_rows {
            push_launcher_source_status_row(&mut rows, row);
        }
    }
    rows.sort_by(|left, right| left.source_kind.cmp(&right.source_kind));
    rows
}

fn push_launcher_source_status_row(
    target: &mut Vec<ActivityAppGameSourceStatusRow>,
    row: &AppGameLauncherEvidenceRow,
) {
    push_source_status_row(SourceStatusRowInput {
        target,
        source_kind: SourceKind(row.observation_mode.clone()),
        observed_at: ObservedAt(row.observed_at.clone()),
        capability_status: CapabilityStatus(row.capability_status.clone()),
        evidence: &row.evidence,
    });
}

fn push_source_status_row(input: SourceStatusRowInput<'_>) {
    if let Some(row) = input
        .target
        .iter_mut()
        .find(|candidate| candidate.source_kind == input.source_kind.0)
    {
        row.row_count += 1;
        if row
            .last_observed_at
            .as_deref()
            .map(|current| input.observed_at.0.as_str() > current)
            .unwrap_or(true)
        {
            row.last_observed_at = Some(input.observed_at.0.clone());
        }
        if source_status_precedence(&input.capability_status)
            < source_status_precedence(&CapabilityStatus(row.capability_status.clone()))
        {
            row.capability_status = input.capability_status.0.clone();
            row.state = row_state(&input.capability_status);
        }
        push_evidence(&mut row.evidence, input.evidence);
        return;
    }
    let mut source_evidence = Vec::new();
    push_evidence(&mut source_evidence, input.evidence);
    input.target.push(ActivityAppGameSourceStatusRow {
        source_kind: input.source_kind.0,
        state: row_state(&input.capability_status),
        row_count: 1,
        last_observed_at: Some(input.observed_at.0),
        capability_status: input.capability_status.0,
        evidence: source_evidence,
    });
}

fn source_status_precedence(capability_status: &CapabilityStatus) -> u8 {
    SOURCE_STATUS_PRECEDENCE_ORDER
        .iter()
        .position(|status| capability_status.0 == *status)
        .map(|index| index as u8)
        .unwrap_or(8)
}
