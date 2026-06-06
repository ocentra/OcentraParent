use std::{collections::BTreeSet, path::PathBuf};

use ocentra_eventing::{
    EventBus, EventSubscriber, EventType, EventingError, JournalPolicy, JournalSelector,
    NdjsonEventJournal, NdjsonJournalOptions, ReplayFilter, ReplayReadReport, ReplayRecord,
    SourceComponent, StoredEventEnvelope, SubscriberId, TargetHandler,
};
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};

use crate::{
    network_event_runtime_phase::NetworkRuntimePhase,
    network_event_runtime_state::NetworkInterventionState, NetworkObservation,
};

use super::remote_delivery_event_chain_journal_types::{
    count_payloads, NetworkRuntimeRemoteEventChainJournalError,
    NetworkRuntimeRemoteEventChainJournalReport, UnsupportedClaimCounts,
};
use super::{
    network_event_metadata, prove_network_runtime_remote_delivery_status, should_publish_phase,
    NetworkRuntimeEventPayload, NetworkRuntimeRemoteDeliveryStatusReport,
};
pub async fn prove_network_runtime_remote_event_chain_journal(
) -> Result<NetworkRuntimeRemoteEventChainJournalReport, NetworkRuntimeRemoteEventChainJournalError>
{
    let remote_delivery_status = prove_network_runtime_remote_delivery_status()
        .await
        .map_err(NetworkRuntimeRemoteEventChainJournalError::RemoteDeliveryStatus)?;
    let (stored_events, projection) = publish_event_chain_to_journal().await?;
    assert_projection_matches(&stored_events, &projection.records)?;
    let payloads = decode_payloads(&projection.records)?;
    let unsupported = unsupported_claim_counts(&payloads, &projection.records);
    if unsupported.has_any() {
        return Err(NetworkRuntimeRemoteEventChainJournalError::UnsupportedClaim);
    }
    build_report(
        remote_delivery_status,
        &stored_events,
        projection,
        payloads,
        unsupported,
    )
}
async fn publish_event_chain_to_journal(
) -> Result<(Vec<StoredEventEnvelope>, ReplayReadReport), NetworkRuntimeRemoteEventChainJournalError>
{
    let journal = NdjsonEventJournal::with_options(
        remote_event_chain_journal_path(),
        NdjsonJournalOptions::hash_chain(),
    );
    let bus = event_chain_bus_with_journal(&journal).await?;
    let observation = remote_event_chain_observation();
    for phase in NetworkRuntimePhase::ordered_chain()
        .iter()
        .copied()
        .filter(|phase| should_publish_phase(*phase, &observation))
    {
        let payload = NetworkRuntimeEventPayload::from_observation(
            phase,
            &observation,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
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
    Ok((stored_events, projection))
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
        let expected_sequence = u64::try_from(index)
            .map(|value| value.saturating_add(1))
            .map_err(|_| NetworkRuntimeRemoteEventChainJournalError::ReplayMismatch)?;
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
fn unsupported_claim_counts(
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
        exact_url_available_count: count_payloads(payloads, |payload| {
            payload.claim_boundary.exact_url_available
        }),
        decrypted_payload_available_count: count_payloads(payloads, |payload| {
            payload.claim_boundary.decrypted_https_payload_available
        }),
        page_content_available_count: count_payloads(payloads, |payload| {
            payload.claim_boundary.page_content_available
        }),
    }
}
fn build_report(
    remote_delivery_status: NetworkRuntimeRemoteDeliveryStatusReport,
    stored_events: &[StoredEventEnvelope],
    projection: ReplayReadReport,
    payloads: Vec<NetworkRuntimeEventPayload>,
    unsupported: UnsupportedClaimCounts,
) -> Result<NetworkRuntimeRemoteEventChainJournalReport, NetworkRuntimeRemoteEventChainJournalError>
{
    if stored_events.is_empty() || projection.records.is_empty() {
        return Err(NetworkRuntimeRemoteEventChainJournalError::EmptyJournal);
    }
    if stored_events.len() != projection.records.len() {
        return Err(NetworkRuntimeRemoteEventChainJournalError::ReplayMismatch);
    }
    if unsupported.has_any() {
        return Err(NetworkRuntimeRemoteEventChainJournalError::UnsupportedClaim);
    }
    Ok(NetworkRuntimeRemoteEventChainJournalReport {
        event_chain_journal_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF,
        )?,
        event_chain_replay_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_REPLAY_REF,
        )?,
        event_chain_export_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_EXPORT_REF,
        )?,
        event_chain_support_status_ref: source_component(
            constants::network_flow::TEST_REMOTE_EVENT_CHAIN_SUPPORT_STATUS_REF,
        )?,
        stored_event_count: stored_events.len(),
        journal_entry_count: projection.records.len(),
        projection_replay_record_count: projection.records.len(),
        replay_cursor_next_sequence: projection.cursor.next_sequence,
        exported_event_type_count: exported_event_type_count(&projection.records),
        exportable_remote_envelope_count: projection.records.len(),
        unavailable_event_count: payloads
            .iter()
            .filter(|payload| payload.intervention_state == NetworkInterventionState::Unavailable)
            .count(),
        enforcement_command_event_count: unsupported.enforcement_command_event_count,
        adapter_action_executed_count: unsupported.adapter_action_executed_count,
        exact_url_available_count: unsupported.exact_url_available_count,
        decrypted_payload_available_count: unsupported.decrypted_payload_available_count,
        page_content_available_count: unsupported.page_content_available_count,
        projection_replay_mode: projection.mode,
        durable_envelope_ready: remote_delivery_status.durable_envelope_ready,
        broker_delivery_implemented: remote_delivery_status.external_transport_delivery_implemented,
        family_hub_delivery_implemented: remote_delivery_status.family_hub_delivery_implemented,
        provider_delivery_implemented: remote_delivery_status.provider_delivery_implemented,
        child_device_delivery_implemented: remote_delivery_status.child_device_delivery_implemented,
        product_ready_claimed: remote_delivery_status.product_ready_claimed,
        policy_authority: remote_delivery_status.policy_authority,
        side_effect_authority: remote_delivery_status.side_effect_authority,
        remote_delivery_status,
    })
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
    file_name.push_str(ocentra_eventing::EventId::generated().as_str());
    file_name.push(constants::delimiter::DOT);
    file_name.push_str(constants::network_flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_EXTENSION);
    std::env::temp_dir().join(file_name)
}
fn exported_event_type_count(records: &[ocentra_eventing::ReplayRecord]) -> usize {
    records
        .iter()
        .map(|record| record.envelope.contract.event_type.as_str().to_string())
        .collect::<BTreeSet<String>>()
        .len()
}
fn source_component(value: &str) -> Result<SourceComponent, EventingError> {
    SourceComponent::parse(value)
}
fn cleanup_journal(path: &std::path::Path) {
    let _ = std::fs::remove_file(path);
}
