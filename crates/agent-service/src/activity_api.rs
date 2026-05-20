use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, ActivityIngestStatus, ActivityRecentSummary, AgentCommandEnvelope,
    AgentEventEnvelope, AgentEventName, LogLevel,
};

use crate::{
    activity_payload::{
        activity_store_error_payload, ingest_status_payload, recent_summary_payload,
    },
    activity_store_path::activity_db_path,
    event_builder::build_event,
};

pub async fn build_activity_ingest_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_activity_ingest_status().await {
        Some(status) => build_event(
            constants::event_id::ACTIVITY_INGEST_STATUS_REPORTED,
            &command.message_id,
            command.source,
            AgentEventName::AgentActivityIngestStatusReported,
            LogLevel::Info,
            ingest_status_payload(&status),
            None,
        ),
        None => activity_store_error_event(
            command,
            constants::event_id::ACTIVITY_INGEST_STATUS_REPORTED,
            AgentEventName::AgentActivityIngestStatusReported,
        ),
    }
}

pub async fn build_activity_recent_summary_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_activity_recent_summary().await {
        Some(summary) => build_event(
            constants::event_id::ACTIVITY_RECENT_SUMMARY_REPORTED,
            &command.message_id,
            command.source,
            AgentEventName::AgentActivityRecentSummaryReported,
            LogLevel::Info,
            recent_summary_payload(&summary),
            None,
        ),
        None => activity_store_error_event(
            command,
            constants::event_id::ACTIVITY_RECENT_SUMMARY_REPORTED,
            AgentEventName::AgentActivityRecentSummaryReported,
        ),
    }
}

async fn load_activity_ingest_status() -> Option<ActivityIngestStatus> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        store.status().ok()
    })
    .await
    .ok()
    .flatten()
}

async fn load_activity_recent_summary() -> Option<ActivityRecentSummary> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        store
            .recent_summary(constants::activity_store::DEFAULT_RECENT_LIMIT)
            .ok()
    })
    .await
    .ok()
    .flatten()
}

fn activity_store_error_event(
    command: AgentCommandEnvelope,
    event_id_suffix: &str,
    event: AgentEventName,
) -> AgentEventEnvelope {
    build_event(
        event_id_suffix,
        &command.message_id,
        command.source,
        event,
        LogLevel::Error,
        activity_store_error_payload(),
        None,
    )
}
