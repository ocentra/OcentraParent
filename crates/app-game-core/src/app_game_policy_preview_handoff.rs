const PREVIEW_READY: &str = "preview-ready";
const MANUAL_REQUIRED: &str = "manual-required";
const REJECTED: &str = "rejected";
const TARGET_DOMAIN_NATIVE_APP: &str = "native-app";
const TARGET_DOMAIN_NATIVE_GAME: &str = "native-game";
const NOT_CLAIMED: &str = "not-claimed";
const NOT_DISPATCHED: &str = "not-dispatched";

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGamePolicyPreviewHandoffOptions {
    pub schema_version: String,
    pub handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGamePolicyPreviewCompiledDecision {
    pub schema_version: String,
    pub compiled_decision_id: String,
    pub compile_request_id: String,
    pub target_kind: String,
    pub policy_decision_id: String,
    pub policy_action: String,
    pub outcome_state: String,
    pub rejection_reason: String,
    pub rule_refs: Vec<String>,
    pub evidence_references: Vec<String>,
    pub capability_refs: Vec<String>,
    pub authority_refs: Vec<String>,
    pub audit_refs: Vec<String>,
    pub dry_run: bool,
    pub enforcement_handoff_state: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGamePolicyPreviewHandoffRow {
    pub schema_version: String,
    pub row_id: String,
    pub target_domain: String,
    pub source_compiled_decision_id: String,
    pub source_compile_request_id: String,
    pub source_target_kind: String,
    pub policy_decision_id: String,
    pub policy_action: String,
    pub outcome_state: String,
    pub preview_status: String,
    pub rejection_reason: String,
    pub rule_refs: Vec<String>,
    pub evidence_references: Vec<String>,
    pub capability_refs: Vec<String>,
    pub authority_refs: Vec<String>,
    pub audit_refs: Vec<String>,
    pub dry_run: bool,
    pub enforcement_handoff_state: String,
    pub policy_evaluator_runtime_claim_state: String,
    pub timer_runtime_claim_state: String,
    pub adapter_dispatch_state: String,
    pub child_delivery_claim_state: String,
    pub platform_enforcement_claim_state: String,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
    pub generated_at: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AppGamePolicyPreviewHandoffReadModel {
    pub schema_version: String,
    pub handoff_id: String,
    pub generated_at: String,
    pub source_contract_refs: Vec<String>,
    pub rows: Vec<AppGamePolicyPreviewHandoffRow>,
    pub native_app_row_count: usize,
    pub native_game_row_count: usize,
    pub preview_ready_count: usize,
    pub manual_required_count: usize,
    pub rejected_count: usize,
    pub policy_evaluator_runtime_claimed: bool,
    pub timer_runtime_claimed: bool,
    pub adapter_dispatch_claimed: bool,
    pub child_delivery_claimed: bool,
    pub platform_enforcement_claimed: bool,
}

pub fn build_app_game_policy_preview_handoff_row(
    options: &AppGamePolicyPreviewHandoffOptions,
    decision: &AppGamePolicyPreviewCompiledDecision,
) -> AppGamePolicyPreviewHandoffRow {
    AppGamePolicyPreviewHandoffRow {
        schema_version: decision.schema_version.clone(),
        row_id: format!("{}:preview", decision.compiled_decision_id),
        target_domain: app_game_policy_preview_target_domain_for_kind(&decision.target_kind)
            .to_string(),
        source_compiled_decision_id: decision.compiled_decision_id.clone(),
        source_compile_request_id: decision.compile_request_id.clone(),
        source_target_kind: decision.target_kind.clone(),
        policy_decision_id: decision.policy_decision_id.clone(),
        policy_action: decision.policy_action.clone(),
        outcome_state: decision.outcome_state.clone(),
        preview_status: app_game_policy_preview_status_for_outcome(&decision.outcome_state)
            .to_string(),
        rejection_reason: decision.rejection_reason.clone(),
        rule_refs: decision.rule_refs.clone(),
        evidence_references: decision.evidence_references.clone(),
        capability_refs: decision.capability_refs.clone(),
        authority_refs: decision.authority_refs.clone(),
        audit_refs: decision.audit_refs.clone(),
        dry_run: decision.dry_run,
        enforcement_handoff_state: decision.enforcement_handoff_state.clone(),
        policy_evaluator_runtime_claim_state: NOT_CLAIMED.to_string(),
        timer_runtime_claim_state: NOT_CLAIMED.to_string(),
        adapter_dispatch_state: NOT_DISPATCHED.to_string(),
        child_delivery_claim_state: NOT_CLAIMED.to_string(),
        platform_enforcement_claim_state: NOT_CLAIMED.to_string(),
        policy_evaluator_runtime_claimed: false,
        timer_runtime_claimed: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        generated_at: options.generated_at.clone(),
    }
}

pub fn build_app_game_policy_preview_handoff_read_model(
    options: &AppGamePolicyPreviewHandoffOptions,
    decisions: &[AppGamePolicyPreviewCompiledDecision],
) -> AppGamePolicyPreviewHandoffReadModel {
    let rows = decisions
        .iter()
        .map(|decision| build_app_game_policy_preview_handoff_row(options, decision))
        .collect::<Vec<_>>();

    AppGamePolicyPreviewHandoffReadModel {
        schema_version: options.schema_version.clone(),
        handoff_id: options.handoff_id.clone(),
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
        rejected_count: rows
            .iter()
            .filter(|row| row.preview_status == REJECTED)
            .count(),
        policy_evaluator_runtime_claimed: false,
        timer_runtime_claimed: false,
        adapter_dispatch_claimed: false,
        child_delivery_claimed: false,
        platform_enforcement_claimed: false,
        rows,
    }
}

pub fn app_game_policy_preview_target_domain_for_kind(target_kind: &str) -> &'static str {
    if is_game_target_kind(target_kind) {
        TARGET_DOMAIN_NATIVE_GAME
    } else {
        TARGET_DOMAIN_NATIVE_APP
    }
}

fn app_game_policy_preview_status_for_outcome(outcome_state: &str) -> &'static str {
    match outcome_state {
        "dry-run-ready" => PREVIEW_READY,
        "manual-required" => MANUAL_REQUIRED,
        "rejected" => REJECTED,
        _ => MANUAL_REQUIRED,
    }
}

fn is_game_target_kind(target_kind: &str) -> bool {
    matches!(
        target_kind,
        "specific-game"
            | "launcher-game-id"
            | "store-game-id"
            | "game-category"
            | "unknown-game"
            | "new-game"
            | "launcher-game-candidate"
            | "multiplayer-game"
            | "ugc-game"
            | "purchase-capable-game"
            | "mature-game"
            | "all-games"
    )
}

pub fn app_game_policy_preview_handoff_values_typescript() -> String {
    include_str!(
        "../../../packages/schema-domain/src/generated-app-game-policy-preview-handoff-values.ts"
    )
    .to_string()
}

pub fn app_game_policy_preview_handoff_typescript() -> String {
    app_game_policy_preview_handoff_values_typescript()
}
