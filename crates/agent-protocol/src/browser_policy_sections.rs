use serde::{Deserialize, Serialize};

use crate::{
    BrowserPolicyApprovalRequiredFor, BrowserPolicyAuditRequiredField,
    BrowserPolicyCustodyAllowedUse, BrowserPolicyRuleAction, BrowserPolicyUrlTargetType,
};

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyRuleTarget {
    pub kind: BrowserPolicyUrlTargetType,
    #[serde(default)]
    pub values: Vec<String>,
    pub match_mode: String,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyRuleActionPlan {
    pub kind: BrowserPolicyRuleAction,
    #[serde(default)]
    pub budget_id: Option<String>,
    #[serde(default)]
    pub approval_kind: Option<BrowserPolicyApprovalRequiredFor>,
    #[serde(default)]
    pub reason_code: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicySchedule {
    pub schedule_id: String,
    pub kind: String,
    #[serde(default)]
    pub timezone: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyChildFacing {
    #[serde(default)]
    pub show_warn_text: bool,
    #[serde(default)]
    pub show_block_reason: bool,
    #[serde(default)]
    pub show_ask_parent_state: bool,
    #[serde(default)]
    pub show_time_left: bool,
    #[serde(default)]
    pub show_use_managed_browser_action: bool,
    #[serde(default)]
    pub hide_parent_diagnostics: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyPortalAi {
    #[serde(default)]
    pub allow_summaries: bool,
    #[serde(default)]
    pub allow_policy_explanation: bool,
    #[serde(default)]
    pub allow_rule_suggestions: bool,
    #[serde(default)]
    pub allow_evidence_refs: bool,
    #[serde(default)]
    pub allow_raw_content: bool,
    #[serde(default)]
    pub requires_manual_review: bool,
    #[serde(default)]
    pub fallback_when_unavailable: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyPlatformCapability {
    #[serde(default)]
    pub enabled: bool,
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default)]
    pub allowed_adapters: Vec<String>,
    #[serde(default)]
    pub manual_required_adapters: Vec<String>,
    #[serde(default)]
    pub authoring_only: bool,
    #[serde(default)]
    pub may_run_capture: bool,
    #[serde(default)]
    pub may_connect_to_browser_bridge: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyPlatforms {
    #[serde(default)]
    pub windows: BrowserPolicyPlatformCapability,
    #[serde(default)]
    pub macos: BrowserPolicyPlatformCapability,
    #[serde(default)]
    pub linux: BrowserPolicyPlatformCapability,
    #[serde(default)]
    pub android: BrowserPolicyPlatformCapability,
    #[serde(default)]
    pub ios: BrowserPolicyPlatformCapability,
    #[serde(default)]
    pub web_portal: BrowserPolicyPlatformCapability,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyFallbacks {
    #[serde(default)]
    pub managed_profile_missing: Option<String>,
    #[serde(default)]
    pub bridge_missing: Option<String>,
    #[serde(default)]
    pub extension_disabled: Option<String>,
    #[serde(default)]
    pub native_host_missing: Option<String>,
    #[serde(default)]
    pub unsupported_browser: Option<String>,
    #[serde(default)]
    pub stale_evidence: Option<String>,
    #[serde(default)]
    pub network_adapter_unavailable: Option<String>,
    #[serde(default)]
    pub process_control_unavailable: Option<String>,
    #[serde(default)]
    pub enforcement_failure: Option<String>,
    #[serde(default)]
    pub child_device_offline: Option<String>,
    #[serde(default)]
    pub platform_unsupported: Option<String>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyCustody {
    #[serde(default)]
    pub allowed_uses: Vec<BrowserPolicyCustodyAllowedUse>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserPolicyAuditPlan {
    #[serde(default)]
    pub required_fields: Vec<BrowserPolicyAuditRequiredField>,
}
