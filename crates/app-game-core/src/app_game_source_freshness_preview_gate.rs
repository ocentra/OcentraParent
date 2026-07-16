use crate::app_game_policy_preview_handoff::AppGamePolicyPreviewHandoffRow;

const PREVIEW_READY: &str = "preview-ready";
const MANUAL_REQUIRED: &str = "manual-required";
const SOURCE_FRESH: &str = "source-fresh";
const SOURCE_MANUAL_REQUIRED: &str = "source-manual-required";
const COMPILER_MANUAL_REQUIRED: &str = "compiler-manual-required";
const TARGET_DOMAIN_NATIVE_APP: &str = "native-app";
const TARGET_DOMAIN_NATIVE_GAME: &str = "native-game";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceFreshnessPreviewGateOptions {
    pub schema_version: String,
    pub gate_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceFreshnessRequirementResult {
    pub requirement_state: String,
    pub reason_code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceFreshnessPolicyReadiness {
    pub readiness_id: String,
    pub policy_request_id: String,
    pub target_kind: String,
    pub readiness_state: String,
    pub policy_compile_allowed: bool,
    pub requirement_results: Vec<AppGameSourceFreshnessRequirementResult>,
    pub policy_evidence_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceFreshnessPreviewGateEntry {
    pub row_id: String,
    pub source_readiness: AppGameSourceFreshnessPolicyReadiness,
    pub compiled_decision_provided: bool,
    pub compiled_decision_target_domain: Option<String>,
    pub preview_row: Option<AppGamePolicyPreviewHandoffRow>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceFreshnessPreviewGateRow {
    pub schema_version: String,
    pub row_id: String,
    pub target_domain: String,
    pub source_readiness_id: String,
    pub source_policy_request_id: String,
    pub source_readiness_state: String,
    pub source_policy_compile_allowed: bool,
    pub source_requirement_states: Vec<String>,
    pub source_reason_codes: Vec<Option<String>>,
    pub source_evidence_refs: Vec<String>,
    pub compiled_decision_provided: bool,
    pub preview_status: String,
    pub gate_state: String,
    pub preview_row: Option<AppGamePolicyPreviewHandoffRow>,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub generated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGameSourceFreshnessPreviewGateReadModel {
    pub schema_version: String,
    pub gate_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub rows: Vec<AppGameSourceFreshnessPreviewGateRow>,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub preview_ready_count: usize,
    pub manual_required_count: usize,
    pub source_manual_required_count: usize,
    pub compiler_manual_required_count: usize,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
}

pub fn build_app_game_source_freshness_preview_gate_row(
    options: &AppGameSourceFreshnessPreviewGateOptions,
    entry: &AppGameSourceFreshnessPreviewGateEntry,
) -> AppGameSourceFreshnessPreviewGateRow {
    let target_domain =
        app_game_source_freshness_preview_gate_target_domain(&entry.source_readiness.target_kind)
            .to_string();
    let preview_row = preview_row_for_entry(entry, &target_domain);
    let preview_status = preview_row
        .as_ref()
        .map(|row| row.preview_status.clone())
        .unwrap_or_else(|| MANUAL_REQUIRED.to_string());
    let gate_state = preview_gate_state(
        &entry.source_readiness.readiness_state,
        preview_row.as_ref(),
    )
    .to_string();

    AppGameSourceFreshnessPreviewGateRow {
        schema_version: options.schema_version.clone(),
        row_id: entry.row_id.clone(),
        target_domain,
        source_readiness_id: entry.source_readiness.readiness_id.clone(),
        source_policy_request_id: entry.source_readiness.policy_request_id.clone(),
        source_readiness_state: entry.source_readiness.readiness_state.clone(),
        source_policy_compile_allowed: entry.source_readiness.policy_compile_allowed,
        source_requirement_states: entry
            .source_readiness
            .requirement_results
            .iter()
            .map(|result| result.requirement_state.clone())
            .collect(),
        source_reason_codes: entry
            .source_readiness
            .requirement_results
            .iter()
            .map(|result| result.reason_code.clone())
            .collect(),
        source_evidence_refs: entry.source_readiness.policy_evidence_refs.clone(),
        compiled_decision_provided: entry.compiled_decision_provided,
        preview_status,
        gate_state,
        preview_row,
        policy_evaluator_runtime_claimed: false,
        timer_runtime_claimed: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        generated_at: options.generated_at.clone(),
    }
}

pub fn build_app_game_source_freshness_preview_gate_read_model(
    options: &AppGameSourceFreshnessPreviewGateOptions,
    entries: &[AppGameSourceFreshnessPreviewGateEntry],
) -> AppGameSourceFreshnessPreviewGateReadModel {
    let rows = entries
        .iter()
        .map(|entry| build_app_game_source_freshness_preview_gate_row(options, entry))
        .collect::<Vec<_>>();

    AppGameSourceFreshnessPreviewGateReadModel {
        schema_version: options.schema_version.clone(),
        gate_id: options.gate_id.clone(),
        generated_at: options.generated_at.clone(),
        source_contract_refs: options.source_contract_refs.clone(),
        native_app_row_count: rows
            .iter()
            .filter(|row| row.target_domain == TARGET_DOMAIN_NATIVE_APP)
            .count(),
        native_game_row_count: rows
            .iter()
            .filter(|row| row.target_domain == TARGET_DOMAIN_NATIVE_GAME)
            .count(),
        preview_ready_count: rows
            .iter()
            .filter(|row| row.preview_status == PREVIEW_READY)
            .count(),
        manual_required_count: rows
            .iter()
            .filter(|row| row.preview_status == MANUAL_REQUIRED)
            .count(),
        source_manual_required_count: rows
            .iter()
            .filter(|row| row.gate_state == SOURCE_MANUAL_REQUIRED)
            .count(),
        compiler_manual_required_count: rows
            .iter()
            .filter(|row| row.gate_state == COMPILER_MANUAL_REQUIRED)
            .count(),
        policy_evaluator_runtime_claimed: false,
        timer_runtime_claimed: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        rows,
    }
}

fn preview_row_for_entry(
    entry: &AppGameSourceFreshnessPreviewGateEntry,
    target_domain: &str,
) -> Option<AppGamePolicyPreviewHandoffRow> {
    if entry.source_readiness.readiness_state == MANUAL_REQUIRED {
        return None;
    }

    let preview_row = entry.preview_row.as_ref()?;
    let compiled_target_domain = entry.compiled_decision_target_domain.as_deref()?;

    if compiled_target_domain != target_domain || preview_row.target_domain != target_domain {
        return None;
    }

    Some(preview_row.clone())
}

fn preview_gate_state(
    source_readiness_state: &str,
    preview_row: Option<&AppGamePolicyPreviewHandoffRow>,
) -> &'static str {
    if source_readiness_state == MANUAL_REQUIRED {
        return SOURCE_MANUAL_REQUIRED;
    }

    match preview_row.map(|row| row.preview_status.as_str()) {
        Some(PREVIEW_READY) => SOURCE_FRESH,
        _ => COMPILER_MANUAL_REQUIRED,
    }
}

fn app_game_source_freshness_preview_gate_target_domain(target_kind: &str) -> &'static str {
    match target_kind {
        "native-game" | "all-native-games" => TARGET_DOMAIN_NATIVE_GAME,
        _ => TARGET_DOMAIN_NATIVE_APP,
    }
}

pub fn app_game_source_freshness_preview_gate_typescript() -> String {
    include_str!("../tests/generated/app-game-source-freshness-preview-gate.ts").to_string()
}
