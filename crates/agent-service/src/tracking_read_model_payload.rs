use ocentra_parent_agent_protocol::{
    constants, LogFieldValue, LogFields, TrackingReadModel, TrackingReadModelRow,
};

use crate::fields::fields_from_pairs;

type FieldPair = (&'static str, LogFieldValue);

pub fn tracking_read_model_payload(read_model: &TrackingReadModel) -> LogFields {
    let latest = read_model.rows.first();
    let mut pairs = read_model_pairs(read_model);
    pairs.extend(latest_row_pairs(latest));
    fields_from_pairs(pairs)
}

fn read_model_pairs(read_model: &TrackingReadModel) -> Vec<FieldPair> {
    vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(read_model.custody_label.clone()),
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
        (
            constants::field::LATEST_EVENT_ID,
            optional_string(read_model.latest_event_id.as_ref()),
        ),
        (
            constants::field::LATEST_OBSERVED_AT,
            optional_string(read_model.latest_observed_at.as_ref()),
        ),
        (
            constants::field::ACTIVITY_TRACKING_READ_MODEL,
            LogFieldValue::String(
                serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES),
            ),
        ),
    ]
}

fn latest_row_pairs(row: Option<&TrackingReadModelRow>) -> Vec<FieldPair> {
    vec![
        (
            constants::field::DEVICE_ID,
            optional_string(row.map(|value| &value.device_id)),
        ),
        (
            constants::field::OBSERVER,
            optional_string(row.map(|value| &value.observer)),
        ),
        (
            constants::field::MOST_RECENT_KIND,
            optional_string(row.map(|value| &value.kind)),
        ),
        (
            constants::field::MOST_RECENT_SUBJECT_KIND,
            optional_string(row.map(|value| &value.subject_kind)),
        ),
        (
            constants::field::MOST_RECENT_SUBJECT_ID,
            optional_string(row.map(|value| &value.subject_id)),
        ),
        (
            constants::field::MOST_RECENT_SUBJECT_NAME,
            optional_string(row.and_then(|value| value.subject_display_name.as_ref())),
        ),
        (
            constants::field::EVIDENCE_REFERENCE_IDS,
            LogFieldValue::String(join_evidence_ids(row)),
        ),
    ]
}

fn optional_string(value: Option<&String>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.clone()),
        None => LogFieldValue::Null(()),
    }
}

fn join_evidence_ids(row: Option<&TrackingReadModelRow>) -> String {
    let separator = constants::delimiter::LIST.to_string();
    row.map(|value| value.evidence_reference_ids.join(&separator))
        .unwrap_or_default()
}
