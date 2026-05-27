use ocentra_parent_agent_protocol::{
    constants, ActivityReportFrequency, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName,
    LogLevel,
};

use crate::{
    activity_surface_adapter::{
        build_activity_history, build_activity_report_document, build_app_use_read_model,
        build_browser_read_model, build_games_read_model, build_network_read_model,
        build_saved_activity_report, build_screen_read_model,
    },
    activity_surface_payload::{
        activity_history_payload, activity_read_model_payload, activity_report_document_payload,
    },
    event_builder::build_event,
};

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
    build_event(
        constants::event_id::ACTIVITY_REPORT_SAVED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityReportSaved,
        LogLevel::Warn,
        activity_report_document_payload(&report),
        None,
    )
}

pub async fn build_activity_report_history(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let history = build_activity_history(&command).await;
    build_event(
        constants::event_id::ACTIVITY_REPORT_HISTORY_REPORTED,
        &command.message_id,
        command.source,
        AgentEventName::AgentActivityReportHistoryReported,
        LogLevel::Info,
        activity_history_payload(&history),
        None,
    )
}

pub async fn build_activity_screen_read_model(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let read_model = build_screen_read_model(&command).await;
    read_model_event(
        command,
        AgentEventName::AgentActivityScreenReadModelReported,
        constants::event_id::ACTIVITY_SCREEN_READ_MODEL_REPORTED,
        constants::activity_surface::READ_MODEL_SCREEN,
        read_model.state,
        read_model.rows.len(),
        serde_json::to_string(&read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
    )
}

pub async fn build_activity_app_use_read_model(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model = build_app_use_read_model(&command).await;
    read_model_event(
        command,
        AgentEventName::AgentActivityAppUseReadModelReported,
        constants::event_id::ACTIVITY_APP_USE_READ_MODEL_REPORTED,
        constants::activity_surface::READ_MODEL_APP_USE,
        read_model.state,
        read_model.rows.len(),
        serde_json::to_string(&read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
    )
}

pub async fn build_activity_browser_read_model(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model = build_browser_read_model(&command).await;
    read_model_event(
        command,
        AgentEventName::AgentActivityBrowserReadModelReported,
        constants::event_id::ACTIVITY_BROWSER_READ_MODEL_REPORTED,
        constants::activity_surface::READ_MODEL_BROWSER,
        read_model.state,
        read_model.rows.len(),
        serde_json::to_string(&read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
    )
}

pub async fn build_activity_games_read_model(command: AgentCommandEnvelope) -> AgentEventEnvelope {
    let read_model = build_games_read_model(&command).await;
    read_model_event(
        command,
        AgentEventName::AgentActivityGamesReadModelReported,
        constants::event_id::ACTIVITY_GAMES_READ_MODEL_REPORTED,
        constants::activity_surface::READ_MODEL_GAMES,
        read_model.state,
        read_model.rows.len(),
        serde_json::to_string(&read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
    )
}

pub async fn build_activity_network_read_model(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let read_model = build_network_read_model(&command).await;
    read_model_event(
        command,
        AgentEventName::AgentActivityNetworkReadModelReported,
        constants::event_id::ACTIVITY_NETWORK_READ_MODEL_REPORTED,
        constants::activity_surface::READ_MODEL_NETWORK,
        read_model.state,
        read_model.rows.len(),
        serde_json::to_string(&read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
    )
}

async fn build_generated_report(
    command: AgentCommandEnvelope,
    frequency: ActivityReportFrequency,
) -> AgentEventEnvelope {
    let report = build_activity_report_document(&command, frequency).await;
    build_event(
        constants::event_id::ACTIVITY_REPORT_GENERATED,
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
    event_id: &'static str,
    read_model_kind: &'static str,
    state: ocentra_parent_agent_protocol::ActivityReadModelState,
    row_count: usize,
    read_model_json: String,
) -> AgentEventEnvelope {
    build_event(
        event_id,
        &command.message_id,
        command.source,
        event,
        LogLevel::Info,
        activity_read_model_payload(read_model_kind, state, row_count, read_model_json),
        None,
    )
}
