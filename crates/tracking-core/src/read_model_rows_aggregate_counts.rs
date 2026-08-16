use std::collections::BTreeMap;

use ocentra_eventing::expect_value::ExpectValue;
use ocentra_parent_agent_protocol::tracking::identifiers::{
    TrackingEvidenceRef, TrackingReadModelCountValue,
};
use ocentra_parent_agent_protocol::tracking::read_model::{
    TrackingReadModelCount, TrackingReadModelRow, TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE,
};

pub(super) fn deleted_evidence_reference_ids(
    rows: &[TrackingReadModelRow],
) -> Vec<TrackingEvidenceRef> {
    let mut ids = Vec::new();
    for row in rows {
        for id in &row.deleted_evidence_reference_ids {
            if !ids.iter().any(|existing| existing == id) {
                ids.push(id.clone());
            }
        }
    }
    ids
}

pub(super) fn active_counts_by(
    rows: &[TrackingReadModelRow],
    value_for_row: impl Fn(&TrackingReadModelRow) -> Option<&str>,
) -> Vec<TrackingReadModelCount> {
    let mut counts = BTreeMap::<String, u64>::new();
    for row in rows {
        if row.query_visibility != TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE {
            continue;
        }
        if let Some(value) = value_for_row(row) {
            *counts.entry(value.to_string()).or_insert(0) += 1;
        }
    }
    counts
        .into_iter()
        .map(|(value, count)| TrackingReadModelCount {
            value: TrackingReadModelCountValue::parse(value)
                .expect_value("tracking read-model count value parses"),
            count,
        })
        .collect()
}
