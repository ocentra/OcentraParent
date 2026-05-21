use ocentra_parent_agent_protocol::{
    constants, ActivityEventKind, ActivityObserver, ActivitySubjectKind, BrowserActiveTabState,
    BrowserCapabilityStatus, BrowserChannel, BrowserCustodyLabel, BrowserFamily, LogFieldValue,
};

use crate::{
    browser_tab_observation_event, BrowserBridgeEventError, BrowserBridgeTargetObservation,
};

#[test]
fn browser_target_observation_maps_to_managed_url_activity_event() {
    let event = browser_tab_observation_event(
        BrowserBridgeTargetObservation {
            browser_family: BrowserFamily::Edge,
            browser_channel: BrowserChannel::Stable,
            managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
            profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
            process_id: 4242,
            target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
            tab_id: Some(constants::activity_store::TEST_BROWSER_TAB_ID.to_string()),
            window_id: None,
            active_state: BrowserActiveTabState::Unknown,
            url: constants::activity_store::TEST_BROWSER_URL.to_string(),
            title: Some(constants::activity_store::TEST_BROWSER_TITLE.to_string()),
            capability_status: BrowserCapabilityStatus::TabListOnly,
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        0,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);

    assert_eq!(event.kind, ActivityEventKind::UrlObserved);
    assert_eq!(
        event.source.observer,
        ActivityObserver::ManagedBrowserBridge
    );
    assert_eq!(event.subject.kind, ActivitySubjectKind::Url);
    assert_eq!(
        event.fields.get(constants::field::URL),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_URL.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::DOMAIN),
        Some(&LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_DOMAIN.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::ACTIVE_STATE),
        Some(&LogFieldValue::String(
            constants::browser::ACTIVE_STATE_UNKNOWN.to_string()
        ))
    );
    assert_eq!(
        event.fields.get(constants::field::CAPABILITY_STATUS),
        Some(&LogFieldValue::String(
            constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY.to_string()
        ))
    );
}

#[test]
fn browser_target_observation_rejects_invalid_url_before_journal_write() {
    let error = browser_tab_observation_event(
        BrowserBridgeTargetObservation {
            browser_family: BrowserFamily::Chrome,
            browser_channel: BrowserChannel::Stable,
            managed_browser_session_id: constants::browser::SESSION_ID_DEV.to_string(),
            profile_id: constants::browser::PROFILE_ID_DEV.to_string(),
            process_id: 4242,
            target_id: constants::activity_store::TEST_BROWSER_TARGET_ID.to_string(),
            tab_id: None,
            window_id: None,
            active_state: BrowserActiveTabState::Unknown,
            url: constants::activity_store::TEST_INVALID_BROWSER_URL.to_string(),
            title: None,
            capability_status: BrowserCapabilityStatus::AdapterError,
            custody_label: BrowserCustodyLabel::ChildDeviceLocal,
        },
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        0,
    )
    .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserBridgeEventError::InvalidUrl);
}
