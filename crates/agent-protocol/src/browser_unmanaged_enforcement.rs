use serde::{Deserialize, Serialize};

use crate::constants;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum BrowserUnmanagedEnforcementState {
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
