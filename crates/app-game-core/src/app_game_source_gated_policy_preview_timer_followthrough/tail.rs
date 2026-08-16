use super::helpers::{
    count_state,
    parent_surface_status_read_model_parent_surface_read_model_handoff_state_for_parent_surface_handoff,
    parent_surface_status_read_model_parent_surface_read_model_state_for_handoff,
};
use super::parent_surface_status::*;
use super::*;

pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffOptions,
    source_handoff: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoff,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoff{
    let rows = source_handoff
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            build_response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_row(
                options,
                index,
                row,
            )
        })
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoff {
        schema_version: options.schema_version.clone(),
        response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_id:
            options
                .response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_id
                .clone(),
        source_response_consumer_parent_surface_status_read_model_handoff_id: source_handoff
            .response_consumer_parent_surface_status_read_model_parent_surface_handoff_id
            .clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        parent_surface_read_model_ref: options.parent_surface_read_model_ref.clone(),
        native_app_row_count: source_handoff.native_app_row_count,
        native_game_row_count: source_handoff.native_game_row_count,
        parent_surface_read_model_proof_required_count: count_state(
            &rows,
            |row| {
                row.response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_state
                    .as_str()
            },
            PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_READ_MODEL_PROOF_REQUIRED,
        ),
        blocked_by_source_freshness_count: count_state(
            &rows,
            |row| {
                row.response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_state
                    .as_str()
            },
            BLOCKED_BY_SOURCE_FRESHNESS,
        ),
        blocked_by_compiler_decision_count: count_state(
            &rows,
            |row| {
                row.response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_state
                    .as_str()
            },
            BLOCKED_BY_COMPILER_DECISION,
        ),
        response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_non_claims:
            RESPONSE_CONSUMER_PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_READ_MODEL_HANDOFF_NON_CLAIMS
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
        rows,
        ..Default::default()
    }
}

pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_parent_surface_read_model(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptions,
    handoff: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoff,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel{
    let rows = handoff
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            build_response_consumer_parent_surface_status_read_model_parent_surface_read_model_row(
                options, index, row,
            )
        })
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel {
        schema_version: options.schema_version.clone(),
        parent_surface_read_model_id: options.parent_surface_read_model_id.clone(),
        source_parent_surface_read_model_handoff_id: handoff
            .response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_id
            .clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        ready_for_parent_surface_read_model_count: count_state(
            &rows,
            |row| row.parent_surface_read_model_state.as_str(),
            READY_FOR_PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_READ_MODEL,
        ),
        blocked_by_source_freshness_count: count_state(
            &rows,
            |row| row.parent_surface_read_model_state.as_str(),
            BLOCKED_BY_SOURCE_FRESHNESS,
        ),
        blocked_by_compiler_decision_count: count_state(
            &rows,
            |row| row.parent_surface_read_model_state.as_str(),
            BLOCKED_BY_COMPILER_DECISION,
        ),
        native_app_row_count: handoff.native_app_row_count,
        native_game_row_count: handoff.native_game_row_count,
        parent_surface_read_model_non_claims:
            RESPONSE_CONSUMER_PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_READ_MODEL_NON_CLAIMS
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
        rows,
        service_command_registered: false,
        service_handler_implemented: false,
        service_read_api_implemented: false,
        service_read_api_response_implemented: false,
        service_read_api_response_consumer_implemented: false,
        service_event_emitted: false,
        agent_protocol_implemented: false,
        rust_protocol_mirrored: false,
        portal_ui_rendered: false,
        portal_response_consumer_rendered: false,
        parent_surface_rendered: false,
        parent_surface_read_model_runtime_implemented: false,
        parent_surface_read_model_persisted: false,
        policy_evaluator_runtime_claimed: false,
        timer_runtime_claimed: false,
        timer_scheduled: false,
        scheduler_persistence_runtime_claimed: false,
        durable_scheduler_storage_claimed: false,
        audit_runtime_claimed: false,
        durable_audit_log_claimed: false,
        rollback_runtime_claimed: false,
        rollback_execution_claimed: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
    }
}

pub fn app_game_source_gated_policy_preview_timer_followthrough_typescript() -> &'static str {
    include_str!(
        "../../tests/generated/app-game-source-gated-policy-preview-timer-followthrough.ts"
    )
}

