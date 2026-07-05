#![forbid(unsafe_code)]

#[path = "../support/fields.rs"]
mod fields;

#[test]
fn browser_runtime_harness_links_status_variants_and_inventory_payload() {
    let profile_missing = browser_runtime_status::profile_missing_status(
        ocentra_parent_agent_protocol::constants::activity_store::TEST_FIRST_OBSERVED_AT
            .to_string(),
    );
    assert_eq!(
        profile_missing.managed_state,
        ocentra_parent_agent_protocol::browser_managed::BrowserManagedState::InstalledSupported
    );

    let error_status = browser_runtime_status::status_with_error(
        ocentra_parent_agent_protocol::constants::activity_store::TEST_FIRST_OBSERVED_AT
            .to_string(),
        ocentra_parent_agent_protocol::constants::value::MANAGED_BROWSER_PROFILE_DIR_MISSING,
    );
    assert_eq!(
        error_status.capability_status,
        ocentra_parent_agent_protocol::browser::BrowserCapabilityStatus::AdapterError
    );

    let connected = browser_runtime_status::connected_status(
        ocentra_parent_agent_protocol::constants::activity_store::TEST_SECOND_OBSERVED_AT
            .to_string(),
        Some("125.0.0.1".to_string()),
        ocentra_parent_agent_protocol::browser::BrowserCapabilityStatus::Available,
        None,
    );
    assert_eq!(
        connected.managed_state,
        ocentra_parent_agent_protocol::browser_managed::BrowserManagedState::BridgeConnected
    );

    let read_model = ocentra_parent_agent_protocol::browser_inventory::BrowserInventoryReadModel {
        schema_version: ocentra_parent_agent_protocol::browser::BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at:
            ocentra_parent_agent_protocol::constants::activity_store::TEST_FIRST_OBSERVED_AT
                .to_string(),
        limit: 10,
        returned: 0,
        latest_observed_at: None,
        capability_status: None,
        custody_label: ocentra_parent_agent_protocol::browser::BrowserCustodyLabel::Unavailable,
        query_visibility:
            ocentra_parent_agent_protocol::browser_managed::BrowserQueryVisibilityLabel::Unavailable,
        rows: Vec::new(),
    };
    let payload = browser_payload::browser_inventory_read_model_payload(&read_model);
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            ocentra_parent_agent_protocol::constants::field::RETURNED,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        ocentra_parent_agent_protocol::logging::LogFieldValue::Number(0.0)
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            ocentra_parent_agent_protocol::constants::field::BROWSER_FAMILY,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        ocentra_parent_agent_protocol::logging::LogFieldValue::Null(())
    );
}
