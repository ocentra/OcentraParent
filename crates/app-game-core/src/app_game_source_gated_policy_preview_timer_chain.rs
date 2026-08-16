const TIMER_HANDOFF_READY_FOR_TIMER_SEQUENCING: &str = "ready-for-timer-sequencing";
const TIMER_HANDOFF_SOURCE_MANUAL_REQUIRED_BEFORE_TIMER: &str =
    "source-manual-required-before-timer";
const TIMER_HANDOFF_COMPILER_MANUAL_REQUIRED_BEFORE_TIMER: &str =
    "compiler-manual-required-before-timer";

const TIMER_RUNTIME_PROOF_REQUIRED: &str = "runtime-proof-required";
const TIMER_RUNTIME_BLOCKED_BY_SOURCE_FRESHNESS: &str = "blocked-by-source-freshness";
const TIMER_RUNTIME_BLOCKED_BY_COMPILER_DECISION: &str = "blocked-by-compiler-decision";

const TIMER_SERVICE_READ_API_PROOF_REQUIRED: &str = "service-read-api-proof-required";
const TIMER_PROTOCOL_PROOF_REQUIRED: &str = "protocol-proof-required";
const TIMER_HANDOFF_NON_CLAIMS: &[&str] = &[
    "no-service-runtime-event",
    "no-portal-ui-rendered",
    "no-policy-evaluator-runtime",
    "no-timer-runtime",
    "no-adapter-dispatch",
    "no-child-delivery",
    "no-platform-enforcement",
    "no-raw-private-source-rows",
];

const TIMER_RUNTIME_NON_CLAIMS: &[&str] = &[
    "no-service-runtime-event",
    "no-portal-ui-rendered",
    "no-policy-evaluator-runtime",
    "no-timer-runtime",
    "no-timer-scheduled",
    "no-scheduler-persistence",
    "no-audit-runtime",
    "no-rollback-runtime",
    "no-adapter-dispatch",
    "no-child-delivery",
    "no-platform-enforcement",
    "no-raw-private-source-rows",
];

const TIMER_SERVICE_READINESS_NON_CLAIMS: &[&str] = &[
    "no-service-runtime-event",
    "no-service-read-api-implemented",
    "no-portal-ui-rendered",
    "no-policy-evaluator-runtime",
    "no-timer-runtime",
    "no-timer-scheduled",
    "no-scheduler-persistence-runtime",
    "no-durable-scheduler-storage",
    "no-audit-runtime",
    "no-durable-audit-log",
    "no-rollback-runtime",
    "no-rollback-execution",
    "no-adapter-dispatch",
    "no-child-delivery",
    "no-platform-enforcement",
    "no-raw-private-source-rows",
];

const TIMER_PROTOCOL_NON_CLAIMS: &[&str] = &[
    "no-agent-protocol-contract-implemented",
    "no-rust-protocol-mirrored",
    "no-service-command-registered",
    "no-service-event-emitted",
    "no-service-read-api-implemented",
    "no-portal-ui-rendered",
    "no-policy-evaluator-runtime",
    "no-timer-runtime",
    "no-timer-scheduled",
    "no-scheduler-persistence-runtime",
    "no-durable-scheduler-storage",
    "no-audit-runtime",
    "no-durable-audit-log",
    "no-rollback-runtime",
    "no-rollback-execution",
    "no-adapter-dispatch",
    "no-child-delivery",
    "no-platform-enforcement",
    "no-raw-private-source-rows",
];

