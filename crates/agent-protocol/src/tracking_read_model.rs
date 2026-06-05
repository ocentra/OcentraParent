use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE: &str = "child-device-query-store";
pub const TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS: &str = "no-tracking-events";
pub const TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE: &str = "active";
pub const TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE: &str = "tombstone";
pub const TRACKING_READ_MODEL_FIELD_ACTIVE_ROWS: &str = "activeRows";
pub const TRACKING_READ_MODEL_FIELD_TOMBSTONE_ROWS: &str = "tombstoneRows";
pub const TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_EVENT_ID: &str = "latestActiveEventId";
pub const TRACKING_READ_MODEL_FIELD_LATEST_ACTIVE_OBSERVED_AT: &str = "latestActiveObservedAt";
pub const TRACKING_READ_MODEL_FIELD_ACTIVE_KIND_COUNTS: &str = "activeKindCounts";
pub const TRACKING_READ_MODEL_FIELD_ACTIVE_DEVICE_COUNTS: &str = "activeDeviceCounts";
pub const TRACKING_READ_MODEL_FIELD_ACTIVE_CAPABILITY_STATUS_COUNTS: &str =
    "activeCapabilityStatusCounts";
pub const TRACKING_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID: &str = "latestTombstoneEventId";
pub const TRACKING_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT: &str =
    "latestTombstoneObservedAt";
pub const TRACKING_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS: &str =
    "deletedEvidenceReferenceIds";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingReadModelCount {
    pub value: String,
    pub count: u64,
}

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
    pub query_visibility: String,
    pub deleted_at: Option<String>,
    pub evidence_reference_ids: Vec<String>,
    pub deleted_evidence_reference_ids: Vec<String>,
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
    pub active_rows: u64,
    pub tombstone_rows: u64,
    pub capability_status: String,
    pub latest_event_id: Option<String>,
    pub latest_observed_at: Option<String>,
    pub latest_active_event_id: Option<String>,
    pub latest_active_observed_at: Option<String>,
    pub latest_tombstone_event_id: Option<String>,
    pub latest_tombstone_observed_at: Option<String>,
    pub active_kind_counts: Vec<TrackingReadModelCount>,
    pub active_device_counts: Vec<TrackingReadModelCount>,
    pub active_capability_status_counts: Vec<TrackingReadModelCount>,
    pub deleted_evidence_reference_ids: Vec<String>,
    pub rows: Vec<TrackingReadModelRow>,
}
