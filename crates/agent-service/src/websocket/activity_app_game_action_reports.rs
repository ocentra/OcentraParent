use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};

use crate::activity_api::{
    app_game_adapter_dispatch_execute_payload::build_activity_app_game_adapter_dispatch_execute_report,
    app_game_timer_parent_preference_setup_request::build_activity_app_game_timer_parent_preference_setup_request_report,
    app_game_timer_parent_surface_report::build_activity_app_game_timer_parent_surface_report,
};

use super::basic_reports::build_log_snapshot_report;

pub(super) async fn build_activity_app_game_action_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentActivityAppGameAdapterDispatchExecute => {
            Box::pin(build_activity_app_game_adapter_dispatch_execute_report(
                command,
            ))
            .await
        }
        AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet => {
            build_activity_app_game_timer_parent_surface_report(command).await
        }
        AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest => {
            build_activity_app_game_timer_parent_preference_setup_request_report(command).await
        }
        _ => build_log_snapshot_report(command),
    }
}
