use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};

use crate::{
    activity_api::{
        browser_intervention_report::build_browser_intervention_read_model_report,
        build_browser_evidence_recent_report, build_browser_inventory_read_model_report,
    },
    browser_runtime::build_browser_managed_status_report,
    browser_runtime_stream_api::build_browser_runtime_event_chain_stream_report,
};

use super::basic_reports::build_log_snapshot_report;

pub(super) async fn build_browser_command_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentBrowserInventoryReadModelGet => {
            build_browser_inventory_read_model_report(command).await
        }
        AgentCommandName::AgentBrowserEvidenceRecentGet => {
            build_browser_evidence_recent_report(command).await
        }
        AgentCommandName::AgentBrowserManagedBridgePoll => {
            build_browser_managed_status_report(command).await
        }
        AgentCommandName::AgentBrowserInterventionReadModelGet => {
            build_browser_intervention_read_model_report(command).await
        }
        AgentCommandName::AgentBrowserRuntimeEventChainStreamGet => {
            build_browser_runtime_event_chain_stream_report(command).await
        }
        _ => build_log_snapshot_report(command),
    }
}
