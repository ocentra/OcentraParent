use super::*;

use ocentra_parent_agent_protocol::app_game_authority_classifier::APP_GAME_PARENT_PLATFORM_WINDOWS;
use ocentra_parent_agent_protocol::app_game_platform_proof_status::{
    AppGamePlatformProofStatusReadModel, AppGamePlatformProofStatusRow,
};

mod app_game_timer_surface_validation;

pub(super) fn apply_app_game_live_activity_impl(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
    apply_notification_panel(input, snapshot);
    apply_policy_panel(input, snapshot);
    apply_platform_proof(input, snapshot);
    apply_child_transport_panel(input, snapshot);
    apply_adapter_dispatch_panel(input, snapshot);
    apply_timer_parent_surface_panel(input, snapshot);
}

fn apply_notification_panel(
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
}

fn apply_policy_panel(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
    if input.app_game_policy_readiness_snapshot.is_some()
        || matches!(input.route, ParentRouteId::AppGameSessions)
    {
        snapshot.app_game_policy_readiness_panel = Some(app_game_policy_readiness_panel_snapshot(
            input
                .app_game_policy_readiness_snapshot
                .map(|snapshot| &snapshot.read_model),
        ));
    }
}

fn apply_platform_proof(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
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
    snapshot.activity_app_game_platform_extension_read_model = input
        .app_game_platform_proof_status_snapshot
        .map(|snapshot| app_game_platform_extension_adapter_value(&snapshot.read_model));
}

fn apply_child_transport_panel(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
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
}

fn apply_adapter_dispatch_panel(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
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
}

fn apply_timer_parent_surface_panel(
    input: &ParentRouteLiveActivitySnapshotInput<'_>,
    snapshot: &mut ParentRouteLiveActivitySnapshot,
) {
    let consumable_timer_parent_surface_snapshot = input
        .app_game_timer_parent_surface_snapshot
        .and_then(|snapshot| {
            app_game_timer_surface_validation::is_consumable(&snapshot.read_model)
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

fn app_game_platform_extension_adapter_value(
    read_model: &AppGamePlatformProofStatusReadModel,
) -> serde_json::Value {
    let rows = read_model
        .rows
        .iter()
        .filter_map(app_game_platform_extension_row_value)
        .collect::<Vec<_>>();
    let state =
        if rows.is_empty() {
            "unavailable"
        } else if read_model.rows.iter().any(|row| {
            row.platform != APP_GAME_PARENT_PLATFORM_WINDOWS && !row.open_gaps.is_empty()
        }) {
            "manual-required"
        } else {
            "ready"
        };
    serde_json::json!({
        "ok": true,
        "value": {
            "schemaVersion": read_model.schema_version,
            "state": state,
            "generatedAt": read_model.generated_at,
            "summary": "App/game platform extension proof-pack readiness from service projection",
            "rows": rows,
        },
    })
}

fn app_game_platform_extension_row_value(
    row: &AppGamePlatformProofStatusRow,
) -> Option<serde_json::Value> {
    if row.platform == APP_GAME_PARENT_PLATFORM_WINDOWS {
        return None;
    }
    let state = if row.open_gaps.is_empty() {
        "ready"
    } else {
        "manual-required"
    };
    let proof_pack_state = if row.open_gaps.is_empty() {
        "proof-pack-ready"
    } else {
        "manual-proof-pack-required"
    };
    Some(serde_json::json!({
        "platform": row.platform,
        "state": state,
        "setupState": state,
        "proofPackState": proof_pack_state,
        "authorityTier": row.authority_state,
        "adapterExecutionClaim": "not-executed",
        "broadBlockingClaimed": row.broad_installed_app_blocking_claimed,
        "privilegedMobileClaimed": false,
        "childDeviceDeliveryClaimed": row.child_device_delivery_claimed,
        "requiredProofRefs": row.proof_refs,
    }))
}
