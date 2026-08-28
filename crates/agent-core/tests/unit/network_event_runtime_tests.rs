use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use ocentra_eventing::{
    error::EventingError,
    ids::EventId,
    replay::{ReplayFilter, ReplayMode},
};
use ocentra_parent_agent_core::{
    network_capture::NetworkObservation,
    network_capture_event::network_observation_event,
    network_event_runtime::{
        network_runtime_event_id_for_source_event, network_runtime_event_ids_for_source_event,
        NetworkRuntimeEventPayload, NetworkRuntimeJournalPath, NetworkRuntimeSpine,
    },
};
use ocentra_parent_agent_protocol::{
    activity_capture::{
        ActivityCaptureCapabilityStatus, ActivityNetworkProtocol, ActivityNetworkTcpState,
    },
    constants,
    network_flow::{
        NetworkAiAuditState, NetworkEvidenceScope, NetworkInterventionState,
        NetworkRiskBudgetState, NetworkRuntimeEvidenceGrade, NetworkRuntimePhase,
    },
    NetworkEvidenceGrade, NetworkPolicyDecisionAction,
};

use crate::test_text::TestText;

type TestResult = Result<(), TestText>;

static TEMP_JOURNAL_SEQUENCE: AtomicU64 = AtomicU64::new(1);

#[tokio::test]
async fn network_consumer_chain_contract_uses_durable_journal_and_exact_source_ids() -> TestResult {
    let temporary = TemporaryJournal::new("domain")?;
    let observation = complete_domain_observation();
    let observed_at = constants::activity_store::TEST_FIRST_OBSERVED_AT;
    let source_event_id = source_event_id(&observation, observed_at, 0)?;
    let spine = open_spine(&temporary).await?;

    let report = publish(&spine, &source_event_id, observation.clone(), observed_at).await?;
    let payloads = decode_payloads(&report)?;
    let expected_phases = vec![
        NetworkRuntimePhase::FlowObserved,
        NetworkRuntimePhase::DomainObserved,
        NetworkRuntimePhase::ActivityClassified,
    ];

    assert_eq!(
        payloads
            .iter()
            .map(|payload| payload.phase)
            .collect::<Vec<_>>(),
        expected_phases
    );
    assert_eq!(report.publish_reports.len(), expected_phases.len());
    assert_eq!(report.stored_events.len(), expected_phases.len());
    assert!(report.dead_letters.is_empty());
    assert!(report.handled_phases.is_empty());
    assert_eq!(report.journal_state.as_str(), "durable");
    assert!(!report.manual_required());

    let expected_ids =
        network_runtime_event_ids_for_source_event(&source_event_id, &observation)
            .map_err(|error| TestText::from_display(format!("expected source ids: {error:?}")))?;
    let report_ids = report
        .publish_reports
        .iter()
        .map(|report| report.event_id.clone())
        .collect::<Vec<_>>();
    let stored_ids = report
        .stored_events
        .iter()
        .map(|event| event.event_id.clone())
        .collect::<Vec<_>>();
    assert_eq!(report_ids, expected_ids);
    assert_eq!(stored_ids, expected_ids);
    assert!(payloads
        .iter()
        .all(|payload| payload.evidence_ref == source_event_id.as_str()));
    assert_eq!(payloads[0].previous_phase_ref, None);
    assert_eq!(
        payloads[1].previous_phase_ref,
        Some(
            network_runtime_event_id_for_source_event(
                NetworkRuntimePhase::FlowObserved,
                source_event_id.as_str(),
            )
            .map_err(|error| TestText::from_display(format!("flow phase id: {error:?}")))?
            .as_str()
            .to_owned(),
        )
    );
    assert_eq!(
        payloads[2].previous_phase_ref,
        Some(
            network_runtime_event_id_for_source_event(
                NetworkRuntimePhase::DomainObserved,
                source_event_id.as_str(),
            )
            .map_err(|error| TestText::from_display(format!("domain phase id: {error:?}")))?
            .as_str()
            .to_owned(),
        )
    );
    assert!(!payloads.iter().any(|payload| {
        matches!(
            payload.phase,
            NetworkRuntimePhase::AiAnalysisRequested
                | NetworkRuntimePhase::AiAnalysisCompleted
                | NetworkRuntimePhase::PolicyEvaluationRequested
                | NetworkRuntimePhase::PolicyDecisionCompleted
                | NetworkRuntimePhase::EnforcementCommandIssued
                | NetworkRuntimePhase::EnforcementResultObserved
                | NetworkRuntimePhase::AuditEntryCommitted
                | NetworkRuntimePhase::PortalReadModelUpdated
        )
    }));
    assert_downstream_authority_remains_open(&report, &payloads);

    Ok(())
}

