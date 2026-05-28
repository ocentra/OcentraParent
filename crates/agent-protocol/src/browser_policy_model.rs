use serde::{Deserialize, Serialize};

use crate::browser_policy_values::{
    BrowserPolicyApprovalState, BrowserPolicyAuditState, BrowserPolicyCapabilityState,
    BrowserPolicyDefaultPosture, BrowserPolicyDownloadState, BrowserPolicyEvidenceProofLevel,
    BrowserPolicyManagedBrowserMode, BrowserPolicyManagementMode, BrowserPolicyProofFallback,
    BrowserPolicyReportState, BrowserPolicyRetentionState, BrowserPolicyUnmanagedBrowserMode,
    BrowserPolicyUrlTargetType,
};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyValue {
    pub schema_version: String,
    pub policy_id: String,
    pub enabled: bool,
    pub default_posture: BrowserPolicyDefaultPosture,
    pub fallback_posture: Option<BrowserPolicyDefaultPosture>,
    pub management_mode: BrowserPolicyManagementMode,
    pub managed_browser: BrowserPolicyManagedBrowser,
    pub unmanaged_browser: BrowserPolicyUnmanagedBrowser,
    pub evidence: BrowserPolicyEvidenceRequirement,
    pub rules: BrowserPolicyRules,
    pub budgets: BrowserPolicyBudgets,
    pub downloads: BrowserPolicyDownloads,
    pub approvals: BrowserPolicyApprovals,
    pub reports: BrowserPolicyReports,
    pub audit: BrowserPolicyAudit,
    pub retention: BrowserPolicyRetention,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyManagedBrowser {
    pub mode: BrowserPolicyManagedBrowserMode,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyUnmanagedBrowser {
    pub mode: BrowserPolicyUnmanagedBrowserMode,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyEvidenceRequirement {
    pub required_proof: BrowserPolicyEvidenceProofLevel,
    pub proof_fallback: Option<BrowserPolicyProofFallback>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyRules {
    pub allowed_target_types: Vec<BrowserPolicyUrlTargetType>,
    pub entries: Vec<BrowserPolicyRule>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyRule {
    pub rule_id: String,
    pub target_type: BrowserPolicyUrlTargetType,
    pub target_value: String,
    pub enabled: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyBudgets {
    pub default_daily_minutes: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyDownloads {
    pub state: BrowserPolicyDownloadState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyApprovals {
    pub state: BrowserPolicyApprovalState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyReports {
    pub state: BrowserPolicyReportState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyAudit {
    pub state: BrowserPolicyAuditState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyRetention {
    pub state: BrowserPolicyRetentionState,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyEffectivePolicy {
    pub schema_version: String,
    pub policy_id: String,
    pub revision_id: String,
    pub compiled_hash: String,
    pub compiled_at: String,
    pub default_posture: BrowserPolicyDefaultPosture,
    pub fallback_posture: Option<BrowserPolicyDefaultPosture>,
    pub budgets: BrowserPolicyBudgets,
    pub rules: Vec<BrowserPolicyEffectiveRule>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyEffectiveRule {
    pub rule_id: String,
    pub target_type: BrowserPolicyUrlTargetType,
    pub target_value: String,
    pub default_posture: BrowserPolicyDefaultPosture,
    pub evidence: BrowserPolicyEvidenceRequirement,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyCapabilityRegistry {
    pub schema_version: String,
    pub generated_at: String,
    pub capabilities: Vec<BrowserPolicyCapability>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyCapability {
    pub capability_id: String,
    pub state: BrowserPolicyCapabilityState,
    pub label: String,
    pub affected_writes_to: Vec<String>,
    pub checked_at: String,
    pub reason: Option<String>,
}
