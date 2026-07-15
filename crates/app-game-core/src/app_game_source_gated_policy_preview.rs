const SENSITIVE_BOUNDARY_REDACTED_EVIDENCE_REFS_ONLY: &str = "redacted-evidence-refs-only";

const READ_MODEL_PREVIEW_READY_VISIBLE: &str = "preview-ready-visible";
const READ_MODEL_SOURCE_MANUAL_REQUIRED_VISIBLE: &str = "source-manual-required-visible";
const READ_MODEL_COMPILER_MANUAL_REQUIRED_VISIBLE: &str = "compiler-manual-required-visible";

const TIMER_STATUS_TIMER_RUNTIME_PROOF_REQUIRED: &str = "timer-runtime-proof-required";
const TIMER_STATUS_SOURCE_FRESHNESS_PROOF_REQUIRED: &str = "source-freshness-proof-required";
const TIMER_STATUS_COMPILER_DECISION_PROOF_REQUIRED: &str = "compiler-decision-proof-required";

#[path = "app_game_source_gated_policy_preview_helpers.rs"]
mod app_game_source_gated_policy_preview_helpers;
use app_game_source_gated_policy_preview_helpers::{
    count_projection_rows, count_rows, projection_state_for_gate_state,
    required_proof_refs_for_timer_status, timer_status_state_for_handoff_state,
};

const READ_MODEL_REQUIRED_NON_CLAIMS: &[&str] = &[
    "no-service-runtime-event",
    "no-portal-ui-rendered",
    "no-policy-evaluator-runtime",
    "no-timer-runtime",
    "no-adapter-dispatch",
    "no-child-delivery",
    "no-platform-enforcement",
    "no-raw-private-source-rows",
];