#[tokio::test]
async fn network_consumer_weak_evidence_requires_manual_review_without_enforcement() -> TestResult {
    let temporary = TemporaryJournal::new("no-domain")?;
    let observation = ip_only_observation();
    let observed_at = constants::activity_store::TEST_FIRST_OBSERVED_AT;
    let source_event_id = source_event_id(&observation, observed_at, 1)?;
    let spine = open_spine(&temporary).await?;

    let report = publish(&spine, &source_event_id, observation.clone(), observed_at).await?;
    let payloads = decode_payloads(&report)?;
    let phases = payloads
        .iter()
        .map(|payload| payload.phase)
        .collect::<Vec<_>>();

    assert_eq!(
        phases,
        vec![
            NetworkRuntimePhase::FlowObserved,
            NetworkRuntimePhase::ActivityClassified,
        ]
    );
    assert_eq!(
        report.publish_reports.len(),
        network_runtime_event_ids_for_source_event(&source_event_id, &observation)
            .map_err(|error| TestText::from_display(format!("expected source ids: {error:?}")))?
            .len()
    );
    assert!(payloads
        .iter()
        .all(|payload| payload.evidence_ref == source_event_id.as_str()));
    assert!(!payloads
        .iter()
        .any(|payload| payload.phase == NetworkRuntimePhase::DomainObserved));
    assert!(report.manual_required());
    assert!(payloads.iter().all(|payload| {
        payload.evidence_scope == NetworkEvidenceScope::MetadataOnly
            && payload.evidence_grade == NetworkRuntimeEvidenceGrade::IpOrProcessPartialMetadata
            && payload.evidence_grade_contract == NetworkEvidenceGrade::C
            && payload.risk_budget_state == NetworkRiskBudgetState::ManualReviewRequired
            && payload.intervention_state == NetworkInterventionState::ManualRequired
            && payload.policy_action == NetworkPolicyDecisionAction::AskParent
            && !payload.claim_boundary.adapter_action_executed
    }));
    assert_downstream_authority_remains_open(&report, &payloads);

    Ok(())
}

#[tokio::test]
async fn durable_runtime_keeps_destination_less_same_timestamp_sources_distinct() -> TestResult {
    let temporary = TemporaryJournal::new("destination-less")?;
    let observed_at = constants::activity_store::TEST_FIRST_OBSERVED_AT;
    let first_observation =
        destination_less_observation(constants::activity_store::TEST_NETWORK_LOCAL_PORT);
    let second_observation = destination_less_observation(
        constants::activity_store::TEST_NETWORK_LOCAL_PORT.saturating_add(1),
    );
    let first_source = source_event_id(&first_observation, observed_at, 0)?;
    let second_source = source_event_id(&second_observation, observed_at, 1)?;
    let spine = open_spine(&temporary).await?;

    let first = publish(&spine, &first_source, first_observation, observed_at).await?;
    let second = publish(&spine, &second_source, second_observation, observed_at).await?;

    assert_ne!(first_source, second_source);
    assert_ne!(
        first.stored_events[0].event_id,
        second.stored_events[0].event_id
    );
    assert_ne!(
        first.stored_events[0].correlation_id,
        second.stored_events[0].correlation_id
    );
    Ok(())
}

#[tokio::test]
async fn durable_runtime_keeps_available_without_metadata_unavailable() -> TestResult {
    let temporary = TemporaryJournal::new("available-without-metadata")?;
    let observation = NetworkObservation::degraded(ActivityCaptureCapabilityStatus::Available);
    let observed_at = constants::activity_store::TEST_FIRST_OBSERVED_AT;
    let source_event_id = source_event_id(&observation, observed_at, 3)?;
    let spine = open_spine(&temporary).await?;

    let report = publish(&spine, &source_event_id, observation, observed_at).await?;
    let payloads = decode_payloads(&report)?;

    assert_eq!(
        payloads
            .iter()
            .map(|payload| payload.phase)
            .collect::<Vec<_>>(),
        vec![
            NetworkRuntimePhase::FlowObserved,
            NetworkRuntimePhase::ActivityClassified,
        ]
    );
    assert!(!report.manual_required());
    assert!(payloads.iter().all(|payload| {
        payload.evidence_scope == NetworkEvidenceScope::MetadataOnly
            && payload.evidence_grade == NetworkRuntimeEvidenceGrade::AdapterUnavailable
            && payload.evidence_grade_contract == NetworkEvidenceGrade::D
            && payload.risk_budget_state == NetworkRiskBudgetState::Unavailable
            && payload.intervention_state == NetworkInterventionState::Unavailable
            && payload.policy_action == NetworkPolicyDecisionAction::Unknown
            && !payload.claim_boundary.adapter_action_executed
    }));
    assert_downstream_authority_remains_open(&report, &payloads);
    Ok(())
}

