use ocentra_app_game_core::app_game_policy_preview_handoff::{
    app_game_policy_preview_handoff_typescript, build_app_game_policy_preview_handoff_read_model,
    build_app_game_policy_preview_handoff_row, AppGamePolicyPreviewCompiledDecision,
    AppGamePolicyPreviewHandoffOptions,
};

#[test]
fn app_game_policy_preview_handoff_builds_native_app_preview_rows_without_runtime_claims() {
    let row =
        build_app_game_policy_preview_handoff_row(&preview_options(), &app_compiled_decision());

    assert_eq!(row.target_domain, "native-app");
    assert_eq!(row.preview_status, "preview-ready");
    assert_eq!(row.policy_action, "time-limit");
    assert_eq!(
        row.source_compiled_decision_id,
        "compiled-decision-preview-app"
    );
    assert_eq!(
        row.evidence_references,
        vec!["app-game-policy-preview-evidence-1"]
    );
    assert_eq!(row.rule_refs, vec!["policy-rule-app-game-preview-1"]);
    assert_eq!(row.capability_refs, vec!["capability-preview-1"]);
    assert_eq!(row.audit_refs, vec!["audit-preview-1"]);
    assert!(row.dry_run);
    assert_eq!(row.enforcement_handoff_state, "disabled");
    assert!(!row.policy_evaluator_runtime_claimed);
    assert!(!row.timer_runtime_claimed);
    assert!(!row.adapter_dispatch_claimed);
    assert!(!row.child_delivery_claimed);
    assert!(!row.platform_enforcement_claimed);
}

#[test]
fn app_game_policy_preview_handoff_keeps_game_manual_rows_manual_required() {
    let row =
        build_app_game_policy_preview_handoff_row(&preview_options(), &game_manual_decision());

    assert_eq!(row.target_domain, "native-game");
    assert_eq!(row.preview_status, "manual-required");
    assert_eq!(row.rejection_reason, "block-launch-manual-required");
    assert_eq!(row.policy_action, "block");
    assert_eq!(row.adapter_dispatch_state, "not-dispatched");
    assert!(!row.adapter_dispatch_claimed);
    assert!(!row.platform_enforcement_claimed);
}

#[test]
fn app_game_policy_preview_handoff_read_model_counts_rows_by_domain_and_status() {
    let read_model = build_app_game_policy_preview_handoff_read_model(
        &preview_options(),
        &[app_compiled_decision(), game_manual_decision()],
    );

    assert_eq!(read_model.native_app_row_count, 1);
    assert_eq!(read_model.native_game_row_count, 1);
    assert_eq!(read_model.preview_ready_count, 1);
    assert_eq!(read_model.manual_required_count, 1);
    assert_eq!(read_model.rejected_count, 0);
    assert_eq!(
        read_model
            .rows
            .iter()
            .map(|row| row.row_id.as_str())
            .collect::<Vec<_>>(),
        vec![
            "compiled-decision-preview-app:preview",
            "compiled-decision-preview-game-manual:preview",
        ]
    );
    assert_eq!(
        read_model.source_contract_refs,
        vec![
            "app-game-policy-target-compiler",
            "docs/expectations/policy.md",
            "docs/expectations/enforcement.md",
        ]
    );
    assert!(!read_model.policy_evaluator_runtime_claimed);
    assert!(!read_model.adapter_dispatch_claimed);
}

#[test]
fn generated_app_game_policy_preview_handoff_helper_stays_checked_in() {
    let checked_in = include_str!(
        "../../../../packages/schema-domain/src/generated-app-game-policy-preview-handoff-values.ts"
    );
    let generated = app_game_policy_preview_handoff_typescript();

    assert_eq!(checked_in, generated);
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

fn app_compiled_decision() -> AppGamePolicyPreviewCompiledDecision {
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

fn game_manual_decision() -> AppGamePolicyPreviewCompiledDecision {
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
