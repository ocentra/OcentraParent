use super::protocol_lookup;
use crate::{constants, BrowserUnmanagedFallbackActionState};

impl BrowserUnmanagedFallbackActionState {
    pub fn from_protocol_str(value: &str) -> Option<Self> {
        protocol_lookup(
            value,
            [
                (
                    constants::browser::UNMANAGED_FALLBACK_ACTION_REPORT_ONLY,
                    Self::ReportOnly,
                ),
                (
                    constants::browser::UNMANAGED_FALLBACK_ACTION_WARN_CHILD,
                    Self::WarnChild,
                ),
                (
                    constants::browser::UNMANAGED_FALLBACK_ACTION_ASK_PARENT,
                    Self::AskParent,
                ),
                (
                    constants::browser::UNMANAGED_FALLBACK_ACTION_TERMINATE_PROCESS,
                    Self::TerminateProcess,
                ),
                (
                    constants::browser::UNMANAGED_FALLBACK_ACTION_RELAUNCH_MANAGED_BROWSER,
                    Self::RelaunchManagedBrowser,
                ),
                (
                    constants::browser::UNMANAGED_FALLBACK_ACTION_OS_BLOCK_CONFIGURED,
                    Self::OsBlockConfigured,
                ),
                (
                    constants::browser::UNMANAGED_FALLBACK_ACTION_OS_BLOCK_MANUAL_REQUIRED,
                    Self::OsBlockManualRequired,
                ),
                (
                    constants::browser::UNMANAGED_FALLBACK_ACTION_ALLOWED_UNMANAGED_EXCEPTION,
                    Self::AllowedUnmanagedException,
                ),
                (
                    constants::browser::UNMANAGED_FALLBACK_ACTION_DEGRADED,
                    Self::Degraded,
                ),
                (
                    constants::browser::UNMANAGED_FALLBACK_ACTION_UNAVAILABLE,
                    Self::Unavailable,
                ),
            ],
        )
    }
}
