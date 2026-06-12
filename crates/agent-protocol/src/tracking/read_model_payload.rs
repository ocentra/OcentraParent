use crate::{
    constants, LogFieldValue, LogFields, TrackingReadModel, TrackingReadModelCount,
    TrackingReadModelRow, TRACKING_READ_MODEL_FIELD_ACTIVE_CAPABILITY_STATUS_COUNTS,
    TRACKING_READ_MODEL_FIELD_ACTIVE_DEVICE_COUNTS, TRACKING_READ_MODEL_FIELD_ACTIVE_KIND_COUNTS,
    TRACKING_READ_MODEL_FIELD_ACTIVE_ROWS,
    TRACKING_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
    TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_EVENT_ID,
    TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_OBSERVED_AT,
    TRACKING_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID,
    TRACKING_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT,
    TRACKING_READ_MODEL_FIELD_TOMBSTONE_ROWS,
};

type FieldPair = (&'static str, LogFieldValue);

pub fn tracking_read_model_payload(read_model: &TrackingReadModel) -> LogFields {
    let latest = read_model.rows.first();
    let mut pairs = read_model_pairs(read_model);
    pairs.extend(latest_row_pairs(latest));
    fields_from_pairs(pairs)
}

fn read_model_pairs(read_model: &TrackingReadModel) -> Vec<FieldPair> {
    let mut pairs = read_model_summary_pairs(read_model);
    pairs.extend(read_model_latest_pairs(read_model));
    pairs.extend(read_model_retention_pairs(read_model));
    pairs.extend(read_model_active_count_pairs(read_model));
    pairs.push((
        constants::field::ACTIVITY_TRACKING_READ_MODEL,
        LogFieldValue::String(tracking_read_model_json(read_model)),
    ));
    pairs
}

fn read_model_summary_pairs(read_model: &TrackingReadModel) -> Vec<FieldPair> {
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
            TRACKING_READ_MODEL_FIELD_ACTIVE_ROWS,
            LogFieldValue::Number(read_model.active_rows as f64),
        ),
        (
            TRACKING_READ_MODEL_FIELD_TOMBSTONE_ROWS,
            LogFieldValue::Number(read_model.tombstone_rows as f64),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(read_model.capability_status.clone()),
        ),
    ]
}

fn read_model_latest_pairs(read_model: &TrackingReadModel) -> Vec<FieldPair> {
    vec![
        (
            constants::field::LATEST_EVENT_ID,
            optional_string(read_model.latest_event_id.as_ref()),
        ),
        (
            constants::field::LATEST_OBSERVED_AT,
            optional_string(read_model.latest_observed_at.as_ref()),
        ),
        (
            TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_EVENT_ID,
            optional_string(read_model.latest_active_event_id.as_ref()),
        ),
        (
            TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_OBSERVED_AT,
            optional_string(read_model.latest_active_observed_at.as_ref()),
        ),
        (
            TRACKING_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID,
            optional_string(read_model.latest_tombstone_event_id.as_ref()),
        ),
        (
            TRACKING_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT,
            optional_string(read_model.latest_tombstone_observed_at.as_ref()),
        ),
    ]
}

fn read_model_retention_pairs(read_model: &TrackingReadModel) -> Vec<FieldPair> {
    let separator = constants::delimiter::LIST.to_string();
    vec![(
        TRACKING_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
        LogFieldValue::String(read_model.deleted_evidence_reference_ids.join(&separator)),
    )]
}

fn read_model_active_count_pairs(read_model: &TrackingReadModel) -> Vec<FieldPair> {
    vec![
        (
            TRACKING_READ_MODEL_FIELD_ACTIVE_KIND_COUNTS,
            LogFieldValue::String(active_counts_json(&read_model.active_kind_counts)),
        ),
        (
            TRACKING_READ_MODEL_FIELD_ACTIVE_DEVICE_COUNTS,
            LogFieldValue::String(active_counts_json(&read_model.active_device_counts)),
        ),
        (
            TRACKING_READ_MODEL_FIELD_ACTIVE_CAPABILITY_STATUS_COUNTS,
            LogFieldValue::String(active_counts_json(
                &read_model.active_capability_status_counts,
            )),
        ),
    ]
}

fn active_counts_json(counts: &[TrackingReadModelCount]) -> String {
    serde_json::to_string(counts).expect(constants::error::AGENT_EVENT_SERIALIZES)
}

fn tracking_read_model_json(read_model: &TrackingReadModel) -> String {
    serde_json::to_string(read_model).expect(constants::error::AGENT_EVENT_SERIALIZES)
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
        (
            constants::field::QUERY_VISIBILITY,
            optional_string(row.map(|value| &value.query_visibility)),
        ),
        (
            constants::field::DELETED_AT,
            optional_string(row.and_then(|value| value.deleted_at.as_ref())),
        ),
    ]
}

fn fields_from_pairs(pairs: Vec<FieldPair>) -> LogFields {
    let mut fields = LogFields::new();
    for (key, value) in pairs {
        fields.insert(key.to_string(), value);
    }
    fields
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
