#[path = "network_windows_wfp_gate_status_bridge/boundary_reason.rs"]
mod boundary_reason;
#[path = "network_windows_wfp_gate_status_bridge/capability_state.rs"]
mod capability_state;
#[path = "network_windows_wfp_gate_status_bridge/gate_input.rs"]
mod gate_input;
#[path = "network_windows_wfp_gate_status_bridge/required_artifact.rs"]
mod required_artifact;
#[path = "network_windows_wfp_gate_status_bridge/status.rs"]
mod status;

use ocentra_network_evidence::windows_wfp_gate::plan_network_windows_wfp_gate;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;

use self::gate_input::gate_input;
use self::status::status_from_proof;
use crate::{event_builder::build_event, fields::fields_from_pairs};

pub(crate) fn build_network_windows_wfp_gate_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let correlation_id = command.message_id.clone();
    let target = command.source;
    match network_windows_wfp_gate_status_payload() {
        Ok(payload) => build_event(
            constants::event_id::NETWORK_WINDOWS_WFP_GATE_STATUS_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentNetworkWindowsWfpGateStatusReported,
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
                    constants::network_flow::ERROR_NETWORK_WINDOWS_WFP_GATE_STATUS.to_string(),
                ),
            )]),
            None,
        ),
    }
}

pub(crate) fn network_windows_wfp_gate_status_payload() -> Result<LogFields, ()> {
    let proof = plan_network_windows_wfp_gate(gate_input()).map_err(|_error| ())?;
    let status = status_from_proof(&proof);
    let serialized = serde_json::to_string(&status).map_err(|_error| ())?;
    Ok(fields_from_pairs(vec![(
        constants::network_flow::FIELD_NETWORK_WINDOWS_WFP_GATE_STATUS,
        LogFieldValue::String(serialized),
    )]))
}
