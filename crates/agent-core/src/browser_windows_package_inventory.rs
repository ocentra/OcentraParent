use ocentra_parent_agent_protocol::browser::{
    BrowserCapabilityStatus, BrowserChannel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_inventory::BrowserActiveTabCapability;
use ocentra_parent_agent_protocol::browser_inventory::BrowserExactUrlCapability;
use ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryInstallState;
use ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryRunningState;
use ocentra_parent_agent_protocol::browser_inventory::BrowserManagedProfileState;
use ocentra_parent_agent_protocol::browser_inventory::BrowserManagementTier;
use ocentra_parent_agent_protocol::browser_inventory::BrowserSupportTier;
use ocentra_parent_agent_protocol::browser_inventory::BrowserUnmanagedFallbackCapability;
use ocentra_parent_agent_protocol::constants;

use crate::browser_windows_inventory::BrowserWindowsInventoryObservation;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserWindowsPackageIdentity {
    pub package_name: String,
    pub display_name: Option<String>,
    pub app_user_model_id: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BrowserWindowsPackageSupportKind {
    ManualChromium,
    Unsupported,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct BrowserWindowsPackageDisplayIdentity {
    browser_family: BrowserFamily,
    browser_channel: BrowserChannel,
    product_name: &'static str,
}

pub fn windows_browser_package_observations(
    package_identities: &[BrowserWindowsPackageIdentity],
) -> Vec<BrowserWindowsInventoryObservation> {
    let mut observations = package_identities
        .iter()
        .filter_map(windows_browser_package_observation)
        .collect::<Vec<_>>();
    observations.sort_by(|left, right| {
        left.product_name.cmp(&right.product_name).then_with(|| {
            left.browser_channel
                .as_protocol_str()
                .cmp(right.browser_channel.as_protocol_str())
        })
    });
    observations
}

fn windows_browser_package_observation(
    package_identity: &BrowserWindowsPackageIdentity,
) -> Option<BrowserWindowsInventoryObservation> {
    let (identity, support_kind) = windows_browser_package_identity(package_identity)?;
    let (
        management_tier,
        support_tier,
        exact_url_capability,
        active_tab_capability,
        managed_profile_state,
        unmanaged_fallback_capability,
        capability_status,
        reason_code,
    ) = package_capability_state(support_kind);
    Some(BrowserWindowsInventoryObservation {
        executable_path: None,
        process_id: None,
        product_name: identity.product_name.to_string(),
        browser_family: identity.browser_family,
        browser_channel: identity.browser_channel,
        install_state: BrowserInventoryInstallState::Packaged,
        running_state: BrowserInventoryRunningState::NotRunning,
        management_tier,
        support_tier,
        exact_url_capability,
        active_tab_capability,
        managed_profile_state,
        unmanaged_fallback_capability,
        capability_status,
        reason_code,
    })
}

fn package_capability_state(
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

fn windows_browser_package_identity(
    package_identity: &BrowserWindowsPackageIdentity,
) -> Option<(
    BrowserWindowsPackageDisplayIdentity,
    BrowserWindowsPackageSupportKind,
)> {
    let mut normalized = package_identity.package_name.to_ascii_lowercase();
    if let Some(display_name) = package_identity.display_name.as_deref() {
        normalized.push_str(&display_name.to_ascii_lowercase());
    }
    if let Some(app_user_model_id) = package_identity.app_user_model_id.as_deref() {
        normalized.push_str(&app_user_model_id.to_ascii_lowercase());
    }
    package_identity_from_normalized(&normalized)
}

fn package_identity_from_normalized(
    value: &str,
) -> Option<(
    BrowserWindowsPackageDisplayIdentity,
    BrowserWindowsPackageSupportKind,
)> {
    if value.contains(constants::browser::PACKAGE_FRAGMENT_MICROSOFT_EDGE)
        || value.contains(constants::browser::PACKAGE_FRAGMENT_EDGE)
    {
        return Some((
            manual_identity(
                BrowserFamily::Edge,
                constants::browser::PRODUCT_NAME_MICROSOFT_EDGE,
            ),
            BrowserWindowsPackageSupportKind::ManualChromium,
        ));
    }
    if value.contains(constants::browser::PACKAGE_FRAGMENT_CHROME) {
        return Some((
            manual_identity(
                BrowserFamily::Chrome,
                constants::browser::PRODUCT_NAME_GOOGLE_CHROME,
            ),
            BrowserWindowsPackageSupportKind::ManualChromium,
        ));
    }
    package_chromium_fork_identity(value).or_else(|| package_unsupported_identity(value))
}

fn package_chromium_fork_identity(
    value: &str,
) -> Option<(
    BrowserWindowsPackageDisplayIdentity,
    BrowserWindowsPackageSupportKind,
)> {
    let (family, product_name) = if value.contains(constants::browser::PACKAGE_FRAGMENT_BRAVE) {
        (
            BrowserFamily::Brave,
            constants::browser::PRODUCT_NAME_BRAVE_BROWSER,
        )
    } else if value.contains(constants::browser::PACKAGE_FRAGMENT_VIVALDI) {
        (
            BrowserFamily::UnknownChromium,
            constants::browser::PRODUCT_NAME_VIVALDI_BROWSER,
        )
    } else if value.contains(constants::browser::PACKAGE_FRAGMENT_OPERA) {
        (
            BrowserFamily::Opera,
            constants::browser::PRODUCT_NAME_OPERA_BROWSER,
        )
    } else if value.contains(constants::browser::PACKAGE_FRAGMENT_CHROMIUM) {
        (
            BrowserFamily::UnknownChromium,
            constants::browser::PRODUCT_NAME_CHROMIUM,
        )
    } else {
        return None;
    };
    Some((
        manual_identity(family, product_name),
        BrowserWindowsPackageSupportKind::ManualChromium,
    ))
}

fn package_unsupported_identity(
    value: &str,
) -> Option<(
    BrowserWindowsPackageDisplayIdentity,
    BrowserWindowsPackageSupportKind,
)> {
    let (family, product_name) = if value.contains(constants::browser::PACKAGE_FRAGMENT_FIREFOX) {
        (
            BrowserFamily::Firefox,
            constants::browser::PRODUCT_NAME_MOZILLA_FIREFOX,
        )
    } else if value.contains(constants::browser::PACKAGE_FRAGMENT_TOR) {
        (
            BrowserFamily::Unknown,
            constants::browser::PRODUCT_NAME_TOR_BROWSER,
        )
    } else if value.contains(constants::browser::PACKAGE_FRAGMENT_DUCKDUCKGO) {
        (
            BrowserFamily::Unknown,
            constants::browser::PRODUCT_NAME_DUCKDUCKGO_BROWSER,
        )
    } else if value.contains(constants::browser::PACKAGE_FRAGMENT_ARC) {
        (
            BrowserFamily::UnknownChromium,
            constants::browser::PRODUCT_NAME_ARC_BROWSER,
        )
    } else {
        return None;
    };
    Some((
        manual_identity(family, product_name),
        BrowserWindowsPackageSupportKind::Unsupported,
    ))
}

fn manual_identity(
    browser_family: BrowserFamily,
    product_name: &'static str,
) -> BrowserWindowsPackageDisplayIdentity {
    BrowserWindowsPackageDisplayIdentity {
        browser_family,
        browser_channel: BrowserChannel::Stable,
        product_name,
    }
}
