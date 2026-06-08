use ocentra_parent_agent_core::{
    prove_network_runtime_remote_delivery_cross_process_custody_readiness,
    prove_network_runtime_remote_delivery_delete_export_propagation,
    prove_network_runtime_remote_delivery_transport_dispatch_state,
    NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
    NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
    NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    NetworkRuntimeRemoteDeliveryProviderChildReadinessReport, NetworkRuntimeRemoteDeliveryState,
    NetworkRuntimeRemoteDeliveryStatusReport,
    NetworkRuntimeRemoteDeliveryTransportDispatchState as RuntimeTransportDispatchState,
    NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
};
use ocentra_parent_agent_protocol::{
    constants, AgentCommandEnvelope, AgentEventEnvelope, AgentEventName, LogFieldValue, LogFields,
    LogLevel, NetworkRemoteDeliveryCrossProcessCustodyReadinessState,
    NetworkRemoteDeliveryProviderChildReadinessState, NetworkRemoteDeliveryStatus,
    NetworkRemoteDeliveryStatusState, NetworkRemoteDeliveryTransportDispatchState,
};
use tokio::sync::OnceCell;

use crate::{event_builder::build_event, fields::fields_from_pairs};

static NETWORK_REMOTE_DELIVERY_STATUS: OnceCell<NetworkRemoteDeliveryStatus> =
    OnceCell::const_new();

pub(crate) async fn build_network_remote_delivery_status_report(
    command: AgentCommandEnvelope,
) -> AgentEventEnvelope {
    let correlation_id = command.message_id.clone();
    let target = command.source;
    match network_remote_delivery_status_payload().await {
        Ok(payload) => build_event(
            constants::event_id::NETWORK_REMOTE_DELIVERY_STATUS_REPORTED,
            &correlation_id,
            target,
            AgentEventName::AgentNetworkRemoteDeliveryStatusReported,
            LogLevel::Info,
            payload,
            None,
        ),
        Err(()) => build_event(
            constants::event_id::COMMAND_REJECTED,
            &correlation_id,
            target,
            AgentEventName::AgentCommandRejected,
            LogLevel::Warn,
            fields_from_pairs(vec![(
                constants::field::REASON,
                LogFieldValue::String(
                    constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_TRANSPORT_DISPATCH_STATE
                        .to_string(),
                ),
            )]),
            None,
        ),
    }
}

pub(crate) async fn network_remote_delivery_status_payload() -> Result<LogFields, ()> {
    let status = network_remote_delivery_status().await?;
    let serialized = serde_json::to_string(&status).map_err(|_| ())?;
    Ok(fields_from_pairs(vec![(
        constants::field::NETWORK_REMOTE_DELIVERY_STATUS,
        LogFieldValue::String(serialized),
    )]))
}

async fn network_remote_delivery_status() -> Result<NetworkRemoteDeliveryStatus, ()> {
    NETWORK_REMOTE_DELIVERY_STATUS
        .get_or_try_init(|| async {
            let report = prove_network_runtime_remote_delivery_transport_dispatch_state()
                .await
                .map_err(|_| ())?;
            let delete_export_report =
                prove_network_runtime_remote_delivery_delete_export_propagation()
                    .await
                    .map_err(|_| ())?;
            let cross_process_custody_readiness =
                prove_network_runtime_remote_delivery_cross_process_custody_readiness()
                    .await
                    .map_err(|_| ())?;
            Ok(status_from_report(
                &report,
                &delete_export_report,
                &cross_process_custody_readiness.provider_child_readiness,
                &cross_process_custody_readiness,
            ))
        })
        .await
        .cloned()
}

fn status_from_report(
    report: &NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    delete_export_report: &NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
    provider_child_readiness: &NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    cross_process_custody_readiness: &NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
) -> NetworkRemoteDeliveryStatus {
    let outbox_report = &report
        .no_enforcement_invariant
        .dispatch_readiness
        .outbox_handoff;
    let durable_report = &outbox_report.durable_envelope;
    let remote_status = &durable_report.receipt_ledger.remote_delivery_status;
    let mut status = NetworkRemoteDeliveryStatus::default();
    apply_remote_status(&mut status, remote_status);
    apply_durable_status(&mut status, outbox_report, durable_report);
    apply_outbox_status(&mut status, outbox_report);
    apply_transport_dispatch_status(&mut status, report, outbox_report);
    apply_fixture_transport_status(&mut status, &delete_export_report.fixture_transport);
    apply_delete_export_status(&mut status, delete_export_report);
    apply_provider_child_readiness_status(
        &mut status,
        provider_child_readiness,
        cross_process_custody_readiness,
    );
    apply_non_claim_status(&mut status, report, remote_status);
    status
}

