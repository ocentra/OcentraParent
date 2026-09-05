#![forbid(unsafe_code)]

#[path = "../../src/browser_payload.rs"]
mod browser_payload;
#[path = "../../src/browser_runtime_status.rs"]
mod browser_runtime_status;
#[path = "../../src/fields.rs"]
mod fields;

use ocentra_parent_agent_protocol::{
    browser::{BrowserCapabilityStatus, BrowserCustodyLabel, BROWSER_EVIDENCE_SCHEMA_VERSION},
    browser_inventory::BrowserInventoryReadModel,
    browser_managed::{BrowserManagedState, BrowserQueryVisibilityLabel},
    constants,
    logging::LogFieldValue,
};

#[test]
fn browser_runtime_status_preserves_the_production_error_boundary() {
    let error_status = browser_runtime_status::status_with_error(
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        constants::value::MANAGED_BROWSER_PROFILE_DIR_MISSING,
    );
    assert_eq!(error_status.managed_state, BrowserManagedState::Error);
    assert_eq!(
        error_status.capability_status,
        BrowserCapabilityStatus::AdapterError
    );
}

#[test]
fn browser_runtime_payloads_serialize_status_and_empty_inventory_fields() {
    let error = browser_runtime_status::status_with_error(
        constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        constants::value::MANAGED_BROWSER_LAUNCH_ERROR,
    );
    let status_payload = browser_payload::browser_managed_status_payload(&error);
    assert_eq!(
        status_payload.get(constants::field::MANAGED_STATE),
        Some(&LogFieldValue::String(
            constants::browser::MANAGED_STATE_ERROR.to_string()
        ))
    );
    assert_eq!(
        status_payload.get(constants::field::BROWSER_VERSION),
        Some(&LogFieldValue::Null(()))
    );
    assert_eq!(
        status_payload.get(constants::field::UNMANAGED_PROCESS_NAME),
        Some(&LogFieldValue::Null(()))
    );

    let read_model = BrowserInventoryReadModel {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        limit: 10,
        returned: 0,
        latest_observed_at: None,
        capability_status: None,
        custody_label: BrowserCustodyLabel::Unavailable,
        query_visibility: BrowserQueryVisibilityLabel::Unavailable,
        rows: Vec::new(),
    };
    let inventory_payload = browser_payload::browser_inventory_read_model_payload(&read_model);
    assert_eq!(
        inventory_payload.get(constants::field::RETURNED),
        Some(&LogFieldValue::Number(0.0))
    );
    assert_eq!(
        inventory_payload.get(constants::field::BROWSER_FAMILY),
        Some(&LogFieldValue::Null(()))
    );
}
