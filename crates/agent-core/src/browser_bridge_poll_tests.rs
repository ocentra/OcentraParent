use ocentra_parent_agent_protocol::{constants, ActivityEventKind, LogFieldValue};

use crate::{
    browser_bridge_poll_test_support::{bridge_config, serve_devtools, serve_keep_alive_devtools},
    poll_chromium_bridge,
};

#[test]
fn poll_chromium_bridge_maps_page_targets_to_browser_activity_events() {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY,
    );
    let config = bridge_config(endpoint);

    let snapshot = poll_chromium_bridge(
        &config,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);

    assert_eq!(
        snapshot.browser_version,
        Some(constants::browser::DEVTOOLS_TEST_BROWSER_VERSION.to_string())
    );
    assert_eq!(snapshot.page_target_count, 1);
    assert_eq!(snapshot.events.len(), 1);
    assert_eq!(snapshot.events[0].kind, ActivityEventKind::UrlObserved);
    assert_eq!(
        snapshot.events[0].fields[constants::field::URL],
        LogFieldValue::String(constants::activity_store::TEST_BROWSER_URL.to_string())
    );
    assert_eq!(
        snapshot.events[0].fields[constants::field::TAB_ID],
        LogFieldValue::String(
            constants::activity_store::TEST_BROWSER_TAB_ID_FROM_TARGET.to_string()
        )
    );
    assert_eq!(
        snapshot.events[0].fields[constants::field::ACTIVE_STATE],
        LogFieldValue::String(constants::browser::ACTIVE_STATE_UNKNOWN.to_string())
    );
    assert_eq!(
        snapshot.events[0].fields[constants::field::ACTIVE_PROOF_SOURCE],
        LogFieldValue::String(constants::browser::ACTIVE_PROOF_SOURCE_TARGET_LIST_ONLY.to_string())
    );
    assert_eq!(
        snapshot.events[0].fields[constants::field::CAPABILITY_STATUS],
        LogFieldValue::String(constants::browser::CAPABILITY_STATUS_TAB_LIST_ONLY.to_string())
    );
}

#[test]
fn poll_chromium_bridge_preserves_adapter_tab_and_window_ids() {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY_WITH_TAB_WINDOW,
    );
    let config = bridge_config(endpoint);

    let snapshot = poll_chromium_bridge(
        &config,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);

    assert_eq!(snapshot.page_target_count, 1);
    assert_eq!(
        snapshot.events[0].fields[constants::field::TAB_ID],
        LogFieldValue::String(constants::activity_store::TEST_BROWSER_TAB_ID.to_string())
    );
    assert_eq!(
        snapshot.events[0].fields[constants::field::WINDOW_ID],
        LogFieldValue::String(constants::activity_store::TEST_BROWSER_WINDOW_ID.to_string())
    );
}

#[test]
fn poll_chromium_bridge_reports_empty_page_target_discovery() {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_EMPTY_LIST_BODY,
    );
    let config = bridge_config(endpoint);

    let snapshot = poll_chromium_bridge(
        &config,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);

    assert_eq!(snapshot.page_target_count, 0);
    assert!(snapshot.events.is_empty());
}

#[test]
fn poll_chromium_bridge_accepts_keep_alive_devtools_response_with_content_length() {
    let endpoint = serve_keep_alive_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY,
    );
    let config = bridge_config(endpoint);

    let snapshot = poll_chromium_bridge(
        &config,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);

    assert_eq!(
        snapshot.browser_version,
        Some(constants::browser::DEVTOOLS_TEST_BROWSER_VERSION.to_string())
    );
    assert_eq!(snapshot.page_target_count, 1);
    assert_eq!(
        snapshot.events[0].fields[constants::field::TITLE],
        LogFieldValue::String(constants::browser::DEVTOOLS_TEST_PAGE_TITLE.to_string())
    );
}
