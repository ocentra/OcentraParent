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
    #[serde(rename = "time-limit")]
    TimeLimit,
    #[serde(rename = "ask-parent")]
    AskParent,
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
            Self::TimeLimit => constants::browser::INTERVENTION_ACTION_TIME_LIMIT,
            Self::AskParent => constants::browser::INTERVENTION_ACTION_ASK_PARENT,
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
    #[serde(rename = "blocked")]
    Blocked,
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
            Self::Blocked => constants::browser::INTERVENTION_OUTCOME_BLOCKED,
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
