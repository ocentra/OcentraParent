use ocentra_app_game_core::app_game_source_gated_policy_preview::{
    app_game_source_gated_policy_preview_typescript,
    build_app_game_source_gated_policy_preview_read_model,
    build_app_game_source_gated_policy_preview_timer_status,
    AppGameSourceFreshnessPreviewGatePreviewRow, AppGameSourceFreshnessPreviewGateReadModel,
    AppGameSourceFreshnessPreviewGateRow, AppGameSourceGatedPolicyPreviewReadModelOptions,
    AppGameSourceGatedPolicyPreviewTimerHandoffReadModel,
    AppGameSourceGatedPolicyPreviewTimerHandoffRow,
    AppGameSourceGatedPolicyPreviewTimerStatusOptions,
};

type AppGameText<'a> = &'a str;

#[test]
fn app_game_source_gated_policy_preview_read_model_keeps_projection_states_and_no_claims() {
    let read_model = build_app_game_source_gated_policy_preview_read_model(
        &read_model_options(),
        &source_gate_read_model(),
    );

    assert_eq!(read_model.native_app_row_count, 1);
    assert_eq!(read_model.native_game_row_count, 2);
    assert_eq!(read_model.preview_ready_visible_count, 1);
    assert_eq!(read_model.source_manual_required_visible_count, 1);
    assert_eq!(read_model.compiler_manual_required_visible_count, 1);

    let states = read_model
        .rows
        .iter()
        .map(|row| row.projection_state.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        states,
        vec![
            "preview-ready-visible",
            "source-manual-required-visible",
            "compiler-manual-required-visible",
        ]
    );

    let ready_row = &read_model.rows[0];
    assert_eq!(
        ready_row.preview_decision_ref.as_deref(),
        Some("policy-decision-preview-app")
    );
    assert_eq!(
        ready_row.preview_compiler_status.as_deref(),
        Some("preview-ready")
    );
    assert_eq!(
        ready_row.sensitive_detail_boundary,
        "redacted-evidence-refs-only"
    );
    assert!(!ready_row.service_runtime_event_claimed);
    assert!(!ready_row.portal_ui_rendered);
    assert!(!ready_row.policy_evaluator_runtime_claimed);
    assert!(!ready_row.timer_runtime_claimed);
    assert!(!ready_row.adapter_dispatch_claimed);
    assert!(!ready_row.child_delivery_claimed);
    assert!(!ready_row.platform_enforcement_claimed);
    assert!(!ready_row.raw_private_source_rows_included);
}

#[test]
fn app_game_source_gated_policy_preview_timer_status_keeps_required_proof_states_explicit() {
    let status = build_app_game_source_gated_policy_preview_timer_status(
        &timer_status_options(),
        &timer_handoff_read_model(),
    );

    assert_eq!(status.timer_runtime_proof_required_count, 1);
    assert_eq!(status.source_freshness_proof_required_count, 1);
    assert_eq!(status.compiler_decision_proof_required_count, 1);

    let states = status
        .rows
        .iter()
        .map(|row| row.timer_status_state.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        states,
        vec![
            "timer-runtime-proof-required",
            "source-freshness-proof-required",
            "compiler-decision-proof-required",
        ]
    );

    assert_eq!(
        status.rows[0].required_proof_refs,
        vec!["future-service-timer-runtime-proof".to_string()]
    );
    assert_eq!(
        status.rows[1].required_proof_refs,
        vec!["source-freshness-proof-required".to_string()]
    );
    assert_eq!(
        status.rows[2].required_proof_refs,
        vec!["compiler-decision-proof-required".to_string()]
    );
    assert!(status.rows.iter().all(|row| !row.timer_scheduled));
    assert!(!status.timer_runtime_claimed);
    assert!(!status.adapter_dispatch_claimed);
}

#[test]
fn generated_app_game_source_gated_policy_preview_helper_stays_checked_in() {
    let checked_in = include_str!("../generated/app-game-source-gated-policy-preview.ts");
    let generated = app_game_source_gated_policy_preview_typescript();

    assert_eq!(
        checked_in
            .matches("buildGeneratedAppGameSourceGatedPolicyPreviewReadModel")
            .count(),
        1
    );
    assert_eq!(
        checked_in
            .matches("buildGeneratedAppGameSourceGatedPolicyPreviewTimerStatus")
            .count(),
        1
    );
    assert_eq!(
        generated
            .matches("buildGeneratedAppGameSourceGatedPolicyPreviewReadModel")
            .count(),
        1
    );
    assert_eq!(
        generated
            .matches("buildGeneratedAppGameSourceGatedPolicyPreviewTimerStatus")
            .count(),
        1
    );
}

