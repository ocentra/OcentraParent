#[path = "app_game_timer_parent_preference_setup_request_support.rs"]
mod app_game_timer_parent_preference_setup_request_support;

use std::path::PathBuf;

use ocentra_parent_agent_protocol::transport::{AgentCommandEnvelope, AgentEventEnvelope};

use crate::activity_store_path::activity_db_path;

use self::app_game_timer_parent_preference_setup_request_support::build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path as build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path_support;

pub struct AppGameTimerSetupStorePath(pub PathBuf);

pub async fn build_activity_app_game_timer_parent_preference_setup_request_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path_support(
        command,
        AppGameTimerSetupStorePath(activity_db_path().into()),
    )
    .await
}

pub(super) async fn build_activity_app_game_timer_parent_preference_setup_request_report_for_test(
    command: AgentCommandEnvelope,
    store_path: AppGameTimerSetupStorePath,
) -> AgentEventEnvelope {
    build_activity_app_game_timer_parent_preference_setup_request_report_for_store_path_support(command, store_path).await
}
