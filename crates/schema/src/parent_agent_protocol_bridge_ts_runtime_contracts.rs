struct RuntimeEventContractNames {
    browser_event_type_const: String,
    browser_phase_const: String,
    browser_capability_status_const: String,
    browser_custody_label_const: String,
    browser_query_visibility_const: String,
    network_event_type_const: String,
    network_evidence_grade_const: String,
    network_domain_attribution_kind_const: String,
    network_activity_kind_const: String,
    network_ai_advisory_state_const: String,
    network_policy_decision_action_const: String,
    network_enforcement_mode_const: String,
    network_enforcement_result_status_const: String,
    network_audit_outcome_const: String,
    network_portal_update_kind_const: String,
}

impl RuntimeEventContractNames {
    fn new(prefix: &str) -> Self {
        Self {
            browser_event_type_const: format!("{prefix}BrowserRuntimeEventType"),
            browser_phase_const: format!("{prefix}BrowserRuntimePhase"),
            browser_capability_status_const: format!("{prefix}BrowserRuntimeCapabilityStatus"),
            browser_custody_label_const: format!("{prefix}BrowserRuntimeCustodyLabel"),
            browser_query_visibility_const: format!("{prefix}BrowserRuntimeQueryVisibility"),
            network_event_type_const: format!("{prefix}NetworkRuntimeEventType"),
            network_evidence_grade_const: format!("{prefix}NetworkEvidenceGrade"),
            network_domain_attribution_kind_const: format!("{prefix}NetworkDomainAttributionKind"),
            network_activity_kind_const: format!("{prefix}NetworkRuntimeActivityKind"),
            network_ai_advisory_state_const: format!("{prefix}NetworkAiAdvisoryState"),
            network_policy_decision_action_const: format!("{prefix}NetworkPolicyDecisionAction"),
            network_enforcement_mode_const: format!("{prefix}NetworkEnforcementMode"),
            network_enforcement_result_status_const: format!(
                "{prefix}NetworkEnforcementResultStatus"
            ),
            network_audit_outcome_const: format!("{prefix}NetworkAuditOutcome"),
            network_portal_update_kind_const: format!("{prefix}NetworkPortalUpdateKind"),
        }
    }
}

fn runtime_event_contract_typescript(names: &ProtocolBridgeNames) -> String {
    let prefix = bridge_prefix(names);
    let contract_names = RuntimeEventContractNames::new(prefix);
    let mut sections = browser_runtime_event_contract_sections(names, prefix, &contract_names);
    sections.extend(network_runtime_event_contract_sections(
        names,
        prefix,
        &contract_names,
    ));
    sections.join(" ")
}

fn browser_runtime_event_contract_sections(
    names: &ProtocolBridgeNames,
    prefix: &str,
    contract_names: &RuntimeEventContractNames,
) -> Vec<String> {
    vec![
        literal_typescript(
            &contract_names.browser_event_type_const,
            &contract_names.browser_event_type_const,
            &browser_runtime_event_type_descriptors(),
        ),
        literal_typescript(
            &contract_names.browser_phase_const,
            &contract_names.browser_phase_const,
            &browser_runtime_phase_descriptors(),
        ),
        literal_typescript(
            &contract_names.browser_capability_status_const,
            &contract_names.browser_capability_status_const,
            &browser_capability_status_descriptors(),
        ),
        literal_typescript(
            &contract_names.browser_custody_label_const,
            &contract_names.browser_custody_label_const,
            &browser_custody_label_descriptors(),
        ),
        literal_typescript(
            &contract_names.browser_query_visibility_const,
            &contract_names.browser_query_visibility_const,
            &browser_query_visibility_descriptors(),
        ),
        browser_runtime_contract_decoders_typescript(names, prefix, contract_names),
    ]
}

fn network_runtime_event_contract_sections(
    names: &ProtocolBridgeNames,
    prefix: &str,
    contract_names: &RuntimeEventContractNames,
) -> Vec<String> {
    vec![
        literal_typescript(
            &contract_names.network_event_type_const,
            &contract_names.network_event_type_const,
            &network_runtime_event_type_descriptors(),
        ),
        literal_typescript(
            &contract_names.network_evidence_grade_const,
            &contract_names.network_evidence_grade_const,
            &network_evidence_grade_descriptors(),
        ),
        literal_typescript(
            &contract_names.network_domain_attribution_kind_const,
            &contract_names.network_domain_attribution_kind_const,
            &network_domain_attribution_kind_descriptors(),
        ),
        literal_typescript(
            &contract_names.network_activity_kind_const,
            &contract_names.network_activity_kind_const,
            &network_activity_kind_descriptors(),
        ),
        literal_typescript(
            &contract_names.network_ai_advisory_state_const,
            &contract_names.network_ai_advisory_state_const,
            &network_ai_advisory_state_descriptors(),
        ),
        literal_typescript(
            &contract_names.network_policy_decision_action_const,
            &contract_names.network_policy_decision_action_const,
            &network_policy_decision_action_descriptors(),
        ),
        literal_typescript(
            &contract_names.network_enforcement_mode_const,
            &contract_names.network_enforcement_mode_const,
            &network_enforcement_mode_descriptors(),
        ),
        literal_typescript(
            &contract_names.network_enforcement_result_status_const,
            &contract_names.network_enforcement_result_status_const,
            &network_enforcement_result_status_descriptors(),
        ),
        literal_typescript(
            &contract_names.network_audit_outcome_const,
            &contract_names.network_audit_outcome_const,
            &network_audit_outcome_descriptors(),
        ),
        literal_typescript(
            &contract_names.network_portal_update_kind_const,
            &contract_names.network_portal_update_kind_const,
            &network_portal_update_kind_descriptors(),
        ),
        network_runtime_contract_decoders_typescript(names, prefix, contract_names),
    ]
}

