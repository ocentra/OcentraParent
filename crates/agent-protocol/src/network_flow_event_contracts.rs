use ocentra_eventing::error::EventingError;

use super::{
    validate_network_event, validate_network_optional_text, validate_network_texts,
    NetworkActivityClassifiedEvent, NetworkAiAnalysisCompletedEvent,
    NetworkAiAnalysisRequestedEvent, NetworkDomainObservedEvent,
    NetworkEnforcementCommandIssuedEvent, NetworkFlowObservedEvent,
    NetworkPolicyDecisionCompletedEvent, NetworkPolicyEvaluationRequestedEvent,
    NetworkRuntimeEventContract, NetworkRuntimeEventPayload,
};

impl NetworkRuntimeEventContract for NetworkFlowObservedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_NETWORK_FLOW_OBSERVED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_network_event(
            self.schema_version,
            &[
                (&self.flow_event_ref, "flow_event_ref"),
                (&self.observed_at, "observed_at"),
                (&self.device_ref, "device_ref"),
                (&self.flow_evidence_ref, "flow_evidence_ref"),
                (&self.custody, "custody"),
            ],
        )
    }
}

impl NetworkRuntimeEventContract for NetworkDomainObservedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_NETWORK_DOMAIN_OBSERVED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_network_event(
            self.schema_version,
            &[
                (&self.domain_event_ref, "domain_event_ref"),
                (&self.previous_event_ref, "previous_event_ref"),
                (&self.flow_evidence_ref, "flow_evidence_ref"),
                (&self.domain_evidence_ref, "domain_evidence_ref"),
            ],
        )
        .and_then(|_| validate_network_texts(&self.uncertainty_codes, "uncertainty_codes", false))
    }
}

impl NetworkRuntimeEventContract for NetworkActivityClassifiedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_NETWORK_ACTIVITY_CLASSIFIED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_network_event(
            self.schema_version,
            &[
                (&self.classification_event_ref, "classification_event_ref"),
                (&self.previous_event_ref, "previous_event_ref"),
            ],
        )
        .and_then(|_| validate_network_texts(&self.evidence_refs, "evidence_refs", true))
        .and_then(|_| validate_network_texts(&self.uncertainty_codes, "uncertainty_codes", false))
        .and_then(|_| {
            (self.confidence.is_finite() && (0.0..=1.0).contains(&self.confidence))
                .then_some(())
                .ok_or_else(|| EventingError::InvalidValue {
                    field: "confidence",
                    value: self.confidence.to_string(),
                })
        })
    }
}

impl NetworkRuntimeEventContract for NetworkAiAnalysisRequestedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_AI_ANALYSIS_REQUESTED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_network_event(
            self.schema_version,
            &[
                (&self.ai_request_ref, "ai_request_ref"),
                (&self.previous_event_ref, "previous_event_ref"),
                (&self.prompt_template_ref, "prompt_template_ref"),
                (&self.custody, "custody"),
            ],
        )
        .and_then(|_| validate_network_texts(&self.evidence_refs, "evidence_refs", true))
        .and_then(|_| {
            (!self.raw_packet_payload_included)
                .then_some(())
                .ok_or_else(|| EventingError::InvalidValue {
                    field: "raw_packet_payload_included",
                    value: "true".to_string(),
                })
        })
    }
}

impl NetworkRuntimeEventContract for NetworkAiAnalysisCompletedEvent {
    const EVENT_TYPE: &'static str = crate::constants::network_flow::EVENT_AI_ANALYSIS_COMPLETED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_network_event(
            self.schema_version,
            &[
                (&self.ai_analysis_ref, "ai_analysis_ref"),
                (&self.ai_request_ref, "ai_request_ref"),
                (&self.previous_event_ref, "previous_event_ref"),
            ],
        )
        .and_then(|_| validate_network_texts(&self.evidence_refs, "evidence_refs", true))
        .and_then(|_| validate_network_texts(&self.unsupported_claims, "unsupported_claims", false))
    }
}

impl NetworkRuntimeEventContract for NetworkPolicyEvaluationRequestedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_POLICY_EVALUATION_REQUESTED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_network_event(
            self.schema_version,
            &[
                (&self.policy_evaluation_ref, "policy_evaluation_ref"),
                (&self.previous_event_ref, "previous_event_ref"),
            ],
        )
        .and_then(|_| validate_network_texts(&self.evidence_refs, "evidence_refs", true))
        .and_then(|_| validate_network_texts(&self.parent_rule_refs, "parent_rule_refs", true))
        .and_then(|_| {
            validate_network_optional_text(self.ai_analysis_ref.as_deref(), "ai_analysis_ref")
        })
    }
}

impl NetworkRuntimeEventContract for NetworkPolicyDecisionCompletedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_POLICY_DECISION_COMPLETED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_network_event(
            self.schema_version,
            &[
                (&self.policy_decision_ref, "policy_decision_ref"),
                (&self.policy_evaluation_ref, "policy_evaluation_ref"),
                (&self.previous_event_ref, "previous_event_ref"),
            ],
        )
        .and_then(|_| validate_network_texts(&self.evidence_refs, "evidence_refs", true))
        .and_then(|_| validate_network_texts(&self.parent_rule_refs, "parent_rule_refs", true))
    }
}

impl NetworkRuntimeEventContract for NetworkEnforcementCommandIssuedEvent {
    const EVENT_TYPE: &'static str =
        crate::constants::network_flow::EVENT_ENFORCEMENT_COMMAND_ISSUED;

    fn validate(&self) -> Result<(), EventingError> {
        validate_network_event(
            self.schema_version,
            &[
                (&self.enforcement_command_ref, "enforcement_command_ref"),
                (&self.previous_event_ref, "previous_event_ref"),
                (&self.policy_decision_ref, "policy_decision_ref"),
                (&self.adapter_capability_ref, "adapter_capability_ref"),
            ],
        )
        .and_then(|_| validate_network_texts(&self.evidence_refs, "evidence_refs", true))
        .and_then(|_| validate_network_optional_text(self.rollback_ref.as_deref(), "rollback_ref"))
    }
}

pub(super) fn runtime_aggregate_key(payload: &NetworkRuntimeEventPayload) -> String {
    let mut value = String::from(crate::constants::network_flow::AGGREGATE_NETWORK_FLOW_PREFIX);
    if let Some(domain) = payload.destination_domain.as_deref() {
        value.push_str(domain);
        return value;
    }
    if let Some(ip) = payload.destination_ip.as_deref() {
        value.push_str(ip);
        if let Some(port) = payload.destination_port {
            value.push(crate::constants::delimiter::HYPHEN);
            value.push_str(&port.to_string());
        }
        return value;
    }
    value.push_str(payload.capability_status.as_protocol_str());
    value
}
