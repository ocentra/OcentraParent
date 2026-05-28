use crate::{
    constants, BrowserBoundaryState, BrowserExactUrlClaimState, BrowserInterventionAction,
    BrowserInterventionCapabilityState, BrowserInterventionDecisionSource,
    BrowserInterventionMechanism, BrowserInterventionOutcome, BrowserInterventionTargetType,
    BrowserUnmanagedDetectionState, BrowserUnmanagedEnforcementState,
};

impl BrowserInterventionDecisionSource {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::INTERVENTION_DECISION_SOURCE_PARENT_RULE => Some(Self::ParentRule),
            constants::browser::INTERVENTION_DECISION_SOURCE_PARENT_PORTAL => {
                Some(Self::ParentPortal)
            }
            constants::browser::INTERVENTION_DECISION_SOURCE_LOCAL_AI => Some(Self::LocalAi),
            constants::browser::INTERVENTION_DECISION_SOURCE_SYSTEM => Some(Self::System),
            constants::browser::INTERVENTION_DECISION_SOURCE_MANUAL_TEST => Some(Self::ManualTest),
            constants::browser::INTERVENTION_DECISION_SOURCE_UNKNOWN => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl BrowserInterventionAction {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::INTERVENTION_ACTION_ALLOW => Some(Self::Allow),
            constants::browser::INTERVENTION_ACTION_WARN => Some(Self::Warn),
            constants::browser::INTERVENTION_ACTION_BLOCK => Some(Self::Block),
            constants::browser::INTERVENTION_ACTION_TIME_LIMIT => Some(Self::TimeLimit),
            constants::browser::INTERVENTION_ACTION_ASK_PARENT => Some(Self::AskParent),
            constants::browser::INTERVENTION_ACTION_MONITOR => Some(Self::Monitor),
            constants::browser::INTERVENTION_ACTION_UNKNOWN => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl BrowserInterventionTargetType {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::INTERVENTION_TARGET_TYPE_SITE => Some(Self::Site),
            constants::browser::INTERVENTION_TARGET_TYPE_DOMAIN => Some(Self::Domain),
            constants::browser::INTERVENTION_TARGET_TYPE_URL => Some(Self::Url),
            constants::browser::INTERVENTION_TARGET_TYPE_VIDEO => Some(Self::Video),
            constants::browser::INTERVENTION_TARGET_TYPE_BROWSER_PROCESS => {
                Some(Self::BrowserProcess)
            }
            constants::browser::INTERVENTION_TARGET_TYPE_BROWSER_SESSION => {
                Some(Self::BrowserSession)
            }
            constants::browser::INTERVENTION_TARGET_TYPE_UNKNOWN => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl BrowserInterventionMechanism {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::INTERVENTION_MECHANISM_CHROMIUM_CDP_FETCH => {
                Some(Self::ChromiumCdpFetch)
            }
            constants::browser::INTERVENTION_MECHANISM_WEBDRIVER_BIDI_NETWORK => {
                Some(Self::WebDriverBidiNetwork)
            }
            constants::browser::INTERVENTION_MECHANISM_MANAGED_EXTENSION => {
                Some(Self::ManagedExtension)
            }
            constants::browser::INTERVENTION_MECHANISM_OS_APP_CONTROL => Some(Self::OsAppControl),
            constants::browser::INTERVENTION_MECHANISM_OWNED_WEBVIEW => Some(Self::OwnedWebView),
            constants::browser::INTERVENTION_MECHANISM_MONITOR_ONLY => Some(Self::MonitorOnly),
            constants::browser::INTERVENTION_MECHANISM_NONE => Some(Self::None),
            _ => None,
        }
    }
}

impl BrowserInterventionOutcome {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::INTERVENTION_OUTCOME_APPLIED => Some(Self::Applied),
            constants::browser::INTERVENTION_OUTCOME_ALLOWED => Some(Self::Allowed),
            constants::browser::INTERVENTION_OUTCOME_BLOCKED => Some(Self::Blocked),
            constants::browser::INTERVENTION_OUTCOME_FAILED => Some(Self::Failed),
            constants::browser::INTERVENTION_OUTCOME_UNSUPPORTED => Some(Self::Unsupported),
            constants::browser::INTERVENTION_OUTCOME_MONITOR_ONLY => Some(Self::MonitorOnly),
            _ => None,
        }
    }
}

