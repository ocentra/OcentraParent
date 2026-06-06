use super::{
    constants, ActivityNetworkEndpoint, ActivityNetworkFlowCounters,
    ActivityNetworkFlowObservation, ActivityNetworkFlowReadModel, NetworkActivityClassifiedEvent,
    NetworkAiAnalysisCompletedEvent, NetworkAiAnalysisRequestedEvent,
    NetworkAuditEntryCommittedEvent, NetworkDomainObservedEvent,
    NetworkEnforcementCommandIssuedEvent, NetworkEnforcementResultObservedEvent,
    NetworkEnforcementResultStatus, NetworkFlowObservedEvent,
    NetworkLocalAiRuntimeResultBridgeState, NetworkLocalAiRuntimeResultQueueStatus,
    NetworkLocalAiRuntimeResultStatus, NetworkPolicyDecisionCompletedEvent,
    NetworkPolicyEvaluationRequestedEvent, NetworkPortalReadModelUpdatedEvent,
    NetworkRemoteDeliveryStatus, NetworkRemoteDeliveryStatusState, NetworkRuntimeEventContract,
    NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE, NETWORK_FLOW_SCHEMA_VERSION,
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
    assert_eq!(
        serialized["deletedEvidenceReferenceIds"][0],
        constants::activity_store::TEST_NETWORK_EVENT_ID
    );
    assert_eq!(serialized["tombstoneRows"], 1);
    assert_eq!(serialized["rows"].as_array().map(Vec::len), Some(0));
}

fn network_remote_delivery_status_fixture() -> NetworkRemoteDeliveryStatus {
    NetworkRemoteDeliveryStatus {
        status_ref: constants::network_flow::TEST_REMOTE_DELIVERY_STATUS_REF.to_string(),
        broker_status: NetworkRemoteDeliveryStatusState::RequirementsSatisfiedButNotImplemented,
        family_hub_status: NetworkRemoteDeliveryStatusState::RequirementsSatisfiedButNotImplemented,
        custody_proof_ref: constants::network_flow::TEST_BROKER_CUSTODY_PROOF_REF.to_string(),
        publisher_auth_ref: constants::network_flow::TEST_BROKER_PUBLISHER_AUTH_REF.to_string(),
        subscriber_auth_ref: constants::network_flow::TEST_BROKER_SUBSCRIBER_AUTH_REF.to_string(),
        encryption_ref: constants::network_flow::TEST_BROKER_ENCRYPTION_REF.to_string(),
        retention_policy_ref: constants::network_flow::TEST_BROKER_RETENTION_POLICY_REF.to_string(),
        replay_plan_ref: constants::network_flow::TEST_BROKER_REPLAY_PLAN_REF.to_string(),
        deletion_plan_ref: constants::network_flow::TEST_BROKER_DELETION_PLAN_REF.to_string(),
        offset_policy_ref: constants::network_flow::TEST_BROKER_OFFSET_POLICY_REF.to_string(),
        dedupe_policy_ref: constants::network_flow::TEST_BROKER_DEDUPE_POLICY_REF.to_string(),
        transport_config_ref: constants::network_flow::TEST_BROKER_CONFIG_REF.to_string(),
        relay_identity_ref: constants::network_flow::TEST_FAMILY_HUB_IDENTITY_REF.to_string(),
        relay_policy_ref: constants::network_flow::TEST_FAMILY_HUB_RELAY_POLICY_REF.to_string(),
        broker_missing_artifact_count: 0,
        family_hub_missing_artifact_count: 0,
        accepted_event_type_count: 3,
        local_idempotency_queue_proved: true,
        dropped_event_dead_letter_count: 1,
        queued_duplicate_rejected: true,
        completed_duplicate_rejected: true,
        cross_process_replay_ref:
            constants::network_flow::TEST_REMOTE_LIFECYCLE_CROSS_PROCESS_REPLAY_REF.to_string(),
        remote_retention_delete_export_ref:
            constants::network_flow::TEST_REMOTE_LIFECYCLE_RETENTION_DELETE_EXPORT_REF.to_string(),
        remote_delivery_ack_ref: constants::network_flow::TEST_REMOTE_LIFECYCLE_DELIVERY_ACK_REF
            .to_string(),
        remote_lifecycle_followup_ref: constants::network_flow::TEST_REMOTE_LIFECYCLE_FOLLOWUP_REF
            .to_string(),
        remote_lifecycle_missing_artifact_count: 3,
        remote_lifecycle_manual_required: true,
        durable_envelope_schema_ref:
            constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_SCHEMA_REF.to_string(),
        durable_envelope_journal_ref:
            constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_JOURNAL_REF.to_string(),
        durable_envelope_replay_readiness_ref:
            constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_REPLAY_REF.to_string(),
        durable_envelope_delete_export_readiness_ref:
            constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_DELETE_EXPORT_REF.to_string(),
        durable_envelope_support_status_ref:
            constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_SUPPORT_STATUS_REF.to_string(),
        durable_envelope_ready: true,
        durable_envelope_missing_artifact_count: 0,
        external_transport_delivery_implemented: false,
        family_hub_delivery_implemented: false,
        cross_process_replay_implemented: false,
        remote_retention_delete_export_propagation_implemented: false,
        provider_delivery_implemented: false,
        child_device_delivery_implemented: false,
        product_ready_claimed: false,
        policy_authority: false,
        side_effect_authority: false,
        enforcement_command_event_count: 0,
        adapter_action_executed_count: 0,
    }
}

