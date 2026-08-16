use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use crate::{
    activity_api::ActivityEventId, activity_payload::activity_store_error_payload,
    event_builder::build_event,
};

pub(crate) fn activity_store_error_event(
    command: AgentCommandEnvelope,
    event_id_suffix: ActivityEventId,
    event: AgentEventName,
) -> AgentEventEnvelope {
    let mut event_id = String::from(constants::value::ACTIVITY_CAPTURE_STORE_ERROR);
    event_id.push_str(event_id_suffix.0);
    build_event(
        &event_id,
        &command.message_id,
        command.source,
        event,
        LogLevel::Error,
        activity_store_error_payload(),
        None,
    )
}
