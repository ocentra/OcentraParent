use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserInterventionDecisionSource {
    #[serde(rename = "parent-rule")]
    ParentRule,
    #[serde(rename = "parent-portal")]
    ParentPortal,
    #[serde(rename = "local-ai")]
    LocalAi,
    #[serde(rename = "system")]
    System,
    #[serde(rename = "manual-test")]
    ManualTest,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserInterventionDecisionSource {
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        constants::browser::INTERVENTION_DECISION_SOURCE_PARENT_RULE,
        constants::browser::INTERVENTION_DECISION_SOURCE_PARENT_PORTAL,
        constants::browser::INTERVENTION_DECISION_SOURCE_LOCAL_AI,
        constants::browser::INTERVENTION_DECISION_SOURCE_SYSTEM,
        constants::browser::INTERVENTION_DECISION_SOURCE_MANUAL_TEST,
        constants::browser::INTERVENTION_DECISION_SOURCE_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserInterventionAction {
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "block")]
    Block,
    #[serde(rename = "redirect")]
    Redirect,
    #[serde(rename = "time-limit")]
    TimeLimit,
    #[serde(rename = "parent-review")]
    AskParent,
    #[serde(rename = "approval-hold")]
    ApprovalHold,
    #[serde(rename = "checking-hold")]
    CheckingHold,
    #[serde(rename = "terminate-process")]
    TerminateProcess,
    #[serde(rename = "relaunch-managed")]
    RelaunchManaged,
    #[serde(rename = "monitor")]
    Monitor,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserInterventionAction {
    const PROTOCOL_STRINGS: [&'static str; 12] = [
        constants::browser::INTERVENTION_ACTION_ALLOW,
        constants::browser::INTERVENTION_ACTION_WARN,
        constants::browser::INTERVENTION_ACTION_BLOCK,
        constants::browser::INTERVENTION_ACTION_REDIRECT,
        constants::browser::INTERVENTION_ACTION_TIME_LIMIT,
        constants::browser::INTERVENTION_ACTION_ASK_PARENT,
        constants::browser::INTERVENTION_ACTION_APPROVAL_HOLD,
        constants::browser::INTERVENTION_ACTION_CHECKING_HOLD,
        constants::browser::INTERVENTION_ACTION_TERMINATE_PROCESS,
        constants::browser::INTERVENTION_ACTION_RELAUNCH_MANAGED,
        constants::browser::INTERVENTION_ACTION_MONITOR,
        constants::browser::INTERVENTION_ACTION_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserInterventionTargetType {
    #[serde(rename = "site")]
    Site,
    #[serde(rename = "domain")]
    Domain,
    #[serde(rename = "url")]
    Url,
    #[serde(rename = "video")]
    Video,
    #[serde(rename = "social-account-creation")]
    SocialAccountCreation,
    #[serde(rename = "social-feed")]
    SocialFeed,
    #[serde(rename = "social-short-video-feed")]
    SocialShortVideoFeed,
    #[serde(rename = "social-messaging")]
    SocialMessaging,
    #[serde(rename = "social-upload-post")]
    SocialUploadPost,
    #[serde(rename = "social-livestream")]
    SocialLivestream,
    #[serde(rename = "unknown-social-site")]
    UnknownSocialSite,
    #[serde(rename = "browser-game")]
    BrowserGame,
    #[serde(rename = "game-account")]
    GameAccount,
    #[serde(rename = "game-purchase")]
    GamePurchase,
    #[serde(rename = "cloud-gaming")]
    CloudGaming,
    #[serde(rename = "unknown-game")]
    UnknownGame,
    #[serde(rename = "unblocked-game-site")]
    UnblockedGameSite,
    #[serde(rename = "browser-process")]
    BrowserProcess,
    #[serde(rename = "browser-session")]
    BrowserSession,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserInterventionTargetType {
    const PROTOCOL_STRINGS: [&'static str; 20] = [
        constants::browser::INTERVENTION_TARGET_TYPE_SITE,
        constants::browser::INTERVENTION_TARGET_TYPE_DOMAIN,
        constants::browser::INTERVENTION_TARGET_TYPE_URL,
        constants::browser::INTERVENTION_TARGET_TYPE_VIDEO,
        constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_ACCOUNT_CREATION,
        constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_FEED,
        constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_SHORT_VIDEO_FEED,
        constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_MESSAGING,
        constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_UPLOAD_POST,
        constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_LIVESTREAM,
        constants::browser::INTERVENTION_TARGET_TYPE_UNKNOWN_SOCIAL_SITE,
        constants::browser::INTERVENTION_TARGET_TYPE_BROWSER_GAME,
        constants::browser::INTERVENTION_TARGET_TYPE_GAME_ACCOUNT,
        constants::browser::INTERVENTION_TARGET_TYPE_GAME_PURCHASE,
        constants::browser::INTERVENTION_TARGET_TYPE_CLOUD_GAMING,
        constants::browser::INTERVENTION_TARGET_TYPE_UNKNOWN_GAME,
        constants::browser::INTERVENTION_TARGET_TYPE_UNBLOCKED_GAME_SITE,
        constants::browser::INTERVENTION_TARGET_TYPE_BROWSER_PROCESS,
        constants::browser::INTERVENTION_TARGET_TYPE_BROWSER_SESSION,
        constants::browser::INTERVENTION_TARGET_TYPE_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserInterventionMechanism {
    #[serde(rename = "chromium-cdp-fetch")]
    ChromiumCdpFetch,
    #[serde(rename = "webdriver-bidi-network")]
    WebDriverBidiNetwork,
    #[serde(rename = "managed-extension")]
    ManagedExtension,
    #[serde(rename = "managed-block-page")]
    ManagedBlockPage,
    #[serde(rename = "approval-hold-page")]
    ApprovalHoldPage,
    #[serde(rename = "checking-hold-page")]
    CheckingHoldPage,
    #[serde(rename = "os-app-control")]
    OsAppControl,
    #[serde(rename = "owned-webview")]
    OwnedWebView,
    #[serde(rename = "monitor-only")]
    MonitorOnly,
    #[serde(rename = "none")]
    None,
}

impl BrowserInterventionMechanism {
    const PROTOCOL_STRINGS: [&'static str; 10] = [
        constants::browser::INTERVENTION_MECHANISM_CHROMIUM_CDP_FETCH,
        constants::browser::INTERVENTION_MECHANISM_WEBDRIVER_BIDI_NETWORK,
        constants::browser::INTERVENTION_MECHANISM_MANAGED_EXTENSION,
        constants::browser::INTERVENTION_MECHANISM_MANAGED_BLOCK_PAGE,
        constants::browser::INTERVENTION_MECHANISM_APPROVAL_HOLD_PAGE,
        constants::browser::INTERVENTION_MECHANISM_CHECKING_HOLD_PAGE,
        constants::browser::INTERVENTION_MECHANISM_OS_APP_CONTROL,
        constants::browser::INTERVENTION_MECHANISM_OWNED_WEBVIEW,
        constants::browser::INTERVENTION_MECHANISM_MONITOR_ONLY,
        constants::browser::INTERVENTION_MECHANISM_NONE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserInterventionOutcome {
    #[serde(rename = "applied")]
    Applied,
    #[serde(rename = "allowed")]
    Allowed,
    #[serde(rename = "warned")]
    Warned,
    #[serde(rename = "blocked")]
    Blocked,
    #[serde(rename = "redirected")]
    Redirected,
    #[serde(rename = "approval-required")]
    ApprovalRequired,
    #[serde(rename = "held")]
    Held,
    #[serde(rename = "terminated")]
    Terminated,
    #[serde(rename = "relaunch-started")]
    RelaunchStarted,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "monitor-only")]
    MonitorOnly,
}

impl BrowserInterventionOutcome {
    const PROTOCOL_STRINGS: [&'static str; 13] = [
        constants::browser::INTERVENTION_OUTCOME_APPLIED,
        constants::browser::INTERVENTION_OUTCOME_ALLOWED,
        constants::browser::INTERVENTION_OUTCOME_WARNED,
        constants::browser::INTERVENTION_OUTCOME_BLOCKED,
        constants::browser::INTERVENTION_OUTCOME_REDIRECTED,
        constants::browser::INTERVENTION_OUTCOME_APPROVAL_REQUIRED,
        constants::browser::INTERVENTION_OUTCOME_HELD,
        constants::browser::INTERVENTION_OUTCOME_TERMINATED,
        constants::browser::INTERVENTION_OUTCOME_RELAUNCH_STARTED,
        constants::browser::INTERVENTION_OUTCOME_MANUAL_REQUIRED,
        constants::browser::INTERVENTION_OUTCOME_FAILED,
        constants::browser::INTERVENTION_OUTCOME_UNSUPPORTED,
        constants::browser::INTERVENTION_OUTCOME_MONITOR_ONLY,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserInterventionCapabilityState {
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "needs-managed-session")]
    NeedsManagedSession,
    #[serde(rename = "needs-managed-extension")]
    NeedsManagedExtension,
    #[serde(rename = "needs-os-app-control")]
    NeedsOsAppControl,
    #[serde(rename = "unsupported-browser")]
    UnsupportedBrowser,
    #[serde(rename = "disabled-by-parent")]
    DisabledByParent,
    #[serde(rename = "adapter-error")]
    AdapterError,
}

impl BrowserInterventionCapabilityState {
    const PROTOCOL_STRINGS: [&'static str; 7] = [
        constants::browser::INTERVENTION_CAPABILITY_READY,
        constants::browser::INTERVENTION_CAPABILITY_NEEDS_MANAGED_SESSION,
        constants::browser::INTERVENTION_CAPABILITY_NEEDS_MANAGED_EXTENSION,
        constants::browser::INTERVENTION_CAPABILITY_NEEDS_OS_APP_CONTROL,
        constants::browser::INTERVENTION_CAPABILITY_UNSUPPORTED_BROWSER,
        constants::browser::INTERVENTION_CAPABILITY_DISABLED_BY_PARENT,
        constants::browser::INTERVENTION_CAPABILITY_ADAPTER_ERROR,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserUnmanagedFallbackActionState {
    #[serde(rename = "report-only")]
    ReportOnly,
    #[serde(rename = "warn-child")]
    WarnChild,
    #[serde(rename = "parent-review")]
    AskParent,
    #[serde(rename = "terminate-process")]
    TerminateProcess,
    #[serde(rename = "relaunch-managed-browser")]
    RelaunchManagedBrowser,
    #[serde(rename = "os-block-configured")]
    OsBlockConfigured,
    #[serde(rename = "os-block-manual-required")]
    OsBlockManualRequired,
    #[serde(rename = "allowed-unmanaged-exception")]
    AllowedUnmanagedException,
    #[serde(rename = "degraded")]
    Degraded,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl Default for BrowserUnmanagedFallbackActionState {
    fn default() -> Self {
        Self::Unavailable
    }
}

impl BrowserUnmanagedFallbackActionState {
    const PROTOCOL_STRINGS: [&'static str; 10] = [
        constants::browser::UNMANAGED_FALLBACK_ACTION_REPORT_ONLY,
        constants::browser::UNMANAGED_FALLBACK_ACTION_WARN_CHILD,
        constants::browser::UNMANAGED_FALLBACK_ACTION_ASK_PARENT,
        constants::browser::UNMANAGED_FALLBACK_ACTION_TERMINATE_PROCESS,
        constants::browser::UNMANAGED_FALLBACK_ACTION_RELAUNCH_MANAGED_BROWSER,
        constants::browser::UNMANAGED_FALLBACK_ACTION_OS_BLOCK_CONFIGURED,
        constants::browser::UNMANAGED_FALLBACK_ACTION_OS_BLOCK_MANUAL_REQUIRED,
        constants::browser::UNMANAGED_FALLBACK_ACTION_ALLOWED_UNMANAGED_EXCEPTION,
        constants::browser::UNMANAGED_FALLBACK_ACTION_DEGRADED,
        constants::browser::UNMANAGED_FALLBACK_ACTION_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserBoundaryState {
    #[serde(rename = "managed-session")]
    ManagedSession,
    #[serde(rename = "unmanaged-browser-process")]
    UnmanagedBrowserProcess,
    #[serde(rename = "browser-like-process")]
    BrowserLikeProcess,
    #[serde(rename = "unsupported")]
    Unsupported,
    #[serde(rename = "unknown")]
    Unknown,
}

impl BrowserBoundaryState {
    const PROTOCOL_STRINGS: [&'static str; 5] = [
        constants::browser::INTERVENTION_BOUNDARY_MANAGED_SESSION,
        constants::browser::INTERVENTION_BOUNDARY_UNMANAGED_BROWSER_PROCESS,
        constants::browser::INTERVENTION_BOUNDARY_BROWSER_LIKE_PROCESS,
        constants::browser::INTERVENTION_BOUNDARY_UNSUPPORTED,
        constants::browser::INTERVENTION_BOUNDARY_UNKNOWN,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserExactUrlClaimState {
    #[serde(rename = "exact-url-proven")]
    ExactUrlProven,
    #[serde(rename = "not-claimed")]
    NotClaimed,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl BrowserExactUrlClaimState {
    const PROTOCOL_STRINGS: [&'static str; 3] = [
        constants::browser::INTERVENTION_EXACT_URL_PROVEN,
        constants::browser::INTERVENTION_EXACT_URL_NOT_CLAIMED,
        constants::browser::INTERVENTION_EXACT_URL_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserUnmanagedDetectionState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "detected")]
    Detected,
    #[serde(rename = "warned")]
    Warned,
    #[serde(rename = "terminated")]
    Terminated,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserInterventionDeliveryState {
    #[serde(rename = "not-delivered")]
    NotDelivered,
    #[serde(rename = "warn-page-rendered")]
    WarnPageRendered,
    #[serde(rename = "block-page-rendered")]
    BlockPageRendered,
    #[serde(rename = "approval-hold-rendered")]
    ApprovalHoldRendered,
    #[serde(rename = "checking-hold-rendered")]
    CheckingHoldRendered,
    #[serde(rename = "portal-row-only")]
    PortalRowOnly,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

impl Default for BrowserInterventionDeliveryState {
    fn default() -> Self {
        Self::NotDelivered
    }
}

impl BrowserInterventionDeliveryState {
    const PROTOCOL_STRINGS: [&'static str; 7] = [
        constants::browser::INTERVENTION_DELIVERY_NOT_DELIVERED,
        constants::browser::INTERVENTION_DELIVERY_WARN_PAGE_RENDERED,
        constants::browser::INTERVENTION_DELIVERY_BLOCK_PAGE_RENDERED,
        constants::browser::INTERVENTION_DELIVERY_APPROVAL_HOLD_RENDERED,
        constants::browser::INTERVENTION_DELIVERY_CHECKING_HOLD_RENDERED,
        constants::browser::INTERVENTION_DELIVERY_PORTAL_ROW_ONLY,
        constants::browser::INTERVENTION_DELIVERY_MANUAL_REQUIRED,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}

impl BrowserUnmanagedDetectionState {
    const PROTOCOL_STRINGS: [&'static str; 6] = [
        constants::browser::INTERVENTION_UNMANAGED_DETECTION_NONE,
        constants::browser::INTERVENTION_UNMANAGED_DETECTION_DETECTED,
        constants::browser::INTERVENTION_UNMANAGED_DETECTION_WARNED,
        constants::browser::INTERVENTION_UNMANAGED_DETECTION_TERMINATED,
        constants::browser::INTERVENTION_UNMANAGED_DETECTION_MANUAL_REQUIRED,
        constants::browser::INTERVENTION_UNMANAGED_DETECTION_UNAVAILABLE,
    ];

    pub fn as_protocol_str(&self) -> &'static str {
        Self::PROTOCOL_STRINGS[*self as usize]
    }
}
