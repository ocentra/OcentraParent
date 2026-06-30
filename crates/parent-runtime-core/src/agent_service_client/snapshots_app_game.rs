use super::*;
use crate::agent_service_client::payload_fields::serialized_enum_label;
use crate::agent_service_client::snapshots_network::app_game_read_model_from_response;
use crate::agent_service_client::transport::rejection_message;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_preflight::AppGameAdapterDispatchPreflightReadModel;
use ocentra_parent_agent_protocol::app_game_adapter_dispatch_result::AppGameAdapterDispatchResultReadModel;
use ocentra_parent_agent_protocol::app_game_child_runtime_transport_receipt::AppGameChildRuntimeTransportReceiptReadModel;
use ocentra_parent_agent_protocol::app_game_notification_readiness::AppGameNotificationReadinessReadModel;
use ocentra_parent_agent_protocol::app_game_platform_proof_status::AppGamePlatformProofStatusReadModel;
use ocentra_parent_agent_protocol::app_game_policy_readiness::AppGamePolicyReadinessReadModel;
use ocentra_parent_agent_protocol::app_game_timer_parent_surface_read_model::AppGameTimerParentSurfaceReadModel;

pub(super) fn app_game_notification_readiness_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<AppGameNotificationReadinessAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    if response_event.event
        != AgentEventName::AgentActivityAppGameNotificationReadinessReadModelReported
    {
        return Err(format!(
            "agent-service expected {}, received {}",
            serialized_enum_label(
                &AgentEventName::AgentActivityAppGameNotificationReadinessReadModelReported
            ),
            serialized_enum_label(&response_event.event)
        ));
    }

    events.last().cloned().ok_or_else(|| {
        "agent-service app/game notification readiness result did not include a response event"
            .to_string()
    })?;
    let read_model = app_game_read_model_from_response::<AppGameNotificationReadinessReadModel>(
        &response_event,
        constants::field::APP_GAME_NOTIFICATION_READINESS_READ_MODEL,
        "notification readiness",
    )?;
    Ok(AppGameNotificationReadinessAgentServiceSnapshot { read_model })
}

pub(super) fn app_game_policy_readiness_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<AppGamePolicyReadinessAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    if response_event.event != AgentEventName::AgentActivityAppGamePolicyReadinessReadModelReported
    {
        return Err(format!(
            "agent-service expected {}, received {}",
            serialized_enum_label(
                &AgentEventName::AgentActivityAppGamePolicyReadinessReadModelReported
            ),
            serialized_enum_label(&response_event.event)
        ));
    }

    events.last().cloned().ok_or_else(|| {
        "agent-service app/game policy readiness result did not include a response event"
            .to_string()
    })?;
    let read_model = app_game_read_model_from_response::<AppGamePolicyReadinessReadModel>(
        &response_event,
        constants::field::APP_GAME_POLICY_READINESS_READ_MODEL,
        "policy readiness",
    )?;
    Ok(AppGamePolicyReadinessAgentServiceSnapshot { read_model })
}

pub(super) fn app_game_platform_proof_status_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<AppGamePlatformProofStatusAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    if response_event.event
        != AgentEventName::AgentActivityAppGamePlatformProofStatusReadModelReported
    {
        return Err(format!(
            "agent-service expected {}, received {}",
            serialized_enum_label(
                &AgentEventName::AgentActivityAppGamePlatformProofStatusReadModelReported
            ),
            serialized_enum_label(&response_event.event)
        ));
    }

    events.last().cloned().ok_or_else(|| {
        "agent-service app/game platform proof status result did not include a response event"
            .to_string()
    })?;
    let read_model = app_game_read_model_from_response::<AppGamePlatformProofStatusReadModel>(
        &response_event,
        constants::field::APP_GAME_PLATFORM_PROOF_STATUS_READ_MODEL,
        "platform proof status",
    )?;
    Ok(AppGamePlatformProofStatusAgentServiceSnapshot { read_model })
}