fn apply_remote_status(
    status: &mut NetworkRemoteDeliveryStatus,
    report: &NetworkRuntimeRemoteDeliveryStatusReport,
) {
    status.broker_status = protocol_remote_delivery_state(report.broker_status);
    status.family_hub_status = protocol_remote_delivery_state(report.family_hub_status);
    status.custody_proof_ref = report.custody_proof_ref.as_str().to_string();
    status.publisher_auth_ref = report.publisher_auth_ref.as_str().to_string();
    status.subscriber_auth_ref = report.subscriber_auth_ref.as_str().to_string();
    status.encryption_ref = report.encryption_ref.as_str().to_string();
    status.retention_policy_ref = report.retention_policy_ref.as_str().to_string();
    status.replay_plan_ref = report.replay_plan_ref.as_str().to_string();
    status.deletion_plan_ref = report.deletion_plan_ref.as_str().to_string();
    status.offset_policy_ref = report.offset_policy_ref.as_str().to_string();
    status.dedupe_policy_ref = report.dedupe_policy_ref.as_str().to_string();
    status.transport_config_ref = report.transport_config_ref.as_str().to_string();
    status.relay_identity_ref = report.relay_identity_ref.as_str().to_string();
    status.relay_policy_ref = report.relay_policy_ref.as_str().to_string();
    status.broker_missing_artifact_count = count(report.broker_missing_artifact_count);
    status.family_hub_missing_artifact_count = count(report.family_hub_missing_artifact_count);
    status.accepted_event_type_count = count(report.accepted_event_type_count);
    status.local_idempotency_queue_proved = report.local_idempotency_queue_proved;
    status.dropped_event_dead_letter_count = count(report.dropped_event_dead_letter_count);
    status.queued_duplicate_rejected = report.queued_duplicate_rejected;
    status.completed_duplicate_rejected = report.completed_duplicate_rejected;
}

fn apply_durable_status(
    status: &mut NetworkRemoteDeliveryStatus,
    outbox_report: &NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    durable_report: &NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
) {
    let (event_chain_journal_ref, receipt_ledger_ref, local_receipt_ack_ref) =
        receipt_refs(durable_report);
    let (durable_replay_ref, durable_delete_export_ref, durable_support_status_ref) =
        durable_refs(durable_report);
    status.event_chain_journal_ref = event_chain_journal_ref;
    status.receipt_ledger_ref = receipt_ledger_ref;
    status.local_receipt_ack_ref = local_receipt_ack_ref;
    status.durable_envelope_ref = outbox_report.durable_envelope_ref.as_str().to_string();
    status.durable_store_ref = outbox_report.durable_store_ref.as_str().to_string();
    status.durable_replay_ref = durable_replay_ref;
    status.durable_delete_export_ref = durable_delete_export_ref;
    status.durable_support_status_ref = durable_support_status_ref;
    status.durable_envelope_ready = durable_envelope_ready(durable_report);
    status.durable_envelope_missing_artifact_count = 0;
}

fn apply_outbox_status(
    status: &mut NetworkRemoteDeliveryStatus,
    report: &NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
) {
    status.outbox_ref = report.outbox_ref.as_str().to_string();
    status.outbox_handoff_ref = report.handoff_ref.as_str().to_string();
    status.outbox_replay_ref = report.outbox_replay_ref.as_str().to_string();
    status.outbox_support_status_ref = report.outbox_support_status_ref.as_str().to_string();
    status.outbox_candidate_count = count(report.outbox_candidate_count);
    status.prepared_not_dispatched_count = count(report.prepared_not_dispatched_count);
    status.duplicate_durable_envelope_rejected = report.duplicate_durable_envelope_rejected;
    status.outbox_candidates_match_durable_envelopes =
        report.outbox_candidates_match_durable_envelopes;
    status.outbox_candidates_match_receipts = report.outbox_candidates_match_receipts;
    status.sequence_gap_count = count(report.sequence_gap_count);
    status.event_id_mismatch_count = count(report.event_id_mismatch_count);
    status.event_type_mismatch_count = count(report.event_type_mismatch_count);
    status.correlation_mismatch_count = count(report.correlation_mismatch_count);
}

