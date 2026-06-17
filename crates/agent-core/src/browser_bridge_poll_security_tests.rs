use std::net::SocketAddr;

use ocentra_parent_agent_protocol::{constants, BrowserFamily};

use crate::{
    browser_bridge_poll_test_support::{
        bridge_config, field_value_contains_raw_debugger_url, serve_devtools,
    },
    poll_chromium_bridge, BrowserBridgePollError,
};

#[test]
fn poll_chromium_bridge_rejects_non_loopback_endpoint() {
    let endpoint = SocketAddr::from((
        [192, 0, 2, 1],
        constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
    ));
    let config = bridge_config(endpoint);

    let error = poll_chromium_bridge(
        &config,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserBridgePollError::NonLoopbackEndpoint);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_NON_LOOPBACK_ENDPOINT
    );
}

#[test]
fn poll_chromium_bridge_rejects_untrusted_bridge_port() {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY,
    );
    let mut config = bridge_config(endpoint);
    config.expected_custody.bridge_port = constants::browser::DEVTOOLS_PORT_UNRESERVED;

    let error = poll_chromium_bridge(
        &config,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserBridgePollError::UntrustedBridgePort);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_UNTRUSTED_PORT
    );
}

#[test]
fn poll_chromium_bridge_rejects_wrong_process_custody() {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY,
    );
    let mut config = bridge_config(endpoint);
    config.expected_custody.process_id = constants::browser::PROCESS_ID_UNKNOWN;

    let error = poll_chromium_bridge(
        &config,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserBridgePollError::UntrustedProcess);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_UNTRUSTED_PROCESS
    );
}

#[test]
fn poll_chromium_bridge_rejects_default_profile_custody() {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY,
    );
    let mut config = bridge_config(endpoint);
    config.profile_id = constants::browser::PATH_SEGMENT_DEFAULT.to_string();
    config.expected_custody.profile_id = constants::browser::PATH_SEGMENT_DEFAULT.to_string();

    let error = poll_chromium_bridge(
        &config,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserBridgePollError::UntrustedProfile);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_UNTRUSTED_PROFILE
    );
}

#[test]
fn poll_chromium_bridge_rejects_stale_session_custody() {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY,
    );
    let mut config = bridge_config(endpoint);
    config.expected_custody.session_fresh_until =
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string();

    let error = poll_chromium_bridge(
        &config,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
        constants::activity_store::TEST_THIRD_OBSERVED_AT,
    )
    .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserBridgePollError::StaleSession);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_STALE_SESSION
    );
}

#[test]
fn poll_chromium_bridge_rejects_wrong_browser_identity() {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY,
    );
    let mut config = bridge_config(endpoint);
    config.expected_custody.browser_family = BrowserFamily::Edge;

    let error = poll_chromium_bridge(
        &config,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect_err(constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL);

    assert_eq!(error, BrowserBridgePollError::UntrustedBrowserIdentity);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_UNTRUSTED_BROWSER_IDENTITY
    );
}

#[test]
fn poll_chromium_bridge_redacts_raw_debugger_urls_from_events() {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY_WITH_DEBUGGER_URL,
    );
    let config = bridge_config(endpoint);

    let snapshot = poll_chromium_bridge(
        &config,
        constants::activity_store::TEST_FIRST_OBSERVED_AT,
        constants::activity_store::TEST_SECOND_OBSERVED_AT,
    )
    .expect(constants::error::BROWSER_BRIDGE_MAPS_TARGET);

    assert_eq!(snapshot.events.len(), 1);
    assert!(snapshot.events[0]
        .fields
        .values()
        .all(|value| !field_value_contains_raw_debugger_url(value)));
}
