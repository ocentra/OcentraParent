use ocentra_parent_agent_protocol::{
    constants, BrowserChannel, BrowserCustodyLabel, BrowserFamily, BrowserInterventionAction,
    BrowserInterventionCapabilityState, BrowserInterventionDecisionSource,
    BrowserInterventionMechanism, BrowserInterventionOutcome, BrowserInterventionTargetType,
    BrowserQueryVisibilityLabel, BrowserUnmanagedEnforcementState, LogFieldValue, LogFields,
};

pub(super) fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    match fields.get(key) {
        Some(LogFieldValue::String(value)) => Some(value.clone()),
        _ => None,
    }
}

pub(super) fn u32_field(fields: &LogFields, key: &str) -> Option<u32> {
    match fields.get(key) {
        Some(LogFieldValue::Number(value)) if value.is_finite() && *value >= 0.0 => {
            u32::try_from(*value as u64).ok()
        }
        _ => None,
    }
}

pub(super) fn browser_family_field(fields: &LogFields) -> Option<BrowserFamily> {
    string_field(fields, constants::field::BROWSER_FAMILY)
        .and_then(|value| BrowserFamily::from_protocol_str(&value))
}

pub(super) fn browser_channel_field(fields: &LogFields) -> Option<BrowserChannel> {
    string_field(fields, constants::field::BROWSER_CHANNEL)
        .and_then(|value| BrowserChannel::from_protocol_str(&value))
}

pub(super) fn decision_source_field(
    fields: &LogFields,
) -> Option<BrowserInterventionDecisionSource> {
    string_field(fields, constants::field::DECISION_SOURCE)
        .and_then(|value| BrowserInterventionDecisionSource::from_protocol_str(&value))
}

pub(super) fn intervention_action_field(fields: &LogFields) -> Option<BrowserInterventionAction> {
    string_field(fields, constants::field::INTERVENTION_ACTION)
        .and_then(|value| BrowserInterventionAction::from_protocol_str(&value))
}

pub(super) fn intervention_target_type_field(
    fields: &LogFields,
) -> Option<BrowserInterventionTargetType> {
    string_field(fields, constants::field::INTERVENTION_TARGET_TYPE)
        .and_then(|value| BrowserInterventionTargetType::from_protocol_str(&value))
}

pub(super) fn intervention_mechanism_field(
    fields: &LogFields,
) -> Option<BrowserInterventionMechanism> {
    string_field(fields, constants::field::INTERVENTION_MECHANISM)
        .and_then(|value| BrowserInterventionMechanism::from_protocol_str(&value))
}

pub(super) fn intervention_outcome_field(fields: &LogFields) -> Option<BrowserInterventionOutcome> {
    string_field(fields, constants::field::INTERVENTION_OUTCOME)
        .and_then(|value| BrowserInterventionOutcome::from_protocol_str(&value))
}

pub(super) fn intervention_capability_field(
    fields: &LogFields,
) -> Option<BrowserInterventionCapabilityState> {
    string_field(
        fields,
        constants::field::MANAGED_SESSION_INTERVENTION_CAPABILITY,
    )
    .and_then(|value| BrowserInterventionCapabilityState::from_protocol_str(&value))
}

pub(super) fn unmanaged_enforcement_field(
    fields: &LogFields,
) -> Option<BrowserUnmanagedEnforcementState> {
    string_field(fields, constants::field::UNMANAGED_BROWSER_ENFORCEMENT)
        .and_then(|value| BrowserUnmanagedEnforcementState::from_protocol_str(&value))
}

pub(super) fn custody_label_field(fields: &LogFields) -> Option<BrowserCustodyLabel> {
    string_field(fields, constants::field::CUSTODY_LABEL)
        .and_then(|value| BrowserCustodyLabel::from_protocol_str(&value))
}

pub(super) fn query_visibility_field(fields: &LogFields) -> Option<BrowserQueryVisibilityLabel> {
    string_field(fields, constants::field::QUERY_VISIBILITY)
        .and_then(|value| BrowserQueryVisibilityLabel::from_protocol_str(&value))
}
