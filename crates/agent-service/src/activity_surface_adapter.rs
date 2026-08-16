use ocentra_parent_agent_protocol::activity_surface::ActivityReportFrequency;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;

use crate::{
    activity_family_sources::family_sources_from_command,
    activity_surface_read_models::{
        app_use::app_use_read_model, browser_read_model, games::games_read_model,
        network_read_model, screen_read_model,
    },
    activity_surface_report::report_document,
    activity_surface_report_store::{history_list, save_report_document},
    activity_surface_request::{report_document_from_command, report_request_from_command},
    activity_surface_store::{
        load_app_game_model, load_browser_model, load_network_model, load_screen_summary,
        local_store_snapshot,
    },
};

pub(crate) async fn build_activity_report_document(
    command: &AgentCommandEnvelope,
    frequency: ActivityReportFrequency,
) -> ocentra_parent_agent_protocol::activity_surface::ActivityReportDocument {
    let request = report_request_from_command(command, frequency);
    report_document(
        request,
        local_store_snapshot().await,
        family_sources_from_command(command),
    )
}

pub(crate) async fn build_saved_activity_report(
    command: &AgentCommandEnvelope,
) -> ocentra_parent_agent_protocol::activity_surface::ActivityReportDocument {
    let report = match report_document_from_command(command) {
        Some(report) => report,
        None => build_activity_report_document(command, ActivityReportFrequency::Daily).await,
    };
    save_report_document(report)
}

pub(crate) async fn build_activity_history(
    command: &AgentCommandEnvelope,
) -> ocentra_parent_agent_protocol::activity_surface::ActivityHistoricalReportList {
    history_list(crate::activity_surface_request::surface_request_from_command(command))
}

pub(crate) async fn build_screen_read_model(
    command: &AgentCommandEnvelope,
) -> ocentra_parent_agent_protocol::activity_surface::ActivityScreenReadModel {
    screen_read_model(
        crate::activity_surface_request::surface_request_from_command(command),
        load_screen_summary().await,
    )
}

pub(crate) async fn build_app_use_read_model(
    command: &AgentCommandEnvelope,
) -> ocentra_parent_agent_protocol::activity_surface::ActivityAppUseReadModel {
    app_use_read_model(
        crate::activity_surface_request::surface_request_from_command(command),
        load_app_game_model().await,
    )
}

pub(crate) async fn build_browser_read_model(
    command: &AgentCommandEnvelope,
) -> ocentra_parent_agent_protocol::activity_surface::ActivityBrowserReadModel {
    browser_read_model(
        crate::activity_surface_request::surface_request_from_command(command),
        load_browser_model().await,
    )
}

pub(crate) async fn build_games_read_model(
    command: &AgentCommandEnvelope,
) -> ocentra_parent_agent_protocol::activity_surface::ActivityGamesReadModel {
    games_read_model(
        crate::activity_surface_request::surface_request_from_command(command),
        load_app_game_model().await,
    )
}

pub(crate) async fn build_network_read_model(
    command: &AgentCommandEnvelope,
) -> ocentra_parent_agent_protocol::activity_surface::ActivityNetworkReadModel {
    network_read_model(
        crate::activity_surface_request::surface_request_from_command(command),
        load_network_model().await,
    )
}
