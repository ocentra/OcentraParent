use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};
use std::{future::Future, pin::Pin};

use super::{
    activity_app_game_command_reports::build_activity_app_game_command_report,
    activity_social_reports::build_activity_social_report,
    activity_summary_reports::build_activity_summary_report,
    activity_surface_command_reports::build_activity_surface_report,
    basic_reports::build_log_snapshot_report,
};

pub(super) fn build_activity_command_report(
    command: AgentCommandEnvelope,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    Box::pin(async move {
        match command.command.clone() {
            AgentCommandName::AgentActivityIngestStatusGet
            | AgentCommandName::AgentActivityRecentSummaryGet
            | AgentCommandName::AgentActivityMemoryGraphGet
            | AgentCommandName::AgentActivityReportDailyGenerate
            | AgentCommandName::AgentActivityReportWeeklyGenerate
            | AgentCommandName::AgentActivityReportMonthlyGenerate
            | AgentCommandName::AgentActivityReportSave
            | AgentCommandName::AgentActivityReportHistoryList => {
                build_activity_summary_report(command).await
            }
            AgentCommandName::AgentActivityScreenReadModelGet
            | AgentCommandName::AgentActivityAppUseReadModelGet
            | AgentCommandName::AgentActivityBrowserReadModelGet
            | AgentCommandName::AgentActivityGamesReadModelGet
            | AgentCommandName::AgentActivityNetworkReadModelGet
            | AgentCommandName::AgentActivityTrackingReadModelGet
            | AgentCommandName::AgentActivityTrackingRetentionSettingsWrite
            | AgentCommandName::AgentParentRuntimeIntentIngressPublish => {
                build_activity_surface_report(command).await
            }
            AgentCommandName::AgentActivityAppGameBoundaryReadModelGet
            | AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet
            | AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet
            | AgentCommandName::AgentActivityAppGameAdapterExecutionReadinessReadModelGet
            | AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet
            | AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet
            | AgentCommandName::AgentActivityAppGameAdapterDispatchPreflightReadModelGet
            | AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet
            | AgentCommandName::AgentActivityAppGameAdapterDispatchExecute
            | AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet
            | AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest => {
                build_activity_app_game_command_report(command).await
            }
            AgentCommandName::AgentBrowserSocialDashboardReadModelGet
            | AgentCommandName::AgentBrowserSocialAuditExplanationReadModelGet
            | AgentCommandName::AgentBrowserSocialAlertReportReadModelGet
            | AgentCommandName::AgentBrowserSocialAlertReportParentSurfaceReadModelGet
            | AgentCommandName::AgentBrowserSocialParentNotificationDeliveryReadModelGet => {
                build_activity_social_report(command).await
            }
            _ => build_log_snapshot_report(command),
        }
    })
}
