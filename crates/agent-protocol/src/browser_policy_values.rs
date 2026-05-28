use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyDefaultPosture {
    #[serde(rename = "observe")]
    Observe,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "block")]
    Block,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyManagementMode {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "observe-only")]
    ObserveOnly,
    #[serde(rename = "managed-browser")]
    ManagedBrowser,
    #[serde(rename = "network-assisted")]
    NetworkAssisted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyManagedBrowserMode {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "preferred")]
    Preferred,
    #[serde(rename = "required-for-exact-rules")]
    RequiredForExactRules,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyUnmanagedBrowserMode {
    #[serde(rename = "observe-only")]
    ObserveOnly,
    #[serde(rename = "network-domain-only")]
    NetworkDomainOnly,
    #[serde(rename = "manual-review")]
    ManualReview,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyUrlTargetType {
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "url-prefix")]
    UrlPrefix,
    #[serde(rename = "exact-url")]
    ExactUrl,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyEvidenceProofLevel {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "network-domain")]
    NetworkDomain,
    #[serde(rename = "managed-active-tab")]
    ManagedActiveTab,
    #[serde(rename = "fresh-managed-active-tab")]
    FreshManagedActiveTab,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyProofFallback {
    #[serde(rename = "downgrade-to-domain")]
    DowngradeToDomain,
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "block-until-proof")]
    BlockUntilProof,
    #[serde(rename = "observe-only")]
    ObserveOnly,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyDownloadState {
    #[serde(rename = "not-configured")]
    NotConfigured,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "block")]
    Block,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyApprovalState {
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "required")]
    Required,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "approved")]
    Approved,
    #[serde(rename = "denied")]
    Denied,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyReportState {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "on-demand")]
    OnDemand,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyAuditState {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "local-only")]
    LocalOnly,
    #[serde(rename = "parent-visible")]
    ParentVisible,
    #[serde(rename = "retained")]
    Retained,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyRetentionState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "seven-days")]
    SevenDays,
    #[serde(rename = "thirty-days")]
    ThirtyDays,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyCapabilityState {
    #[serde(rename = "supported")]
    Supported,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[serde(rename = "unknown")]
    Unknown,
}
