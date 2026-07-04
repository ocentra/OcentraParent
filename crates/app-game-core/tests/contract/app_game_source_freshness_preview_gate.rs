use ocentra_app_game_core::app_game_policy_preview_handoff::{
    build_app_game_policy_preview_handoff_row, AppGamePolicyPreviewCompiledDecision,
    AppGamePolicyPreviewHandoffOptions,
};
use ocentra_app_game_core::app_game_source_freshness_preview_gate::{
    app_game_source_freshness_preview_gate_typescript,
    build_app_game_source_freshness_preview_gate_read_model,
    build_app_game_source_freshness_preview_gate_row, AppGameSourceFreshnessPolicyReadiness,
    AppGameSourceFreshnessPreviewGateEntry, AppGameSourceFreshnessPreviewGateOptions,
    AppGameSourceFreshnessRequirementResult,
};

#[test]
fn app_game_source_freshness_preview_gate_keeps_source_and_compiler_states_explicit() {
    let ready_row =
        build_app_game_source_freshness_preview_gate_row(&gate_options(), &ready_app_entry());
    assert_eq!(ready_row.target_domain, "native-app");
    assert_eq!(ready_row.preview_status, "preview-ready");
    assert_eq!(ready_row.gate_state, "source-fresh");
    assert!(!ready_row.adapter_dispatch_claimed);
    assert!(!ready_row.policy_evaluator_runtime_claimed);

    let manual_row =
        build_app_game_source_freshness_preview_gate_row(&gate_options(), &manual_game_entry());
    assert_eq!(manual_row.target_domain, "native-game");
    assert_eq!(manual_row.preview_status, "manual-required");
    assert_eq!(manual_row.gate_state, "source-manual-required");
    assert!(manual_row.preview_row.is_none());
    assert!(!manual_row.compiled_decision_provided);

    let compiler_manual_row = build_app_game_source_freshness_preview_gate_row(
        &gate_options(),
        &compiler_manual_game_entry(),
    );
    assert_eq!(compiler_manual_row.target_domain, "native-game");
    assert_eq!(compiler_manual_row.preview_status, "manual-required");
    assert_eq!(compiler_manual_row.gate_state, "compiler-manual-required");
    assert!(!compiler_manual_row.adapter_dispatch_claimed);
    assert!(!compiler_manual_row.platform_enforcement_claimed);
}

#[test]
fn app_game_source_freshness_preview_gate_makes_domain_mismatches_manual_required() {
    let row = build_app_game_source_freshness_preview_gate_row(
        &gate_options(),
        &AppGameSourceFreshnessPreviewGateEntry {
            row_id: "source-gate-row-domain-mismatch".to_string(),
            source_readiness: ready_game_source(),
            compiled_decision_provided: true,
            compiled_decision_target_domain: Some("native-app".to_string()),
            preview_row: Some(build_app_game_policy_preview_handoff_row(
                &preview_options(),
                &app_decision(),
            )),
        },
    );

    assert_eq!(row.preview_status, "manual-required");
    assert_eq!(row.gate_state, "compiler-manual-required");
    assert!(row.preview_row.is_none());
}

#[test]
fn app_game_source_freshness_preview_gate_read_model_counts_rows_by_gate_state() {
    let read_model = build_app_game_source_freshness_preview_gate_read_model(
        &gate_options(),
        &[
            ready_app_entry(),
            manual_game_entry(),
            compiler_manual_game_entry(),
        ],
    );

    assert_eq!(read_model.native_app_row_count, 1);
    assert_eq!(read_model.native_game_row_count, 2);
    assert_eq!(read_model.preview_ready_count, 1);
    assert_eq!(read_model.manual_required_count, 2);
    assert_eq!(read_model.source_manual_required_count, 1);
    assert_eq!(read_model.compiler_manual_required_count, 1);
    assert!(!read_model.adapter_dispatch_claimed);
}

