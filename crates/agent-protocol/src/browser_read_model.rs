use serde::{Deserialize, Serialize};

use crate::{
    BrowserActiveProofSource, BrowserActiveTabState, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserFamily, BrowserQueryVisibilityLabel,
};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserTabEvidence {
    pub schema_version: u16,
    pub browser_evidence_id: String,
    pub observed_at: String,
    pub fresh_until: String,
    pub source_id: String,
    pub adapter_id: String,
    pub device_id: String,
    pub browser_family: BrowserFamily,
    pub browser_channel: BrowserChannel,
    pub managed_browser_session_id: String,
    pub profile_id: String,
    pub process_id: u32,
    pub window_id: Option<String>,
    pub tab_id: Option<String>,
    pub target_id: Option<String>,
    pub active_state: BrowserActiveTabState,
    pub active_proof_source: BrowserActiveProofSource,
    pub url: String,
    pub origin: String,
    pub domain: String,
    pub title: Option<String>,
    pub capability_status: BrowserCapabilityStatus,
    pub degraded_reason: Option<String>,
    pub stale_at: String,
    pub custody_label: BrowserCustodyLabel,
    pub query_visibility: BrowserQueryVisibilityLabel,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserEvidenceReadModel {
    pub schema_version: u16,
    pub generated_at: String,
    pub limit: u64,
    pub returned: u64,
    pub latest_event_id: Option<String>,
    pub latest_observed_at: Option<String>,
    pub capability_status: Option<BrowserCapabilityStatus>,
    pub custody_label: BrowserCustodyLabel,
    pub query_visibility: BrowserQueryVisibilityLabel,
    pub rows: Vec<BrowserTabEvidence>,
}
