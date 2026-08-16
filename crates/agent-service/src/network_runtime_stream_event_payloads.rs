use serde_json::Value;

use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::network_flow::NetworkActivityClassifiedEvent;
use ocentra_parent_agent_protocol::network_flow::NetworkActivityKind;
use ocentra_parent_agent_protocol::network_flow::NetworkAiAnalysisCompletedEvent;
use ocentra_parent_agent_protocol::network_flow::NetworkAiAnalysisRequestedEvent;
use ocentra_parent_agent_protocol::network_flow::NetworkAuditEntryCommittedEvent;
use ocentra_parent_agent_protocol::network_flow::NetworkAuditOutcome;
use ocentra_parent_agent_protocol::network_flow::NetworkDomainObservedEvent;
use ocentra_parent_agent_protocol::network_flow::NetworkEnforcementCommandIssuedEvent;
use ocentra_parent_agent_protocol::network_flow::NetworkEnforcementResultObservedEvent;
use ocentra_parent_agent_protocol::network_flow::NetworkFlowObservedEvent;
use ocentra_parent_agent_protocol::network_flow::NetworkInterventionState;
use ocentra_parent_agent_protocol::network_flow::NetworkPolicyDecisionCompletedEvent;
use ocentra_parent_agent_protocol::network_flow::NetworkPolicyEvaluationRequestedEvent;
use ocentra_parent_agent_protocol::network_flow::NetworkPortalReadModelUpdatedEvent;
use ocentra_parent_agent_protocol::network_flow::NetworkRuntimeEventPayload;
use ocentra_parent_agent_protocol::policy_constants;

use crate::network_runtime_stream_event_values as values;

pub(crate) fn network_flow_observed(
    event_ref: &values::NetworkRuntimeStreamRef,
    payload: &NetworkRuntimeEventPayload,
) -> Value {
    values::json_value(NetworkFlowObservedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        flow_event_ref: event_ref.0.clone(),
        observed_at: payload.observed_at.clone(),
        device_ref: constants::peer::LOCAL_DEV_AGENT.to_string(),
        flow_evidence_ref: payload.evidence_ref.clone(),
        custody: values::custody(payload).0,
        evidence_grade: values::evidence_grade(payload),
        claim_boundary: values::no_claim_boundary(),
    })
}

pub(crate) fn network_domain_observed(
    event_ref: &values::NetworkRuntimeStreamRef,
    payload: &NetworkRuntimeEventPayload,
) -> Value {
    values::json_value(NetworkDomainObservedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        domain_event_ref: event_ref.0.clone(),
        previous_event_ref: values::previous_event_ref(payload).0,
        flow_evidence_ref: payload.evidence_ref.clone(),
        domain_evidence_ref: event_ref.0.clone(),
        attribution: values::domain_attribution(payload),
        evidence_grade: values::evidence_grade(payload),
        uncertainty_codes: values::uncertainty_codes()
            .into_iter()
            .map(|value| value.0)
            .collect(),
        claim_boundary: values::no_claim_boundary(),
    })
}

pub(crate) fn network_activity_classified(
    event_ref: &values::NetworkRuntimeStreamRef,
    payload: &NetworkRuntimeEventPayload,
) -> Value {
    values::json_value(NetworkActivityClassifiedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        classification_event_ref: event_ref.0.clone(),
        previous_event_ref: values::previous_event_ref(payload).0,
        evidence_refs: values::evidence_refs(payload)
            .into_iter()
            .map(|value| value.0)
            .collect(),
        activity_kind: NetworkActivityKind::Unknown,
        confidence: values::confidence(payload),
        evidence_grade: values::evidence_grade(payload),
        uncertainty_codes: values::uncertainty_codes()
            .into_iter()
            .map(|value| value.0)
            .collect(),
    })
}

pub(crate) fn network_ai_analysis_requested(
    event_ref: &values::NetworkRuntimeStreamRef,
    payload: &NetworkRuntimeEventPayload,
) -> Value {
    values::json_value(NetworkAiAnalysisRequestedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        ai_request_ref: event_ref.0.clone(),
        previous_event_ref: values::previous_event_ref(payload).0,
        evidence_refs: values::evidence_refs(payload)
            .into_iter()
            .map(|value| value.0)
            .collect(),
        prompt_template_ref: constants::network_flow::TEST_PROMPT_TEMPLATE_REF.to_string(),
        custody: values::custody(payload).0,
        raw_packet_payload_included: false,
    })
}

pub(crate) fn network_ai_analysis_completed(
    event_ref: &values::NetworkRuntimeStreamRef,
    payload: &NetworkRuntimeEventPayload,
) -> Value {
    let ai_request_ref = payload
        .ai_request_ref
        .as_ref()
        .map(|value| values::NetworkRuntimeStreamRef(value.clone()));
    values::json_value(NetworkAiAnalysisCompletedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        ai_analysis_ref: event_ref.0.clone(),
        ai_request_ref: values::ref_or_current(&ai_request_ref, event_ref).0,
        previous_event_ref: values::previous_event_ref(payload).0,
        advisory_state: values::ai_advisory_state(payload),
        evidence_refs: values::evidence_refs(payload)
            .into_iter()
            .map(|value| value.0)
            .collect(),
        unsupported_claims: vec![
            constants::network_flow::UNSUPPORTED_CLAIM_DECRYPTED_HTTPS_PAYLOAD.to_string(),
        ],
    })
}

