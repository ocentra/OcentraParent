use ocentra_parent_agent_protocol::browser::{BrowserChannel, BrowserFamily};
use ocentra_parent_agent_protocol::browser_inventory::{
    BrowserInventoryInstallState, BrowserInventoryRunningState,
};

use crate::browser_windows_inventory::BrowserWindowsInventoryObservation;

#[path = "browser_windows_package_inventory_capability.rs"]
mod browser_windows_package_inventory_capability;
#[path = "browser_windows_package_inventory_identity.rs"]
mod browser_windows_package_inventory_identity;

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
    let (identity, support_kind) =
        browser_windows_package_inventory_identity::windows_browser_package_identity(
            package_identity,
        )?;
    let (
        management_tier,
        support_tier,
        exact_url_capability,
        active_tab_capability,
        managed_profile_state,
        unmanaged_fallback_capability,
        capability_status,
        reason_code,
    ) = browser_windows_package_inventory_capability::package_capability_state(support_kind);
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
