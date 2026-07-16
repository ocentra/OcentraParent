use ocentra_parent_agent_core::browser_managed_discovery::BrowserUnmanagedProcessObservation;
use ocentra_parent_agent_core::browser_managed_session::BrowserManagedLaunch;
use ocentra_parent_agent_protocol as parent_protocol;
use ocentra_parent_agent_protocol::browser::{
    BrowserCapabilityStatus, BrowserChannel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_managed::{
    BrowserManagedProfileLifecycleState, BrowserManagedProfileStoreEntry, BrowserManagedState,
    BrowserQueryVisibilityLabel, BrowserUnmanagedDetectionConfidence,
    BrowserUnmanagedDetectionReason, BrowserUnmanagedProcessKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{
    browser_payload::browser_managed_status_payload,
    browser_runtime_status::{
        bridge_disconnected_status, managed_profile_ready_status, missing_browser_status,
        running_managed_status, unmanaged_browser_status,
    },
};

#[test]
fn missing_browser_status_reports_typed_degraded_state() {
    let status =
        missing_browser_status(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string());
    let payload = browser_managed_status_payload(&status);

    assert_eq!(status.managed_state, BrowserManagedState::NotInstalled);
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::MANAGED_STATE,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::MANAGED_STATE_NOT_INSTALLED.to_string())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::REASON,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::value::MANAGED_BROWSER_EXECUTABLE_MISSING.to_string())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::QUERY_VISIBILITY,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::QUERY_VISIBILITY_UNAVAILABLE.to_string())
    );
}

#[test]
fn unmanaged_browser_status_reports_discoverable_but_unmanaged_process() {
    let status = unmanaged_browser_status(
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        unmanaged_process_observation(),
    );
    let payload = browser_managed_status_payload(&status);

    assert_eq!(
        status.managed_state,
        BrowserManagedState::InstalledSupported
    );
    assert_eq!(
        status.capability_status,
        BrowserCapabilityStatus::UnmanagedBrowser
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::BROWSER_FAMILY,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::FAMILY_CHROME.to_string())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::REASON,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::value::MANAGED_BROWSER_UNMANAGED_PROCESS.to_string())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::UNMANAGED_PROCESS_NAME,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::UNMANAGED_PROCESS_KIND,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(
            constants::browser::UNMANAGED_PROCESS_KIND_SUPPORTED_BROWSER.to_string()
        )
    );
    assert_eq!(
        crate::test_invariants::log_field(&payload, constants::field::UNMANAGED_DETECTION_REASON, constants::error::AGENT_EVENT_SERIALIZES),
        LogFieldValue::String(
            constants::browser::UNMANAGED_DETECTION_REASON_SUPPORTED_BROWSER_OUTSIDE_MANAGED_SESSION
                .to_string()
        )
    );
}

#[test]
fn managed_profile_ready_status_is_headless_until_explicit_launch() {
    let status = managed_profile_ready_status(
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        BrowserFamily::Chrome,
        BrowserChannel::Stable,
        profile_store_entry(),
    );
    let payload = browser_managed_status_payload(&status);

    assert_eq!(
        status.managed_state,
        BrowserManagedState::ManagedProfileReady
    );
    assert_eq!(
        status.capability_status,
        BrowserCapabilityStatus::BridgeMissing
    );
    assert_eq!(
        status.query_visibility,
        BrowserQueryVisibilityLabel::LiveLocal
    );
    assert_eq!(status.process_id, None);
    assert_eq!(status.started_at, None);
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::MANAGED_STATE,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::MANAGED_STATE_MANAGED_PROFILE_READY.to_string())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::PROCESS_ID,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::Null(())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::STARTED_AT,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::Null(())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::PROFILE_ROOT_REF,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::PROFILE_ROOT_REF_MANAGED.to_string())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::PROFILE_LIFECYCLE_STATE,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::PROFILE_STORE_LIFECYCLE_READY.to_string())
    );
}

