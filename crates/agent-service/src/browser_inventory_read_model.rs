use ocentra_parent_agent_core::browser_windows_inventory::BrowserWindowsInventoryObservation;
use ocentra_parent_agent_protocol::browser::BrowserCustodyLabel;
use ocentra_parent_agent_protocol::browser_inventory::{
    BrowserInventoryReadModel, BrowserInventoryRow,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::BROWSER_EVIDENCE_SCHEMA_VERSION;

pub fn browser_inventory_read_model_from_windows_inventory(
    scanned_at: String,
    observations: &[BrowserWindowsInventoryObservation],
) -> BrowserInventoryReadModel {
    let rows = observations
        .iter()
        .enumerate()
        .map(|(index, observation)| {
            browser_inventory_row_from_windows_observation(&scanned_at, index, observation)
        })
        .collect::<Vec<_>>();
    let returned = rows.len() as u64;
    let latest_observed_at = latest_observed_at(&scanned_at, returned);

    BrowserInventoryReadModel {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at: scanned_at,
        limit: returned,
        returned,
        latest_observed_at,
        capability_status: None,
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        rows,
    }
}

fn browser_inventory_row_from_windows_observation(
    scanned_at: &str,
    row_index: usize,
    observation: &BrowserWindowsInventoryObservation,
) -> BrowserInventoryRow {
    BrowserInventoryRow {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        inventory_row_id: windows_inventory_row_id(observation, row_index),
        scanned_at: scanned_at.to_string(),
        device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        product_name: observation.product_name.clone(),
        browser_family: observation.browser_family.clone(),
        browser_channel: observation.browser_channel.clone(),
        browser_version: None,
        install_state: observation.install_state.clone(),
        running_state: observation.running_state.clone(),
        management_tier: observation.management_tier.clone(),
        support_tier: observation.support_tier.clone(),
        exact_url_capability: observation.exact_url_capability.clone(),
        active_tab_capability: observation.active_tab_capability.clone(),
        managed_profile_state: observation.managed_profile_state.clone(),
        unmanaged_fallback_capability: observation.unmanaged_fallback_capability.clone(),
        executable_path_ref: observation.executable_path.as_ref().map(|_| {
            constants::browser::INVENTORY_EXECUTABLE_PATH_REF_WINDOWS_REDACTED.to_string()
        }),
        publisher_signature_ref: None,
        file_hash_ref: None,
        profile_id: None,
        process_id: observation.process_id,
        capability_status: observation.capability_status.clone(),
        reason_code: observation.reason_code.to_string(),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    }
}

fn latest_observed_at(scanned_at: &str, returned: u64) -> Option<String> {
    if returned == 0 {
        return None;
    }
    Some(scanned_at.to_string())
}

fn windows_inventory_row_id(
    observation: &BrowserWindowsInventoryObservation,
    row_index: usize,
) -> String {
    let mut row_id = String::from(constants::browser::INVENTORY_ROW_ID_PREFIX_WINDOWS);
    row_id.push(constants::delimiter::HYPHEN);
    row_id.push_str(observation.browser_family.as_protocol_str());
    row_id.push(constants::delimiter::HYPHEN);
    row_id.push_str(observation.browser_channel.as_protocol_str());
    row_id.push(constants::delimiter::HYPHEN);
    row_id.push_str(observation.install_state.as_protocol_str());
    row_id.push(constants::delimiter::HYPHEN);
    match observation.process_id {
        Some(process_id) => row_id.push_str(&process_id.to_string()),
        None => row_id.push_str(&row_index.to_string()),
    }
    row_id
}