const TIMER_STATUS_REQUIRED_NON_CLAIMS: &[&str] = &[
    "no-service-runtime-event",
    "no-portal-ui-rendered",
    "no-policy-evaluator-runtime",
    "no-timer-runtime",
    "no-timer-scheduled",
    "no-adapter-dispatch",
    "no-child-delivery",
    "no-platform-enforcement",
    "no-raw-private-source-rows",
];

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewReadModelOptions {
    pub schema_version: String,
    pub read_model_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceFreshnessPreviewGatePreviewRow {
    pub policy_decision_id: String,
    pub preview_status: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceFreshnessPreviewGateRow {
    pub row_id: String,
    pub target_domain: String,
    pub source_readiness_id: String,
    pub source_policy_request_id: String,
    pub source_readiness_state: String,
    pub source_requirement_states: Vec<String>,
    pub source_policy_compile_allowed: bool,
    pub source_evidence_refs: Vec<String>,
    pub gate_state: String,
    pub preview_status: String,
    pub preview_row: Option<AppGameSourceFreshnessPreviewGatePreviewRow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceFreshnessPreviewGateReadModel {
    pub gate_id: String,
    pub source_contract_refs: Vec<String>,
    pub rows: Vec<AppGameSourceFreshnessPreviewGateRow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewReadModelRow {
    pub schema_version: String,
    pub row_id: String,
    pub source_gate_row_id: String,
    pub source_gate_id: String,
    pub target_domain: String,
    pub source_readiness_id: String,
    pub source_policy_request_id: String,
    pub source_readiness_state: String,
    pub source_requirement_states: Vec<String>,
    pub source_policy_compile_allowed: bool,
    pub source_evidence_refs: Vec<String>,
    pub gate_state: String,
    pub projection_state: String,
    pub preview_status: String,
    pub preview_decision_ref: Option<String>,
    pub preview_compiler_status: Option<String>,
    pub sensitive_detail_boundary: String,
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
pub struct AppGameSourceGatedPolicyPreviewReadModelReadModel {
    pub schema_version: String,
    pub read_model_id: String,
    pub source_gate_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub source_gate_contract_refs: Vec<String>,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewReadModelRow>,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub preview_ready_visible_count: usize,
    pub source_manual_required_visible_count: usize,
    pub compiler_manual_required_visible_count: usize,
    pub read_model_non_claims: Vec<String>,
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
pub struct AppGameSourceGatedPolicyPreviewTimerStatusOptions {
    pub schema_version: String,
    pub status_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub timer_runtime_proof_ref: String,
    pub source_freshness_proof_ref: String,
    pub compiler_decision_proof_ref: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerHandoffRow {
    pub row_id: String,
    pub target_domain: String,
    pub timer_handoff_state: String,
    pub source_evidence_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerHandoffReadModel {
    pub handoff_id: String,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewTimerHandoffRow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerStatusRow {
    pub schema_version: String,
    pub row_id: String,
    pub source_timer_handoff_row_id: String,
    pub target_domain: String,
    pub timer_status_state: String,
    pub timer_runtime_proof_required: bool,
    pub source_freshness_proof_required: bool,
    pub compiler_decision_proof_required: bool,
    pub required_proof_refs: Vec<String>,
    pub source_evidence_refs: Vec<String>,
    pub service_runtime_event_claimed: bool,
    pub portal_ui_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub timer_scheduled: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
    pub generated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceGatedPolicyPreviewTimerStatus {
    pub schema_version: String,
    pub status_id: String,
    pub source_timer_handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub rows: Vec<AppGameSourceGatedPolicyPreviewTimerStatusRow>,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub timer_runtime_proof_required_count: usize,
    pub source_freshness_proof_required_count: usize,
    pub compiler_decision_proof_required_count: usize,
    pub timer_status_non_claims: Vec<String>,
    pub service_runtime_event_claimed: bool,
    pub portal_ui_rendered: bool,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub timer_scheduled: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub raw_private_source_rows_included: bool,
}

pub fn build_app_game_source_gated_policy_preview_read_model(
    options: &AppGameSourceGatedPolicyPreviewReadModelOptions,
    source_gate_read_model: &AppGameSourceFreshnessPreviewGateReadModel,
) -> AppGameSourceGatedPolicyPreviewReadModelReadModel {
    let rows = source_gate_read_model
        .rows
        .iter()
        .map(|row| AppGameSourceGatedPolicyPreviewReadModelRow {
            schema_version: options.schema_version.clone(),
            row_id: format!("{}:source-gated-preview-read-model", row.row_id),
            source_gate_row_id: row.row_id.clone(),
            source_gate_id: source_gate_read_model.gate_id.clone(),
            target_domain: row.target_domain.clone(),
            source_readiness_id: row.source_readiness_id.clone(),
            source_policy_request_id: row.source_policy_request_id.clone(),
            source_readiness_state: row.source_readiness_state.clone(),
            source_requirement_states: row.source_requirement_states.clone(),
            source_policy_compile_allowed: row.source_policy_compile_allowed,
            source_evidence_refs: row.source_evidence_refs.clone(),
            gate_state: row.gate_state.clone(),
            projection_state: projection_state_for_gate_state(&row.gate_state).to_string(),
            preview_status: row.preview_status.clone(),
            preview_decision_ref: row
                .preview_row
                .as_ref()
                .map(|preview_row| preview_row.policy_decision_id.clone()),
            preview_compiler_status: row
                .preview_row
                .as_ref()
                .map(|preview_row| preview_row.preview_status.clone()),
            sensitive_detail_boundary: SENSITIVE_BOUNDARY_REDACTED_EVIDENCE_REFS_ONLY.to_string(),
            service_runtime_event_claimed: false,
            portal_ui_rendered: false,
            policy_evaluator_runtime_claimed: false,
            timer_runtime_claimed: false,
            adapter_dispatch_claimed: false,
            child_delivery_claimed: false,
            platform_enforcement_claimed: false,
            raw_private_source_rows_included: false,
            generated_at: options.generated_at.clone(),
        })
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewReadModelReadModel {
        schema_version: options.schema_version.clone(),
        read_model_id: options.read_model_id.clone(),
        source_gate_id: source_gate_read_model.gate_id.clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        source_gate_contract_refs: source_gate_read_model.source_contract_refs.clone(),
        native_app_row_count: count_rows(&rows, "native-app"),
        native_game_row_count: count_rows(&rows, "native-game"),
        preview_ready_visible_count: count_projection_rows(&rows, READ_MODEL_PREVIEW_READY_VISIBLE),
        source_manual_required_visible_count: count_projection_rows(
            &rows,
            READ_MODEL_SOURCE_MANUAL_REQUIRED_VISIBLE,
        ),
        compiler_manual_required_visible_count: count_projection_rows(
            &rows,
            READ_MODEL_COMPILER_MANUAL_REQUIRED_VISIBLE,
        ),
        read_model_non_claims: READ_MODEL_REQUIRED_NON_CLAIMS
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

pub fn build_app_game_source_gated_policy_preview_timer_status(
    options: &AppGameSourceGatedPolicyPreviewTimerStatusOptions,
    timer_handoff: &AppGameSourceGatedPolicyPreviewTimerHandoffReadModel,
) -> AppGameSourceGatedPolicyPreviewTimerStatus {
    let rows = timer_handoff
        .rows
        .iter()
        .map(|handoff_row| {
            let timer_status_state =
                timer_status_state_for_handoff_state(&handoff_row.timer_handoff_state);
            AppGameSourceGatedPolicyPreviewTimerStatusRow {
                schema_version: options.schema_version.clone(),
                row_id: format!("{}:timer-status", handoff_row.row_id),
                source_timer_handoff_row_id: handoff_row.row_id.clone(),
                target_domain: handoff_row.target_domain.clone(),
                timer_status_state: timer_status_state.to_string(),
                timer_runtime_proof_required: timer_status_state
                    == TIMER_STATUS_TIMER_RUNTIME_PROOF_REQUIRED,
                source_freshness_proof_required: timer_status_state
                    == TIMER_STATUS_SOURCE_FRESHNESS_PROOF_REQUIRED,
                compiler_decision_proof_required: timer_status_state
                    == TIMER_STATUS_COMPILER_DECISION_PROOF_REQUIRED,
                required_proof_refs: required_proof_refs_for_timer_status(
                    options,
                    timer_status_state,
                ),
                source_evidence_refs: handoff_row.source_evidence_refs.clone(),
                service_runtime_event_claimed: false,
                portal_ui_rendered: false,
                policy_evaluator_runtime_claimed: false,
                timer_runtime_claimed: false,
                timer_scheduled: false,
                adapter_dispatch_claimed: false,
                child_delivery_claimed: false,
                platform_enforcement_claimed: false,
                raw_private_source_rows_included: false,
                generated_at: options.generated_at.clone(),
            }
        })
        .collect::<Vec<_>>();

    AppGameSourceGatedPolicyPreviewTimerStatus {
        schema_version: options.schema_version.clone(),
        status_id: options.status_id.clone(),
        source_timer_handoff_id: timer_handoff.handoff_id.clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        native_app_row_count: timer_handoff.native_app_row_count,
        native_game_row_count: timer_handoff.native_game_row_count,
        timer_runtime_proof_required_count: rows
            .iter()
            .filter(|row| row.timer_status_state == TIMER_STATUS_TIMER_RUNTIME_PROOF_REQUIRED)
            .count(),
        source_freshness_proof_required_count: rows
            .iter()
            .filter(|row| row.timer_status_state == TIMER_STATUS_SOURCE_FRESHNESS_PROOF_REQUIRED)
            .count(),
        compiler_decision_proof_required_count: rows
            .iter()
            .filter(|row| row.timer_status_state == TIMER_STATUS_COMPILER_DECISION_PROOF_REQUIRED)
            .count(),
        timer_status_non_claims: TIMER_STATUS_REQUIRED_NON_CLAIMS
            .iter()
            .map(|claim| (*claim).to_string())
            .collect(),
        service_runtime_event_claimed: false,
        portal_ui_rendered: false,
        policy_evaluator_runtime_claimed: false,
        timer_runtime_claimed: false,
        timer_scheduled: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        raw_private_source_rows_included: false,
        rows,
    }
}

pub fn app_game_source_gated_policy_preview_typescript() -> String {
    include_str!("../tests/generated/app-game-source-gated-policy-preview.ts").to_string()
}
