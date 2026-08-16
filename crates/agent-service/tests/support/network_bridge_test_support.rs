use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    remote_delivery_transport_dispatch_state_types::NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    NetworkRuntimeJournalState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};
use ocentra_parent_agent_protocol::network_flow::{
    ActivityNetworkFlowDigest, ActivityNetworkFlowReadModel,
    NETWORK_FLOW_CUSTODY_PARENT_OWNED_EXPORT, NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS,
    NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
    NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS, NETWORK_FLOW_READ_MODEL_FIELD_EXPORT_CUSTODY,
    NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS,
};
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use std::string::String as TestString;

use crate::test_text::TestText;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkBridgeTestError {
    /// BRAND-INVARIANT: this is a fixed test label, not user data.
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

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct NetworkRuntimeServiceDeliveryReportForTest {
    pub observed_rows: usize,
    pub delivered_rows: usize,
    pub failed_rows: usize,
    pub publish_reports: usize,
    pub stored_events: usize,
    pub dead_letters: usize,
    pub manual_required_rows: usize,
    pub enforcement_command_events: usize,
    pub journal_state: NetworkRuntimeJournalState,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct NetworkRuntimeServiceStreamReportForTest {
    pub observed_rows: usize,
    pub streamed_events: usize,
    pub failed_rows: usize,
    pub manual_required_rows: usize,
    pub enforcement_command_events: usize,
    pub active_rows: usize,
    pub tombstone_rows: usize,
    pub exportable_rows: usize,
    pub deleted_evidence_reference_ids: Vec<TestString>,
    pub entries: Vec<serde_json::Value>,
}

pub(crate) async fn handle_local_command_text_for_test(body: TestText) -> AgentEventEnvelope {
    crate::agent_service_lib::websocket::dispatch_local_command_text(
        crate::agent_service_lib::websocket::WebsocketCommandText(body.0),
    )
    .await
}

pub async fn lock_activity_report_env_for_test() -> tokio::sync::MutexGuard<'static, ()> {
    crate::activity_report_env_lock::REPORT_ENV_LOCK
        .lock()
        .await
}

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

pub async fn deliver_network_runtime_for_read_model_for_test(
    read_model: &ActivityNetworkFlowReadModel,
) -> NetworkRuntimeServiceDeliveryReportForTest {
    let report =
        crate::network_runtime_delivery::deliver_network_runtime_for_read_model(read_model).await;
    delivery_report_for_test(&report)
}

pub fn network_flow_read_model_payload_with_runtime_delivery_for_test(
    read_model: &ActivityNetworkFlowReadModel,
    delivery: Option<&NetworkRuntimeServiceDeliveryReportForTest>,
) -> LogFields {
    crate::activity_network_flow_payload::network_flow_read_model_payload_with_runtime_delivery(
        read_model,
        delivery.map(delivery_report_from_test).as_ref(),
    )
}

pub async fn stream_network_runtime_event_chain_for_read_model_for_test(
    read_model: &ActivityNetworkFlowReadModel,
) -> NetworkRuntimeServiceStreamReportForTest {
    stream_report_for_test(
        crate::network_runtime_stream_payload::stream_network_runtime_event_chain_for_read_model(
            read_model,
        )
        .await,
    )
}

pub fn network_runtime_event_chain_stream_payload_for_test(
    report: &NetworkRuntimeServiceStreamReportForTest,
) -> LogFields {
    let separator = constants::delimiter::LIST.to_string();
    crate::fields::fields_from_pairs(vec![
        (
            constants::field::NETWORK_RUNTIME_OBSERVED_ROWS,
            network_count_value(report.observed_rows),
        ),
        (
            constants::field::NETWORK_RUNTIME_STREAMED_EVENTS,
            network_count_value(report.streamed_events),
        ),
        (
            constants::field::NETWORK_RUNTIME_FAILED_ROWS,
            network_count_value(report.failed_rows),
        ),
        (
            constants::field::NETWORK_RUNTIME_MANUAL_REQUIRED_ROWS,
            network_count_value(report.manual_required_rows),
        ),
        (
            constants::field::NETWORK_RUNTIME_ENFORCEMENT_COMMAND_EVENTS,
            network_count_value(report.enforcement_command_events),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_ACTIVE_ROWS,
            network_count_value(report.active_rows),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_TOMBSTONE_ROWS,
            network_count_value(report.tombstone_rows),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_EXPORTABLE_ROWS,
            network_count_value(report.exportable_rows),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_EXPORT_CUSTODY,
            LogFieldValue::String(NETWORK_FLOW_CUSTODY_PARENT_OWNED_EXPORT.to_string()),
        ),
        (
            NETWORK_FLOW_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(report.deleted_evidence_reference_ids.join(&separator)),
        ),
        (
            constants::field::NETWORK_RUNTIME_EVENT_CHAIN_STREAM,
            LogFieldValue::String(match serde_json::to_string(&report.entries) {
                Ok(text) => text,
                Err(_error) => constants::value::EMPTY.to_string(),
            }),
        ),
    ])
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

fn delivery_report_for_test(
    report: &crate::network_runtime_delivery::NetworkRuntimeServiceDeliveryReport,
) -> NetworkRuntimeServiceDeliveryReportForTest {
    NetworkRuntimeServiceDeliveryReportForTest {
        observed_rows: report.observed_rows,
        delivered_rows: report.delivered_rows,
        failed_rows: report.failed_rows,
        publish_reports: report.publish_reports,
        stored_events: report.stored_events,
        dead_letters: report.dead_letters,
        manual_required_rows: report.manual_required_rows,
        enforcement_command_events: report.enforcement_command_events,
        journal_state: report.journal_state,
    }
}

fn delivery_report_from_test(
    report: &NetworkRuntimeServiceDeliveryReportForTest,
) -> crate::network_runtime_delivery::NetworkRuntimeServiceDeliveryReport {
    crate::network_runtime_delivery::NetworkRuntimeServiceDeliveryReport {
        observed_rows: report.observed_rows,
        delivered_rows: report.delivered_rows,
        failed_rows: report.failed_rows,
        publish_reports: report.publish_reports,
        stored_events: report.stored_events,
        dead_letters: report.dead_letters,
        manual_required_rows: report.manual_required_rows,
        enforcement_command_events: report.enforcement_command_events,
        journal_state: report.journal_state,
    }
}

fn stream_report_for_test(
    report: crate::network_runtime_stream_payload::NetworkRuntimeServiceStreamReport,
) -> NetworkRuntimeServiceStreamReportForTest {
    NetworkRuntimeServiceStreamReportForTest {
        observed_rows: report.observed_rows,
        streamed_events: report.streamed_events,
        failed_rows: report.failed_rows,
        manual_required_rows: report.manual_required_rows,
        enforcement_command_events: report.enforcement_command_events,
        active_rows: report.active_rows,
        tombstone_rows: report.tombstone_rows,
        exportable_rows: report.exportable_rows,
        deleted_evidence_reference_ids: report.deleted_evidence_reference_ids,
        entries: report
            .entries
            .into_iter()
            .map(|entry| match serde_json::to_value(entry) {
                Ok(value) => value,
                Err(_error) => serde_json::Value::Null,
            })
            .collect(),
    }
}

fn network_count_value(value: usize) -> LogFieldValue {
    LogFieldValue::Number(value as f64)
}
