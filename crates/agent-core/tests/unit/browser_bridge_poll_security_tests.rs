use std::net::SocketAddr;

use ocentra_parent_agent_protocol::browser::BrowserFamily;
use ocentra_parent_agent_protocol::constants;

use crate::{
    browser_bridge_poll_support::{
        bridge_config, err, field_value_contains_raw_debugger_url, ok, try_serve_devtools,
        DevtoolsBodies, TestResult,
    },
    poll_chromium_bridge, BrowserBridgePollError,
};

#[test]
fn poll_chromium_bridge_rejects_non_loopback_endpoint() -> TestResult {
    let endpoint = SocketAddr::from((
        [192, 0, 2, 1],
        constants::browser::DEVTOOLS_TEST_BRIDGE_PORT,
    ));
    let config = bridge_config(endpoint);

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_eq!(error, BrowserBridgePollError::NonLoopbackEndpoint);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_NON_LOOPBACK_ENDPOINT
    );

    Ok(())
}

#[test]
fn poll_chromium_bridge_rejects_untrusted_bridge_port() -> TestResult {
    let endpoint = try_serve_devtools(DevtoolsBodies {
        version_body: constants::browser::DEVTOOLS_TEST_VERSION_BODY.to_string(),
        list_body: constants::browser::DEVTOOLS_TEST_LIST_BODY.to_string(),
    })?;
    let mut config = bridge_config(endpoint);
    config.expected_custody.bridge_port = constants::browser::DEVTOOLS_PORT_UNRESERVED;

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_eq!(error, BrowserBridgePollError::UntrustedBridgePort);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_UNTRUSTED_PORT
    );

    Ok(())
}

#[test]
fn poll_chromium_bridge_rejects_wrong_process_custody() -> TestResult {
    let endpoint = try_serve_devtools(DevtoolsBodies {
        version_body: constants::browser::DEVTOOLS_TEST_VERSION_BODY.to_string(),
        list_body: constants::browser::DEVTOOLS_TEST_LIST_BODY.to_string(),
    })?;
    let mut config = bridge_config(endpoint);
    config.expected_custody.process_id = constants::browser::PROCESS_ID_UNKNOWN;

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_eq!(error, BrowserBridgePollError::UntrustedProcess);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_UNTRUSTED_PROCESS
    );

    Ok(())
}

#[test]
fn poll_chromium_bridge_rejects_default_profile_custody() -> TestResult {
    let endpoint = try_serve_devtools(DevtoolsBodies {
        version_body: constants::browser::DEVTOOLS_TEST_VERSION_BODY.to_string(),
        list_body: constants::browser::DEVTOOLS_TEST_LIST_BODY.to_string(),
    })?;
    let mut config = bridge_config(endpoint);
    config.profile_id = constants::browser::PATH_SEGMENT_DEFAULT.to_string();
    config.expected_custody.profile_id = constants::browser::PATH_SEGMENT_DEFAULT.to_string();

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_eq!(error, BrowserBridgePollError::UntrustedProfile);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_UNTRUSTED_PROFILE
    );

    Ok(())
}

#[test]
fn poll_chromium_bridge_rejects_stale_session_custody() -> TestResult {
    let endpoint = try_serve_devtools(DevtoolsBodies {
        version_body: constants::browser::DEVTOOLS_TEST_VERSION_BODY.to_string(),
        list_body: constants::browser::DEVTOOLS_TEST_LIST_BODY.to_string(),
    })?;
    let mut config = bridge_config(endpoint);
    config.expected_custody.session_fresh_until =
        constants::activity_store::TEST_FIRST_OBSERVED_AT.to_string();

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
            constants::activity_store::TEST_THIRD_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_eq!(error, BrowserBridgePollError::StaleSession);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_STALE_SESSION
    );

    Ok(())
}

#[test]
fn poll_chromium_bridge_rejects_wrong_browser_identity() -> TestResult {
    let endpoint = try_serve_devtools(DevtoolsBodies {
        version_body: constants::browser::DEVTOOLS_TEST_VERSION_BODY.to_string(),
        list_body: constants::browser::DEVTOOLS_TEST_LIST_BODY.to_string(),
    })?;
    let mut config = bridge_config(endpoint);
    config.expected_custody.browser_family = BrowserFamily::Edge;

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_eq!(error, BrowserBridgePollError::UntrustedBrowserIdentity);
    assert_eq!(
        error.reason(),
        constants::value::BROWSER_BRIDGE_UNTRUSTED_BROWSER_IDENTITY
    );

    Ok(())
}

#[test]
fn poll_chromium_bridge_redacts_raw_debugger_urls_from_events() -> TestResult {
    let endpoint = try_serve_devtools(DevtoolsBodies {
        version_body: constants::browser::DEVTOOLS_TEST_VERSION_BODY.to_string(),
        list_body: constants::browser::DEVTOOLS_TEST_LIST_BODY_WITH_DEBUGGER_URL.to_string(),
    })?;
    let config = bridge_config(endpoint);

    let snapshot = ok(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    assert_eq!(snapshot.events.len(), 1);
    assert!(snapshot.events[0]
        .fields
        .iter()
        .all(|(_, value)| !field_value_contains_raw_debugger_url(value)));

    Ok(())
}