impl BrowserInterventionCapabilityState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::INTERVENTION_CAPABILITY_READY => Some(Self::Ready),
            constants::browser::INTERVENTION_CAPABILITY_NEEDS_MANAGED_SESSION => {
                Some(Self::NeedsManagedSession)
            }
            constants::browser::INTERVENTION_CAPABILITY_NEEDS_MANAGED_EXTENSION => {
                Some(Self::NeedsManagedExtension)
            }
            constants::browser::INTERVENTION_CAPABILITY_NEEDS_OS_APP_CONTROL => {
                Some(Self::NeedsOsAppControl)
            }
            constants::browser::INTERVENTION_CAPABILITY_UNSUPPORTED_BROWSER => {
                Some(Self::UnsupportedBrowser)
            }
            constants::browser::INTERVENTION_CAPABILITY_DISABLED_BY_PARENT => {
                Some(Self::DisabledByParent)
            }
            constants::browser::INTERVENTION_CAPABILITY_ADAPTER_ERROR => Some(Self::AdapterError),
            _ => None,
        }
    }
}

impl BrowserUnmanagedEnforcementState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::UNMANAGED_ENFORCEMENT_MONITOR_ONLY => Some(Self::MonitorOnly),
            constants::browser::UNMANAGED_ENFORCEMENT_REQUIRES_OS_APP_CONTROL => {
                Some(Self::RequiresOsAppControl)
            }
            constants::browser::UNMANAGED_ENFORCEMENT_READY_TO_BLOCK => Some(Self::ReadyToBlock),
            constants::browser::UNMANAGED_ENFORCEMENT_BLOCKED_AND_RELAUNCHED_MANAGED => {
                Some(Self::BlockedAndRelaunchedManaged)
            }
            constants::browser::UNMANAGED_ENFORCEMENT_UNSUPPORTED => Some(Self::Unsupported),
            _ => None,
        }
    }
}

impl BrowserBoundaryState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::INTERVENTION_BOUNDARY_MANAGED_SESSION => Some(Self::ManagedSession),
            constants::browser::INTERVENTION_BOUNDARY_UNMANAGED_BROWSER_PROCESS => {
                Some(Self::UnmanagedBrowserProcess)
            }
            constants::browser::INTERVENTION_BOUNDARY_BROWSER_LIKE_PROCESS => {
                Some(Self::BrowserLikeProcess)
            }
            constants::browser::INTERVENTION_BOUNDARY_UNSUPPORTED => Some(Self::Unsupported),
            constants::browser::INTERVENTION_BOUNDARY_UNKNOWN => Some(Self::Unknown),
            _ => None,
        }
    }
}

impl BrowserExactUrlClaimState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::INTERVENTION_EXACT_URL_PROVEN => Some(Self::ExactUrlProven),
            constants::browser::INTERVENTION_EXACT_URL_NOT_CLAIMED => Some(Self::NotClaimed),
            constants::browser::INTERVENTION_EXACT_URL_UNAVAILABLE => Some(Self::Unavailable),
            _ => None,
        }
    }
}

impl BrowserUnmanagedDetectionState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        match value {
            constants::browser::INTERVENTION_UNMANAGED_DETECTION_NONE => Some(Self::None),
            constants::browser::INTERVENTION_UNMANAGED_DETECTION_DETECTED => Some(Self::Detected),
            constants::browser::INTERVENTION_UNMANAGED_DETECTION_WARNED => Some(Self::Warned),
            constants::browser::INTERVENTION_UNMANAGED_DETECTION_TERMINATED => {
                Some(Self::Terminated)
            }
            constants::browser::INTERVENTION_UNMANAGED_DETECTION_MANUAL_REQUIRED => {
                Some(Self::ManualRequired)
            }
            constants::browser::INTERVENTION_UNMANAGED_DETECTION_UNAVAILABLE => {
                Some(Self::Unavailable)
            }
            _ => None,
        }
    }
}
