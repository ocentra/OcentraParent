use ocentra_app_game_core::app_game_source_gated_policy_preview_timer_chain::{
    build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_handoff,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelInput,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowInput,
};
use ocentra_app_game_core::app_game_source_gated_policy_preview_timer_followthrough::{
    app_game_source_gated_policy_preview_timer_followthrough_typescript,
    build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff,
    build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model,
    build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_consumer_handoff,
    build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_handoff,
    build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_handoff,
    build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_read_model_handoff,
    build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_handoff,
    build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_handoff,
    build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_parent_surface_handoff,
    build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_parent_surface_read_model,
    build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff,
    build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff,
    build_app_game_source_gated_policy_preview_timer_service_readiness_service_read_api_handoff,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffOptions,
};

#[test]
fn timer_protocol_read_model_keeps_protocol_rows_and_counts() {
    let handoff = protocol_handoff();
    let read_model =
        build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
            &protocol_read_model_options(),
            &handoff,
        );

    assert_eq!(read_model.protocol_read_model_proof_required_count, 1);
    assert_eq!(read_model.blocked_by_source_freshness_count, 1);
    assert_eq!(read_model.blocked_by_compiler_decision_count, 1);
    assert_eq!(
        read_model.rows[0].required_protocol_proof_refs,
        vec![
            "future-agent-protocol-command-contract-proof".to_string(),
            "future-agent-protocol-event-contract-proof".to_string(),
            "future-rust-protocol-mirror-proof".to_string(),
            "future-service-handler-proof".to_string(),
        ]
    );
}

#[test]
fn timer_protocol_command_handoff_only_expands_future_refs_for_proof_required_rows() {
    let command_handoff =
        build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
            &protocol_command_options(),
            &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
                &protocol_read_model_options(),
                &protocol_handoff(),
            ),
        );

    assert_eq!(
        command_handoff.protocol_command_handoff_proof_required_count,
        1
    );
    assert_eq!(
        command_handoff.rows[0].required_agent_protocol_command_refs,
        vec!["agent.activity.app-game.timer-service-readiness.read-model.get".to_string()]
    );
    assert!(command_handoff.rows[1]
        .required_agent_protocol_command_refs
        .is_empty());
}

#[test]
fn timer_service_handler_handoff_preserves_protocol_refs_only_for_required_rows() {
    let service_handler =
        build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff(
            &service_handler_options(),
            &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
                &protocol_command_options(),
                &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
                    &protocol_read_model_options(),
                    &protocol_handoff(),
                ),
            ),
        );

    assert_eq!(service_handler.service_handler_proof_required_count, 1);
    assert_eq!(
        service_handler.rows[0].required_service_read_api_proof_refs,
        vec!["future-app-game-timer-service-readiness-read-api-proof".to_string()]
    );
    assert!(service_handler.rows[2]
        .required_service_read_api_proof_refs
        .is_empty());
}

#[test]
fn timer_service_read_api_handoff_preserves_handler_refs() {
    let read_api =
        build_app_game_source_gated_policy_preview_timer_service_readiness_service_read_api_handoff(
            &service_read_api_options(),
            &build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff(
                &service_handler_options(),
                &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
                    &protocol_command_options(),
                    &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
                        &protocol_read_model_options(),
                        &protocol_handoff(),
                    ),
                ),
            ),
        );

    assert_eq!(read_api.service_read_api_proof_required_count, 1);
    assert_eq!(
        read_api.rows[0].inherited_service_handler_refs,
        vec!["future-app-game-timer-service-readiness-command-handler-proof".to_string()]
    );
}

#[test]
fn timer_read_api_response_handoff_only_expands_response_proofs_for_required_rows() {
    let response_handoff =
        build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_handoff(
            &read_api_response_options(),
            &build_app_game_source_gated_policy_preview_timer_service_readiness_service_read_api_handoff(
                &service_read_api_options(),
                &build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff(
                    &service_handler_options(),
                    &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
                        &protocol_command_options(),
                        &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
                            &protocol_read_model_options(),
                            &protocol_handoff(),
                        ),
                    ),
                ),
            ),
        );

    assert_eq!(response_handoff.read_api_response_proof_required_count, 1);
    assert_eq!(
        response_handoff.rows[0].required_read_api_response_proof_refs,
        vec!["future-app-game-timer-service-readiness-read-api-response-proof".to_string()]
    );
    assert!(response_handoff.rows[1]
        .required_read_api_response_proof_refs
        .is_empty());
}

