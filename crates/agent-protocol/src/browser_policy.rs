use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        match self {
            Self::Get => crate::constants::browser_policy::UPDATE_KIND_GET,
            Self::Preview => crate::constants::browser_policy::UPDATE_KIND_PREVIEW,
            Self::Patch => crate::constants::browser_policy::UPDATE_KIND_PATCH,
            Self::Replace => crate::constants::browser_policy::UPDATE_KIND_REPLACE,
            Self::Rollback => crate::constants::browser_policy::UPDATE_KIND_ROLLBACK,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyUpdateStatus {
    #[serde(rename = "accepted")]
    Accepted,
    #[serde(rename = "rejected")]
    Rejected,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyRejectionReason {
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
    #[serde(rename = "scaffold-unavailable")]
    ScaffoldUnavailable,
    #[serde(rename = "revision-not-found")]
    RevisionNotFound,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyPatch {
    pub op: String,
    pub field_id: String,
    pub writes_to: String,
    pub value: serde_json::Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyPatchRequest {
    pub schema_version: String,
    pub request_id: String,
    pub kind: BrowserPolicyUpdateKind,
    pub policy_id: String,
    pub base_revision_id: String,
    pub patches: Vec<BrowserPolicyPatch>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyUpdateResponse {
    pub schema_version: String,
    pub request_id: String,
    pub kind: BrowserPolicyUpdateKind,
    pub status: BrowserPolicyUpdateStatus,
    pub policy: Option<serde_json::Value>,
    pub effective_policy: Option<serde_json::Value>,
    pub capability_registry: Option<serde_json::Value>,
    pub rejection_reason: Option<BrowserPolicyRejectionReason>,
    pub audit_event_id: Option<String>,
    pub message: Option<String>,
}
