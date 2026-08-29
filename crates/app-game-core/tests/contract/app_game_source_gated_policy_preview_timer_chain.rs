use ocentra_app_game_core::app_game_source_gated_policy_preview_timer_chain::{
    app_game_source_gated_policy_preview_timer_chain_typescript,
    build_app_game_source_gated_policy_preview_timer_handoff,
    build_app_game_source_gated_policy_preview_timer_runtime_readiness,
    build_app_game_source_gated_policy_preview_timer_service_readiness_handoff,
    build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_handoff,
    AppGameSourceGatedPolicyPreviewReadModelInput,
    AppGameSourceGatedPolicyPreviewReadModelRowInput,
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentInput,
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowInput,
    AppGameSourceGatedPolicyPreviewTimerHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelInput,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowInput,
    AppGameSourceGatedPolicyPreviewTimerStatusInput,
    AppGameSourceGatedPolicyPreviewTimerStatusRowInput,
};

type AppGameText<'a> = &'a str;

#[test]
fn timer_handoff_keeps_preview_projection_mapping() {
    let handoff = build_app_game_source_gated_policy_preview_timer_handoff(
        &timer_handoff_options(),
        &preview_read_model_input(),
    );

    assert_eq!(handoff.timer_sequence_candidate_count, 1);
    assert_eq!(handoff.source_manual_blocked_count, 1);
    assert_eq!(handoff.compiler_manual_blocked_count, 1);
    assert_eq!(
        handoff
            .rows
            .iter()
            .map(|row| row.timer_handoff_state.as_str())
            .collect::<Vec<_>>(),
        vec![
            "ready-for-timer-sequencing",
            "source-manual-required-before-timer",
            "compiler-manual-required-before-timer",
        ]
    );
}

#[test]
fn timer_runtime_readiness_keeps_proof_ref_expansion_only_for_runtime_rows() {
    let readiness = build_app_game_source_gated_policy_preview_timer_runtime_readiness(
        &timer_runtime_options(),
        &timer_status_input(),
    );

    assert_eq!(readiness.runtime_proof_required_count, 1);
    assert_eq!(readiness.blocked_by_source_freshness_count, 1);
    assert_eq!(readiness.blocked_by_compiler_decision_count, 1);
    assert_eq!(
        readiness.rows[0].required_proof_refs,
        vec![
            "future-service-timer-runtime-proof".to_string(),
            "future-scheduler-persistence-proof".to_string(),
            "future-timer-audit-proof".to_string(),
            "future-timer-rollback-proof".to_string(),
        ]
    );
    assert_eq!(
        readiness.rows[1].required_proof_refs,
        vec!["source-freshness-proof-required".to_string()]
    );
}

#[test]
fn timer_runtime_readiness_keeps_scheduler_persistence_fail_closed() {
    let readiness = build_app_game_source_gated_policy_preview_timer_runtime_readiness(
        &timer_runtime_options(),
        &timer_status_input(),
    );

    let runtime_row = &readiness.rows[0];
    assert!(runtime_row.scheduler_persistence_proof_required);
    assert!(!runtime_row.scheduler_persistence_claimed);
    assert!(!runtime_row.timer_scheduled);
    assert!(!readiness.scheduler_persistence_claimed);
    assert!(!readiness.timer_scheduled);
    assert!(readiness
        .runtime_readiness_non_claims
        .iter()
        .any(|claim| claim == "no-scheduler-persistence"));
}

#[test]
fn timer_service_readiness_handoff_keeps_parent_surface_to_service_mapping() {
    let handoff = build_app_game_source_gated_policy_preview_timer_service_readiness_handoff(
        &service_readiness_options(),
        &parent_surface_intent_input(),
    );

    assert_eq!(handoff.service_read_api_proof_required_count, 1);
    assert_eq!(handoff.blocked_by_source_freshness_count, 1);
    assert_eq!(handoff.blocked_by_compiler_decision_count, 1);
    assert_eq!(
        handoff.rows[0].required_proof_refs,
        vec![
            "future-parent-surface-proof".to_string(),
            "future-service-readiness-proof".to_string(),
            "future-service-read-api-proof".to_string(),
        ]
    );
    assert_eq!(
        handoff.rows[0].service_read_api_ref,
        "future-service-read-api-contract-ref"
    );
}

#[test]
fn timer_protocol_handoff_keeps_service_read_model_to_protocol_mapping() {
    let handoff =
        build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_handoff(
            &protocol_handoff_options(),
            &service_readiness_read_model_input(),
        );

    assert_eq!(handoff.protocol_proof_required_count, 1);
    assert_eq!(handoff.blocked_by_source_freshness_count, 1);
    assert_eq!(handoff.blocked_by_compiler_decision_count, 1);
    assert_eq!(
        handoff.rows[0].required_protocol_proof_refs,
        vec![
            "future-agent-protocol-command-contract-proof".to_string(),
            "future-agent-protocol-event-contract-proof".to_string(),
            "future-rust-protocol-mirror-proof".to_string(),
            "future-service-handler-proof".to_string(),
        ]
    );
}

