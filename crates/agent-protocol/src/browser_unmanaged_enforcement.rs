use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserUnmanagedEnforcementState {
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
    #[serde(rename = "monitor-only")]
    MonitorOnly,
    #[serde(rename = "requires-os-app-control")]
    RequiresOsAppControl,
    #[serde(rename = "ready-to-block")]
    ReadyToBlock,
    #[serde(rename = "blocked-and-relaunched-managed")]
    BlockedAndRelaunchedManaged,
    #[serde(rename = "unsupported")]
    Unsupported,
}

impl BrowserUnmanagedEnforcementState {
    pub fn as_protocol_str(&self) -> &'static str {
        match self {
            Self::ReportOnly => constants::browser::UNMANAGED_ENFORCEMENT_REPORT_ONLY,
            Self::WarnChild => constants::browser::UNMANAGED_ENFORCEMENT_WARN_CHILD,
            Self::AskParent => constants::browser::UNMANAGED_ENFORCEMENT_ASK_PARENT,
            Self::TerminateProcess => constants::browser::UNMANAGED_ENFORCEMENT_TERMINATE_PROCESS,
            Self::RelaunchManagedBrowser => {
                constants::browser::UNMANAGED_ENFORCEMENT_RELAUNCH_MANAGED_BROWSER
            }
            Self::OsBlockConfigured => {
                constants::browser::UNMANAGED_ENFORCEMENT_OS_BLOCK_CONFIGURED
            }
            Self::OsBlockManualRequired => {
                constants::browser::UNMANAGED_ENFORCEMENT_OS_BLOCK_MANUAL_REQUIRED
            }
            Self::AllowedUnmanagedException => {
                constants::browser::UNMANAGED_ENFORCEMENT_ALLOWED_UNMANAGED_EXCEPTION
            }
            Self::Degraded => constants::browser::UNMANAGED_ENFORCEMENT_DEGRADED,
            Self::Unavailable => constants::browser::UNMANAGED_ENFORCEMENT_UNAVAILABLE,
            Self::MonitorOnly => constants::browser::UNMANAGED_ENFORCEMENT_MONITOR_ONLY,
            Self::RequiresOsAppControl => {
                constants::browser::UNMANAGED_ENFORCEMENT_REQUIRES_OS_APP_CONTROL
            }
            Self::ReadyToBlock => constants::browser::UNMANAGED_ENFORCEMENT_READY_TO_BLOCK,
            Self::BlockedAndRelaunchedManaged => {
                constants::browser::UNMANAGED_ENFORCEMENT_BLOCKED_AND_RELAUNCHED_MANAGED
            }
            Self::Unsupported => constants::browser::UNMANAGED_ENFORCEMENT_UNSUPPORTED,
        }
    }
}
