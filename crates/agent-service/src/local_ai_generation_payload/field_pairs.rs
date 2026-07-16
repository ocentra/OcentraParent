use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::fields::fields_from_pairs;

pub(super) struct LocalAiGenerationFieldPair(pub(super) &'static str, pub(super) LogFieldValue);

#[derive(Clone, Copy)]
pub(super) struct LocalAiGenerationFieldKey(pub(super) &'static str);

pub(super) struct LocalAiGenerationOwnedText(pub(super) String);

#[derive(Clone, Copy)]
pub(super) struct LocalAiGenerationTextRef<'a>(pub(super) &'a str);

pub(super) fn local_ai_generation_fields_from_pairs(
    pairs: Vec<LocalAiGenerationFieldPair>,
) -> LogFields {
    fields_from_pairs(
        pairs
            .into_iter()
            .map(|LocalAiGenerationFieldPair(key, value)| (key, value))
            .collect(),
    )
}

pub(super) fn text_field(
    key: LocalAiGenerationFieldKey,
    value: LocalAiGenerationOwnedText,
) -> LocalAiGenerationFieldPair {
    LocalAiGenerationFieldPair(key.0, LogFieldValue::String(value.0))
}

pub(super) fn protocol_field(
    key: LocalAiGenerationFieldKey,
    value: LocalAiGenerationTextRef<'_>,
) -> LocalAiGenerationFieldPair {
    LocalAiGenerationFieldPair(key.0, LogFieldValue::String(value.0.to_string()))
}

pub(super) fn number_field(
    key: LocalAiGenerationFieldKey,
    value: u64,
) -> LocalAiGenerationFieldPair {
    LocalAiGenerationFieldPair(key.0, LogFieldValue::Number(value as f64))
}

pub(super) fn optional_text_field(
    key: LocalAiGenerationFieldKey,
    value: Option<LocalAiGenerationTextRef<'_>>,
) -> LocalAiGenerationFieldPair {
    LocalAiGenerationFieldPair(
        key.0,
        value.map_or(LogFieldValue::Null(()), |text| {
            LogFieldValue::String(text.0.to_string())
        }),
    )
}

pub(super) fn optional_exit_code_field(
    key: LocalAiGenerationFieldKey,
    value: Option<i32>,
) -> LocalAiGenerationFieldPair {
    LocalAiGenerationFieldPair(
        key.0,
        value.map_or(LogFieldValue::Null(()), |number| {
            LogFieldValue::Number(f64::from(number))
        }),
    )
}
