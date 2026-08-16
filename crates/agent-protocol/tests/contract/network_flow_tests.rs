use super::{
    constants, ActivityCaptureCapabilityStatus, ActivityDomainAttributionStatus,
    ActivityNetworkEndpoint, ActivityNetworkFlowCounters, ActivityNetworkFlowObservation,
    ActivityNetworkFlowReadModel, ActivityNetworkProtocol, ActivityNetworkTcpState,
    ActivityProcessAttributionStatus, NetworkActivityClassifiedEvent,
    NetworkAiAnalysisCompletedEvent, NetworkAiAnalysisRequestedEvent, NetworkAiAuditState,
    NetworkAuditEntryCommittedEvent, NetworkDomainObservedEvent,
    NetworkEnforcementCommandIssuedEvent, NetworkEnforcementResultObservedEvent,
    NetworkEnforcementResultStatus, NetworkEvidenceGrade, NetworkEvidenceScope,
    NetworkFlowObservedEvent, NetworkInterventionState, NetworkPolicyDecisionAction,
    NetworkPolicyDecisionCompletedEvent, NetworkPolicyEvaluationRequestedEvent,
    NetworkPortalReadModelUpdatedEvent, NetworkRemoteDeliveryCrossProcessCustodyReadinessState,
    NetworkRemoteDeliveryStatus, NetworkRiskBudgetState, NetworkRuntimeClaimBoundary,
    NetworkRuntimeEventPayload, NetworkRuntimeEvidenceGrade, NetworkRuntimePhase,
    NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE, NETWORK_FLOW_SCHEMA_VERSION,
};
use crate::network_flow::{
    NetworkRemoteDeliveryExternalCrossProcessTransportState, NetworkRuntimeEventContract,
};
use ocentra_eventing::envelope::{DomainEvent, EventEnvelope, EventMetadata, EventSource};
use ocentra_eventing::error::EventingError;
use ocentra_eventing::ids::{
    CorrelationId, EventCustody, RuntimeInstanceId, RuntimeRole, SourceComponent, SourceService,
};

macro_rules! serialized_field {
    ($value:expr, $field:expr $(,)?) => {{
        serde_json::to_value($value).unwrap_or_default()[$field].clone()
    }};
    ($value:expr, $field:expr, $nested:expr $(,)?) => {{
        serde_json::to_value($value).unwrap_or_default()[$field][$nested].clone()
    }};
}

#[path = "network_flow_event_fixtures.rs"]
mod network_flow_event_fixtures;

use constants::network_flow as flow;
use network_flow_event_fixtures::{
    network_activity_classified_event, network_ai_analysis_completed_event,
    network_ai_analysis_requested_event, network_audit_entry_committed_event,
    network_domain_observed_event, network_enforcement_command_issued_event,
    network_enforcement_result_observed_event, network_flow_observed_event,
    network_policy_decision_completed_event, network_policy_evaluation_requested_event,
    network_portal_read_model_updated_event,
};

#[test]
fn network_flow_observation_serializes_to_contract_shape() {
    let observation = ActivityNetworkFlowObservation {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        event_id: constants::activity_store::TEST_NETWORK_EVENT_ID.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        observer: constants::activity_observer::WINDOWS_NETWORK.to_string(),
        capability_status: constants::activity_capture::CAPABILITY_STATUS_AVAILABLE.to_string(),
        adapter_id: constants::activity_capture::NETWORK_ADAPTER_ID.to_string(),
        protocol: Some(constants::activity_capture::NETWORK_PROTOCOL_TCP.to_string()),
        tcp_state: Some(constants::activity_capture::TCP_STATE_ESTABLISHED.to_string()),
        local_endpoint: ActivityNetworkEndpoint {
            ip: Some(constants::test_network::LOOPBACK_IP.to_string()),
            port: Some(constants::activity_store::TEST_NETWORK_LOCAL_PORT),
        },
        destination_endpoint: ActivityNetworkEndpoint {
            ip: Some(constants::activity_store::TEST_NETWORK_DESTINATION_IP.to_string()),
            port: Some(constants::activity_store::TEST_NETWORK_DESTINATION_PORT),
        },
        destination_domain: Some(constants::activity_store::TEST_NETWORK_DOMAIN.to_string()),
        domain_attribution_status:
            constants::activity_capture::DOMAIN_ATTRIBUTION_STATUS_DOMAIN_OBSERVED.to_string(),
        process_attribution_status:
            constants::activity_capture::PROCESS_ATTRIBUTION_STATUS_ATTRIBUTED.to_string(),
        process_id: Some(4242),
        process_name: Some(constants::activity_store::TEST_PROCESS_SUBJECT_NAME.to_string()),
        counters: ActivityNetworkFlowCounters {
            connection_count: 1,
            bytes_sent: None,
            bytes_received: None,
            first_seen_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
            last_seen_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        },
        evidence: Vec::new(),
    };

    let serialized = serde_json::to_value(observation).unwrap_or_default();

    assert_eq!(serialized["schemaVersion"], NETWORK_FLOW_SCHEMA_VERSION);
    assert_eq!(
        serialized["destinationEndpoint"]["port"],
        constants::activity_store::TEST_NETWORK_DESTINATION_PORT
    );
    assert_eq!(
        serialized["destinationDomain"],
        constants::activity_store::TEST_NETWORK_DOMAIN
    );
    assert_eq!(
        serialized["counters"]["connectionCount"],
        serde_json::json!(1)
    );
}