pub(crate) fn network_policy_evaluation_requested(
    event_ref: &values::NetworkRuntimeStreamRef,
    payload: &NetworkRuntimeEventPayload,
) -> Value {
    values::json_value(NetworkPolicyEvaluationRequestedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        policy_evaluation_ref: event_ref.0.clone(),
        previous_event_ref: values::previous_event_ref(payload).0,
        evidence_refs: values::evidence_refs(payload)
            .into_iter()
            .map(|value| value.0)
            .collect(),
        ai_analysis_ref: payload.ai_analysis_ref.clone(),
        parent_rule_refs: values::parent_rule_refs()
            .into_iter()
            .map(|value| value.0)
            .collect(),
        dry_run: true,
    })
}

pub(crate) fn network_policy_decision_completed(
    event_ref: &values::NetworkRuntimeStreamRef,
    payload: &NetworkRuntimeEventPayload,
) -> Value {
    let policy_evaluation_ref = payload
        .policy_evaluation_ref
        .as_ref()
        .map(|value| values::NetworkRuntimeStreamRef(value.clone()));
    values::json_value(NetworkPolicyDecisionCompletedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        policy_decision_ref: event_ref.0.clone(),
        policy_evaluation_ref: values::ref_or_current(&policy_evaluation_ref, event_ref).0,
        previous_event_ref: values::previous_event_ref(payload).0,
        decision_action: values::policy_decision_action(payload),
        evidence_refs: values::evidence_refs(payload)
            .into_iter()
            .map(|value| value.0)
            .collect(),
        parent_rule_refs: values::parent_rule_refs()
            .into_iter()
            .map(|value| value.0)
            .collect(),
        adapter_capability_required: payload.intervention_state
            == NetworkInterventionState::DryRunOnly,
    })
}

pub(crate) fn network_enforcement_command_issued(
    event_ref: &values::NetworkRuntimeStreamRef,
    payload: &NetworkRuntimeEventPayload,
) -> Value {
    let policy_decision_ref = payload
        .policy_decision_ref
        .as_ref()
        .map(|value| values::NetworkRuntimeStreamRef(value.clone()));
    let adapter_capability_ref = payload
        .adapter_capability_ref
        .as_ref()
        .map(|value| values::NetworkRuntimeStreamRef(value.clone()));
    values::json_value(NetworkEnforcementCommandIssuedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        enforcement_command_ref: event_ref.0.clone(),
        previous_event_ref: values::previous_event_ref(payload).0,
        policy_decision_ref: values::ref_or_current(&policy_decision_ref, event_ref).0,
        adapter_capability_ref: values::ref_or_current(&adapter_capability_ref, event_ref).0,
        enforcement_mode: values::enforcement_mode(payload),
        evidence_refs: values::evidence_refs(payload)
            .into_iter()
            .map(|value| value.0)
            .collect(),
        rollback_ref: None,
    })
}

pub(crate) fn network_enforcement_result_observed(
    event_ref: &values::NetworkRuntimeStreamRef,
    payload: &NetworkRuntimeEventPayload,
) -> Value {
    let enforcement_command_ref = payload
        .enforcement_command_ref
        .as_ref()
        .map(|value| values::NetworkRuntimeStreamRef(value.clone()));
    values::json_value(NetworkEnforcementResultObservedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        enforcement_result_ref: event_ref.0.clone(),
        enforcement_command_ref: values::ref_or_current(&enforcement_command_ref, event_ref).0,
        previous_event_ref: values::previous_event_ref(payload).0,
        result_status: values::enforcement_result_status(payload),
        adapter_action_executed: false,
        rollback_ref: None,
        unavailable_reason_code: values::unavailable_reason_code(payload).map(|value| value.0),
    })
}

pub(crate) fn network_audit_entry_committed(
    event_ref: &values::NetworkRuntimeStreamRef,
    payload: &NetworkRuntimeEventPayload,
) -> Value {
    values::json_value(NetworkAuditEntryCommittedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        audit_entry_ref: event_ref.0.clone(),
        previous_event_ref: values::previous_event_ref(payload).0,
        policy_decision_ref: payload
            .policy_decision_ref
            .clone()
            .unwrap_or_else(|| policy_constants::HANDOFF_NOT_REQUESTED.to_string()),
        enforcement_command_ref: payload.enforcement_command_ref.clone(),
        enforcement_result_ref: payload.enforcement_result_ref.clone(),
        evidence_refs: values::evidence_refs(payload)
            .into_iter()
            .map(|value| value.0)
            .collect(),
        audit_outcome: NetworkAuditOutcome::Committed,
    })
}

pub(crate) fn network_portal_read_model_updated(
    event_ref: &values::NetworkRuntimeStreamRef,
    payload: &NetworkRuntimeEventPayload,
) -> Value {
    let audit_entry_ref = payload
        .audit_entry_ref
        .as_ref()
        .map(|value| values::NetworkRuntimeStreamRef(value.clone()));
    values::json_value(NetworkPortalReadModelUpdatedEvent {
        schema_version: constants::network_flow::EVENT_SCHEMA_VERSION,
        read_model_ref: event_ref.0.clone(),
        previous_event_ref: values::previous_event_ref(payload).0,
        audit_entry_ref: values::ref_or_current(&audit_entry_ref, event_ref).0,
        update_kind: values::portal_update_kind(payload),
        visible_manual_required: payload.intervention_state
            == NetworkInterventionState::ManualRequired,
        visible_unavailable: payload.intervention_state == NetworkInterventionState::Unavailable,
    })
}
