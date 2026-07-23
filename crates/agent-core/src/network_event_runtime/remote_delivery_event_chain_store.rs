use std::{collections::BTreeSet, path::PathBuf};

use ocentra_eventing::{
    bus::subscriber::EventSubscriber, bus::EventBus, envelope::StoredEventEnvelope,
    error::EventingError, ids::EventType, ids::SourceComponent, ids::SubscriberId,
    ids::TargetHandler, journal::ndjson::NdjsonEventJournal, journal::ndjson::NdjsonJournalOptions,
    journal::policy::JournalPolicy, journal::policy::JournalSelector, replay::ReplayFilter,
    replay::ReplayReadReport, replay::ReplayRecord,
};
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::NetworkRuntimePhase;

use crate::NetworkObservation;

use super::remote_delivery_event_chain_journal_types::{
    count_payloads, NetworkRuntimeRemoteEventChainJournalError, UnsupportedClaimCounts,
};
use super::{
    network_event_metadata, should_publish_phase_for_runtime_decision, NetworkRuntimeEventPayload,
};

pub(super) struct NetworkRuntimeRemoteEventChainStore {
    pub stored_events: Vec<StoredEventEnvelope>,
    pub projection: ReplayReadReport,
    pub payloads: Vec<NetworkRuntimeEventPayload>,
}

pub(super) async fn publish_network_runtime_remote_event_chain_store(
) -> Result<NetworkRuntimeRemoteEventChainStore, NetworkRuntimeRemoteEventChainJournalError> {
    let journal = NdjsonEventJournal::with_options(
        remote_event_chain_journal_path(),
        NdjsonJournalOptions::hash_chain(),
    );
    let bus = event_chain_bus_with_journal(&journal).await?;
    let observation = remote_event_chain_observation();
    let decision = super::network_runtime_decision_from_observation(&observation);
    for phase in NetworkRuntimePhase::ordered_chain()
        .iter()
        .copied()
        .filter(|phase| should_publish_phase_for_runtime_decision(*phase, &observation, &decision))
    {
        let payload = super::network_runtime_event_payload_from_observation(
            phase,
            &observation,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            decision,
        );
        let metadata = network_event_metadata(
            phase,
            &observation,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            phase.target_handler(),
        )?;
        bus.publish(payload, metadata).await?;
    }

    let stored_events = bus.journal().await;
    let projection = journal.replay_projection(ReplayFilter::all()).await?;
    cleanup_journal(journal.path());
    assert_projection_matches(&stored_events, &projection.records)?;
    let payloads = decode_payloads(&projection.records)?;
    Ok(NetworkRuntimeRemoteEventChainStore {
        stored_events,
        projection,
        payloads,
    })
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

async fn event_chain_bus_with_journal(
    journal: &NdjsonEventJournal,
) -> Result<EventBus, EventingError> {
    let bus = EventBus::with_journal(
        JournalPolicy::after_dispatch(JournalSelector::All),
        journal.clone().shared(),
    );
    for phase in NetworkRuntimePhase::ordered_chain() {
        bus.subscribe::<NetworkRuntimeEventPayload, _, _>(
            EventSubscriber::new(
                SubscriberId::parse(phase.subscriber_id())?,
                EventType::parse(phase.event_type())?,
                TargetHandler::parse(phase.target_handler())?,
            ),
            |_| async { Ok(()) },
        )
        .await?;
    }
    Ok(bus)
}

fn assert_projection_matches(
    stored_events: &[StoredEventEnvelope],
    records: &[ReplayRecord],
) -> Result<(), NetworkRuntimeRemoteEventChainJournalError> {
    if stored_events.is_empty() || records.is_empty() {
        return Err(NetworkRuntimeRemoteEventChainJournalError::EmptyJournal);
    }
    if stored_events.len() != records.len() {
        return Err(NetworkRuntimeRemoteEventChainJournalError::ReplayMismatch);
    }
    for (index, (stored_event, record)) in stored_events.iter().zip(records.iter()).enumerate() {
        let expected_sequence = match u64::try_from(index) {
            Ok(value) => value.saturating_add(1),
            Err(_) => return Err(NetworkRuntimeRemoteEventChainJournalError::ReplayMismatch),
        };
        if record.sequence != expected_sequence || &record.envelope != stored_event {
            return Err(NetworkRuntimeRemoteEventChainJournalError::ReplayMismatch);
        }
    }
    Ok(())
}

fn decode_payloads(
    records: &[ReplayRecord],
) -> Result<Vec<NetworkRuntimeEventPayload>, EventingError> {
    records
        .iter()
        .map(|record| record.envelope.decode().map(|envelope| envelope.payload))
        .collect::<Result<Vec<NetworkRuntimeEventPayload>, EventingError>>()
}

fn remote_event_chain_observation() -> NetworkObservation {
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Unavailable,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        destination_ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
        destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        tcp_state: Some(ActivityNetworkTcpState::Established),
        pid: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        associated_pid_count: 1,
    }
}

fn remote_event_chain_journal_path() -> PathBuf {
    let mut file_name =
        String::from(constants::network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_PATH_PREFIX);
    file_name.push(constants::delimiter::HYPHEN);
    file_name.push_str(&std::process::id().to_string());
    file_name.push(constants::delimiter::HYPHEN);
    file_name.push_str(ocentra_eventing::ids::EventId::generated().as_str());
    file_name.push(constants::delimiter::DOT);
    file_name.push_str(constants::network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_EXTENSION);
    std::env::temp_dir().join(file_name)
}

fn cleanup_journal(path: &std::path::Path) {
    let _ = std::fs::remove_file(path);
}
