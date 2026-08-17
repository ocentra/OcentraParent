use ocentra_parent_agent_protocol::transport::AgentCommandName;

pub(super) fn is_activity_command(command: &AgentCommandName) -> bool {
    matches!(
        command,
        AgentCommandName::AgentActivityIngestStatusGet
            | AgentCommandName::AgentActivityRecentSummaryGet
            | AgentCommandName::AgentActivityMemoryGraphGet
            | AgentCommandName::AgentActivityReportDailyGenerate
            | AgentCommandName::AgentActivityReportWeeklyGenerate
            | AgentCommandName::AgentActivityReportMonthlyGenerate
            | AgentCommandName::AgentActivityReportSave
            | AgentCommandName::AgentActivityReportHistoryList
            | AgentCommandName::AgentActivityScreenReadModelGet
            | AgentCommandName::AgentActivityAppUseReadModelGet
            | AgentCommandName::AgentActivityBrowserReadModelGet
            | AgentCommandName::AgentActivityGamesReadModelGet
            | AgentCommandName::AgentActivityAppGameBoundaryReadModelGet
            | AgentCommandName::AgentActivityAppGamePolicyReadinessReadModelGet
            | AgentCommandName::AgentActivityAppGameNotificationReadinessReadModelGet
            | AgentCommandName::AgentActivityAppGameAdapterExecutionReadinessReadModelGet
            | AgentCommandName::AgentActivityAppGamePlatformProofStatusReadModelGet
            | AgentCommandName::AgentActivityAppGameChildRuntimeTransportReceiptReadModelGet
            | AgentCommandName::AgentActivityAppGameAdapterDispatchPreflightReadModelGet
            | AgentCommandName::AgentActivityAppGameAdapterDispatchResultReadModelGet
            | AgentCommandName::AgentActivityAppGameAdapterDispatchExecute
            | AgentCommandName::AgentActivityAppGameTimerParentSurfaceReadModelGet
            | AgentCommandName::AgentActivityAppGameTimerParentPreferenceSetupRequest
            | AgentCommandName::AgentBrowserSocialDashboardReadModelGet
            | AgentCommandName::AgentBrowserSocialAuditExplanationReadModelGet
            | AgentCommandName::AgentBrowserSocialAlertReportReadModelGet
            | AgentCommandName::AgentBrowserSocialAlertReportParentSurfaceReadModelGet
            | AgentCommandName::AgentBrowserSocialParentNotificationDeliveryReadModelGet
            | AgentCommandName::AgentActivityNetworkReadModelGet
            | AgentCommandName::AgentActivityTrackingReadModelGet
            | AgentCommandName::AgentActivityTrackingRetentionSettingsWrite
    )
}

pub(super) fn is_lan_runtime_command(command: &AgentCommandName) -> bool {
    command.is_lan_command()
}

pub(super) fn is_browser_policy_command(command: &AgentCommandName) -> bool {
    matches!(
        command,
        AgentCommandName::AgentBrowserPolicyGet
            | AgentCommandName::AgentBrowserPolicyPreview
            | AgentCommandName::AgentBrowserPolicyPatch
            | AgentCommandName::AgentBrowserPolicyReplace
            | AgentCommandName::AgentBrowserPolicyRollback
    )
}