#[test]
fn generated_app_game_source_freshness_preview_gate_helper_stays_checked_in() {
    let checked_in = include_str!("../generated/app-game-source-freshness-preview-gate.ts");
    let generated = app_game_source_freshness_preview_gate_typescript();

    assert_eq!(checked_in, generated);
    assert_eq!(
        generated
            .matches("buildGeneratedAppGameSourceFreshnessPreviewGateRow")
            .count(),
        1
    );
    assert_eq!(
        generated
            .matches("buildGeneratedAppGameSourceFreshnessPreviewGateReadModel")
            .count(),
        1
    );
}

fn gate_options() -> AppGameSourceFreshnessPreviewGateOptions {
    AppGameSourceFreshnessPreviewGateOptions {
        schema_version: "v0.6".to_string(),
        gate_id: "source-freshness-preview-gate-proof".to_string(),
        generated_at: "2026-06-05T14:45:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-freshness-policy-consumption".to_string(),
            "app-game-policy-preview-handoff".to_string(),
            "docs/expectations/app-game-evidence.md".to_string(),
            "docs/expectations/policy.md".to_string(),
        ],
    }
}

fn preview_options() -> AppGamePolicyPreviewHandoffOptions {
    AppGamePolicyPreviewHandoffOptions {
        schema_version: "v0.6".to_string(),
        handoff_id: "app-game-policy-preview-handoff-proof".to_string(),
        generated_at: "2026-06-05T14:45:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-policy-target-compiler".to_string(),
            "docs/expectations/policy.md".to_string(),
            "docs/expectations/enforcement.md".to_string(),
        ],
    }
}

fn app_decision() -> AppGamePolicyPreviewCompiledDecision {
    AppGamePolicyPreviewCompiledDecision {
        schema_version: "v0.6".to_string(),
        compiled_decision_id: "compiled-decision-preview-app".to_string(),
        compile_request_id: "compile-request-preview-app".to_string(),
        target_kind: "specific-app".to_string(),
        policy_decision_id: "policy-decision-preview-app".to_string(),
        policy_action: "time-limit".to_string(),
        outcome_state: "dry-run-ready".to_string(),
        rejection_reason: "none".to_string(),
        rule_refs: vec!["policy-rule-app-game-preview-1".to_string()],
        evidence_references: vec!["app-game-policy-preview-evidence-1".to_string()],
        capability_refs: vec!["capability-preview-1".to_string()],
        authority_refs: vec!["authority-preview-1".to_string()],
        audit_refs: vec!["audit-preview-1".to_string()],
        dry_run: true,
        enforcement_handoff_state: "disabled".to_string(),
    }
}

fn manual_game_decision() -> AppGamePolicyPreviewCompiledDecision {
    AppGamePolicyPreviewCompiledDecision {
        schema_version: "v0.6".to_string(),
        compiled_decision_id: "compiled-decision-preview-game-manual".to_string(),
        compile_request_id: "compile-request-preview-game".to_string(),
        target_kind: "specific-game".to_string(),
        policy_decision_id: "policy-decision-preview-game".to_string(),
        policy_action: "block".to_string(),
        outcome_state: "manual-required".to_string(),
        rejection_reason: "block-launch-manual-required".to_string(),
        rule_refs: vec!["policy-rule-app-game-preview-1".to_string()],
        evidence_references: vec!["app-game-policy-preview-evidence-1".to_string()],
        capability_refs: vec!["capability-preview-game-manual".to_string()],
        authority_refs: vec!["authority-preview-game-manual".to_string()],
        audit_refs: vec!["audit-preview-game-manual".to_string()],
        dry_run: true,
        enforcement_handoff_state: "disabled".to_string(),
    }
}

fn ready_app_entry() -> AppGameSourceFreshnessPreviewGateEntry {
    AppGameSourceFreshnessPreviewGateEntry {
        row_id: "source-gate-row-ready-app".to_string(),
        source_readiness: ready_app_source(),
        compiled_decision_provided: true,
        compiled_decision_target_domain: Some("native-app".to_string()),
        preview_row: Some(build_app_game_policy_preview_handoff_row(
            &preview_options(),
            &app_decision(),
        )),
    }
}

