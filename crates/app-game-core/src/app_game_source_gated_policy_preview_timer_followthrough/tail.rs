use super::*;
use super::app_game_source_gated_policy_preview_timer_followthrough_protocol::*;
use super::app_game_source_gated_policy_preview_timer_followthrough_parent_surface::*;
use super::app_game_source_gated_policy_preview_timer_followthrough_parent_surface_status::*;

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
                options,
                index,
                row,
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
    include_str!("generated/app-game-source-gated-policy-preview-timer-followthrough.ts")
}

fn protocol_read_model_state_for_handoff(protocol_handoff_state: &str) -> &'static str {
    match protocol_handoff_state {
        "protocol-proof-required" => PROTOCOL_READ_MODEL_PROOF_REQUIRED,
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn protocol_command_handoff_state_for_read_model(protocol_read_model_state: &str) -> &'static str {
    match protocol_read_model_state {
        PROTOCOL_READ_MODEL_PROOF_REQUIRED => PROTOCOL_COMMAND_HANDOFF_PROOF_REQUIRED,
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn service_handler_state_for_command_handoff(protocol_command_handoff_state: &str) -> &'static str {
    match protocol_command_handoff_state {
        PROTOCOL_COMMAND_HANDOFF_PROOF_REQUIRED => SERVICE_HANDLER_PROOF_REQUIRED,
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn service_read_api_state_for_service_handler_handoff(
    service_handler_handoff_state: &str,
) -> &'static str {
    match service_handler_handoff_state {
        SERVICE_HANDLER_PROOF_REQUIRED => SERVICE_READ_API_PROOF_REQUIRED,
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn read_api_response_state_for_read_api_handoff(
    service_read_api_handoff_state: &str,
) -> &'static str {
    match service_read_api_handoff_state {
        SERVICE_READ_API_PROOF_REQUIRED => READ_API_RESPONSE_PROOF_REQUIRED,
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn read_api_response_consumer_state_for_response_handoff(
    read_api_response_handoff_state: &str,
) -> &'static str {
    match read_api_response_handoff_state {
        READ_API_RESPONSE_PROOF_REQUIRED => READ_API_RESPONSE_CONSUMER_PROOF_REQUIRED,
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn response_consumer_parent_surface_state_for_read_api_response_consumer_handoff(
    read_api_response_consumer_handoff_state: &str,
) -> &'static str {
    match read_api_response_consumer_handoff_state {
        READ_API_RESPONSE_CONSUMER_PROOF_REQUIRED => PARENT_SURFACE_PROOF_REQUIRED,
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn response_consumer_parent_surface_read_model_state_for_parent_surface_handoff(
    parent_surface_handoff_state: &str,
) -> &'static str {
    match parent_surface_handoff_state {
        PARENT_SURFACE_PROOF_REQUIRED => PARENT_SURFACE_READ_MODEL_PROOF_REQUIRED,
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn response_consumer_parent_surface_status_state_for_read_model_handoff(
    parent_surface_read_model_handoff_state: &str,
) -> &'static str {
    match parent_surface_read_model_handoff_state {
        PARENT_SURFACE_READ_MODEL_PROOF_REQUIRED => PARENT_SURFACE_STATUS_PROOF_REQUIRED,
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn response_consumer_parent_surface_status_read_model_state_for_status_handoff(
    parent_surface_status_handoff_state: &str,
) -> &'static str {
    match parent_surface_status_handoff_state {
        PARENT_SURFACE_STATUS_PROOF_REQUIRED => PARENT_SURFACE_STATUS_READ_MODEL_PROOF_REQUIRED,
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn parent_surface_status_read_model_parent_surface_state_for_status_read_model_handoff(
    parent_surface_status_read_model_handoff_state: &str,
) -> &'static str {
    match parent_surface_status_read_model_handoff_state {
        PARENT_SURFACE_STATUS_READ_MODEL_PROOF_REQUIRED => {
            PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_PROOF_REQUIRED
        }
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn parent_surface_status_read_model_parent_surface_read_model_handoff_state_for_parent_surface_handoff(
    parent_surface_handoff_state: &str,
) -> &'static str {
    match parent_surface_handoff_state {
        PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_PROOF_REQUIRED => {
            PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_READ_MODEL_PROOF_REQUIRED
        }
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn parent_surface_status_read_model_parent_surface_read_model_state_for_handoff(
    parent_surface_read_model_handoff_state: &str,
) -> &'static str {
    match parent_surface_read_model_handoff_state {
        PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_READ_MODEL_PROOF_REQUIRED => {
            READY_FOR_PARENT_SURFACE_STATUS_READ_MODEL_PARENT_SURFACE_READ_MODEL
        }
        BLOCKED_BY_SOURCE_FRESHNESS => BLOCKED_BY_SOURCE_FRESHNESS,
        _ => BLOCKED_BY_COMPILER_DECISION,
    }
}

fn count_state<T, F>(rows: &[T], state_of: F, needle: &str) -> usize
where
    F: Fn(&T) -> &str,
{
    rows.iter().filter(|row| state_of(row) == needle).count()
}