#[test]
fn running_managed_status_tracks_process_without_claiming_bridge_connected() {
    let status = running_managed_status(
        constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        launch_result(),
        profile_store_entry(),
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
    );
    let payload = browser_managed_status_payload(&status);

    assert_eq!(status.managed_state, BrowserManagedState::RunningManaged);
    assert_eq!(
        status.capability_status,
        BrowserCapabilityStatus::BridgeMissing
    );
    assert_eq!(
        status.process_id,
        Some(constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID)
    );
    assert_eq!(
        status.degraded_reason,
        Some(constants::value::MANAGED_BROWSER_BRIDGE_CONNECT_PENDING.to_string())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::MANAGED_STATE,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::MANAGED_STATE_RUNNING_MANAGED.to_string())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::STARTED_AT,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string())
    );
}

#[test]
fn bridge_disconnected_status_reports_stale_bridge_state() {
    let status = bridge_disconnected_status(
        constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        constants::value::BROWSER_BRIDGE_STALE_SESSION,
    );
    let payload = browser_managed_status_payload(&status);

    assert_eq!(
        status.managed_state,
        BrowserManagedState::BridgeDisconnected
    );
    assert_eq!(status.capability_status, BrowserCapabilityStatus::Stale);
    assert_eq!(
        status.degraded_reason,
        Some(constants::value::BROWSER_BRIDGE_STALE_SESSION.to_string())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::CAPABILITY_STATUS,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::CAPABILITY_STATUS_STALE.to_string())
    );
    assert_eq!(
        crate::test_invariants::log_field(
            &payload,
            constants::field::QUERY_VISIBILITY,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::QUERY_VISIBILITY_LIVE_LOCAL.to_string())
    );
}

fn launch_result() -> BrowserManagedLaunch {
    BrowserManagedLaunch {
        process_id: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
        bridge_port: constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
        browser_family: BrowserFamily::Chrome,
        browser_channel: BrowserChannel::Stable,
        profile_path_ref: constants::browser::PROFILE_PATH_REF_MANAGED.to_string(),
        bridge_endpoint_ref: constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS.to_string(),
    }
}

fn profile_store_entry() -> BrowserManagedProfileStoreEntry {
    BrowserManagedProfileStoreEntry {
        schema_version: parent_protocol::BROWSER_EVIDENCE_SCHEMA_VERSION,
        profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
        profile_path_ref: constants::browser::PROFILE_PATH_REF_MANAGED.to_string(),
        profile_root_ref: constants::browser::PROFILE_ROOT_REF_MANAGED.to_string(),
        profile_scope_id: constants::browser::PROFILE_SCOPE_ID_DEV.to_string(),
        device_id: constants::browser::PROFILE_STORE_TEST_DEVICE_ID.to_string(),
        browser_family: BrowserFamily::Chrome,
        browser_channel: BrowserChannel::Stable,
        lifecycle_state: BrowserManagedProfileLifecycleState::Ready,
        custody_label:
            ocentra_parent_agent_protocol::browser::BrowserCustodyLabel::ChildDeviceLocal,
        policy_revision: constants::browser::PROFILE_POLICY_REVISION_DEV.to_string(),
        created_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        updated_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        missing_since: None,
        repaired_at: None,
        deleted_at: None,
        repair_reason: None,
    }
}

fn unmanaged_process_observation() -> BrowserUnmanagedProcessObservation {
    BrowserUnmanagedProcessObservation {
        process_id: constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
        process_name: constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string(),
        executable_path_ref: Some(
            constants::browser::INVENTORY_EXECUTABLE_PATH_REF_WINDOWS_REDACTED.to_string(),
        ),
        signature_ref: None,
        process_hash_ref: None,
        browser_family: BrowserFamily::Chrome,
        browser_channel: BrowserChannel::Stable,
        process_kind: BrowserUnmanagedProcessKind::SupportedBrowser,
        detection_confidence: BrowserUnmanagedDetectionConfidence::High,
        detection_reason: BrowserUnmanagedDetectionReason::SupportedBrowserOutsideManagedSession,
    }
}