fn apply_transport_dispatch_status(
    status: &mut NetworkRemoteDeliveryStatus,
    report: &NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    outbox_report: &NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
) {
    status.status_ref = report.dispatch_state_ref.as_str().to_string();
    status.transport_dispatch_state_ref = report.dispatch_state_ref.as_str().to_string();
    status.blocked_dispatch_ref = report.blocked_dispatch_ref.as_str().to_string();
    status.future_transport_seam_ref = report.future_transport_seam_ref.as_str().to_string();
    status.transport_dispatch_state = match report.state {
        RuntimeTransportDispatchState::ManualRequiredBlocked => {
            NetworkRemoteDeliveryTransportDispatchState::ManualRequiredBlocked
        }
    };
    status.source_outbox_candidate_count = count(report.source_outbox_candidate_count);
    status.blocked_dispatch_record_count = count(report.blocked_dispatch_record_count);
    status.blocked_dispatch_records_match_outbox_candidates =
        blocked_dispatch_records_match_outbox_candidates(report, outbox_report);
    status.dispatch_ready_candidate_count = count(report.dispatch_ready_candidate_count);
    status.dispatch_attempt_count = count(report.dispatch_attempt_count);
    status.remote_ack_count = count(report.remote_ack_count);
}

fn apply_fixture_transport_status(
    status: &mut NetworkRemoteDeliveryStatus,
    report: &NetworkRuntimeRemoteDeliveryFixtureTransportReport,
) {
    status.fixture_transport_ref = report.fixture_transport_ref.as_str().to_string();
    status.fixture_dispatch_attempt_ref = report.fixture_dispatch_attempt_ref.as_str().to_string();
    status.fixture_ack_ref = report.fixture_ack_ref.as_str().to_string();
    status.fixture_source_outbox_candidate_count = count(report.source_outbox_candidate_count);
    status.fixture_dispatch_attempt_count = count(report.fixture_dispatch_attempt_count);
    status.fixture_remote_ack_count = count(report.fixture_remote_ack_count);
    status.fixture_records_match_outbox_candidates = report.fixture_records_match_outbox_candidates;
}

fn apply_delete_export_status(
    status: &mut NetworkRemoteDeliveryStatus,
    report: &NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
) {
    status.status_ref =
        constants::network_flow::TEST_REMOTE_DELIVERY_DELETE_EXPORT_STATUS_BRIDGE_REF.to_string();
    status.delete_export_propagation_ref =
        report.delete_export_propagation_ref.as_str().to_string();
    status.remote_delete_readiness_ref = report.remote_delete_readiness_ref.as_str().to_string();
    status.remote_export_readiness_ref = report.remote_export_readiness_ref.as_str().to_string();
    status.delete_export_readiness_record_count = count(report.propagation_readiness_record_count);
    status.remote_delete_ready_count = count(report.remote_delete_ready_count);
    status.remote_export_ready_count = count(report.remote_export_ready_count);
    status.delete_export_records_match_fixture_acks =
        report.propagation_records_match_fixture_records;
}

