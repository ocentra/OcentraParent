use ocentra_app_game_core::app_game_source_gated_policy_preview_timer_chain::{
    build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_handoff,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelInput,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadModelRowInput,
};
use ocentra_app_game_core::app_game_source_gated_policy_preview_timer_followthrough::{
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolCommandHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolReadModelOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseConsumerHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessReadApiResponseHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceHandlerHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessServiceReadApiHandoffOptions,
};
use ocentra_app_game_core::app_game_source_gated_policy_preview_timer_followthrough::parent_surface::{
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceReadModelHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelHandoffOptions,
};
use ocentra_app_game_core::app_game_source_gated_policy_preview_timer_followthrough::parent_surface_status::{
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelHandoffOptions,
    AppGameSourceGatedPolicyPreviewTimerServiceReadinessResponseConsumerParentSurfaceStatusReadModelParentSurfaceReadModelOptions,
};

type AppGameText<'a> = &'a str;

pub(crate) fn protocol_handoff(
) -> AppGameSourceGatedPolicyPreviewTimerServiceReadinessProtocolHandoff {
    build_app_game_source_gated_policy_preview_timer_service_readiness_protocol_handoff(
        &protocol_handoff_options(),
        &service_readiness_read_model_input(),
    )
}

pub(crate) fn protocol_read_model_options(
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

pub(crate) fn protocol_command_options(
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

pub(crate) fn service_handler_options(
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

pub(crate) fn service_read_api_options(
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

pub(crate) fn read_api_response_options(
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

pub(crate) fn read_api_response_consumer_options(
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

pub(crate) fn parent_surface_handoff_options(
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

pub(crate) fn parent_surface_read_model_handoff_options()
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

pub(crate) fn parent_surface_status_handoff_options()
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

pub(crate) fn parent_surface_status_read_model_handoff_options()
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

pub(crate) fn parent_surface_status_read_model_parent_surface_handoff_options()
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

pub(crate) fn parent_surface_status_read_model_parent_surface_read_model_handoff_options()
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

pub(crate) fn parent_surface_status_read_model_parent_surface_read_model_options()
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
    row_id: AppGameText<'_>,
    target_domain: AppGameText<'_>,
    state: AppGameText<'_>,
    required_proof_refs: Vec<AppGameText<'_>>,
    service_read_api_ref: AppGameText<'_>,
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
