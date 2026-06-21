use ocentra_parent_agent_core::activity_store::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogLevel,
};

use crate::{
    activity_store_path::activity_db_path, event_builder::build_event, time::timestamp_now,
};

use super::{
    activity_store_error_event,
    browser_intervention_payload::browser_intervention_read_model_payload,
};

pub async fn build_browser_intervention_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_browser_intervention_read_model().await {
        Some(read_model) => build_event(
            constants::browser::INTERVENTION_READ_MODEL_REPORTED_EVENT_ID,
            &command.message_id,
            command.source,
            AgentEventName::AgentBrowserInterventionReadModelReported,
            LogLevel::Info,
            browser_intervention_read_model_payload(&read_model),
            None,
        ),
        None => activity_store_error_event(
            command,
            constants::browser::INTERVENTION_READ_MODEL_REPORTED_EVENT_ID,
            AgentEventName::AgentBrowserInterventionReadModelReported,
        ),
    }
}

async fn load_browser_intervention_read_model(
) -> Option<ocentra_parent_agent_protocol::BrowserInterventionReadModel> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        store
            .browser_intervention_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &timestamp_now(),
            )
            .ok()
    })
    .await
    .ok()
    .flatten()
}
