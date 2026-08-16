use ocentra_parent_agent_protocol::browser::BrowserCapabilityStatus;
use ocentra_parent_agent_protocol::browser_inventory::BrowserActiveTabCapability;
use ocentra_parent_agent_protocol::browser_inventory::BrowserExactUrlCapability;
use ocentra_parent_agent_protocol::browser_inventory::BrowserManagedProfileState;
use ocentra_parent_agent_protocol::browser_inventory::BrowserManagementTier;
use ocentra_parent_agent_protocol::browser_inventory::BrowserSupportTier;
use ocentra_parent_agent_protocol::browser_inventory::BrowserUnmanagedFallbackCapability;
use ocentra_parent_agent_protocol::constants;

use crate::browser_windows_package_inventory::BrowserWindowsPackageSupportKind;

pub(crate) fn package_capability_state(
    support_kind: BrowserWindowsPackageSupportKind,
) -> (
    BrowserManagementTier,
    BrowserSupportTier,
    BrowserExactUrlCapability,
    BrowserActiveTabCapability,
    BrowserManagedProfileState,
    BrowserUnmanagedFallbackCapability,
    BrowserCapabilityStatus,
    &'static str,
) {
    match support_kind {
        BrowserWindowsPackageSupportKind::ManualChromium => (
            BrowserManagementTier::ManualRequired,
            BrowserSupportTier::ManualRequired,
            BrowserExactUrlCapability::ManualRequired,
            BrowserActiveTabCapability::ManualRequired,
            BrowserManagedProfileState::ManualRequired,
            BrowserUnmanagedFallbackCapability::OsBlockManualRequired,
            BrowserCapabilityStatus::PermissionLimited,
            constants::browser::INVENTORY_REASON_WINDOWS_PACKAGE_MANUAL_REQUIRED,
        ),
        BrowserWindowsPackageSupportKind::Unsupported => (
            BrowserManagementTier::Unsupported,
            BrowserSupportTier::Unsupported,
            BrowserExactUrlCapability::Unsupported,
            BrowserActiveTabCapability::Unsupported,
            BrowserManagedProfileState::NotApplicable,
            BrowserUnmanagedFallbackCapability::Unsupported,
            BrowserCapabilityStatus::UnsupportedBrowser,
            constants::browser::INVENTORY_REASON_WINDOWS_UNSUPPORTED_LATER_ADAPTER,
        ),
    }
}