#[test]
fn timer_read_api_response_consumer_handoff_only_expands_consumer_proofs_for_required_rows() {
    let consumer_handoff = build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_consumer_handoff(
        &read_api_response_consumer_options(),
        &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_handoff(
            &read_api_response_options(),
            &build_app_game_source_gated_policy_preview_timer_service_readiness_service_read_api_handoff(
                &service_read_api_options(),
                &build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff(
                    &service_handler_options(),
                    &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
                        &protocol_command_options(),
                        &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
                            &protocol_read_model_options(),
                            &protocol_handoff(),
                        ),
                    ),
                ),
            ),
        ),
    );

    assert_eq!(
        consumer_handoff.read_api_response_consumer_proof_required_count,
        1
    );
    assert_eq!(
        consumer_handoff.rows[0].required_read_api_response_consumer_proof_refs,
        vec![
            "future-app-game-timer-service-readiness-read-api-response-consumer-proof".to_string()
        ]
    );
    assert!(consumer_handoff.rows[2]
        .required_read_api_response_consumer_proof_refs
        .is_empty());
}

#[test]
fn timer_parent_surface_handoff_only_expands_parent_surface_proofs_for_required_rows() {
    let parent_surface_handoff =
        build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_handoff(
            &parent_surface_handoff_options(),
            &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_consumer_handoff(
                &read_api_response_consumer_options(),
                &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_handoff(
                    &read_api_response_options(),
                    &build_app_game_source_gated_policy_preview_timer_service_readiness_service_read_api_handoff(
                        &service_read_api_options(),
                        &build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff(
                            &service_handler_options(),
                            &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
                                &protocol_command_options(),
                                &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
                                    &protocol_read_model_options(),
                                    &protocol_handoff(),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );

    assert_eq!(
        parent_surface_handoff.parent_surface_proof_required_count,
        1
    );
    assert_eq!(
        parent_surface_handoff.rows[0].required_parent_surface_proof_refs,
        vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-proof"
                .to_string()
        ]
    );
    assert!(parent_surface_handoff.rows[1]
        .required_parent_surface_proof_refs
        .is_empty());
}

#[test]
fn timer_parent_surface_read_model_handoff_only_expands_read_model_proofs_for_required_rows() {
    let read_model_handoff =
        build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_read_model_handoff(
            &parent_surface_read_model_handoff_options(),
            &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_handoff(
                &parent_surface_handoff_options(),
                &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_consumer_handoff(
                    &read_api_response_consumer_options(),
                    &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_handoff(
                        &read_api_response_options(),
                        &build_app_game_source_gated_policy_preview_timer_service_readiness_service_read_api_handoff(
                            &service_read_api_options(),
                            &build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff(
                                &service_handler_options(),
                                &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
                                    &protocol_command_options(),
                                    &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
                                        &protocol_read_model_options(),
                                        &protocol_handoff(),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );

    assert_eq!(
        read_model_handoff.parent_surface_read_model_proof_required_count,
        1
    );
    assert_eq!(
        read_model_handoff.rows[0].required_parent_surface_read_model_proof_refs,
        vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-read-model-proof"
                .to_string()
        ]
    );
    assert!(read_model_handoff.rows[2]
        .required_parent_surface_read_model_proof_refs
        .is_empty());
}

#[test]
fn timer_parent_surface_status_handoff_only_expands_status_proofs_for_required_rows() {
    let status_handoff =
        build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_handoff(
            &parent_surface_status_handoff_options(),
            &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_read_model_handoff(
                &parent_surface_read_model_handoff_options(),
                &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_handoff(
                    &parent_surface_handoff_options(),
                    &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_consumer_handoff(
                        &read_api_response_consumer_options(),
                        &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_handoff(
                            &read_api_response_options(),
                            &build_app_game_source_gated_policy_preview_timer_service_readiness_service_read_api_handoff(
                                &service_read_api_options(),
                                &build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff(
                                    &service_handler_options(),
                                    &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
                                        &protocol_command_options(),
                                        &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
                                            &protocol_read_model_options(),
                                            &protocol_handoff(),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );

    assert_eq!(status_handoff.parent_surface_status_proof_required_count, 1);
    assert_eq!(
        status_handoff.rows[0].required_parent_surface_status_proof_refs,
        vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-status-proof"
                .to_string()
        ]
    );
    assert!(status_handoff.rows[1]
        .required_parent_surface_status_proof_refs
        .is_empty());
}

#[test]
fn timer_parent_surface_status_read_model_handoff_only_expands_status_read_model_proofs_for_required_rows(
) {
    let status_read_model_handoff =
        build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_handoff(
            &parent_surface_status_read_model_handoff_options(),
            &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_handoff(
                &parent_surface_status_handoff_options(),
                &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_read_model_handoff(
                    &parent_surface_read_model_handoff_options(),
                    &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_handoff(
                        &parent_surface_handoff_options(),
                        &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_consumer_handoff(
                            &read_api_response_consumer_options(),
                            &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_handoff(
                                &read_api_response_options(),
                                &build_app_game_source_gated_policy_preview_timer_service_readiness_service_read_api_handoff(
                                    &service_read_api_options(),
                                    &build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff(
                                        &service_handler_options(),
                                        &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
                                            &protocol_command_options(),
                                            &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
                                                &protocol_read_model_options(),
                                                &protocol_handoff(),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );

    assert_eq!(
        status_read_model_handoff.parent_surface_status_read_model_proof_required_count,
        1
    );
    assert_eq!(
        status_read_model_handoff.rows[0].required_parent_surface_status_read_model_proof_refs,
        vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-proof"
                .to_string()
        ]
    );
    assert!(status_read_model_handoff.rows[2]
        .required_parent_surface_status_read_model_proof_refs
        .is_empty());
}

#[test]
fn timer_parent_surface_status_read_model_parent_surface_handoff_only_expands_parent_surface_proofs_for_required_rows(
) {
    let parent_surface_handoff =
        build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_parent_surface_handoff(
            &parent_surface_status_read_model_parent_surface_handoff_options(),
            &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_handoff(
                &parent_surface_status_read_model_handoff_options(),
                &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_handoff(
                    &parent_surface_status_handoff_options(),
                    &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_read_model_handoff(
                        &parent_surface_read_model_handoff_options(),
                        &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_handoff(
                            &parent_surface_handoff_options(),
                            &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_consumer_handoff(
                                &read_api_response_consumer_options(),
                                &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_handoff(
                                    &read_api_response_options(),
                                    &build_app_game_source_gated_policy_preview_timer_service_readiness_service_read_api_handoff(
                                        &service_read_api_options(),
                                        &build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff(
                                            &service_handler_options(),
                                            &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
                                                &protocol_command_options(),
                                                &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
                                                    &protocol_read_model_options(),
                                                    &protocol_handoff(),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );

    assert_eq!(
        parent_surface_handoff.parent_surface_proof_required_count,
        1
    );
    assert_eq!(
        parent_surface_handoff.rows[0].required_parent_surface_proof_refs,
        vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-proof"
                .to_string()
        ]
    );
    assert!(parent_surface_handoff.rows[1]
        .required_parent_surface_proof_refs
        .is_empty());
}

#[test]
fn timer_parent_surface_status_read_model_parent_surface_read_model_handoff_only_expands_read_model_proofs_for_required_rows(
) {
    let read_model_handoff =
        build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff(
            &parent_surface_status_read_model_parent_surface_read_model_handoff_options(),
            &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_parent_surface_handoff(
                &parent_surface_status_read_model_parent_surface_handoff_options(),
                &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_handoff(
                    &parent_surface_status_read_model_handoff_options(),
                    &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_handoff(
                        &parent_surface_status_handoff_options(),
                        &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_read_model_handoff(
                            &parent_surface_read_model_handoff_options(),
                            &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_handoff(
                                &parent_surface_handoff_options(),
                                &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_consumer_handoff(
                                    &read_api_response_consumer_options(),
                                    &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_handoff(
                                        &read_api_response_options(),
                                        &build_app_game_source_gated_policy_preview_timer_service_readiness_service_read_api_handoff(
                                            &service_read_api_options(),
                                            &build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff(
                                                &service_handler_options(),
                                                &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
                                                    &protocol_command_options(),
                                                    &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
                                                        &protocol_read_model_options(),
                                                        &protocol_handoff(),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );

    assert_eq!(
        read_model_handoff.parent_surface_read_model_proof_required_count,
        1
    );
    assert_eq!(
        read_model_handoff.rows[0].required_parent_surface_read_model_proof_refs,
        vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof"
                .to_string()
        ]
    );
    assert!(read_model_handoff.rows[2]
        .required_parent_surface_read_model_proof_refs
        .is_empty());
}

#[test]
fn timer_parent_surface_status_read_model_parent_surface_read_model_builds_ready_contract_rows() {
    let read_model =
        build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_parent_surface_read_model(
            &parent_surface_status_read_model_parent_surface_read_model_options(),
            &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff(
                &parent_surface_status_read_model_parent_surface_read_model_handoff_options(),
                &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_parent_surface_handoff(
                    &parent_surface_status_read_model_parent_surface_handoff_options(),
                    &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_read_model_handoff(
                        &parent_surface_status_read_model_handoff_options(),
                        &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_status_handoff(
                            &parent_surface_status_handoff_options(),
                            &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_read_model_handoff(
                                &parent_surface_read_model_handoff_options(),
                                &build_app_game_source_gated_policy_preview_timer_service_readiness_response_consumer_parent_surface_handoff(
                                    &parent_surface_handoff_options(),
                                    &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_consumer_handoff(
                                        &read_api_response_consumer_options(),
                                        &build_app_game_source_gated_policy_preview_timer_service_readiness_read_api_response_handoff(
                                            &read_api_response_options(),
                                            &build_app_game_source_gated_policy_preview_timer_service_readiness_service_read_api_handoff(
                                                &service_read_api_options(),
                                                &build_app_game_source_gated_policy_preview_timer_service_readiness_service_handler_handoff(
                                                    &service_handler_options(),
                                                    &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_command_handoff(
                                                        &protocol_command_options(),
                                                        &build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_read_model(
                                                            &protocol_read_model_options(),
                                                            &protocol_handoff(),
                                                        ),
                                                    ),
                                                ),
                                            ),
                                        ),
                                    ),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        );

    assert_eq!(read_model.ready_for_parent_surface_read_model_count, 1);
    assert_eq!(
        read_model.rows[0].parent_safe_summary,
        "native-app:ready-for-parent-surface-status-read-model-parent-surface-read-model-contract"
            .to_string()
    );
    assert_eq!(
        read_model.rows[0].required_parent_surface_read_model_proof_refs,
        vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof"
                .to_string()
        ]
    );
    assert!(read_model.rows[1]
        .required_parent_surface_read_model_proof_refs
        .is_empty());
}

#[test]
fn generated_timer_followthrough_helper_stays_checked_in() {
    let checked_in = include_str!(
        "../../src/generated/app-game-source-gated-policy-preview-timer-followthrough.ts"
    );

    assert_eq!(
        checked_in,
        app_game_source_gated_policy_preview_timer_followthrough_typescript()
    );
    assert!(checked_in.contains(
        "buildGeneratedAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel"
    ));
    assert!(checked_in.contains(
        "buildGeneratedAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff"
    ));
    assert!(checked_in.contains(
        "buildGeneratedAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff"
    ));
    assert!(checked_in.contains(
        "buildGeneratedAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel"
    ));
}

fn protocol_handoff(
) -> ocentra_app_game_core::app_game_source_gated_policy_preview_timer_chain::AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff
{
    build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_handoff(
        &protocol_handoff_options(),
        &service_readiness_read_model_input(),
    )
}

fn protocol_handoff_options(
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions {
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions {
        schema_version: "v0.6".to_string(),
        handoff_id: "source-gated-policy-preview-timer-service-readiness-protocol-handoff-proof"
            .to_string(),
        generated_at: "2026-06-06T07:12:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-read-model".to_string(),
            "docs/expectations/app-game-evidence.md".to_string(),
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
            service_readiness_row(
                "service-readiness-row-app",
                "native-app",
                "service-read-model-proof-required",
                vec!["future-service-readiness-proof"],
                "future-service-read-api-contract-ref",
            ),
            service_readiness_row(
                "service-readiness-row-source",
                "native-game",
                "blocked-by-source-freshness",
                vec!["source-freshness-proof-required"],
                "future-service-read-api-contract-ref",
            ),
            service_readiness_row(
                "service-readiness-row-compiler",
                "native-game",
                "blocked-by-compiler-decision",
                vec!["compiler-decision-proof-required"],
                "future-service-read-api-contract-ref",
            ),
        ],
    }
}

fn service_readiness_row(
    row_id: &str,
    target_domain: &str,
    state: &str,
    required_proof_refs: Vec<&str>,
    service_read_api_ref: &str,
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowInput {
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowInput {
        row_id: row_id.to_string(),
        target_domain: target_domain.to_string(),
        service_readiness_read_model_state: state.to_string(),
        required_proof_refs: required_proof_refs
            .into_iter()
            .map(|value| value.to_string())
            .collect(),
        source_evidence_refs: vec![format!("{row_id}-evidence")],
        service_read_api_ref: service_read_api_ref.to_string(),
    }
}

fn protocol_read_model_options(
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelOptions {
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelOptions {
        schema_version: "v0.6".to_string(),
        read_model_id:
            "source-gated-policy-preview-timer-service-readiness-protocol-read-model-proof"
                .to_string(),
        generated_at: "2026-06-06T07:23:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-protocol-handoff"
                .to_string(),
        ],
        protocol_summary_ref: "future-service-readiness-protocol-read-model-summary-proof"
            .to_string(),
    }
}

fn protocol_command_options(
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffOptions {
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffOptions {
        schema_version: "v0.6".to_string(),
        command_handoff_id:
            "source-gated-policy-preview-timer-service-readiness-protocol-command-handoff-proof"
                .to_string(),
        generated_at: "2026-06-06T07:45:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-protocol-read-model"
                .to_string(),
        ],
        protocol_command_refs: vec![
            "agent.activity.app-game.timer-service-readiness.read-model.get".to_string(),
        ],
        protocol_event_refs: vec![
            "agent.activity.app-game.timer-service-readiness.read-model.reported".to_string(),
        ],
        service_handler_refs: vec![
            "future-app-game-timer-service-readiness-command-handler-proof".to_string(),
        ],
        command_summary_ref: "future-service-readiness-protocol-command-handoff-summary-proof"
            .to_string(),
    }
}

fn service_handler_options(
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffOptions {
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffOptions {
        schema_version: "v0.6".to_string(),
        service_handler_handoff_id:
            "source-gated-policy-preview-timer-service-readiness-service-handler-handoff-proof"
                .to_string(),
        generated_at: "2026-06-06T07:58:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-protocol-command-handoff"
                .to_string(),
        ],
        service_read_api_proof_refs: vec![
            "future-app-game-timer-service-readiness-read-api-proof".to_string()
        ],
        service_handler_summary_ref:
            "future-service-readiness-service-handler-handoff-summary-proof".to_string(),
    }
}

fn service_read_api_options(
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffOptions {
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffOptions {
        schema_version: "v0.6".to_string(),
        service_read_api_handoff_id:
            "source-gated-policy-preview-timer-service-readiness-read-api-handoff-proof".to_string(),
        generated_at: "2026-06-06T08:10:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-service-handler-handoff"
                .to_string(),
        ],
        service_read_api_proof_refs: vec![
            "future-app-game-timer-service-readiness-read-api-proof".to_string()
        ],
        service_read_api_summary_ref: "future-service-readiness-read-api-handoff-summary-proof"
            .to_string(),
    }
}

fn read_api_response_options(
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffOptions {
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffOptions {
        schema_version: "v0.6".to_string(),
        read_api_response_handoff_id:
            "source-gated-policy-preview-timer-service-readiness-read-api-response-handoff-proof"
                .to_string(),
        generated_at: "2026-06-06T08:36:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-read-api-handoff"
                .to_string(),
        ],
        read_api_response_proof_refs: vec![
            "future-app-game-timer-service-readiness-read-api-response-proof".to_string(),
        ],
        read_api_response_summary_ref:
            "future-service-readiness-read-api-response-handoff-summary-proof".to_string(),
    }
}

fn read_api_response_consumer_options(
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffOptions {
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffOptions {
        schema_version: "v0.6".to_string(),
        read_api_response_consumer_handoff_id:
            "source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff-proof".to_string(),
        generated_at: "2026-06-06T08:56:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-handoff".to_string(),
        ],
        read_api_response_consumer_proof_refs: vec![
            "future-app-game-timer-service-readiness-read-api-response-consumer-proof".to_string(),
        ],
        read_api_response_consumer_summary_ref:
            "future-service-readiness-read-api-response-consumer-handoff-summary-proof".to_string(),
    }
}

fn parent_surface_handoff_options(
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffOptions
{
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffOptions {
        schema_version: "v0.6".to_string(),
        response_consumer_parent_surface_handoff_id:
            "source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff-proof"
                .to_string(),
        generated_at: "2026-06-06T09:10:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-read-api-response-consumer-handoff"
                .to_string(),
        ],
        parent_surface_proof_refs: vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-proof".to_string(),
        ],
        parent_surface_summary_ref:
            "future-service-readiness-response-consumer-parent-surface-summary-proof".to_string(),
    }
}

fn parent_surface_read_model_handoff_options()
-> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffOptions{
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffOptions {
        schema_version: "v0.6".to_string(),
        response_consumer_parent_surface_read_model_handoff_id:
            "source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff-proof"
                .to_string(),
        generated_at: "2026-06-06T09:18:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-handoff"
                .to_string(),
        ],
        parent_surface_read_model_proof_refs: vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-read-model-proof"
                .to_string(),
        ],
        parent_surface_read_model_ref:
            "future-service-readiness-response-consumer-parent-surface-read-model-proof".to_string(),
    }
}

fn parent_surface_status_handoff_options()
-> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffOptions{
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffOptions {
        schema_version: "v0.6".to_string(),
        response_consumer_parent_surface_status_handoff_id:
            "source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff-proof"
                .to_string(),
        generated_at: "2026-06-06T09:26:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-read-model-handoff"
                .to_string(),
        ],
        parent_surface_status_proof_refs: vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-status-proof".to_string(),
        ],
        parent_surface_status_ref:
            "future-service-readiness-response-consumer-parent-surface-status-proof".to_string(),
    }
}

fn parent_surface_status_read_model_handoff_options()
-> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffOptions{
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffOptions {
        schema_version: "v0.6".to_string(),
        response_consumer_parent_surface_status_read_model_handoff_id:
            "source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff-proof"
                .to_string(),
        generated_at: "2026-06-06T09:34:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-handoff"
                .to_string(),
        ],
        parent_surface_status_read_model_proof_refs: vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-proof"
                .to_string(),
        ],
        parent_surface_status_read_model_ref:
            "future-service-readiness-response-consumer-parent-surface-status-read-model-proof".to_string(),
    }
}

fn parent_surface_status_read_model_parent_surface_handoff_options()
-> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffOptions{
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffOptions {
        schema_version: "v0.6".to_string(),
        response_consumer_parent_surface_status_read_model_parent_surface_handoff_id:
            "source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff-proof"
                .to_string(),
        generated_at: "2026-06-06T10:45:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-handoff"
                .to_string(),
            "docs/expectations/app-game-evidence.md".to_string(),
            "docs/expectations/enforcement.md".to_string(),
        ],
        parent_surface_proof_refs: vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-proof"
                .to_string(),
        ],
        parent_surface_ref:
            "future-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-proof"
                .to_string(),
    }
}

fn parent_surface_status_read_model_parent_surface_read_model_handoff_options()
-> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffOptions{
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffOptions {
        schema_version: "v0.6".to_string(),
        response_consumer_parent_surface_status_read_model_parent_surface_read_model_handoff_id:
            "source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff-proof"
                .to_string(),
        generated_at: "2026-06-06T11:05:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-handoff"
                .to_string(),
            "docs/expectations/app-game-evidence.md".to_string(),
            "docs/expectations/enforcement.md".to_string(),
        ],
        parent_surface_read_model_proof_refs: vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof"
                .to_string(),
        ],
        parent_surface_read_model_ref:
            "future-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof"
                .to_string(),
    }
}

fn parent_surface_status_read_model_parent_surface_read_model_options()
-> AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptions{
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptions {
        schema_version: "v0.6".to_string(),
        parent_surface_read_model_id:
            "source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-proof"
                .to_string(),
        generated_at: "2026-06-06T11:30:00Z".to_string(),
        source_contract_refs: vec![
            "app-game-source-gated-policy-preview-timer-service-readiness-response-consumer-parent-surface-status-read-model-parent-surface-read-model-handoff"
                .to_string(),
            "docs/expectations/app-game-evidence.md".to_string(),
            "docs/expectations/enforcement.md".to_string(),
        ],
    }
}
