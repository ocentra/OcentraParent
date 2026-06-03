use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE: &str = "child-device-query-store";
pub const TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS: &str = "no-tracking-events";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingReadModelRow {
    pub schema_version: u16,
    pub event_id: String,
    pub observed_at: String,
    pub device_id: String,
    pub platform: String,
    pub observer: String,
    pub kind: String,
    pub subject_kind: String,
    pub subject_id: String,
    pub subject_display_name: Option<String>,
    pub capability_status: Option<String>,
    pub evidence_reference_ids: Vec<String>,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub custody_label: String,
    pub limit: u64,
    pub returned: u64,
    pub capability_status: String,
    pub latest_event_id: Option<String>,
    pub latest_observed_at: Option<String>,
    pub evidence_reference_ids: Vec<String>,
    pub retention_tombstone_count: u64,
    pub retention_tombstone_evidence_reference_ids: Vec<String>,
    pub rows: Vec<TrackingReadModelRow>,
}
