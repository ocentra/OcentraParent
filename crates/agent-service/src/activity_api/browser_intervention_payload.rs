#[path = "browser_intervention_payload/field_pairs.rs"]
mod field_pairs;

use ocentra_parent_agent_protocol::browser_intervention::{
    BrowserInterventionReadModel, BrowserInterventionRow,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use self::field_pairs::{
    browser_intervention_fields_from_pairs, optional_string_list, optional_text, optional_u32,
    BrowserInterventionFieldPair, BrowserInterventionStringListRef, BrowserInterventionTextRef,
};

pub(super) fn browser_intervention_read_model_payload(
    read_model: &BrowserInterventionReadModel,
) -> LogFields {
    let latest = read_model.rows.first();
    let mut pairs = browser_intervention_read_model_pairs(read_model);
    pairs.extend(browser_intervention_identity_pairs(latest));
    pairs.extend(browser_intervention_decision_pairs(latest));
    pairs.extend(browser_intervention_target_pairs(latest));
    pairs.extend(browser_intervention_state_pairs(latest));
    if let Ok(serialized) = serde_json::to_string(read_model) {
        pairs.push(BrowserInterventionFieldPair(
            constants::field::BROWSER_INTERVENTION_READ_MODEL_JSON,
            LogFieldValue::String(serialized),
        ));
    }
    browser_intervention_fields_from_pairs(pairs)
}

fn browser_intervention_read_model_pairs(
    read_model: &BrowserInterventionReadModel,
) -> Vec<BrowserInterventionFieldPair> {
    vec![
        BrowserInterventionFieldPair(
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        BrowserInterventionFieldPair(
            constants::field::LIMIT,
            LogFieldValue::Number(read_model.limit as f64),
        ),
        BrowserInterventionFieldPair(
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        BrowserInterventionFieldPair(
            constants::field::LATEST_EVENT_ID,
            optional_text(
                read_model
                    .latest_event_id
                    .as_ref()
                    .map(|text| BrowserInterventionTextRef(text.as_str())),
            ),
        ),
        BrowserInterventionFieldPair(
            constants::field::LATEST_OBSERVED_AT,
            optional_text(
                read_model
                    .latest_observed_at
                    .as_ref()
                    .map(|text| BrowserInterventionTextRef(text.as_str())),
            ),
        ),
        BrowserInterventionFieldPair(
            constants::field::MANAGED_SESSION_INTERVENTION_CAPABILITY,
            LogFieldValue::String(
                read_model
                    .managed_session_intervention_capability
                    .as_protocol_str()
                    .to_string(),
            ),
        ),
        BrowserInterventionFieldPair(
            constants::field::UNMANAGED_BROWSER_ENFORCEMENT,
            LogFieldValue::String(
                read_model
                    .unmanaged_browser_enforcement
                    .as_protocol_str()
                    .to_string(),
            ),
        ),
        BrowserInterventionFieldPair(
            constants::field::UNMANAGED_FALLBACK_ACTION,
            LogFieldValue::String(
                read_model
                    .unmanaged_fallback_action
                    .as_protocol_str()
                    .to_string(),
            ),
        ),
    ]
}

fn browser_intervention_identity_pairs(
    row: Option<&BrowserInterventionRow>,
) -> Vec<BrowserInterventionFieldPair> {
    vec![
        BrowserInterventionFieldPair(
            constants::field::BROWSER_INTERVENTION_ID,
            optional_text(
                row.map(|value| BrowserInterventionTextRef(value.browser_intervention_id.as_str())),
            ),
        ),
        BrowserInterventionFieldPair(
            constants::field::SOURCE_ID,
            optional_text(row.map(|value| BrowserInterventionTextRef(value.source_id.as_str()))),
        ),
        BrowserInterventionFieldPair(
            constants::field::BROWSER_FAMILY,
            optional_text(
                row.and_then(|value| value.browser_family.as_ref())
                    .map(|family| BrowserInterventionTextRef(family.as_protocol_str())),
            ),
        ),
        BrowserInterventionFieldPair(
            constants::field::BROWSER_CHANNEL,
            optional_text(
                row.and_then(|value| value.browser_channel.as_ref())
                    .map(|channel| BrowserInterventionTextRef(channel.as_protocol_str())),
            ),
        ),
        BrowserInterventionFieldPair(
            constants::field::MANAGED_BROWSER_SESSION_ID,
            optional_text(row.and_then(|value| {
                value
                    .managed_browser_session_id
                    .as_ref()
                    .map(|text| BrowserInterventionTextRef(text.as_str()))
            })),
        ),
        BrowserInterventionFieldPair(
            constants::field::PROFILE_ID,
            optional_text(row.and_then(|value| {
                value
                    .profile_id
                    .as_ref()
                    .map(|text| BrowserInterventionTextRef(text.as_str()))
            })),
        ),
        BrowserInterventionFieldPair(
            constants::field::PROCESS_ID,
            optional_u32(row.and_then(|value| value.process_id)),
        ),
    ]
}

fn browser_intervention_decision_pairs(
    row: Option<&BrowserInterventionRow>,
) -> Vec<BrowserInterventionFieldPair> {
    vec![
        BrowserInterventionFieldPair(
            constants::field::POLICY_DECISION_ID,
            optional_text(row.and_then(|value| {
                value
                    .policy_decision_id
                    .as_ref()
                    .map(|text| BrowserInterventionTextRef(text.as_str()))
            })),
        ),
        BrowserInterventionFieldPair(
            constants::field::BROWSER_INTERVENTION_ACTION_ID,
            optional_text(row.and_then(|value| {
                value
                    .intervention_action_id
                    .as_ref()
                    .map(|text| BrowserInterventionTextRef(text.as_str()))
            })),
        ),
        BrowserInterventionFieldPair(
            constants::field::BROWSER_INTERVENTION_AUDIT_ID,
            optional_text(row.and_then(|value| {
                value
                    .intervention_audit_id
                    .as_ref()
                    .map(|text| BrowserInterventionTextRef(text.as_str()))
            })),
        ),
        BrowserInterventionFieldPair(
            constants::field::EVIDENCE_REFERENCE_IDS,
            optional_string_list(row.map(|value| {
                BrowserInterventionStringListRef(value.evidence_reference_ids.as_slice())
            })),
        ),
        BrowserInterventionFieldPair(
            constants::field::DECISION_SOURCE,
            optional_text(
                row.map(|value| {
                    BrowserInterventionTextRef(value.decision_source.as_protocol_str())
                }),
            ),
        ),
        BrowserInterventionFieldPair(
            constants::field::INTERVENTION_ACTION,
            optional_text(row.map(|value| {
                BrowserInterventionTextRef(value.intervention_action.as_protocol_str())
            })),
        ),
    ]
}

fn browser_intervention_target_pairs(
    row: Option<&BrowserInterventionRow>,
) -> Vec<BrowserInterventionFieldPair> {
    vec![
        BrowserInterventionFieldPair(
            constants::field::INTERVENTION_TARGET_TYPE,
            optional_text(row.map(|value| {
                BrowserInterventionTextRef(value.intervention_target_type.as_protocol_str())
            })),
        ),
        BrowserInterventionFieldPair(
            constants::field::INTERVENTION_TARGET_VALUE,
            optional_text(
                row.map(|value| {
                    BrowserInterventionTextRef(value.intervention_target_value.as_str())
                }),
            ),
        ),
        BrowserInterventionFieldPair(
            constants::field::REQUESTED_URL,
            optional_text(row.and_then(|value| {
                value
                    .requested_url
                    .as_ref()
                    .map(|text| BrowserInterventionTextRef(text.as_str()))
            })),
        ),
        BrowserInterventionFieldPair(
            constants::field::OBSERVED_URL,
            optional_text(row.and_then(|value| {
                value
                    .observed_url
                    .as_ref()
                    .map(|text| BrowserInterventionTextRef(text.as_str()))
            })),
        ),
    ]
}

fn browser_intervention_state_pairs(
    row: Option<&BrowserInterventionRow>,
) -> Vec<BrowserInterventionFieldPair> {
    vec![
        BrowserInterventionFieldPair(
            constants::field::INTERVENTION_MECHANISM,
            optional_text(row.map(|value| {
                BrowserInterventionTextRef(value.intervention_mechanism.as_protocol_str())
            })),
        ),
        BrowserInterventionFieldPair(
            constants::field::INTERVENTION_OUTCOME,
            optional_text(row.map(|value| {
                BrowserInterventionTextRef(value.intervention_outcome.as_protocol_str())
            })),
        ),
        BrowserInterventionFieldPair(
            constants::browser::INTERVENTION_FIELD_BROWSER_BOUNDARY_STATE,
            optional_text(row.map(|value| {
                BrowserInterventionTextRef(value.browser_boundary_state.as_protocol_str())
            })),
        ),
        BrowserInterventionFieldPair(
            constants::browser::INTERVENTION_FIELD_EXACT_URL_CLAIM_STATE,
            optional_text(row.map(|value| {
                BrowserInterventionTextRef(value.exact_url_claim_state.as_protocol_str())
            })),
        ),
        BrowserInterventionFieldPair(
            constants::browser::INTERVENTION_FIELD_UNMANAGED_DETECTION_STATE,
            optional_text(row.map(|value| {
                BrowserInterventionTextRef(value.unmanaged_detection_state.as_protocol_str())
            })),
        ),
        BrowserInterventionFieldPair(
            constants::field::CHILD_DELIVERY_STATE,
            optional_text(row.map(|value| {
                BrowserInterventionTextRef(value.child_delivery_state.as_protocol_str())
            })),
        ),
        BrowserInterventionFieldPair(
            constants::field::REASON,
            optional_text(row.and_then(|value| {
                value
                    .reason
                    .as_ref()
                    .map(|text| BrowserInterventionTextRef(text.as_str()))
            })),
        ),
        BrowserInterventionFieldPair(
            constants::field::CUSTODY_LABEL,
            optional_text(
                row.map(|value| BrowserInterventionTextRef(value.custody_label.as_protocol_str())),
            ),
        ),
        BrowserInterventionFieldPair(
            constants::field::QUERY_VISIBILITY,
            optional_text(
                row.map(|value| {
                    BrowserInterventionTextRef(value.query_visibility.as_protocol_str())
                }),
            ),
        ),
    ]
}
