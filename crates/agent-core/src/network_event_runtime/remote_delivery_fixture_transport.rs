use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_event_chain_store::source_component;
use super::remote_delivery_fixture_transport_types::{
    NetworkRuntimeRemoteDeliveryFixtureTransportError,
    NetworkRuntimeRemoteDeliveryFixtureTransportRecord,
    NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    NetworkRuntimeRemoteDeliveryFixtureTransportState,
};
use super::remote_delivery_outbox_handoff::prove_network_runtime_remote_delivery_outbox_handoff;
use super::remote_delivery_outbox_handoff_types::{
    NetworkRuntimeRemoteDeliveryOutboxCandidate, NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
};

pub async fn prove_network_runtime_remote_delivery_fixture_transport() -> Result<
    NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    NetworkRuntimeRemoteDeliveryFixtureTransportError,
> {
    let outbox_handoff = prove_network_runtime_remote_delivery_outbox_handoff()
        .await
        .map_err(NetworkRuntimeRemoteDeliveryFixtureTransportError::OutboxHandoff)?;
    prove_network_runtime_remote_delivery_fixture_transport_from_outbox(outbox_handoff)
}

pub fn prove_network_runtime_remote_delivery_fixture_transport_from_outbox(
    outbox_handoff: NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
) -> Result<
    NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    NetworkRuntimeRemoteDeliveryFixtureTransportError,
> {
    build_fixture_transport_report(outbox_handoff)
}

fn build_fixture_transport_report(
    outbox_handoff: NetworkRuntimeRemoteDeliveryOutboxHandoffReport,
) -> Result<
    NetworkRuntimeRemoteDeliveryFixtureTransportReport,
    NetworkRuntimeRemoteDeliveryFixtureTransportError,
> {
    if outbox_handoff.candidates.is_empty() {
        return Err(NetworkRuntimeRemoteDeliveryFixtureTransportError::EmptyOutbox);
    }
    if has_unsupported_claims(&outbox_handoff) {
        return Err(NetworkRuntimeRemoteDeliveryFixtureTransportError::UnsupportedClaim);
    }

    let fixture_transport_ref =
        source_component(constants::network_flow::TEST_REMOTE_DELIVERY_FIXTURE_TRANSPORT_REF)?;
    let fixture_dispatch_attempt_ref = source_component(
        constants::network_flow::TEST_REMOTE_DELIVERY_FIXTURE_DISPATCH_ATTEMPT_REF,
    )?;
    let fixture_ack_ref =
        source_component(constants::network_flow::TEST_REMOTE_DELIVERY_FIXTURE_ACK_REF)?;
    let records = outbox_handoff
        .candidates
        .iter()
        .map(|candidate| {
            fixture_record(
                candidate,
                &fixture_transport_ref,
                &fixture_dispatch_attempt_ref,
                &fixture_ack_ref,
            )
        })
        .collect::<Vec<_>>();

    if !fixture_records_match_outbox_candidates(&records, &outbox_handoff.candidates) {
        return Err(NetworkRuntimeRemoteDeliveryFixtureTransportError::FixtureRecordMismatch);
    }

    Ok(NetworkRuntimeRemoteDeliveryFixtureTransportReport {
        source_outbox_candidate_count: outbox_handoff.outbox_candidate_count,
        fixture_dispatch_attempt_count: records.len(),
        fixture_remote_ack_count: records.len(),
        fixture_records_match_outbox_candidates: true,
        broker_delivery_implemented: outbox_handoff.broker_delivery_implemented,
        family_hub_delivery_implemented: outbox_handoff.family_hub_delivery_implemented,
        remote_delivery_ack_implemented: outbox_handoff.remote_delivery_ack_implemented,
        provider_delivery_implemented: outbox_handoff.provider_delivery_implemented,
        child_device_delivery_implemented: outbox_handoff.child_device_delivery_implemented,
        remote_delete_export_propagation_implemented: outbox_handoff
            .remote_delete_export_propagation_implemented,
        product_ready_remote_delivery: outbox_handoff.product_ready_remote_delivery,
        policy_authority: outbox_handoff.policy_authority,
        side_effect_authority: outbox_handoff.side_effect_authority,
        enforcement_command_event_count: outbox_handoff.enforcement_command_event_count,
        adapter_action_executed_count: outbox_handoff.adapter_action_executed_count,
        raw_pcap_available_count: outbox_handoff.raw_pcap_available_count,
        exact_url_available_count: outbox_handoff.exact_url_available_count,
        decrypted_payload_available_count: outbox_handoff.decrypted_payload_available_count,
        page_content_available_count: outbox_handoff.page_content_available_count,
        video_content_available_count: outbox_handoff.video_content_available_count,
        private_message_content_available_count: outbox_handoff
            .private_message_content_available_count,
        search_query_available_count: outbox_handoff.search_query_available_count,
        fixture_transport_ref,
        fixture_dispatch_attempt_ref,
        fixture_ack_ref,
        records,
        outbox_handoff,
    })
}

fn fixture_record(
    candidate: &NetworkRuntimeRemoteDeliveryOutboxCandidate,
    fixture_transport_ref: &ocentra_eventing::ids::SourceComponent,
    fixture_dispatch_attempt_ref: &ocentra_eventing::ids::SourceComponent,
    fixture_ack_ref: &ocentra_eventing::ids::SourceComponent,
) -> NetworkRuntimeRemoteDeliveryFixtureTransportRecord {
    NetworkRuntimeRemoteDeliveryFixtureTransportRecord {
        sequence: candidate.sequence,
        event_id: candidate.event_id.clone(),
        event_type: candidate.event_type.clone(),
        correlation_id: candidate.correlation_id.clone(),
        source_outbox_state: candidate.state,
        fixture_state: NetworkRuntimeRemoteDeliveryFixtureTransportState::FixtureAckRecorded,
        outbox_ref: candidate.outbox_ref.clone(),
        handoff_ref: candidate.handoff_ref.clone(),
        fixture_transport_ref: fixture_transport_ref.clone(),
        fixture_dispatch_attempt_ref: fixture_dispatch_attempt_ref.clone(),
        fixture_ack_ref: fixture_ack_ref.clone(),
    }
}

fn fixture_records_match_outbox_candidates(
    records: &[NetworkRuntimeRemoteDeliveryFixtureTransportRecord],
    candidates: &[NetworkRuntimeRemoteDeliveryOutboxCandidate],
) -> bool {
    records.len() == candidates.len()
        && records
            .iter()
            .zip(candidates.iter())
            .all(|(record, candidate)| {
                record.sequence == candidate.sequence
                    && record.event_id == candidate.event_id
                    && record.event_type == candidate.event_type
                    && record.correlation_id == candidate.correlation_id
                    && record.source_outbox_state == candidate.state
                    && record.outbox_ref == candidate.outbox_ref
                    && record.handoff_ref == candidate.handoff_ref
            })
}

fn has_unsupported_claims(report: &NetworkRuntimeRemoteDeliveryOutboxHandoffReport) -> bool {
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
