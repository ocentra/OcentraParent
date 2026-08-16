use std::collections::BTreeSet;

use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_durable_envelope::prove_network_runtime_remote_delivery_durable_envelope;
use super::remote_delivery_durable_envelope_types::{
    NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord,
    NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
};
use super::remote_delivery_event_chain_store::source_component;
use super::remote_delivery_outbox_handoff_types::{
    NetworkRuntimeRemoteDeliveryOutboxCandidate, NetworkRuntimeRemoteDeliveryOutboxHandoffError,
    NetworkRuntimeRemoteDeliveryOutboxHandoffReport, NetworkRuntimeRemoteDeliveryOutboxState,
};
use super::remote_delivery_receipt_ledger_types::NetworkRuntimeRemoteDeliveryReceiptRecord;

pub async fn prove_network_runtime_remote_delivery_outbox_handoff() -> Result<
    NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    NetworkRuntimeRemoteDeliveryOutboxHandoffError,
> {
    let durable_envelope = prove_network_runtime_remote_delivery_durable_envelope()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::DurableEnvelope)?;
    if has_unsupported_claims(&durable_envelope) {
        return Err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::UnsupportedClaim);
    }
    let candidates = outbox_candidates_from_durable_records(&durable_envelope.durable_records)?;
    let duplicate_durable_envelope_rejected =
        duplicate_durable_envelope_rejected(&durable_envelope.durable_records);
    assert_outbox_matches_durable_envelopes_and_receipts(
        &durable_envelope.durable_records,
        &durable_envelope.receipt_ledger.receipts,
        &candidates,
    )?;
    build_outbox_handoff_report(
        durable_envelope,
        candidates,
        duplicate_durable_envelope_rejected,
    )
}

fn build_outbox_handoff_report(
    durable_envelope: NetworkRuntimeRemoteDeliveryDurableEnvelopeReport,
    candidates: Vec<NetworkRuntimeRemoteDeliveryOutboxCandidate>,
    duplicate_durable_envelope_rejected: bool,
) -> Result<
    NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
    NetworkRuntimeRemoteDeliveryOutboxHandoffError,
> {
    Ok(NetworkRuntimeRemoteDeliveryOutboxHandoffReport {
        durable_envelope_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF,
        )?,
        durable_store_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_STORE_REF,
        )?,
        outbox_ref: source_component(constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_REF)?,
        handoff_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_HANDOFF_REF,
        )?,
        outbox_replay_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_REPLAY_REF,
        )?,
        outbox_support_status_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_SUPPORT_STATUS_REF,
        )?,
        source_durable_envelope_count: durable_envelope.durable_envelope_count,
        source_receipt_record_count: durable_envelope.source_receipt_record_count,
        outbox_candidate_count: candidates.len(),
        prepared_not_dispatched_count: candidates.len(),
        dispatch_attempt_count: 0,
        remote_ack_count: 0,
        duplicate_durable_envelope_rejected,
        outbox_candidates_match_durable_envelopes: true,
        outbox_candidates_match_receipts: true,
        sequence_gap_count: 0,
        event_id_mismatch_count: 0,
        event_type_mismatch_count: 0,
        correlation_mismatch_count: 0,
        unique_event_id_count: unique_event_id_count(&candidates),
        unique_correlation_id_count: unique_correlation_id_count(&candidates),
        broker_delivery_implemented: durable_envelope.broker_delivery_implemented,
        family_hub_delivery_implemented: durable_envelope.family_hub_delivery_implemented,
        remote_delivery_ack_implemented: durable_envelope.remote_delivery_ack_implemented,
        provider_delivery_implemented: durable_envelope.provider_delivery_implemented,
        child_device_delivery_implemented: durable_envelope.child_device_delivery_implemented,
        remote_delete_export_propagation_implemented: durable_envelope
            .remote_delete_export_propagation_implemented,
        product_ready_remote_delivery: durable_envelope.product_ready_remote_delivery,
        policy_authority: durable_envelope.policy_authority,
        side_effect_authority: durable_envelope.side_effect_authority,
        enforcement_command_event_count: durable_envelope.enforcement_command_event_count,
        adapter_action_executed_count: durable_envelope.adapter_action_executed_count,
        raw_pcap_available_count: durable_envelope.raw_pcap_available_count,
        exact_url_available_count: durable_envelope.exact_url_available_count,
        decrypted_payload_available_count: durable_envelope.decrypted_payload_available_count,
        page_content_available_count: durable_envelope.page_content_available_count,
        video_content_available_count: durable_envelope.video_content_available_count,
        private_message_content_available_count: durable_envelope
            .private_message_content_available_count,
        search_query_available_count: durable_envelope.search_query_available_count,
        durable_envelope,
        candidates,
    })
}

fn outbox_candidates_from_durable_records(
    durable_records: &[NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord],
) -> Result<
    Vec<NetworkRuntimeRemoteDeliveryOutboxCandidate>,
    NetworkRuntimeRemoteDeliveryOutboxHandoffError,
> {
    if durable_records.is_empty() {
        return Err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::EmptyOutbox);
    }
    assert_unique_outbox_candidate_inputs(durable_records)?;
    durable_records
        .iter()
        .map(outbox_candidate_from_durable_record)
        .collect::<Result<Vec<NetworkRuntimeRemoteDeliveryOutboxCandidate>, _>>()
}

fn assert_unique_outbox_candidate_inputs(
    durable_records: &[NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord],
) -> Result<(), NetworkRuntimeRemoteDeliveryOutboxHandoffError> {
    let mut sequences = BTreeSet::new();
    let mut event_ids = BTreeSet::new();
    for durable_record in durable_records {
        if !sequences.insert(durable_record.sequence)
            || !event_ids.insert(durable_record.event_id.as_str().to_string())
        {
            return Err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::DuplicateOutboxCandidate);
        }
    }
    Ok(())
}

