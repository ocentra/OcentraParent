use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReportQueryCustodySourceFreshness {
    #[serde(rename = "fresh")]
    Fresh,
    #[serde(rename = "stale")]
    Stale,
    #[serde(rename = "deleted")]
    Deleted,
    #[serde(rename = "conflicted")]
    Conflicted,
    #[serde(rename = "expired")]
    Expired,
    #[serde(rename = "rate-limited")]
    RateLimited,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReportQueryCustodyBoundary {
    #[serde(rename = "parent-owned-citations-only")]
    ParentOwnedCitationsOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReportQueryCustodyPayloadRedaction {
    #[serde(rename = "fully-redacted")]
    FullyRedacted,
    #[serde(rename = "partially-redacted")]
    PartiallyRedacted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReportQueryCustodySourceDataClass {
    #[serde(rename = "sqlite-query-row")]
    SqliteQueryRow,
    #[serde(rename = "notification-history")]
    NotificationHistory,
    #[serde(rename = "audit-event")]
    AuditEvent,
    #[serde(rename = "generated-summary")]
    GeneratedSummary,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReportQueryCustodyNonClaim {
    #[serde(rename = "no-second-truth-store")]
    NoSecondTruthStore,
    #[serde(rename = "no-portal-ui")]
    NoPortalUi,
    #[serde(rename = "no-raw-child-evidence")]
    NoRawChildEvidence,
    #[serde(rename = "no-unbounded-pagination")]
    NoUnboundedPagination,
    #[serde(rename = "no-provider-routing")]
    NoProviderRouting,
    #[serde(rename = "no-ocentra-hosted-family-data-custody")]
    NoOcentraHostedFamilyDataCustody,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum ReportQueryCustodyTombstoneState {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "written")]
    Written,
}
