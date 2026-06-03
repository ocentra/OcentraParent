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
    #[serde(rename = "parent-review")]
    AskParent,
    #[serde(rename = "block")]
    Block,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyExecutionMode {
    #[default]
    #[serde(rename = "observe")]
    Observe,
    #[serde(rename = "dry-run")]
    DryRun,
    #[serde(rename = "warn-ask")]
    WarnAsk,
    #[serde(rename = "enforce")]
    Enforce,
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
    #[serde(rename = "report-only")]
    ReportOnly,
    #[serde(rename = "observe-only")]
    ObserveOnly,
    #[serde(rename = "network-domain-only")]
    NetworkDomainOnly,
    #[serde(rename = "manual-review")]
    ManualReview,
    #[serde(rename = "allow")]
    Allow,
    #[serde(rename = "allowed-unmanaged-exception")]
    AllowedUnmanagedException,
    #[serde(rename = "monitor")]
    Monitor,
    #[serde(rename = "warn-child")]
    WarnChild,
    #[serde(rename = "warn")]
    Warn,
    #[serde(rename = "parent-review")]
    AskParent,
    #[serde(rename = "ask")]
    Ask,
    #[serde(rename = "terminate-process")]
    TerminateProcess,
    #[serde(rename = "relaunch-managed")]
    RelaunchManaged,
    #[serde(rename = "os-block-configured")]
    OsBlockConfigured,
    #[serde(rename = "os-block-manual-required")]
    OsBlockManualRequired,
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
    #[serde(rename = "social-platform")]
    SocialPlatform,
    #[serde(rename = "social-route-kind")]
    SocialRouteKind,
    #[serde(rename = "social-account-creation")]
    SocialAccountCreation,
    #[serde(rename = "social-unknown-account")]
    SocialUnknownAccount,
    #[serde(rename = "social-secondary-account")]
    SocialSecondaryAccount,
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
    #[serde(rename = "browser-game-platform")]
    BrowserGamePlatform,
    #[serde(rename = "browser-game-portal")]
    BrowserGamePortal,
    #[serde(rename = "browser-game-url")]
    BrowserGameUrl,
    #[serde(rename = "educational-game")]
    EducationalGame,
    #[serde(rename = "cloud-gaming")]
    CloudGaming,
    #[serde(rename = "webgl-canvas-game")]
    WebglCanvasGame,
    #[serde(rename = "multiplayer-ugc-game")]
    MultiplayerUgcGame,
    #[serde(rename = "game-chat")]
    GameChat,
    #[serde(rename = "game-account")]
    GameAccount,
    #[serde(rename = "game-purchase")]
    GamePurchase,
    #[serde(rename = "game-loot-box")]
    GameLootBox,
    #[serde(rename = "unknown-game")]
    UnknownGame,
    #[serde(rename = "unblocked-game-site")]
    UnblockedGameSite,
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
    #[serde(rename = "classifier-category")]
    ClassifierCategory,
    #[serde(rename = "url-shape-metadata")]
    UrlShapeMetadata,
    #[serde(rename = "social-route-evidence")]
    SocialRouteEvidence,
    #[serde(rename = "browser-game-runtime-signal")]
    BrowserGameRuntimeSignal,
    #[serde(rename = "browser-policy-writer")]
    BrowserPolicyWriter,
    #[serde(rename = "adapter-action")]
    AdapterAction,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyProofFallback {
    #[serde(rename = "downgrade-to-domain")]
    DowngradeToDomain,
    #[serde(rename = "parent-review")]
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
    #[serde(rename = "parent-review")]
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
    #[serde(rename = "ready")]
    Ready,
    #[serde(rename = "manual-required")]
    ManualRequired,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyTargetProofRequirement {
    #[default]
    #[serde(rename = "none")]
    None,
    #[serde(rename = "managed-exact-url")]
    ManagedExactUrl,
    #[serde(rename = "domain-or-managed-url")]
    DomainOrManagedUrl,
    #[serde(rename = "classifier-category")]
    ClassifierCategory,
    #[serde(rename = "url-shape-metadata")]
    UrlShapeMetadata,
    #[serde(rename = "social-route-evidence")]
    SocialRouteEvidence,
    #[serde(rename = "browser-game-runtime-signal")]
    BrowserGameRuntimeSignal,
    #[serde(rename = "browser-policy-writer")]
    BrowserPolicyWriter,
    #[serde(rename = "process-detection")]
    ProcessDetection,
    #[serde(rename = "download-evidence")]
    DownloadEvidence,
    #[serde(rename = "capability-state")]
    CapabilityState,
    #[serde(rename = "adapter-action")]
    AdapterAction,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyActionExecutionState {
    #[default]
    #[serde(rename = "observe-only")]
    ObserveOnly,
    #[serde(rename = "dry-run-no-execution")]
    DryRunNoExecution,
    #[serde(rename = "deterministic-parent-policy")]
    DeterministicParentPolicy,
    #[serde(rename = "adapter-ready")]
    AdapterReady,
    #[serde(rename = "manual-required")]
    ManualRequired,
    #[serde(rename = "unavailable")]
    Unavailable,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserPolicyAiAuthority {
    #[default]
    #[serde(rename = "parent-policy-only")]
    ParentPolicyOnly,
    #[serde(rename = "ai-candidate-only")]
    AiCandidateOnly,
}
