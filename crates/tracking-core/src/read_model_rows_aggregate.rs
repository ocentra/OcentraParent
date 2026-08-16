use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::activity::ActivityEvidenceRef;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFields;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingEvidenceRef, TrackingReadModelDeletedAt, TrackingReadModelQueryVisibility,
};
use ocentra_parent_agent_protocol::tracking::read_model::{
    TrackingReadModelCount, TrackingReadModelRow, TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE,
    TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE,
};

use super::read_model_rows::{TrackingReadModelRowLifecycleState, TrackingStoreRow};

#[path = "read_model_rows_aggregate_counts.rs"]
mod read_model_rows_aggregate_counts;
#[path = "read_model_rows_aggregate_fields.rs"]
mod read_model_rows_aggregate_fields;

pub(crate) fn query_visibility(
    lifecycle_state: TrackingReadModelRowLifecycleState,
) -> TrackingReadModelQueryVisibility {
    match lifecycle_state {
        TrackingReadModelRowLifecycleState::Tombstone => {
            TrackingReadModelQueryVisibility::parse(TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE)
                .expect_value("tracking read-model tombstone visibility parses")
        }
        TrackingReadModelRowLifecycleState::Active => {
            TrackingReadModelQueryVisibility::parse(TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE)
                .expect_value("tracking read-model active visibility parses")
        }
    }
}

pub(crate) fn deleted_at(
    row: &TrackingStoreRow,
    lifecycle_state: TrackingReadModelRowLifecycleState,
) -> Option<TrackingReadModelDeletedAt> {
    match lifecycle_state {
        TrackingReadModelRowLifecycleState::Active => None,
        TrackingReadModelRowLifecycleState::Tombstone => {
            string_field(&row.fields, constants::field::DELETED_AT)
                .or_else(|| Some(row.observed_at.clone()))
                .map(|value| {
                    TrackingReadModelDeletedAt::parse(value)
                        .expect_value("tracking read-model deleted-at parses")
                })
        }
    }
}

pub(crate) fn deleted_evidence_reference_ids(
    rows: &[TrackingReadModelRow],
) -> Vec<TrackingEvidenceRef> {
    read_model_rows_aggregate_counts::deleted_evidence_reference_ids(rows)
}

pub(crate) fn active_counts_by(
    rows: &[TrackingReadModelRow],
    value_for_row: impl Fn(&TrackingReadModelRow) -> Option<&str>,
) -> Vec<TrackingReadModelCount> {
    read_model_rows_aggregate_counts::active_counts_by(rows, value_for_row)
}

pub(crate) fn evidence_reference_ids(
    fields: &LogFields,
    evidence: &[ActivityEvidenceRef],
) -> Vec<TrackingEvidenceRef> {
    read_model_rows_aggregate_fields::evidence_reference_ids(fields, evidence)
}

pub(crate) fn string_field(fields: &LogFields, key: &str) -> Option<String> {
    read_model_rows_aggregate_fields::string_field(fields, key)
}
