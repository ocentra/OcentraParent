use super::{
    constants, ActivityNetworkEndpoint, ActivityNetworkFlowCounters,
    ActivityNetworkFlowObservation, ActivityNetworkFlowReadModel, NetworkActivityClassifiedEvent,
    NetworkAiAnalysisCompletedEvent, NetworkAiAnalysisRequestedEvent,
    NetworkAuditEntryCommittedEvent, NetworkDomainObservedEvent,
    NetworkEnforcementCommandIssuedEvent, NetworkEnforcementResultObservedEvent,
    NetworkEnforcementResultStatus, NetworkFlowObservedEvent, NetworkPolicyDecisionCompletedEvent,
    NetworkPolicyEvaluationRequestedEvent, NetworkPortalReadModelUpdatedEvent,
    NetworkRuntimeEventContract, NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE,
    NETWORK_FLOW_SCHEMA_VERSION,
};

#[path = "network_flow_event_fixtures.rs"]
mod network_flow_event_fixtures;

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

    let serialized =
        serde_json::to_value(observation).expect(constants::error::AGENT_EVENT_SERIALIZES);

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
        capability_status: constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS
            .to_string(),
        rows: Vec::new(),
    };

    let serialized =
        serde_json::to_value(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["custody"],
        NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE
    );
    assert_eq!(
        serialized["capabilityStatus"],
        constants::activity_capture::CAPABILITY_STATUS_NO_NETWORK_OBSERVATIONS
    );
    assert_eq!(serialized["rows"].as_array().map(Vec::len), Some(0));
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
fn network_observation_contracts_serialize_claim_boundaries() {
    assert_eq!(
        serialized_field(
            &network_flow_observed_event(),
            "claimBoundary",
            "exactUrlAvailable"
        ),
        false
    );
    assert_eq!(
        serialized_field(
            &network_flow_observed_event(),
            "claimBoundary",
            "adapterActionExecuted"
        ),
        false
    );
    assert_eq!(
        serialized_field(&network_domain_observed_event(), "attribution", ""),
        serde_json::json!("dns-answer")
    );
    assert_eq!(
        serialized_field(&network_activity_classified_event(), "activityKind", ""),
        serde_json::json!("vpn-proxy-tunnel-candidate")
    );
}

#[test]
fn network_ai_and_policy_contracts_serialize_chain_refs() {
    assert_eq!(
        serialized_field(
            &network_ai_analysis_requested_event(),
            "rawPacketPayloadIncluded",
            ""
        ),
        false
    );
    assert_eq!(
        serialized_field(&network_ai_analysis_completed_event(), "advisoryState", ""),
        serde_json::json!("completed")
    );
    assert_eq!(
        serialized_field(&network_policy_evaluation_requested_event(), "dryRun", ""),
        true
    );
    assert_eq!(
        serialized_field(
            &network_policy_decision_completed_event(),
            "decisionAction",
            ""
        ),
        serde_json::json!("manual-review")
    );
}

#[test]
fn network_enforcement_audit_and_portal_contracts_serialize_refs() {
    assert_eq!(
        serialized_field(
            &network_enforcement_command_issued_event(),
            "policyDecisionRef",
            ""
        ),
        constants::network_flow::TEST_POLICY_DECISION_REF
    );
    assert_eq!(
        serialized_field(
            &network_enforcement_command_issued_event(),
            "enforcementMode",
            ""
        ),
        serde_json::json!("dry-run")
    );
    assert_eq!(
        serialized_field(
            &network_enforcement_result_observed_event(),
            "adapterActionExecuted",
            ""
        ),
        false
    );
    assert_eq!(
        serialized_field(
            &network_audit_entry_committed_event(),
            "enforcementResultRef",
            ""
        ),
        constants::network_flow::TEST_ENFORCEMENT_RESULT_REF
    );
    assert_eq!(
        serialized_field(
            &network_portal_read_model_updated_event(),
            "visibleManualRequired",
            ""
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

    assert!(parsed.is_err());
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

    let serialized = serde_json::to_value(result).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["resultStatus"], "manual-required");
    assert_eq!(serialized["adapterActionExecuted"], false);
    assert_eq!(
        serialized["unavailableReasonCode"],
        constants::network_flow::UNAVAILABLE_REASON_MANUAL_REQUIRED
    );
}

fn serialized_field<T>(value: &T, field: &str, nested: &str) -> serde_json::Value
where
    T: serde::Serialize,
{
    let serialized = serde_json::to_value(value).expect(constants::error::AGENT_EVENT_SERIALIZES);
    if nested.is_empty() {
        return serialized[field].clone();
    }
    serialized[field][nested].clone()
}
