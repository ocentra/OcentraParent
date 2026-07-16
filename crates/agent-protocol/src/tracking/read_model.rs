use serde::{Deserialize, Serialize};

use super::identifiers::{
    TrackingEvidenceRef, TrackingReadModelCapabilityStatus, TrackingReadModelCountValue,
    TrackingReadModelCustodyLabel, TrackingReadModelDeletedAt, TrackingReadModelDeviceId,
    TrackingReadModelEventId, TrackingReadModelGeneratedAt, TrackingReadModelKind,
    TrackingReadModelObservedAt, TrackingReadModelObserver, TrackingReadModelPlatform,
    TrackingReadModelQueryVisibility, TrackingReadModelSubjectDisplayName,
    TrackingReadModelSubjectId, TrackingReadModelSubjectKind,
};
use crate::ActivityEvidenceRef;

pub const TRACKING_READ_MODEL_SCHEMA_VERSION: u16 = crate::ACTIVITY_QUERY_SCHEMA_VERSION;
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
    pub value: TrackingReadModelCountValue,
    pub count: u64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingReadModelRow {
    pub schema_version: u16,
    pub event_id: TrackingReadModelEventId,
    pub observed_at: TrackingReadModelObservedAt,
    pub device_id: TrackingReadModelDeviceId,
    pub platform: TrackingReadModelPlatform,
    pub observer: TrackingReadModelObserver,
    pub kind: TrackingReadModelKind,
    pub subject_kind: TrackingReadModelSubjectKind,
    pub subject_id: TrackingReadModelSubjectId,
    pub subject_display_name: Option<TrackingReadModelSubjectDisplayName>,
    pub capability_status: Option<TrackingReadModelCapabilityStatus>,
    pub query_visibility: TrackingReadModelQueryVisibility,
    pub deleted_at: Option<TrackingReadModelDeletedAt>,
    pub evidence_reference_ids: Vec<TrackingEvidenceRef>,
    pub deleted_evidence_reference_ids: Vec<TrackingEvidenceRef>,
    pub evidence: Vec<ActivityEvidenceRef>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingReadModel {
    pub schema_version: u16,
    pub generated_at: TrackingReadModelGeneratedAt,
    pub custody_label: TrackingReadModelCustodyLabel,
    pub limit: u64,
    pub returned: u64,
    pub active_rows: u64,
    pub tombstone_rows: u64,
    pub capability_status: TrackingReadModelCapabilityStatus,
    pub latest_event_id: Option<TrackingReadModelEventId>,
    pub latest_observed_at: Option<TrackingReadModelObservedAt>,
    pub latest_active_event_id: Option<TrackingReadModelEventId>,
    pub latest_active_observed_at: Option<TrackingReadModelObservedAt>,
    pub latest_tombstone_event_id: Option<TrackingReadModelEventId>,
    pub latest_tombstone_observed_at: Option<TrackingReadModelObservedAt>,
    pub active_kind_counts: Vec<TrackingReadModelCount>,
    pub active_device_counts: Vec<TrackingReadModelCount>,
    pub active_capability_status_counts: Vec<TrackingReadModelCount>,
    pub deleted_evidence_reference_ids: Vec<TrackingEvidenceRef>,
    pub rows: Vec<TrackingReadModelRow>,
}
