use ocentra_parent_agent_core::network_event_runtime::{
    remote_delivery_cross_process_custody_readiness_types::NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
    remote_delivery_cross_process_replay_types::NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    remote_delivery_delete_export_propagation::prove_network_runtime_remote_delivery_delete_export_propagation,
    remote_delivery_delete_export_propagation_types::NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
    remote_delivery_durable_envelope_types::NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
    remote_delivery_external_cross_process_transport::prove_network_runtime_remote_delivery_external_cross_process_transport,
    remote_delivery_external_cross_process_transport_types::NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
    remote_delivery_fixture_transport_types::NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    remote_delivery_outbox_handoff_types::NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    remote_delivery_provider_child_readiness_types::NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    remote_delivery_status::{
        NetworkRuntimeRemoteDeliveryState, NetworkRuntimeRemoteDeliveryStatusReport,
    },
    remote_delivery_transport_dispatch_state::prove_network_runtime_remote_delivery_transport_dispatch_state,
    remote_delivery_transport_dispatch_state_types::{
        NetworkRuntimeRemoteDeliveryTransportDispatchState as RuntimeTransportDispatchState,
        NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    },
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::logging::LogLevel;
use ocentra_parent_agent_protocol::network_flow::NetworkRemoteDeliveryStatus;
use ocentra_parent_agent_protocol::network_flow::NetworkRemoteDeliveryStatusState;
use ocentra_parent_agent_protocol::network_flow::NetworkRemoteDeliveryTransportDispatchState;
use ocentra_parent_agent_protocol::transport::AgentCommandEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::transport::AgentEventName;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::OnceCell;

use crate::{
    event_builder::build_event,
    fields::fields_from_pairs,
    network_remote_delivery_status_cross_process::{
        apply_cross_process_replay_status, apply_external_cross_process_transport_status,
        apply_provider_child_readiness_status,
    },
};

static NETWORK_REMOTE_DELIVERY_STATUS: OnceCell<NetworkRemoteDeliveryStatus> =
    OnceCell::const_new();

pub(crate) fn build_network_remote_delivery_status_report(
    command: AgentCommandEnvelope,
) -> Pin<Box<dyn Future<Output = AgentEventEnvelope> + Send>> {
    Box::pin(async move {
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
                        constants::network_flow::ERROR_NETWORK_RUNTIME_REMOTE_CROSS_PROCESS_REPLAY_STATUS_BRIDGE
                            .to_string(),
                    ),
                )]),
                None,
            ),
        }
    })
}

pub(crate) fn network_remote_delivery_status_payload(
) -> Pin<Box<dyn Future<Output = Result<LogFields, ()>> + Send>> {
    Box::pin(async move {
        let status = network_remote_delivery_status().await?;
        let serialized = serde_json::to_string(&status).map_err(|_error| ())?;
        Ok(fields_from_pairs(vec![(
            constants::field::NETWORK_REMOTE_DELIVERY_STATUS,
            LogFieldValue::String(serialized),
        )]))
    })
}

fn network_remote_delivery_status(
) -> Pin<Box<dyn Future<Output = Result<&'static NetworkRemoteDeliveryStatus, ()>> + Send>> {
    Box::pin(async move {
        NETWORK_REMOTE_DELIVERY_STATUS
            .get_or_try_init(network_remote_delivery_status_value)
            .await
    })
}

fn network_remote_delivery_status_value(
) -> Pin<Box<dyn Future<Output = Result<NetworkRemoteDeliveryStatus, ()>> + Send>> {
    Box::pin(async move {
        let report = prove_network_runtime_remote_delivery_transport_dispatch_state()
            .await
            .map_err(|_error| ())?;
        let delete_export_report =
            prove_network_runtime_remote_delivery_delete_export_propagation()
                .await
                .map_err(|_error| ())?;
        let external_cross_process_transport =
            prove_network_runtime_remote_delivery_external_cross_process_transport()
                .await
                .map_err(|_error| ())?;
        let cross_process_replay = &external_cross_process_transport.cross_process_replay;
        Ok::<NetworkRemoteDeliveryStatus, ()>(status_from_report(
            &report,
            &delete_export_report,
            &cross_process_replay
                .cross_process_custody_readiness
                .provider_child_readiness,
            &cross_process_replay.cross_process_custody_readiness,
            cross_process_replay,
            &external_cross_process_transport,
        ))
    })
}

