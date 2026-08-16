use crate::{
    constants, NetworkActivityClassifiedEvent, NetworkActivityKind, NetworkAiAdvisoryState,
    NetworkAiAnalysisCompletedEvent, NetworkAiAnalysisRequestedEvent,
    NetworkAuditEntryCommittedEvent, NetworkAuditOutcome, NetworkClaimBoundary,
    NetworkDomainAttributionKind, NetworkDomainObservedEvent, NetworkEnforcementCommandIssuedEvent,
    NetworkEnforcementMode, NetworkEnforcementResultObservedEvent, NetworkEnforcementResultStatus,
    NetworkEvidenceGrade, NetworkFlowObservedEvent, NetworkPolicyDecisionAction,
    NetworkPolicyDecisionCompletedEvent, NetworkPolicyEvaluationRequestedEvent,
    NetworkPortalReadModelUpdatedEvent, NetworkPortalUpdateKind,
    NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE,
};

pub(super) fn network_flow_observed_event() -> NetworkFlowObservedEvent {
    NetworkFlowObservedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        flow_event_ref: constants::network_flow::TEST_FLOW_EVENT_REF.to_string(),
        observed_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        device_ref: constants::network_flow::TEST_DEVICE_REF.to_string(),
        flow_evidence_ref: constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_string(),
        custody: NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        evidence_grade: NetworkEvidenceGrade::B,
        claim_boundary: no_claim_boundary(),
    }
}

pub(super) fn network_domain_observed_event() -> NetworkDomainObservedEvent {
    NetworkDomainObservedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        domain_event_ref: constants::network_flow::TEST_DOMAIN_EVENT_REF.to_string(),
        previous_event_ref: constants::network_flow::TEST_FLOW_EVENT_REF.to_string(),
        flow_evidence_ref: constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_string(),
        domain_evidence_ref: constants::network_flow::TEST_DOMAIN_EVIDENCE_REF.to_string(),
        attribution: NetworkDomainAttributionKind::DnsAnswer,
        evidence_grade: NetworkEvidenceGrade::B,
        uncertainty_codes: vec![
            constants::network_flow::UNCERTAINTY_NETWORK_ONLY_NO_EXACT_URL.to_string(),
        ],
        claim_boundary: no_claim_boundary(),
    }
}

pub(super) fn network_activity_classified_event() -> NetworkActivityClassifiedEvent {
    NetworkActivityClassifiedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        classification_event_ref: constants::network_flow::TEST_CLASSIFICATION_EVENT_REF
            .to_string(),
        previous_event_ref: constants::network_flow::TEST_DOMAIN_EVENT_REF.to_string(),
        evidence_refs: vec![
            constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_string(),
            constants::network_flow::TEST_DOMAIN_EVIDENCE_REF.to_string(),
        ],
        activity_kind: NetworkActivityKind::VpnProxyTunnelCandidate,
        confidence: 0.72,
        evidence_grade: NetworkEvidenceGrade::C,
        uncertainty_codes: vec![
            constants::network_flow::UNCERTAINTY_NETWORK_ONLY_NO_EXACT_URL.to_string(),
        ],
    }
}

pub(super) fn network_ai_analysis_requested_event() -> NetworkAiAnalysisRequestedEvent {
    NetworkAiAnalysisRequestedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        ai_request_ref: constants::network_flow::TEST_AI_REQUEST_REF.to_string(),
        previous_event_ref: constants::network_flow::TEST_CLASSIFICATION_EVENT_REF.to_string(),
        evidence_refs: vec![
            constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_string(),
            constants::network_flow::TEST_DOMAIN_EVIDENCE_REF.to_string(),
        ],
        prompt_template_ref: constants::network_flow::TEST_PROMPT_TEMPLATE_REF.to_string(),
        custody: NETWORK_FLOW_CUSTODY_CHILD_DEVICE_QUERY_STORE.to_string(),
        raw_packet_payload_included: false,
    }
}

pub(super) fn network_ai_analysis_completed_event() -> NetworkAiAnalysisCompletedEvent {
    NetworkAiAnalysisCompletedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        ai_analysis_ref: constants::network_flow::TEST_AI_ANALYSIS_REF.to_string(),
        ai_request_ref: constants::network_flow::TEST_AI_REQUEST_REF.to_string(),
        previous_event_ref: constants::network_flow::TEST_AI_REQUEST_REF.to_string(),
        advisory_state: NetworkAiAdvisoryState::Completed,
        evidence_refs: vec![
            constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_string(),
            constants::network_flow::TEST_DOMAIN_EVIDENCE_REF.to_string(),
        ],
        unsupported_claims: vec![
            constants::network_flow::UNSUPPORTED_CLAIM_DECRYPTED_HTTPS_PAYLOAD.to_string(),
        ],
    }
}

