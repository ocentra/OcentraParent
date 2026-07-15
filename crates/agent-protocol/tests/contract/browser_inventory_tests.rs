use super::{
    constants, BrowserActiveTabCapability, BrowserCapabilityStatus, BrowserChannel,
    BrowserCustodyLabel, BrowserExactUrlCapability, BrowserFamily, BrowserInventoryInstallState,
    BrowserInventoryReadModel, BrowserInventoryRow, BrowserInventoryRunningState,
    BrowserManagedProfileState, BrowserManagementTier, BrowserQueryVisibilityLabel,
    BrowserSupportTier, BrowserUnmanagedFallbackCapability, BROWSER_EVIDENCE_SCHEMA_VERSION,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn browser_inventory_read_model_serializes_managed_target_list_boundary() {
    let read_model = BrowserInventoryReadModel {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        generated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        limit: constants::activity_store::DEFAULT_RECENT_LIMIT,
        returned: 1,
        latest_observed_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        capability_status: Some(BrowserCapabilityStatus::TabListOnly),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
        rows: vec![managed_edge_inventory_row()],
    };

    let serialized =
        serde_json::to_value(read_model).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(serialized["schemaVersion"], BROWSER_EVIDENCE_SCHEMA_VERSION);
    assert_eq!(
        serialized["rows"][0]["inventoryRowId"],
        constants::browser::INVENTORY_ROW_ID_EDGE_STABLE
    );
    assert_eq!(
        serialized["rows"][0]["exactUrlCapability"],
        constants::browser::EXACT_URL_CAPABILITY_MANAGED_TARGET_LIST_ONLY
    );
    assert_eq!(
        serialized["rows"][0]["activeTabCapability"],
        constants::browser::ACTIVE_TAB_CAPABILITY_TARGET_LIST_ONLY
    );
    assert_eq!(
        serialized["rows"][0]["publisherSignatureRef"],
        serde_json::Value::Null
    );
    assert_eq!(
        serialized["rows"][0]["fileHashRef"],
        serde_json::Value::Null
    );
}

#[test]
fn browser_inventory_rows_keep_unmanaged_exact_url_unclaimed() {
    let row = unmanaged_chrome_inventory_row();

    assert!(row.claim_boundary_is_honest());
    assert_eq!(
        row.exact_url_capability.as_protocol_str(),
        constants::browser::EXACT_URL_CAPABILITY_NOT_CLAIMED
    );
    assert_eq!(
        row.management_tier.as_protocol_str(),
        constants::browser::MANAGEMENT_TIER_UNMANAGED
    );
    assert_eq!(
        row.publisher_signature_ref.as_deref(),
        Some(constants::browser::INVENTORY_PUBLISHER_SIGNATURE_REF_WINDOWS_REDACTED)
    );
    assert_eq!(
        row.file_hash_ref.as_deref(),
        Some(constants::browser::INVENTORY_FILE_HASH_REF_WINDOWS_REDACTED)
    );
}

#[test]
fn browser_inventory_rows_detect_dishonest_unmanaged_exact_url_claims() {
    let mut row = unmanaged_chrome_inventory_row();
    row.exact_url_capability = BrowserExactUrlCapability::ManagedExactUrlAvailable;
    row.active_tab_capability = BrowserActiveTabCapability::KnownActiveSupported;

    assert!(!row.claim_boundary_is_honest());
}

fn managed_edge_inventory_row() -> BrowserInventoryRow {
    BrowserInventoryRow {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        inventory_row_id: constants::browser::INVENTORY_ROW_ID_EDGE_STABLE.to_string(),
        scanned_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        browser_family: BrowserFamily::Edge,
        browser_channel: BrowserChannel::Stable,
        product_name: constants::browser::PRODUCT_NAME_MICROSOFT_EDGE.to_string(),
        browser_version: Some(constants::browser::DEVTOOLS_TEST_BROWSER_VERSION.to_string()),
        install_state: BrowserInventoryInstallState::Installed,
        running_state: BrowserInventoryRunningState::RunningManaged,
        management_tier: BrowserManagementTier::Managed,
        support_tier: BrowserSupportTier::ManagedTargetList,
        exact_url_capability: BrowserExactUrlCapability::ManagedTargetListOnly,
        active_tab_capability: BrowserActiveTabCapability::TargetListOnly,
        managed_profile_state: BrowserManagedProfileState::Ready,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::OsBlockManualRequired,
        executable_path_ref: Some(constants::browser::DEVTOOLS_TEST_MSEDGE_BETA_PATH.to_string()),
        publisher_signature_ref: None,
        file_hash_ref: None,
        profile_id: Some(constants::browser::PROFILE_ID_DEV.to_string()),
        process_id: Some(constants::browser::PROCESS_ID_UNKNOWN),
        capability_status: BrowserCapabilityStatus::TabListOnly,
        reason_code: constants::browser::INVENTORY_REASON_MANAGED_TARGET_LIST_ACTIVE_TAB_UNPROVED
            .to_string(),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    }
}

fn unmanaged_chrome_inventory_row() -> BrowserInventoryRow {
    BrowserInventoryRow {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        inventory_row_id: constants::browser::INVENTORY_ROW_ID_UNMANAGED_CHROME.to_string(),
        scanned_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        device_id: constants::peer::LOCAL_DEV_AGENT.to_string(),
        browser_family: BrowserFamily::Chrome,
        browser_channel: BrowserChannel::Stable,
        product_name: constants::browser::PRODUCT_NAME_GOOGLE_CHROME.to_string(),
        browser_version: None,
        install_state: BrowserInventoryInstallState::CandidateRunning,
        running_state: BrowserInventoryRunningState::RunningUnmanaged,
        management_tier: BrowserManagementTier::Unmanaged,
        support_tier: BrowserSupportTier::UnmanagedProcessOnly,
        exact_url_capability: BrowserExactUrlCapability::NotClaimed,
        active_tab_capability: BrowserActiveTabCapability::NotClaimed,
        managed_profile_state: BrowserManagedProfileState::NotApplicable,
        unmanaged_fallback_capability: BrowserUnmanagedFallbackCapability::ReportOnly,
        executable_path_ref: Some(constants::browser::DEVTOOLS_TEST_EXECUTABLE_PATH.to_string()),
        publisher_signature_ref: Some(
            constants::browser::INVENTORY_PUBLISHER_SIGNATURE_REF_WINDOWS_REDACTED.to_string(),
        ),
        file_hash_ref: Some(
            constants::browser::INVENTORY_FILE_HASH_REF_WINDOWS_REDACTED.to_string(),
        ),
        profile_id: None,
        process_id: Some(constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID),
        capability_status: BrowserCapabilityStatus::UnmanagedBrowser,
        reason_code: constants::browser::INVENTORY_REASON_UNMANAGED_BROWSER_PROCESS_ONLY
            .to_string(),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::Unavailable,
    }
}
