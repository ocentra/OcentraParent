use ocentra_parent_agent_core::ActivityStore;
use ocentra_parent_agent_protocol::{
    constants, ActivityIngestStatus, ActivityRecentSummary, AgentCommandEnvelope,
    AgentEventEnvelope, AgentEventName, LogLevel,
};

use crate::{
    activity_memory_graph_payload::activity_memory_graph_payload,
    activity_network_flow_payload::network_flow_read_model_payload,
    activity_payload::{
        activity_store_error_payload, ingest_status_payload, recent_summary_payload,
    },
    activity_store_path::activity_db_path,
    browser_evidence_payload::browser_evidence_recent_payload,
    event_builder::build_event,
    time::timestamp_now,
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
            constants::event_id::ACTIVITY_MEMORY_GRAPH_REPORTED,
            AgentEventName::AgentActivityMemoryGraphReported,
        ),
    }
}

pub async fn build_browser_evidence_recent_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_browser_evidence_recent_summary().await {
        Some(summary) => build_event(
            constants::event_id::BROWSER_EVIDENCE_RECENT_REPORTED,
            &command.message_id,
            command.source,
            AgentEventName::AgentBrowserEvidenceRecentReported,
            LogLevel::Info,
            browser_evidence_recent_payload(&summary),
            None,
        ),
        None => activity_store_error_event(
            command,
            constants::event_id::BROWSER_EVIDENCE_RECENT_REPORTED,
            AgentEventName::AgentBrowserEvidenceRecentReported,
        ),
    }
}

pub async fn build_network_flow_read_model_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match load_network_flow_read_model().await {
        Some(read_model) => build_event(
            constants::event_id::NETWORK_FLOW_READ_MODEL_REPORTED,
            &command.message_id,
            command.source,
            AgentEventName::AgentNetworkFlowReadModelReported,
            LogLevel::Info,
            network_flow_read_model_payload(&read_model),
            None,
        ),
        None => activity_store_error_event(
            command,
            constants::event_id::NETWORK_FLOW_READ_MODEL_REPORTED,
            AgentEventName::AgentNetworkFlowReadModelReported,
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

async fn load_browser_evidence_recent_summary(
) -> Option<ocentra_parent_agent_protocol::BrowserEvidenceRecentSummary> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        store.browser_recent_summary().ok()
    })
    .await
    .ok()
    .flatten()
}

async fn load_activity_memory_graph(
) -> Option<ocentra_parent_agent_protocol::ActivityMemoryGraphReadModel> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        store
            .activity_memory_graph_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &timestamp_now(),
            )
            .ok()
    })
    .await
    .ok()
    .flatten()
}

async fn load_network_flow_read_model(
) -> Option<ocentra_parent_agent_protocol::ActivityNetworkFlowReadModel> {
    let path = activity_db_path();
    tokio::task::spawn_blocking(move || {
        let store = ActivityStore::open(path).ok()?;
        store
            .network_flow_read_model(
                constants::activity_store::DEFAULT_RECENT_LIMIT,
                &timestamp_now(),
            )
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
