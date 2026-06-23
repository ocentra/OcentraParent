use ocentra_parent_agent_protocol::activity::ACTIVITY_SCHEMA_VERSION;
use ocentra_parent_agent_protocol::activity::{
    ActivityEvent, ActivityEventKind, ActivityObserver, ActivitySource, ActivitySubject,
    ActivitySubjectKind,
};
use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserCustodyLabel, BrowserFamily};
use ocentra_parent_agent_protocol::browser_intervention_values::{
    BrowserBoundaryState, BrowserExactUrlClaimState, BrowserInterventionAction,
    BrowserInterventionCapabilityState, BrowserInterventionDecisionSource,
    BrowserInterventionDeliveryState, BrowserInterventionMechanism, BrowserInterventionOutcome,
    BrowserInterventionTargetType, BrowserUnmanagedDetectionState,
    BrowserUnmanagedFallbackActionState,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::browser_unmanaged_enforcement::BrowserUnmanagedEnforcementState;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

mod ids;
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserInterventionObservation {
    pub browser_family: Option<BrowserFamily>,
    pub browser_channel: Option<BrowserChannel>,
    pub managed_browser_session_id: Option<String>,
    pub profile_id: Option<String>,
    pub process_id: Option<u32>,
    pub intervention_action_id: Option<String>,
    pub intervention_audit_id: Option<String>,
    pub evidence_reference_ids: Vec<String>,
    pub policy_decision_id: Option<String>,
    pub decision_source: BrowserInterventionDecisionSource,
    pub intervention_action: BrowserInterventionAction,
    pub intervention_target_type: BrowserInterventionTargetType,
    pub intervention_target_value: String,
    pub requested_url: Option<String>,
    pub observed_url: Option<String>,
    pub intervention_mechanism: BrowserInterventionMechanism,
    pub intervention_outcome: BrowserInterventionOutcome,
    pub browser_boundary_state: BrowserBoundaryState,
    pub exact_url_claim_state: BrowserExactUrlClaimState,
    pub unmanaged_detection_state: BrowserUnmanagedDetectionState,
    pub unmanaged_fallback_action: BrowserUnmanagedFallbackActionState,
    pub child_delivery_state: BrowserInterventionDeliveryState,
    pub managed_session_intervention_capability: BrowserInterventionCapabilityState,
    pub unmanaged_browser_enforcement: BrowserUnmanagedEnforcementState,
    pub reason: Option<String>,
    pub custody_label: BrowserCustodyLabel,
    pub query_visibility: BrowserQueryVisibilityLabel,
}

pub fn browser_intervention_applied_event(
    observation: BrowserInterventionObservation,
    observed_at: &str,
    sequence_index: usize,
) -> ActivityEvent {
    let intervention_id = ids::browser_intervention_id(sequence_index);
    let mut fields = browser_intervention_fields(&observation, &intervention_id);
    insert_optional_text(
        &mut fields,
        constants::field::BROWSER_INTERVENTION_ACTION_ID,
        &observation.intervention_action_id,
    );
    insert_optional_text(
        &mut fields,
        constants::field::BROWSER_INTERVENTION_AUDIT_ID,
        &observation.intervention_audit_id,
    );
    insert_optional_text_list(
        &mut fields,
        constants::field::EVIDENCE_REFERENCE_IDS,
        &observation.evidence_reference_ids,
    );
    insert_optional_text(
        &mut fields,
        constants::field::POLICY_DECISION_ID,
        &observation.policy_decision_id,
    );
    insert_optional_text(
        &mut fields,
        constants::field::MANAGED_BROWSER_SESSION_ID,
        &observation.managed_browser_session_id,
    );
    insert_optional_text(
        &mut fields,
        constants::field::PROFILE_ID,
        &observation.profile_id,
    );
    insert_optional_text(
        &mut fields,
        constants::field::REQUESTED_URL,
        &observation.requested_url,
    );
    insert_optional_text(
        &mut fields,
        constants::field::OBSERVED_URL,
        &observation.observed_url,
    );
    insert_optional_text(&mut fields, constants::field::REASON, &observation.reason);
    if let Some(process_id) = observation.process_id {
        fields.insert(
            constants::field::PROCESS_ID.to_string(),
            LogFieldValue::Number(f64::from(process_id)),
        );
    }

    ActivityEvent {
        schema_version: ACTIVITY_SCHEMA_VERSION,
        event_id: ids::browser_intervention_event_id(sequence_index),
        observed_at: observed_at.to_string(),
        source: ActivitySource {
            device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
            platform: std::env::consts::OS.to_string(),
            observer: ActivityObserver::ManagedBrowserBridge,
            source_id: constants::browser::INTERVENTION_SOURCE_ID_MANAGED_BROWSER.to_string(),
        },
        kind: ActivityEventKind::BrowserInterventionApplied,
        subject: ActivitySubject {
            kind: ActivitySubjectKind::Intervention,
            subject_id: ids::browser_intervention_subject_id(sequence_index),
            display_name: Some(observation.intervention_target_value),
        },
        fields,
        evidence: Vec::new(),
    }
}

fn browser_intervention_fields(
    observation: &BrowserInterventionObservation,
    intervention_id: &str,
) -> LogFields {
    let mut fields = LogFields::new();
    insert_intervention_identity_fields(&mut fields, observation, intervention_id);
    insert_intervention_decision_fields(&mut fields, observation);
    insert_intervention_state_fields(&mut fields, observation);
    fields
}

fn insert_intervention_identity_fields(
    fields: &mut LogFields,
    observation: &BrowserInterventionObservation,
    intervention_id: &str,
) {
    insert_protocol_text(
        fields,
        constants::field::BROWSER_INTERVENTION_ID.to_string(),
        intervention_id,
    );
    insert_protocol_text(
        fields,
        constants::field::SOURCE_ID,
        constants::browser::INTERVENTION_SOURCE_ID_MANAGED_BROWSER,
    );
    insert_optional_protocol(
        fields,
        constants::field::BROWSER_FAMILY,
        observation
            .browser_family
            .as_ref()
            .map(|family| family.as_protocol_str()),
    );
    insert_optional_protocol(
        fields,
        constants::field::BROWSER_CHANNEL,
        observation
            .browser_channel
            .as_ref()
            .map(|channel| channel.as_protocol_str()),
    );
}

fn insert_intervention_decision_fields(
    fields: &mut LogFields,
    observation: &BrowserInterventionObservation,
) {
    insert_protocol_text(
        fields,
        constants::field::DECISION_SOURCE.to_string(),
        observation.decision_source.as_protocol_str(),
    );
    insert_protocol_text(
        fields,
        constants::field::INTERVENTION_ACTION.to_string(),
        observation.intervention_action.as_protocol_str(),
    );
    insert_protocol_text(
        fields,
        constants::field::INTERVENTION_TARGET_TYPE.to_string(),
        observation.intervention_target_type.as_protocol_str(),
    );
    fields.insert(
        constants::field::INTERVENTION_TARGET_VALUE.to_string(),
        LogFieldValue::String(observation.intervention_target_value.clone()),
    );
}

fn insert_intervention_state_fields(
    fields: &mut LogFields,
    observation: &BrowserInterventionObservation,
) {
    insert_protocol_text(
        fields,
        constants::field::INTERVENTION_MECHANISM.to_string(),
        observation.intervention_mechanism.as_protocol_str(),
    );
    insert_protocol_text(
        fields,
        constants::field::INTERVENTION_OUTCOME.to_string(),
        observation.intervention_outcome.as_protocol_str(),
    );
    insert_protocol_text(
        fields,
        constants::browser::INTERVENTION_FIELD_BROWSER_BOUNDARY_STATE,
        observation.browser_boundary_state.as_protocol_str(),
    );
    insert_protocol_text(
        fields,
        constants::browser::INTERVENTION_FIELD_EXACT_URL_CLAIM_STATE,
        observation.exact_url_claim_state.as_protocol_str(),
    );
    insert_protocol_text(
        fields,
        constants::browser::INTERVENTION_FIELD_UNMANAGED_DETECTION_STATE,
        observation.unmanaged_detection_state.as_protocol_str(),
    );
    insert_protocol_text(
        fields,
        constants::field::UNMANAGED_FALLBACK_ACTION.to_string(),
        observation.unmanaged_fallback_action.as_protocol_str(),
    );
    insert_protocol_text(
        fields,
        constants::field::CHILD_DELIVERY_STATE.to_string(),
        observation.child_delivery_state.as_protocol_str(),
    );
    insert_protocol_text(
        fields,
        constants::field::MANAGED_SESSION_INTERVENTION_CAPABILITY.to_string(),
        observation
            .managed_session_intervention_capability
            .as_protocol_str(),
    );
    insert_protocol_text(
        fields,
        constants::field::UNMANAGED_BROWSER_ENFORCEMENT.to_string(),
        observation.unmanaged_browser_enforcement.as_protocol_str(),
    );
    insert_protocol_text(
        fields,
        constants::field::CUSTODY_LABEL.to_string(),
        observation.custody_label.as_protocol_str(),
    );
    insert_protocol_text(
        fields,
        constants::field::QUERY_VISIBILITY.to_string(),
        observation.query_visibility.as_protocol_str(),
    );
}

fn insert_protocol_text(fields: &mut LogFields, key: impl Into<String>, value: &str) {
    fields.insert(key.into(), LogFieldValue::String(value.to_string()));
}

fn insert_optional_text(fields: &mut LogFields, key: &str, value: &Option<String>) {
    if let Some(text) = value {
        fields.insert(key.to_string(), LogFieldValue::String(text.clone()));
    }
}

fn insert_optional_text_list(fields: &mut LogFields, key: &str, values: &[String]) {
    if values.is_empty() {
        return;
    }
    fields.insert(
        key.to_string(),
        LogFieldValue::String(values.join(&constants::delimiter::LIST.to_string())),
    );
}

fn insert_optional_protocol(fields: &mut LogFields, key: &str, value: Option<&str>) {
    if let Some(text) = value {
        fields.insert(key.to_string(), LogFieldValue::String(text.to_string()));
    }
}
