use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};
use std::{future::Future, pin::Pin};

use crate::{
    activity_api::{
        activity_memory_graph_report::build_activity_memory_graph_report,
        build_activity_ingest_status_report, build_activity_recent_summary_report,
    },
    activity_surface_api::{
        build_activity_daily_report, build_activity_monthly_report, build_activity_report_history,
        build_activity_report_save, build_activity_weekly_report,
    },
};

use super::basic_reports::build_log_snapshot_report;

pub(super) fn build_activity_summary_report(
    command: AgentCommandEnvelope,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    Box::pin(async move {
        match command.command.clone() {
            AgentCommandName::AgentActivityIngestStatusGet => {
                build_activity_ingest_status_report(command).await
            }
            AgentCommandName::AgentActivityRecentSummaryGet => {
                build_activity_recent_summary_report(command).await
            }
            AgentCommandName::AgentActivityMemoryGraphGet => {
                build_activity_memory_graph_report(command).await
            }
            AgentCommandName::AgentActivityReportDailyGenerate => {
                build_activity_daily_report(command).await
            }
            AgentCommandName::AgentActivityReportWeeklyGenerate => {
                build_activity_weekly_report(command).await
            }
            AgentCommandName::AgentActivityReportMonthlyGenerate => {
                build_activity_monthly_report(command).await
            }
            AgentCommandName::AgentActivityReportSave => build_activity_report_save(command).await,
            AgentCommandName::AgentActivityReportHistoryList => {
                build_activity_report_history(command).await
            }
            _ => build_log_snapshot_report(command),
        }
    })
}