struct NetworkRuntimeDecoderNames {
    claim_boundary_type: String,
    flow_observed_type: String,
    domain_observed_type: String,
    activity_classified_type: String,
    ai_analysis_requested_type: String,
    ai_analysis_completed_type: String,
    policy_evaluation_requested_type: String,
    policy_decision_completed_type: String,
    enforcement_command_issued_type: String,
    enforcement_result_observed_type: String,
    audit_entry_committed_type: String,
    portal_read_model_updated_type: String,
    runtime_event_payload_type: String,
    payload_decoder_fn: String,
    event_type_schema_const: String,
    helper_prefix: String,
}

impl NetworkRuntimeDecoderNames {
    fn new(prefix: &str) -> Self {
        Self {
            claim_boundary_type: format!("{prefix}NetworkClaimBoundary"),
            flow_observed_type: format!("{prefix}NetworkFlowObservedEvent"),
            domain_observed_type: format!("{prefix}NetworkDomainObservedEvent"),
            activity_classified_type: format!("{prefix}NetworkActivityClassifiedEvent"),
            ai_analysis_requested_type: format!("{prefix}NetworkAiAnalysisRequestedEvent"),
            ai_analysis_completed_type: format!("{prefix}NetworkAiAnalysisCompletedEvent"),
            policy_evaluation_requested_type: format!(
                "{prefix}NetworkPolicyEvaluationRequestedEvent"
            ),
            policy_decision_completed_type: format!("{prefix}NetworkPolicyDecisionCompletedEvent"),
            enforcement_command_issued_type: format!(
                "{prefix}NetworkEnforcementCommandIssuedEvent"
            ),
            enforcement_result_observed_type: format!(
                "{prefix}NetworkEnforcementResultObservedEvent"
            ),
            audit_entry_committed_type: format!("{prefix}NetworkAuditEntryCommittedEvent"),
            portal_read_model_updated_type: format!("{prefix}NetworkPortalReadModelUpdatedEvent"),
            runtime_event_payload_type: format!("{prefix}NetworkRuntimeEventPayload"),
            payload_decoder_fn: format!("decode{prefix}NetworkRuntimeEventPayload"),
            event_type_schema_const: format!("{prefix}NetworkRuntimeEventTypeSchema"),
            helper_prefix: format!("__{prefix}NetworkRuntime"),
        }
    }
}

fn network_runtime_contract_decoders_typescript(
    names: &ProtocolBridgeNames,
    prefix: &str,
    contract_names: &RuntimeEventContractNames,
) -> String {
    let decoder_names = NetworkRuntimeDecoderNames::new(prefix);
    let tokens = network_runtime_decoder_tokens(names, contract_names, &decoder_names);
    replace_tokens(
        parent_agent_protocol_bridge_ts_runtime_02_template(),
        &tokens,
    )
}

fn network_runtime_decoder_tokens<'a>(
    names: &'a ProtocolBridgeNames,
    contract_names: &'a RuntimeEventContractNames,
    decoder_names: &'a NetworkRuntimeDecoderNames,
) -> Vec<(&'static str, &'a str)> {
    let mut tokens = network_runtime_decoder_const_tokens(names, contract_names);
    tokens.extend(network_runtime_decoder_type_tokens(decoder_names));
    tokens
}

fn network_runtime_decoder_const_tokens<'a>(
    names: &'a ProtocolBridgeNames,
    contract_names: &'a RuntimeEventContractNames,
) -> Vec<(&'static str, &'a str)> {
    let mut tokens = network_runtime_decoder_identity_tokens(names, contract_names);
    tokens.extend(network_runtime_decoder_state_tokens(contract_names));
    tokens
}

fn network_runtime_decoder_identity_tokens<'a>(
    names: &'a ProtocolBridgeNames,
    contract_names: &'a RuntimeEventContractNames,
) -> Vec<(&'static str, &'a str)> {
    vec![
        ("__RUNTIME_CONST__", names.runtime_const),
        (
            "__NETWORK_EVENT_TYPE_CONST__",
            &contract_names.network_event_type_const,
        ),
        (
            "__NETWORK_EVENT_TYPE_TYPE__",
            &contract_names.network_event_type_const,
        ),
        (
            "__NETWORK_EVIDENCE_GRADE_CONST__",
            &contract_names.network_evidence_grade_const,
        ),
        (
            "__NETWORK_EVIDENCE_GRADE_TYPE__",
            &contract_names.network_evidence_grade_const,
        ),
        (
            "__NETWORK_DOMAIN_ATTRIBUTION_KIND_CONST__",
            &contract_names.network_domain_attribution_kind_const,
        ),
        (
            "__NETWORK_DOMAIN_ATTRIBUTION_KIND_TYPE__",
            &contract_names.network_domain_attribution_kind_const,
        ),
    ]
}

