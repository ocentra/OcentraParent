use super::snapshots_lan_replay::lan_runtime_replay_events_from_payload;
use super::types::LanRuntimeReplaySnapshot;
use super::*;

pub(super) fn lan_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<LanAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
        ..
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }

    let read_model_json = response_event
        .payload
        .get(constants::field::LAN_ADD_DEVICE_READ_MODEL)
        .and_then(log_field_string)
        .ok_or_else(|| {
            format!(
                "agent-service {} did not include {}",
                serialized_enum_label(&response_event.event),
                constants::field::LAN_ADD_DEVICE_READ_MODEL
            )
        })?;
    let read_model = serde_json::from_str::<LanBrowserAddDeviceReadModel>(read_model_json)
        .map_err(|error| format!("agent-service LAN read model parse failed: {error}"))?;
    let event = events.last().cloned().ok_or_else(|| {
        "agent-service command result did not include a response event".to_string()
    })?;

    Ok(LanAgentServiceSnapshot {
        event,
        events,
        read_model,
    })
}

pub(super) fn lan_runtime_replay_events_from_result(
    result: AgentServiceCommandResult,
) -> Result<LanRuntimeReplaySnapshot, String> {
    let AgentServiceCommandResult {
        events: _,
        command,
        command_message_id,
        response_event,
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    lan_runtime_replay_events_from_payload(&response_event, &command, &command_message_id)
}

pub(crate) fn network_flow_snapshot_from_parts(
    response_event: &AgentEventEnvelope,
    events: &[ParentRouteEventSnapshot],
) -> Result<NetworkFlowAgentServiceSnapshot, String> {
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(response_event));
    }
    if response_event.event != AgentEventName::AgentNetworkFlowReadModelReported {
        return Err(format!(
            "agent-service expected {}, received {}",
            serialized_enum_label(&AgentEventName::AgentNetworkFlowReadModelReported),
            serialized_enum_label(&response_event.event)
        ));
    }

    let event = events.last().cloned().ok_or_else(|| {
        "agent-service network flow result did not include a response event".to_string()
    })?;
    let read_model = network_flow_read_model_from_payload(&response_event.payload)?;

    Ok(NetworkFlowAgentServiceSnapshot {
        event,
        events: events.to_vec(),
        read_model,
    })
}

pub(super) fn network_flow_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<NetworkFlowAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
        ..
    } = result;
    network_flow_snapshot_from_parts(&response_event, &events)
}

pub(super) fn network_runtime_event_chain_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<NetworkRuntimeEventChainAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events: _,
        response_event,
        ..
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    if response_event.event != AgentEventName::AgentNetworkRuntimeEventChainStreamReported {
        return Err(format!(
            "agent-service expected {}, received {}",
            serialized_enum_label(&AgentEventName::AgentNetworkRuntimeEventChainStreamReported),
            serialized_enum_label(&response_event.event)
        ));
    }

    Ok(NetworkRuntimeEventChainAgentServiceSnapshot {
        stream: network_runtime_event_chain_stream_from_payload(&response_event.payload),
    })
}

pub(super) fn policy_preview_snapshot_from_result(
    result: AgentServiceCommandResult,
) -> Result<PolicyPreviewAgentServiceSnapshot, String> {
    let AgentServiceCommandResult {
        events,
        response_event,
        ..
    } = result;
    if response_event.event == AgentEventName::AgentCommandRejected {
        return Err(rejection_message(&response_event));
    }
    if response_event.event != AgentEventName::AgentPolicyPreviewReadModelReported {
        return Err(format!(
            "agent-service expected {}, received {}",
            serialized_enum_label(&AgentEventName::AgentPolicyPreviewReadModelReported),
            serialized_enum_label(&response_event.event)
        ));
    }

    let event = events.last().cloned().ok_or_else(|| {
        "agent-service policy preview result did not include a response event".to_string()
    })?;
    let read_model = policy_preview_read_model_from_payload(&response_event.payload)?;

    Ok(PolicyPreviewAgentServiceSnapshot { event, read_model })
}