#[test]
fn generated_timer_chain_helper_stays_checked_in() {
    let checked_in =
        include_str!("../generated/app-game-source-gated-policy-preview-timer-chain.ts");

    assert_eq!(
        checked_in,
        app_game_source_gated_policy_preview_timer_chain_typescript()
    );
    assert_eq!(
        checked_in
            .matches("buildGeneratedAppGameSourceGatedPolicyPreviewTimerHandoff")
            .count(),
        1
    );
    assert_eq!(
        checked_in
            .matches(
                "buildGeneratedAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff"
            )
            .count(),
        1
    );
}

fn timer_handoff_options() -> AppGameSourceGatedPolicyPreviewTimerHandoffOptions {
    AppGameSourceGatedPolicyPreviewTimerHandoffOptions {
        schema_version: "v0.6".to_string(),
        handoff_id: "source-gated-policy-preview-timer-handoff-proof".to_string(),
        generated_at: "2026-06-05T09:12:00Z".to_string(),
        source_contract_refs: vec!["app-game-source-gated-policy-preview-read-model".to_string()],
    }
}

fn preview_read_model_input() -> AppGameSourceGatedPolicyPreviewReadModelInput {
    AppGameSourceGatedPolicyPreviewReadModelInput {
        read_model_id: "source-gated-policy-preview-read-model-proof".to_string(),
        native_app_row_count: 1,
        native_game_row_count: 2,
        rows: vec![
            preview_row(
                "source-gate-row-ready-app:source-gated-preview-read-model",
                "native-app",
                "preview-ready-visible",
                Some("policy-decision-preview-app"),
            ),
            preview_row(
                "source-gate-row-manual-game:source-gated-preview-read-model",
                "native-game",
                "source-manual-required-visible",
                None,
            ),
            preview_row(
                "source-gate-row-compiler-manual-game:source-gated-preview-read-model",
                "native-game",
                "compiler-manual-required-visible",
                Some("policy-decision-preview-game"),
            ),
        ],
    }
}

fn preview_row(
    row_id: AppGameText<'_>,
    target_domain: AppGameText<'_>,
    projection_state: AppGameText<'_>,
    preview_decision_ref: Option<AppGameText<'_>>,
) -> AppGameSourceGatedPolicyPreviewReadModelRowInput {
    AppGameSourceGatedPolicyPreviewReadModelRowInput {
        row_id: row_id.to_string(),
        target_domain: target_domain.to_string(),
        projection_state: projection_state.to_string(),
        source_evidence_refs: vec![format!("{row_id}-evidence")],
        preview_decision_ref: preview_decision_ref.map(|value| value.to_string()),
    }
}

fn timer_runtime_options() -> AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions {
    AppGameSourceGatedPolicyPreviewTimerRuntimeReadinessOptions {
        schema_version: "v0.6".to_string(),
        readiness_id: "source-gated-policy-preview-timer-runtime-readiness-proof".to_string(),
        generated_at: "2026-06-05T09:12:00Z".to_string(),
        source_contract_refs: vec!["app-game-source-gated-policy-preview-timer-status".to_string()],
        timer_runtime_proof_ref: "future-service-timer-runtime-proof".to_string(),
        scheduler_persistence_proof_ref: "future-scheduler-persistence-proof".to_string(),
        audit_proof_ref: "future-timer-audit-proof".to_string(),
        rollback_proof_ref: "future-timer-rollback-proof".to_string(),
    }
}

fn timer_status_input() -> AppGameSourceGatedPolicyPreviewTimerStatusInput {
    AppGameSourceGatedPolicyPreviewTimerStatusInput {
        status_id: "source-gated-policy-preview-timer-status-proof".to_string(),
        native_app_row_count: 1,
        native_game_row_count: 2,
        rows: vec![
            timer_status_row(
                "source-gate-row-ready-app:source-gated-preview-read-model:timer-handoff:timer-status",
                "native-app",
                "timer-runtime-proof-required",
                vec!["future-service-timer-runtime-proof"],
            ),
            timer_status_row(
                "source-gate-row-manual-game:source-gated-preview-read-model:timer-handoff:timer-status",
                "native-game",
                "source-freshness-proof-required",
                vec!["source-freshness-proof-required"],
            ),
            timer_status_row(
                "source-gate-row-compiler-manual-game:source-gated-preview-read-model:timer-handoff:timer-status",
                "native-game",
                "compiler-decision-proof-required",
                vec!["compiler-decision-proof-required"],
            ),
        ],
    }
}

fn timer_status_row(
    row_id: AppGameText<'_>,
    target_domain: AppGameText<'_>,
    timer_status_state: AppGameText<'_>,
    required_proof_refs: Vec<AppGameText<'_>>,
) -> AppGameSourceGatedPolicyPreviewTimerStatusRowInput {
    AppGameSourceGatedPolicyPreviewTimerStatusRowInput {
        row_id: row_id.to_string(),
        target_domain: target_domain.to_string(),
        timer_status_state: timer_status_state.to_string(),
        required_proof_refs: required_proof_refs
            .into_iter()
            .map(|value| value.to_string())
            .collect(),
        source_evidence_refs: vec![format!("{row_id}-evidence")],
    }
}

