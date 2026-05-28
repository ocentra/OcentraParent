use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyDefaultPosture {
    #[serde(rename = "observe")]
    Observe,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "ask")]
    Ask,
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
    #[serde(rename = "local-child-agent")]
    LocalChildAgent,
    #[serde(rename = "lan-live")]
    LanLive,
    #[serde(rename = "authoring-only")]
    AuthoringOnly,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyManagedBrowserMode {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "not-required")]
    NotRequired,
    #[serde(rename = "preferred")]
    Preferred,
    #[serde(rename = "available-for-exact-rules")]
    AvailableForExactRules,
    #[serde(rename = "required-for-exact-rules")]
    RequiredForExactRules,
    #[serde(rename = "required-for-all-browsing")]
    RequiredForAllBrowsing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyUnmanagedBrowserMode {
    #[serde(rename = "observe-only")]
    ObserveOnly,
    #[serde(rename = "network-domain-only")]
    NetworkDomainOnly,
    #[serde(rename = "manual-review")]
    ManualReview,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "monitor")]
    Monitor,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "ask")]
    Ask,
    #[serde(rename = "relaunch-managed")]
    RelaunchManaged,
    #[serde(rename = "block")]
    Block,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyUrlTargetType {
    #[default]
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "url-prefix")]
    UrlPrefix,
    #[serde(rename = "exact-url")]
    ExactUrl,
    #[serde(rename = "domain-origin")]
    DomainOrigin,
    #[serde(rename = "site-category")]
    SiteCategory,
    #[serde(rename = "search-terms")]
    SearchTerms,
    #[serde(rename = "video-channel")]
    VideoChannel,
    #[serde(rename = "browser-session")]
    BrowserSession,
    #[serde(rename = "browser-process")]
    BrowserProcess,
    #[serde(rename = "capability-state")]
    CapabilityState,
    #[serde(rename = "download")]
    Download,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyEvidenceProofLevel {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "process-running")]
    ProcessRunning,
    #[serde(rename = "foreground-window")]
    ForegroundWindow,
    #[serde(rename = "network-domain")]
    NetworkDomain,
    #[serde(rename = "managed-tab-list")]
    ManagedTabList,
    #[serde(rename = "managed-active-tab")]
    ManagedActiveTab,
    #[serde(rename = "fresh-managed-tab-list")]
    FreshManagedTabList,
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
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "observe")]
    Observe,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "ask")]
    Ask,
    #[serde(rename = "block-until-ready")]
    BlockUntilReady,
    #[serde(rename = "mark-unavailable")]
    MarkUnavailable,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyDownloadState {
    #[default]
    #[serde(rename = "not-configured")]
    NotConfigured,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "observe")]
    Observe,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "ask")]
    Ask,
    #[serde(rename = "ask-parent")]
    AskParent,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "block-risky")]
    BlockRisky,
    #[serde(rename = "block-all")]
    BlockAll,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyApprovalState {
    #[default]
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyReportState {
    #[default]
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "daily")]
    Daily,
    #[serde(rename = "weekly")]
    Weekly,
    #[serde(rename = "on-demand")]
    OnDemand,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyAuditState {
    #[default]
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "local-only")]
    LocalOnly,
    #[serde(rename = "parent-visible")]
    ParentVisible,
    #[serde(rename = "retained")]
    Retained,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyRetentionState {
    #[default]
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
