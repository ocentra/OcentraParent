use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use crate::{
    activity_memory_graph_payload::activity_memory_graph_payload,
    activity_store_path::activity_db_path, event_builder::build_event, time::timestamp_now,
};

use super::activity_store_error_event::activity_store_error_event;

pub async fn build_activity_memory_graph_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_activity_memory_graph().await {
        Some(read_model) => build_event(
            constants::event_id::ACTIVITY_MEMORY_GRAPH_REPORTED,
            &command.message_id,
            command.source,
            AgentEventName::AgentActivityMemoryGraphReported,
            LogLevel::Info,
            activity_memory_graph_payload(&read_model),
            None,
        ),
        None => activity_store_error_event(
            command,
            crate::activity_api::ActivityEventId(
                constants::event_id::ACTIVITY_MEMORY_GRAPH_REPORTED,
            ),
            AgentEventName::AgentActivityMemoryGraphReported,
        ),
    }
}

async fn load_activity_memory_graph(
) -> Option<ocentra_parent_agent_protocol::activity_memory_graph::ActivityMemoryGraphReadModel> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        store
            .activity_memory_graph_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                timestamp_now::<String>().as_str(),
            )
            .ok()
    })
    .await
    .ok()
    .flatten()
}
