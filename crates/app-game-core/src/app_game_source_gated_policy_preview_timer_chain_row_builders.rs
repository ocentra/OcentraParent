use super::app_game_source_gated_policy_preview_timer_chain_helpers::{
    runtime_readiness_state_for_timer_status, timer_handoff_state_for_projection,
};
use super::app_game_source_gated_policy_preview_timer_chain_service_helpers::{
    protocol_handoff_state_for_read_model, service_readiness_handoff_state_for_parent_surface,
};
use super::{
    AppGameSourceGatedPolicyPreviewReadModelRowInput,
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowInput,
    AppGameSourceGatedPolicyPreviewTimerHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerHandoffRow,
    AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions,
    AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRow,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowInput,
    AppGameSourceGatedPolicyPreviewTimerStatusRowInput, TIMER_HANDOFF_READY_FOR_TIMER_SEQUENCING,
    TIMER_RUNTIME_PROOF_REQUIRED, TIMER_SERVICE_READ_API_PROOF_REQUIRED,
};

const TIMER_STATUS_RUNTIME_PROOF_REQUIRED: &str = "timer-runtime-proof-required";
const TIMER_PROTOCOL_PROOF_REQUIRED: &str = "protocol-proof-required";

pub(super) fn build_app_game_source_gated_policy_preview_timer_handoff_row(
    options: &AppGameSourceGatedPolicyPreviewTimerHandoffOptions,
    row: &AppGameSourceGatedPolicyPreviewReadModelRowInput,
) -> AppGameSourceGatedPolicyPreviewTimerHandoffRow {
    let timer_handoff_state = timer_handoff_state_for_projection(&row.projection_state);
    AppGameSourceGatedPolicyPreviewTimerHandoffRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:timer-handoff", row.row_id),
        source_read_model_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        timer_handoff_state: timer_handoff_state.to_string(),
        timer_runtime_required: timer_handoff_state == TIMER_HANDOFF_READY_FOR_TIMER_SEQUENCING,
        manual_proof_required: timer_handoff_state != TIMER_HANDOFF_READY_FOR_TIMER_SEQUENCING,
        source_evidence_refs: row.source_evidence_refs.clone(),
        preview_decision_ref: row.preview_decision_ref.clone(),
        service_runtime_event_claimed: false,
        portal_ui_rendered: false,
        policy_evaluator_runtime_claimed: false,
        timer_runtime_claimed: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
        generated_at: options.generated_at.clone(),
    }
}

pub(super) fn build_app_game_source_gated_policy_preview_timer_runtime_readiness_row(
    options: &AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions,
    row: &AppGameSourceGatedPolicyPreviewTimerStatusRowInput,
) -> AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow {
    let runtime_readiness_state = runtime_readiness_state_for_timer_status(&row.timer_status_state);
    let required_proof_refs = if runtime_readiness_state == TIMER_RUNTIME_PROOF_REQUIRED {
        vec![
            options.timer_runtime_proof_ref.clone(),
            options.scheduler_persistence_proof_ref.clone(),
            options.audit_proof_ref.clone(),
            options.rollback_proof_ref.clone(),
        ]
    } else {
        row.required_proof_refs.clone()
    };

    AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:runtime-readiness", row.row_id),
        source_timer_status_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        runtime_readiness_state: runtime_readiness_state.to_string(),
        timer_runtime_proof_required: row.timer_status_state == TIMER_STATUS_RUNTIME_PROOF_REQUIRED,
        scheduler_persistence_proof_required: row.timer_status_state
            == TIMER_STATUS_RUNTIME_PROOF_REQUIRED,
        audit_proof_required: row.timer_status_state == TIMER_STATUS_RUNTIME_PROOF_REQUIRED,
        rollback_proof_required: row.timer_status_state == TIMER_STATUS_RUNTIME_PROOF_REQUIRED,
        required_proof_refs,
        source_evidence_refs: row.source_evidence_refs.clone(),
        service_runtime_event_claimed: false,
        portal_ui_rendered: false,
        policy_evaluator_runtime_claimed: false,
        timer_runtime_claimed: false,
        timer_scheduled: false,
        scheduler_persistence_claimed: false,
        audit_runtime_claimed: false,
        rollback_runtime_claimed: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
        generated_at: options.generated_at.clone(),
    }
}

pub(super) fn build_app_game_source_gated_policy_preview_timer_service_readiness_handoff_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions,
    row: &AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowInput,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow {
    let service_readiness_handoff_state =
        service_readiness_handoff_state_for_parent_surface(&row.parent_surface_intent_state);
    let mut required_proof_refs = row.required_proof_refs.clone();
    if service_readiness_handoff_state == TIMER_SERVICE_READ_API_PROOF_REQUIRED {
        required_proof_refs.push(options.service_readiness_proof_ref.clone());
        required_proof_refs.push(options.service_read_api_proof_ref.clone());
    }

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:service-readiness-handoff", row.row_id),
        source_parent_surface_intent_row_id: row.row_id.clone(),
        source_audit_rollback_read_model_row_id: row
            .source_audit_rollback_read_model_row_id
            .clone(),
        source_audit_rollback_handoff_row_id: row.source_audit_rollback_handoff_row_id.clone(),
        source_scheduler_persistence_row_id: row.source_scheduler_persistence_row_id.clone(),
        target_domain: row.target_domain.clone(),
        service_readiness_handoff_state: service_readiness_handoff_state.to_string(),
        parent_surface_proof_required: row.parent_surface_proof_required,
        service_readiness_proof_required: service_readiness_handoff_state
            == TIMER_SERVICE_READ_API_PROOF_REQUIRED,
        service_read_api_proof_required: service_readiness_handoff_state
            == TIMER_SERVICE_READ_API_PROOF_REQUIRED,
        required_proof_refs,
        source_evidence_refs: row.source_evidence_refs.clone(),
        service_read_api_ref: options.service_read_api_ref.clone(),
        service_runtime_event_claimed: false,
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

pub(super) fn build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_handoff_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions,
    row: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowInput,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRow {
    let protocol_handoff_state =
        protocol_handoff_state_for_read_model(&row.service_readiness_read_model_state);
    let inherited_service_readiness_proof_refs = row.required_proof_refs.clone();
    let required_protocol_proof_refs = if protocol_handoff_state == TIMER_PROTOCOL_PROOF_REQUIRED {
        vec![
            options.protocol_command_contract_proof_ref.clone(),
            options.protocol_event_contract_proof_ref.clone(),
            options.rust_protocol_mirror_proof_ref.clone(),
            options.service_handler_proof_ref.clone(),
        ]
    } else {
        inherited_service_readiness_proof_refs.clone()
    };

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:protocol-handoff", row.row_id),
        source_service_readiness_read_model_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        protocol_handoff_state: protocol_handoff_state.to_string(),
        required_protocol_proof_refs,
        inherited_service_readiness_proof_refs,
        source_evidence_refs: row.source_evidence_refs.clone(),
        service_read_api_ref: row.service_read_api_ref.clone(),
        agent_protocol_contract_implemented: false,
        rust_protocol_mirrored: false,
        service_command_registered: false,
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
