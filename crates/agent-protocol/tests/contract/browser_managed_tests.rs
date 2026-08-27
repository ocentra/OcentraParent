use super::{
    constants, BrowserBridgeKind, BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel,
    BrowserFamily, BrowserManagedProfileLifecycleState, BrowserManagedSessionStatus,
    BrowserManagedState, BrowserQueryVisibilityLabel, BrowserUnmanagedDetectionConfidence,
    BrowserUnmanagedDetectionReason, BrowserUnmanagedProcessKind, BROWSER_EVIDENCE_SCHEMA_VERSION,
};
use ocentra_eventing::expect_value::ExpectValue;

#[test]
fn browser_managed_status_serializes_to_contract_shape() {
    let status = BrowserManagedSessionStatus {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        checked_at: constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        managed_browser_session_id: Some(constants::browser::SESSION_ID_DEV.to_string()),
        browser_family: Some(BrowserFamily::UnknownChromium),
        browser_channel: Some(BrowserChannel::Unknown),
        browser_version: Some(constants::browser::DEVTOOLS_TEST_BROWSER_VERSION.to_string()),
        profile_id: Some(constants::browser::PROFILE_ID_DEV.to_string()),
        profile_path_ref: Some(constants::browser::PROFILE_PATH_REF_MANAGED.to_string()),
        profile_root_ref: Some(constants::browser::PROFILE_ROOT_REF_MANAGED.to_string()),
        profile_scope_id: Some(constants::browser::PROFILE_SCOPE_ID_DEV.to_string()),
        profile_lifecycle_state: Some(BrowserManagedProfileLifecycleState::Ready),
        policy_revision: Some(constants::browser::PROFILE_POLICY_REVISION_DEV.to_string()),
        process_id: Some(constants::browser::PROCESS_ID_UNKNOWN),
        bridge_kind: Some(BrowserBridgeKind::ChromiumDevtoolsProtocol),
        bridge_endpoint_ref: Some(
            constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS.to_string(),
        ),
        unmanaged_process_name: None,
        unmanaged_executable_path_ref: None,
        unmanaged_signature_ref: None,
        unmanaged_process_hash_ref: None,
        unmanaged_process_kind: None,
        unmanaged_detection_confidence: None,
        unmanaged_detection_reason: None,
        managed_state: BrowserManagedState::BridgeConnected,
        capability_status: BrowserCapabilityStatus::TabListOnly,
        degraded_reason: None,
        started_at: None,
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    };

    let serialized =
        serde_json::to_value(status).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized[constants::field::MANAGED_STATE],
        constants::browser::MANAGED_STATE_BRIDGE_CONNECTED
    );
    assert_eq!(
        serialized[constants::field::BRIDGE_KIND],
        constants::browser::BRIDGE_KIND_CHROMIUM_DEVTOOLS_PROTOCOL
    );
    assert_eq!(
        serialized[constants::field::BRIDGE_ENDPOINT_REF],
        constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS
    );
    assert_eq!(
        serialized[constants::field::PROFILE_ROOT_REF],
        constants::browser::PROFILE_ROOT_REF_MANAGED
    );
    assert_eq!(
        serialized[constants::field::PROFILE_LIFECYCLE_STATE],
        constants::browser::PROFILE_STORE_LIFECYCLE_READY
    );
}

