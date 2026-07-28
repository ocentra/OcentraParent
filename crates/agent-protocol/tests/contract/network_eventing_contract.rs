use std::sync::{Arc, Mutex};

use ocentra_eventing::{
    bus::{subscriber::EventSubscriber, EventBus},
    envelope::{DomainEvent, EventEnvelope, EventMetadata, EventSource},
    expect_value::{ExpectErrValue, ExpectValue},
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
const INVALID_SEMANTICS: &str = "invalid eventing value for network_runtime_payload_semantics: evidence/risk/intervention/policy tuple is inconsistent";

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
            serde_json::to_value(protocol_grade).expect_value(CONTRACT_EXPECTATION),
            serde_json::to_value(evidence_grade).expect_value(CONTRACT_EXPECTATION)
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

#[test]
fn network_runtime_payload_schema_mutations_fail_closed() {
    let payload = payload();
    let valid = serde_json::to_value(&payload).expect_value(CONTRACT_EXPECTATION);
    assert_eq!(
        serde_json::from_value::<NetworkRuntimeEventPayload>(valid.clone())
            .expect_value(CONTRACT_EXPECTATION),
        payload
    );

    let mut unknown_enum = valid.clone();
    unknown_enum["evidence_grade_contract"] = serde_json::json!("future-grade");
    let mut missing = valid.clone();
    missing
        .as_object_mut()
        .expect_value(CONTRACT_EXPECTATION)
        .remove("evidence_ref");
    let mut extra = valid.clone();
    extra["futureField"] = serde_json::json!(true);
    let mut type_corruption = valid;
    type_corruption["destination_port"] = serde_json::json!("not-a-port");

    for mutation in [unknown_enum, missing, extra, type_corruption] {
        assert_eq!(
            serde_json::from_value::<NetworkRuntimeEventPayload>(mutation)
                .err()
                .map(|error| error.classify()),
            Some(serde_json::error::Category::Data)
        );
    }
}

#[test]
fn network_runtime_payload_schema_version_skew_fails_closed() {
    let live = EventEnvelope::from_event(payload(), metadata()).expect_value(CONTRACT_EXPECTATION);
    let mut newer = live.store().expect_value(CONTRACT_EXPECTATION);
    newer.contract.schema_version = ocentra_eventing::ids::SchemaVersion::new(
        constants::network_flow::RUNTIME_EVENT_SCHEMA_VERSION + 1,
    )
    .expect_value(CONTRACT_EXPECTATION);
    assert_eq!(
        newer
            .decode::<NetworkRuntimeEventPayload>()
            .err()
            .map(|error| error.to_string()),
        Some(format!(
            "event contract mismatch: expected {}@{}, received {}@{}",
            NetworkRuntimePhase::FlowObserved.event_type(),
            constants::network_flow::RUNTIME_EVENT_SCHEMA_VERSION,
            NetworkRuntimePhase::FlowObserved.event_type(),
            constants::network_flow::RUNTIME_EVENT_SCHEMA_VERSION + 1,
        ))
    );

    let mut incompatible_fixture = live.store().expect_value(CONTRACT_EXPECTATION);
    incompatible_fixture.contract.schema_version =
        ocentra_eventing::ids::SchemaVersion::new(constants::network_flow::EVENT_SCHEMA_VERSION)
            .expect_value(CONTRACT_EXPECTATION);
    assert_eq!(
        incompatible_fixture
            .decode::<NetworkRuntimeEventPayload>()
            .err()
            .map(|error| error.to_string()),
        Some(format!(
            "event contract mismatch: expected {}@{}, received {}@{}",
            NetworkRuntimePhase::FlowObserved.event_type(),
            constants::network_flow::RUNTIME_EVENT_SCHEMA_VERSION,
            NetworkRuntimePhase::FlowObserved.event_type(),
            constants::network_flow::EVENT_SCHEMA_VERSION,
        ))
    );
}

#[test]
fn network_runtime_payload_exhaustively_round_trips_canonical_grade_and_policy_values() {
    let grades = [
        ocentra_parent_agent_protocol::NetworkEvidenceGrade::A,
        ocentra_parent_agent_protocol::NetworkEvidenceGrade::B,
        ocentra_parent_agent_protocol::NetworkEvidenceGrade::C,
        ocentra_parent_agent_protocol::NetworkEvidenceGrade::D,
    ];
    let actions = [
        ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::Observe,
        ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::Warn,
        ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::AskParent,
        ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::Limit,
        ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::Block,
        ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::ManualReview,
        ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::Unknown,
    ];

    for grade in grades {
        for action in actions {
            let mut candidate = payload();
            candidate.evidence_grade_contract = grade;
            candidate.policy_action = action;
            let encoded = serde_json::to_value(&candidate).expect_value(CONTRACT_EXPECTATION);
            assert_eq!(
                serde_json::from_value::<NetworkRuntimeEventPayload>(encoded)
                    .expect_value(CONTRACT_EXPECTATION),
                candidate
            );
        }
    }
}

#[test]
fn network_runtime_payload_rejects_impossible_semantic_tuple_before_dispatch() {
    let mut candidate = payload();
    candidate.evidence_grade_contract = ocentra_parent_agent_protocol::NetworkEvidenceGrade::A;
    candidate.policy_action = ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::Block;
    assert_eq!(
        candidate
            .validate_semantics()
            .err()
            .map(|error| error.to_string()),
        Some(INVALID_SEMANTICS.to_string())
    );
    assert_eq!(
        candidate.contract().err().map(|error| error.to_string()),
        Some(INVALID_SEMANTICS.to_string())
    );

    let mut unattributed = payload();
    unattributed.process_attribution_status = ActivityProcessAttributionStatus::ProcessUnknown;
    assert_eq!(
        unattributed
            .validate_semantics()
            .expect_err_value(CONTRACT_EXPECTATION)
            .to_string(),
        INVALID_SEMANTICS
    );
    assert_eq!(
        unattributed
            .contract()
            .expect_err_value(CONTRACT_EXPECTATION)
            .to_string(),
        INVALID_SEMANTICS
    );
}

#[test]
fn unavailable_capture_cannot_claim_domain_metadata_grade_before_dispatch() {
    let mut candidate = payload();
    candidate.capability_status = ActivityCaptureCapabilityStatus::Unavailable;
    candidate.evidence_scope = NetworkEvidenceScope::AdapterUnavailable;
    assert_eq!(
        candidate
            .validate_semantics()
            .err()
            .map(|error| error.to_string()),
        Some(INVALID_SEMANTICS.to_string())
    );
    assert_eq!(
        candidate.contract().err().map(|error| error.to_string()),
        Some(INVALID_SEMANTICS.to_string())
    );
}

#[test]
fn available_destination_less_capture_remains_partial_metadata_publishable() {
    let mut candidate = payload();
    candidate.domain_attribution_status = ActivityDomainAttributionStatus::Unavailable;
    candidate.destination_ip = None;
    candidate.destination_port = None;
    candidate.destination_domain = None;
    candidate.evidence_grade = NetworkRuntimeEvidenceGrade::IpOrProcessPartialMetadata;
    candidate.evidence_grade_contract = ocentra_parent_agent_protocol::NetworkEvidenceGrade::C;
    candidate.risk_budget_state = NetworkRiskBudgetState::ManualReviewRequired;
    candidate.intervention_state = NetworkInterventionState::ManualRequired;
    candidate.policy_action = ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::AskParent;

    assert_eq!(
        candidate
            .validate_semantics()
            .map_err(|error| error.to_string()),
        Ok(())
    );
    assert_eq!(
        candidate
            .contract()
            .map(|_| ())
            .map_err(|error| error.to_string()),
        Ok(())
    );
}

#[test]
fn available_metadata_free_capture_remains_diagnostics_only_grade_d() {
    let mut candidate = payload();
    candidate.domain_attribution_status = ActivityDomainAttributionStatus::Unavailable;
    candidate.process_attribution_status = ActivityProcessAttributionStatus::ProcessUnknown;
    candidate.protocol = None;
    candidate.tcp_state = None;
    candidate.local_ip = None;
    candidate.local_port = None;
    candidate.destination_ip = None;
    candidate.destination_port = None;
    candidate.destination_domain = None;
    candidate.process_id = None;
    candidate.process_name = None;
    candidate.evidence_grade = NetworkRuntimeEvidenceGrade::AdapterUnavailable;
    candidate.evidence_grade_contract = ocentra_parent_agent_protocol::NetworkEvidenceGrade::D;
    candidate.risk_budget_state = NetworkRiskBudgetState::Unavailable;
    candidate.intervention_state = NetworkInterventionState::Unavailable;
    candidate.policy_action = ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::Unknown;

    assert_eq!(
        candidate
            .validate_semantics()
            .map_err(|error| error.to_string()),
        Ok(())
    );
    assert_eq!(
        candidate
            .contract()
            .map(|_| ())
            .map_err(|error| error.to_string()),
        Ok(())
    );
}

#[test]
fn network_contract_schema_fuzz_seeded_mutation_cases_fail_closed() {
    const SEED: u64 = 0x4e57_5030_315f_7632;
    let valid = serde_json::to_value(payload()).expect_value(CONTRACT_EXPECTATION);
    let mut cases = Vec::new();

    let mut unknown_enum = valid.clone();
    unknown_enum["evidence_grade_contract"] = serde_json::json!(format!("future-{SEED:x}"));
    cases.push(unknown_enum);
    let mut missing = valid.clone();
    missing
        .as_object_mut()
        .expect_value(CONTRACT_EXPECTATION)
        .remove("evidence_ref");
    cases.push(missing);
    let mut extra = valid.clone();
    extra["seeded_future_field"] = serde_json::json!(SEED);
    cases.push(extra);
    let mut type_corruption = valid.clone();
    type_corruption["destination_port"] = serde_json::json!(format!("seed-{SEED:x}"));
    cases.push(type_corruption);
    let mut state = SEED;
    for _ in 0..256 {
        state = state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let mut candidate = valid.clone();
        candidate["destination_port"] = serde_json::json!(format!("fuzz-{state:x}"));
        cases.push(candidate);
    }
    for candidate in cases {
        assert_eq!(
            serde_json::from_value::<NetworkRuntimeEventPayload>(candidate)
                .err()
                .map(|error| error.classify()),
            Some(serde_json::error::Category::Data)
        );
    }
}

#[tokio::test]
async fn invalid_serialized_runtime_payload_is_rejected_before_handler_receipt() {
    let bus = EventBus::new();
    let delivered = Arc::new(Mutex::new(Vec::new()));
    let captured = Arc::clone(&delivered);
    bus.subscribe::<NetworkRuntimeEventPayload, _, _>(subscriber(), move |context| {
        let captured = Arc::clone(&captured);
        async move {
            captured
                .lock()
                .expect_value(CONTRACT_EXPECTATION)
                .push(context.payload().phase);
            Ok(())
        }
    })
    .await
    .expect_value(CONTRACT_EXPECTATION);

    let mut serialized = serde_json::to_value(payload()).expect_value(CONTRACT_EXPECTATION);
    serialized["policy_action"] = serde_json::json!("block");
    let invalid = serde_json::from_value::<NetworkRuntimeEventPayload>(serialized)
        .expect_value(CONTRACT_EXPECTATION);
    assert_eq!(
        bus.publish_and_wait(invalid, metadata())
            .await
            .err()
            .map(|error| error.to_string()),
        Some(INVALID_SEMANTICS.to_string())
    );
    assert!(delivered
        .lock()
        .expect_value(CONTRACT_EXPECTATION)
        .is_empty());
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
        evidence_grade_contract: ocentra_parent_agent_protocol::NetworkEvidenceGrade::B,
        ai_audit_state: NetworkAiAuditState::NotRequested,
        risk_budget_state: NetworkRiskBudgetState::ObserveOnly,
        intervention_state: NetworkInterventionState::DryRunOnly,
        policy_action: ocentra_parent_agent_protocol::NetworkPolicyDecisionAction::Observe,
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
