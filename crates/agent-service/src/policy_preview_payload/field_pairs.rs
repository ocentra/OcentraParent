use ocentra_parent_agent_protocol::activity::policy_preview::PolicyPreviewReadModelRow;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::fields::fields_from_pairs;

pub(super) struct PolicyPreviewFieldPair(pub(super) &'static str, pub(super) LogFieldValue);

pub(super) struct PolicyPreviewTextRef<'a>(pub(super) &'a str);

pub(super) struct PolicyPreviewStringListRef<'a>(pub(super) &'a [String]);

pub(super) fn policy_preview_fields_from_pairs(pairs: Vec<PolicyPreviewFieldPair>) -> LogFields {
    fields_from_pairs(
        pairs
            .into_iter()
            .map(|PolicyPreviewFieldPair(key, value)| (key, value))
            .collect(),
    )
}

pub(super) fn optional_text(value: Option<PolicyPreviewTextRef<'_>>) -> LogFieldValue {
    value.map_or(LogFieldValue::Null(()), |text| {
        LogFieldValue::String(text.0.to_string())
    })
}

pub(super) fn optional_string_list(value: Option<PolicyPreviewStringListRef<'_>>) -> LogFieldValue {
    value
        .filter(|values| !values.0.is_empty())
        .map_or(LogFieldValue::Null(()), |values| {
            LogFieldValue::String(values.0.join(&constants::delimiter::LIST.to_string()))
        })
}

pub(super) fn optional_parent_rule_context_ref_ids(
    row: Option<&PolicyPreviewReadModelRow>,
) -> LogFieldValue {
    row.filter(|value| !value.parent_rule_context_references.is_empty())
        .map_or(LogFieldValue::Null(()), |value| {
            LogFieldValue::String(
                value
                    .parent_rule_context_references
                    .iter()
                    .map(|reference| reference.parent_rule_ref_id.as_str())
                    .collect::<Vec<_>>()
                    .join(&constants::delimiter::LIST.to_string()),
            )
        })
}

pub(super) fn optional_bool(value: Option<bool>) -> LogFieldValue {
    value.map_or(LogFieldValue::Null(()), LogFieldValue::Boolean)
}

pub(super) fn optional_u64(value: Option<u64>) -> LogFieldValue {
    value.map_or(LogFieldValue::Null(()), |number| {
        LogFieldValue::Number(number as f64)
    })
}