#[test]
fn network_remote_delivery_status_serializes_false_claim_boundary() {
    let status = network_remote_delivery_status_fixture();
    let serialized = serde_json::to_value(status).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["status_ref"],
        constants::network_flow::TEST_REMOTE_DELIVERY_STATUS_REF
    );
    assert_eq!(
        serialized["broker_status"],
        "RequirementsSatisfiedButNotImplemented"
    );
    assert_eq!(serialized["accepted_event_type_count"], 3);
    assert_eq!(serialized["dropped_event_dead_letter_count"], 1);
    assert_eq!(
        serialized["cross_process_replay_ref"],
        constants::network_flow::TEST_REMOTE_LIFECYCLE_CROSS_PROCESS_REPLAY_REF
    );
    assert_eq!(
        serialized["remote_retention_delete_export_ref"],
        constants::network_flow::TEST_REMOTE_LIFECYCLE_RETENTION_DELETE_EXPORT_REF
    );
    assert_eq!(
        serialized["remote_delivery_ack_ref"],
        constants::network_flow::TEST_REMOTE_LIFECYCLE_DELIVERY_ACK_REF
    );
    assert_eq!(
        serialized["remote_lifecycle_followup_ref"],
        constants::network_flow::TEST_REMOTE_LIFECYCLE_FOLLOWUP_REF
    );
    assert_eq!(serialized["remote_lifecycle_missing_artifact_count"], 3);
    assert_eq!(serialized["remote_lifecycle_manual_required"], true);
    assert_eq!(
        serialized["durable_envelope_schema_ref"],
        constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_SCHEMA_REF
    );
    assert_eq!(
        serialized["durable_envelope_journal_ref"],
        constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_JOURNAL_REF
    );
    assert_eq!(
        serialized["durable_envelope_replay_readiness_ref"],
        constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_REPLAY_REF
    );
    assert_eq!(
        serialized["durable_envelope_delete_export_readiness_ref"],
        constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_DELETE_EXPORT_REF
    );
    assert_eq!(
        serialized["durable_envelope_support_status_ref"],
        constants::network_flow::TEST_REMOTE_DURABLE_ENVELOPE_SUPPORT_STATUS_REF
    );
    assert_eq!(serialized["durable_envelope_ready"], true);
    assert_eq!(serialized["durable_envelope_missing_artifact_count"], 0);
    assert_eq!(serialized["external_transport_delivery_implemented"], false);
    assert_eq!(serialized["family_hub_delivery_implemented"], false);
    assert_eq!(serialized["cross_process_replay_implemented"], false);
    assert_eq!(
        serialized["remote_retention_delete_export_propagation_implemented"],
        false
    );
    assert_eq!(serialized["provider_delivery_implemented"], false);
    assert_eq!(serialized["child_device_delivery_implemented"], false);
    assert_eq!(serialized["product_ready_claimed"], false);
    assert_eq!(serialized["policy_authority"], false);
    assert_eq!(serialized["side_effect_authority"], false);
    assert_eq!(serialized["enforcement_command_event_count"], 0);
    assert_eq!(serialized["adapter_action_executed_count"], 0);
}

