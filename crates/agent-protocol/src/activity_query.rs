use serde::{Deserialize, Serialize};

use crate::{ActivityEventKind, ActivityObserver, ActivitySubjectKind};

pub const ACTIVITY_QUERY_SCHEMA_VERSION: u16 = crate::ACTIVITY_QUERY_SCHEMA_VERSION;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityIngestStatus {
    pub schema_version: u16,
    pub database_ready: bool,
    pub events_ingested: u64,
    pub events_stored: u64,
    pub duplicate_events: u64,
    pub last_event_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityRecentQuery {
    pub schema_version: u16,
    pub limit: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityRecentSummary {
    pub schema_version: u16,
    pub limit: u64,
    pub returned: u64,
    pub first_observed_at: Option<String>,
    pub last_observed_at: Option<String>,
    pub last_event_id: Option<String>,
    pub most_recent_kind: Option<ActivityEventKind>,
    pub most_recent_observer: Option<ActivityObserver>,
    pub most_recent_subject_kind: Option<ActivitySubjectKind>,
    pub most_recent_subject_id: Option<String>,
    pub most_recent_subject_name: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityStoreRow {
    pub event_id: String,
    pub observed_at: String,
    pub device_id: String,
    pub platform: String,
    pub observer: String,
    pub kind: String,
    pub subject_kind: String,
    pub subject_id: String,
    pub subject_display_name: Option<String>,
}
