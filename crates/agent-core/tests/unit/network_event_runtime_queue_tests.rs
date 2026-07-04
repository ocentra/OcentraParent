use std::fmt::{Debug, Display};
use std::time::Duration;

use ocentra_eventing::{
    bus::reports::dead_letter::DeadLetterReason, error::EventingError,
    queue::policy::QueueDisposition,
};
use ocentra_parent_agent_protocol::activity_capture::{
    ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::{NetworkEvidenceScope, NetworkRuntimePhase};

use crate::test_text::TestText;
use ocentra_parent_agent_core::network_capture::NetworkObservation;
use ocentra_parent_agent_core::network_event_runtime::queue::{
    queue_network_runtime_flow_expires_before_drain,
    queue_network_runtime_flow_overflow_dead_letters,
    queue_network_runtime_flow_rejects_duplicate_idempotency,
    queue_network_runtime_flow_until_subscriber, NetworkRuntimeQueueDrainReport,
    NetworkRuntimeQueueIdempotencyReport, NetworkRuntimeQueueOverflowReport,
    NetworkRuntimeQueueTtlReport,
};
use ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeEventPayload;

type TestResult = Result<(), TestText>;

fn ok<T, E: Debug>(result: Result<T, E>, context: impl Display) -> Result<T, TestText> {
    result.map_err(|error| TestText::from_display(format!("{context}: {error:?}")))
}

#[tokio::test]
async fn network_runtime_queues_flow_until_subscriber_drains() -> TestResult {
    let report: NetworkRuntimeQueueDrainReport = ok(
        queue_network_runtime_flow_until_subscriber(
            complete_domain_observation(),
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
        )
        .await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_QUEUE_DRAINS,
    )?;

    assert_eq!(
        report.queued_publish_report.event_type.as_str(),
        constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED
    );
    assert_eq!(
        report.queued_publish_report.queue_report.disposition,
        QueueDisposition::QueuedNoSubscriber
    );
    assert_eq!(report.queued_publish_report.queue_report.queued_count, 1);
    assert_eq!(report.queued_publish_report.subscriber_count, 0);
    assert_eq!(report.drain_report.queued_before, 1);
    assert_eq!(report.drain_report.dispatched_count, 1);
    assert_eq!(report.drain_report.expired_count, 0);
    assert_eq!(report.drain_report.remaining_count, 0);
    assert_eq!(report.drain_report.dispatch_reports[0].handled_count, 1);
    assert_eq!(
        report.drain_report.dispatch_reports[0].event_type.as_str(),
        constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED
    );
    assert_eq!(report.stored_events.len(), 1);
    assert!(report.dead_letters.is_empty());

    Ok(())
}

#[tokio::test]
async fn network_runtime_queue_overflow_dead_letters_oldest_flow() -> TestResult {
    let report: NetworkRuntimeQueueOverflowReport = ok(
        queue_network_runtime_flow_overflow_dead_letters(
            complete_domain_observation(),
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            complete_domain_observation(),
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        )
        .await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_QUEUE_OVERFLOW_DEAD_LETTERS,
    )?;
    let payloads = decode_stored_payloads(&report.stored_events)?;

    assert_eq!(
        report.first_publish_report.queue_report.disposition,
        QueueDisposition::QueuedNoSubscriber
    );
    assert_eq!(report.first_publish_report.queue_report.queued_count, 1);
    assert_eq!(
        report.overflow_publish_report.queue_report.disposition,
        QueueDisposition::DeadLetteredQueueOverflow
    );
    assert_eq!(report.overflow_publish_report.queue_report.queued_count, 1);
    assert_eq!(report.overflow_publish_report.dead_letter_count, 1);
    assert_eq!(report.dead_letters.len(), 1);
    assert_eq!(
        report.dead_letters[0].reason,
        DeadLetterReason::QueueOverflow
    );
    assert_eq!(
        report.dead_letters[0].envelope.observed_at.as_str(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT
    );
    assert_eq!(report.stored_events.len(), 2);
    assert!(payloads.iter().all(|payload| {
        payload.phase == NetworkRuntimePhase::FlowObserved
            && payload.evidence_scope == NetworkEvidenceScope::MetadataOnly
            && !payload.claim_boundary.exact_url_available
            && !payload.claim_boundary.adapter_action_executed
    }));

    Ok(())
}

#[tokio::test]
async fn network_runtime_queue_ttl_expires_before_dispatch() -> TestResult {
    let report: NetworkRuntimeQueueTtlReport = ok(
        queue_network_runtime_flow_expires_before_drain(
            complete_domain_observation(),
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            Duration::from_millis(5),
            Duration::from_millis(10),
        )
        .await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_QUEUE_TTL_EXPIRES,
    )?;
    let payloads = decode_stored_payloads(&report.stored_events)?;

    assert_eq!(
        report.queued_publish_report.queue_report.disposition,
        QueueDisposition::QueuedNoSubscriber
    );
    assert_eq!(report.drain_report.queued_before, 1);
    assert_eq!(report.drain_report.dispatched_count, 0);
    assert_eq!(report.drain_report.expired_count, 1);
    assert_eq!(report.drain_report.remaining_count, 0);
    assert!(report.drain_report.dispatch_reports.is_empty());
    assert_eq!(report.dead_letters.len(), 1);
    assert_eq!(
        report.dead_letters[0].reason,
        DeadLetterReason::QueueExpired
    );
    assert_eq!(payloads.len(), 1);
    assert_eq!(payloads[0].phase, NetworkRuntimePhase::FlowObserved);
    assert!(!payloads[0].claim_boundary.adapter_action_executed);

    Ok(())
}

#[tokio::test]
async fn network_runtime_queue_idempotency_rejects_queued_and_completed_duplicates() -> TestResult {
    let report: NetworkRuntimeQueueIdempotencyReport = ok(
        queue_network_runtime_flow_rejects_duplicate_idempotency(
            complete_domain_observation(),
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
        )
        .await,
        constants::network_flow::ERROR_NETWORK_RUNTIME_QUEUE_IDEMPOTENCY_REJECTS,
    )?;

    assert_eq!(
        report.first_publish_report.queue_report.disposition,
        QueueDisposition::QueuedNoSubscriber
    );
    assert!(duplicate_idempotency_error_mentions_network_flow(
        &report.queued_duplicate_error
    ));
    assert_eq!(report.drain_report.queued_before, 1);
    assert_eq!(report.drain_report.dispatched_count, 1);
    assert_eq!(report.drain_report.dispatch_reports[0].handled_count, 1);
    assert!(duplicate_idempotency_error_mentions_network_flow(
        &report.completed_duplicate_error
    ));
    assert_eq!(report.stored_events.len(), 1);
    assert!(report.dead_letters.is_empty());

    Ok(())
}

fn complete_domain_observation() -> NetworkObservation {
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
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

fn decode_stored_payloads(
    stored_events: &[ocentra_eventing::envelope::StoredEventEnvelope],
) -> Result<Vec<NetworkRuntimeEventPayload>, TestText> {
    stored_events
        .iter()
        .map(|event| {
            ok(
                event.decode::<NetworkRuntimeEventPayload>(),
                constants::network_flow::ERROR_NETWORK_RUNTIME_PAYLOAD_DECODES,
            )
            .map(|envelope| envelope.payload)
        })
        .collect()
}

fn duplicate_idempotency_error_mentions_network_flow(error: &EventingError) -> bool {
    matches!(
        error,
        EventingError::DuplicateIdempotencyKey { idempotency_key }
            if idempotency_key
                .as_str()
                .contains(constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED)
    )
}
