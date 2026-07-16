use ocentra_parent_agent_protocol::activity_surface::{
    ActivityReportDocument, ActivityReportFrequency, ActivitySavedReportState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
};

use crate::{
    activity_surface_adapter::{
        build_activity_history, build_activity_report_document, build_app_use_read_model,
        build_browser_read_model, build_games_read_model, build_network_read_model,
        build_saved_activity_report, build_screen_read_model,
    },
    activity_surface_payload::{
        activity_history_payload, activity_read_model_payload, activity_report_document_payload,
        ReadModelJson, ReadModelKind,
    },
    event_builder::build_event,
};

#[derive(Clone, Debug, PartialEq, Eq)]
struct ActivitySurfaceEventId(&'static str);

pub async fn build_activity_daily_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_generated_report(command, ActivityReportFrequency::Daily).await
}

pub async fn build_activity_weekly_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_generated_report(command, ActivityReportFrequency::Weekly).await
}

pub async fn build_activity_monthly_report(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    build_generated_report(command, ActivityReportFrequency::Monthly).await
}

pub async fn build_activity_report_save(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let report = build_saved_activity_report(&command).await;
    let level = activity_report_save_log_level(&report);
    build_event(
        constants::event_id::ACTIVITY_REPORT_SAVED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityReportSaved,
        level,
        activity_report_document_payload(&report),
        None,
    )
}

pub async fn build_activity_report_history(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let history = build_activity_history(&command).await;
    build_event(
        ActivitySurfaceEventId(constants::event_id::ACTIVITY_REPORT_HISTORY_REPORTED).0,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityReportHistoryReported,
        LogLevel::Info,
        activity_history_payload(&history),
        None,
    )
}

fn activity_report_save_log_level(report: &ActivityReportDocument) -> LogLevel {
    match report
        .saved_metadata
        .as_ref()
        .map(|metadata| metadata.saved_state)
    {
        Some(ActivitySavedReportState::Saved) => LogLevel::Info,
        _ => LogLevel::Warn,
    }
}

pub async fn build_activity_screen_read_model(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let read_model = build_screen_read_model(&command).await;
    read_model_event(
        command,
        AgentEventName::AgentActivityScreenReadModelReported,
        &ActivitySurfaceEventId(constants::event_id::ACTIVITY_SCREEN_READ_MODEL_REPORTED),
        ReadModelKind(constants::activity_surface::READ_MODEL_SCREEN.to_string()),
        read_model.state,
        read_model.rows.len(),
        serialized_json(&read_model),
    )
}

pub async fn build_activity_app_use_read_model(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model = build_app_use_read_model(&command).await;
    read_model_event(
        command,
        AgentEventName::AgentActivityAppUseReadModelReported,
        &ActivitySurfaceEventId(constants::event_id::ACTIVITY_APP_USE_READ_MODEL_REPORTED),
        ReadModelKind(constants::activity_surface::READ_MODEL_APP_USE.to_string()),
        read_model.state,
        read_model.rows.len(),
        serialized_json(&read_model),
    )
}

pub async fn build_activity_browser_read_model(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model = build_browser_read_model(&command).await;
    read_model_event(
        command,
        AgentEventName::AgentActivityBrowserReadModelReported,
        &ActivitySurfaceEventId(constants::event_id::ACTIVITY_BROWSER_READ_MODEL_REPORTED),
        ReadModelKind(constants::activity_surface::READ_MODEL_BROWSER.to_string()),
        read_model.state,
        read_model.rows.len(),
        serialized_json(&read_model),
    )
}

pub async fn build_activity_games_read_model(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let read_model = build_games_read_model(&command).await;
    read_model_event(
        command,
        AgentEventName::AgentActivityGamesReadModelReported,
        &ActivitySurfaceEventId(constants::event_id::ACTIVITY_GAMES_READ_MODEL_REPORTED),
        ReadModelKind(constants::activity_surface::READ_MODEL_GAMES.to_string()),
        read_model.state,
        read_model.rows.len(),
        serialized_json(&read_model),
    )
}

pub async fn build_activity_network_read_model(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model = build_network_read_model(&command).await;
    read_model_event(
        command,
        AgentEventName::AgentActivityNetworkReadModelReported,
        &ActivitySurfaceEventId(constants::event_id::ACTIVITY_NETWORK_READ_MODEL_REPORTED),
        ReadModelKind(constants::activity_surface::READ_MODEL_NETWORK.to_string()),
        read_model.state,
        read_model.rows.len(),
        serialized_json(&read_model),
    )
}

async fn build_generated_report(
    command: AgentCommandEnvelope,
    frequency: ActivityReportFrequency,
) -> AgentEventEnvelope {
    let report = build_activity_report_document(&command, frequency).await;
    build_event(
        ActivitySurfaceEventId(constants::event_id::ACTIVITY_REPORT_GENERATED).0,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityReportGenerated,
        LogLevel::Info,
        activity_report_document_payload(&report),
        None,
    )
}

fn read_model_event(
    command: AgentCommandEnvelope,
    event: AgentEventName,
    event_id: &ActivitySurfaceEventId,
    read_model_kind: ReadModelKind,
    state: ocentra_parent_agent_protocol::activity_surface::ActivityReadModelState,
    row_count: usize,
    read_model_json: ReadModelJson,
) -> AgentEventEnvelope {
    build_event(
        event_id.0,
        &command.message_id,
        command.source,
        event,
        LogLevel::Info,
        activity_read_model_payload(read_model_kind, state, row_count, read_model_json),
        None,
    )
}

fn serialized_json<T>(value: &T) -> ReadModelJson
where
    T: serde::Serialize,
{
    ReadModelJson(serde_json::to_string(value).unwrap_or_else(|_| {
        serde_json::Value::String(constants::error::AGENT_EVENT_SERIALIZES.to_string()).to_string()
    }))
}
