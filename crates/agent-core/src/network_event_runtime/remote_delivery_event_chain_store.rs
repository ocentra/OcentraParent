use std::collections::BTreeSet;

use ocentra_eventing::{
    envelope::StoredEventEnvelope, error::EventingError, ids::SourceComponent,
    replay::ReplayReadReport, replay::ReplayRecord,
};
use ocentra_parent_agent_protocol::constants;

use super::remote_delivery_event_chain_journal_types::{
    count_payloads, NetworkRuntimeRemoteEventChainJournalError, UnsupportedClaimCounts,
};
use super::NetworkRuntimeEventPayload;

pub(super) struct NetworkRuntimeRemoteEventChainStore {
    pub stored_events: Vec<StoredEventEnvelope>,
    pub projection: ReplayReadReport,
    pub payloads: Vec<NetworkRuntimeEventPayload>,
}

pub(super) async fn publish_network_runtime_remote_event_chain_store(
) -> Result<NetworkRuntimeRemoteEventChainStore, NetworkRuntimeRemoteEventChainJournalError> {
    Err(NetworkRuntimeRemoteEventChainJournalError::RuntimeOwnerUnavailable)
}

pub(super) fn unsupported_claim_counts(
    payloads: &[NetworkRuntimeEventPayload],
    records: &[ReplayRecord],
) -> UnsupportedClaimCounts {
    UnsupportedClaimCounts {
        enforcement_command_event_count: records
            .iter()
            .filter(|record| {
                record.envelope.contract.event_type.as_str()
                    == constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
            })
            .count(),
        adapter_action_executed_count: count_payloads(payloads, |payload| {
            payload.claim_boundary.adapter_action_executed
        }),
        raw_pcap_available_count: count_payloads(payloads, |payload| {
            payload.claim_boundary.raw_pcap_available
        }),
        exact_url_available_count: count_payloads(payloads, |payload| {
            payload.claim_boundary.exact_url_available
        }),
        decrypted_payload_available_count: count_payloads(payloads, |payload| {
            payload.claim_boundary.decrypted_https_payload_available
        }),
        page_content_available_count: count_payloads(payloads, |payload| {
            payload.claim_boundary.page_content_available
        }),
        video_content_available_count: count_payloads(payloads, |payload| {
            payload.claim_boundary.video_content_available
        }),
        private_message_content_available_count: count_payloads(payloads, |payload| {
            payload.claim_boundary.private_message_content_available
        }),
        search_query_available_count: count_payloads(payloads, |payload| {
            payload.claim_boundary.search_query_available
        }),
    }
}

pub(super) fn exported_event_type_count(records: &[ReplayRecord]) -> usize {
    records
        .iter()
        .map(|record| record.envelope.contract.event_type.as_str().to_string())
        .collect::<BTreeSet<String>>()
        .len()
}

pub(super) fn source_component(value: &str) -> Result<SourceComponent, EventingError> {
    SourceComponent::parse(value)
}
