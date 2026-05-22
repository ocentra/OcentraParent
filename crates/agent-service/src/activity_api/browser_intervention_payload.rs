use ocentra_parent_agent_protocol::{
    constants, BrowserInterventionReadModel, BrowserInterventionRow, LogFieldValue, LogFields,
};

use crate::fields::fields_from_pairs;

type FieldPair = (&'static str, LogFieldValue);

pub(super) fn browser_intervention_read_model_payload(
    read_model: &BrowserInterventionReadModel,
) -> LogFields {
    let latest = read_model.rows.first();
    let mut pairs = browser_intervention_read_model_pairs(read_model);
    pairs.extend(browser_intervention_identity_pairs(latest));
    pairs.extend(browser_intervention_decision_pairs(latest));
    pairs.extend(browser_intervention_target_pairs(latest));
    pairs.extend(browser_intervention_state_pairs(latest));
    fields_from_pairs(pairs)
}

fn browser_intervention_read_model_pairs(
    read_model: &BrowserInterventionReadModel,
) -> Vec<FieldPair> {
    vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::LIMIT,
            LogFieldValue::Number(read_model.limit as f64),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        (
            constants::field::LATEST_EVENT_ID,
            optional_string(read_model.latest_event_id.as_ref()),
        ),
        (
            constants::field::LATEST_OBSERVED_AT,
            optional_string(read_model.latest_observed_at.as_ref()),
        ),
        (
            constants::field::MANAGED_SESSION_INTERVENTION_CAPABILITY,
            LogFieldValue::String(
                read_model
                    .managed_session_intervention_capability
                    .as_protocol_str()
                    .to_string(),
            ),
        ),
        (
            constants::field::UNMANAGED_BROWSER_ENFORCEMENT,
            LogFieldValue::String(
                read_model
                    .unmanaged_browser_enforcement
                    .as_protocol_str()
                    .to_string(),
            ),
        ),
    ]
}

fn browser_intervention_identity_pairs(row: Option<&BrowserInterventionRow>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::BROWSER_INTERVENTION_ID,
            optional_string(row.map(|value| &value.browser_intervention_id)),
        ),
        (
            constants::field::SOURCE_ID,
            optional_string(row.map(|value| &value.source_id)),
        ),
        (
            constants::field::BROWSER_FAMILY,
            optional_enum(
                row.and_then(|value| value.browser_family.as_ref())
                    .map(|family| family.as_protocol_str()),
            ),
        ),
        (
            constants::field::BROWSER_CHANNEL,
            optional_enum(
                row.and_then(|value| value.browser_channel.as_ref())
                    .map(|channel| channel.as_protocol_str()),
            ),
        ),
        (
            constants::field::MANAGED_BROWSER_SESSION_ID,
            optional_string(row.and_then(|value| value.managed_browser_session_id.as_ref())),
        ),
        (
            constants::field::PROFILE_ID,
            optional_string(row.and_then(|value| value.profile_id.as_ref())),
        ),
        (
            constants::field::PROCESS_ID,
            optional_u32(row.and_then(|value| value.process_id)),
        ),
    ]
}

fn browser_intervention_decision_pairs(row: Option<&BrowserInterventionRow>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::POLICY_DECISION_ID,
            optional_string(row.and_then(|value| value.policy_decision_id.as_ref())),
        ),
        (
            constants::field::DECISION_SOURCE,
            optional_enum(row.map(|value| value.decision_source.as_protocol_str())),
        ),
        (
            constants::field::INTERVENTION_ACTION,
            optional_enum(row.map(|value| value.intervention_action.as_protocol_str())),
        ),
    ]
}

fn browser_intervention_target_pairs(row: Option<&BrowserInterventionRow>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::INTERVENTION_TARGET_TYPE,
            optional_enum(row.map(|value| value.intervention_target_type.as_protocol_str())),
        ),
        (
            constants::field::INTERVENTION_TARGET_VALUE,
            optional_string(row.map(|value| &value.intervention_target_value)),
        ),
        (
            constants::field::REQUESTED_URL,
            optional_string(row.and_then(|value| value.requested_url.as_ref())),
        ),
        (
            constants::field::OBSERVED_URL,
            optional_string(row.and_then(|value| value.observed_url.as_ref())),
        ),
    ]
}

fn browser_intervention_state_pairs(row: Option<&BrowserInterventionRow>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::INTERVENTION_MECHANISM,
            optional_enum(row.map(|value| value.intervention_mechanism.as_protocol_str())),
        ),
        (
            constants::field::INTERVENTION_OUTCOME,
            optional_enum(row.map(|value| value.intervention_outcome.as_protocol_str())),
        ),
        (
            constants::field::REASON,
            optional_string(row.and_then(|value| value.reason.as_ref())),
        ),
        (
            constants::field::CUSTODY_LABEL,
            optional_enum(row.map(|value| value.custody_label.as_protocol_str())),
        ),
        (
            constants::field::QUERY_VISIBILITY,
            optional_enum(row.map(|value| value.query_visibility.as_protocol_str())),
        ),
    ]
}

fn optional_string(value: Option<&String>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.clone()),
        None => LogFieldValue::Null(()),
    }
}

fn optional_enum(value: Option<&str>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.to_string()),
        None => LogFieldValue::Null(()),
    }
}

fn optional_u32(value: Option<u32>) -> LogFieldValue {
    match value {
        Some(number) => LogFieldValue::Number(number as f64),
        None => LogFieldValue::Null(()),
    }
}
