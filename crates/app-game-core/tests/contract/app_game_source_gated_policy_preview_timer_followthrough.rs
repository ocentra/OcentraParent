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
};

#[path = "app_game_source_gated_policy_preview_timer_followthrough_support.rs"]
mod app_game_source_gated_policy_preview_timer_followthrough_support;

use self::app_game_source_gated_policy_preview_timer_followthrough_support::{
    parent_surface_handoff_options, parent_surface_read_model_handoff_options,
    parent_surface_status_handoff_options, parent_surface_status_read_model_handoff_options,
    parent_surface_status_read_model_parent_surface_handoff_options,
    parent_surface_status_read_model_parent_surface_read_model_handoff_options,
    parent_surface_status_read_model_parent_surface_read_model_options, protocol_command_options,
    protocol_handoff, protocol_read_model_options, read_api_response_consumer_options,
    read_api_response_options, service_handler_options, service_read_api_options,
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
    assert!(read_model.protocol_read_model_non_claims.iter().any(|claim| {
        claim == "no-agent-protocol-contract-implemented"
    }));
    assert!(read_model.protocol_read_model_non_claims.iter().any(|claim| {
        claim == "no-service-read-api-implemented"
    }));
    assert!(!read_model.agent_protocol_contract_implemented);
    assert!(!read_model.rust_protocol_mirrored);
    assert!(!read_model.service_command_registered);
    assert!(!read_model.service_event_emitted);
    assert!(!read_model.service_read_api_implemented);
    assert!(!read_model.timer_runtime_claimed);
    assert!(!read_model.timer_scheduled);
    assert!(!read_model.adapter_dispatch_claimed);
    assert!(!read_model.raw_private_source_rows_included);
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
    assert_eq!(
        parent_surface_handoff.rows[0]
            .source_response_consumer_parent_surface_status_read_model_handoff_row_id,
        "service-readiness-row-app:protocol-handoff:protocol-read-model:protocol-command-handoff:service-handler-handoff:read-api-handoff:read-api-response-handoff:read-api-response-consumer-handoff:response-consumer-parent-surface-handoff:parent-surface-read-model-handoff:parent-surface-status-handoff:parent-surface-status-read-model-handoff"
    );
    assert_eq!(
        parent_surface_handoff.rows[0].inherited_parent_surface_status_read_model_proof_refs,
        vec![
            "future-app-game-timer-service-readiness-response-consumer-parent-surface-status-read-model-proof"
                .to_string()
        ]
    );
    assert_eq!(
        parent_surface_handoff.rows[0].parent_surface_status_read_model_ref,
            "future-service-readiness-response-consumer-parent-surface-status-read-model-proof"
    );
    assert!(!parent_surface_handoff.rows[0].parent_surface_status_implemented);
    assert!(!parent_surface_handoff.rows[0].parent_surface_status_read_model_implemented);
    assert!(!parent_surface_handoff.rows[0]
        .parent_surface_status_read_model_parent_surface_implemented);
    assert!(!parent_surface_handoff.rows[0].parent_surface_rendered);
    assert!(!parent_surface_handoff.rows[0].raw_private_source_rows_included);
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
    let checked_in =
        include_str!("../generated/app-game-source-gated-policy-preview-timer-followthrough.ts");

    assert_eq!(
        checked_in,
        app_game_source_gated_policy_preview_timer_followthrough_typescript()
    );
    assert_eq!(
        checked_in
            .matches(
                "buildGeneratedAppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModel"
            )
            .count(),
        1
    );
    assert_eq!(
        checked_in
            .matches(
                "buildGeneratedAppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoff"
            )
            .count(),
        1
    );
    assert_eq!(
        checked_in
            .matches(
                "buildGeneratedAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoff"
            )
            .count(),
        1
    );
    assert_eq!(
        checked_in
            .matches(
                "buildGeneratedAppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModel"
            )
            .count(),
        2
    );
}