fn read_model_options() -> AppGameSourceGatedPolicyPreviewReadModelOptions {
    AppGameSourceGatedPolicyPreviewReadModelOptions {
        schema_version: "v0.6".to_string(),
        read_model_id: "source-gated-policy-preview-read-model-proof".to_string(),
        generated_at: "2026-06-05T09:12:00Z".to_string(),
        source_contract_refs: vec!["app-game-source-freshness-preview-gate".to_string()],
    }
}

fn source_gate_read_model() -> AppGameSourceFreshnessPreviewGateReadModel {
    AppGameSourceFreshnessPreviewGateReadModel {
        gate_id: "source-freshness-preview-gate-proof".to_string(),
        source_contract_refs: vec![
            "app-game-source-freshness-policy-consumption".to_string(),
            "app-game-policy-preview-handoff".to_string(),
        ],
        rows: vec![
            source_gate_row(
                "source-gate-row-ready-app",
                "native-app",
                "source-fresh",
                true,
                Some(("policy-decision-preview-app", "preview-ready")),
            ),
            source_gate_row(
                "source-gate-row-manual-game",
                "native-game",
                "source-manual-required",
                false,
                None,
            ),
            source_gate_row(
                "source-gate-row-compiler-manual-game",
                "native-game",
                "compiler-manual-required",
                true,
                Some(("policy-decision-preview-game", "manual-required")),
            ),
        ],
    }
}

fn source_gate_row(
    row_id: AppGameText<'_>,
    target_domain: AppGameText<'_>,
    gate_state: AppGameText<'_>,
    source_policy_compile_allowed: bool,
    preview_row: Option<(AppGameText<'_>, AppGameText<'_>)>,
) -> AppGameSourceFreshnessPreviewGateRow {
    AppGameSourceFreshnessPreviewGateRow {
        row_id: row_id.to_string(),
        target_domain: target_domain.to_string(),
        source_readiness_id: format!("{row_id}-readiness"),
        source_policy_request_id: format!("{row_id}-request"),
        source_readiness_state: if gate_state == "source-manual-required" {
            "manual-required".to_string()
        } else {
            "fresh".to_string()
        },
        source_requirement_states: vec![if gate_state == "source-manual-required" {
            "source-manual-required".to_string()
        } else {
            "fresh".to_string()
        }],
        source_policy_compile_allowed,
        source_evidence_refs: vec![format!("{row_id}-evidence")],
        gate_state: gate_state.to_string(),
        preview_status: match gate_state {
            "source-fresh" => "preview-ready".to_string(),
            _ => "manual-required".to_string(),
        },
        preview_row: preview_row.map(|(policy_decision_id, preview_status)| {
            AppGameSourceFreshnessPreviewGatePreviewRow {
                policy_decision_id: policy_decision_id.to_string(),
                preview_status: preview_status.to_string(),
            }
        }),
    }
}

fn timer_status_options() -> AppGameSourceGatedPolicyPreviewTimerStatusOptions {
    AppGameSourceGatedPolicyPreviewTimerStatusOptions {
        schema_version: "v0.6".to_string(),
        status_id: "source-gated-policy-preview-timer-status-proof".to_string(),
        generated_at: "2026-06-05T09:12:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-handoff".to_string(),
            "docs/expectations/app-game-evidence.md".to_string(),
            "docs/expectations/policy.md".to_string(),
        ],
        timer_runtime_proof_ref: "future-service-timer-runtime-proof".to_string(),
        source_freshness_proof_ref: "source-freshness-proof-required".to_string(),
        compiler_decision_proof_ref: "compiler-decision-proof-required".to_string(),
    }
}

fn timer_handoff_read_model() -> AppGameSourceGatedPolicyPreviewTimerHandoffReadModel {
    AppGameSourceGatedPolicyPreviewTimerHandoffReadModel {
        handoff_id: "source-gated-policy-preview-timer-handoff-proof".to_string(),
        native_app_row_count: 1,
        native_game_row_count: 2,
        rows: vec![
            timer_handoff_row(
                "source-gate-row-ready-app:source-gated-preview-read-model:timer-handoff",
                "native-app",
                "ready-for-timer-sequencing",
            ),
            timer_handoff_row(
                "source-gate-row-manual-game:source-gated-preview-read-model:timer-handoff",
                "native-game",
                "source-manual-required-before-timer",
            ),
            timer_handoff_row(
                "source-gate-row-compiler-manual-game:source-gated-preview-read-model:timer-handoff",
                "native-game",
                "compiler-manual-required-before-timer",
            ),
        ],
    }
}

fn timer_handoff_row(
    row_id: AppGameText<'_>,
    target_domain: AppGameText<'_>,
    timer_handoff_state: AppGameText<'_>,
) -> AppGameSourceGatedPolicyPreviewTimerHandoffRow {
    AppGameSourceGatedPolicyPreviewTimerHandoffRow {
        row_id: row_id.to_string(),
        target_domain: target_domain.to_string(),
        timer_handoff_state: timer_handoff_state.to_string(),
        source_evidence_refs: vec![format!("{row_id}-evidence")],
    }
}
