use std::sync::{Arc, Mutex};

use ocentra_eventing::{
    bus::{subscriber::EventSubscriber, EventBus},
    envelope::{EventMetadata, EventSource},
    expect_value::ExpectValue,
    ids::{
        CorrelationId, EventCustody, EventId, EventType, RecordedAt, RuntimeInstanceId,
        RuntimeRole, SourceComponent, SourceService, SubscriberId, TargetHandler,
    },
};
use ocentra_network_evidence::dns::types::NetworkEvidenceGrade as EvidenceGrade;
use ocentra_parent_agent_protocol::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus,
    ActivityNetworkProtocol, ActivityNetworkTcpState, ActivityProcessAttributionStatus,
    NetworkAiAuditState, NetworkEvidenceScope, NetworkInterventionState, NetworkRiskBudgetState,
    NetworkRuntimeClaimBoundary, NetworkRuntimeEventPayload, NetworkRuntimeEvidenceGrade,
    NetworkRuntimePhase,
};

const CONTRACT_EXPECTATION: &str = "network eventing contract fixture parses";
const EVENT_ID: &str = "network-eventing-contract-event";
const CORRELATION_ID: &str = "network-eventing-contract-correlation";
const CUSTODY: &str = "local-only";
const SOURCE_SERVICE: &str = "network-eventing-contract-service";
const SOURCE_COMPONENT: &str = "network-eventing-contract-component";
const RUNTIME_INSTANCE_ID: &str = "network-eventing-contract-runtime";
const OBSERVED_AT: &str = "2026-07-18T17:00:00Z";

#[test]
fn network_evidence_grade_wire_values_match_evidence_contract() {
    let protocol_grades = [
        ocentra_parent_agent_protocol::NetworkEvidenceGrade::A,
        ocentra_parent_agent_protocol::NetworkEvidenceGrade::B,
        ocentra_parent_agent_protocol::NetworkEvidenceGrade::C,
        ocentra_parent_agent_protocol::NetworkEvidenceGrade::D,
    ];
    let evidence_grades = [
        EvidenceGrade::A,
        EvidenceGrade::B,
        EvidenceGrade::C,
        EvidenceGrade::D,
    ];

    for (protocol_grade, evidence_grade) in protocol_grades.into_iter().zip(evidence_grades) {
        assert_eq!(
            serde_json::to_value(protocol_grade).unwrap_or_default(),
            serde_json::to_value(evidence_grade).unwrap_or_default()
        );
    }

    assert_eq!(
        serde_json::from_value::<ocentra_parent_agent_protocol::NetworkEvidenceGrade>(
            serde_json::json!("future-grade")
        )
        .err()
        .map(|error| error.classify()),
        Some(serde_json::error::Category::Data)
    );
    assert_eq!(
        serde_json::from_value::<EvidenceGrade>(serde_json::json!("future-grade"))
            .err()
            .map(|error| error.classify()),
        Some(serde_json::error::Category::Data)
    );
}

#[tokio::test]
async fn network_runtime_payload_dispatches_once_through_shared_event_bus() {
    let bus = EventBus::new();
    let delivered = Arc::new(Mutex::new(Vec::new()));
    let captured = Arc::clone(&delivered);
    let payload = payload();

    bus.subscribe::<NetworkRuntimeEventPayload, _, _>(subscriber(), move |context| {
        let captured = Arc::clone(&captured);
        async move {
            captured.lock().expect_value(CONTRACT_EXPECTATION).push((
                context.payload().phase,
                context.payload().evidence_ref.clone(),
            ));
            Ok(())
        }
    })
    .await
    .expect_value(CONTRACT_EXPECTATION);

    let report = bus
        .publish_and_wait(payload, metadata())
        .await
        .expect_value(CONTRACT_EXPECTATION);

    assert_eq!(report.handled_count, 1);
    assert_eq!(
        delivered
            .lock()
            .expect_value(CONTRACT_EXPECTATION)
            .as_slice(),
        &[(
            NetworkRuntimePhase::FlowObserved,
            constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_string(),
        )]
    );
}

fn subscriber() -> EventSubscriber {
    EventSubscriber::new(
        SubscriberId::parse(NetworkRuntimePhase::FlowObserved.subscriber_id())
            .expect_value(CONTRACT_EXPECTATION),
        EventType::parse(NetworkRuntimePhase::FlowObserved.event_type())
            .expect_value(CONTRACT_EXPECTATION),
        TargetHandler::parse(NetworkRuntimePhase::FlowObserved.target_handler())
            .expect_value(CONTRACT_EXPECTATION),
    )
}

fn metadata() -> EventMetadata {
    EventMetadata::from_parts(
        EventId::parse(EVENT_ID).expect_value(CONTRACT_EXPECTATION),
        CorrelationId::parse(CORRELATION_ID).expect_value(CONTRACT_EXPECTATION),
        EventSource::new(
            EventCustody::parse(CUSTODY).expect_value(CONTRACT_EXPECTATION),
            RuntimeRole::parse(
                NetworkRuntimePhase::FlowObserved
                    .runtime_role()
                    .expect_value(CONTRACT_EXPECTATION)
                    .as_str(),
            )
            .expect_value(CONTRACT_EXPECTATION),
            SourceService::parse(SOURCE_SERVICE).expect_value(CONTRACT_EXPECTATION),
            SourceComponent::parse(SOURCE_COMPONENT).expect_value(CONTRACT_EXPECTATION),
            RuntimeInstanceId::parse(RUNTIME_INSTANCE_ID).expect_value(CONTRACT_EXPECTATION),
        ),
        RecordedAt::parse(OBSERVED_AT).expect_value(CONTRACT_EXPECTATION),
        Some(
            TargetHandler::parse(NetworkRuntimePhase::FlowObserved.target_handler())
                .expect_value(CONTRACT_EXPECTATION),
        ),
    )
}

fn payload() -> NetworkRuntimeEventPayload {
    NetworkRuntimeEventPayload {
        phase: NetworkRuntimePhase::FlowObserved,
        capability_status: ActivityCaptureCapabilityStatus::Available,
        domain_attribution_status: ActivityDomainAttributionStatus::DomainObserved,
        process_attribution_status: ActivityProcessAttributionStatus::ProcessAttributed,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        tcp_state: Some(ActivityNetworkTcpState::Established),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
        local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        destination_ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
        destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        process_id: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        evidence_scope: NetworkEvidenceScope::MetadataOnly,
        evidence_grade: NetworkRuntimeEvidenceGrade::DomainAndProcessMetadata,
        ai_audit_state: NetworkAiAuditState::NotRequested,
        risk_budget_state: NetworkRiskBudgetState::ObserveOnly,
        intervention_state: NetworkInterventionState::DryRunOnly,
        claim_boundary: NetworkRuntimeClaimBoundary::metadata_only(),
        previous_phase_ref: None,
        evidence_ref: constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_string(),
        ai_request_ref: None,
        ai_analysis_ref: None,
        policy_evaluation_ref: None,
        policy_decision_ref: None,
        adapter_capability_ref: None,
        enforcement_command_ref: None,
        enforcement_result_ref: None,
        audit_entry_ref: None,
        observed_at: OBSERVED_AT.to_string(),
    }
}
