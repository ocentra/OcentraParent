use std::fmt::Debug;

use ocentra_parent_agent_protocol::browser::BrowserCapabilityStatus;
use ocentra_parent_agent_protocol::constants;

use crate::{
    browser_bridge_poll_test_support::{
        bridge_config, serve_devtools, serve_devtools_owned, serve_unresponsive_devtools,
    },
    poll_chromium_bridge, BrowserBridgePollError,
};

type TestResult = Result<(), String>;

fn ok<T, E: Debug>(result: Result<T, E>, context: &str) -> Result<T, String> {
    result.map_err(|error| format!("{context}: {error:?}"))
}

fn err<T, E: Debug>(result: Result<T, E>, context: &str) -> Result<E, String> {
    match result {
        Ok(_) => Err(format!("{context}: expected error")),
        Err(error) => Ok(error),
    }
}

#[test]
fn poll_chromium_bridge_skips_blank_and_internal_page_targets() -> TestResult {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_BLANK_AND_INTERNAL_LIST_BODY,
    );
    let config = bridge_config(endpoint);

    let snapshot = ok(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_MAPS_TARGET,
    )?;

    assert_eq!(snapshot.page_target_count, 0);
    assert!(snapshot.events.is_empty());

    Ok(())
}

#[test]
fn poll_chromium_bridge_rejects_non_array_target_payload() -> TestResult {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_INVALID_LIST_BODY,
    );
    let config = bridge_config(endpoint);

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_adapter_error(&error, &BrowserBridgePollError::InvalidTargetPayload);

    Ok(())
}

#[test]
fn poll_chromium_bridge_rejects_invalid_json_body() -> TestResult {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_INVALID_JSON_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY,
    );
    let config = bridge_config(endpoint);

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_adapter_error(&error, &BrowserBridgePollError::InvalidJson);

    Ok(())
}

#[test]
fn poll_chromium_bridge_rejects_non_object_version_payload() -> TestResult {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_EMPTY_LIST_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY,
    );
    let config = bridge_config(endpoint);

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_adapter_error(&error, &BrowserBridgePollError::InvalidTargetPayload);

    Ok(())
}

#[test]
fn poll_chromium_bridge_rejects_missing_page_url() -> TestResult {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY_MISSING_URL,
    );
    let config = bridge_config(endpoint);

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_adapter_error(&error, &BrowserBridgePollError::InvalidTargetPayload);

    Ok(())
}

#[test]
fn poll_chromium_bridge_rejects_missing_target_id() -> TestResult {
    let endpoint = serve_devtools(
        constants::browser::DEVTOOLS_TEST_VERSION_BODY,
        constants::browser::DEVTOOLS_TEST_LIST_BODY_MISSING_ID,
    );
    let config = bridge_config(endpoint);

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_adapter_error(&error, &BrowserBridgePollError::InvalidTargetPayload);

    Ok(())
}

#[test]
fn poll_chromium_bridge_rejects_oversized_devtools_response() -> TestResult {
    let oversized_body = constants::browser::DEVTOOLS_TEST_OVERSIZED_BODY_UNIT
        .repeat(constants::browser::DEVTOOLS_TEST_OVERSIZED_REPEAT_COUNT);
    let endpoint = serve_devtools_owned(
        oversized_body,
        constants::browser::DEVTOOLS_TEST_LIST_BODY.to_string(),
    );
    let config = bridge_config(endpoint);

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_adapter_error(&error, &BrowserBridgePollError::ResponseTooLarge);

    Ok(())
}

#[test]
fn poll_chromium_bridge_reports_devtools_timeout() -> TestResult {
    let endpoint = serve_unresponsive_devtools();
    let config = bridge_config(endpoint);

    let error = err(
        poll_chromium_bridge(
            &config,
            constants::activity_store::TEST_FIRST_OBSERVED_AT,
            constants::activity_store::TEST_SECOND_OBSERVED_AT,
        ),
        constants::error::BROWSER_BRIDGE_REJECTS_INVALID_URL,
    )?;

    assert_eq!(error, BrowserBridgePollError::Timeout);
    assert_eq!(error.reason(), constants::value::BROWSER_BRIDGE_TIMEOUT);
    assert_eq!(
        error.capability_status(),
        BrowserCapabilityStatus::BridgeMissing
    );

    Ok(())
}

fn assert_adapter_error(error: &BrowserBridgePollError, expected: &BrowserBridgePollError) {
    assert_eq!(error, expected);
    assert_eq!(
        error.capability_status(),
        BrowserCapabilityStatus::AdapterError
    );
}
