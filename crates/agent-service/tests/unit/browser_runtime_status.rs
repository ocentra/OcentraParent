#![forbid(unsafe_code)]

#[path = "../../src/browser_payload.rs"]
mod browser_payload;
#[path = "../../src/browser_runtime_status.rs"]
mod browser_runtime_status;
#[path = "../../src/fields.rs"]
mod fields;

use ocentra_parent_agent_core::browser_managed_discovery::BrowserUnmanagedProcessObservation;
use ocentra_parent_agent_protocol::{
    browser::{
        BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel, BrowserFamily,
        BROWSER_EVIDENCE_SCHEMA_VERSION,
    },
    browser_inventory::BrowserInventoryReadModel,
    browser_managed::{
        BrowserManagedProfileLifecycleState, BrowserManagedState, BrowserQueryVisibilityLabel,
        BrowserUnmanagedDetectionConfidence, BrowserUnmanagedDetectionReason,
        BrowserUnmanagedProcessKind,
    },
    constants,
    logging::LogFieldValue,
};

#[test]
fn browser_runtime_status_variants_preserve_managed_and_unmanaged_details() {
    let profile_missing = browser_runtime_status::profile_missing_status(
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
    );
    assert_eq!(
        profile_missing.managed_state,
        BrowserManagedState::InstalledSupported
    );
    assert_eq!(
        profile_missing.profile_lifecycle_state,
        Some(BrowserManagedProfileLifecycleState::Missing)
    );

    let unmanaged = browser_runtime_status::unmanaged_browser_status(
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        unmanaged_process(),
    );
    assert_eq!(
        unmanaged.capability_status,
        BrowserCapabilityStatus::UnmanagedBrowser
    );
    assert_eq!(unmanaged.process_id, Some(417));
    assert_eq!(
        unmanaged.unmanaged_process_kind,
        Some(BrowserUnmanagedProcessKind::SupportedBrowser)
    );

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

fn unmanaged_process() -> BrowserUnmanagedProcessObservation {
    BrowserUnmanagedProcessObservation {
        process_id: 417,
        process_name: constants::browser::SESSION_ID_DEV.to_string(),
        executable_path_ref: Some(constants::browser::PROFILE_PATH_REF_MANAGED.to_string()),
        signature_ref: Some(constants::browser::PROFILE_SCOPE_ID_DEV.to_string()),
        process_hash_ref: Some(constants::browser::PROFILE_POLICY_REVISION_DEV.to_string()),
        browser_family: BrowserFamily::UnknownChromium,
        browser_channel: BrowserChannel::Unknown,
        process_kind: BrowserUnmanagedProcessKind::SupportedBrowser,
        detection_confidence: BrowserUnmanagedDetectionConfidence::High,
        detection_reason: BrowserUnmanagedDetectionReason::SupportedBrowserOutsideManagedSession,
    }
}
