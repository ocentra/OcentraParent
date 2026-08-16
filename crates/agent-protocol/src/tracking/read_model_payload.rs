use super::read_model::{
    TrackingReadModel, TrackingReadModelCount, TrackingReadModelRow,
    TRACKING_READ_MODEL_FIELD_ACTIVE_CAPABILITY_STATUS_COUNTS,
    TRACKING_READ_MODEL_FIELD_ACTIVE_DEVICE_COUNTS, TRACKING_READ_MODEL_FIELD_ACTIVE_KIND_COUNTS,
    TRACKING_READ_MODEL_FIELD_ACTIVE_ROWS,
    TRACKING_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
    TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_EVENT_ID,
    TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_OBSERVED_AT,
    TRACKING_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID,
    TRACKING_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT,
    TRACKING_READ_MODEL_FIELD_TOMBSTONE_ROWS,
};
use crate::{constants, LogFieldValue, LogFields};
use ocentra_eventing::expect_value::ExpectValue;

struct FieldPair {
    key: &'static str,
    value: LogFieldValue,
}

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
    pairs.push(FieldPair {
        key: constants::field::ACTIVITY_TRACKING_READ_MODEL,
        value: LogFieldValue::String(tracking_read_model_json(read_model)),
    });
    pairs
}

fn read_model_summary_pairs(read_model: &TrackingReadModel) -> Vec<FieldPair> {
    vec![
        FieldPair {
            key: constants::field::GENERATED_AT,
            value: LogFieldValue::String(read_model.generated_at.to_string()),
        },
        FieldPair {
            key: constants::field::CUSTODY_LABEL,
            value: LogFieldValue::String(read_model.custody_label.to_string()),
        },
        FieldPair {
            key: constants::field::LIMIT,
            value: LogFieldValue::Number(read_model.limit as f64),
        },
        FieldPair {
            key: constants::field::RETURNED,
            value: LogFieldValue::Number(read_model.returned as f64),
        },
        FieldPair {
            key: TRACKING_READ_MODEL_FIELD_ACTIVE_ROWS,
            value: LogFieldValue::Number(read_model.active_rows as f64),
        },
        FieldPair {
            key: TRACKING_READ_MODEL_FIELD_TOMBSTONE_ROWS,
            value: LogFieldValue::Number(read_model.tombstone_rows as f64),
        },
        FieldPair {
            key: constants::field::CAPABILITY_STATUS,
            value: LogFieldValue::String(read_model.capability_status.to_string()),
        },
    ]
}

fn read_model_latest_pairs(read_model: &TrackingReadModel) -> Vec<FieldPair> {
    vec![
        FieldPair {
            key: constants::field::LATEST_EVENT_ID,
            value: optional_string(read_model.latest_event_id.as_ref()),
        },
        FieldPair {
            key: constants::field::LATEST_OBSERVED_AT,
            value: optional_string(read_model.latest_observed_at.as_ref()),
        },
        FieldPair {
            key: TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_EVENT_ID,
            value: optional_string(read_model.latest_active_event_id.as_ref()),
        },
        FieldPair {
            key: TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_OBSERVED_AT,
            value: optional_string(read_model.latest_active_observed_at.as_ref()),
        },
        FieldPair {
            key: TRACKING_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID,
            value: optional_string(read_model.latest_tombstone_event_id.as_ref()),
        },
        FieldPair {
            key: TRACKING_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT,
            value: optional_string(read_model.latest_tombstone_observed_at.as_ref()),
        },
    ]
}

fn read_model_retention_pairs(read_model: &TrackingReadModel) -> Vec<FieldPair> {
    let separator = constants::delimiter::LIST.to_string();
    vec![FieldPair {
        key: TRACKING_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS,
        value: LogFieldValue::String(
            read_model
                .deleted_evidence_reference_ids
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(&separator),
        ),
    }]
}

fn read_model_active_count_pairs(read_model: &TrackingReadModel) -> Vec<FieldPair> {
    vec![
        FieldPair {
            key: TRACKING_READ_MODEL_FIELD_ACTIVE_KIND_COUNTS,
            value: LogFieldValue::String(active_counts_json(&read_model.active_kind_counts)),
        },
        FieldPair {
            key: TRACKING_READ_MODEL_FIELD_ACTIVE_DEVICE_COUNTS,
            value: LogFieldValue::String(active_counts_json(&read_model.active_device_counts)),
        },
        FieldPair {
            key: TRACKING_READ_MODEL_FIELD_ACTIVE_CAPABILITY_STATUS_COUNTS,
            value: LogFieldValue::String(active_counts_json(
                &read_model.active_capability_status_counts,
            )),
        },
    ]
}

fn active_counts_json(counts: &[TrackingReadModelCount]) -> String {
    serde_json::to_string(counts).expect_value(constants::error::AGENT_EVENT_SERIALIZES)
}

fn tracking_read_model_json(read_model: &TrackingReadModel) -> String {
    serde_json::to_string(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES)
}

fn latest_row_pairs(row: Option<&TrackingReadModelRow>) -> Vec<FieldPair> {
    vec![
        FieldPair {
            key: constants::field::DEVICE_ID,
            value: optional_string(row.map(|value| &value.device_id)),
        },
        FieldPair {
            key: constants::field::OBSERVER,
            value: optional_string(row.map(|value| &value.observer)),
        },
        FieldPair {
            key: constants::field::MOST_RECENT_KIND,
            value: optional_string(row.map(|value| &value.kind)),
        },
        FieldPair {
            key: constants::field::MOST_RECENT_SUBJECT_KIND,
            value: optional_string(row.map(|value| &value.subject_kind)),
        },
        FieldPair {
            key: constants::field::MOST_RECENT_SUBJECT_ID,
            value: optional_string(row.map(|value| &value.subject_id)),
        },
        FieldPair {
            key: constants::field::MOST_RECENT_SUBJECT_NAME,
            value: optional_string(row.and_then(|value| value.subject_display_name.as_ref())),
        },
        FieldPair {
            key: constants::field::EVIDENCE_REFERENCE_IDS,
            value: LogFieldValue::String(join_evidence_ids(row)),
        },
        FieldPair {
            key: constants::field::QUERY_VISIBILITY,
            value: optional_string(row.map(|value| &value.query_visibility)),
        },
        FieldPair {
            key: constants::field::DELETED_AT,
            value: optional_string(row.and_then(|value| value.deleted_at.as_ref())),
        },
    ]
}

fn fields_from_pairs(pairs: Vec<FieldPair>) -> LogFields {
    let mut fields = LogFields::new();
    for pair in pairs {
        fields.insert(pair.key.to_string(), pair.value);
    }
    fields
}

fn optional_string(value: Option<impl std::fmt::Display>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.to_string()),
        None => LogFieldValue::Null(()),
    }
}

fn join_evidence_ids(row: Option<&TrackingReadModelRow>) -> String {
    let separator = constants::delimiter::LIST.to_string();
    row.map(|value| {
        value
            .evidence_reference_ids
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(&separator)
    })
    .unwrap_or_default()
}