pub(super) fn network_policy_evaluation_requested_event() -> NetworkPolicyEvaluationRequestedEvent {
    NetworkPolicyEvaluationRequestedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        policy_evaluation_ref: constants::network_flow::TEST_POLICY_EVALUATION_REF.to_string(),
        previous_event_ref: constants::network_flow::TEST_AI_ANALYSIS_REF.to_string(),
        evidence_refs: vec![
            constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_string(),
            constants::network_flow::TEST_DOMAIN_EVIDENCE_REF.to_string(),
        ],
        ai_analysis_ref: Some(constants::network_flow::TEST_AI_ANALYSIS_REF.to_string()),
        parent_rule_refs: vec![constants::network_flow::TEST_PARENT_RULE_REF.to_string()],
        dry_run: true,
    }
}

pub(super) fn network_policy_decision_completed_event() -> NetworkPolicyDecisionCompletedEvent {
    NetworkPolicyDecisionCompletedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        policy_decision_ref: constants::network_flow::TEST_POLICY_DECISION_REF.to_string(),
        policy_evaluation_ref: constants::network_flow::TEST_POLICY_EVALUATION_REF.to_string(),
        previous_event_ref: constants::network_flow::TEST_POLICY_EVALUATION_REF.to_string(),
        decision_action: NetworkPolicyDecisionAction::ManualReview,
        evidence_refs: vec![
            constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_string(),
            constants::network_flow::TEST_DOMAIN_EVIDENCE_REF.to_string(),
        ],
        parent_rule_refs: vec![constants::network_flow::TEST_PARENT_RULE_REF.to_string()],
        adapter_capability_required: true,
    }
}

pub(super) fn network_enforcement_command_issued_event() -> NetworkEnforcementCommandIssuedEvent {
    NetworkEnforcementCommandIssuedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        enforcement_command_ref: constants::network_flow::TEST_ENFORCEMENT_COMMAND_REF.to_string(),
        previous_event_ref: constants::network_flow::TEST_POLICY_DECISION_REF.to_string(),
        policy_decision_ref: constants::network_flow::TEST_POLICY_DECISION_REF.to_string(),
        adapter_capability_ref: constants::network_flow::TEST_ADAPTER_CAPABILITY_REF.to_string(),
        enforcement_mode: NetworkEnforcementMode::DryRun,
        evidence_refs: vec![
            constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_string(),
            constants::network_flow::TEST_DOMAIN_EVIDENCE_REF.to_string(),
        ],
        rollback_ref: Some(constants::network_flow::TEST_ROLLBACK_REF.to_string()),
    }
}

pub(super) fn network_enforcement_result_observed_event() -> NetworkEnforcementResultObservedEvent {
    NetworkEnforcementResultObservedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        enforcement_result_ref: constants::network_flow::TEST_ENFORCEMENT_RESULT_REF.to_string(),
        enforcement_command_ref: constants::network_flow::TEST_ENFORCEMENT_COMMAND_REF.to_string(),
        previous_event_ref: constants::network_flow::TEST_ENFORCEMENT_COMMAND_REF.to_string(),
        result_status: NetworkEnforcementResultStatus::DryRun,
        adapter_action_executed: false,
        rollback_ref: Some(constants::network_flow::TEST_ROLLBACK_REF.to_string()),
        unavailable_reason_code: None,
    }
}

pub(super) fn network_audit_entry_committed_event() -> NetworkAuditEntryCommittedEvent {
    NetworkAuditEntryCommittedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        audit_entry_ref: constants::network_flow::TEST_AUDIT_ENTRY_REF.to_string(),
        previous_event_ref: constants::network_flow::TEST_ENFORCEMENT_RESULT_REF.to_string(),
        policy_decision_ref: constants::network_flow::TEST_POLICY_DECISION_REF.to_string(),
        enforcement_command_ref: Some(
            constants::network_flow::TEST_ENFORCEMENT_COMMAND_REF.to_string(),
        ),
        enforcement_result_ref: Some(
            constants::network_flow::TEST_ENFORCEMENT_RESULT_REF.to_string(),
        ),
        evidence_refs: vec![
            constants::network_flow::TEST_FLOW_EVIDENCE_REF.to_string(),
            constants::network_flow::TEST_DOMAIN_EVIDENCE_REF.to_string(),
        ],
        audit_outcome: NetworkAuditOutcome::Committed,
    }
}

pub(super) fn network_portal_read_model_updated_event() -> NetworkPortalReadModelUpdatedEvent {
    NetworkPortalReadModelUpdatedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        read_model_ref: constants::network_flow::TEST_PORTAL_READ_MODEL_REF.to_string(),
        previous_event_ref: constants::network_flow::TEST_AUDIT_ENTRY_REF.to_string(),
        audit_entry_ref: constants::network_flow::TEST_AUDIT_ENTRY_REF.to_string(),
        update_kind: NetworkPortalUpdateKind::NetworkReadModel,
        visible_manual_required: true,
        visible_unavailable: false,
    }
}

fn no_claim_boundary() -> NetworkClaimBoundary {
    NetworkClaimBoundary {
        exact_url_available: false,
        decrypted_https_payload_available: false,
        message_content_available: false,
        search_query_available: false,
        adapter_action_executed: false,
    }
}
