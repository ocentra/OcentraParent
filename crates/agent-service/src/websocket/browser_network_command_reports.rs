use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};

use super::{
    basic_reports::build_log_snapshot_report,
    browser_command_reports::build_browser_command_report,
    network_command_reports::build_network_command_report,
};

pub(super) async fn build_browser_network_command_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    match command.command.clone() {
        AgentCommandName::AgentBrowserInventoryReadModelGet
        | AgentCommandName::AgentBrowserEvidenceRecentGet
        | AgentCommandName::AgentBrowserManagedBridgePoll
        | AgentCommandName::AgentBrowserInterventionReadModelGet
        | AgentCommandName::AgentBrowserRuntimeEventChainStreamGet => {
            build_browser_command_report(command).await
        }
        AgentCommandName::AgentNetworkFlowReadModelGet
        | AgentCommandName::AgentNetworkRuntimeEventChainStreamGet
        | AgentCommandName::AgentNetworkRemoteDeliveryStatusGet
        | AgentCommandName::AgentNetworkLiveCaptureStatusGet
        | AgentCommandName::AgentNetworkAndroidVpnServiceGateStatusGet
        | AgentCommandName::AgentNetworkAppleNetworkExtensionGateStatusGet
        | AgentCommandName::AgentNetworkLinuxNftablesLabStatusGet
        | AgentCommandName::AgentNetworkWindowsFirewallLabStatusGet
        | AgentCommandName::AgentNetworkWindowsWfpGateStatusGet => {
            build_network_command_report(command).await
        }
        _ => build_log_snapshot_report(command),
    }
}
