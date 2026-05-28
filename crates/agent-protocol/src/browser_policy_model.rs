use serde::{Deserialize, Serialize};

use crate::browser_policy_values::{
    BrowserPolicyApprovalState, BrowserPolicyAuditState, BrowserPolicyCapabilityState,
    BrowserPolicyDefaultPosture, BrowserPolicyDownloadState, BrowserPolicyEvidenceProofLevel,
    BrowserPolicyManagedBrowserMode, BrowserPolicyManagementMode, BrowserPolicyProofFallback,
    BrowserPolicyReportState, BrowserPolicyRetentionState, BrowserPolicyUnmanagedBrowserMode,
    BrowserPolicyUrlTargetType,
};
use crate::{
    BrowserPolicyApprovalRequiredFor, BrowserPolicyApprovalUnansweredDefault,
    BrowserPolicyAuditPlan, BrowserPolicyAuditRequiredField, BrowserPolicyBudgetCountingMode,
    BrowserPolicyChildFacing, BrowserPolicyCustody, BrowserPolicyDownloadBlockedType,
    BrowserPolicyEvidenceNeverCollect, BrowserPolicyEvidenceUrlScope, BrowserPolicyFallbacks,
    BrowserPolicyManagedBrowserBridgeRequirement, BrowserPolicyManagedBrowserFamily,
    BrowserPolicyManagedBrowserIntegrationMechanism, BrowserPolicyManagedBrowserLaunchMode,
    BrowserPolicyManagedBrowserProfileMode, BrowserPolicyPlatforms, BrowserPolicyPortalAi,
    BrowserPolicyReportVisibleField, BrowserPolicyRetentionExactUrl, BrowserPolicyRuleAction,
    BrowserPolicyRuleActionPlan, BrowserPolicyRuleTarget, BrowserPolicySchedule,
    BrowserPolicyUnmanagedBrowserClassificationTarget,
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
    #[serde(default)]
    pub custody: BrowserPolicyCustody,
    #[serde(default)]
    pub schedules: Vec<BrowserPolicySchedule>,
    #[serde(default)]
    pub child_facing: BrowserPolicyChildFacing,
    #[serde(default)]
    pub portal_ai: BrowserPolicyPortalAi,
    #[serde(default)]
    pub platforms: BrowserPolicyPlatforms,
    #[serde(default)]
    pub fallbacks: BrowserPolicyFallbacks,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyManagedBrowser {
    pub mode: BrowserPolicyManagedBrowserMode,
    #[serde(default)]
    pub allowed_families: Vec<BrowserPolicyManagedBrowserFamily>,
    #[serde(default)]
    pub launch_mode: BrowserPolicyManagedBrowserLaunchMode,
    #[serde(default)]
    pub profile_mode: BrowserPolicyManagedBrowserProfileMode,
    #[serde(default)]
    pub bridge_requirements: Vec<BrowserPolicyManagedBrowserBridgeRequirement>,
    #[serde(default)]
    pub integration_mechanisms: Vec<BrowserPolicyManagedBrowserIntegrationMechanism>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyUnmanagedBrowser {
    pub mode: BrowserPolicyUnmanagedBrowserMode,
    #[serde(default)]
    pub grace_seconds: u32,
    #[serde(default)]
    pub allow_recover_launch_url: bool,
    #[serde(default)]
    pub classification_targets: Vec<BrowserPolicyUnmanagedBrowserClassificationTarget>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyEvidenceRequirement {
    #[serde(default)]
    pub url_scope: BrowserPolicyEvidenceUrlScope,
    pub required_proof: BrowserPolicyEvidenceProofLevel,
    pub proof_fallback: Option<BrowserPolicyProofFallback>,
    #[serde(default = "default_when_proof_unavailable")]
    pub when_proof_unavailable: BrowserPolicyProofFallback,
    #[serde(default)]
    pub never_collect: Vec<BrowserPolicyEvidenceNeverCollect>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyRules {
    pub allowed_target_types: Vec<BrowserPolicyUrlTargetType>,
    #[serde(default)]
    pub allowed_actions: Vec<BrowserPolicyRuleAction>,
    #[serde(default)]
    pub items: Vec<BrowserPolicyRule>,
    #[serde(default)]
    pub entries: Vec<BrowserPolicyRule>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyRule {
    pub rule_id: String,
    #[serde(default)]
    pub target_type: Option<BrowserPolicyUrlTargetType>,
    #[serde(default)]
    pub target_value: Option<String>,
    pub enabled: bool,
    #[serde(default)]
    pub priority: Option<u32>,
    #[serde(default)]
    pub target: Option<BrowserPolicyRuleTarget>,
    #[serde(default)]
    pub action: Option<BrowserPolicyRuleActionPlan>,
    #[serde(default)]
    pub proof_requirement: Option<String>,
    #[serde(default)]
    pub schedule_id: Option<String>,
    #[serde(default)]
    pub budget_id: Option<String>,
    #[serde(default)]
    pub audit_level: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyBudgets {
    #[serde(default = "default_budget_enabled")]
    pub enabled: bool,
    pub default_daily_minutes: Option<u32>,
    #[serde(default)]
    pub counting_mode: BrowserPolicyBudgetCountingMode,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyDownloads {
    #[serde(default)]
    pub mode: BrowserPolicyDownloadState,
    #[serde(default)]
    pub blocked_types: Vec<BrowserPolicyDownloadBlockedType>,
    #[serde(default)]
    pub state: BrowserPolicyDownloadState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyApprovals {
    #[serde(default)]
    pub required_for: Vec<BrowserPolicyApprovalRequiredFor>,
    #[serde(default)]
    pub unanswered_default: BrowserPolicyApprovalUnansweredDefault,
    #[serde(default)]
    pub state: BrowserPolicyApprovalState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyReports {
    #[serde(default)]
    pub visible_fields: Vec<BrowserPolicyReportVisibleField>,
    #[serde(default)]
    pub state: BrowserPolicyReportState,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyAudit {
    #[serde(default)]
    pub required_fields: Vec<BrowserPolicyAuditRequiredField>,
    #[serde(default)]
    pub state: BrowserPolicyAuditState,
    #[serde(default)]
    pub plan: BrowserPolicyAuditPlan,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyRetention {
    #[serde(default)]
    pub exact_url: BrowserPolicyRetentionExactUrl,
    #[serde(default)]
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

fn default_budget_enabled() -> bool {
    true
}

fn default_when_proof_unavailable() -> BrowserPolicyProofFallback {
    BrowserPolicyProofFallback::MarkUnavailable
}