#[test]
fn network_local_ai_runtime_result_status_serializes_no_claim_boundary() {
    let status = NetworkLocalAiRuntimeResultStatus {
        status_ref: constants::network_flow::TEST_LOCAL_AI_RUNTIME_RESULT_STATUS_REF.to_string(),
        bridge_state: NetworkLocalAiRuntimeResultBridgeState::ResultReady,
        queue_status: NetworkLocalAiRuntimeResultQueueStatus::Queued,
        trigger_ref: constants::network_flow::TEST_LOCAL_AI_TRIGGER_REF.to_string(),
        queue_job_ref: Some(constants::network_flow::TEST_LOCAL_AI_QUEUE_JOB_REF.to_string()),
        queue_ref: Some(constants::network_flow::TEST_LOCAL_AI_QUEUE_REF.to_string()),
        model_runtime_ref: Some(
            constants::network_flow::TEST_LOCAL_AI_MODEL_RUNTIME_REF.to_string(),
        ),
        local_ai_result_ref: Some(constants::network_flow::TEST_LOCAL_AI_RESULT_REF.to_string()),
        runtime_reference_id: Some(
            constants::network_flow::TEST_LOCAL_AI_RUNTIME_REFERENCE_ID.to_string(),
        ),
        model_reference: Some(constants::network_flow::TEST_LOCAL_AI_MODEL_REF.to_string()),
        model_version_ref: Some(
            constants::network_flow::TEST_LOCAL_AI_MODEL_VERSION_REF.to_string(),
        ),
        prompt_template_ref: constants::network_flow::TEST_LOCAL_AI_PROMPT_TEMPLATE_REF.to_string(),
        policy_context_ref: constants::network_flow::TEST_LOCAL_AI_POLICY_CONTEXT_REF.to_string(),
        parent_rule_refs: vec![constants::network_flow::TEST_PARENT_RULE_REF.to_string()],
        evidence_refs: vec![constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_string()],
        summary_refs: vec![constants::network_flow::TEST_LOCAL_AI_NETWORK_SUMMARY_REF.to_string()],
        managed_browser_exact_url_evidence_refs: vec![
            constants::network_flow::TEST_LOCAL_AI_MANAGED_BROWSER_EXACT_URL_EVIDENCE_REF
                .to_string(),
        ],
        output_summary_ref: Some(
            constants::network_flow::TEST_LOCAL_AI_OUTPUT_SUMMARY_REF.to_string(),
        ),
        local_runtime_result_observed: true,
        audit_input_ready: true,
        local_model_output_available: true,
        model_execution_proved: false,
        raw_pcap_available: false,
        exact_url_claimed: false,
        decrypted_payload_available: false,
        page_content_available: false,
        private_message_available: false,
        search_query_available: false,
        remote_ai_used: false,
        policy_authority: false,
        adapter_authority: false,
        enforcement_commands_published: 0,
    };

    let serialized = serde_json::to_value(status).expect(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized["status_ref"],
        constants::network_flow::TEST_LOCAL_AI_RUNTIME_RESULT_STATUS_REF
    );
    assert_eq!(serialized["bridge_state"], "ResultReady");
    assert_eq!(serialized["queue_status"], "Queued");
    assert_eq!(
        serialized["output_summary_ref"],
        constants::network_flow::TEST_LOCAL_AI_OUTPUT_SUMMARY_REF
    );
    assert_eq!(serialized["local_runtime_result_observed"], true);
    assert_eq!(serialized["audit_input_ready"], true);
    assert_eq!(serialized["local_model_output_available"], true);
    assert_eq!(serialized["model_execution_proved"], false);
    assert_eq!(serialized["raw_pcap_available"], false);
    assert_eq!(serialized["exact_url_claimed"], false);
    assert_eq!(serialized["remote_ai_used"], false);
    assert_eq!(serialized["policy_authority"], false);
    assert_eq!(serialized["adapter_authority"], false);
    assert_eq!(serialized["enforcement_commands_published"], 0);
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