fn status_from_report(
    report: &NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    delete_export_report: &NetworkRuntimeRemoteDeliveryDeleteExportPropagationReport,
    provider_child_readiness: &NetworkRuntimeRemoteDeliveryProviderChildReadinessReport,
    cross_process_custody_readiness: &NetworkRuntimeRemoteDeliveryCrossProcessCustodyReadinessReport,
    cross_process_replay: &NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    external_cross_process_transport: &NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
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
    apply_cross_process_replay_status(&mut status, cross_process_replay);
    apply_external_cross_process_transport_status(&mut status, external_cross_process_transport);
    apply_non_claim_status(
        &mut status,
        report,
        remote_status,
        cross_process_replay,
        external_cross_process_transport,
    );
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
    let receipt_refs = receipt_refs(durable_report);
    let durable_refs = durable_refs(durable_report);
    status.event_chain_journal_ref = receipt_refs.event_chain_journal_ref;
    status.receipt_ledger_ref = receipt_refs.receipt_ledger_ref;
    status.local_receipt_ack_ref = receipt_refs.local_receipt_ack_ref;
    status.durable_envelope_ref = outbox_report.durable_envelope_ref.as_str().to_string();
    status.durable_store_ref = outbox_report.durable_store_ref.as_str().to_string();
    status.durable_replay_ref = durable_refs.durable_replay_ref;
    status.durable_delete_export_ref = durable_refs.durable_delete_export_ref;
    status.durable_support_status_ref = durable_refs.durable_support_status_ref;
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

fn apply_non_claim_status(
    status: &mut NetworkRemoteDeliveryStatus,
    report: &NetworkRuntimeRemoteDeliveryTransportDispatchStateReport,
    remote_status: &NetworkRuntimeRemoteDeliveryStatusReport,
    cross_process_replay: &NetworkRuntimeRemoteDeliveryCrossProcessReplayReport,
    external_cross_process_transport: &NetworkRuntimeRemoteDeliveryExternalCrossProcessTransportReport,
) {
    status.broker_delivery_implemented = report.broker_delivery_implemented;
    status.family_hub_delivery_implemented = report.family_hub_delivery_implemented;
    status.remote_delivery_ack_implemented = report.remote_delivery_ack_implemented;
    status.provider_delivery_implemented = report.provider_delivery_implemented;
    status.child_device_delivery_implemented = report.child_device_delivery_implemented;
    debug_assert!(!remote_status.cross_process_replay_implemented);
    status.cross_process_replay_implemented = cross_process_replay.cross_process_replay_implemented;
    status.external_cross_process_transport_implemented =
        external_cross_process_transport.external_cross_process_transport_implemented;
    status.remote_delete_export_propagation_implemented =
        report.remote_delete_export_propagation_implemented;
    status.product_ready_remote_delivery = report.product_ready_remote_delivery;
    status.policy_authority = report.policy_authority;
    status.side_effect_authority = report.side_effect_authority;
    status.host_filtering_claimed = false;
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

struct ReceiptRefs {
    event_chain_journal_ref: String,
    receipt_ledger_ref: String,
    local_receipt_ack_ref: String,
}

fn receipt_refs(report: &NetworkRuntimeRemoteDeliveryDurableEnvelopeReport) -> ReceiptRefs {
    ReceiptRefs {
        event_chain_journal_ref: report
            .receipt_ledger
            .event_chain_journal_ref
            .as_str()
            .to_string(),
        receipt_ledger_ref: report
            .receipt_ledger
            .receipt_ledger_ref
            .as_str()
            .to_string(),
        local_receipt_ack_ref: report
            .receipt_ledger
            .local_receipt_ack_ref
            .as_str()
            .to_string(),
    }
}

struct DurableRefs {
    durable_replay_ref: String,
    durable_delete_export_ref: String,
    durable_support_status_ref: String,
}

fn durable_refs(report: &NetworkRuntimeRemoteDeliveryDurableEnvelopeReport) -> DurableRefs {
    DurableRefs {
        durable_replay_ref: report.durable_replay_ref.as_str().to_string(),
        durable_delete_export_ref: report.delete_export_readiness_ref.as_str().to_string(),
        durable_support_status_ref: report.durable_support_status_ref.as_str().to_string(),
    }
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