#[path = "app_game_source_gated_policy_preview_timer_chain_helpers.rs"]
mod app_game_source_gated_policy_preview_timer_chain_helpers;
#[path = "app_game_source_gated_policy_preview_timer_chain_row_builders.rs"]
mod app_game_source_gated_policy_preview_timer_chain_row_builders;
#[path = "app_game_source_gated_policy_preview_timer_chain_service_helpers.rs"]
mod app_game_source_gated_policy_preview_timer_chain_service_helpers;
use app_game_source_gated_policy_preview_timer_chain_row_builders::{
    build_app_game_source_gated_policy_preview_timer_handoff_row,
    build_app_game_source_gated_policy_preview_timer_runtime_readiness_row,
    build_app_game_source_gated_policy_preview_timer_service_readiness_handoff_row,
    build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_handoff_row,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerHandoffOptions {
    pub schema_version: String,
    pub handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewReadModelRowInput {
    pub row_id: String,
    pub target_domain: String,
    pub projection_state: String,
    pub source_evidence_refs: Vec<String>,
    pub preview_decision_ref: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewReadModelInput {
    pub read_model_id: String,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewReadModelRowInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerHandoffRow {
    pub schema_version: String,
    pub row_id: String,
    pub source_read_model_row_id: String,
    pub target_domain: String,
    pub timer_handoff_state: String,
    pub timer_runtime_required: bool,
    pub manual_proof_required: bool,
    pub source_evidence_refs: Vec<String>,
    pub preview_decision_ref: Option<String>,
    pub service_runtime_event_claimed: bool,
    pub portal_ui_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
    pub generated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerHandoff {
    pub schema_version: String,
    pub handoff_id: String,
    pub source_read_model_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewTimerHandoffRow>,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub timer_sequence_candidate_count: usize,
    pub source_manual_blocked_count: usize,
    pub compiler_manual_blocked_count: usize,
    pub timer_handoff_non_claims: Vec<String>,
    pub service_runtime_event_claimed: bool,
    pub portal_ui_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions {
    pub schema_version: String,
    pub readiness_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub timer_runtime_proof_ref: String,
    pub scheduler_persistence_proof_ref: String,
    pub audit_proof_ref: String,
    pub rollback_proof_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerStatusRowInput {
    pub row_id: String,
    pub target_domain: String,
    pub timer_status_state: String,
    pub required_proof_refs: Vec<String>,
    pub source_evidence_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerStatusInput {
    pub status_id: String,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewTimerStatusRowInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow {
    pub schema_version: String,
    pub row_id: String,
    pub source_timer_status_row_id: String,
    pub target_domain: String,
    pub runtime_readiness_state: String,
    pub timer_runtime_proof_required: bool,
    pub scheduler_persistence_proof_required: bool,
    pub audit_proof_required: bool,
    pub rollback_proof_required: bool,
    pub required_proof_refs: Vec<String>,
    pub source_evidence_refs: Vec<String>,
    pub service_runtime_event_claimed: bool,
    pub portal_ui_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub timer_scheduled: bool,
    pub scheduler_persistence_claimed: bool,
    pub audit_runtime_claimed: bool,
    pub rollback_runtime_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
    pub generated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerRuntimeReadiness {
    pub schema_version: String,
    pub readiness_id: String,
    pub source_timer_status_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessRow>,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub runtime_proof_required_count: usize,
    pub blocked_by_source_freshness_count: usize,
    pub blocked_by_compiler_decision_count: usize,
    pub runtime_readiness_non_claims: Vec<String>,
    pub service_runtime_event_claimed: bool,
    pub portal_ui_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub timer_scheduled: bool,
    pub scheduler_persistence_claimed: bool,
    pub audit_runtime_claimed: bool,
    pub rollback_runtime_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions {
    pub schema_version: String,
    pub handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub service_readiness_proof_ref: String,
    pub service_read_api_proof_ref: String,
    pub service_read_api_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowInput {
    pub row_id: String,
    pub source_audit_rollback_read_model_row_id: String,
    pub source_audit_rollback_handoff_row_id: String,
    pub source_scheduler_persistence_row_id: String,
    pub target_domain: String,
    pub parent_surface_intent_state: String,
    pub parent_surface_proof_required: bool,
    pub required_proof_refs: Vec<String>,
    pub source_evidence_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentInput {
    pub intent_id: String,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow {
    pub schema_version: String,
    pub row_id: String,
    pub source_parent_surface_intent_row_id: String,
    pub source_audit_rollback_read_model_row_id: String,
    pub source_audit_rollback_handoff_row_id: String,
    pub source_scheduler_persistence_row_id: String,
    pub target_domain: String,
    pub service_readiness_handoff_state: String,
    pub parent_surface_proof_required: bool,
    pub service_readiness_proof_required: bool,
    pub service_read_api_proof_required: bool,
    pub required_proof_refs: Vec<String>,
    pub source_evidence_refs: Vec<String>,
    pub service_read_api_ref: String,
    pub service_runtime_event_claimed: bool,
    pub service_read_api_implemented: bool,
    pub portal_ui_rendered: bool,
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
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff {
    pub schema_version: String,
    pub handoff_id: String,
    pub source_parent_surface_intent_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffRow>,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub service_read_api_proof_required_count: usize,
    pub blocked_by_source_freshness_count: usize,
    pub blocked_by_compiler_decision_count: usize,
    pub service_readiness_handoff_non_claims: Vec<String>,
    pub service_runtime_event_claimed: bool,
    pub service_read_api_implemented: bool,
    pub portal_ui_rendered: bool,
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
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions {
    pub schema_version: String,
    pub handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub protocol_command_contract_proof_ref: String,
    pub protocol_event_contract_proof_ref: String,
    pub rust_protocol_mirror_proof_ref: String,
    pub service_handler_proof_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowInput {
    pub row_id: String,
    pub target_domain: String,
    pub service_readiness_read_model_state: String,
    pub required_proof_refs: Vec<String>,
    pub source_evidence_refs: Vec<String>,
    pub service_read_api_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelInput {
    pub read_model_id: String,
    pub source_service_readiness_handoff_id: String,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowInput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRow {
    pub schema_version: String,
    pub row_id: String,
    pub source_service_readiness_read_model_row_id: String,
    pub target_domain: String,
    pub protocol_handoff_state: String,
    pub required_protocol_proof_refs: Vec<String>,
    pub inherited_service_readiness_proof_refs: Vec<String>,
    pub source_evidence_refs: Vec<String>,
    pub service_read_api_ref: String,
    pub agent_protocol_contract_implemented: bool,
    pub rust_protocol_mirrored: bool,
    pub service_command_registered: bool,
    pub service_event_emitted: bool,
    pub service_read_api_implemented: bool,
    pub portal_ui_rendered: bool,
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
pub struct AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff {
    pub schema_version: String,
    pub handoff_id: String,
    pub source_service_readiness_read_model_id: String,
    pub source_service_readiness_handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffRow>,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub protocol_proof_required_count: usize,
    pub blocked_by_source_freshness_count: usize,
    pub blocked_by_compiler_decision_count: usize,
    pub protocol_handoff_non_claims: Vec<String>,
    pub agent_protocol_contract_implemented: bool,
    pub rust_protocol_mirrored: bool,
    pub service_command_registered: bool,
    pub service_event_emitted: bool,
    pub service_read_api_implemented: bool,
    pub portal_ui_rendered: bool,
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

pub fn build_app_game_source_gated_policy_preview_timer_handoff(
    options: &AppGameSourceGatedPolicyPreviewTimerHandoffOptions,
    read_model: &AppGameSourceGatedPolicyPreviewReadModelInput,
) -> AppGameSourceGatedPolicyPreviewTimerHandoff {
    let rows = read_model
        .rows
        .iter()
        .map(|row| build_app_game_source_gated_policy_preview_timer_handoff_row(options, row))
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerHandoff {
        schema_version: options.schema_version.clone(),
        handoff_id: options.handoff_id.clone(),
        source_read_model_id: read_model.read_model_id.clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        native_app_row_count: read_model.native_app_row_count,
        native_game_row_count: read_model.native_game_row_count,
        timer_sequence_candidate_count: rows
            .iter()
            .filter(|row| row.timer_handoff_state == TIMER_HANDOFF_READY_FOR_TIMER_SEQUENCING)
            .count(),
        source_manual_blocked_count: rows
            .iter()
            .filter(|row| {
                row.timer_handoff_state == TIMER_HANDOFF_SOURCE_MANUAL_REQUIRED_BEFORE_TIMER
            })
            .count(),
        compiler_manual_blocked_count: rows
            .iter()
            .filter(|row| {
                row.timer_handoff_state == TIMER_HANDOFF_COMPILER_MANUAL_REQUIRED_BEFORE_TIMER
            })
            .count(),
        timer_handoff_non_claims: TIMER_HANDOFF_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect(),
        service_runtime_event_claimed: false,
        portal_ui_rendered: false,
        policy_evaluator_runtime_claimed: false,
        timer_runtime_claimed: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
        rows,
    }
}
pub fn build_app_game_source_gated_policy_preview_timer_runtime_readiness(
    options: &AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions,
    timer_status: &AppGameSourceGatedPolicyPreviewTimerStatusInput,
) -> AppGameSourceGatedPolicyPreviewTimerRuntimeReadiness {
    let rows = timer_status
        .rows
        .iter()
        .map(|row| {
            build_app_game_source_gated_policy_preview_timer_runtime_readiness_row(options, row)
        })
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerRuntimeReadiness {
        schema_version: options.schema_version.clone(),
        readiness_id: options.readiness_id.clone(),
        source_timer_status_id: timer_status.status_id.clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        native_app_row_count: timer_status.native_app_row_count,
        native_game_row_count: timer_status.native_game_row_count,
        runtime_proof_required_count: rows
            .iter()
            .filter(|row| row.runtime_readiness_state == TIMER_RUNTIME_PROOF_REQUIRED)
            .count(),
        blocked_by_source_freshness_count: rows
            .iter()
            .filter(|row| row.runtime_readiness_state == TIMER_RUNTIME_BLOCKED_BY_SOURCE_FRESHNESS)
            .count(),
        blocked_by_compiler_decision_count: rows
            .iter()
            .filter(|row| row.runtime_readiness_state == TIMER_RUNTIME_BLOCKED_BY_COMPILER_DECISION)
            .count(),
        runtime_readiness_non_claims: TIMER_RUNTIME_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect(),
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
        rows,
    }
}
pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_handoff(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions,
    intent: &AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentInput,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff {
    let rows = intent
        .rows
        .iter()
        .map(|row| {
            build_app_game_source_gated_policy_preview_timer_service_readiness_handoff_row(
                options, row,
            )
        })
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoff {
        schema_version: options.schema_version.clone(),
        handoff_id: options.handoff_id.clone(),
        source_parent_surface_intent_id: intent.intent_id.clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        native_app_row_count: intent.native_app_row_count,
        native_game_row_count: intent.native_game_row_count,
        service_read_api_proof_required_count: rows
            .iter()
            .filter(|row| {
                row.service_readiness_handoff_state == TIMER_SERVICE_READ_API_PROOF_REQUIRED
            })
            .count(),
        blocked_by_source_freshness_count: rows
            .iter()
            .filter(|row| {
                row.service_readiness_handoff_state == TIMER_RUNTIME_BLOCKED_BY_SOURCE_FRESHNESS
            })
            .count(),
        blocked_by_compiler_decision_count: rows
            .iter()
            .filter(|row| {
                row.service_readiness_handoff_state == TIMER_RUNTIME_BLOCKED_BY_COMPILER_DECISION
            })
            .count(),
        service_readiness_handoff_non_claims: TIMER_SERVICE_READINESS_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect(),
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
        rows,
    }
}
pub fn build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_handoff(
    options: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions,
    read_model: &AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelInput,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff {
    let rows = read_model
        .rows
        .iter()
        .map(|row| {
            build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_handoff_row(
                options, row,
            )
        })
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff {
        schema_version: options.schema_version.clone(),
        handoff_id: options.handoff_id.clone(),
        source_service_readiness_read_model_id: read_model.read_model_id.clone(),
        source_service_readiness_handoff_id: read_model.source_service_readiness_handoff_id.clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        native_app_row_count: read_model.native_app_row_count,
        native_game_row_count: read_model.native_game_row_count,
        protocol_proof_required_count: rows
            .iter()
            .filter(|row| row.protocol_handoff_state == TIMER_PROTOCOL_PROOF_REQUIRED)
            .count(),
        blocked_by_source_freshness_count: rows
            .iter()
            .filter(|row| row.protocol_handoff_state == TIMER_RUNTIME_BLOCKED_BY_SOURCE_FRESHNESS)
            .count(),
        blocked_by_compiler_decision_count: rows
            .iter()
            .filter(|row| row.protocol_handoff_state == TIMER_RUNTIME_BLOCKED_BY_COMPILER_DECISION)
            .count(),
        protocol_handoff_non_claims: TIMER_PROTOCOL_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect(),
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
        rows,
    }
}

pub fn app_game_source_gated_policy_preview_timer_chain_typescript() -> &'static str {
    include_str!("../tests/generated/app-game-source-gated-policy-preview-timer-chain.ts")
}
