use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};
use std::{future::Future, pin::Pin, sync::Arc};

use super::{
    activity_app_game_action_reports::build_activity_app_game_action_report,
    activity_app_game_read_model_reports::build_activity_app_game_read_model_report,
    basic_reports::build_log_snapshot_report, WebsocketPeerProvenance,
    WebsocketPlatformProbeDispatcher,
};

pub(super) fn build_activity_app_game_command_report(
    command: AgentCommandEnvelope,
    probe_dispatcher: Arc<WebsocketPlatformProbeDispatcher>,
    provenance: WebsocketPeerProvenance,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    Box::pin(async move {
        match command.command.clone() {
            AgentCommandName::AgentActivityAppGameBoundaryReadModelGet
            | AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet
            | AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet
            | AgentCommandName::AgentActivityAppGameAdapterExecutionReadinessReadModelGet
            | AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet
            | AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet
            | AgentCommandName::AgentActivityAppGameAdapterDispatchPreflightReadModelGet
            | AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet => {
                build_activity_app_game_read_model_report(command, probe_dispatcher, provenance)
                    .await
            }
            AgentCommandName::AgentActivityAppGameAdapterDispatchExecute
            | AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet
            | AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest => {
                Box::pin(build_activity_app_game_action_report(command)).await
            }
            _ => build_log_snapshot_report(command),
        }
    })
}
