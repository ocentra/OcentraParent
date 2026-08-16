use super::*;
use crate::agent_service_client::payload_fields::{log_field_string, serialized_enum_label};
use crate::agent_service_client::snapshots_network::app_game_read_model_from_response;
use crate::agent_service_client::transport::rejection_message;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::AppGameAdapterDispatchPreflightReadModel;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::AppGameAdapterDispatchResultReadModel;
use ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::AppGameChildRuntimeTransportReceiptReadModel;
use ocentra_parent_agent_protocol::app_game_notification_readiness::AppGameNotificationReadinessReadModel;
use ocentra_parent_agent_protocol::app_game_notification_status::AppGameNotificationStatusReadModels;
use ocentra_parent_agent_protocol::app_game_platform_proof_status::AppGamePlatformProofStatusReadModel;
use ocentra_parent_agent_protocol::app_game_policy_readiness::AppGamePolicyReadinessReadModel;
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::AppGameTimerParentSurfaceReadModel;

fn validate_app_game_response_event<'a>(
    result: &'a AgentServiceCommandResult,
    expected_event: &AgentEventName,
    result_label: &str,
) -> Result<&'a AgentEventEnvelope, String> {
    if result.response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&result.response_event));
    }
    if result.response_event.event != *expected_event {
        return Err(format!(
            "agent-service expected {}, received {}",
            serialized_enum_label(&expected_event),
            serialized_enum_label(&result.response_event.event)
        ));
    }
    let _ = result_label;
    Ok(&result.response_event)
}

pub(super) fn app_game_notification_readiness_snapshot_from_result(
    result: &AgentServiceCommandResult,
) -> Result<AppGameNotificationReadinessAgentServiceSnapshot, String> {
    let response_event = validate_app_game_response_event(
        result,
        &AgentEventName::AgentActivityAppGameNotificationReadinessReadModelReported,
        "notification readiness",
    )?;
    let read_model = app_game_read_model_from_response::<AppGameNotificationReadinessReadModel>(
        response_event,
        constants::field::APP_GAME_NOTIFICATION_READINESS_READ_MODEL,
        "notification readiness",
    )?;
    let status_read_models = response_event
        .payload
        .get(constants::field::APP_GAME_NOTIFICATION_STATUS_READ_MODELS)
        .and_then(log_field_string)
        .and_then(|value| serde_json::from_str::<AppGameNotificationStatusReadModels>(value).ok());
    Ok(AppGameNotificationReadinessAgentServiceSnapshot {
        read_model,
        status_read_models,
    })
}

pub(super) fn app_game_policy_readiness_snapshot_from_result(
    result: &AgentServiceCommandResult,
) -> Result<AppGamePolicyReadinessAgentServiceSnapshot, String> {
    let response_event = validate_app_game_response_event(
        result,
        &AgentEventName::AgentActivityAppGamePolicyReadinessReadModelReported,
        "policy readiness",
    )?;
    let read_model = app_game_read_model_from_response::<AppGamePolicyReadinessReadModel>(
        response_event,
        constants::field::APP_GAME_POLICY_READINESS_READ_MODEL,
        "policy readiness",
    )?;
    Ok(AppGamePolicyReadinessAgentServiceSnapshot { read_model })
}

pub(super) fn app_game_platform_proof_status_snapshot_from_result(
    result: &AgentServiceCommandResult,
) -> Result<AppGamePlatformProofStatusAgentServiceSnapshot, String> {
    let response_event = validate_app_game_response_event(
        result,
        &AgentEventName::AgentActivityAppGamePlatformProofStatusReadModelReported,
        "platform proof status",
    )?;
    let read_model = app_game_read_model_from_response::<AppGamePlatformProofStatusReadModel>(
        response_event,
        constants::field::APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL,
        "platform proof status",
    )?;
    Ok(AppGamePlatformProofStatusAgentServiceSnapshot { read_model })
}

pub(super) fn app_game_child_runtime_transport_receipt_snapshot_from_result(
    result: &AgentServiceCommandResult,
) -> Result<AppGameChildRuntimeTransportReceiptAgentServiceSnapshot, String> {
    let response_event = validate_app_game_response_event(
        result,
        &AgentEventName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelReported,
        "child runtime transport receipt",
    )?;
    let read_model =
        app_game_read_model_from_response::<AppGameChildRuntimeTransportReceiptReadModel>(
            response_event,
            constants::field::APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL,
            "child runtime transport receipt",
        )?;
    Ok(AppGameChildRuntimeTransportReceiptAgentServiceSnapshot { read_model })
}

pub(super) fn app_game_adapter_dispatch_preflight_snapshot_from_result(
    result: &AgentServiceCommandResult,
) -> Result<AppGameAdapterDispatchPreflightAgentServiceSnapshot, String> {
    let response_event = validate_app_game_response_event(
        result,
        &AgentEventName::AgentActivityAppGameAdapterDispatchPreflightReadModelReported,
        "adapter dispatch preflight",
    )?;
    let read_model = app_game_read_model_from_response::<AppGameAdapterDispatchPreflightReadModel>(
        response_event,
        constants::field::APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL,
        "adapter dispatch preflight",
    )?;
    Ok(AppGameAdapterDispatchPreflightAgentServiceSnapshot { read_model })
}

pub(super) fn app_game_adapter_dispatch_result_snapshot_from_result(
    result: &AgentServiceCommandResult,
) -> Result<AppGameAdapterDispatchResultAgentServiceSnapshot, String> {
    let response_event = validate_app_game_response_event(
        result,
        &AgentEventName::AgentActivityAppGameAdapterDispatchResultReadModelReported,
        "adapter dispatch result",
    )?;
    let read_model = app_game_read_model_from_response::<AppGameAdapterDispatchResultReadModel>(
        response_event,
        constants::field::APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL,
        "adapter dispatch result",
    )?;
    Ok(AppGameAdapterDispatchResultAgentServiceSnapshot { read_model })
}

pub(super) fn app_game_timer_parent_surface_snapshot_from_result(
    result: &AgentServiceCommandResult,
) -> Result<AppGameTimerParentSurfaceAgentServiceSnapshot, String> {
    let response_event = validate_app_game_response_event(
        result,
        &AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported,
        "timer parent surface",
    )?;
    let read_model = app_game_read_model_from_response::<AppGameTimerParentSurfaceReadModel>(
        response_event,
        constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL,
        "timer parent surface",
    )?;
    Ok(AppGameTimerParentSurfaceAgentServiceSnapshot { read_model })
}