#[test]
fn network_flow_read_model_serializes_rows_without_payload_claims() {
    let read_model = ActivityNetworkFlowReadModel {
        schema_version: NETWORK_FLOW_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        custody: NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 0,
        active_rows: 0,
        tombstone_rows: 1,
        exportable_rows: 0,
        capability_status: constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS
            .to_string(),
        latest_event_id: Some(
            constants::activity_store::TEST_NETWORK_RETENTION_DELETE_EVENT_ID.to_string(),
        ),
        latest_observed_at: Some(
            constants::activity_store::TEST_NETWORK_RETENTION_DELETE_OBSERVED_AT.to_string(),
        ),
        latest_tombstone_event_id: Some(
            constants::activity_store::TEST_NETWORK_RETENTION_DELETE_EVENT_ID.to_string(),
        ),
        latest_tombstone_observed_at: Some(
            constants::activity_store::TEST_NETWORK_RETENTION_DELETE_OBSERVED_AT.to_string(),
        ),
        deleted_evidence_reference_ids: vec![
            constants::activity_store::TEST_NETWORK_EVENT_ID.to_string()
        ],
        rows: Vec::new(),
    };

    let serialized = serde_json::to_value(read_model).unwrap_or_default();

    assert_eq!(
        serialized["custody"],
        NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE
    );
    assert_eq!(
        serialized["capabilityStatus"],
        constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS
    );
    assert_eq!(
        serialized["deletedEvidenceReferenceIds"][0],
        constants::activity_store::TEST_NETWORK_EVENT_ID
    );
    assert_eq!(serialized["tombstoneRows"], 1);
    assert_eq!(serialized["rows"].as_array().map(Vec::len), Some(0));
}

#[test]
fn network_remote_delivery_status_serializes_row10t_external_transport_status_without_product_claims(
) {
    let serialized = serde_json::to_value(remote_delivery_status_fixture()).unwrap_or_default();

    assert_remote_delivery_status_refs(&serialized);
    assert_remote_delivery_status_counts(&serialized);
    assert_remote_delivery_status_no_product_claims(&serialized);
}

fn assert_remote_delivery_status_refs(serialized: &serde_json::Value) {
    assert_remote_delivery_status_core_refs(serialized);
    assert_remote_delivery_status_fixture_refs(serialized);
    assert_remote_delivery_status_provider_child_refs(serialized);
}

