use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};
use std::{future::Future, pin::Pin};

use crate::{
    activity_api::build_activity_tracking_read_model_report,
    activity_surface_api::{
        build_activity_app_use_read_model, build_activity_browser_read_model,
        build_activity_games_read_model, build_activity_network_read_model,
        build_activity_screen_read_model,
    },
};

use super::{
    basic_reports::build_log_snapshot_report,
    parent_runtime_intent::build_parent_runtime_intent_ingress_report,
    tracking_retention_settings_write::build_tracking_retention_settings_write_report,
};

pub(super) fn build_activity_surface_report(
    command: AgentCommandEnvelope,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    Box::pin(async move {
        match command.command.clone() {
            AgentCommandName::AgentActivityScreenReadModelGet => {
                build_activity_screen_read_model(command).await
            }
            AgentCommandName::AgentActivityAppUseReadModelGet => {
                build_activity_app_use_read_model(command).await
            }
            AgentCommandName::AgentActivityBrowserReadModelGet => {
                build_activity_browser_read_model(command).await
            }
            AgentCommandName::AgentActivityGamesReadModelGet => {
                build_activity_games_read_model(command).await
            }
            AgentCommandName::AgentActivityNetworkReadModelGet => {
                build_activity_network_read_model(command).await
            }
            AgentCommandName::AgentActivityTrackingReadModelGet => {
                build_activity_tracking_read_model_report(command).await
            }
            AgentCommandName::AgentActivityTrackingRetentionSettingsWrite => {
                Box::pin(build_tracking_retention_settings_write_report(command)).await
            }
            AgentCommandName::AgentParentRuntimeIntentIngressPublish => {
                build_parent_runtime_intent_ingress_report(command).await
            }
            _ => build_log_snapshot_report(command),
        }
    })
}