fn manual_game_entry() -> AppGameSourceFreshnessPreviewGateEntry {
    AppGameSourceFreshnessPreviewGateEntry {
        row_id: "source-gate-row-manual-game".to_string(),
        source_readiness: manual_game_source(),
        compiled_decision_provided: false,
        compiled_decision_target_domain: None,
        preview_row: None,
    }
}

fn compiler_manual_game_entry() -> AppGameSourceFreshnessPreviewGateEntry {
    AppGameSourceFreshnessPreviewGateEntry {
        row_id: "source-gate-row-compiler-manual-game".to_string(),
        source_readiness: ready_game_source(),
        compiled_decision_provided: true,
        compiled_decision_target_domain: Some("native-game".to_string()),
        preview_row: Some(build_app_game_policy_preview_handoff_row(
            &preview_options(),
            &manual_game_decision(),
        )),
    }
}

fn ready_app_source() -> AppGameSourceFreshnessPolicyReadiness {
    AppGameSourceFreshnessPolicyReadiness {
        readiness_id: "ready-app-readiness".to_string(),
        policy_request_id: "ready-app-request".to_string(),
        target_kind: "native-app".to_string(),
        readiness_state: "policy-ready".to_string(),
        policy_compile_allowed: true,
        requirement_results: vec![
            AppGameSourceFreshnessRequirementResult {
                requirement_state: "fresh".to_string(),
                reason_code: Some("inventory-collected".to_string()),
            },
            AppGameSourceFreshnessRequirementResult {
                requirement_state: "fresh".to_string(),
                reason_code: Some("runtime-observed".to_string()),
            },
            AppGameSourceFreshnessRequirementResult {
                requirement_state: "fresh".to_string(),
                reason_code: Some("foreground-proof".to_string()),
            },
        ],
        policy_evidence_refs: vec![
            "evidence-app-inventory-parental-controls-helper".to_string(),
            "evidence-app-runtime-parental-controls-helper".to_string(),
            "evidence-app-foreground-parental-controls-helper".to_string(),
        ],
    }
}

fn ready_game_source() -> AppGameSourceFreshnessPolicyReadiness {
    AppGameSourceFreshnessPolicyReadiness {
        readiness_id: "ready-game-readiness".to_string(),
        policy_request_id: "ready-game-request".to_string(),
        target_kind: "native-game".to_string(),
        readiness_state: "policy-ready".to_string(),
        policy_compile_allowed: true,
        requirement_results: vec![AppGameSourceFreshnessRequirementResult {
            requirement_state: "fresh".to_string(),
            reason_code: Some("game-inventory-collected".to_string()),
        }],
        policy_evidence_refs: vec!["evidence-game-policy-ready".to_string()],
    }
}

fn manual_game_source() -> AppGameSourceFreshnessPolicyReadiness {
    AppGameSourceFreshnessPolicyReadiness {
        readiness_id: "manual-game-readiness".to_string(),
        policy_request_id: "manual-game-request".to_string(),
        target_kind: "native-game".to_string(),
        readiness_state: "manual-required".to_string(),
        policy_compile_allowed: false,
        requirement_results: vec![
            AppGameSourceFreshnessRequirementResult {
                requirement_state: "stale".to_string(),
                reason_code: Some("stale-source".to_string()),
            },
            AppGameSourceFreshnessRequirementResult {
                requirement_state: "missing".to_string(),
                reason_code: Some("missing-evidence".to_string()),
            },
            AppGameSourceFreshnessRequirementResult {
                requirement_state: "not-claimed".to_string(),
                reason_code: None,
            },
        ],
        policy_evidence_refs: vec!["evidence-game-manual-required".to_string()],
    }
}
