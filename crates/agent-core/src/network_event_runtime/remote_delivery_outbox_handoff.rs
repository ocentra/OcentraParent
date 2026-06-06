use std::collections::BTreeSet;

use ocentra_eventing::ReplayRecord;
use ocentra_parent_agent_protocol::constants;

use super::prove_network_runtime_remote_delivery_status;
use super::remote_delivery_event_chain_store::{
    publish_network_runtime_remote_event_chain_store, source_component, unsupported_claim_counts,
};
use super::remote_delivery_outbox_handoff_types::{
    NetworkRuntimeRemoteDeliveryOutboxCandidate, NetworkRuntimeRemoteDeliveryOutboxHandoffError,
    NetworkRuntimeRemoteDeliveryOutboxHandoffReport, NetworkRuntimeRemoteDeliveryOutboxState,
};
use super::remote_delivery_receipt_ledger::receipt_records_from_projection;
use super::remote_delivery_receipt_ledger_types::NetworkRuntimeRemoteDeliveryReceiptRecord;

pub async fn prove_network_runtime_remote_delivery_outbox_handoff() -> Result<
    NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    NetworkRuntimeRemoteDeliveryOutboxHandoffError,
> {
    let remote_delivery_status = prove_network_runtime_remote_delivery_status()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::RemoteDeliveryStatus)?;
    let store = publish_network_runtime_remote_event_chain_store()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::EventChainJournal)?;
    let unsupported = unsupported_claim_counts(&store.payloads, &store.projection.records);
    if unsupported.has_any() {
        return Err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::UnsupportedClaim);
    }
    let receipts = receipt_records_from_projection(&store.projection.records)
        .map_err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::ReceiptLedger)?;
    let candidates = outbox_candidates_from_projection(&store.projection.records, &receipts)?;
    assert_outbox_matches_projection_and_receipts(
        &store.projection.records,
        &receipts,
        &candidates,
    )?;
    Ok(NetworkRuntimeRemoteDeliveryOutboxHandoffReport {
        event_chain_export_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_EXPORT_REF,
        )?,
        receipt_ledger_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF,
        )?,
        outbox_ref: source_component(constants::network_flow::TEST_REMOTE_EVENT_CHAIN_OUTBOX_REF)?,
        handoff_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_HANDOFF_REF,
        )?,
        outbox_replay_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_OUTBOX_REPLAY_REF,
        )?,
        outbox_support_status_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_OUTBOX_SUPPORT_STATUS_REF,
        )?,
        source_projection_replay_record_count: store.projection.records.len(),
        receipt_record_count: receipts.len(),
        outbox_candidate_count: candidates.len(),
        prepared_not_dispatched_count: candidates.len(),
        dispatch_attempt_count: 0,
        remote_ack_count: 0,
        outbox_candidates_match_projection: true,
        outbox_candidates_match_receipts: true,
        sequence_gap_count: 0,
        event_id_mismatch_count: 0,
        event_type_mismatch_count: 0,
        correlation_mismatch_count: 0,
        unique_event_id_count: unique_event_id_count(&candidates),
        unique_idempotency_key_count: unique_idempotency_key_count(&candidates),
        target_handler_count: target_handler_count(&candidates),
        broker_requirement_ref_count: broker_requirement_ref_count(&remote_delivery_status),
        receipt_ref_count: receipt_ref_count(&receipts),
        projection_replay_mode: store.projection.mode,
        broker_delivery_implemented: remote_delivery_status.external_transport_delivery_implemented,
        family_hub_delivery_implemented: remote_delivery_status.family_hub_delivery_implemented,
        cross_process_replay_implemented: remote_delivery_status.cross_process_replay_implemented,
        remote_retention_delete_export_propagation_implemented: remote_delivery_status
            .remote_retention_delete_export_propagation_implemented,
        remote_delivery_ack_implemented: false,
        policy_authority: remote_delivery_status.policy_authority,
        side_effect_authority: remote_delivery_status.side_effect_authority,
        enforcement_command_event_count: unsupported.enforcement_command_event_count,
        adapter_action_executed_count: unsupported.adapter_action_executed_count,
        exact_url_available_count: unsupported.exact_url_available_count,
        decrypted_payload_available_count: unsupported.decrypted_payload_available_count,
        page_content_available_count: unsupported.page_content_available_count,
        remote_delivery_status,
        candidates,
    })
}

fn outbox_candidates_from_projection(
    records: &[ReplayRecord],
    receipts: &[NetworkRuntimeRemoteDeliveryReceiptRecord],
) -> Result<
    Vec<NetworkRuntimeRemoteDeliveryOutboxCandidate>,
    NetworkRuntimeRemoteDeliveryOutboxHandoffError,
> {
    if records.is_empty() || receipts.is_empty() || records.len() != receipts.len() {
        return Err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::EmptyOutbox);
    }
    records
        .iter()
        .zip(receipts.iter())
        .map(outbox_candidate_from_projection)
        .collect::<Result<Vec<NetworkRuntimeRemoteDeliveryOutboxCandidate>, _>>()
}

fn outbox_candidate_from_projection(
    (record, receipt): (&ReplayRecord, &NetworkRuntimeRemoteDeliveryReceiptRecord),
) -> Result<
    NetworkRuntimeRemoteDeliveryOutboxCandidate,
    NetworkRuntimeRemoteDeliveryOutboxHandoffError,
