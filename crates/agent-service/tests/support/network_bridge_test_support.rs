use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    remote_delivery_transport_dispatch_state_types::NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
};
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkFlowDigest, ActivityNetworkFlowReadModel,
};
use std::string::String as TestString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkBridgeTestError {
    /// BRAND-INVARIANT: this is test-only failure context, not user data.
    context: TestString,
}

impl NetworkBridgeTestError {
    fn new(context: TestString) -> Self {
        Self { context }
    }
}

impl std::fmt::Display for NetworkBridgeTestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.context.as_str())
    }
}

impl std::error::Error for NetworkBridgeTestError {}

pub fn network_android_vpn_service_gate_status_payload_for_test(
) -> Result<LogFields, NetworkBridgeTestError> {
    crate::network_android_vpn_service_gate_status_bridge::network_android_vpn_service_gate_status_payload()
        .map_err(|_error| {
            NetworkBridgeTestError::new(
                "network_android_vpn_service_gate_status_payload_for_test".to_string(),
            )
        })
}

pub fn network_apple_network_extension_gate_status_payload_for_test(
) -> Result<LogFields, NetworkBridgeTestError> {
    crate::network_apple_network_extension_gate_status_bridge::network_apple_network_extension_gate_status_payload()
        .map_err(|_error| {
            NetworkBridgeTestError::new(
                "network_apple_network_extension_gate_status_payload_for_test".to_string(),
            )
        })
}

pub fn network_linux_nftables_lab_status_payload_for_test(
) -> Result<LogFields, NetworkBridgeTestError> {
    crate::network_linux_nftables_lab_status_bridge::network_linux_nftables_lab_status_payload()
        .map_err(|_error| {
            NetworkBridgeTestError::new(
                "network_linux_nftables_lab_status_payload_for_test".to_string(),
            )
        })
}

pub fn network_live_capture_status_payload_for_test() -> Result<LogFields, NetworkBridgeTestError> {
    crate::network_live_capture_readiness_bridge::network_live_capture_status_payload().map_err(
        |_error| {
            NetworkBridgeTestError::new("network_live_capture_status_payload_for_test".to_string())
        },
    )
}

pub fn network_windows_firewall_lab_status_payload_for_test(
) -> Result<LogFields, NetworkBridgeTestError> {
    crate::network_windows_firewall_lab_status_bridge::network_windows_firewall_lab_status_payload()
        .map_err(|_error| {
            NetworkBridgeTestError::new(
                "network_windows_firewall_lab_status_payload_for_test".to_string(),
            )
        })
}

pub fn network_windows_wfp_gate_status_payload_for_test(
) -> Result<LogFields, NetworkBridgeTestError> {
    crate::network_windows_wfp_gate_status_bridge::network_windows_wfp_gate_status_payload()
        .map_err(|_error| {
            NetworkBridgeTestError::new(
                "network_windows_wfp_gate_status_payload_for_test".to_string(),
            )
        })
}

pub fn network_flow_digest_for_test(
    read_model: &ActivityNetworkFlowReadModel,
) -> ActivityNetworkFlowDigest {
    crate::network_flow_digest::network_flow_digest(read_model)
}

pub async fn network_remote_delivery_status_payload_for_test(
) -> Result<LogFields, NetworkBridgeTestError> {
    crate::network_remote_delivery_status_payload::network_remote_delivery_status_payload()
        .await
        .map_err(|_error| {
            NetworkBridgeTestError::new(
                "network_remote_delivery_status_payload_for_test".to_string(),
            )
        })
}

pub fn blocked_dispatch_records_match_outbox_candidates_for_test(
    report: &NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    outbox_report: &NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
) -> bool {
    crate::network_remote_delivery_status_payload::blocked_dispatch_records_match_outbox_candidates(
        report,
        outbox_report,
    )
}