#[test]
fn browser_managed_running_status_serializes_process_state() {
    let status = BrowserManagedSessionStatus {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        checked_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        managed_browser_session_id: Some(constants::browser::SESSION_ID_DEV.to_string()),
        browser_family: Some(BrowserFamily::Chrome),
        browser_channel: Some(BrowserChannel::Stable),
        browser_version: None,
        profile_id: Some(constants::browser::PROFILE_ID_DEV.to_string()),
        profile_path_ref: Some(constants::browser::PROFILE_PATH_REF_MANAGED.to_string()),
        profile_root_ref: Some(constants::browser::PROFILE_ROOT_REF_MANAGED.to_string()),
        profile_scope_id: Some(constants::browser::PROFILE_SCOPE_ID_DEV.to_string()),
        profile_lifecycle_state: Some(BrowserManagedProfileLifecycleState::Ready),
        policy_revision: Some(constants::browser::PROFILE_POLICY_REVISION_DEV.to_string()),
        process_id: Some(constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID),
        bridge_kind: Some(BrowserBridgeKind::ChromiumDevtoolsProtocol),
        bridge_endpoint_ref: Some(
            constants::browser::BRIDGE_ENDPOINT_REF_LOOPBACK_DEVTOOLS.to_string(),
        ),
        unmanaged_process_name: None,
        unmanaged_executable_path_ref: None,
        unmanaged_signature_ref: None,
        unmanaged_process_hash_ref: None,
        unmanaged_process_kind: None,
        unmanaged_detection_confidence: None,
        unmanaged_detection_reason: None,
        managed_state: BrowserManagedState::RunningManaged,
        capability_status: BrowserCapabilityStatus::BridgeMissing,
        degraded_reason: Some(constants::value::MANAGED_BROWSER_BRIDGE_CONNECT_PENDING.to_string()),
        started_at: Some(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string()),
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::LiveLocal,
    };

    let serialized =
        serde_json::to_value(status).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized[constants::field::MANAGED_STATE],
        constants::browser::MANAGED_STATE_RUNNING_MANAGED
    );
    assert_eq!(
        serialized[constants::field::PROCESS_ID],
        constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID
    );
    assert_eq!(
        serialized[constants::field::DEGRADED_REASON],
        constants::value::MANAGED_BROWSER_BRIDGE_CONNECT_PENDING
    );
}

#[test]
fn browser_unmanaged_status_serializes_process_only_fields() {
    let status = BrowserManagedSessionStatus {
        schema_version: BROWSER_EVIDENCE_SCHEMA_VERSION,
        checked_at: constants::activity_store::TEST_SECOND_OBSERVED_AT.to_string(),
        managed_browser_session_id: None,
        browser_family: Some(BrowserFamily::Chrome),
        browser_channel: Some(BrowserChannel::Stable),
        browser_version: None,
        profile_id: None,
        profile_path_ref: None,
        profile_root_ref: None,
        profile_scope_id: None,
        profile_lifecycle_state: None,
        policy_revision: None,
        process_id: Some(constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID),
        bridge_kind: None,
        bridge_endpoint_ref: None,
        unmanaged_process_name: Some(constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string()),
        unmanaged_executable_path_ref: Some(
            constants::browser::INVENTORY_EXECUTABLE_PATH_REF_WINDOWS_REDACTED.to_string(),
        ),
        unmanaged_signature_ref: Some(
            constants::browser::UNMANAGED_SIGNATURE_REF_WINDOWS_REDACTED.to_string(),
        ),
        unmanaged_process_hash_ref: Some(
            constants::browser::UNMANAGED_PROCESS_HASH_REF_WINDOWS_REDACTED.to_string(),
        ),
        unmanaged_process_kind: Some(BrowserUnmanagedProcessKind::SupportedBrowser),
        unmanaged_detection_confidence: Some(BrowserUnmanagedDetectionConfidence::High),
        unmanaged_detection_reason: Some(
            BrowserUnmanagedDetectionReason::SupportedBrowserOutsideManagedSession,
        ),
        managed_state: BrowserManagedState::InstalledSupported,
        capability_status: BrowserCapabilityStatus::UnmanagedBrowser,
        degraded_reason: Some(constants::value::MANAGED_BROWSER_UNMANAGED_PROCESS.to_string()),
        started_at: None,
        custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        query_visibility: BrowserQueryVisibilityLabel::Unavailable,
    };

    let serialized =
        serde_json::to_value(status).expect_value(constants::error::AGENT_EVENT_SERIALIZES);

    assert_eq!(
        serialized[constants::field::UNMANAGED_PROCESS_NAME],
        constants::browser::EXECUTABLE_CHROME_WINDOWS
    );
    assert_eq!(
        serialized[constants::field::UNMANAGED_PROCESS_KIND],
        constants::browser::UNMANAGED_PROCESS_KIND_SUPPORTED_BROWSER
    );
    assert_eq!(serialized.get(constants::field::URL), None);
    assert_eq!(serialized.get(constants::field::TAB_ID), None);
}
