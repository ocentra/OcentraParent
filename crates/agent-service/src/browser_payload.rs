use ocentra_parent_agent_protocol::browser_inventory::{
    BrowserInventoryReadModel, BrowserInventoryRow,
};
use ocentra_parent_agent_protocol::browser_managed::BrowserManagedSessionStatus;
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::{LogFieldValue, LogFields};

use crate::fields::fields_from_pairs;

#[derive(Default)]
struct PayloadPairs(Vec<(&'static str, LogFieldValue)>);

impl PayloadPairs {
    fn extend(&mut self, other: PayloadPairs) {
        self.0.extend(other.0);
    }
}

fn optional_text<T: ToString>(value: Option<T>) -> LogFieldValue {
    match value {
        Some(text) => LogFieldValue::String(text.to_string()),
        None => LogFieldValue::Null(()),
    }
}

pub fn browser_managed_status_payload(status: &BrowserManagedSessionStatus) -> LogFields {
    let mut pairs = browser_managed_identity_pairs(status);
    pairs.extend(browser_managed_unmanaged_process_pairs(status));
    pairs.extend(browser_managed_state_pairs(status));
    if let Ok(serialized) = serde_json::to_string(status) {
        pairs.0.push((
            constants::field::BROWSER_MANAGED_STATUS_JSON,
            LogFieldValue::String(serialized),
        ));
    }
    fields_from_pairs(pairs.0)
}

pub fn browser_inventory_read_model_payload(read_model: &BrowserInventoryReadModel) -> LogFields {
    let latest = read_model.rows.first();
    let mut pairs = browser_inventory_read_model_pairs(read_model);
    pairs.extend(browser_inventory_latest_identity_pairs(latest));
    pairs.extend(browser_inventory_latest_state_pairs(latest));
    fields_from_pairs(pairs.0)
}

fn browser_inventory_read_model_pairs(read_model: &BrowserInventoryReadModel) -> PayloadPairs {
    PayloadPairs(vec![
        (
            constants::field::GENERATED_AT,
            LogFieldValue::String(read_model.generated_at.clone()),
        ),
        (
            constants::field::LIMIT,
            LogFieldValue::Number(read_model.limit as f64),
        ),
        (
            constants::field::RETURNED,
            LogFieldValue::Number(read_model.returned as f64),
        ),
        (
            constants::field::LATEST_OBSERVED_AT,
            optional_text(read_model.latest_observed_at.clone()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            optional_text(
                read_model
                    .capability_status
                    .as_ref()
                    .map(|status| status.as_protocol_str()),
            ),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(read_model.custody_label.as_protocol_str().to_string()),
        ),
        (
            constants::field::QUERY_VISIBILITY,
            LogFieldValue::String(read_model.query_visibility.as_protocol_str().to_string()),
        ),
    ])
}

fn browser_inventory_latest_identity_pairs(row: Option<&BrowserInventoryRow>) -> PayloadPairs {
    PayloadPairs(vec![
        (
            constants::field::BROWSER_INVENTORY_ROW_ID,
            optional_text(row.map(|value| value.inventory_row_id.clone())),
        ),
        (
            constants::field::BROWSER_FAMILY,
            optional_text(row.map(|value| value.browser_family.as_protocol_str())),
        ),
        (
            constants::field::BROWSER_CHANNEL,
            optional_text(row.map(|value| value.browser_channel.as_protocol_str())),
        ),
        (
            constants::field::PRODUCT_NAME,
            optional_text(row.map(|value| value.product_name.clone())),
        ),
        (
            constants::field::BROWSER_VERSION,
            optional_text(row.and_then(|value| value.browser_version.clone())),
        ),
        (
            constants::field::PROFILE_ID,
            optional_text(row.and_then(|value| value.profile_id.clone())),
        ),
        (
            constants::field::PROCESS_ID,
            optional_u32(row.and_then(|value| value.process_id)),
        ),
        (
            constants::field::EXECUTABLE_PATH_REF,
            optional_text(row.and_then(|value| value.executable_path_ref.clone())),
        ),
        (
            constants::field::PUBLISHER_SIGNATURE_REF,
            optional_text(row.and_then(|value| value.publisher_signature_ref.clone())),
        ),
        (
            constants::field::FILE_HASH_REF,
            optional_text(row.and_then(|value| value.file_hash_ref.clone())),
        ),
    ])
}

fn browser_inventory_latest_state_pairs(row: Option<&BrowserInventoryRow>) -> PayloadPairs {
    PayloadPairs(vec![
        (
            constants::field::INSTALL_STATE,
            optional_text(row.map(|value| value.install_state.as_protocol_str())),
        ),
        (
            constants::field::RUNNING_STATE,
            optional_text(row.map(|value| value.running_state.as_protocol_str())),
        ),
        (
            constants::field::MANAGEMENT_TIER,
            optional_text(row.map(|value| value.management_tier.as_protocol_str())),
        ),
        (
            constants::field::SUPPORT_TIER,
            optional_text(row.map(|value| value.support_tier.as_protocol_str())),
        ),
        (
            constants::field::EXACT_URL_CAPABILITY,
            optional_text(row.map(|value| value.exact_url_capability.as_protocol_str())),
        ),
        (
            constants::field::ACTIVE_TAB_CAPABILITY,
            optional_text(row.map(|value| value.active_tab_capability.as_protocol_str())),
        ),
        (
            constants::field::MANAGED_PROFILE_STATE,
            optional_text(row.map(|value| value.managed_profile_state.as_protocol_str())),
        ),
        (
            constants::field::UNMANAGED_FALLBACK_CAPABILITY,
            optional_text(row.map(|value| value.unmanaged_fallback_capability.as_protocol_str())),
        ),
        (
            constants::field::REASON,
            optional_text(row.map(|value| value.reason_code.clone())),
        ),
    ])
}

fn browser_managed_identity_pairs(status: &BrowserManagedSessionStatus) -> PayloadPairs {
    let mut pairs = browser_managed_identity_core_pairs(status);
    pairs.extend(browser_managed_identity_bridge_pairs(status));
    pairs
}

fn browser_managed_identity_core_pairs(status: &BrowserManagedSessionStatus) -> PayloadPairs {
    PayloadPairs(vec![
        (
            constants::field::CHECKED_AT,
            LogFieldValue::String(status.checked_at.clone()),
        ),
        (
            constants::field::MANAGED_BROWSER_SESSION_ID,
            optional_text(status.managed_browser_session_id.clone()),
        ),
        (
            constants::field::BROWSER_FAMILY,
            optional_text(
                status
                    .browser_family
                    .as_ref()
                    .map(|family| family.as_protocol_str()),
            ),
        ),
        (
            constants::field::BROWSER_CHANNEL,
            optional_text(
                status
                    .browser_channel
                    .as_ref()
                    .map(|channel| channel.as_protocol_str()),
            ),
        ),
        (
            constants::field::BROWSER_VERSION,
            optional_text(status.browser_version.clone()),
        ),
        (
            constants::field::PROFILE_ID,
            optional_text(status.profile_id.clone()),
        ),
        (
            constants::field::PROFILE_PATH_REF,
            optional_text(status.profile_path_ref.clone()),
        ),
        (
            constants::field::PROFILE_ROOT_REF,
            optional_text(status.profile_root_ref.clone()),
        ),
        (
            constants::field::PROFILE_SCOPE_ID,
            optional_text(status.profile_scope_id.clone()),
        ),
        (
            constants::field::PROFILE_LIFECYCLE_STATE,
            optional_text(
                status
                    .profile_lifecycle_state
                    .as_ref()
                    .map(|state| state.as_protocol_str()),
            ),
        ),
        (
            constants::field::POLICY_REVISION,
            optional_text(status.policy_revision.clone()),
        ),
    ])
}

fn browser_managed_identity_bridge_pairs(status: &BrowserManagedSessionStatus) -> PayloadPairs {
    PayloadPairs(vec![
        (
            constants::field::PROCESS_ID,
            optional_u32(status.process_id),
        ),
        (
            constants::field::BRIDGE_KIND,
            optional_text(
                status
                    .bridge_kind
                    .as_ref()
                    .map(|bridge_kind| bridge_kind.as_protocol_str()),
            ),
        ),
        (
            constants::field::BRIDGE_ENDPOINT_REF,
            optional_text(status.bridge_endpoint_ref.clone()),
        ),
    ])
}

fn browser_managed_state_pairs(status: &BrowserManagedSessionStatus) -> PayloadPairs {
    PayloadPairs(vec![
        (
            constants::field::MANAGED_STATE,
            LogFieldValue::String(status.managed_state.as_protocol_str().to_string()),
        ),
        (
            constants::field::CAPABILITY_STATUS,
            LogFieldValue::String(status.capability_status.as_protocol_str().to_string()),
        ),
        (
            constants::field::REASON,
            optional_text(status.degraded_reason.clone()),
        ),
        (
            constants::field::STARTED_AT,
            optional_text(status.started_at.clone()),
        ),
        (
            constants::field::CUSTODY_LABEL,
            LogFieldValue::String(status.custody_label.as_protocol_str().to_string()),
        ),
        (
            constants::field::QUERY_VISIBILITY,
            LogFieldValue::String(status.query_visibility.as_protocol_str().to_string()),
        ),
    ])
}

fn browser_managed_unmanaged_process_pairs(status: &BrowserManagedSessionStatus) -> PayloadPairs {
    PayloadPairs(vec![
        (
            constants::field::UNMANAGED_PROCESS_NAME,
            optional_text(status.unmanaged_process_name.clone()),
        ),
        (
            constants::field::UNMANAGED_EXECUTABLE_PATH_REF,
            optional_text(status.unmanaged_executable_path_ref.clone()),
        ),
        (
            constants::field::UNMANAGED_SIGNATURE_REF,
            optional_text(status.unmanaged_signature_ref.clone()),
        ),
        (
            constants::field::UNMANAGED_PROCESS_HASH_REF,
            optional_text(status.unmanaged_process_hash_ref.clone()),
        ),
        (
            constants::field::UNMANAGED_PROCESS_KIND,
            optional_text(
                status
                    .unmanaged_process_kind
                    .as_ref()
                    .map(|kind| kind.as_protocol_str()),
            ),
        ),
        (
            constants::field::UNMANAGED_DETECTION_CONFIDENCE,
            optional_text(
                status
                    .unmanaged_detection_confidence
                    .as_ref()
                    .map(|confidence| confidence.as_protocol_str()),
            ),
        ),
        (
            constants::field::UNMANAGED_DETECTION_REASON,
            optional_text(
                status
                    .unmanaged_detection_reason
                    .as_ref()
                    .map(|reason| reason.as_protocol_str()),
            ),
        ),
    ])
}

fn optional_u32(value: Option<u32>) -> LogFieldValue {
    match value {
        Some(number) => LogFieldValue::Number(number as f64),
        None => LogFieldValue::Null(()),
    }
}