fn duplicate_durable_envelope_rejected(
    durable_records: &[NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord],
) -> bool {
    let Some(first_record) = durable_records.first() else {
        return false;
    };
    let mut duplicated_records = durable_records.to_vec();
    duplicated_records.push(first_record.clone());
    matches!(
        outbox_candidates_from_durable_records(&duplicated_records),
        Err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::DuplicateOutboxCandidate)
    )
}

fn outbox_candidate_from_durable_record(
    durable_record: &NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord,
) -> Result<
    NetworkRuntimeRemoteDeliveryOutboxCandidate,
    NetworkRuntimeRemoteDeliveryOutboxHandoffError,
> {
    Ok(NetworkRuntimeRemoteDeliveryOutboxCandidate {
        sequence: durable_record.sequence,
        event_id: durable_record.event_id.clone(),
        event_type: durable_record.event_type.clone(),
        correlation_id: durable_record.correlation_id.clone(),
        state: NetworkRuntimeRemoteDeliveryOutboxState::PreparedNotDispatched,
        durable_envelope_ref: durable_record.durable_envelope_ref.clone(),
        durable_store_ref: durable_record.durable_store_ref.clone(),
        receipt_ledger_ref: durable_record.receipt_ledger_ref.clone(),
        local_receipt_ack_ref: durable_record.local_receipt_ack_ref.clone(),
        outbox_ref: source_component(constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_REF)?,
        handoff_ref: source_component(
            constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_HANDOFF_REF,
        )?,
    })
}

fn assert_outbox_matches_durable_envelopes_and_receipts(
    durable_records: &[NetworkRuntimeRemoteDeliveryDurableEnvelopeRecord],
    receipts: &[NetworkRuntimeRemoteDeliveryReceiptRecord],
    candidates: &[NetworkRuntimeRemoteDeliveryOutboxCandidate],
) -> Result<(), NetworkRuntimeRemoteDeliveryOutboxHandoffError> {
    if durable_records.is_empty() || receipts.is_empty() || candidates.is_empty() {
        return Err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::EmptyOutbox);
    }
    if durable_records.len() != receipts.len() || durable_records.len() != candidates.len() {
        return Err(NetworkRuntimeRemoteDeliveryOutboxHandoffError::OutboxDurableEnvelopeMismatch);
    }
    for (index, ((durable_record, receipt), candidate)) in durable_records
        .iter()
        .zip(receipts.iter())
        .zip(candidates.iter())
        .enumerate()
    {
        let expected_sequence =
            match u64::try_from(index) {
                Ok(value) => value.saturating_add(1),
                Err(_) => return Err(
                    NetworkRuntimeRemoteDeliveryOutboxHandoffError::OutboxDurableEnvelopeMismatch,
                ),
            };
        if durable_record.sequence != expected_sequence
            || receipt.sequence != expected_sequence
            || candidate.sequence != expected_sequence
            || candidate.event_id != durable_record.event_id
            || candidate.event_id != receipt.event_id
            || candidate.event_type != durable_record.event_type
            || candidate.event_type != receipt.event_type
            || candidate.correlation_id != durable_record.correlation_id
            || candidate.correlation_id != receipt.correlation_id
            || candidate.state != NetworkRuntimeRemoteDeliveryOutboxState::PreparedNotDispatched
            || candidate.durable_envelope_ref != durable_record.durable_envelope_ref
            || candidate.durable_store_ref != durable_record.durable_store_ref
            || candidate.receipt_ledger_ref != durable_record.receipt_ledger_ref
            || candidate.local_receipt_ack_ref != durable_record.local_receipt_ack_ref
            || candidate.outbox_ref.as_str()
                != constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_REF
            || candidate.handoff_ref.as_str()
                != constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_HANDOFF_REF
        {
            return Err(
                NetworkRuntimeRemoteDeliveryOutboxHandoffError::OutboxDurableEnvelopeMismatch,
            );
        }
    }
    Ok(())
}

fn has_unsupported_claims(report: &NetworkRuntimeRemoteDeliveryDurableEnvelopeReport) -> bool {
    report.broker_delivery_implemented
        || report.family_hub_delivery_implemented
        || report.remote_delivery_ack_implemented
        || report.provider_delivery_implemented
        || report.child_device_delivery_implemented
        || report.remote_delete_export_propagation_implemented
        || report.product_ready_remote_delivery
        || report.policy_authority
        || report.side_effect_authority
        || report.enforcement_command_event_count > 0
        || report.adapter_action_executed_count > 0
        || report.raw_pcap_available_count > 0
        || report.exact_url_available_count > 0
        || report.decrypted_payload_available_count > 0
        || report.page_content_available_count > 0
        || report.video_content_available_count > 0
        || report.private_message_content_available_count > 0
        || report.search_query_available_count > 0
}

fn unique_event_id_count(candidates: &[NetworkRuntimeRemoteDeliveryOutboxCandidate]) -> usize {
    candidates
        .iter()
        .map(|candidate| candidate.event_id.as_str().to_string())
        .collect::<BTreeSet<String>>()
        .len()
}

fn unique_correlation_id_count(
    candidates: &[NetworkRuntimeRemoteDeliveryOutboxCandidate],
) -> usize {
    candidates
        .iter()
        .map(|candidate| candidate.correlation_id.as_str().to_string())
        .collect::<BTreeSet<String>>()
        .len()
}
