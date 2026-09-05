use ocentra_parent_agent_core::browser_managed_discovery::BrowserUnmanagedProcessObservation;
use ocentra_parent_agent_protocol::browser::{
    BrowserCapabilityStatus, BrowserChannel, BrowserFamily,
};
use ocentra_parent_agent_protocol::browser_managed::{
    BrowserManagedState, BrowserUnmanagedDetectionConfidence, BrowserUnmanagedDetectionReason,
    BrowserUnmanagedProcessKind,
};
use ocentra_parent_agent_protocol::constants;
use ocentra_parent_agent_protocol::logging::LogFieldValue;

use crate::{
    browser_payload::browser_managed_status_payload,
    browser_runtime_status::{missing_browser_status, unmanaged_browser_status},
};

#[test]
fn missing_browser_status_reports_typed_degraded_state() {
    let status =
        missing_browser_status(constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string());
    let payload = browser_managed_status_payload(&status);

    assert_eq!(status.managed_state, BrowserManagedState::NotInstalled);
    assert_eq!(
        crate::test_log_field::log_field(
            &payload,
            constants::field::MANAGED_STATE,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::MANAGED_STATE_NOT_INSTALLED.to_string())
    );
    assert_eq!(
        crate::test_log_field::log_field(
            &payload,
            constants::field::REASON,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::value::MANAGED_BROWSER_EXECUTABLE_MISSING.to_string())
    );
    assert_eq!(
        crate::test_log_field::log_field(
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
        crate::test_log_field::log_field(
            &payload,
            constants::field::BROWSER_FAMILY,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::FAMILY_CHROME.to_string())
    );
    assert_eq!(
        crate::test_log_field::log_field(
            &payload,
            constants::field::REASON,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::value::MANAGED_BROWSER_UNMANAGED_PROCESS.to_string())
    );
    assert_eq!(
        crate::test_log_field::log_field(
            &payload,
            constants::field::UNMANAGED_PROCESS_NAME,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(constants::browser::EXECUTABLE_CHROME_WINDOWS.to_string())
    );
    assert_eq!(
        crate::test_log_field::log_field(
            &payload,
            constants::field::UNMANAGED_PROCESS_KIND,
            constants::error::AGENT_EVENT_SERIALIZES
        ),
        LogFieldValue::String(
            constants::browser::UNMANAGED_PROCESS_KIND_SUPPORTED_BROWSER.to_string()
        )
    );
    assert_eq!(
        crate::test_log_field::log_field(&payload, constants::field::UNMANAGED_DETECTION_REASON, constants::error::AGENT_EVENT_SERIALIZES),
        LogFieldValue::String(
            constants::browser::UNMANAGED_DETECTION_REASON_SUPPORTED_BROWSER_OUTSIDE_MANAGED_SESSION
                .to_string()
        )
    );
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
