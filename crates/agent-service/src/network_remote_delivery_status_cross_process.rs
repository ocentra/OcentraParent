use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_cross_process_custody_readiness_types::{
        NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
        NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessState as RuntimeCrossProcessCustodyReadinessState,
    },
    remote_delivery_cross_process_replay_types::NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    remote_delivery_external_cross_process_transport_types::NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
    remote_delivery_provider_child_readiness_types::{
        NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
        NetworkRuntimeRemoteDeliveryProviderChildReadinessState as RuntimeProviderChildReadinessState,
    },
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::NetworkRemoteDeliveryCrossProcessCustodyReadinessState;
use ocentra_parent_agent_protocol::network_flow::NetworkRemoteDeliveryProviderChildReadinessState;
use ocentra_parent_agent_protocol::network_flow::NetworkRemoteDeliveryStatus;

pub(crate) fn apply_provider_child_readiness_status(
    status: &mut NetworkRemoteDeliveryStatus,
    report: &NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    cross_process_report: &NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
) {
    status.provider_route_ref = report.provider_route_ref.as_str().to_string();
    status.child_device_route_ref = report.child_device_route_ref.as_str().to_string();
    status.provider_delivery_readiness_ref = report.provider_readiness_ref.as_str().to_string();
    status.child_device_delivery_readiness_ref =
        report.child_device_readiness_ref.as_str().to_string();
    status.provider_delivery_readiness_state = match report.provider_state {
        RuntimeProviderChildReadinessState::ManualRequiredUnavailable => {
            NetworkRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable
        }
    };
    status.child_device_delivery_readiness_state = match report.child_device_state {
        RuntimeProviderChildReadinessState::ManualRequiredUnavailable => {
            NetworkRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable
        }
    };
    status.provider_delivery_readiness_record_count =
        count(report.provider_delivery_readiness_record_count);
    status.child_device_delivery_readiness_record_count =
        count(report.child_device_delivery_readiness_record_count);
    status.provider_delivery_artifact_count = count(report.provider_delivery_artifact_count);
    status.child_device_delivery_artifact_count =
        count(report.child_device_delivery_artifact_count);
    status.provider_delivery_records_match_fixture_acks =
        report.provider_delivery_records_match_fixture_acks;
    status.child_device_delivery_records_match_fixture_acks =
        report.child_device_delivery_records_match_fixture_acks;
    status.status_ref =
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_CUSTODY_STATUS_REF.to_string();
    status.cross_process_custody_status_ref = cross_process_report
        .cross_process_custody_status_ref
        .as_str()
        .to_string();
    status.cross_process_replay_readiness_ref = cross_process_report
        .cross_process_replay_readiness_ref
        .as_str()
        .to_string();
    status.remote_retention_readiness_ref = cross_process_report
        .remote_retention_readiness_ref
        .as_str()
        .to_string();
    status.remote_delete_custody_readiness_ref = cross_process_report
        .remote_delete_custody_readiness_ref
        .as_str()
        .to_string();
    status.remote_export_custody_readiness_ref = cross_process_report
        .remote_export_custody_readiness_ref
        .as_str()
        .to_string();
    status.cross_process_custody_readiness_state = match cross_process_report.custody_state {
        RuntimeCrossProcessCustodyReadinessState::ManualRequiredUnavailable => {
            NetworkRemoteDeliveryCrossProcessCustodyReadinessState::ManualRequiredUnavailable
        }
    };
    status.cross_process_replay_readiness_record_count =
        count(cross_process_report.cross_process_replay_readiness_record_count);
    status.remote_retention_readiness_record_count =
        count(cross_process_report.remote_retention_readiness_record_count);
    status.remote_delete_custody_readiness_record_count =
        count(cross_process_report.remote_delete_custody_readiness_record_count);
    status.remote_export_custody_readiness_record_count =
        count(cross_process_report.remote_export_custody_readiness_record_count);
    status.cross_process_custody_records_match_provider_child_readiness =
        cross_process_report.custody_records_match_provider_child_readiness;
    status.cross_process_replay_artifact_count =
        count(cross_process_report.cross_process_replay_artifact_count);
    status.remote_retention_artifact_count =
        count(cross_process_report.remote_retention_artifact_count);
    status.remote_delete_custody_artifact_count =
        count(cross_process_report.remote_delete_custody_artifact_count);
    status.remote_export_custody_artifact_count =
        count(cross_process_report.remote_export_custody_artifact_count);
}

pub(crate) fn apply_cross_process_replay_status(
    status: &mut NetworkRemoteDeliveryStatus,
    report: &NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
) {
    status.status_ref =
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_STATUS_REF.to_string();
    status.cross_process_replay_ref = report.cross_process_replay_ref.as_str().to_string();
    status.cross_process_replay_store_ref =
        report.cross_process_replay_store_ref.as_str().to_string();
    status.cross_process_replay_cursor_ref =
        report.cross_process_replay_cursor_ref.as_str().to_string();
    status.cross_process_replay_record_count = count(report.cross_process_replay_record_count);
    status.cross_process_replay_store_write_count =
        count(report.cross_process_replay_store_write_count);
    status.cross_process_replay_cursor_next_sequence =
        report.cross_process_replay_cursor_next_sequence;
    status.cross_process_replay_records_match_durable_envelopes =
        report.cross_process_replay_records_match_durable_envelopes;
    status.cross_process_replay_records_match_custody_readiness =
        report.cross_process_replay_records_match_custody_readiness;
    status.cross_process_replay_implemented = report.cross_process_replay_implemented;
}

pub(crate) fn apply_external_cross_process_transport_status(
    status: &mut NetworkRemoteDeliveryStatus,
    report: &NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
) {
    status.status_ref =
        constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_STATUS_REF
            .to_string();
    status.external_cross_process_transport_ref = report
        .external_cross_process_transport_ref
        .as_str()
        .to_string();
    status.external_cross_process_transport_envelope_ref = report
        .external_cross_process_transport_envelope_ref
        .as_str()
        .to_string();
    status.external_cross_process_transport_ack_ref = report
        .external_cross_process_transport_ack_ref
        .as_str()
        .to_string();
    status.external_cross_process_transport_record_count =
        count(report.external_cross_process_transport_record_count);
    status.external_cross_process_transport_envelope_count =
        count(report.external_cross_process_transport_envelope_count);
    status.external_cross_process_transport_ack_count =
        count(report.external_cross_process_transport_ack_count);
    status.external_cross_process_transport_records_match_replay_records =
        report.external_cross_process_transport_records_match_replay_records;
    status.external_cross_process_transport_ack_records_match_envelopes =
        report.external_cross_process_transport_ack_records_match_envelopes;
    status.external_cross_process_transport_implemented =
        report.external_cross_process_transport_implemented;
}

fn count(value: usize) -> u64 {
    value as u64
}
