use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ParentRule => constants::browser::INTERVENTION_DECISION_SOURCE_PARENT_RULE,
            Self::ParentPortal => constants::browser::INTERVENTION_DECISION_SOURCE_PARENT_PORTAL,
            Self::LocalAi => constants::browser::INTERVENTION_DECISION_SOURCE_LOCAL_AI,
            Self::System => constants::browser::INTERVENTION_DECISION_SOURCE_SYSTEM,
            Self::ManualTest => constants::browser::INTERVENTION_DECISION_SOURCE_MANUAL_TEST,
            Self::Unknown => constants::browser::INTERVENTION_DECISION_SOURCE_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    #[serde(rename = "ask-parent")]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Allow => constants::browser::INTERVENTION_ACTION_ALLOW,
            Self::Warn => constants::browser::INTERVENTION_ACTION_WARN,
            Self::Block => constants::browser::INTERVENTION_ACTION_BLOCK,
            Self::Redirect => constants::browser::INTERVENTION_ACTION_REDIRECT,
            Self::TimeLimit => constants::browser::INTERVENTION_ACTION_TIME_LIMIT,
            Self::AskParent => constants::browser::INTERVENTION_ACTION_ASK_PARENT,
            Self::ApprovalHold => constants::browser::INTERVENTION_ACTION_APPROVAL_HOLD,
            Self::CheckingHold => constants::browser::INTERVENTION_ACTION_CHECKING_HOLD,
            Self::TerminateProcess => constants::browser::INTERVENTION_ACTION_TERMINATE_PROCESS,
            Self::RelaunchManaged => constants::browser::INTERVENTION_ACTION_RELAUNCH_MANAGED,
            Self::Monitor => constants::browser::INTERVENTION_ACTION_MONITOR,
            Self::Unknown => constants::browser::INTERVENTION_ACTION_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Site => constants::browser::INTERVENTION_TARGET_TYPE_SITE,
            Self::Domain => constants::browser::INTERVENTION_TARGET_TYPE_DOMAIN,
            Self::Url => constants::browser::INTERVENTION_TARGET_TYPE_URL,
            Self::Video => constants::browser::INTERVENTION_TARGET_TYPE_VIDEO,
            Self::SocialAccountCreation => {
                constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_ACCOUNT_CREATION
            }
            Self::SocialFeed => constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_FEED,
            Self::SocialShortVideoFeed => {
                constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_SHORT_VIDEO_FEED
            }
            Self::SocialMessaging => constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_MESSAGING,
            Self::SocialUploadPost => {
                constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_UPLOAD_POST
            }
            Self::SocialLivestream => {
                constants::browser::INTERVENTION_TARGET_TYPE_SOCIAL_LIVESTREAM
            }
            Self::UnknownSocialSite => {
                constants::browser::INTERVENTION_TARGET_TYPE_UNKNOWN_SOCIAL_SITE
            }
            Self::BrowserGame => constants::browser::INTERVENTION_TARGET_TYPE_BROWSER_GAME,
            Self::GameAccount => constants::browser::INTERVENTION_TARGET_TYPE_GAME_ACCOUNT,
            Self::GamePurchase => constants::browser::INTERVENTION_TARGET_TYPE_GAME_PURCHASE,
            Self::CloudGaming => constants::browser::INTERVENTION_TARGET_TYPE_CLOUD_GAMING,
            Self::UnknownGame => constants::browser::INTERVENTION_TARGET_TYPE_UNKNOWN_GAME,
            Self::UnblockedGameSite => {
                constants::browser::INTERVENTION_TARGET_TYPE_UNBLOCKED_GAME_SITE
            }
            Self::BrowserProcess => constants::browser::INTERVENTION_TARGET_TYPE_BROWSER_PROCESS,
            Self::BrowserSession => constants::browser::INTERVENTION_TARGET_TYPE_BROWSER_SESSION,
            Self::Unknown => constants::browser::INTERVENTION_TARGET_TYPE_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ChromiumCdpFetch => constants::browser::INTERVENTION_MECHANISM_CHROMIUM_CDP_FETCH,
            Self::WebDriverBidiNetwork => {
                constants::browser::INTERVENTION_MECHANISM_WEBDRIVER_BIDI_NETWORK
            }
            Self::ManagedExtension => constants::browser::INTERVENTION_MECHANISM_MANAGED_EXTENSION,
            Self::ManagedBlockPage => constants::browser::INTERVENTION_MECHANISM_MANAGED_BLOCK_PAGE,
            Self::ApprovalHoldPage => constants::browser::INTERVENTION_MECHANISM_APPROVAL_HOLD_PAGE,
            Self::CheckingHoldPage => constants::browser::INTERVENTION_MECHANISM_CHECKING_HOLD_PAGE,
            Self::OsAppControl => constants::browser::INTERVENTION_MECHANISM_OS_APP_CONTROL,
            Self::OwnedWebView => constants::browser::INTERVENTION_MECHANISM_OWNED_WEBVIEW,
            Self::MonitorOnly => constants::browser::INTERVENTION_MECHANISM_MONITOR_ONLY,
            Self::None => constants::browser::INTERVENTION_MECHANISM_NONE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Applied => constants::browser::INTERVENTION_OUTCOME_APPLIED,
            Self::Allowed => constants::browser::INTERVENTION_OUTCOME_ALLOWED,
            Self::Warned => constants::browser::INTERVENTION_OUTCOME_WARNED,
            Self::Blocked => constants::browser::INTERVENTION_OUTCOME_BLOCKED,
            Self::Redirected => constants::browser::INTERVENTION_OUTCOME_REDIRECTED,
            Self::ApprovalRequired => constants::browser::INTERVENTION_OUTCOME_APPROVAL_REQUIRED,
            Self::Held => constants::browser::INTERVENTION_OUTCOME_HELD,
            Self::Terminated => constants::browser::INTERVENTION_OUTCOME_TERMINATED,
            Self::RelaunchStarted => constants::browser::INTERVENTION_OUTCOME_RELAUNCH_STARTED,
            Self::ManualRequired => constants::browser::INTERVENTION_OUTCOME_MANUAL_REQUIRED,
            Self::Failed => constants::browser::INTERVENTION_OUTCOME_FAILED,
            Self::Unsupported => constants::browser::INTERVENTION_OUTCOME_UNSUPPORTED,
            Self::MonitorOnly => constants::browser::INTERVENTION_OUTCOME_MONITOR_ONLY,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::Ready => constants::browser::INTERVENTION_CAPABILITY_READY,
            Self::NeedsManagedSession => {
                constants::browser::INTERVENTION_CAPABILITY_NEEDS_MANAGED_SESSION
            }
            Self::NeedsManagedExtension => {
                constants::browser::INTERVENTION_CAPABILITY_NEEDS_MANAGED_EXTENSION
            }
            Self::NeedsOsAppControl => {
                constants::browser::INTERVENTION_CAPABILITY_NEEDS_OS_APP_CONTROL
            }
            Self::UnsupportedBrowser => {
                constants::browser::INTERVENTION_CAPABILITY_UNSUPPORTED_BROWSER
            }
            Self::DisabledByParent => {
                constants::browser::INTERVENTION_CAPABILITY_DISABLED_BY_PARENT
            }
            Self::AdapterError => constants::browser::INTERVENTION_CAPABILITY_ADAPTER_ERROR,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserUnmanagedFallbackActionState {
    #[serde(rename = "report-only")]
    ReportOnly,
    #[serde(rename = "warn-child")]
    WarnChild,
    #[serde(rename = "ask-parent")]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ReportOnly => constants::browser::UNMANAGED_FALLBACK_ACTION_REPORT_ONLY,
            Self::WarnChild => constants::browser::UNMANAGED_FALLBACK_ACTION_WARN_CHILD,
            Self::AskParent => constants::browser::UNMANAGED_FALLBACK_ACTION_ASK_PARENT,
            Self::TerminateProcess => {
                constants::browser::UNMANAGED_FALLBACK_ACTION_TERMINATE_PROCESS
            }
            Self::RelaunchManagedBrowser => {
                constants::browser::UNMANAGED_FALLBACK_ACTION_RELAUNCH_MANAGED_BROWSER
            }
            Self::OsBlockConfigured => {
                constants::browser::UNMANAGED_FALLBACK_ACTION_OS_BLOCK_CONFIGURED
            }
            Self::OsBlockManualRequired => {
                constants::browser::UNMANAGED_FALLBACK_ACTION_OS_BLOCK_MANUAL_REQUIRED
            }
            Self::AllowedUnmanagedException => {
                constants::browser::UNMANAGED_FALLBACK_ACTION_ALLOWED_UNMANAGED_EXCEPTION
            }
            Self::Degraded => constants::browser::UNMANAGED_FALLBACK_ACTION_DEGRADED,
            Self::Unavailable => constants::browser::UNMANAGED_FALLBACK_ACTION_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ManagedSession => constants::browser::INTERVENTION_BOUNDARY_MANAGED_SESSION,
            Self::UnmanagedBrowserProcess => {
                constants::browser::INTERVENTION_BOUNDARY_UNMANAGED_BROWSER_PROCESS
            }
            Self::BrowserLikeProcess => {
                constants::browser::INTERVENTION_BOUNDARY_BROWSER_LIKE_PROCESS
            }
            Self::Unsupported => constants::browser::INTERVENTION_BOUNDARY_UNSUPPORTED,
            Self::Unknown => constants::browser::INTERVENTION_BOUNDARY_UNKNOWN,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserExactUrlClaimState {
    #[serde(rename = "exact-url-proven")]
    ExactUrlProven,
    #[serde(rename = "not-claimed")]
    NotClaimed,
    #[serde(rename = "unavailable")]
    Unavailable,
}

impl BrowserExactUrlClaimState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ExactUrlProven => constants::browser::INTERVENTION_EXACT_URL_PROVEN,
            Self::NotClaimed => constants::browser::INTERVENTION_EXACT_URL_NOT_CLAIMED,
            Self::Unavailable => constants::browser::INTERVENTION_EXACT_URL_UNAVAILABLE,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::NotDelivered => constants::browser::INTERVENTION_DELIVERY_NOT_DELIVERED,
            Self::WarnPageRendered => constants::browser::INTERVENTION_DELIVERY_WARN_PAGE_RENDERED,
            Self::BlockPageRendered => {
                constants::browser::INTERVENTION_DELIVERY_BLOCK_PAGE_RENDERED
            }
            Self::ApprovalHoldRendered => {
                constants::browser::INTERVENTION_DELIVERY_APPROVAL_HOLD_RENDERED
            }
            Self::CheckingHoldRendered => {
                constants::browser::INTERVENTION_DELIVERY_CHECKING_HOLD_RENDERED
            }
            Self::PortalRowOnly => constants::browser::INTERVENTION_DELIVERY_PORTAL_ROW_ONLY,
            Self::ManualRequired => constants::browser::INTERVENTION_DELIVERY_MANUAL_REQUIRED,
        }
    }
}

impl BrowserUnmanagedDetectionState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::None => constants::browser::INTERVENTION_UNMANAGED_DETECTION_NONE,
            Self::Detected => constants::browser::INTERVENTION_UNMANAGED_DETECTION_DETECTED,
            Self::Warned => constants::browser::INTERVENTION_UNMANAGED_DETECTION_WARNED,
            Self::Terminated => constants::browser::INTERVENTION_UNMANAGED_DETECTION_TERMINATED,
            Self::ManualRequired => {
                constants::browser::INTERVENTION_UNMANAGED_DETECTION_MANUAL_REQUIRED
            }
            Self::Unavailable => constants::browser::INTERVENTION_UNMANAGED_DETECTION_UNAVAILABLE,
        }
    }
}
