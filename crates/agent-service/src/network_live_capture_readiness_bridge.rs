#[path = "network_live_capture_readiness_bridge/boolean_counts.rs"]
mod boolean_counts;
#[path = "network_live_capture_readiness_bridge/execution_counts.rs"]
mod execution_counts;
#[path = "network_live_capture_readiness_bridge/inputs.rs"]
mod inputs;
#[path = "network_live_capture_readiness_bridge/mapping.rs"]
mod mapping;
#[path = "network_live_capture_readiness_bridge/platform_mapping.rs"]
mod platform_mapping;
#[path = "network_live_capture_readiness_bridge/proof_counts.rs"]
mod proof_counts;
#[path = "network_live_capture_readiness_bridge/proof_state_mapping.rs"]
mod proof_state_mapping;
#[path = "network_live_capture_readiness_bridge/rows.rs"]
mod rows;
#[path = "network_live_capture_readiness_bridge/state_counts.rs"]
mod state_counts;
#[path = "network_live_capture_readiness_bridge/status.rs"]
mod status;
#[path = "network_live_capture_readiness_bridge/storage_counts.rs"]
mod storage_counts;
#[path = "network_live_capture_readiness_bridge/storage_state_mapping.rs"]
mod storage_state_mapping;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::network_flow::NetworkLiveCaptureStatus;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use self::rows::live_capture_rows;
use self::status::status_from_rows;
use crate::{event_builder::build_event, fields::fields_from_pairs};

pub(crate) fn build_network_live_capture_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let correlation_id = command.message_id.clone();
    let target = command.source;
    match network_live_capture_status_payload() {
        Ok(payload) => build_event(
            constants::event_id::NETWORK_LIVE_CAPTURE_STATUS_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentNetworkLiveCaptureStatusReported,
            LogLevel::Info,
            payload,
            None,
        ),
        Err(()) => build_event(
            constants::event_id::COMMAND_REJECTED,
            &correlation_id,
            target,
            AgentEventName::AgentCommandRejected,
            LogLevel::Warn,
            fields_from_pairs(vec![(
                constants::field::REASON,
                LogFieldValue::String(
                    constants::network_flow::ERROR_NETWORK_LIVE_CAPTURE_STATUS.to_string(),
                ),
            )]),
            None,
        ),
    }
}

pub(crate) fn network_live_capture_status_payload() -> Result<LogFields, ()> {
    let status = network_live_capture_status()?;
    let serialized = serde_json::to_string(&status).map_err(|_error| ())?;
    Ok(fields_from_pairs(vec![(
        constants::network_flow::FIELD_NETWORK_LIVE_CAPTURE_STATUS,
        LogFieldValue::String(serialized),
    )]))
}

fn network_live_capture_status() -> Result<NetworkLiveCaptureStatus, ()> {
    let rows = live_capture_rows()?;
    Ok(status_from_rows(rows))
}
