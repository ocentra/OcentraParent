use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentPlatform {
    #[serde(rename = "windows")]
    Windows,
    #[serde(rename = "linux")]
    Linux,
    #[serde(rename = "macos")]
    Macos,
    #[serde(rename = "android")]
    Android,
    #[serde(rename = "ios")]
    Ios,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentActorRole {
    #[serde(rename = "parent")]
    Parent,
    #[serde(rename = "guardian")]
    Guardian,
    #[serde(rename = "system")]
    System,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ParentEvidenceReferenceKind {
    #[serde(rename = "journal-event")]
    JournalEvent,
    #[serde(rename = "query-store-summary")]
    QueryStoreSummary,
    #[serde(rename = "activity-event")]
    ActivityEvent,
    #[serde(rename = "policy-decision")]
    PolicyDecision,
    #[serde(rename = "local-ai-result")]
    LocalAiResult,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReportQueryCustodyState {
    #[serde(rename = "derivedFresh")]
    DerivedFresh,
    #[serde(rename = "derivedStale")]
    DerivedStale,
    #[serde(rename = "partiallyRedacted")]
    PartiallyRedacted,
    #[serde(rename = "deletedSource")]
    DeletedSource,
    #[serde(rename = "syncConflict")]
    SyncConflict,
    #[serde(rename = "cursorExpired")]
    CursorExpired,
    #[serde(rename = "rateLimited")]
    RateLimited,
}

impl ReportQueryCustodyState {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DerivedFresh => super::REPORT_QUERY_CUSTODY_STATE_DERIVED_FRESH,
            Self::DerivedStale => super::REPORT_QUERY_CUSTODY_STATE_DERIVED_STALE,
            Self::PartiallyRedacted => super::REPORT_QUERY_CUSTODY_STATE_PARTIALLY_REDACTED,
            Self::DeletedSource => super::REPORT_QUERY_CUSTODY_STATE_DELETED_SOURCE,
            Self::SyncConflict => super::REPORT_QUERY_CUSTODY_STATE_SYNC_CONFLICT,
            Self::CursorExpired => super::REPORT_QUERY_CUSTODY_STATE_CURSOR_EXPIRED,
            Self::RateLimited => super::REPORT_QUERY_CUSTODY_STATE_RATE_LIMITED,
        }
    }
}