fn apply_provider_child_readiness_status(
    status: &mut NetworkRemoteDeliveryStatus,
    report: &NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    cross_process_report: &NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
) {
    status.provider_route_ref = report.provider_route_ref.as_str().to_string();
    status.child_device_route_ref = report.child_device_route_ref.as_str().to_string();
    status.provider_delivery_readiness_ref = report.provider_readiness_ref.as_str().to_string();
    status.child_device_delivery_readiness_ref =
        report.child_device_readiness_ref.as_str().to_string();
    status.provider_delivery_readiness_state =
        NetworkRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable;
    status.child_device_delivery_readiness_state =
        NetworkRemoteDeliveryProviderChildReadinessState::ManualRequiredUnavailable;
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
    status.cross_process_custody_readiness_state =
        NetworkRemoteDeliveryCrossProcessCustodyReadinessState::ManualRequiredUnavailable;
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

fn apply_non_claim_status(
    status: &mut NetworkRemoteDeliveryStatus,
    report: &NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    remote_status: &NetworkRuntimeRemoteDeliveryStatusReport,
) {
    status.broker_delivery_implemented = report.broker_delivery_implemented;
    status.family_hub_delivery_implemented = report.family_hub_delivery_implemented;
    status.remote_delivery_ack_implemented = report.remote_delivery_ack_implemented;
    status.provider_delivery_implemented = report.provider_delivery_implemented;
    status.child_device_delivery_implemented = report.child_device_delivery_implemented;
    status.cross_process_replay_implemented = remote_status.cross_process_replay_implemented;
    status.remote_delete_export_propagation_implemented =
        report.remote_delete_export_propagation_implemented;
    status.product_ready_remote_delivery = report.product_ready_remote_delivery;
    status.policy_authority = report.policy_authority;
    status.side_effect_authority = report.side_effect_authority;
    status.enforcement_command_event_count = count(report.enforcement_command_event_count);
    status.adapter_action_executed_count = count(report.adapter_action_executed_count);
    status.raw_pcap_available_count = count(report.raw_pcap_available_count);
    status.exact_url_available_count = count(report.exact_url_available_count);
    status.decrypted_payload_available_count = count(report.decrypted_payload_available_count);
    status.page_content_available_count = count(report.page_content_available_count);
    status.video_content_available_count = count(report.video_content_available_count);
    status.private_message_content_available_count =
        count(report.private_message_content_available_count);
    status.search_query_available_count = count(report.search_query_available_count);
}

pub(crate) fn blocked_dispatch_records_match_outbox_candidates(
    report: &NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    outbox_report: &NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
) -> bool {
    if report.blocked_dispatch_record_count != report.source_outbox_candidate_count
        || report.source_outbox_candidate_count != outbox_report.outbox_candidate_count
        || report.blocked_dispatch_records.len() != outbox_report.candidates.len()
    {
        return false;
    }

    report
        .blocked_dispatch_records
        .iter()
        .zip(&outbox_report.candidates)
        .all(|(record, candidate)| {
            record.sequence == candidate.sequence
                && record.event_id == candidate.event_id
                && record.event_type == candidate.event_type
                && record.correlation_id == candidate.correlation_id
                && record.source_outbox_state == candidate.state
                && record.outbox_ref == candidate.outbox_ref
                && record.handoff_ref == candidate.handoff_ref
                && record.dispatch_state_ref == report.dispatch_state_ref
                && record.blocked_dispatch_ref == report.blocked_dispatch_ref
                && record.future_transport_seam_ref == report.future_transport_seam_ref
        })
}

fn receipt_refs(
    report: &NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
) -> (String, String, String) {
    (
        report
            .receipt_ledger
            .event_chain_journal_ref
            .as_str()
            .to_string(),
        report
            .receipt_ledger
            .receipt_ledger_ref
            .as_str()
            .to_string(),
        report
            .receipt_ledger
            .local_receipt_ack_ref
            .as_str()
            .to_string(),
    )
}

fn durable_refs(
    report: &NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
) -> (String, String, String) {
    (
        report.durable_replay_ref.as_str().to_string(),
        report.delete_export_readiness_ref.as_str().to_string(),
        report.durable_support_status_ref.as_str().to_string(),
    )
}

fn durable_envelope_ready(report: &NetworkRuntimeRemoteDeliveryDurableEnvelopeReport) -> bool {
    report.durable_store_ready
        && report.durable_replay_ready
        && report.delete_export_readiness_recorded
        && report.durable_records_match_receipts
}

fn protocol_remote_delivery_state(
    state: NetworkRuntimeRemoteDeliveryState,
) -> NetworkRemoteDeliveryStatusState {
    match state {
        NetworkRuntimeRemoteDeliveryState::FixtureRequirementsRecordedButNotImplemented => {
            NetworkRemoteDeliveryStatusState::FixtureRequirementsRecordedButNotImplemented
        }
        NetworkRuntimeRemoteDeliveryState::ManualRequired => {
            NetworkRemoteDeliveryStatusState::ManualRequired
        }
    }
}

fn count(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}
