use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

pub(super) struct BrowserEvidenceFieldPair(pub(super) &'static str, pub(super) LogFieldValue);

pub(super) struct BrowserEvidenceFieldKey(pub(super) &'static str);

pub(super) struct BrowserEvidenceTextRef<'a>(pub(super) &'a str);

pub(super) fn browser_evidence_fields_from_pairs(
    pairs: Vec<BrowserEvidenceFieldPair>,
) -> LogFields {
    crate::fields::fields_from_pairs(pairs.into_iter().map(|pair| (pair.0, pair.1)).collect())
}

pub(super) fn field_pair(
    BrowserEvidenceFieldKey(key): BrowserEvidenceFieldKey,
    value: LogFieldValue,
) -> BrowserEvidenceFieldPair {
    BrowserEvidenceFieldPair(key, value)
}

pub(super) fn optional_text(value: Option<BrowserEvidenceTextRef<'_>>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.0.to_string()),
        None => LogFieldValue::Null(()),
    }
}

pub(super) fn optional_u32(value: Option<u32>) -> LogFieldValue {
    match value {
        Some(number) => LogFieldValue::Number(number as f64),
        None => LogFieldValue::Null(()),
    }
}