fn service_readiness_options() -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions
{
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessHandoffOptions {
        schema_version: "v0.6".to_string(),
        handoff_id: "source-gated-policy-preview-timer-service-readiness-handoff-proof".to_string(),
        generated_at: "2026-06-06T07:12:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-audit-rollback-parent-surface-intent"
                .to_string(),
        ],
        service_readiness_proof_ref: "future-service-readiness-proof".to_string(),
        service_read_api_proof_ref: "future-service-read-api-proof".to_string(),
        service_read_api_ref: "future-service-read-api-contract-ref".to_string(),
    }
}

fn parent_surface_intent_input(
) -> AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentInput {
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentInput {
        intent_id: "source-gated-policy-preview-timer-audit-rollback-parent-surface-intent-proof"
            .to_string(),
        native_app_row_count: 1,
        native_game_row_count: 2,
        rows: vec![
            parent_surface_row(
                "parent-surface-ready-app",
                "native-app",
                "audit-rollback-parent-surface-proof-required",
                true,
                vec!["future-parent-surface-proof"],
            ),
            parent_surface_row(
                "parent-surface-source-game",
                "native-game",
                "blocked-by-source-freshness",
                false,
                vec!["source-freshness-proof-required"],
            ),
            parent_surface_row(
                "parent-surface-compiler-game",
                "native-game",
                "blocked-by-compiler-decision",
                false,
                vec!["compiler-decision-proof-required"],
            ),
        ],
    }
}

fn parent_surface_row(
    row_id: AppGameText<'_>,
    target_domain: AppGameText<'_>,
    parent_surface_intent_state: AppGameText<'_>,
    parent_surface_proof_required: bool,
    required_proof_refs: Vec<AppGameText<'_>>,
) -> AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowInput {
    AppGameSourceGatedPolicyPreviewTimerAuditRollbackParentSurfaceIntentRowInput {
        row_id: row_id.to_string(),
        source_audit_rollback_read_model_row_id: format!("{row_id}-audit-read-model"),
        source_audit_rollback_handoff_row_id: format!("{row_id}-audit-handoff"),
        source_scheduler_persistence_row_id: format!("{row_id}-scheduler"),
        target_domain: target_domain.to_string(),
        parent_surface_intent_state: parent_surface_intent_state.to_string(),
        parent_surface_proof_required,
        required_proof_refs: required_proof_refs
            .into_iter()
            .map(|value| value.to_string())
            .collect(),
        source_evidence_refs: vec![format!("{row_id}-evidence")],
    }
}

fn protocol_handoff_options(
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions {
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions {
        schema_version: "v0.6".to_string(),
        handoff_id: "source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof"
            .to_string(),
        generated_at: "2026-06-06T07:23:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-read-model".to_string(),
            "packages/agent-protocol-domain".to_string(),
            "crates/agent-protocol".to_string(),
        ],
        protocol_command_contract_proof_ref: "future-agent-protocol-command-contract-proof"
            .to_string(),
        protocol_event_contract_proof_ref: "future-agent-protocol-event-contract-proof".to_string(),
        rust_protocol_mirror_proof_ref: "future-rust-protocol-mirror-proof".to_string(),
        service_handler_proof_ref: "future-service-handler-proof".to_string(),
    }
}

fn service_readiness_read_model_input(
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelInput {
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelInput {
        read_model_id: "source-gated-policy-preview-timer-service-readiness-read-model-proof"
            .to_string(),
        source_service_readiness_handoff_id:
            "source-gated-policy-preview-timer-service-readiness-handoff-proof".to_string(),
        native_app_row_count: 1,
        native_game_row_count: 2,
        rows: vec![
            service_readiness_read_model_row(
                "service-readiness-ready-app",
                "native-app",
                "service-read-model-proof-required",
                vec![
                    "future-service-readiness-proof",
                    "future-service-read-api-proof",
                ],
            ),
            service_readiness_read_model_row(
                "service-readiness-source-game",
                "native-game",
                "blocked-by-source-freshness",
                vec!["source-freshness-proof-required"],
            ),
            service_readiness_read_model_row(
                "service-readiness-compiler-game",
                "native-game",
                "blocked-by-compiler-decision",
                vec!["compiler-decision-proof-required"],
            ),
        ],
    }
}

fn service_readiness_read_model_row(
    row_id: AppGameText<'_>,
    target_domain: AppGameText<'_>,
    service_readiness_read_model_state: AppGameText<'_>,
    required_proof_refs: Vec<AppGameText<'_>>,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowInput {
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowInput {
        row_id: row_id.to_string(),
        target_domain: target_domain.to_string(),
        service_readiness_read_model_state: service_readiness_read_model_state.to_string(),
        required_proof_refs: required_proof_refs
            .into_iter()
            .map(|value| value.to_string())
            .collect(),
        source_evidence_refs: vec![format!("{row_id}-evidence")],
        service_read_api_ref: "future-service-read-api-contract-ref".to_string(),
    }
}
