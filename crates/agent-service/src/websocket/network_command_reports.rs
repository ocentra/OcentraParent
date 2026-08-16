use ocentra_parent_agent_protocol::transport::{
    AgentCommandEnvelope, AgentCommandName, AgentEventEnvelope,
};
use std::{future::Future, pin::Pin};

use crate::{
    activity_api::{
        build_network_flow_read_model_report, build_network_runtime_event_chain_stream_report,
    },
    network_android_vpn_service_gate_status_bridge::build_network_android_vpn_service_gate_status_report,
    network_apple_network_extension_gate_status_bridge::build_network_apple_network_extension_gate_status_report,
    network_linux_nftables_lab_status_bridge::build_network_linux_nftables_lab_status_report,
    network_live_capture_readiness_bridge::build_network_live_capture_status_report,
    network_remote_delivery_status_payload::build_network_remote_delivery_status_report,
    network_windows_firewall_lab_status_bridge::build_network_windows_firewall_lab_status_report,
    network_windows_wfp_gate_status_bridge::build_network_windows_wfp_gate_status_report,
};

use super::basic_reports::build_log_snapshot_report;

pub(super) fn build_network_command_report(
    command: AgentCommandEnvelope,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send + 'static>> {
    Box::pin(async move {
        match command.command.clone() {
            AgentCommandName::AgentNetworkFlowReadModelGet => {
                build_network_flow_read_model_report(command).await
            }
            AgentCommandName::AgentNetworkRuntimeEventChainStreamGet => {
                build_network_runtime_event_chain_stream_report(command).await
            }
            AgentCommandName::AgentNetworkRemoteDeliveryStatusGet => {
                build_network_remote_delivery_status_report(command).await
            }
            AgentCommandName::AgentNetworkLiveCaptureStatusGet => {
                build_network_live_capture_status_report(command)
            }
            AgentCommandName::AgentNetworkAndroidVpnServiceGateStatusGet => {
                build_network_android_vpn_service_gate_status_report(command)
            }
            AgentCommandName::AgentNetworkAppleNetworkExtensionGateStatusGet => {
                build_network_apple_network_extension_gate_status_report(command)
            }
            AgentCommandName::AgentNetworkLinuxNftablesLabStatusGet => {
                build_network_linux_nftables_lab_status_report(command)
            }
            AgentCommandName::AgentNetworkWindowsFirewallLabStatusGet => {
                build_network_windows_firewall_lab_status_report(command)
            }
            AgentCommandName::AgentNetworkWindowsWfpGateStatusGet => {
                build_network_windows_wfp_gate_status_report(command)
            }
            _ => build_log_snapshot_report(command),
        }
    })
}