> {
    if record.sequence != receipt.sequence
        || record.envelope.event_id != receipt.event_id
        || record.envelope.contract.event_type != receipt.event_type
        || record.envelope.correlation_id != receipt.correlation_id
    {
        return Err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::OutboxProjectionMismatch);
    }
    Ok(NetworkRuntimeRemoteDeliveryOutboxCandidate {
        sequence: record.sequence,
        event_id: record.envelope.event_id.clone(),
        event_type: record.envelope.contract.event_type.clone(),
        correlation_id: record.envelope.correlation_id.clone(),
        idempotency_key: record.envelope.idempotency_key.clone(),
        target_handler: record.envelope.target_handler.clone(),
        state: NetworkRuntimeRemoteDeliveryOutboxState::PreparedNotDispatched,
        event_chain_export_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_EXPORT_REF,
        )?,
        receipt_ledger_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF,
        )?,
        outbox_ref: source_component(constants::network_flow::TEST_REMOTE_EVENT_CHAIN_OUTBOX_REF)?,
        handoff_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_HANDOFF_REF,
        )?,
    })
}

fn assert_outbox_matches_projection_and_receipts(
    records: &[ReplayRecord],
    receipts: &[NetworkRuntimeRemoteDeliveryReceiptRecord],
    candidates: &[NetworkRuntimeRemoteDeliveryOutboxCandidate],
) -> Result<(), NetworkRuntimeRemoteDeliveryOutboxHandoffError> {
    if records.is_empty() || receipts.is_empty() || candidates.is_empty() {
        return Err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::EmptyOutbox);
    }
    if records.len() != receipts.len() || records.len() != candidates.len() {
        return Err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::OutboxProjectionMismatch);
    }
    for (index, ((record, receipt), candidate)) in records
        .iter()
        .zip(receipts.iter())
        .zip(candidates.iter())
        .enumerate()
    {
        let expected_sequence = u64::try_from(index)
            .map(|value| value.saturating_add(1))
            .map_err(|_| {
                NetworkRuntimeRemoteDeliveryOutboxHandoffError::OutboxProjectionMismatch
            })?;
        if record.sequence != expected_sequence
            || receipt.sequence != expected_sequence
            || candidate.sequence != expected_sequence
            || candidate.event_id != record.envelope.event_id
            || candidate.event_id != receipt.event_id
            || candidate.event_type != record.envelope.contract.event_type
            || candidate.event_type != receipt.event_type
            || candidate.correlation_id != record.envelope.correlation_id
            || candidate.correlation_id != receipt.correlation_id
            || candidate.idempotency_key != record.envelope.idempotency_key
            || candidate.target_handler != record.envelope.target_handler
            || candidate.state != NetworkRuntimeRemoteDeliveryOutboxState::PreparedNotDispatched
            || candidate.event_chain_export_ref.as_str()
                != constants::network_flow::TEST_REMOTE_EVENT_CHAIN_EXPORT_REF
            || candidate.receipt_ledger_ref.as_str()
                != constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF
            || candidate.outbox_ref.as_str()
                != constants::network_flow::TEST_REMOTE_EVENT_CHAIN_OUTBOX_REF
            || candidate.handoff_ref.as_str()
                != constants::network_flow::TEST_REMOTE_EVENT_CHAIN_HANDOFF_REF
        {
            return Err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::OutboxProjectionMismatch);
        }
    }
    Ok(())
}

fn unique_event_id_count(candidates: &[NetworkRuntimeRemoteDeliveryOutboxCandidate]) -> usize {
    candidates
        .iter()
        .map(|candidate| candidate.event_id.as_str().to_string())
        .collect::<BTreeSet<String>>()
        .len()
}

fn unique_idempotency_key_count(
    candidates: &[NetworkRuntimeRemoteDeliveryOutboxCandidate],
) -> usize {
    candidates
        .iter()
        .map(|candidate| candidate.idempotency_key.as_str().to_string())
        .collect::<BTreeSet<String>>()
        .len()
}

fn target_handler_count(candidates: &[NetworkRuntimeRemoteDeliveryOutboxCandidate]) -> usize {
    candidates
        .iter()
        .filter(|candidate| candidate.target_handler.is_some())
        .count()
}

fn receipt_ref_count(receipts: &[NetworkRuntimeRemoteDeliveryReceiptRecord]) -> usize {
    receipts
        .iter()
        .filter(|receipt| {
            receipt.local_receipt_ack_ref.as_str()
                == constants::network_flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF
        })
        .count()
}

fn broker_requirement_ref_count(report: &super::NetworkRuntimeRemoteDeliveryStatusReport) -> usize {
    [
        report.custody_proof_ref.as_str(),
        report.publisher_auth_ref.as_str(),
        report.subscriber_auth_ref.as_str(),
        report.encryption_ref.as_str(),
        report.retention_policy_ref.as_str(),
        report.replay_plan_ref.as_str(),
        report.deletion_plan_ref.as_str(),
        report.offset_policy_ref.as_str(),
        report.dedupe_policy_ref.as_str(),
        report.transport_config_ref.as_str(),
        report.relay_identity_ref.as_str(),
        report.relay_policy_ref.as_str(),
    ]
    .len()
}