#[tokio::test]
async fn durable_runtime_keeps_adapter_error_unavailable_without_authority() -> TestResult {
    let temporary = TemporaryJournal::new("adapter-error")?;
    let observation = NetworkObservation::degraded(ActivityCaptureCapabilityStatus::AdapterError);
    let observed_at = constants::activity_store::TEST_FIRST_OBSERVED_AT;
    let source_event_id = source_event_id(&observation, observed_at, 4)?;
    let spine = open_spine(&temporary).await?;

    let report = publish(&spine, &source_event_id, observation, observed_at).await?;
    let payloads = decode_payloads(&report)?;

    assert_eq!(
        payloads
            .iter()
            .map(|payload| payload.phase)
            .collect::<Vec<_>>(),
        vec![
            NetworkRuntimePhase::FlowObserved,
            NetworkRuntimePhase::ActivityClassified,
        ]
    );
    assert!(!report.manual_required());
    assert!(payloads.iter().all(|payload| {
        payload.evidence_scope == NetworkEvidenceScope::AdapterUnavailable
            && payload.evidence_grade == NetworkRuntimeEvidenceGrade::AdapterUnavailable
            && payload.evidence_grade_contract == NetworkEvidenceGrade::D
            && payload.risk_budget_state == NetworkRiskBudgetState::Unavailable
            && payload.intervention_state == NetworkInterventionState::Unavailable
            && payload.policy_action == NetworkPolicyDecisionAction::Unknown
            && !payload.claim_boundary.adapter_action_executed
    }));
    assert_downstream_authority_remains_open(&report, &payloads);
    Ok(())
}

#[tokio::test]
async fn network_consumer_source_identity_is_deterministic_and_replay_idempotent() -> TestResult {
    let temporary = TemporaryJournal::new("reopen-retry")?;
    let observation = complete_domain_observation();
    let observed_at = constants::activity_store::TEST_FIRST_OBSERVED_AT;
    let source_event_id = source_event_id(&observation, observed_at, 2)?;

    let first_spine = open_spine(&temporary).await?;
    let first = publish(
        &first_spine,
        &source_event_id,
        observation.clone(),
        observed_at,
    )
    .await?;
    let first_projection = first_spine
        .replay_projection(ReplayFilter::all())
        .await
        .map_err(|error| TestText::from_display(format!("first projection: {error:?}")))?;
    assert_eq!(first_projection.mode, ReplayMode::ProjectionOnly);
    assert_eq!(first_projection.records.len(), first.stored_events.len());
    drop(first_spine);

    let reopened_spine = open_spine(&temporary).await?;
    let reopened_projection = reopened_spine
        .replay_projection(ReplayFilter::all())
        .await
        .map_err(|error| TestText::from_display(format!("reopened projection: {error:?}")))?;
    assert_eq!(reopened_projection.mode, ReplayMode::ProjectionOnly);
    assert_eq!(reopened_projection.records.len(), first.stored_events.len());

    let retry = publish(
        &reopened_spine,
        &source_event_id,
        observation.clone(),
        observed_at,
    )
    .await?;
    assert_eq!(
        retry
            .publish_reports
            .iter()
            .map(|report| report.event_id.clone())
            .collect::<Vec<_>>(),
        first
            .publish_reports
            .iter()
            .map(|report| report.event_id.clone())
            .collect::<Vec<_>>()
    );
    let retry_projection = reopened_spine
        .replay_projection(ReplayFilter::all())
        .await
        .map_err(|error| TestText::from_display(format!("retry projection: {error:?}")))?;
    assert_eq!(
        retry_projection.records.len(),
        first_projection.records.len()
    );

    let mut conflicting_observation = observation;
    conflicting_observation.destination_domain = Some("different.example".to_owned());
    let conflict = publish_raw(
        &reopened_spine,
        &source_event_id,
        conflicting_observation,
        observed_at,
    )
    .await
    .expect_err("same source event id with a different envelope must fail closed");
    assert!(matches!(conflict, EventingError::DuplicateEventId { .. }));

    Ok(())
}