fn build_response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffOptions,
    index: usize,
    row: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffRow,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRow{
    let state = parent_surface_status_read_model_parent_surface_read_model_handoff_state_for_parent_surface_handoff(
        &row.response_consumer_parent_surface_status_read_model_parent_surface_handoff_state,
    );
    let required =
        state == PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_READ_MODEL_PROOF_REQUIRED;
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRow {
        schema_version: options.schema_version.clone(),
        row_id: format!(
            "{}-row-{}",
            options
                .response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_id,
            index + 1
        ),
        source_response_consumer_parent_surface_status_read_model_handoff_row_id: row
            .row_id
            .clone(),
        target_domain: row.target_domain.clone(),
        response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_state:
            state.to_string(),
        inherited_parent_surface_status_read_model_proof_refs: row
            .required_parent_surface_proof_refs
            .clone(),
        required_parent_surface_read_model_proof_refs: if required {
            options.parent_surface_read_model_proof_refs.clone()
        } else {
            vec![]
        },
        source_evidence_refs: row.source_evidence_refs.clone(),
        parent_surface_status_read_model_ref: row.parent_surface_ref.clone(),
        parent_surface_read_model_ref: options.parent_surface_read_model_ref.clone(),
        service_command_registered: false,
        service_handler_implemented: false,
        service_read_api_implemented: false,
        service_read_api_response_implemented: false,
        service_read_api_response_consumer_implemented: false,
        parent_surface_read_model_implemented: false,
        parent_surface_status_implemented: false,
        parent_surface_status_read_model_implemented: false,
        parent_surface_status_read_model_parent_surface_implemented: false,
        parent_surface_status_read_model_parent_surface_read_model_implemented: false,
        service_event_emitted: false,
        agent_protocol_implemented: false,
        rust_protocol_mirrored: false,
        portal_ui_rendered: false,
        portal_response_consumer_rendered: false,
        parent_surface_rendered: false,
        policy_evaluator_runtime_claimed: false,
        timer_runtime_claimed: false,
        timer_scheduled: false,
        scheduler_persistence_runtime_claimed: false,
        durable_scheduler_storage_claimed: false,
        audit_runtime_claimed: false,
        durable_audit_log_claimed: false,
        rollback_runtime_claimed: false,
        rollback_execution_claimed: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
        generated_at: options.generated_at.clone(),
    }
}

fn build_response_consumer_parent_surface_status_read_model_parent_surface_read_model_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptions,
    index: usize,
    row: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffRow,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRow{
    let state = parent_surface_status_read_model_parent_surface_read_model_state_for_handoff(
        &row.response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_state,
    );
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}-row-{}", options.parent_surface_read_model_id, index + 1),
        source_parent_surface_read_model_handoff_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        parent_surface_read_model_state: state.to_string(),
        parent_safe_summary: format!("{}:{}", row.target_domain, state),
        inherited_parent_surface_status_read_model_proof_refs: row
            .inherited_parent_surface_status_read_model_proof_refs
            .clone(),
        required_parent_surface_read_model_proof_refs: row
            .required_parent_surface_read_model_proof_refs
            .clone(),
        source_evidence_refs: row.source_evidence_refs.clone(),
        parent_surface_status_read_model_ref: row.parent_surface_status_read_model_ref.clone(),
        parent_surface_read_model_ref: row.parent_surface_read_model_ref.clone(),
        service_command_registered: false,
        service_handler_implemented: false,
        service_read_api_implemented: false,
        service_read_api_response_implemented: false,
        service_read_api_response_consumer_implemented: false,
        service_event_emitted: false,
        agent_protocol_implemented: false,
        rust_protocol_mirrored: false,
        portal_ui_rendered: false,
        portal_response_consumer_rendered: false,
        parent_surface_rendered: false,
        parent_surface_read_model_runtime_implemented: false,
        parent_surface_read_model_persisted: false,
        policy_evaluator_runtime_claimed: false,
        timer_runtime_claimed: false,
        timer_scheduled: false,
        scheduler_persistence_runtime_claimed: false,
        durable_scheduler_storage_claimed: false,
        audit_runtime_claimed: false,
        durable_audit_log_claimed: false,
        rollback_runtime_claimed: false,
        rollback_execution_claimed: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
        generated_at: options.generated_at.clone(),
    }
}
