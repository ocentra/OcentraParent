use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyManagedBrowserLaunchMode {
    #[default]
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "ocentra-launcher")]
    OcentraLauncher,
    #[serde(rename = "default-browser-route")]
    DefaultBrowserRoute,
    #[serde(rename = "managed-shell")]
    ManagedShell,
    #[serde(rename = "admin-provisioned")]
    AdminProvisioned,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyManagedBrowserProfileMode {
    #[default]
    #[serde(rename = "persistent-managed-profile")]
    PersistentManagedProfile,
    #[serde(rename = "clear-on-schedule")]
    ClearOnSchedule,
    #[serde(rename = "clear-on-session-end")]
    ClearOnSessionEnd,
    #[serde(rename = "ephemeral")]
    Ephemeral,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyEvidenceUrlScope {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "domain-only")]
    DomainOnly,
    #[serde(rename = "domain-origin-title")]
    DomainOriginTitle,
    #[serde(rename = "full-url-without-query")]
    FullUrlWithoutQuery,
    #[serde(rename = "full-url-with-query")]
    FullUrlWithQuery,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyBudgetCountingMode {
    #[default]
    #[serde(rename = "foreground-browser-time")]
    ForegroundBrowserTime,
    #[serde(rename = "managed-active-tab-time")]
    ManagedActiveTabTime,
    #[serde(rename = "managed-session-time")]
    ManagedSessionTime,
    #[serde(rename = "all-browser-process-time")]
    AllBrowserProcessTime,
    #[serde(rename = "unmanaged-as-unknown-web-time")]
    UnmanagedAsUnknownWebTime,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyApprovalUnansweredDefault {
    #[default]
    #[serde(rename = "deny")]
    Deny,
    #[serde(rename = "allow-temporarily")]
    AllowTemporarily,
    #[serde(rename = "continue-observe-only")]
    ContinueObserveOnly,
    #[serde(rename = "keep-waiting")]
    KeepWaiting,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyRetentionExactUrl {
    #[default]
    #[serde(rename = "fresh-only")]
    FreshOnly,
    #[serde(rename = "24-hours")]
    TwentyFourHours,
    #[serde(rename = "7-days")]
    SevenDays,
    #[serde(rename = "30-days")]
    ThirtyDays,
    #[serde(rename = "until-reset")]
    UntilReset,
    #[serde(rename = "delete-expired")]
    DeleteExpired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyManagedBrowserFamily {
    #[serde(rename = "edge-stable")]
    EdgeStable,
    #[serde(rename = "edge-beta")]
    EdgeBeta,
    #[serde(rename = "edge-dev")]
    EdgeDev,
    #[serde(rename = "chrome-stable")]
    ChromeStable,
    #[serde(rename = "chrome-beta")]
    ChromeBeta,
    #[serde(rename = "chrome-dev")]
    ChromeDev,
    #[serde(rename = "chrome-for-testing")]
    ChromeForTesting,
    #[serde(rename = "brave")]
    Brave,
    #[serde(rename = "firefox")]
    Firefox,
    #[serde(rename = "safari-webkit")]
    SafariWebkit,
    #[serde(rename = "owned-webview")]
    OwnedWebview,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyManagedBrowserBridgeRequirement {
    #[serde(rename = "owned-profile")]
    OwnedProfile,
    #[serde(rename = "loopback-only")]
    LoopbackOnly,
    #[serde(rename = "random-port")]
    RandomPort,
    #[serde(rename = "reject-default-profile")]
    RejectDefaultProfile,
    #[serde(rename = "reject-unmanaged-profile")]
    RejectUnmanagedProfile,
    #[serde(rename = "redacted-refs")]
    RedactedRefs,
    #[serde(rename = "close-on-session-end")]
    CloseOnSessionEnd,
    #[serde(rename = "degrade-safely")]
    DegradeSafely,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyManagedBrowserIntegrationMechanism {
    #[serde(rename = "chromium-cdp")]
    ChromiumCdp,
    #[serde(rename = "webdriver-bidi")]
    WebdriverBidi,
    #[serde(rename = "managed-extension-native-host")]
    ManagedExtensionNativeHost,
    #[serde(rename = "browser-policy")]
    BrowserPolicy,
    #[serde(rename = "owned-webview")]
    OwnedWebview,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyManagedPolicyWriterControl {
    #[serde(rename = "disable-incognito")]
    DisableIncognito,
    #[serde(rename = "disable-guest-browsing")]
    DisableGuestBrowsing,
    #[serde(rename = "disable-profile-adding")]
    DisableProfileAdding,
    #[serde(rename = "limit-history-deletion")]
    LimitHistoryDeletion,
    #[serde(rename = "force-safe-search")]
    ForceSafeSearch,
    #[serde(rename = "force-restricted-mode")]
    ForceRestrictedMode,
    #[serde(rename = "url-allow-list")]
    UrlAllowList,
    #[serde(rename = "url-block-list")]
    UrlBlockList,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyManagedPolicyWriterFallback {
    #[serde(rename = "observe-only")]
    ObserveOnly,
    #[default]
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "not-claimed")]
    NotClaimed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyUnmanagedBrowserClassificationTarget {
    #[serde(rename = "known-browser")]
    KnownBrowser,
    #[serde(rename = "portable-browser")]
    PortableBrowser,
    #[serde(rename = "renamed-browser")]
    RenamedBrowser,
    #[serde(rename = "browser-like-process")]
    BrowserLikeProcess,
    #[serde(rename = "embedded-webview")]
    EmbeddedWebview,
    #[serde(rename = "private-or-tor")]
    PrivateOrTor,
    #[serde(rename = "unknown")]
    Unknown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyEvidenceNeverCollect {
    #[serde(rename = "page-body")]
    PageBody,
    #[serde(rename = "chat-content")]
    ChatContent,
    #[serde(rename = "screenshots")]
    Screenshots,
    #[serde(rename = "keystrokes")]
    Keystrokes,
    #[serde(rename = "form-values")]
    FormValues,
    #[serde(rename = "secrets")]
    Secrets,
    #[serde(rename = "decrypted-https-payload")]
    DecryptedHttpsPayload,
    #[serde(rename = "raw-protocol-dumps")]
    RawProtocolDumps,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyRuleAction {
    #[serde(rename = "allow")]
    Allow,
    #[default]
    #[serde(rename = "monitor")]
    Monitor,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "ask")]
    Ask,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "close-tab")]
    CloseTab,
    #[serde(rename = "close-browser")]
    CloseBrowser,
    #[serde(rename = "terminate-process")]
    TerminateProcess,
    #[serde(rename = "relaunch-managed")]
    RelaunchManaged,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyBrowserGamePolicyMode {
    #[serde(rename = "allow")]
    Allow,
    #[default]
    #[serde(rename = "observe")]
    Observe,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "parent-review")]
    AskParent,
    #[serde(rename = "limit")]
    Limit,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyBrowserGameApprovalMode {
    #[serde(rename = "allow")]
    Allow,
    #[default]
    #[serde(rename = "parent-review")]
    AskParent,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyDownloadBlockedType {
    #[serde(rename = "executable")]
    Executable,
    #[serde(rename = "script")]
    Script,
    #[serde(rename = "archive")]
    Archive,
    #[serde(rename = "media")]
    Media,
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "large-file")]
    LargeFile,
    #[serde(rename = "browser-danger")]
    BrowserDanger,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyApprovalRequiredFor {
    #[serde(rename = "blocked-site")]
    BlockedSite,
    #[serde(rename = "new-domain")]
    NewDomain,
    #[serde(rename = "unknown-category")]
    UnknownCategory,
    #[serde(rename = "unmanaged-browser")]
    UnmanagedBrowser,
    #[serde(rename = "download")]
    Download,
    #[serde(rename = "time-extension")]
    TimeExtension,
    #[serde(rename = "managed-setup")]
    ManagedSetup,
    #[serde(rename = "new-browser-install")]
    NewBrowserInstall,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyReportVisibleField {
    #[serde(rename = "managed-status")]
    ManagedStatus,
    #[serde(rename = "recent-url")]
    RecentUrl,
    #[serde(rename = "recent-domain-title")]
    RecentDomainTitle,
    #[serde(rename = "unmanaged-use")]
    UnmanagedUse,
    #[serde(rename = "policy-decisions")]
    PolicyDecisions,
    #[serde(rename = "block-results")]
    BlockResults,
    #[serde(rename = "time-budget")]
    TimeBudget,
    #[serde(rename = "download-events")]
    DownloadEvents,
    #[serde(rename = "source-capability")]
    SourceCapability,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyCustodyAllowedUse {
    #[serde(rename = "child-local")]
    ChildLocal,
    #[serde(rename = "lan-live")]
    LanLive,
    #[serde(rename = "parent-cache")]
    ParentCache,
    #[serde(rename = "parent-export")]
    ParentExport,
    #[serde(rename = "parent-report")]
    ParentReport,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyAuditRequiredField {
    #[serde(rename = "policy-decision")]
    PolicyDecision,
    #[serde(rename = "evidence-ref")]
    EvidenceRef,
    #[serde(rename = "ai-ref")]
    AiRef,
    #[serde(rename = "adapter-result")]
    AdapterResult,
    #[serde(rename = "timer-state")]
    TimerState,
    #[serde(rename = "parent-override")]
    ParentOverride,
    #[serde(rename = "rollback")]
    Rollback,
    #[serde(rename = "policy-version")]
    PolicyVersion,
    #[serde(rename = "capability-state")]
    CapabilityState,
    #[serde(rename = "custody-label")]
    CustodyLabel,
}