fn assert_downstream_authority_remains_open(
    report: &ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeReport,
    payloads: &[NetworkRuntimeEventPayload],
) {
    assert!(report.handled_phases.is_empty());
    assert!(payloads.iter().all(|payload| {
        payload.ai_audit_state == NetworkAiAuditState::NotRequested
            && payload.ai_request_ref.is_none()
            && payload.ai_analysis_ref.is_none()
            && payload.policy_evaluation_ref.is_none()
            && payload.policy_decision_ref.is_none()
            && payload.enforcement_command_ref.is_none()
            && payload.enforcement_result_ref.is_none()
            && payload.audit_entry_ref.is_none()
            && !payload.claim_boundary.raw_pcap_available
            && !payload.claim_boundary.decrypted_https_payload_available
            && !payload.claim_boundary.exact_url_available
            && !payload.claim_boundary.page_content_available
            && !payload.claim_boundary.video_content_available
            && !payload.claim_boundary.private_message_content_available
            && !payload.claim_boundary.search_query_available
            && !payload.claim_boundary.adapter_action_executed
    }));
}

async fn open_spine(temporary: &TemporaryJournal) -> Result<NetworkRuntimeSpine, TestText> {
    NetworkRuntimeSpine::with_durable_journal(&temporary.path)
        .await
        .map_err(|error| TestText::from_display(format!("open durable spine: {error:?}")))
}

async fn publish(
    spine: &NetworkRuntimeSpine,
    source_event_id: &EventId,
    observation: NetworkObservation,
    observed_at: &str,
) -> Result<ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeReport, TestText> {
    publish_raw(spine, source_event_id, observation, observed_at)
        .await
        .map_err(|error| TestText::from_display(format!("publish durable observation: {error:?}")))
}

async fn publish_raw(
    spine: &NetworkRuntimeSpine,
    source_event_id: &EventId,
    observation: NetworkObservation,
    observed_at: &str,
) -> Result<ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeReport, EventingError> {
    spine
        .publish_observation_chain_for_source_event(
            source_event_id.as_str(),
            observation,
            observed_at,
        )
        .await
}

fn decode_payloads(
    report: &ocentra_parent_agent_core::network_event_runtime::NetworkRuntimeReport,
) -> Result<Vec<NetworkRuntimeEventPayload>, TestText> {
    report
        .stored_events
        .iter()
        .map(|event| {
            event
                .decode::<NetworkRuntimeEventPayload>()
                .map(|envelope| envelope.into_payload())
                .map_err(|error| {
                    TestText::from_display(format!("decode runtime payload: {error:?}"))
                })
        })
        .collect()
}

fn source_event_id(
    observation: &NetworkObservation,
    observed_at: &str,
    sequence_index: usize,
) -> Result<EventId, TestText> {
    let source_event = network_observation_event(observation.clone(), observed_at, sequence_index);
    EventId::parse(source_event.event_id).map_err(|error| {
        TestText::from_display(format!("parse captured source event id: {error:?}"))
    })
}

struct TemporaryJournal {
    path: NetworkRuntimeJournalPath,
}

impl TemporaryJournal {
    fn new(label: &str) -> Result<Self, TestText> {
        let sequence = TEMP_JOURNAL_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let artifact_directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../target/test-artifacts/network-runtime-core");
        std::fs::create_dir_all(&artifact_directory).map_err(|error| {
            TestText::from_display(format!(
                "create bounded journal artifact directory: {error}"
            ))
        })?;
        let journal_file = artifact_directory.join(format!(
            "network-runtime-{label}-{}-{sequence}.ndjson",
            std::process::id()
        ));
        Ok(Self {
            path: NetworkRuntimeJournalPath::new(journal_file),
        })
    }
}

fn complete_domain_observation() -> NetworkObservation {
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_owned()),
        local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        destination_ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_owned()),
        destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_owned()),
        tcp_state: Some(ActivityNetworkTcpState::Established),
        pid: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_owned()),
        associated_pid_count: 1,
    }
}

fn destination_less_observation(local_port: u16) -> NetworkObservation {
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Udp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_owned()),
        local_port: Some(local_port),
        destination_ip: None,
        destination_port: None,
        destination_domain: None,
        tcp_state: None,
        pid: None,
        process_name: None,
        associated_pid_count: 0,
    }
}

fn ip_only_observation() -> NetworkObservation {
    NetworkObservation {
        status: ActivityCaptureCapabilityStatus::Available,
        protocol: Some(ActivityNetworkProtocol::Tcp),
        local_ip: Some(constants::test_network::LOOPBACK_IP.to_owned()),
        local_port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        destination_ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_owned()),
        destination_port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        destination_domain: None,
        tcp_state: Some(ActivityNetworkTcpState::Established),
        pid: None,
        process_name: None,
        associated_pid_count: 0,
    }
}
