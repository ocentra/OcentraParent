use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::fields::fields_from_pairs;

pub(super) struct BrowserInterventionFieldPair(pub(super) &'static str, pub(super) LogFieldValue);

pub(super) struct BrowserInterventionTextRef<'a>(pub(super) &'a str);

pub(super) struct BrowserInterventionStringListRef<'a>(pub(super) &'a [String]);

pub(super) fn browser_intervention_fields_from_pairs(
    pairs: Vec<BrowserInterventionFieldPair>,
) -> LogFields {
    fields_from_pairs(
        pairs
            .into_iter()
            .map(|BrowserInterventionFieldPair(key, value)| (key, value))
            .collect(),
    )
}

pub(super) fn optional_text(value: Option<BrowserInterventionTextRef<'_>>) -> LogFieldValue {
    value.map_or(LogFieldValue::Null(()), |text| {
        LogFieldValue::String(text.0.to_string())
    })
}

pub(super) fn optional_u32(value: Option<u32>) -> LogFieldValue {
    value.map_or(LogFieldValue::Null(()), |number| {
        LogFieldValue::Number(number as f64)
    })
}

pub(super) fn optional_string_list(
    value: Option<BrowserInterventionStringListRef<'_>>,
) -> LogFieldValue {
    value
        .filter(|items| !items.0.is_empty())
        .map_or(LogFieldValue::Null(()), |items| {
            LogFieldValue::String(items.0.join(&constants::delimiter::LIST.to_string()))
        })
}
