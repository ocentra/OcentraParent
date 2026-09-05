use ocentra_parent_agent_core::browser_platform_inventory::{
    browser_platform_inventory_observations, BrowserPlatformInventoryObservation,
};
use ocentra_parent_agent_core::browser_windows_package_inventory::windows_browser_package_observations;
use ocentra_parent_agent_core::browser_windows_package_source::live_windows_browser_package_entries_with_limit;
use ocentra_parent_agent_core::process_capture::ProcessObservation;
use ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryReadModel;
use ocentra_parent_agent_protocol::constants;

use crate::browser_inventory_read_model::{
    browser_inventory_read_model_from_platform_inventory, BrowserInventoryGeneratedAtText,
};

pub fn browser_inventory_read_model_from_service_defaults_for_test(
    generated_at: BrowserInventoryGeneratedAtText,
    process_observations: &[ProcessObservation],
) -> BrowserInventoryReadModel {
    let candidate_paths = crate::browser_runtime_paths::system_browser_candidate_paths();
    let mut observations =
        browser_platform_inventory_observations(&candidate_paths.0, process_observations, None);
    let package_identities = live_windows_browser_package_entries_with_limit(
        constants::browser::PACKAGE_SCAN_LIMIT_BROWSER_DISCOVERY,
    );
    observations.extend(
        windows_browser_package_observations(&package_identities)
            .iter()
            .map(BrowserPlatformInventoryObservation::from),
    );
    browser_inventory_read_model_from_platform_inventory(generated_at, &observations)
}