pub(super) fn app_game_child_runtime_transport_receipt_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<AppGameChildRuntimeTransportReceiptAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    if response_event.event
        != AgentEventName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelReported
    {
        return Err(format!(
            "agent-service expected {}, received {}",
            serialized_enum_label(
                &AgentEventName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelReported
            ),
            serialized_enum_label(&response_event.event)
        ));
    }

    events.last().cloned().ok_or_else(|| {
        "agent-service app/game child runtime transport receipt result did not include a response event"
            .to_string()
    })?;
    let read_model =
        app_game_read_model_from_response::<AppGameChildRuntimeTransportReceiptReadModel>(
            &response_event,
            constants::field::APP_GAME_CHILD_RUNTIME_TRANSPORT_RECEIPT_READ_MODEL,
            "child runtime transport receipt",
        )?;
    Ok(AppGameChildRuntimeTransportReceiptAgentServiceSnapshot { read_model })
}

pub(super) fn app_game_adapter_dispatch_preflight_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<AppGameAdapterDispatchPreflightAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    if response_event.event
        != AgentEventName::AgentActivityAppGameAdapterDispatchPreflightReadModelReported
    {
        return Err(format!(
            "agent-service expected {}, received {}",
            serialized_enum_label(
                &AgentEventName::AgentActivityAppGameAdapterDispatchPreflightReadModelReported
            ),
            serialized_enum_label(&response_event.event)
        ));
    }

    events.last().cloned().ok_or_else(|| {
        "agent-service app/game adapter dispatch preflight result did not include a response event"
            .to_string()
    })?;
    let read_model = app_game_read_model_from_response::<AppGameAdapterDispatchPreflightReadModel>(
        &response_event,
        constants::field::APP_GAME_ADAPTER_DISPATCH_PREFLIGHT_READ_MODEL,
        "adapter dispatch preflight",
    )?;
    Ok(AppGameAdapterDispatchPreflightAgentServiceSnapshot { read_model })
}

pub(super) fn app_game_adapter_dispatch_result_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<AppGameAdapterDispatchResultAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    if response_event.event
        != AgentEventName::AgentActivityAppGameAdapterDispatchResultReadModelReported
    {
        return Err(format!(
            "agent-service expected {}, received {}",
            serialized_enum_label(
                &AgentEventName::AgentActivityAppGameAdapterDispatchResultReadModelReported
            ),
            serialized_enum_label(&response_event.event)
        ));
    }

    events.last().cloned().ok_or_else(|| {
        "agent-service app/game adapter dispatch result did not include a response event"
            .to_string()
    })?;
    let read_model = app_game_read_model_from_response::<AppGameAdapterDispatchResultReadModel>(
        &response_event,
        constants::field::APP_GAME_ADAPTER_DISPATCH_RESULT_READ_MODEL,
        "adapter dispatch result",
    )?;
    Ok(AppGameAdapterDispatchResultAgentServiceSnapshot { read_model })
}

pub(super) fn app_game_timer_parent_surface_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<AppGameTimerParentSurfaceAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    if response_event.event
        != AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported
    {
        return Err(format!(
            "agent-service expected {}, received {}",
            serialized_enum_label(
                &AgentEventName::AgentActivityAppGameTimerParentSurfaceReadModelReported
            ),
            serialized_enum_label(&response_event.event)
        ));
    }

    events.last().cloned().ok_or_else(|| {
        "agent-service app/game timer parent surface result did not include a response event"
            .to_string()
    })?;
    let read_model = app_game_read_model_from_response::<AppGameTimerParentSurfaceReadModel>(
        &response_event,
        constants::field::APP_GAME_TIMER_PARENT_SURFACE_READ_MODEL,
        "timer parent surface",
    )?;
    Ok(AppGameTimerParentSurfaceAgentServiceSnapshot { read_model })
}
