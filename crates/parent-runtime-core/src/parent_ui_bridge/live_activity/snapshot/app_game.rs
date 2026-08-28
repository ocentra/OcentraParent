use super::*;

use ocentra_parent_agent_protocol::app_game::APP_GAME_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::{
    AppGameTimerParentSurfaceReadModel,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_COMPILER_DECISION,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE,
    APP_GAME_TIMER_PARENT_SURFACE_STATE_RUNTIME_MANUAL_REQUIRED,
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_NO_ROWS, APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL,
    APP_GAME_TIMER_PARENT_SURFACE_STATUS_READY, APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP,
    APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME,
};

pub(super) fn apply_app_game_live_activity_impl(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
    if input.app_game_notification_readiness_snapshot.is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_notification_parent_surface_panel =
            Some(app_game_notification_parent_surface_panel_snapshot(
                input
                    .app_game_notification_readiness_snapshot
                    .map(|snapshot| &snapshot.read_model),
                input
                    .app_game_notification_readiness_snapshot
                    .and_then(|snapshot| snapshot.status_read_models.as_ref()),
            ));
    }
    if input.app_game_policy_readiness_snapshot.is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_policy_readiness_panel = Some(app_game_policy_readiness_panel_snapshot(
            input
                .app_game_policy_readiness_snapshot
                .map(|snapshot| &snapshot.read_model),
        ));
    }
    if input.app_game_platform_proof_status_snapshot.is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_platform_proof_status_panel =
            Some(app_game_platform_proof_status_panel_snapshot(
                input
                    .app_game_platform_proof_status_snapshot
                    .map(|snapshot| &snapshot.read_model),
            ));
    }
    if input
        .app_game_child_runtime_transport_receipt_snapshot
        .is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_child_runtime_transport_receipt_panel =
            Some(app_game_child_runtime_transport_receipt_panel_snapshot(
                input
                    .app_game_child_runtime_transport_receipt_snapshot
                    .map(|snapshot| &snapshot.read_model),
            ));
    }
    if input.app_game_adapter_dispatch_preflight_snapshot.is_some()
        || input.app_game_adapter_dispatch_result_snapshot.is_some()
        || input.app_game_adapter_dispatch_execute_result.is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_adapter_dispatch_panel = Some(app_game_adapter_dispatch_panel_snapshot(
            input
                .app_game_adapter_dispatch_preflight_snapshot
                .map(|snapshot| &snapshot.read_model),
            input
                .app_game_adapter_dispatch_result_snapshot
                .map(|snapshot| &snapshot.read_model),
            input.app_game_adapter_dispatch_execute_result,
        ));
    }
    let consumable_timer_parent_surface_snapshot = input
        .app_game_timer_parent_surface_snapshot
        .and_then(|snapshot| {
            app_game_timer_parent_surface_read_model_is_consumable(&snapshot.read_model)
                .then_some(snapshot)
        });
    if input.app_game_timer_parent_surface_snapshot.is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_timer_parent_surface_panel =
            Some(app_game_timer_parent_surface_panel_snapshot(
                consumable_timer_parent_surface_snapshot.map(|snapshot| &snapshot.read_model),
            ));
    }
}

fn app_game_timer_parent_surface_read_model_is_consumable(
    read_model: &AppGameTimerParentSurfaceReadModel,
) -> bool {
    if read_model.schema_version != APP_GAME_SCHEMA_VERSION
        || read_model.generated_at.trim().is_empty()
        || read_model.custody_label.trim().is_empty()
        || read_model.raw_private_source_rows_included
        || read_model.timer_runtime_claimed
        || read_model.scheduler_persistence_claimed
        || read_model.durable_scheduler_storage_claimed
        || read_model.adapter_dispatch_claimed
        || read_model.platform_enforcement_claimed
        || read_model.child_delivery_claimed
        || read_model.returned != read_model.rows.len() as u64
    {
        return false;
    }

    if !read_model
        .child_ux_local_handoff_artifact_records
        .iter()
        .all(|record| {
            !record.child_delivery_claimed
                && !record.notification_delivery_claimed
                && !record.raw_private_source_rows_included
        })
        || !read_model
            .child_ux_parent_surface_intent_records
            .iter()
            .all(|record| {
                !record.sensitive_detail_included
                    && !record.parent_notification_ui_rendered
                    && !record.parent_preference_mutation_claimed
                    && !record.provider_delivery_claimed
                    && !record.child_delivery_claimed
                    && !record.raw_private_source_rows_included
            })
        || !read_model
            .child_ux_parent_preference_setup_records
            .iter()
            .all(|record| {
                !record.parent_preference_ui_rendered
                    && !record.parent_frequency_control_ui_rendered
                    && !record.parent_preference_mutation_claimed
                    && !record.notification_rule_mutation_claimed
                    && !record.provider_delivery_claimed
                    && !record.child_delivery_claimed
                    && !record.raw_private_source_rows_included
            })
    {
        return false;
    }

    let mut ready_for_parent_surface_count = 0_u64;
    let mut blocked_by_source_freshness_count = 0_u64;
    let mut blocked_by_compiler_decision_count = 0_u64;
    let mut runtime_manual_required_count = 0_u64;
    let mut row_ids = std::collections::HashSet::new();
    let mut evidence_ids = std::collections::HashSet::new();

    for row in &read_model.rows {
        if row.schema_version != APP_GAME_SCHEMA_VERSION
            || row.row_id.trim().is_empty()
            || !row_ids.insert(row.row_id.as_str())
            || !matches!(
                row.target_domain.as_str(),
                APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_APP
                    | APP_GAME_TIMER_PARENT_SURFACE_TARGET_NATIVE_GAME
            )
            || row.row_count != row.evidence_reference_ids.len() as u64
            || row.evidence_reference_ids.len() != row.evidence.len()
        {
            return false;
        }

        match row.timer_surface_state.as_str() {
            APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_SOURCE_FRESHNESS => {
                blocked_by_source_freshness_count += 1;
            }
            APP_GAME_TIMER_PARENT_SURFACE_STATE_BLOCKED_BY_COMPILER_DECISION => {
                blocked_by_compiler_decision_count += 1;
            }
            APP_GAME_TIMER_PARENT_SURFACE_STATE_RUNTIME_MANUAL_REQUIRED => {
                runtime_manual_required_count += 1;
            }
            APP_GAME_TIMER_PARENT_SURFACE_STATE_READY_FOR_PARENT_SURFACE => {
                ready_for_parent_surface_count += 1;
            }
            _ => return false,
        }

        for (reference_id, evidence) in row.evidence_reference_ids.iter().zip(row.evidence.iter()) {
            if reference_id.trim().is_empty()
                || reference_id != &evidence.evidence_id
                || evidence.evidence_id.trim().is_empty()
                || !evidence_ids.insert(evidence.evidence_id.as_str())
            {
                return false;
            }
        }
    }

    if read_model.ready_for_parent_surface_count != ready_for_parent_surface_count
        || read_model.blocked_by_source_freshness_count != blocked_by_source_freshness_count
        || read_model.blocked_by_compiler_decision_count != blocked_by_compiler_decision_count
        || read_model.runtime_manual_required_count != runtime_manual_required_count
    {
        return false;
    }

    let expected_status = if read_model.returned == 0 {
        APP_GAME_TIMER_PARENT_SURFACE_STATUS_NO_ROWS
    } else if ready_for_parent_surface_count == read_model.returned {
        APP_GAME_TIMER_PARENT_SURFACE_STATUS_READY
    } else {
        APP_GAME_TIMER_PARENT_SURFACE_STATUS_PARTIAL
    };
    read_model.capability_status == expected_status
}
