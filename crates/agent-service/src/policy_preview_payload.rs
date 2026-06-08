use ocentra_parent_agent_protocol::{
    constants, policy_constants as policy, LogFieldValue, LogFields, PolicyPreviewReadModel,
    PolicyPreviewReadModelRow,
};

use crate::fields::fields_from_pairs;

type FieldPair = (&'static str, LogFieldValue);

pub fn policy_preview_read_model_payload(read_model: &PolicyPreviewReadModel) -> LogFields {
    let latest = read_model.rows.first();
    let mut pairs = read_model_pairs(read_model);
    pairs.extend(row_pairs(latest));
    pairs.extend(decision_pairs(latest));
    pairs.extend(network_evidence_mapping_pairs(latest));
    fields_from_pairs(pairs)
}

fn read_model_pairs(read_model: &PolicyPreviewReadModel) -> Vec<FieldPair> {
    vec![
        (
            constants::field::SCHEMA_VERSION,
            LogFieldValue::String(read_model.schema_version.clone()),
        ),
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY,
            LogFieldValue::String(read_model.custody.clone()),
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
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(read_model.capability_status.clone()),
        ),
    ]
}

fn row_pairs(row: Option<&PolicyPreviewReadModelRow>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::POLICY_PREVIEW_ID,
            optional_string(row.map(|value| &value.preview_id)),
        ),
        (
            constants::field::LATEST_EVENT_ID,
            optional_string(row.map(|value| &value.source_event_id)),
        ),
        (
            constants::field::LATEST_OBSERVED_AT,
            optional_string(row.map(|value| &value.observed_at)),
        ),
        (
            constants::field::TARGET_ID,
            optional_string(row.map(|value| &value.target.target_id)),
        ),
        (
            constants::field::POLICY_TARGET_TYPE,
            optional_protocol_string(row.map(|value| value.target.target_type.as_protocol_str())),
        ),
        (
            constants::field::POLICY_TARGET_VALUE,
            optional_string(row.map(|value| &value.target.target_value)),
        ),
        (
            constants::field::POLICY_EVIDENCE_REFERENCE_COUNT,
            optional_u64(row.map(|value| value.evidence_references.len() as u64)),
        ),
        (
            policy::PARENT_RULE_CONTEXT_REFERENCE_COUNT_FIELD,
            optional_u64(row.map(|value| value.parent_rule_context_references.len() as u64)),
        ),
        (
            policy::PARENT_RULE_CONTEXT_REF_IDS_FIELD,
            optional_parent_rule_context_ref_ids(row),
        ),
    ]
}

fn decision_pairs(row: Option<&PolicyPreviewReadModelRow>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::POLICY_DECISION_ID,
            optional_string(row.map(|value| &value.decision.decision_id)),
        ),
        (
            constants::field::POLICY_ACTION,
            optional_protocol_string(row.map(|value| value.decision.action.as_protocol_str())),
        ),
        (
            constants::field::POLICY_REASON_CODES,
            optional_list(row.map(|value| value.decision.reason_codes.as_slice())),
        ),
        (
            constants::field::POLICY_RULE_IDS,
            optional_list(row.map(|value| value.decision.rule_ids.as_slice())),
        ),
        (
            constants::field::LOCAL_AI_RESULT_ID,
            optional_string(row.and_then(|value| value.decision.local_ai_result_id.as_ref())),
        ),
        (
            constants::field::POLICY_DRY_RUN,
            optional_bool(row.map(|value| value.decision.dry_run)),
        ),
        (
            constants::field::POLICY_HANDOFF_STATE,
            optional_protocol_string(
                row.map(|value| value.decision.enforcement_handoff_state.as_protocol_str()),
            ),
        ),
    ]
}

fn network_evidence_mapping_pairs(row: Option<&PolicyPreviewReadModelRow>) -> Vec<FieldPair> {
    let mapping = row.and_then(|value| value.network_evidence_mapping.as_ref());
    vec![
        (
            constants::field::NETWORK_EVIDENCE_GRADE,
            optional_string(mapping.map(|value| &value.evidence_grade)),
        ),
        (
            constants::field::NETWORK_REQUESTED_POLICY_ACTION,
            optional_string(mapping.map(|value| &value.requested_action)),
        ),
        (
            constants::field::NETWORK_MAPPED_POLICY_ACTION,
            optional_string(mapping.map(|value| &value.mapped_action)),
        ),
        (
            constants::field::NETWORK_POLICY_MAPPING_MODE,
            optional_string(mapping.map(|value| &value.mode)),
        ),
        (
            constants::field::NETWORK_ADAPTER_ACTION_AUTHORIZED,
            optional_bool(mapping.map(|value| value.adapter_action_authorized)),
        ),
        (
            constants::field::NETWORK_ENFORCEMENT_COMMAND_AUTHORIZED,
            optional_bool(mapping.map(|value| value.enforcement_command_authorized)),
        ),
    ]
}

fn optional_string(value: Option<&String>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.clone()),
        None => LogFieldValue::Null(()),
    }
}

fn optional_protocol_string(value: Option<&str>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.to_string()),
        None => LogFieldValue::Null(()),
    }
}

fn optional_list(value: Option<&[String]>) -> LogFieldValue {
    match value {
        Some(values) if !values.is_empty() => {
            LogFieldValue::String(values.join(&constants::delimiter::LIST.to_string()))
        }
        _ => LogFieldValue::Null(()),
    }
}

fn optional_parent_rule_context_ref_ids(row: Option<&PolicyPreviewReadModelRow>) -> LogFieldValue {
    match row {
        Some(value) if !value.parent_rule_context_references.is_empty() => LogFieldValue::String(
            value
                .parent_rule_context_references
                .iter()
                .map(|reference| reference.parent_rule_ref_id.as_str())
                .collect::<Vec<_>>()
                .join(&constants::delimiter::LIST.to_string()),
        ),
        _ => LogFieldValue::Null(()),
    }
}

fn optional_bool(value: Option<bool>) -> LogFieldValue {
    match value {
        Some(flag) => LogFieldValue::Boolean(flag),
        None => LogFieldValue::Null(()),
    }
}

fn optional_u64(value: Option<u64>) -> LogFieldValue {
    match value {
        Some(number) => LogFieldValue::Number(number as f64),
        None => LogFieldValue::Null(()),
    }
}
