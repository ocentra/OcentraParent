use serde::{Deserialize, Serialize};

use crate::ActivityEvidenceRef;

pub const TRACKING_READ_MODEL_CUSTODY_CHILD_DEVICE_QUERY_STORE: &str = "child-device-query-store";
pub const TRACKING_READ_MODEL_STATUS_NO_TRACKING_EVENTS: &str = "no-tracking-events";
pub const TRACKING_READ_MODEL_ROW_VISIBILITY_ACTIVE: &str = "active";
pub const TRACKING_READ_MODEL_ROW_VISIBILITY_TOMBSTONE: &str = "tombstone";
pub const TRACKING_READ_MODEL_FIELD_ACTIVE_ROWS: &str = "activeRows";
pub const TRACKING_READ_MODEL_FIELD_TOMBSTONE_ROWS: &str = "tombstoneRows";
pub const TRACKING_READ_MODEL_FIELD_LATEST_TOMBSTONE_EVENT_ID: &str = "latestTombstoneEventId";
pub const TRACKING_READ_MODEL_FIELD_LATEST_TOMBSTONE_OBSERVED_AT: &str =
    "latestTombstoneObservedAt";
pub const TRACKING_READ_MODEL_FIELD_DELETED_EVIDENCE_REFERENCE_IDS: &str =
    "deletedEvidenceReferenceIds";
pub const TRACKING_READ_MODEL_SURFACE_LOCATION: &str = "location";
pub const TRACKING_READ_MODEL_SURFACE_GEOFENCE: &str = "geofence";
pub const TRACKING_READ_MODEL_SURFACE_EXPECTED_PLACE: &str = "expected-place";
pub const TRACKING_READ_MODEL_SURFACE_CHILD_CHECK_IN: &str = "child-check-in";
pub const TRACKING_READ_MODEL_SURFACE_RETENTION: &str = "retention";
pub const TRACKING_READ_MODEL_MISSING_PROOF_PLATFORM_REPLAY: &str =
    "platform-replay-proof-required";
pub const TRACKING_READ_MODEL_MISSING_PROOF_CHILD_RUNTIME: &str =
    "child-device-runtime-proof-required";
pub const TRACKING_READ_MODEL_MISSING_PROOF_PRODUCT_UI: &str = "broader-product-ui-proof-required";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingReadModelCoverageRow {
    pub schema_version: u16,
    pub surface: String,
    pub active_rows: u64,
    pub tombstone_rows: u64,
    pub citation_count: u64,
    pub latest_event_id: Option<String>,
    pub latest_observed_at: Option<String>,
    pub ready_for_product_claim: bool,
    pub missing_proof: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingReadModelProductClaimState {
    pub physical_device_claimed: bool,
    pub provider_delivery_claimed: bool,
    pub notification_delivery_claimed: bool,
    pub child_device_runtime_claimed: bool,
    pub ocentra_hosted_storage_claimed: bool,
    pub product_complete_claimed: bool,
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
    pub latest_tombstone_event_id: Option<String>,
    pub latest_tombstone_observed_at: Option<String>,
    pub deleted_evidence_reference_ids: Vec<String>,
    pub coverage_rows: Vec<TrackingReadModelCoverageRow>,
    pub product_claim_state: TrackingReadModelProductClaimState,
    pub rows: Vec<TrackingReadModelRow>,
}
