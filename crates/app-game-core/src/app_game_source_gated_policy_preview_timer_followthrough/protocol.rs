use super::helpers::{
    count_state, protocol_command_handoff_state_for_read_model,
    protocol_read_model_state_for_handoff, read_api_response_consumer_state_for_response_handoff,
    read_api_response_state_for_read_api_handoff, service_handler_state_for_command_handoff,
    service_read_api_state_for_service_handler_handoff,
};
use super::*;
use crate::app_game_source_gated_policy_preview_timer_chain::{
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRow,
};

fn build_protocol_read_model_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelOptions,
    row: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRow,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRow {
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:protocol-read-model", row.row_id),
        source_protocol_handoff_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        protocol_read_model_state: protocol_read_model_state_for_handoff(
            &row.protocol_handoff_state,
        )
        .to_string(),
        required_protocol_proof_refs: row.required_protocol_proof_refs.clone(),
        inherited_service_readiness_proof_refs: row.inherited_service_readiness_proof_refs.clone(),
        source_evidence_refs: row.source_evidence_refs.clone(),
        service_read_api_ref: row.service_read_api_ref.clone(),
        protocol_summary_ref: options.protocol_summary_ref.clone(),
        agent_protocol_contract_implemented: false,
        rust_protocol_mirrored: false,
        service_command_registered: false,
        service_event_emitted: false,
        service_read_api_implemented: false,
        service_read_model_event_emitted: false,
        portal_ui_rendered: false,
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

pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelOptions,
    handoff: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel {
    let rows = handoff
        .rows
        .iter()
        .map(|row| build_protocol_read_model_row(options, row))
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel {
        schema_version: options.schema_version.clone(),
        read_model_id: options.read_model_id.clone(),
        source_protocol_handoff_id: handoff.handoff_id.clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        protocol_summary_ref: options.protocol_summary_ref.clone(),
        native_app_row_count: handoff.native_app_row_count,
        native_game_row_count: handoff.native_game_row_count,
        protocol_read_model_proof_required_count: count_state(
            &rows,
            |row| row.protocol_read_model_state.as_str(),
            PROTOCOL_READ_MODEL_PROOF_REQUIRED,
        ),
        blocked_by_source_freshness_count: count_state(
            &rows,
            |row| row.protocol_read_model_state.as_str(),
            BLOCKED_BY_SOURCE_FRESHNESS,
        ),
        blocked_by_compiler_decision_count: count_state(
            &rows,
            |row| row.protocol_read_model_state.as_str(),
            BLOCKED_BY_COMPILER_DECISION,
        ),
        protocol_read_model_non_claims: PROTOCOL_READ_MODEL_NON_CLAIMS
            .iter()
            .map(|v| (*v).to_string())
            .collect(),
        rows,
        agent_protocol_contract_implemented: false,
        rust_protocol_mirrored: false,
        service_command_registered: false,
        service_event_emitted: false,
        service_read_api_implemented: false,
        service_read_model_event_emitted: false,
        portal_ui_rendered: false,
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

fn build_protocol_command_handoff_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffOptions,
    row: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelRow,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRow {
    let state = protocol_command_handoff_state_for_read_model(&row.protocol_read_model_state);
    let required = state == PROTOCOL_COMMAND_HANDOFF_PROOF_REQUIRED;
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:protocol-command-handoff", row.row_id),
        source_protocol_read_model_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        protocol_command_handoff_state: state.to_string(),
        required_protocol_proof_refs: row.required_protocol_proof_refs.clone(),
        required_agent_protocol_command_refs: if required {
            options.protocol_command_refs.clone()
        } else {
            vec![]
        },
        required_agent_protocol_event_refs: if required {
            options.protocol_event_refs.clone()
        } else {
            vec![]
        },
        required_service_handler_refs: if required {
            options.service_handler_refs.clone()
        } else {
            vec![]
        },
        inherited_service_readiness_proof_refs: row.inherited_service_readiness_proof_refs.clone(),
        source_evidence_refs: row.source_evidence_refs.clone(),
        service_read_api_ref: row.service_read_api_ref.clone(),
        command_summary_ref: options.command_summary_ref.clone(),
        agent_protocol_command_implemented: false,
        agent_protocol_event_implemented: false,
        rust_protocol_mirrored: false,
        service_command_registered: false,
        service_handler_implemented: false,
        service_event_emitted: false,
        service_read_api_implemented: false,
        portal_ui_rendered: false,
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

pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffOptions,
    read_model: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoff {
    let rows = read_model
        .rows
        .iter()
        .map(|row| build_protocol_command_handoff_row(options, row))
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoff {
        schema_version: options.schema_version.clone(),
        command_handoff_id: options.command_handoff_id.clone(),
        source_protocol_read_model_id: read_model.read_model_id.clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        command_summary_ref: options.command_summary_ref.clone(),
        native_app_row_count: read_model.native_app_row_count,
        native_game_row_count: read_model.native_game_row_count,
        protocol_command_handoff_proof_required_count: count_state(
            &rows,
            |row| row.protocol_command_handoff_state.as_str(),
            PROTOCOL_COMMAND_HANDOFF_PROOF_REQUIRED,
        ),
        blocked_by_source_freshness_count: count_state(
            &rows,
            |row| row.protocol_command_handoff_state.as_str(),
            BLOCKED_BY_SOURCE_FRESHNESS,
        ),
        blocked_by_compiler_decision_count: count_state(
            &rows,
            |row| row.protocol_command_handoff_state.as_str(),
            BLOCKED_BY_COMPILER_DECISION,
        ),
        protocol_command_handoff_non_claims: PROTOCOL_COMMAND_HANDOFF_NON_CLAIMS
            .iter()
            .map(|v| (*v).to_string())
            .collect(),
        rows,
        agent_protocol_command_implemented: false,
        agent_protocol_event_implemented: false,
        rust_protocol_mirrored: false,
        service_command_registered: false,
        service_handler_implemented: false,
        service_event_emitted: false,
        service_read_api_implemented: false,
        portal_ui_rendered: false,
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

fn build_service_handler_handoff_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffOptions,
    row: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffRow,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRow {
    let state = service_handler_state_for_command_handoff(&row.protocol_command_handoff_state);
    let required = state == SERVICE_HANDLER_PROOF_REQUIRED;
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:service-handler-handoff", row.row_id),
        source_protocol_command_handoff_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        service_handler_handoff_state: state.to_string(),
        inherited_protocol_proof_refs: row.required_protocol_proof_refs.clone(),
        inherited_agent_protocol_command_refs: row.required_agent_protocol_command_refs.clone(),
        inherited_agent_protocol_event_refs: row.required_agent_protocol_event_refs.clone(),
        required_service_handler_refs: if required {
            row.required_service_handler_refs.clone()
        } else {
            vec![]
        },
        required_service_read_api_proof_refs: if required {
            options.service_read_api_proof_refs.clone()
        } else {
            vec![]
        },
        inherited_service_readiness_proof_refs: row.inherited_service_readiness_proof_refs.clone(),
        source_evidence_refs: row.source_evidence_refs.clone(),
        service_read_api_ref: row.service_read_api_ref.clone(),
        service_handler_summary_ref: options.service_handler_summary_ref.clone(),
        service_command_registered: false,
        service_handler_implemented: false,
        service_event_emitted: false,
        service_read_api_implemented: false,
        agent_protocol_implemented: false,
        rust_protocol_mirrored: false,
        portal_ui_rendered: false,
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

pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffOptions,
    command_handoff: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoff,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoff {
    let rows = command_handoff
        .rows
        .iter()
        .map(|row| build_service_handler_handoff_row(options, row))
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoff {
        schema_version: options.schema_version.clone(),
        service_handler_handoff_id: options.service_handler_handoff_id.clone(),
        source_protocol_command_handoff_id: command_handoff.command_handoff_id.clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        service_handler_summary_ref: options.service_handler_summary_ref.clone(),
        native_app_row_count: command_handoff.native_app_row_count,
        native_game_row_count: command_handoff.native_game_row_count,
        service_handler_proof_required_count: count_state(
            &rows,
            |row| row.service_handler_handoff_state.as_str(),
            SERVICE_HANDLER_PROOF_REQUIRED,
        ),
        blocked_by_source_freshness_count: count_state(
            &rows,
            |row| row.service_handler_handoff_state.as_str(),
            BLOCKED_BY_SOURCE_FRESHNESS,
        ),
        blocked_by_compiler_decision_count: count_state(
            &rows,
            |row| row.service_handler_handoff_state.as_str(),
            BLOCKED_BY_COMPILER_DECISION,
        ),
        service_handler_handoff_non_claims: SERVICE_HANDLER_NON_CLAIMS
            .iter()
            .map(|v| (*v).to_string())
            .collect(),
        rows,
        service_command_registered: false,
        service_handler_implemented: false,
        service_event_emitted: false,
        service_read_api_implemented: false,
        agent_protocol_implemented: false,
        rust_protocol_mirrored: false,
        portal_ui_rendered: false,
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

fn build_service_read_api_handoff_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffOptions,
    row: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffRow,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRow {
    let state =
        service_read_api_state_for_service_handler_handoff(&row.service_handler_handoff_state);
    let required = state == SERVICE_READ_API_PROOF_REQUIRED;
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:read-api-handoff", row.row_id),
        source_service_handler_handoff_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        service_read_api_handoff_state: state.to_string(),
        inherited_protocol_proof_refs: row.inherited_protocol_proof_refs.clone(),
        inherited_agent_protocol_command_refs: row.inherited_agent_protocol_command_refs.clone(),
        inherited_agent_protocol_event_refs: row.inherited_agent_protocol_event_refs.clone(),
        inherited_service_handler_refs: row.required_service_handler_refs.clone(),
        required_service_read_api_proof_refs: if required {
            row.required_service_read_api_proof_refs.clone()
        } else {
            vec![]
        },
        inherited_service_readiness_proof_refs: row.inherited_service_readiness_proof_refs.clone(),
        source_evidence_refs: row.source_evidence_refs.clone(),
        service_read_api_ref: row.service_read_api_ref.clone(),
        service_read_api_summary_ref: options.service_read_api_summary_ref.clone(),
        service_command_registered: false,
        service_handler_implemented: false,
        service_read_api_implemented: false,
        service_event_emitted: false,
        agent_protocol_implemented: false,
        rust_protocol_mirrored: false,
        portal_ui_rendered: false,
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

pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_service_read_api_handoff(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffOptions,
    service_handler_handoff: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoff,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoff {
    let rows = service_handler_handoff
        .rows
        .iter()
        .map(|row| build_service_read_api_handoff_row(options, row))
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoff {
        schema_version: options.schema_version.clone(),
        service_read_api_handoff_id: options.service_read_api_handoff_id.clone(),
        source_service_handler_handoff_id: service_handler_handoff
            .service_handler_handoff_id
            .clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        service_read_api_summary_ref: options.service_read_api_summary_ref.clone(),
        native_app_row_count: service_handler_handoff.native_app_row_count,
        native_game_row_count: service_handler_handoff.native_game_row_count,
        service_read_api_proof_required_count: count_state(
            &rows,
            |row| row.service_read_api_handoff_state.as_str(),
            SERVICE_READ_API_PROOF_REQUIRED,
        ),
        blocked_by_source_freshness_count: count_state(
            &rows,
            |row| row.service_read_api_handoff_state.as_str(),
            BLOCKED_BY_SOURCE_FRESHNESS,
        ),
        blocked_by_compiler_decision_count: count_state(
            &rows,
            |row| row.service_read_api_handoff_state.as_str(),
            BLOCKED_BY_COMPILER_DECISION,
        ),
        service_read_api_handoff_non_claims: SERVICE_READ_API_NON_CLAIMS
            .iter()
            .map(|v| (*v).to_string())
            .collect(),
        rows,
        service_command_registered: false,
        service_handler_implemented: false,
        service_read_api_implemented: false,
        service_event_emitted: false,
        agent_protocol_implemented: false,
        rust_protocol_mirrored: false,
        portal_ui_rendered: false,
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

fn build_read_api_response_handoff_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffOptions,
    row: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffRow,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRow {
    let state = read_api_response_state_for_read_api_handoff(&row.service_read_api_handoff_state);
    let required = state == READ_API_RESPONSE_PROOF_REQUIRED;
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:read-api-response-handoff", row.row_id),
        source_read_api_handoff_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        read_api_response_handoff_state: state.to_string(),
        inherited_protocol_proof_refs: row.inherited_protocol_proof_refs.clone(),
        inherited_agent_protocol_command_refs: row.inherited_agent_protocol_command_refs.clone(),
        inherited_agent_protocol_event_refs: row.inherited_agent_protocol_event_refs.clone(),
        inherited_service_handler_refs: row.inherited_service_handler_refs.clone(),
        inherited_service_read_api_proof_refs: row.required_service_read_api_proof_refs.clone(),
        required_read_api_response_proof_refs: if required {
            options.read_api_response_proof_refs.clone()
        } else {
            vec![]
        },
        inherited_service_readiness_proof_refs: row.inherited_service_readiness_proof_refs.clone(),
        source_evidence_refs: row.source_evidence_refs.clone(),
        service_read_api_ref: row.service_read_api_ref.clone(),
        read_api_response_summary_ref: options.read_api_response_summary_ref.clone(),
        service_command_registered: false,
        service_handler_implemented: false,
        service_read_api_implemented: false,
        service_read_api_response_implemented: false,
        service_event_emitted: false,
        agent_protocol_implemented: false,
        rust_protocol_mirrored: false,
        portal_ui_rendered: false,
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

pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_handoff(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffOptions,
    read_api_handoff: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoff,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoff {
    let rows = read_api_handoff
        .rows
        .iter()
        .map(|row| build_read_api_response_handoff_row(options, row))
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoff {
        schema_version: options.schema_version.clone(),
        read_api_response_handoff_id: options.read_api_response_handoff_id.clone(),
        source_read_api_handoff_id: read_api_handoff.service_read_api_handoff_id.clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        read_api_response_summary_ref: options.read_api_response_summary_ref.clone(),
        native_app_row_count: read_api_handoff.native_app_row_count,
        native_game_row_count: read_api_handoff.native_game_row_count,
        read_api_response_proof_required_count: count_state(
            &rows,
            |row| row.read_api_response_handoff_state.as_str(),
            READ_API_RESPONSE_PROOF_REQUIRED,
        ),
        blocked_by_source_freshness_count: count_state(
            &rows,
            |row| row.read_api_response_handoff_state.as_str(),
            BLOCKED_BY_SOURCE_FRESHNESS,
        ),
        blocked_by_compiler_decision_count: count_state(
            &rows,
            |row| row.read_api_response_handoff_state.as_str(),
            BLOCKED_BY_COMPILER_DECISION,
        ),
        read_api_response_handoff_non_claims: READ_API_RESPONSE_NON_CLAIMS
            .iter()
            .map(|v| (*v).to_string())
            .collect(),
        rows,
        service_command_registered: false,
        service_handler_implemented: false,
        service_read_api_implemented: false,
        service_read_api_response_implemented: false,
        service_event_emitted: false,
        agent_protocol_implemented: false,
        rust_protocol_mirrored: false,
        portal_ui_rendered: false,
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

pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_consumer_handoff(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffOptions,
    response_handoff: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoff,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff {
    let rows = response_handoff
        .rows
        .iter()
        .map(|row| build_read_api_response_consumer_handoff_row(options, row))
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff {
        schema_version: options.schema_version.clone(),
        read_api_response_consumer_handoff_id: options
            .read_api_response_consumer_handoff_id
            .clone(),
        source_read_api_response_handoff_id: response_handoff.read_api_response_handoff_id.clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        read_api_response_consumer_summary_ref: options
            .read_api_response_consumer_summary_ref
            .clone(),
        native_app_row_count: response_handoff.native_app_row_count,
        native_game_row_count: response_handoff.native_game_row_count,
        read_api_response_consumer_proof_required_count: count_state(
            &rows,
            |row| row.read_api_response_consumer_handoff_state.as_str(),
            READ_API_RESPONSE_CONSUMER_PROOF_REQUIRED,
        ),
        blocked_by_source_freshness_count: count_state(
            &rows,
            |row| row.read_api_response_consumer_handoff_state.as_str(),
            BLOCKED_BY_SOURCE_FRESHNESS,
        ),
        blocked_by_compiler_decision_count: count_state(
            &rows,
            |row| row.read_api_response_consumer_handoff_state.as_str(),
            BLOCKED_BY_COMPILER_DECISION,
        ),
        read_api_response_consumer_handoff_non_claims: READ_API_RESPONSE_CONSUMER_NON_CLAIMS
            .iter()
            .map(|v| (*v).to_string())
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

fn build_read_api_response_consumer_handoff_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffOptions,
    row: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffRow,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRow {
    let state =
        read_api_response_consumer_state_for_response_handoff(&row.read_api_response_handoff_state);
    let required = state == READ_API_RESPONSE_CONSUMER_PROOF_REQUIRED;
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:read-api-response-consumer-handoff", row.row_id),
        source_read_api_response_handoff_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        read_api_response_consumer_handoff_state: state.to_string(),
        inherited_protocol_proof_refs: row.inherited_protocol_proof_refs.clone(),
        inherited_agent_protocol_command_refs: row.inherited_agent_protocol_command_refs.clone(),
        inherited_agent_protocol_event_refs: row.inherited_agent_protocol_event_refs.clone(),
        inherited_service_handler_refs: row.inherited_service_handler_refs.clone(),
        inherited_service_read_api_proof_refs: row.inherited_service_read_api_proof_refs.clone(),
        inherited_read_api_response_proof_refs: row.required_read_api_response_proof_refs.clone(),
        required_read_api_response_consumer_proof_refs: if required {
            options.read_api_response_consumer_proof_refs.clone()
        } else {
            vec![]
        },
        inherited_service_readiness_proof_refs: row.inherited_service_readiness_proof_refs.clone(),
        source_evidence_refs: row.source_evidence_refs.clone(),
        service_read_api_ref: row.service_read_api_ref.clone(),
        read_api_response_consumer_summary_ref: options
            .read_api_response_consumer_summary_ref
            .clone(),
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
