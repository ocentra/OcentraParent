use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
};

use crate::network_event_runtime::{
    queue_network_runtime_flow_until_subscriber, request_network_runtime_review_for_observation,
    NetworkRuntimeQueueDrainReport, NetworkRuntimeReviewReport, NetworkRuntimeReviewResponse,
};

use super::{
    publish_network_runtime_chain_for_observation, NetworkAiAuditState, NetworkEvidenceGrade,
    NetworkEvidenceScope, NetworkInterventionState, NetworkObservation, NetworkRiskBudgetState,
    NetworkRuntimeEventPayload, NetworkRuntimePhase, NetworkRuntimeReport,
};

#[tokio::test]
async fn network_runtime_chain_publishes_full_metadata_only_flow() {
    let report = publish_network_runtime_chain_for_observation(
        complete_domain_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert_eq!(
        report.publish_reports.len(),
        NetworkRuntimePhase::ordered_chain().len()
    );
    assert_eq!(
        report.stored_events.len(),
        NetworkRuntimePhase::ordered_chain().len()
    );
    assert!(report.dead_letters.is_empty());
    assert!(!report.manual_required());
    assert_eq!(payloads[0].phase, NetworkRuntimePhase::FlowObserved);
    assert_eq!(payloads[3].ai_audit_state, NetworkAiAuditState::Requested);
    assert_eq!(payloads[4].ai_audit_state, NetworkAiAuditState::Completed);
    assert_eq!(
        count_event_type(
            &report,
            constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
        ),
        1
    );
    assert_eq!(
        count_event_type(
            &report,
            constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED
        ),
        1
    );
    assert!(payloads.iter().all(|payload| {
        payload.evidence_scope == NetworkEvidenceScope::MetadataOnly
            && payload.evidence_grade == NetworkEvidenceGrade::DomainAndProcessMetadata
            && payload.risk_budget_state == NetworkRiskBudgetState::ObserveOnly
            && payload.intervention_state == NetworkInterventionState::DryRunOnly
            && !payload.claim_boundary.decrypted_https_payload_available
            && !payload.claim_boundary.exact_url_available
            && !payload.claim_boundary.page_content_available
            && !payload.claim_boundary.adapter_action_executed
    }));
}

#[tokio::test]
async fn manual_required_network_evidence_does_not_publish_enforcement_command() {
    let report = publish_network_runtime_chain_for_observation(
        ip_only_unknown_process_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES);
    let payloads = decode_payloads(&report);

    assert!(report.manual_required());
    assert_eq!(
        report.publish_reports.len(),
        NetworkRuntimePhase::ordered_chain().len() - 2
    );
    assert_eq!(
        count_event_type(
            &report,
            constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
        ),
        0
    );
    assert_eq!(
        count_event_type(
            &report,
            constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED
        ),
        0
    );
    assert!(payloads.iter().all(|payload| {
        payload.evidence_grade == NetworkEvidenceGrade::IpOrProcessPartialMetadata
            && payload.risk_budget_state == NetworkRiskBudgetState::ManualReviewRequired
            && payload.intervention_state == NetworkInterventionState::ManualRequired
            && !payload.claim_boundary.exact_url_available
            && !payload.claim_boundary.adapter_action_executed
    }));
}

#[tokio::test]
async fn degraded_adapter_flow_stays_unavailable_without_adapter_action() {
    let report = publish_network_runtime_chain_for_observation(
        NetworkObservation::degraded(ActivityCaptureCapabilityStatus::AdapterError),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_CHAIN_PUBLISHES_DEGRADED);
    let payloads = decode_payloads(&report);

    assert!(!report.manual_required());
    assert_eq!(
        report.publish_reports.len(),
        NetworkRuntimePhase::ordered_chain().len() - 2
    );
    assert_eq!(
        count_event_type(
            &report,
            constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
        ),
        0
    );
    assert_eq!(
        count_event_type(
            &report,
            constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED
        ),
        0
    );
    assert!(payloads.iter().all(|payload| {
        payload.evidence_scope == NetworkEvidenceScope::AdapterUnavailable
            && payload.evidence_grade == NetworkEvidenceGrade::AdapterUnavailable
            && payload.risk_budget_state == NetworkRiskBudgetState::Unavailable
            && payload.intervention_state == NetworkInterventionState::Unavailable
            && !payload.claim_boundary.decrypted_https_payload_available
            && !payload.claim_boundary.adapter_action_executed
    }));
}

#[tokio::test]
async fn network_runtime_queues_flow_until_subscriber_drains() {
    let report: NetworkRuntimeQueueDrainReport = queue_network_runtime_flow_until_subscriber(
        complete_domain_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_QUEUE_DRAINS);

    assert_eq!(
        report.queued_publish_report.event_type.as_str(),
        constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED
    );
    assert_eq!(
        report.queued_publish_report.queue_report.disposition,
        ocentra_eventing::QueueDisposition::QueuedNoSubscriber
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
}

#[tokio::test]
async fn network_runtime_review_request_resolves_associated_response() {
    let report: NetworkRuntimeReviewReport = request_network_runtime_review_for_observation(
        ip_only_unknown_process_observation(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
    )
    .await
    .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_REVIEW_COMPLETES);
    let response: &NetworkRuntimeReviewResponse = &report.request_report.response;

    assert_eq!(
        report.request_report.publish_report.event_type.as_str(),
        constants::network_flow::EVENT_NETWORK_REVIEW_REQUESTED
    );
    assert_eq!(report.request_report.publish_report.handled_count, 1);
    assert_eq!(
        response.evidence_grade,
        NetworkEvidenceGrade::IpOrProcessPartialMetadata
    );
    assert_eq!(
        response.risk_budget_state,
        NetworkRiskBudgetState::ManualReviewRequired
    );
    assert_eq!(
        response.intervention_state,
        NetworkInterventionState::ManualRequired
    );
    assert!(response.review_required);
    assert!(!response.claim_boundary.adapter_action_executed);
    assert_eq!(report.stored_events.len(), 1);
    assert_eq!(
        report.stored_events[0].contract.event_type.as_str(),
        constants::network_flow::EVENT_NETWORK_REVIEW_REQUESTED
    );
    assert!(report.dead_letters.is_empty());
}

fn decode_payloads(report: &NetworkRuntimeReport) -> Vec<NetworkRuntimeEventPayload> {
    report
        .stored_events
        .iter()
        .map(|event| {
            let envelope: ocentra_eventing::EventEnvelope<NetworkRuntimeEventPayload> = event
                .decode()
                .expect(constants::network_flow::ERROR_NETWORK_RUNTIME_PAYLOAD_DECODES);
            envelope.payload
        })
        .collect()
}

fn count_event_type(report: &NetworkRuntimeReport, event_type: &str) -> usize {
    report
        .stored_events
        .iter()
        .filter(|event| event.contract.event_type.as_str() == event_type)
        .count()
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

fn ip_only_unknown_process_observation() -> NetworkObservation {
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        destination_ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
        destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        destination_domain: None,
        tcp_state: Some(ActivityNetworkTcpState::Established),
        pid: None,
        process_name: None,
        associated_pid_count: 0,
    }
}
