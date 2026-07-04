use ocentra_parent_agent_core::browser_windows_inventory::windows_browser_inventory_observations;
use ocentra_parent_agent_core::browser_windows_package_inventory::windows_browser_package_observations;
use ocentra_parent_agent_core::browser_windows_package_source::live_windows_browser_package_entries_with_limit;
use ocentra_parent_agent_core::process_capture::ProcessObservation;
use ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryReadModel;
use ocentra_parent_agent_protocol::transport::AgentEventEnvelope;
use ocentra_parent_agent_protocol::{constants, BrowserPolicyValue};
use std::string::String as TestString;

use crate::test_text::TestText;

pub async fn handle_local_command_text_for_test(body: TestText) -> AgentEventEnvelope {
    crate::agent_service_lib::websocket::dispatch_local_command_text(body.0.as_str()).await
}

pub fn browser_inventory_read_model_from_service_defaults_for_test(
    generated_at: TestText,
    process_observations: &[ProcessObservation],
) -> BrowserInventoryReadModel {
    let candidate_paths = crate::browser_runtime_paths::system_browser_candidate_paths();
    let mut observations =
        windows_browser_inventory_observations(&candidate_paths, process_observations, None);
    let package_identities = live_windows_browser_package_entries_with_limit(
        constants::browser::PACKAGE_SCAN_LIMIT_BROWSER_DISCOVERY,
    );
    observations.extend(windows_browser_package_observations(&package_identities));
    crate::browser_inventory_read_model::browser_inventory_read_model_from_windows_inventory(
        generated_at.0,
        &observations,
    )
}

pub fn default_browser_policy_for_test(policy_id: impl Into<TestString>) -> BrowserPolicyValue {
    crate::browser_policy_runtime_support::default_policy(policy_id.into())
}
