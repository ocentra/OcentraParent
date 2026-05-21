use ocentra_parent_agent_protocol::{
    constants, BrowserCapabilityStatus, BrowserChannel, BrowserFamily, BrowserManagedState,
    LogFieldValue,
};

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
        payload[constants::field::MANAGED_STATE],
        LogFieldValue::String(constants::browser::MANAGED_STATE_NOT_INSTALLED.to_string())
    );
    assert_eq!(
        payload[constants::field::REASON],
        LogFieldValue::String(constants::value::MANAGED_BROWSER_EXECUTABLE_MISSING.to_string())
    );
    assert_eq!(
        payload[constants::field::QUERY_VISIBILITY],
        LogFieldValue::String(constants::browser::QUERY_VISIBILITY_UNAVAILABLE.to_string())
    );
}

#[test]
fn unmanaged_browser_status_reports_discoverable_but_unmanaged_process() {
    let status = unmanaged_browser_status(
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string(),
        constants::browser::DEVTOOLS_TEST_UNMANAGED_PROCESS_ID,
        BrowserFamily::Chrome,
        BrowserChannel::Stable,
    );
    let payload = browser_managed_status_payload(&status);

    assert_eq!(
        status.managed_state,
        BrowserManagedState::InstalledUnsupported
    );
    assert_eq!(
        status.capability_status,
        BrowserCapabilityStatus::UnmanagedBrowser
    );
    assert_eq!(
        payload[constants::field::BROWSER_FAMILY],
        LogFieldValue::String(constants::browser::FAMILY_CHROME.to_string())
    );
    assert_eq!(
        payload[constants::field::REASON],
        LogFieldValue::String(constants::value::MANAGED_BROWSER_UNMANAGED_PROCESS.to_string())
    );
}
