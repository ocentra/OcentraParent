use ocentra_parent_agent_core::browser_platform_inventory::BrowserPlatformInventoryObservation;
use ocentra_parent_agent_core::browser_windows_inventory::BrowserWindowsInventoryObservation;
use ocentra_parent_agent_protocol::browser::BrowserCustodyLabel;
use ocentra_parent_agent_protocol::browser_inventory::{
    BrowserInventoryReadModel, BrowserInventoryRow,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::BROWSER_EVIDENCE_SCHEMA_VERSION;

#[derive(Clone)]
pub struct BrowserInventoryGeneratedAtText(pub String);

struct BrowserInventoryRowIdText(String);

#[derive(Clone, Copy)]
struct BrowserInventoryRowIdPrefix(&'static str);

#[derive(Clone, Copy)]
struct BrowserInventoryExecutablePathRef(&'static str);

pub fn browser_inventory_read_model_from_windows_inventory(
    generated_at: BrowserInventoryGeneratedAtText,
    observations: &[BrowserWindowsInventoryObservation],
) -> BrowserInventoryReadModel {
    let observations = observations
        .iter()
        .map(BrowserPlatformInventoryObservation::from)
        .collect::<Vec<_>>();
    browser_inventory_read_model_from_observations(
        generated_at,
        &observations,
        BrowserInventoryRowIdPrefix(constants::browser::INVENTORY_ROW_ID_PREFIX_WINDOWS),
        BrowserInventoryExecutablePathRef(
            constants::browser::INVENTORY_EXECUTABLE_PATH_REF_WINDOWS_REDACTED,
        ),
    )
}

pub fn browser_inventory_read_model_from_platform_inventory(
    generated_at: BrowserInventoryGeneratedAtText,
    observations: &[BrowserPlatformInventoryObservation],
) -> BrowserInventoryReadModel {
    browser_inventory_read_model_from_observations(
        generated_at,
        observations,
        BrowserInventoryRowIdPrefix(constants::browser::INVENTORY_ROW_ID_PREFIX_PLATFORM),
        BrowserInventoryExecutablePathRef(
            constants::browser::INVENTORY_EXECUTABLE_PATH_REF_PLATFORM_REDACTED,
        ),
    )
}

fn browser_inventory_read_model_from_observations(
    generated_at: BrowserInventoryGeneratedAtText,
    observations: &[BrowserPlatformInventoryObservation],
    row_id_prefix: BrowserInventoryRowIdPrefix,
    executable_path_ref: BrowserInventoryExecutablePathRef,
) -> BrowserInventoryReadModel {
    let rows = observations
        .iter()
        .enumerate()
        .map(|(index, observation)| {
            browser_inventory_row_from_observation(
                &generated_at,
                index,
                observation,
                row_id_prefix,
                executable_path_ref,
            )
        })
        .collect::<Vec<_>>();
    let returned = rows.len() as u64;
    let latest_observed_at = latest_observed_at(&generated_at, returned);

    BrowserInventoryReadModel {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at: generated_at.0,
        limit: returned,
        returned,
        latest_observed_at: latest_observed_at.map(|observed_at| observed_at.0),
        capability_status: None,
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        rows,
    }
}

fn browser_inventory_row_from_observation(
    generated_at: &BrowserInventoryGeneratedAtText,
    row_index: usize,
    observation: &BrowserPlatformInventoryObservation,
    row_id_prefix: BrowserInventoryRowIdPrefix,
    executable_path_ref: BrowserInventoryExecutablePathRef,
) -> BrowserInventoryRow {
    BrowserInventoryRow {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        inventory_row_id: inventory_row_id(observation, row_index, row_id_prefix).0,
        scanned_at: generated_at.0.clone(),
        device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        product_name: observation.product_name.clone(),
        browser_family: observation.browser_family,
        browser_channel: observation.browser_channel,
        browser_version: None,
        install_state: observation.install_state,
        running_state: observation.running_state,
        management_tier: observation.management_tier,
        support_tier: observation.support_tier,
        exact_url_capability: observation.exact_url_capability,
        active_tab_capability: observation.active_tab_capability,
        managed_profile_state: observation.managed_profile_state,
        unmanaged_fallback_capability: observation.unmanaged_fallback_capability,
        executable_path_ref: observation
            .executable_path
            .as_ref()
            .map(|_| executable_path_ref.0.to_string()),
        publisher_signature_ref: None,
        file_hash_ref: None,
        profile_id: None,
        process_id: observation.process_id,
        capability_status: observation.capability_status,
        reason_code: observation.reason_code.to_string(),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    }
}

fn latest_observed_at(
    generated_at: &BrowserInventoryGeneratedAtText,
    returned: u64,
) -> Option<BrowserInventoryGeneratedAtText> {
    if returned == 0 {
        return None;
    }
    Some(generated_at.clone())
}

fn inventory_row_id(
    observation: &BrowserPlatformInventoryObservation,
    row_index: usize,
    row_id_prefix: BrowserInventoryRowIdPrefix,
) -> BrowserInventoryRowIdText {
    let mut row_id = String::from(row_id_prefix.0);
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
    BrowserInventoryRowIdText(row_id)
}
