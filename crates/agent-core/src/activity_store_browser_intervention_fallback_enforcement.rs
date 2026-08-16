use ocentra_parent_agent_protocol::browser_intervention_values::BrowserUnmanagedFallbackActionState;
use ocentra_parent_agent_protocol::browser_unmanaged_enforcement::BrowserUnmanagedEnforcementState;

pub(crate) fn fallback_action_for_unmanaged_enforcement(
    unmanaged_browser_enforcement: &BrowserUnmanagedEnforcementState,
) -> Option<BrowserUnmanagedFallbackActionState> {
    match unmanaged_browser_enforcement {
        BrowserUnmanagedEnforcementState::ReportOnly
        | BrowserUnmanagedEnforcementState::MonitorOnly => {
            Some(BrowserUnmanagedFallbackActionState::ReportOnly)
        }
        BrowserUnmanagedEnforcementState::WarnChild => {
            Some(BrowserUnmanagedFallbackActionState::WarnChild)
        }
        BrowserUnmanagedEnforcementState::AskParent => {
            Some(BrowserUnmanagedFallbackActionState::AskParent)
        }
        BrowserUnmanagedEnforcementState::TerminateProcess
        | BrowserUnmanagedEnforcementState::ReadyToBlock => {
            Some(BrowserUnmanagedFallbackActionState::TerminateProcess)
        }
        BrowserUnmanagedEnforcementState::RelaunchManagedBrowser
        | BrowserUnmanagedEnforcementState::BlockedAndRelaunchedManaged => {
            Some(BrowserUnmanagedFallbackActionState::RelaunchManagedBrowser)
        }
        BrowserUnmanagedEnforcementState::OsBlockConfigured => {
            Some(BrowserUnmanagedFallbackActionState::OsBlockConfigured)
        }
        BrowserUnmanagedEnforcementState::RequiresOsAppControl
        | BrowserUnmanagedEnforcementState::OsBlockManualRequired => {
            Some(BrowserUnmanagedFallbackActionState::OsBlockManualRequired)
        }
        BrowserUnmanagedEnforcementState::AllowedUnmanagedException => {
            Some(BrowserUnmanagedFallbackActionState::AllowedUnmanagedException)
        }
        BrowserUnmanagedEnforcementState::Degraded => {
            Some(BrowserUnmanagedFallbackActionState::Degraded)
        }
        BrowserUnmanagedEnforcementState::Unavailable
        | BrowserUnmanagedEnforcementState::Unsupported => None,
    }
}
