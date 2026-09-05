use std::collections::HashSet;

use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::{
    AppGameTimerParentSurfaceReadModel, AppGameTimerParentSurfaceRow,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_COMPILER_DECISION,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_RUNTIME_MANUAL_REQUIRED,
    APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP,
    APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
};

#[derive(Default)]
struct RowCounts {
    ready_for_parent_surface: u64,
    blocked_by_source_freshness: u64,
    blocked_by_compiler_decision: u64,
    runtime_manual_required: u64,
}

pub(super) fn valid(read_model: &AppGameTimerParentSurfaceReadModel) -> bool {
    let mut counts = RowCounts::default();
    let mut row_ids = HashSet::<String>::new();
    let mut evidence_ids = HashSet::<String>::new();
    for row in &read_model.rows {
        if !valid_row(row, &mut row_ids) {
            return false;
        }
        if !counts.record(row) {
            return false;
        }
        if !valid_evidence(row, &mut evidence_ids) {
            return false;
        }
    }
    counts.match_read_model(read_model)
        && super::status_matches(read_model, counts.ready_for_parent_surface)
}

fn valid_row(row: &AppGameTimerParentSurfaceRow, row_ids: &mut HashSet<String>) -> bool {
    row.schema_version == APP_GAME_SCHEMA_VERSION
        && !row.row_id.trim().is_empty()
        && row_ids.insert(row.row_id.clone())
        && valid_target(row)
        && row.row_count == row.evidence_reference_ids.len() as u64
        && row.evidence_reference_ids.len() == row.evidence.len()
}

fn valid_target(row: &AppGameTimerParentSurfaceRow) -> bool {
    matches!(
        row.target_domain.as_str(),
        APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP
            | APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME
    )
}

fn valid_evidence(row: &AppGameTimerParentSurfaceRow, evidence_ids: &mut HashSet<String>) -> bool {
    row.evidence_reference_ids
        .iter()
        .zip(row.evidence.iter())
        .all(|(reference_id, evidence)| {
            !reference_id.trim().is_empty()
                && reference_id == &evidence.evidence_id
                && !evidence.evidence_id.trim().is_empty()
                && evidence_ids.insert(evidence.evidence_id.clone())
        })
}

impl RowCounts {
    fn record(&mut self, row: &AppGameTimerParentSurfaceRow) -> bool {
        match row.timer_surface_state.as_str() {
            APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS => {
                self.blocked_by_source_freshness += 1;
            }
            APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_COMPILER_DECISION => {
                self.blocked_by_compiler_decision += 1;
            }
            APP_GAME_TIMER_PARENT_SURFACE_STATE_RUNTIME_MANUAL_REQUIRED => {
                self.runtime_manual_required += 1;
            }
            APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE => {
                self.ready_for_parent_surface += 1;
            }
            _ => return false,
        }
        true
    }

    fn match_read_model(&self, read_model: &AppGameTimerParentSurfaceReadModel) -> bool {
        read_model.ready_for_parent_surface_count == self.ready_for_parent_surface
            && read_model.blocked_by_source_freshness_count == self.blocked_by_source_freshness
            && read_model.blocked_by_compiler_decision_count == self.blocked_by_compiler_decision
            && read_model.runtime_manual_required_count == self.runtime_manual_required
    }
}
