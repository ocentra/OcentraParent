use super::helpers::{
    count_state, response_consumer_parent_surface_read_model_state_for_parent_surface_handoff,
    response_consumer_parent_surface_state_for_read_api_response_consumer_handoff,
    response_consumer_parent_surface_status_state_for_read_model_handoff,
};
use super::*;

pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffOptions
{
    pub schema_version: String,
    pub response_consumer_parent_surface_handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub parent_surface_proof_refs: Vec<String>,
    pub parent_surface_summary_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRow
{
    pub schema_version: String,
    pub row_id: String,
    pub source_read_api_response_consumer_handoff_row_id: String,
    pub target_domain: String,
    pub response_consumer_parent_surface_handoff_state: String,
    pub inherited_protocol_proof_refs: Vec<String>,
    pub inherited_agent_protocol_command_refs: Vec<String>,
    pub inherited_agent_protocol_event_refs: Vec<String>,
    pub inherited_service_handler_refs: Vec<String>,
    pub inherited_service_read_api_proof_refs: Vec<String>,
    pub inherited_read_api_response_proof_refs: Vec<String>,
    pub inherited_read_api_response_consumer_proof_refs: Vec<String>,
    pub required_parent_surface_proof_refs: Vec<String>,
    pub inherited_service_readiness_proof_refs: Vec<String>,
    pub source_evidence_refs: Vec<String>,
    pub service_read_api_ref: String,
    pub parent_surface_summary_ref: String,
    pub service_command_registered: bool,
    pub service_handler_implemented: bool,
    pub service_read_api_implemented: bool,
    pub service_read_api_response_implemented: bool,
    pub service_read_api_response_consumer_implemented: bool,
    pub service_event_emitted: bool,
    pub agent_protocol_implemented: bool,
    pub rust_protocol_mirrored: bool,
    pub portal_ui_rendered: bool,
    pub portal_response_consumer_rendered: bool,
    pub parent_surface_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub timer_scheduled: bool,
    pub scheduler_persistence_runtime_claimed: bool,
    pub durable_scheduler_storage_claimed: bool,
    pub audit_runtime_claimed: bool,
    pub durable_audit_log_claimed: bool,
    pub rollback_runtime_claimed: bool,
    pub rollback_execution_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
    pub generated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff
{
    pub schema_version: String,
    pub response_consumer_parent_surface_handoff_id: String,
    pub source_read_api_response_consumer_handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub parent_surface_summary_ref: String,
    pub rows: Vec<
        AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRow,
    >,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub parent_surface_proof_required_count: usize,
    pub blocked_by_source_freshness_count: usize,
    pub blocked_by_compiler_decision_count: usize,
    pub response_consumer_parent_surface_handoff_non_claims: Vec<String>,
    pub service_command_registered: bool,
    pub service_handler_implemented: bool,
    pub service_read_api_implemented: bool,
    pub service_read_api_response_implemented: bool,
    pub service_read_api_response_consumer_implemented: bool,
    pub service_event_emitted: bool,
    pub agent_protocol_implemented: bool,
    pub rust_protocol_mirrored: bool,
    pub portal_ui_rendered: bool,
    pub portal_response_consumer_rendered: bool,
    pub parent_surface_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub timer_scheduled: bool,
    pub scheduler_persistence_runtime_claimed: bool,
    pub durable_scheduler_storage_claimed: bool,
    pub audit_runtime_claimed: bool,
    pub durable_audit_log_claimed: bool,
    pub rollback_runtime_claimed: bool,
    pub rollback_execution_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffOptions
{
    pub schema_version: String,
    pub response_consumer_parent_surface_read_model_handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub parent_surface_read_model_proof_refs: Vec<String>,
    pub parent_surface_read_model_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRow
{
    pub schema_version: String,
    pub row_id: String,
    pub source_response_consumer_parent_surface_handoff_row_id: String,
    pub target_domain: String,
    pub response_consumer_parent_surface_read_model_handoff_state: String,
    pub inherited_protocol_proof_refs: Vec<String>,
    pub inherited_agent_protocol_command_refs: Vec<String>,
    pub inherited_agent_protocol_event_refs: Vec<String>,
    pub inherited_service_handler_refs: Vec<String>,
    pub inherited_service_read_api_proof_refs: Vec<String>,
    pub inherited_read_api_response_proof_refs: Vec<String>,
    pub inherited_read_api_response_consumer_proof_refs: Vec<String>,
    pub inherited_parent_surface_proof_refs: Vec<String>,
    pub required_parent_surface_read_model_proof_refs: Vec<String>,
    pub inherited_service_readiness_proof_refs: Vec<String>,
    pub source_evidence_refs: Vec<String>,
    pub service_read_api_ref: String,
    pub parent_surface_read_model_ref: String,
    pub service_command_registered: bool,
    pub service_handler_implemented: bool,
    pub service_read_api_implemented: bool,
    pub service_read_api_response_implemented: bool,
    pub service_read_api_response_consumer_implemented: bool,
    pub parent_surface_read_model_implemented: bool,
    pub service_event_emitted: bool,
    pub agent_protocol_implemented: bool,
    pub rust_protocol_mirrored: bool,
    pub portal_ui_rendered: bool,
    pub portal_response_consumer_rendered: bool,
    pub parent_surface_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub timer_scheduled: bool,
    pub scheduler_persistence_runtime_claimed: bool,
    pub durable_scheduler_storage_claimed: bool,
    pub audit_runtime_claimed: bool,
    pub durable_audit_log_claimed: bool,
    pub rollback_runtime_claimed: bool,
    pub rollback_execution_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
    pub generated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff
{
    pub schema_version: String,
    pub response_consumer_parent_surface_read_model_handoff_id: String,
    pub source_response_consumer_parent_surface_handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub parent_surface_read_model_ref: String,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRow>,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub parent_surface_read_model_proof_required_count: usize,
    pub blocked_by_source_freshness_count: usize,
    pub blocked_by_compiler_decision_count: usize,
    pub response_consumer_parent_surface_read_model_handoff_non_claims: Vec<String>,
    pub service_command_registered: bool,
    pub service_handler_implemented: bool,
    pub service_read_api_implemented: bool,
    pub service_read_api_response_implemented: bool,
    pub service_read_api_response_consumer_implemented: bool,
    pub parent_surface_read_model_implemented: bool,
    pub service_event_emitted: bool,
    pub agent_protocol_implemented: bool,
    pub rust_protocol_mirrored: bool,
    pub portal_ui_rendered: bool,
    pub portal_response_consumer_rendered: bool,
    pub parent_surface_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub timer_scheduled: bool,
    pub scheduler_persistence_runtime_claimed: bool,
    pub durable_scheduler_storage_claimed: bool,
    pub audit_runtime_claimed: bool,
    pub durable_audit_log_claimed: bool,
    pub rollback_runtime_claimed: bool,
    pub rollback_execution_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffOptions
{
    pub schema_version: String,
    pub response_consumer_parent_surface_status_handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub parent_surface_status_proof_refs: Vec<String>,
    pub parent_surface_status_ref: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRow
{
    pub schema_version: String,
    pub row_id: String,
    pub source_response_consumer_parent_surface_read_model_handoff_row_id: String,
    pub target_domain: String,
    pub response_consumer_parent_surface_status_handoff_state: String,
    pub inherited_protocol_proof_refs: Vec<String>,
    pub inherited_agent_protocol_command_refs: Vec<String>,
    pub inherited_agent_protocol_event_refs: Vec<String>,
    pub inherited_service_handler_refs: Vec<String>,
    pub inherited_service_read_api_proof_refs: Vec<String>,
    pub inherited_read_api_response_proof_refs: Vec<String>,
    pub inherited_read_api_response_consumer_proof_refs: Vec<String>,
    pub inherited_parent_surface_proof_refs: Vec<String>,
    pub inherited_parent_surface_read_model_proof_refs: Vec<String>,
    pub required_parent_surface_status_proof_refs: Vec<String>,
    pub inherited_service_readiness_proof_refs: Vec<String>,
    pub source_evidence_refs: Vec<String>,
    pub service_read_api_ref: String,
    pub parent_surface_read_model_ref: String,
    pub parent_surface_status_ref: String,
    pub service_command_registered: bool,
    pub service_handler_implemented: bool,
    pub service_read_api_implemented: bool,
    pub service_read_api_response_implemented: bool,
    pub service_read_api_response_consumer_implemented: bool,
    pub parent_surface_read_model_implemented: bool,
    pub parent_surface_status_implemented: bool,
    pub service_event_emitted: bool,
    pub agent_protocol_implemented: bool,
    pub rust_protocol_mirrored: bool,
    pub portal_ui_rendered: bool,
    pub portal_response_consumer_rendered: bool,
    pub parent_surface_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub timer_scheduled: bool,
    pub scheduler_persistence_runtime_claimed: bool,
    pub durable_scheduler_storage_claimed: bool,
    pub audit_runtime_claimed: bool,
    pub durable_audit_log_claimed: bool,
    pub rollback_runtime_claimed: bool,
    pub rollback_execution_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
    pub generated_at: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoff
{
    pub schema_version: String,
    pub response_consumer_parent_surface_status_handoff_id: String,
    pub source_response_consumer_parent_surface_read_model_handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub parent_surface_status_ref: String,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRow>,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub parent_surface_status_proof_required_count: usize,
    pub blocked_by_source_freshness_count: usize,
    pub blocked_by_compiler_decision_count: usize,
    pub response_consumer_parent_surface_status_handoff_non_claims: Vec<String>,
    pub service_command_registered: bool,
    pub service_handler_implemented: bool,
    pub service_read_api_implemented: bool,
    pub service_read_api_response_implemented: bool,
    pub service_read_api_response_consumer_implemented: bool,
    pub parent_surface_read_model_implemented: bool,
    pub parent_surface_status_implemented: bool,
    pub service_event_emitted: bool,
    pub agent_protocol_implemented: bool,
    pub rust_protocol_mirrored: bool,
    pub portal_ui_rendered: bool,
    pub portal_response_consumer_rendered: bool,
    pub parent_surface_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub timer_scheduled: bool,
    pub scheduler_persistence_runtime_claimed: bool,
    pub durable_scheduler_storage_claimed: bool,
    pub audit_runtime_claimed: bool,
    pub durable_audit_log_claimed: bool,
    pub rollback_runtime_claimed: bool,
    pub rollback_execution_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffOptions
{
    pub schema_version: String,
    pub response_consumer_parent_surface_status_read_model_handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub parent_surface_status_read_model_proof_refs: Vec<String>,
    pub parent_surface_status_read_model_ref: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffRow
{
    pub schema_version: String,
    pub row_id: String,
    pub source_response_consumer_parent_surface_status_handoff_row_id: String,
    pub target_domain: String,
    pub response_consumer_parent_surface_status_read_model_handoff_state: String,
    pub inherited_protocol_proof_refs: Vec<String>,
    pub inherited_agent_protocol_command_refs: Vec<String>,
    pub inherited_agent_protocol_event_refs: Vec<String>,
    pub inherited_service_handler_refs: Vec<String>,
    pub inherited_service_read_api_proof_refs: Vec<String>,
    pub inherited_read_api_response_proof_refs: Vec<String>,
    pub inherited_read_api_response_consumer_proof_refs: Vec<String>,
    pub inherited_parent_surface_proof_refs: Vec<String>,
    pub inherited_parent_surface_read_model_proof_refs: Vec<String>,
    pub inherited_parent_surface_status_proof_refs: Vec<String>,
    pub required_parent_surface_status_read_model_proof_refs: Vec<String>,
    pub inherited_service_readiness_proof_refs: Vec<String>,
    pub source_evidence_refs: Vec<String>,
    pub service_read_api_ref: String,
    pub parent_surface_read_model_ref: String,
    pub parent_surface_status_ref: String,
    pub parent_surface_status_read_model_ref: String,
    pub service_command_registered: bool,
    pub service_handler_implemented: bool,
    pub service_read_api_implemented: bool,
    pub service_read_api_response_implemented: bool,
    pub service_read_api_response_consumer_implemented: bool,
    pub parent_surface_read_model_implemented: bool,
    pub parent_surface_status_implemented: bool,
    pub parent_surface_status_read_model_implemented: bool,
    pub service_event_emitted: bool,
    pub agent_protocol_implemented: bool,
    pub rust_protocol_mirrored: bool,
    pub portal_ui_rendered: bool,
    pub portal_response_consumer_rendered: bool,
    pub parent_surface_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub timer_scheduled: bool,
    pub scheduler_persistence_runtime_claimed: bool,
    pub durable_scheduler_storage_claimed: bool,
    pub audit_runtime_claimed: bool,
    pub durable_audit_log_claimed: bool,
    pub rollback_runtime_claimed: bool,
    pub rollback_execution_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
    pub generated_at: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff
{
    pub schema_version: String,
    pub response_consumer_parent_surface_status_read_model_handoff_id: String,
    pub source_response_consumer_parent_surface_status_handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub parent_surface_status_read_model_ref: String,
    pub rows:
        Vec<AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffRow>,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub parent_surface_status_read_model_proof_required_count: usize,
    pub blocked_by_source_freshness_count: usize,
    pub blocked_by_compiler_decision_count: usize,
    pub response_consumer_parent_surface_status_read_model_handoff_non_claims: Vec<String>,
    pub service_command_registered: bool,
    pub service_handler_implemented: bool,
    pub service_read_api_implemented: bool,
    pub service_read_api_response_implemented: bool,
    pub service_read_api_response_consumer_implemented: bool,
    pub parent_surface_read_model_implemented: bool,
    pub parent_surface_status_implemented: bool,
    pub parent_surface_status_read_model_implemented: bool,
    pub service_event_emitted: bool,
    pub agent_protocol_implemented: bool,
    pub rust_protocol_mirrored: bool,
    pub portal_ui_rendered: bool,
    pub portal_response_consumer_rendered: bool,
    pub parent_surface_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub timer_scheduled: bool,
    pub scheduler_persistence_runtime_claimed: bool,
    pub durable_scheduler_storage_claimed: bool,
    pub audit_runtime_claimed: bool,
    pub durable_audit_log_claimed: bool,
    pub rollback_runtime_claimed: bool,
    pub rollback_execution_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
}

pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_handoff(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffOptions,
    response_consumer_handoff: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff {
    let rows = response_consumer_handoff
        .rows
        .iter()
        .map(|row| build_response_consumer_parent_surface_handoff_row(options, row))
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff {
        schema_version: options.schema_version.clone(),
        response_consumer_parent_surface_handoff_id: options
            .response_consumer_parent_surface_handoff_id
            .clone(),
        source_read_api_response_consumer_handoff_id: response_consumer_handoff
            .read_api_response_consumer_handoff_id
            .clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        parent_surface_summary_ref: options.parent_surface_summary_ref.clone(),
        native_app_row_count: response_consumer_handoff.native_app_row_count,
        native_game_row_count: response_consumer_handoff.native_game_row_count,
        parent_surface_proof_required_count: count_state(
            &rows,
            |row| row.response_consumer_parent_surface_handoff_state.as_str(),
            PARENT_SURFACE_PROOF_REQUIRED,
        ),
        blocked_by_source_freshness_count: count_state(
            &rows,
            |row| row.response_consumer_parent_surface_handoff_state.as_str(),
            BLOCKED_BY_SOURCE_FRESHNESS,
        ),
        blocked_by_compiler_decision_count: count_state(
            &rows,
            |row| row.response_consumer_parent_surface_handoff_state.as_str(),
            BLOCKED_BY_COMPILER_DECISION,
        ),
        response_consumer_parent_surface_handoff_non_claims:
            RESPONSE_CONSUMER_PARENT_SURFACE_NON_CLAIMS
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

fn build_response_consumer_parent_surface_read_model_handoff_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffOptions,
    row: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRow,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRow
{
    let state = response_consumer_parent_surface_read_model_state_for_parent_surface_handoff(
        &row.response_consumer_parent_surface_handoff_state,
    );
    let required = state == PARENT_SURFACE_READ_MODEL_PROOF_REQUIRED;
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:parent-surface-read-model-handoff", row.row_id),
        source_response_consumer_parent_surface_handoff_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        response_consumer_parent_surface_read_model_handoff_state: state.to_string(),
        inherited_protocol_proof_refs: row.inherited_protocol_proof_refs.clone(),
        inherited_agent_protocol_command_refs: row
            .inherited_agent_protocol_command_refs
            .clone(),
        inherited_agent_protocol_event_refs: row.inherited_agent_protocol_event_refs.clone(),
        inherited_service_handler_refs: row.inherited_service_handler_refs.clone(),
        inherited_service_read_api_proof_refs: row
            .inherited_service_read_api_proof_refs
            .clone(),
        inherited_read_api_response_proof_refs: row
            .inherited_read_api_response_proof_refs
            .clone(),
        inherited_read_api_response_consumer_proof_refs: row
            .inherited_read_api_response_consumer_proof_refs
            .clone(),
        inherited_parent_surface_proof_refs: row.required_parent_surface_proof_refs.clone(),
        required_parent_surface_read_model_proof_refs: if required {
            options.parent_surface_read_model_proof_refs.clone()
        } else {
            vec![]
        },
        inherited_service_readiness_proof_refs: row
            .inherited_service_readiness_proof_refs
            .clone(),
        source_evidence_refs: row.source_evidence_refs.clone(),
        service_read_api_ref: row.service_read_api_ref.clone(),
        parent_surface_read_model_ref: options.parent_surface_read_model_ref.clone(),
        service_command_registered: false,
        service_handler_implemented: false,
        service_read_api_implemented: false,
        service_read_api_response_implemented: false,
        service_read_api_response_consumer_implemented: false,
        parent_surface_read_model_implemented: false,
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

fn build_response_consumer_parent_surface_status_handoff_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffOptions,
    row: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffRow,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRow
{
    let state = response_consumer_parent_surface_status_state_for_read_model_handoff(
        &row.response_consumer_parent_surface_read_model_handoff_state,
    );
    let required = state == PARENT_SURFACE_STATUS_PROOF_REQUIRED;
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:parent-surface-status-handoff", row.row_id),
        source_response_consumer_parent_surface_read_model_handoff_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        response_consumer_parent_surface_status_handoff_state: state.to_string(),
        inherited_protocol_proof_refs: row.inherited_protocol_proof_refs.clone(),
        inherited_agent_protocol_command_refs: row
            .inherited_agent_protocol_command_refs
            .clone(),
        inherited_agent_protocol_event_refs: row.inherited_agent_protocol_event_refs.clone(),
        inherited_service_handler_refs: row.inherited_service_handler_refs.clone(),
        inherited_service_read_api_proof_refs: row
            .inherited_service_read_api_proof_refs
            .clone(),
        inherited_read_api_response_proof_refs: row
            .inherited_read_api_response_proof_refs
            .clone(),
        inherited_read_api_response_consumer_proof_refs: row
            .inherited_read_api_response_consumer_proof_refs
            .clone(),
        inherited_parent_surface_proof_refs: row.inherited_parent_surface_proof_refs.clone(),
        inherited_parent_surface_read_model_proof_refs: row
            .required_parent_surface_read_model_proof_refs
            .clone(),
        required_parent_surface_status_proof_refs: if required {
            options.parent_surface_status_proof_refs.clone()
        } else {
            vec![]
        },
        inherited_service_readiness_proof_refs: row
            .inherited_service_readiness_proof_refs
            .clone(),
        source_evidence_refs: row.source_evidence_refs.clone(),
        service_read_api_ref: row.service_read_api_ref.clone(),
        parent_surface_read_model_ref: row.parent_surface_read_model_ref.clone(),
        parent_surface_status_ref: options.parent_surface_status_ref.clone(),
        generated_at: options.generated_at.clone(),
        ..Default::default()
    }
}

fn build_response_consumer_parent_surface_handoff_row(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffOptions,
    row: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffRow,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRow {
    let state = response_consumer_parent_surface_state_for_read_api_response_consumer_handoff(
        &row.read_api_response_consumer_handoff_state,
    );
    let required = state == PARENT_SURFACE_PROOF_REQUIRED;
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffRow {
        schema_version: options.schema_version.clone(),
        row_id: format!("{}:response-consumer-parent-surface-handoff", row.row_id),
        source_read_api_response_consumer_handoff_row_id: row.row_id.clone(),
        target_domain: row.target_domain.clone(),
        response_consumer_parent_surface_handoff_state: state.to_string(),
        inherited_protocol_proof_refs: row.inherited_protocol_proof_refs.clone(),
        inherited_agent_protocol_command_refs: row.inherited_agent_protocol_command_refs.clone(),
        inherited_agent_protocol_event_refs: row.inherited_agent_protocol_event_refs.clone(),
        inherited_service_handler_refs: row.inherited_service_handler_refs.clone(),
        inherited_service_read_api_proof_refs: row.inherited_service_read_api_proof_refs.clone(),
        inherited_read_api_response_proof_refs: row.inherited_read_api_response_proof_refs.clone(),
        inherited_read_api_response_consumer_proof_refs: row
            .required_read_api_response_consumer_proof_refs
            .clone(),
        required_parent_surface_proof_refs: if required {
            options.parent_surface_proof_refs.clone()
        } else {
            vec![]
        },
        inherited_service_readiness_proof_refs: row.inherited_service_readiness_proof_refs.clone(),
        source_evidence_refs: row.source_evidence_refs.clone(),
        service_read_api_ref: row.service_read_api_ref.clone(),
        parent_surface_summary_ref: options.parent_surface_summary_ref.clone(),
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

pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_read_model_handoff(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffOptions,
    parent_surface_handoff: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoff,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff
{
    let rows = parent_surface_handoff
        .rows
        .iter()
        .map(|row| build_response_consumer_parent_surface_read_model_handoff_row(options, row))
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff {
        schema_version: options.schema_version.clone(),
        response_consumer_parent_surface_read_model_handoff_id: options
            .response_consumer_parent_surface_read_model_handoff_id
            .clone(),
        source_response_consumer_parent_surface_handoff_id: parent_surface_handoff
            .response_consumer_parent_surface_handoff_id
            .clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        parent_surface_read_model_ref: options.parent_surface_read_model_ref.clone(),
        native_app_row_count: parent_surface_handoff.native_app_row_count,
        native_game_row_count: parent_surface_handoff.native_game_row_count,
        parent_surface_read_model_proof_required_count: count_state(
            &rows,
            |row| row
                .response_consumer_parent_surface_read_model_handoff_state
                .as_str(),
            PARENT_SURFACE_READ_MODEL_PROOF_REQUIRED,
        ),
        blocked_by_source_freshness_count: count_state(
            &rows,
            |row| row
                .response_consumer_parent_surface_read_model_handoff_state
                .as_str(),
            BLOCKED_BY_SOURCE_FRESHNESS,
        ),
        blocked_by_compiler_decision_count: count_state(
            &rows,
            |row| row
                .response_consumer_parent_surface_read_model_handoff_state
                .as_str(),
            BLOCKED_BY_COMPILER_DECISION,
        ),
        response_consumer_parent_surface_read_model_handoff_non_claims:
            RESPONSE_CONSUMER_PARENT_SURFACE_READ_MODEL_NON_CLAIMS
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

pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_handoff(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffOptions,
    parent_surface_read_model_handoff: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoff,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoff
{
    let rows = parent_surface_read_model_handoff
        .rows
        .iter()
        .map(|row| build_response_consumer_parent_surface_status_handoff_row(options, row))
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoff {
        schema_version: options.schema_version.clone(),
        response_consumer_parent_surface_status_handoff_id: options
            .response_consumer_parent_surface_status_handoff_id
            .clone(),
        source_response_consumer_parent_surface_read_model_handoff_id:
            parent_surface_read_model_handoff
                .response_consumer_parent_surface_read_model_handoff_id
                .clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        parent_surface_status_ref: options.parent_surface_status_ref.clone(),
        native_app_row_count: parent_surface_read_model_handoff.native_app_row_count,
        native_game_row_count: parent_surface_read_model_handoff.native_game_row_count,
        parent_surface_status_proof_required_count: count_state(
            &rows,
            |row| {
                row.response_consumer_parent_surface_status_handoff_state
                    .as_str()
            },
            PARENT_SURFACE_STATUS_PROOF_REQUIRED,
        ),
        blocked_by_source_freshness_count: count_state(
            &rows,
            |row| {
                row.response_consumer_parent_surface_status_handoff_state
                    .as_str()
            },
            BLOCKED_BY_SOURCE_FRESHNESS,
        ),
        blocked_by_compiler_decision_count: count_state(
            &rows,
            |row| {
                row.response_consumer_parent_surface_status_handoff_state
                    .as_str()
            },
            BLOCKED_BY_COMPILER_DECISION,
        ),
        response_consumer_parent_surface_status_handoff_non_claims:
            RESPONSE_CONSUMER_PARENT_SURFACE_STATUS_NON_CLAIMS
                .iter()
                .map(|value| (*value).to_string())
                .collect(),
        rows,
        ..Default::default()
    }
}
