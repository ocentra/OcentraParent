use serde::{Deserialize, Serialize};

use crate::{BrowserPolicyCapabilityRegistry, BrowserPolicyEffectivePolicy, BrowserPolicyValue};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyUpdateKind {
    #[serde(rename = "get")]
    Get,
    #[serde(rename = "preview")]
    Preview,
    #[serde(rename = "patch")]
    Patch,
    #[serde(rename = "replace")]
    Replace,
    #[serde(rename = "rollback")]
    Rollback,
}

impl BrowserPolicyUpdateKind {
    pub fn as_protocol_str(&self) -> &'static str {
        const PROTOCOL_STRINGS: [&str; 5] = [
            crate::constants::browser_policy::UPDATE_KIND_GET,
            crate::constants::browser_policy::UPDATE_KIND_PREVIEW,
            crate::constants::browser_policy::UPDATE_KIND_PATCH,
            crate::constants::browser_policy::UPDATE_KIND_REPLACE,
            crate::constants::browser_policy::UPDATE_KIND_ROLLBACK,
        ];
        PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyUpdateStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyRejectionReason {
    #[serde(rename = "invalid-request")]
    InvalidRequest,
    #[serde(rename = "unknown-writes-to")]
    UnknownWritesTo,
    #[serde(rename = "unknown-field")]
    UnknownField,
    #[serde(rename = "invalid-enum-value")]
    InvalidEnumValue,
    #[serde(rename = "missing-budget-or-fallback")]
    MissingBudgetOrFallback,
    #[serde(rename = "missing-managed-proof-or-fallback")]
    MissingManagedProofOrFallback,
    #[serde(rename = "capability-unavailable")]
    CapabilityUnavailable,
    #[serde(rename = "storage-unavailable")]
    StorageUnavailable,
    #[serde(rename = "stale-revision")]
    StaleRevision,
    #[serde(rename = "scaffold-unavailable")]
    ScaffoldUnavailable,
    #[serde(rename = "revision-not-found")]
    RevisionNotFound,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BrowserPolicyUpdateRequest {
    Get(BrowserPolicyGetRequest),
    Preview(BrowserPolicyPreviewRequest),
    Patch(BrowserPolicyPatchRequest),
    Replace(BrowserPolicyReplaceRequest),
    Rollback(BrowserPolicyRollbackRequest),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BrowserPolicyGetRequest {
    pub schema_version: String,
    pub request_id: String,
    pub kind: BrowserPolicyUpdateKind,
    pub policy_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BrowserPolicyPreviewRequest {
    pub schema_version: String,
    pub request_id: String,
    pub kind: BrowserPolicyUpdateKind,
    pub policy: BrowserPolicyValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BrowserPolicyPatch {
    pub op: String,
    pub field_id: String,
    pub writes_to: String,
    pub value: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BrowserPolicyPatchRequest {
    pub schema_version: String,
    pub request_id: String,
    pub kind: BrowserPolicyUpdateKind,
    pub policy_id: String,
    pub base_revision_id: String,
    pub patches: Vec<BrowserPolicyPatch>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BrowserPolicyReplaceRequest {
    pub schema_version: String,
    pub request_id: String,
    pub kind: BrowserPolicyUpdateKind,
    pub base_revision_id: Option<String>,
    pub policy: BrowserPolicyValue,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BrowserPolicyRollbackRequest {
    pub schema_version: String,
    pub request_id: String,
    pub kind: BrowserPolicyUpdateKind,
    pub policy_id: String,
    pub target_revision_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyUpdateResponse {
    pub schema_version: String,
    pub request_id: String,
    pub kind: BrowserPolicyUpdateKind,
    pub status: BrowserPolicyUpdateStatus,
    pub policy: Option<BrowserPolicyValue>,
    pub effective_policy: Option<BrowserPolicyEffectivePolicy>,
    pub capability_registry: Option<BrowserPolicyCapabilityRegistry>,
    pub rejection_reason: Option<BrowserPolicyRejectionReason>,
    pub audit_event_id: Option<String>,
    pub message: Option<String>,
}
