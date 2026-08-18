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
        BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry, BrowserManagedState,
        BrowserQueryVisibilityLabel, BrowserUnmanagedDetectionConfidence,
        BrowserUnmanagedDetectionReason, BrowserUnmanagedProcessKind,
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

    let ready = browser_runtime_status::managed_profile_ready_status(
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        BrowserFamily::UnknownChromium,
        BrowserChannel::Unknown,
        profile_store_entry(),
    );
    assert_eq!(
        ready.managed_state,
        BrowserManagedState::ManagedProfileReady
    );
    assert_eq!(
        ready.profile_id,
        Some(constants::browser::PROFILE_ID_DEV.to_string())
    );

    let running = browser_runtime_status::running_managed_status(
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        managed_launch(),
        profile_store_entry(),
        constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
    );
    assert_eq!(running.managed_state, BrowserManagedState::RunningManaged);
    assert_eq!(running.process_id, Some(418));
    assert_eq!(
        running.bridge_endpoint_ref,
        Some(constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS.to_string())
    );

    let disconnected = browser_runtime_status::bridge_disconnected_status(
        constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        constants::value::MANAGED_BROWSER_PROFILE_DIR_MISSING,
    );
    assert_eq!(
        disconnected.managed_state,
        BrowserManagedState::BridgeDisconnected
    );
    assert_eq!(
        disconnected.capability_status,
        BrowserCapabilityStatus::Stale
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
    let connected = browser_runtime_status::connected_status(
        constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        Some(constants::browser::SESSION_ID_DEV.to_string()),
        BrowserCapabilityStatus::Available,
        None::<String>,
    );
    let status_payload = browser_payload::browser_managed_status_payload(&connected);
    assert_eq!(
        status_payload.get(constants::field::MANAGED_STATE),
        Some(&LogFieldValue::String(
            constants::browser::MANAGED_STATE_BRIDGE_CONNECTED.to_string()
        ))
    );
    assert_eq!(
        status_payload.get(constants::field::BROWSER_VERSION),
        Some(&LogFieldValue::String(
            constants::browser::SESSION_ID_DEV.to_string()
        ))
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

fn profile_store_entry() -> BrowserManagedProfileStoreEntry {
    BrowserManagedProfileStoreEntry {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        profile_path_ref: constants::browser::PROFILE_PATH_REF_MANAGED.to_string(),
        profile_root_ref: constants::browser::PROFILE_ROOT_REF_MANAGED.to_string(),
        profile_scope_id: constants::browser::PROFILE_SCOPE_ID_DEV.to_string(),
        device_id: constants::browser::SESSION_ID_DEV.to_string(),
        browser_family: BrowserFamily::UnknownChromium,
        browser_channel: BrowserChannel::Unknown,
        lifecycle_state: BrowserManagedProfileLifecycleState::Ready,
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        policy_revision: constants::browser::PROFILE_POLICY_REVISION_DEV.to_string(),
        created_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        updated_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        missing_since: None,
        repaired_at: None,
        deleted_at: None,
        repair_reason: None,
    }
}

fn managed_launch() -> browser_runtime_status::BrowserManagedLaunchStatus {
    browser_runtime_status::BrowserManagedLaunchStatus::new(
        418,
        BrowserFamily::UnknownChromium,
        BrowserChannel::Unknown,
        browser_runtime_status::BrowserRuntimeText::from(
            constants::browser::PROFILE_PATH_REF_MANAGED,
        ),
        browser_runtime_status::BrowserRuntimeText::from(
            constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS,
        ),
    )
}