fn network_runtime_decoder_state_tokens(
    contract_names: &RuntimeEventContractNames,
) -> Vec<(&'static str, &str)> {
    vec![
        (
            "__NETWORK_ACTIVITY_KIND_CONST__",
            &contract_names.network_activity_kind_const,
        ),
        (
            "__NETWORK_ACTIVITY_KIND_TYPE__",
            &contract_names.network_activity_kind_const,
        ),
        (
            "__NETWORK_AI_ADVISORY_STATE_CONST__",
            &contract_names.network_ai_advisory_state_const,
        ),
        (
            "__NETWORK_AI_ADVISORY_STATE_TYPE__",
            &contract_names.network_ai_advisory_state_const,
        ),
        (
            "__NETWORK_POLICY_DECISION_ACTION_CONST__",
            &contract_names.network_policy_decision_action_const,
        ),
        (
            "__NETWORK_POLICY_DECISION_ACTION_TYPE__",
            &contract_names.network_policy_decision_action_const,
        ),
        (
            "__NETWORK_ENFORCEMENT_MODE_CONST__",
            &contract_names.network_enforcement_mode_const,
        ),
        (
            "__NETWORK_ENFORCEMENT_MODE_TYPE__",
            &contract_names.network_enforcement_mode_const,
        ),
        (
            "__NETWORK_ENFORCEMENT_RESULT_STATUS_CONST__",
            &contract_names.network_enforcement_result_status_const,
        ),
        (
            "__NETWORK_ENFORCEMENT_RESULT_STATUS_TYPE__",
            &contract_names.network_enforcement_result_status_const,
        ),
        (
            "__NETWORK_AUDIT_OUTCOME_CONST__",
            &contract_names.network_audit_outcome_const,
        ),
        (
            "__NETWORK_AUDIT_OUTCOME_TYPE__",
            &contract_names.network_audit_outcome_const,
        ),
        (
            "__NETWORK_PORTAL_UPDATE_KIND_CONST__",
            &contract_names.network_portal_update_kind_const,
        ),
        (
            "__NETWORK_PORTAL_UPDATE_KIND_TYPE__",
            &contract_names.network_portal_update_kind_const,
        ),
    ]
}

fn network_runtime_decoder_type_tokens(
    decoder_names: &NetworkRuntimeDecoderNames,
) -> Vec<(&'static str, &str)> {
    vec![
        (
            "__NETWORK_CLAIM_BOUNDARY_TYPE__",
            &decoder_names.claim_boundary_type,
        ),
        (
            "__NETWORK_FLOW_OBSERVED_TYPE__",
            &decoder_names.flow_observed_type,
        ),
        (
            "__NETWORK_DOMAIN_OBSERVED_TYPE__",
            &decoder_names.domain_observed_type,
        ),
        (
            "__NETWORK_ACTIVITY_CLASSIFIED_TYPE__",
            &decoder_names.activity_classified_type,
        ),
        (
            "__NETWORK_AI_ANALYSIS_REQUESTED_TYPE__",
            &decoder_names.ai_analysis_requested_type,
        ),
        (
            "__NETWORK_AI_ANALYSIS_COMPLETED_TYPE__",
            &decoder_names.ai_analysis_completed_type,
        ),
        (
            "__NETWORK_POLICY_EVALUATION_REQUESTED_TYPE__",
            &decoder_names.policy_evaluation_requested_type,
        ),
        (
            "__NETWORK_POLICY_DECISION_COMPLETED_TYPE__",
            &decoder_names.policy_decision_completed_type,
        ),
        (
            "__NETWORK_ENFORCEMENT_COMMAND_ISSUED_TYPE__",
            &decoder_names.enforcement_command_issued_type,
        ),
        (
            "__NETWORK_ENFORCEMENT_RESULT_OBSERVED_TYPE__",
            &decoder_names.enforcement_result_observed_type,
        ),
        (
            "__NETWORK_AUDIT_ENTRY_COMMITTED_TYPE__",
            &decoder_names.audit_entry_committed_type,
        ),
        (
            "__NETWORK_PORTAL_READ_MODEL_UPDATED_TYPE__",
            &decoder_names.portal_read_model_updated_type,
        ),
        (
            "__NETWORK_RUNTIME_EVENT_PAYLOAD_TYPE__",
            &decoder_names.runtime_event_payload_type,
        ),
        (
            "__NETWORK_PAYLOAD_DECODER_FN__",
            &decoder_names.payload_decoder_fn,
        ),
        (
            "__NETWORK_EVENT_TYPE_SCHEMA_CONST__",
            &decoder_names.event_type_schema_const,
        ),
        ("__HELPER_PREFIX__", &decoder_names.helper_prefix),
    ]
}