fn assert_remote_delivery_status_core_refs(serialized: &serde_json::Value) {
    assert_eq!(
        serialized["statusRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_STATUS_REF
    );
    assert_eq!(
        serialized["brokerStatus"],
        "fixture-requirements-recorded-but-not-implemented"
    );
    assert_eq!(
        serialized["durableEnvelopeRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF
    );
    assert_eq!(serialized["durableEnvelopeReady"], true);
    assert_eq!(
        serialized["outboxRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_REF
    );
    assert_eq!(
        serialized["outboxHandoffRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_OUTBOX_HANDOFF_REF
    );
}

fn assert_remote_delivery_status_fixture_refs(serialized: &serde_json::Value) {
    assert_eq!(
        serialized["transportDispatchStateRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_TRANSPORT_DISPATCH_STATE_REF
    );
    assert_eq!(
        serialized["blockedDispatchRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_DISPATCH_BLOCKED_MANUAL_REF
    );
    assert_eq!(
        serialized["futureTransportSeamRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_FUTURE_TRANSPORT_SEAM_REF
    );
    assert_eq!(
        serialized["fixtureTransportRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_FIXTURE_TRANSPORT_REF
    );
    assert_eq!(
        serialized["fixtureDispatchAttemptRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_FIXTURE_DISPATCH_ATTEMPT_REF
    );
    assert_eq!(
        serialized["fixtureAckRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_FIXTURE_ACK_REF
    );
    assert_eq!(
        serialized["deleteExportPropagationRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_DELETE_EXPORT_PROPAGATION_REF
    );
    assert_eq!(
        serialized["remoteDeleteReadinessRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_REF
    );
    assert_eq!(
        serialized["remoteExportReadinessRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_REF
    );
    assert_eq!(
        serialized["transportDispatchState"],
        "manual-required-blocked"
    );
}

fn assert_remote_delivery_status_provider_child_refs(serialized: &serde_json::Value) {
    assert_eq!(
        serialized["providerRouteRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_PROVIDER_ROUTE_REF
    );
    assert_eq!(
        serialized["childDeviceRouteRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_CHILD_DEVICE_ROUTE_REF
    );
    assert_eq!(
        serialized["providerDeliveryReadinessRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_PROVIDER_READINESS_REF
    );
    assert_eq!(
        serialized["childDeviceDeliveryReadinessRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_CHILD_DEVICE_READINESS_REF
    );
    assert_eq!(
        serialized["providerDeliveryReadinessState"],
        "manual-required-unavailable"
    );
    assert_eq!(
        serialized["childDeviceDeliveryReadinessState"],
        "manual-required-unavailable"
    );
    assert_eq!(
        serialized["crossProcessCustodyStatusRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_CUSTODY_STATUS_REF
    );
    assert_eq!(
        serialized["crossProcessReplayReadinessRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_READINESS_REF
    );
    assert_eq!(
        serialized["remoteRetentionReadinessRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_RETENTION_READINESS_REF
    );
    assert_eq!(
        serialized["remoteDeleteCustodyReadinessRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_CUSTODY_REF
    );
    assert_eq!(
        serialized["remoteExportCustodyReadinessRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_CUSTODY_REF
    );
    assert_eq!(
        serialized["crossProcessReplayRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_REF
    );
    assert_eq!(
        serialized["crossProcessReplayStoreRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_STORE_REF
    );
    assert_eq!(
        serialized["crossProcessReplayCursorRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_CURSOR_REF
    );
    assert_eq!(
        serialized["crossProcessCustodyReadinessState"],
        "manual-required-unavailable"
    );
    assert_eq!(
        serialized["externalCrossProcessTransportRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_REF
    );
    assert_eq!(
        serialized["externalCrossProcessTransportEnvelopeRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ENVELOPE_REF
    );
    assert_eq!(
        serialized["externalCrossProcessTransportAckRef"],
        constants::network_flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ACK_REF
    );
    assert_eq!(
        serialized["externalCrossProcessTransportState"],
        "deterministic-envelope-ack-recorded"
    );
}

fn assert_remote_delivery_status_counts(serialized: &serde_json::Value) {
    assert_eq!(serialized["outboxCandidateCount"], 3);
    assert_eq!(serialized["sourceOutboxCandidateCount"], 3);
    assert_eq!(serialized["preparedNotDispatchedCount"], 3);
    assert_eq!(serialized["blockedDispatchRecordCount"], 3);
    assert_eq!(
        serialized["blockedDispatchRecordsMatchOutboxCandidates"],
        true
    );
    assert_eq!(serialized["fixtureSourceOutboxCandidateCount"], 3);
    assert_eq!(serialized["fixtureDispatchAttemptCount"], 3);
    assert_eq!(serialized["fixtureRemoteAckCount"], 3);
    assert_eq!(serialized["fixtureRecordsMatchOutboxCandidates"], true);
    assert_eq!(serialized["deleteExportReadinessRecordCount"], 3);
    assert_eq!(serialized["remoteDeleteReadyCount"], 3);
    assert_eq!(serialized["remoteExportReadyCount"], 3);
    assert_eq!(serialized["deleteExportRecordsMatchFixtureAcks"], true);
    assert_eq!(serialized["providerDeliveryReadinessRecordCount"], 3);
    assert_eq!(serialized["childDeviceDeliveryReadinessRecordCount"], 3);
    assert_eq!(serialized["providerDeliveryArtifactCount"], 0);
    assert_eq!(serialized["childDeviceDeliveryArtifactCount"], 0);
    assert_eq!(serialized["providerDeliveryRecordsMatchFixtureAcks"], true);
    assert_eq!(
        serialized["childDeviceDeliveryRecordsMatchFixtureAcks"],
        true
    );
    assert_eq!(serialized["crossProcessReplayReadinessRecordCount"], 3);
    assert_eq!(serialized["remoteRetentionReadinessRecordCount"], 3);
    assert_eq!(serialized["remoteDeleteCustodyReadinessRecordCount"], 3);
    assert_eq!(serialized["remoteExportCustodyReadinessRecordCount"], 3);
    assert_eq!(
        serialized["crossProcessCustodyRecordsMatchProviderChildReadiness"],
        true
    );
    assert_eq!(serialized["crossProcessReplayArtifactCount"], 0);
    assert_eq!(serialized["remoteRetentionArtifactCount"], 0);
    assert_eq!(serialized["remoteDeleteCustodyArtifactCount"], 0);
    assert_eq!(serialized["remoteExportCustodyArtifactCount"], 0);
    assert_eq!(serialized["crossProcessReplayRecordCount"], 3);
    assert_eq!(serialized["crossProcessReplayStoreWriteCount"], 3);
    assert_eq!(serialized["crossProcessReplayCursorNextSequence"], 4);
    assert_eq!(
        serialized["crossProcessReplayRecordsMatchDurableEnvelopes"],
        true
    );
    assert_eq!(
        serialized["crossProcessReplayRecordsMatchCustodyReadiness"],
        true
    );
    assert_eq!(serialized["externalCrossProcessTransportRecordCount"], 3);
    assert_eq!(serialized["externalCrossProcessTransportEnvelopeCount"], 3);
    assert_eq!(serialized["externalCrossProcessTransportAckCount"], 3);
    assert_eq!(
        serialized["externalCrossProcessTransportRecordsMatchReplayRecords"],
        true
    );
    assert_eq!(
        serialized["externalCrossProcessTransportAckRecordsMatchEnvelopes"],
        true
    );
}

fn assert_remote_delivery_status_no_product_claims(serialized: &serde_json::Value) {
    assert_eq!(serialized["dispatchReadyCandidateCount"], 0);
    assert_eq!(serialized["dispatchAttemptCount"], 0);
    assert_eq!(serialized["remoteAckCount"], 0);
    assert_eq!(serialized["duplicateDurableEnvelopeRejected"], true);
    assert_eq!(serialized["remoteDeliveryAckImplemented"], false);
    assert_eq!(serialized["crossProcessReplayImplemented"], true);
    assert_eq!(serialized["externalCrossProcessTransportImplemented"], true);
    assert_eq!(serialized["productReadyRemoteDelivery"], false);
    assert_eq!(serialized["hostFilteringClaimed"], false);
    assert_eq!(serialized["enforcementCommandEventCount"], 0);
    assert_eq!(serialized["adapterActionExecutedCount"], 0);
    assert_eq!(serialized["exactUrlAvailableCount"], 0);
    assert_eq!(serialized["searchQueryAvailableCount"], 0);
}

fn remote_delivery_status_fixture() -> NetworkRemoteDeliveryStatus {
    with_cross_process_custody_fixture(NetworkRemoteDeliveryStatus {
        status_ref: flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_STATUS_REF
            .to_string(),
        custody_proof_ref: flow::TEST_BROKER_CUSTODY_PROOF_REF.to_string(),
        publisher_auth_ref: flow::TEST_BROKER_PUBLISHER_AUTH_REF.to_string(),
        subscriber_auth_ref: flow::TEST_BROKER_SUBSCRIBER_AUTH_REF.to_string(),
        encryption_ref: flow::TEST_BROKER_ENCRYPTION_REF.to_string(),
        retention_policy_ref: flow::TEST_BROKER_RETENTION_POLICY_REF.to_string(),
        replay_plan_ref: flow::TEST_BROKER_REPLAY_PLAN_REF.to_string(),
        deletion_plan_ref: flow::TEST_BROKER_DELETION_PLAN_REF.to_string(),
        offset_policy_ref: flow::TEST_BROKER_OFFSET_POLICY_REF.to_string(),
        dedupe_policy_ref: flow::TEST_BROKER_DEDUPE_POLICY_REF.to_string(),
        transport_config_ref: flow::TEST_BROKER_CONFIG_REF.to_string(),
        relay_identity_ref: flow::TEST_FAMILY_HUB_IDENTITY_REF.to_string(),
        relay_policy_ref: flow::TEST_FAMILY_HUB_RELAY_POLICY_REF.to_string(),
        accepted_event_type_count: 3,
        local_idempotency_queue_proved: true,
        dropped_event_dead_letter_count: 1,
        queued_duplicate_rejected: true,
        completed_duplicate_rejected: true,
        event_chain_journal_ref: flow::TEST_REMOTE_EVENT_CHAIN_JOURNAL_REF.to_string(),
        receipt_ledger_ref: flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_LEDGER_REF.to_string(),
        local_receipt_ack_ref: flow::TEST_REMOTE_EVENT_CHAIN_RECEIPT_ACK_REF.to_string(),
        durable_envelope_ref: flow::TEST_REMOTE_DELIVERY_DURABLE_ENVELOPE_REF.to_string(),
        durable_store_ref: flow::TEST_REMOTE_DELIVERY_DURABLE_STORE_REF.to_string(),
        durable_replay_ref: flow::TEST_REMOTE_DELIVERY_DURABLE_REPLAY_REF.to_string(),
        durable_delete_export_ref: flow::TEST_REMOTE_DELIVERY_DURABLE_DELETE_EXPORT_REF.to_string(),
        durable_support_status_ref: flow::TEST_REMOTE_DELIVERY_DURABLE_SUPPORT_STATUS_REF
            .to_string(),
        durable_envelope_ready: true,
        outbox_ref: flow::TEST_REMOTE_DELIVERY_OUTBOX_REF.to_string(),
        outbox_handoff_ref: flow::TEST_REMOTE_DELIVERY_OUTBOX_HANDOFF_REF.to_string(),
        outbox_replay_ref: flow::TEST_REMOTE_DELIVERY_OUTBOX_REPLAY_REF.to_string(),
        outbox_support_status_ref: flow::TEST_REMOTE_DELIVERY_OUTBOX_SUPPORT_STATUS_REF.to_string(),
        transport_dispatch_state_ref: flow::TEST_REMOTE_DELIVERY_TRANSPORT_DISPATCH_STATE_REF
            .to_string(),
        blocked_dispatch_ref: flow::TEST_REMOTE_DELIVERY_DISPATCH_BLOCKED_MANUAL_REF.to_string(),
        future_transport_seam_ref: flow::TEST_REMOTE_DELIVERY_FUTURE_TRANSPORT_SEAM_REF.to_string(),
        fixture_transport_ref: flow::TEST_REMOTE_DELIVERY_FIXTURE_TRANSPORT_REF.to_string(),
        fixture_dispatch_attempt_ref: flow::TEST_REMOTE_DELIVERY_FIXTURE_DISPATCH_ATTEMPT_REF
            .to_string(),
        fixture_ack_ref: flow::TEST_REMOTE_DELIVERY_FIXTURE_ACK_REF.to_string(),
        delete_export_propagation_ref: flow::TEST_REMOTE_DELIVERY_DELETE_EXPORT_PROPAGATION_REF
            .to_string(),
        remote_delete_readiness_ref: flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_REF.to_string(),
        remote_export_readiness_ref: flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_REF.to_string(),
        provider_route_ref: flow::TEST_REMOTE_DELIVERY_PROVIDER_ROUTE_REF.to_string(),
        child_device_route_ref: flow::TEST_REMOTE_DELIVERY_CHILD_DEVICE_ROUTE_REF.to_string(),
        provider_delivery_readiness_ref: flow::TEST_REMOTE_DELIVERY_PROVIDER_READINESS_REF
            .to_string(),
        child_device_delivery_readiness_ref: flow::TEST_REMOTE_DELIVERY_CHILD_DEVICE_READINESS_REF
            .to_string(),
        outbox_candidate_count: 3,
        source_outbox_candidate_count: 3,
        prepared_not_dispatched_count: 3,
        blocked_dispatch_record_count: 3,
        blocked_dispatch_records_match_outbox_candidates: true,
        fixture_source_outbox_candidate_count: 3,
        fixture_dispatch_attempt_count: 3,
        fixture_remote_ack_count: 3,
        fixture_records_match_outbox_candidates: true,
        delete_export_readiness_record_count: 3,
        remote_delete_ready_count: 3,
        remote_export_ready_count: 3,
        delete_export_records_match_fixture_acks: true,
        provider_delivery_readiness_record_count: 3,
        child_device_delivery_readiness_record_count: 3,
        provider_delivery_records_match_fixture_acks: true,
        child_device_delivery_records_match_fixture_acks: true,
        duplicate_durable_envelope_rejected: true,
        outbox_candidates_match_durable_envelopes: true,
        outbox_candidates_match_receipts: true,
        ..NetworkRemoteDeliveryStatus::default()
    })
}

fn with_cross_process_custody_fixture(
    mut status: NetworkRemoteDeliveryStatus,
) -> NetworkRemoteDeliveryStatus {
    status.cross_process_custody_status_ref =
        flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_CUSTODY_STATUS_REF.to_string();
    status.cross_process_replay_readiness_ref =
        flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_READINESS_REF.to_string();
    status.remote_retention_readiness_ref =
        flow::TEST_REMOTE_DELIVERY_REMOTE_RETENTION_READINESS_REF.to_string();
    status.remote_delete_custody_readiness_ref =
        flow::TEST_REMOTE_DELIVERY_REMOTE_DELETE_CUSTODY_REF.to_string();
    status.remote_export_custody_readiness_ref =
        flow::TEST_REMOTE_DELIVERY_REMOTE_EXPORT_CUSTODY_REF.to_string();
    status.cross_process_replay_ref =
        flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_REF.to_string();
    status.cross_process_replay_store_ref =
        flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_STORE_REF.to_string();
    status.cross_process_replay_cursor_ref =
        flow::TEST_REMOTE_DELIVERY_CROSS_PROCESS_REPLAY_CURSOR_REF.to_string();
    status.cross_process_custody_readiness_state =
        NetworkRemoteDeliveryCrossProcessCustodyReadinessState::ManualRequiredUnavailable;
    status.cross_process_replay_readiness_record_count = 3;
    status.remote_retention_readiness_record_count = 3;
    status.remote_delete_custody_readiness_record_count = 3;
    status.remote_export_custody_readiness_record_count = 3;
    status.cross_process_custody_records_match_provider_child_readiness = true;
    status.cross_process_replay_record_count = 3;
    status.cross_process_replay_store_write_count = 3;
    status.cross_process_replay_cursor_next_sequence = 4;
    status.cross_process_replay_records_match_durable_envelopes = true;
    status.cross_process_replay_records_match_custody_readiness = true;
    status.cross_process_replay_implemented = true;
    status.external_cross_process_transport_ref =
        flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_REF.to_string();
    status.external_cross_process_transport_envelope_ref =
        flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ENVELOPE_REF.to_string();
    status.external_cross_process_transport_ack_ref =
        flow::TEST_REMOTE_DELIVERY_EXTERNAL_CROSS_PROCESS_TRANSPORT_ACK_REF.to_string();
    status.external_cross_process_transport_state =
        NetworkRemoteDeliveryExternalCrossProcessTransportState::DeterministicEnvelopeAckRecorded;
    status.external_cross_process_transport_record_count = 3;
    status.external_cross_process_transport_envelope_count = 3;
    status.external_cross_process_transport_ack_count = 3;
    status.external_cross_process_transport_records_match_replay_records = true;
    status.external_cross_process_transport_ack_records_match_envelopes = true;
    status.external_cross_process_transport_implemented = true;
    status
}

#[test]
fn network_runtime_event_contracts_name_exact_event_types() {
    assert_eq!(
        NetworkFlowObservedEvent::EVENT_TYPE,
        constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED
    );
    assert_eq!(
        NetworkDomainObservedEvent::EVENT_TYPE,
        constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED
    );
    assert_eq!(
        NetworkActivityClassifiedEvent::EVENT_TYPE,
        constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED
    );
    assert_eq!(
        NetworkAiAnalysisRequestedEvent::EVENT_TYPE,
        constants::network_flow::EVENT_AI_ANALYSIS_REQUESTED
    );
    assert_eq!(
        NetworkAiAnalysisCompletedEvent::EVENT_TYPE,
        constants::network_flow::EVENT_AI_ANALYSIS_COMPLETED
    );
    assert_eq!(
        NetworkPolicyEvaluationRequestedEvent::EVENT_TYPE,
        constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED
    );
    assert_eq!(
        NetworkPolicyDecisionCompletedEvent::EVENT_TYPE,
        constants::network_flow::EVENT_POLICY_DECISION_COMPLETED
    );
    assert_eq!(
        NetworkEnforcementCommandIssuedEvent::EVENT_TYPE,
        constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED
    );
    assert_eq!(
        NetworkEnforcementResultObservedEvent::EVENT_TYPE,
        constants::network_flow::EVENT_ENFORCEMENT_RESULT_OBSERVED
    );
    assert_eq!(
        NetworkAuditEntryCommittedEvent::EVENT_TYPE,
        constants::network_flow::EVENT_AUDIT_ENTRY_COMMITTED
    );
    assert_eq!(
        NetworkPortalReadModelUpdatedEvent::EVENT_TYPE,
        constants::network_flow::EVENT_PORTAL_READ_MODEL_UPDATED
    );
}

#[test]
fn network_runtime_event_payload_uses_rust_owned_contract_and_key_shapes(
) -> Result<(), EventingError> {
    let payload = network_runtime_event_payload_fixture();

    let contract = payload.contract()?;
    let aggregate_key = payload.aggregate_key()?;
    let idempotency_key = payload.idempotency_key()?;

    assert_eq!(
        contract.event_type.as_str(),
        constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED
    );
    assert_eq!(
        contract.schema_version.value(),
        constants::network_flow::RUNTIME_EVENT_SCHEMA_VERSION
    );
    assert_eq!(
        aggregate_key.as_str(),
        format!(
            "{}{}",
            constants::network_flow::AGGREGATE_NETWORK_FLOW_PREFIX,
            constants::activity_store::TEST_NETWORK_DOMAIN
        )
    );
    assert_eq!(
        idempotency_key.as_str(),
        format!(
            "{}{}-{}-{}",
            constants::network_flow::IDEMPOTENCY_NETWORK_RUNTIME_PREFIX,
            constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED,
            aggregate_key.as_str(),
            constants::activity_store::TEST_FIRST_OBSERVED_AT
        )
    );

    Ok(())
}

#[test]
fn network_flow_observed_event_round_trips_through_typed_event_envelope(
) -> Result<(), EventingError> {
    let payload = network_flow_observed_event();
    let envelope = EventEnvelope::from_event(
        payload.clone(),
        EventMetadata::new(
            CorrelationId::parse("network-flow-envelope-round-trip-1")?,
            EventSource::new(
                EventCustody::parse("test-custody")?,
                RuntimeRole::parse("child-agent")?,
                SourceService::parse("agent-protocol-contract-test")?,
                SourceComponent::parse("network-flow-eventing-contract")?,
                RuntimeInstanceId::parse("network-flow-eventing-contract-1")?,
            ),
        ),
    )?;
    let decoded: EventEnvelope<NetworkFlowObservedEvent> = envelope.store()?.decode()?;

    assert_eq!(
        envelope.contract.event_type.as_str(),
        constants::network_flow::EVENT_NETWORK_FLOW_EVENTING_OBSERVED
    );
    assert_ne!(
        envelope.contract.event_type.as_str(),
        NetworkFlowObservedEvent::EVENT_TYPE
    );
    assert_eq!(
        envelope.aggregate_key.as_str(),
        format!(
            "{}{}",
            constants::network_flow::AGGREGATE_NETWORK_FLOW_PREFIX,
            constants::network_flow::TEST_DEVICE_REF
        )
    );
    assert_eq!(
        envelope.idempotency_key.as_str(),
        format!(
            "{}{}-{}:{}-{}:{}",
            constants::network_flow::IDEMPOTENCY_NETWORK_RUNTIME_PREFIX,
            constants::network_flow::EVENT_NETWORK_FLOW_EVENTING_OBSERVED,
            envelope.aggregate_key.as_str().len(),
            envelope.aggregate_key.as_str(),
            constants::network_flow::TEST_FLOW_EVENT_REF.len(),
            constants::network_flow::TEST_FLOW_EVENT_REF
        )
    );
    assert_eq!(decoded, envelope);
    assert_eq!(decoded.payload, payload);

    Ok(())
}

#[test]
fn network_flow_observed_event_rejects_noncanonical_schema_version() {
    let mut payload = network_flow_observed_event();
    payload.schema_version = constants::network_flow::EVENT_SCHEMA_VERSION + 1;

    assert_eq!(payload.contract(), Err(EventingError::InvalidVersion));
}

#[test]
fn network_flow_observed_event_idempotency_is_device_scoped() -> Result<(), EventingError> {
    let first = network_flow_observed_event();
    let mut second = first.clone();
    second.device_ref = "network-flow-eventing-contract-2".to_string();

    assert_ne!(first.aggregate_key()?, second.aggregate_key()?);
    assert_ne!(first.idempotency_key()?, second.idempotency_key()?);

    Ok(())
}

#[test]
fn network_flow_observed_event_idempotency_disambiguates_hyphenated_components(
) -> Result<(), EventingError> {
    let mut first = network_flow_observed_event();
    first.device_ref = "child-a".to_string();
    first.flow_event_ref = "b-c".to_string();

    let mut second = network_flow_observed_event();
    second.device_ref = "child-a-b".to_string();
    second.flow_event_ref = "c".to_string();

    assert_ne!(first.idempotency_key()?, second.idempotency_key()?);

    Ok(())
}

#[test]
fn network_flow_observed_event_rejects_blank_device_reference() {
    let mut payload = network_flow_observed_event();
    payload.device_ref = "   ".to_string();

    assert_eq!(
        payload.aggregate_key(),
        Err(EventingError::EmptyValue {
            field: "runtime_instance_id"
        })
    );
}

#[test]
fn network_runtime_event_contract_rejects_domain_attribution_without_domain_payload() {
    let mut payload = network_runtime_event_payload_fixture();
    payload.destination_domain = None;

    assert!(matches!(
        payload.contract(),
        Err(EventingError::InvalidValue {
            field: "network_runtime_payload_semantics",
            ..
        })
    ));
}

#[test]
fn network_runtime_event_contract_rejects_process_attribution_without_process_id_payload() {
    let mut payload = network_runtime_event_payload_fixture();
    payload.process_id = None;

    assert!(matches!(
        payload.contract(),
        Err(EventingError::InvalidValue {
            field: "network_runtime_payload_semantics",
            ..
        })
    ));
}

#[test]
fn network_observation_contracts_serialize_claim_boundaries() {
    assert_eq!(
        serialized_field!(
            &network_flow_observed_event(),
            "claimBoundary",
            "exactUrlAvailable"
        ),
        false
    );
    assert_eq!(
        serialized_field!(
            &network_flow_observed_event(),
            "claimBoundary",
            "adapterActionExecuted"
        ),
        false
    );
    assert_eq!(
        serialized_field!(&network_domain_observed_event(), "attribution"),
        serde_json::json!("dns-answer")
    );
    assert_eq!(
        serialized_field!(&network_activity_classified_event(), "activityKind"),
        serde_json::json!("vpn-proxy-tunnel-candidate")
    );
}

#[test]
fn network_ai_and_policy_contracts_serialize_chain_refs() {
    assert_eq!(
        serialized_field!(
            &network_ai_analysis_requested_event(),
            "rawPacketPayloadIncluded",
        ),
        false
    );
    assert_eq!(
        serialized_field!(&network_ai_analysis_completed_event(), "advisoryState"),
        serde_json::json!("completed")
    );
    assert_eq!(
        serialized_field!(&network_policy_evaluation_requested_event(), "dryRun"),
        true
    );
    assert_eq!(
        serialized_field!(&network_policy_decision_completed_event(), "decisionAction",),
        serde_json::json!("manual-review")
    );
}

#[test]
fn network_enforcement_audit_and_portal_contracts_serialize_refs() {
    assert_eq!(
        serialized_field!(
            &network_enforcement_command_issued_event(),
            "policyDecisionRef",
        ),
        constants::network_flow::TEST_POLICY_DECISION_REF
    );
    assert_eq!(
        serialized_field!(
            &network_enforcement_command_issued_event(),
            "enforcementMode",
        ),
        serde_json::json!("dry-run")
    );
    assert_eq!(
        serialized_field!(
            &network_enforcement_result_observed_event(),
            "adapterActionExecuted",
        ),
        false
    );
    assert_eq!(
        serialized_field!(
            &network_audit_entry_committed_event(),
            "enforcementResultRef",
        ),
        constants::network_flow::TEST_ENFORCEMENT_RESULT_REF
    );
    assert_eq!(
        serialized_field!(
            &network_portal_read_model_updated_event(),
            "visibleManualRequired",
        ),
        true
    );
}

#[test]
fn enforcement_command_contract_rejects_missing_policy_decision_ref() {
    let command = serde_json::json!({
        "schemaVersion": constants::network_flow::EVENT_SCHEMA_VERSION,
        "enforcementCommandRef": constants::network_flow::TEST_ENFORCEMENT_COMMAND_REF,
        "previousEventRef": constants::network_flow::TEST_POLICY_DECISION_REF,
        "adapterCapabilityRef": constants::network_flow::TEST_ADAPTER_CAPABILITY_REF,
        "enforcementMode": "manual-required",
        "evidenceRefs": [constants::network_flow::TEST_FLOW_EVIDENCE_REF],
        "rollbackRef": null
    });

    let parsed = serde_json::from_value::<NetworkEnforcementCommandIssuedEvent>(command);

    assert_eq!(
        parsed.err().map(|error| error.classify()),
        Some(serde_json::error::Category::Data)
    );
}

#[test]
fn manual_required_enforcement_result_keeps_adapter_action_false() {
    let result = NetworkEnforcementResultObservedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        enforcement_result_ref: constants::network_flow::TEST_ENFORCEMENT_RESULT_REF.to_string(),
        enforcement_command_ref: constants::network_flow::TEST_ENFORCEMENT_COMMAND_REF.to_string(),
        previous_event_ref: constants::network_flow::TEST_ENFORCEMENT_COMMAND_REF.to_string(),
        result_status: NetworkEnforcementResultStatus::ManualRequired,
        adapter_action_executed: false,
        rollback_ref: None,
        unavailable_reason_code: Some(
            constants::network_flow::UNAVAILABLE_REASON_MANUAL_REQUIRED.to_string(),
        ),
    };

    let serialized = serde_json::to_value(result).unwrap_or_default();

    assert_eq!(serialized["resultStatus"], "manual-required");
    assert_eq!(serialized["adapterActionExecuted"], false);
    assert_eq!(
        serialized["unavailableReasonCode"],
        constants::network_flow::UNAVAILABLE_REASON_MANUAL_REQUIRED
    );
}

fn network_runtime_event_payload_fixture() -> NetworkRuntimeEventPayload {
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
        evidence_grade_contract: NetworkEvidenceGrade::B,
        ai_audit_state: NetworkAiAuditState::NotRequested,
        risk_budget_state: NetworkRiskBudgetState::ObserveOnly,
        intervention_state: NetworkInterventionState::DryRunOnly,
        policy_action: NetworkPolicyDecisionAction::Observe,
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
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
    }
}
