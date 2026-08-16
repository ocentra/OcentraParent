use serde::{Deserialize, Serialize};

use crate::constants;

macro_rules! protocol_str_lookup {
    ($self:expr, [$($value:expr),+ $(,)?]) => {{
        const VALUES: &[&str] = &[$($value),+];
        VALUES[*$self as usize]
    }};
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
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
        protocol_str_lookup!(
            self,
            [
                constants::browser::UNMANAGED_ENFORCEMENT_REPORT_ONLY,
                constants::browser::UNMANAGED_ENFORCEMENT_WARN_CHILD,
                constants::browser::UNMANAGED_ENFORCEMENT_ASK_PARENT,
                constants::browser::UNMANAGED_ENFORCEMENT_TERMINATE_PROCESS,
                constants::browser::UNMANAGED_ENFORCEMENT_RELAUNCH_MANAGED_BROWSER,
                constants::browser::UNMANAGED_ENFORCEMENT_OS_BLOCK_CONFIGURED,
                constants::browser::UNMANAGED_ENFORCEMENT_OS_BLOCK_MANUAL_REQUIRED,
                constants::browser::UNMANAGED_ENFORCEMENT_ALLOWED_UNMANAGED_EXCEPTION,
                constants::browser::UNMANAGED_ENFORCEMENT_DEGRADED,
                constants::browser::UNMANAGED_ENFORCEMENT_UNAVAILABLE,
                constants::browser::UNMANAGED_ENFORCEMENT_MONITOR_ONLY,
                constants::browser::UNMANAGED_ENFORCEMENT_REQUIRES_OS_APP_CONTROL,
                constants::browser::UNMANAGED_ENFORCEMENT_READY_TO_BLOCK,
                constants::browser::UNMANAGED_ENFORCEMENT_BLOCKED_AND_RELAUNCHED_MANAGED,
                constants::browser::UNMANAGED_ENFORCEMENT_UNSUPPORTED,
            ]
        )
    }
}
